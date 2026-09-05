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
    define::{
        SKIP_SPSYNC,
        FINAL_MAIN_MODULE_CONFIG
    },
    util_functions::{
        pidof,
        sigkill,
        resetprop,
        read_to_string,
        create_file,
        delete_file,
        setting_get_positive
    },
    cli::Mode,
    bridge::log
};

use std::{
    process,
    path::Path,
};

use regex_lite::Regex;

pub fn sync(mode: Mode) -> anyhow::Result<()> {
    if mode.boot && Path::new(SKIP_SPSYNC).exists() {
        log::info("跳过: 同步安全补丁级别");
        process::exit(1)
    }

    let security_patch_full_path = format!("{}/security_patch.txt", *FINAL_MAIN_MODULE_CONFIG);
    let security_patch_file = Path::new(&security_patch_full_path);
    if !security_patch_file.exists() {
        log::warn("文件不存在");
        process::exit(1)
    }
    if let Ok(success) = read_to_string(security_patch_file) {
        if success.is_empty() {
            log::warn("解析失败");
            process::exit(1)
        }
        let date = if Regex::new(r"^\d{4}-\d{2}-\d{2}$")?.is_match(&success) {
            Some(success)
        } else if Regex::new(r"^\d{8}$")?.is_match(&success) {
            Some(format!(
                "{}-{}-{}",
                &success[0..4],
                &success[4..6],
                &success[6..8]
            ))
        } else {
            None
        };
        if let Some(date) = date {
            resetprop(&["ro.vendor.build.security_patch", &date])?;
            resetprop(&["ro.build.version.security_patch", &date])?;

            if let Some(pid) = pidof("com.google.android.gms.unstable") {
                sigkill(pid)?
            }

            log::info("同步完毕")
        } else {
            log::error("格式错误");
            process::exit(1)
        }
    }

    Ok(())
}

pub fn spsync_on() {
    delete_file(SKIP_SPSYNC)
}

pub fn spsync_off() {
    create_file(SKIP_SPSYNC)
}

pub fn spsync_get() -> ! {
    setting_get_positive(SKIP_SPSYNC)
}