/*
 * This file is part of FS-Enhancer-Extreme.
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program;
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * Copyright (C) 2026 XtrLumen
 */

#![allow(unused_must_use)]

mod bridge;
mod define;

use {
    define::{
        FSEEC,
        SKIP_APPCHECK,
        SKIP_MODCHECK
    },
    bridge::log
};

use std::{
    fs,
    thread,
    path::Path,
    process::Command,
    thread::JoinHandle,
    os::unix::process::ExitStatusExt,
    ffi::{
        c_void,
        CString
    }
};

use libc::{
    read,
    close,
    inotify_init,
    inotify_add_watch,
    IN_ISDIR,
    IN_CREATE,
    IN_DELETE,
};

fn monitor(args: &[&str], path: &str, events: u32) {
    macro_rules! proxy {
        ($log_level:path, $msg:expr) => {
            $log_level(format!("{}: {} << {:032b}", $msg, path, events))
        }
    }

    if !Path::new(path).exists() {
        log::warn(format!("目录 {} 不存在, 尝试创建", path));
        if let Err(error) = fs::create_dir_all(path) {
            log::error(format!("目录 {} 创建失败: {}, 结束线程", path, error));
            return
        }
    }

    let instance = unsafe {
        inotify_init()
    };
    if instance < 0 {
        proxy!(log::error, "实例创建失败");
        return
    }

    let watch = unsafe {
        let target = CString::new(path).unwrap();
        inotify_add_watch(instance, target.as_ptr(), events)
    };
    if watch < 0 {
        proxy!(log::error, "监听添加失败");
        unsafe {
            close(instance);
        }
        return
    }

    proxy!(log::info, "监听就绪");

    let mut buffer = [0u8; 1024];
    //循环启动
    loop {
        //阻塞
        unsafe {
            read(
                instance,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len()
            )
        };

        let all_args = args.join(" + ");
        log::info(format!("执行 fseec {}", all_args));

        for arg in args {
            match Command::new(FSEEC).arg(*arg)
                .output()
            {
                Ok(output) => if !output.status.success() && output.status.signal() != Some(11) {
                    log::warn(format!(
                        "{} | {}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
                Err(error) => log::error(format!("启动失败: {}", error))
            }
        }
    }
}

fn main() {
    if bridge::verify() == Some(false) {
        bridge::sigsegv()
    }
    log::info("启动线程");

    let mut app_args: Vec<&str> = vec!["listrefresh"];
    if Path::new(SKIP_APPCHECK).exists() {
        log::info("跳过推入 appcheck 参数")
    } else {
        app_args.push("appcheck")
    }
    let app_handle: JoinHandle<()> = thread::spawn(move||
        monitor(
            &app_args,
            "/data/app",
            IN_CREATE | IN_DELETE
        )
    );

    let mod_handle: Option<JoinHandle<()>> = if Path::new(SKIP_MODCHECK).exists() {
        log::info("跳过启动 modcheck 线程");
        None
    } else {
        Some(
            thread::spawn(||
                monitor(
                    &["modcheck"],
                    "/data/adb/modules_update",
                    IN_CREATE | IN_ISDIR
                )
            )
        )
    };

    app_handle.join();
    if let Some(handle) = mod_handle {
        handle.join();
    }
}