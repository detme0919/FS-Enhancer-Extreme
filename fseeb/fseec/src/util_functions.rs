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

use crate::{
    define::IS_ZHCN,
    bridge::log
};

use std::{
    fs,
    str,
    process,
    path::Path,
    fmt::Display,
    io::{
        Error,
        Result
    },
    ffi::{
        CStr,
        c_char,
        CString
    }
};

use anyhow::anyhow;
use libc::{
    kill,
    SIGKILL
};

pub fn pidof(name: &str) -> Option<i32> {
    let proc = match fs::read_dir("/proc") {
        Ok(dir) => dir.flatten(),
        Err(error) => {
            log::error(format!("/proc读取失败: {}", error));
            panic!("/proc读取失败: {}", error)
        }
    };
    for process in proc {
        let temp_file_name = process.file_name();
        let file_name = temp_file_name.to_str().unwrap();

        let pid: i32 = if let Ok(integer) = file_name.parse() {
            integer
        } else {
            continue
        };

        let cmdline = if let Ok(exist) =  fs::read(process.path().join("cmdline")) {
            exist
        } else {
            continue
        };

        let Some(bytes) = cmdline.split(|&byte| byte == 0).next() else {
            continue
        };
        let arg_one = if let Ok(normal) = str::from_utf8(bytes) {
            normal
        } else {
            continue
        };
        let basename = arg_one.rsplit('/').next().unwrap();

        if basename == name {
            return Some(pid)
        }
    }

    None
}

pub fn sigkill(pid: i32) -> anyhow::Result<()> {
    if unsafe {
        kill(pid, SIGKILL)
    } == 0 {
        Ok(())
    } else {
        log::error(&format!("kill调用失败: {}", Error::last_os_error()));
        Err(anyhow!("{}|{}", pid, Error::last_os_error()))
    }
}

fn result_process(result: &process::Output, is_stderr: bool) -> (i32, String) {
    let code = result.status.code().unwrap();
    let source = if is_stderr {
        &result.stderr
    } else {
        &result.stdout
    };
    let output = String::from_utf8_lossy(source).trim().to_string();
    (code, output)
}

fn intercept_log_err(result: &process::Output, is_stderr: bool) {
    let (code, output) = result_process(result, is_stderr);
    log::error(format!("{}|{}", code, output));
}

fn pass_through_err(result: &process::Output, is_stderr: bool) -> anyhow::Error {
    let (code, output) = result_process(result, is_stderr);
    anyhow!("{}|{}", code, output)
}

fn intercept_log_and_pass_through_err(result: Result<process::Output>, command: &str, is_stderr: bool) -> anyhow::Result<process::Output> {
    match result {
        Ok(success) => {
            if success.status.success() {
                Ok(success)
            } else {
                intercept_log_err(&success, is_stderr);
                Err(pass_through_err(&success, is_stderr))
            }
        }
        Err(error) => {
            log::error(format!("{} 执行失败: {}", command, error));
            Err(error.into())
        }
    }
}

unsafe extern "C" {
    fn __system_property_get(__name: *const c_char, __value: *mut c_char) -> i32;
}

pub fn getprop(prop: &str) -> String {
    let name = CString::new(prop).unwrap();
    let mut buffer = [0u8; 96];

    unsafe {
        __system_property_get(name.as_ptr(), buffer.as_mut_ptr())
    };

    CStr::from_bytes_until_nul(&buffer).unwrap().to_string_lossy().into()
}

pub fn resetprop(args: &[&str]) -> anyhow::Result<()> {
    let result = process::Command::new("resetprop").args(args)
        .output();
    intercept_log_and_pass_through_err(result, "resetprop", true)?;

    Ok(())
}

pub fn am_start(args: &[&str]) -> anyhow::Result<()> {
    let am_result = process::Command::new("cmd").args(["activity", "start", "-n"]).args(args)
        .output()?;
    if am_result.status.success() {
        Ok(())
    } else {
        Err(pass_through_err(&am_result, false))
    }
}

fn pm(args: &[&str]) -> Result<process::Output> {
    process::Command::new("cmd").arg("package").args(args).output()
}

pub fn pm_install(arg: String) -> bool {
    match pm(&["install", &arg]) {
        Ok(result) => {
            if result.status.success() {
                true
            } else {
                log::error("安装失败");
                intercept_log_err(&result, false);
                false
            }
        }
        Err(error) => {
            log::error(format!("安装失败: {}", error));
            false
        }
    }
}

pub fn pm_uninstall(arg: &str) {
    match pm(&["uninstall", arg]) {
        Ok(result) => {
            if !result.status.success() {
                log::error("卸载失败");
                intercept_log_err(&result, false);
            }
        }
        Err(error) => log::error(format!("卸载失败: {}", error))
    }
}

pub fn pm_list(arg: &str) -> anyhow::Result<String> {
    let result = pm(&["list", "package", arg]);
    Ok(
        String::from_utf8(
            intercept_log_and_pass_through_err(result, "pm list", false)?.stdout
        ).unwrap().trim().to_string()
    )
}

pub fn pm_path(arg: &str, crash: bool) -> anyhow::Result<bool> {
    match pm(&["path", arg]) {
        Ok(success) => match success.status.code() {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => if crash {
                Err(pass_through_err(&success, false))
            } else {
                intercept_log_err(&success, false);
                Ok(false)
            }
        }
        Err(error) => if crash {
            Err(error.into())
        } else {
            log::error(format!("pm path执行失败: {}", error));
            Ok(false)
        }
    }
}

pub fn read_to_string(path: impl AsRef<Path>) -> anyhow::Result<String> {
    fs::read_to_string(&path).map_err(|error|{
        log::error(format!("{} 读取失败: {}", path.as_ref().display(), error));
        error.into()
    })
}

pub fn write(path: impl AsRef<Path>, data: impl AsRef<[u8]>, log: bool) {
    if let Err(error) = fs::write(&path, data) {
        log::error(format!("{} 写入失败: {}", path.as_ref().display(), error));
    } else {
        if log {
            log::info("写入成功")
        }
    }
}

pub fn override_description(path: &str, description: impl AsRef<str>) {
    let file = format!("{}/module.prop", path);
    let Ok(content) = read_to_string(&file) else {
        return
    };

    let desc_prefix: &str = "description=";
    if content.contains(desc_prefix) {
        let final_desc: String = format!("{}{}", desc_prefix, description.as_ref());
        let data: String = content.lines().map(|line|
            if line.starts_with(desc_prefix) {
                &final_desc
            } else {
                line
            }
        ).intersperse("\n").collect();

        write(file, data, false)
    } else {
        log::error("文件损坏")
    }
}

pub fn create_file(path: impl AsRef<Path>) {
    if let Err(error) = fs::File::create(path) {
        log::error(format!("创建失败: {}", error))
    }
}

pub fn delete_file(path: impl AsRef<Path>) {
    fs::remove_file(path).ok();
}

pub fn setting_get_positive(setting_file: &str) -> ! {
    if Path::new(setting_file).exists() {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

pub fn print_cn(msg: impl Display) {
    if *IS_ZHCN {
        print!("- {}", msg);
    }
}
pub fn print_en(msg: impl Display) {
    if !*IS_ZHCN {
        print!("- {}", msg);
    }
}
pub fn println_cn(msg: impl Display) {
    if *IS_ZHCN {
        println!("{}", msg);
    }
}
pub fn println_en(msg: impl Display) {
    if !*IS_ZHCN {
        println!("{}", msg);
    }
}