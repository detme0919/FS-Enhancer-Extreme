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

use crate::util_functions::{
    getprop,
    pm_list,
    read_to_string
};

use std::{
    ffi::CStr,
    os::raw::c_char
};

use anyhow::{
    bail,
    ensure
};

fn print_pm_list_json(arg: &str) -> anyhow::Result<()> {
    let result: String = pm_list(arg)?;

    let packages: Vec<&str> = result.lines().filter_map(|line|
        line.strip_prefix("package:")
    ).collect();

    print!("{:?}", packages);

    Ok(())
}

pub fn system_package() -> anyhow::Result<()> {
    print_pm_list_json("-s")
}

pub fn user_package() -> anyhow::Result<()> {
    print_pm_list_json("-3")
}

// todo!();

pub fn getenforce() -> anyhow::Result<()> {
    let status: &str = match read_to_string("/sys/fs/selinux/enforce")?.trim() {
        "0" => "Permissive",
        "1" => "Enforcing",
        other => bail!("SELinux 未知状态: {}", other)
    };

    print!("{}", status);

    Ok(())
}

fn print_prop(arg: &str) -> anyhow::Result<()> {
    let result: String = getprop(arg);

    print!("{}", result);

    Ok(())
}

pub fn android_version() -> anyhow::Result<()> {
    let release: String = getprop("ro.build.version.release");
    let sdk: String = getprop("ro.build.version.sdk");

    print!("{} (API {})", release, sdk);

    Ok(())
}

pub fn device_arch() -> anyhow::Result<()> {
    print_prop("ro.product.cpu.abi")
}

#[repr(C)]
struct Utsname {
    sysname:    [c_char; 65],
    nodename:   [c_char; 65],
    release:    [c_char; 65],
    version:    [c_char; 65],
    machine:    [c_char; 65],
    domainname: [c_char; 65]
}

unsafe extern "C" {
    fn uname(uts: *mut Utsname) -> i32;
}

pub fn kernel_version() -> anyhow::Result<()> {
    let mut buffer = Utsname {
        sysname:    [0; 65],
        nodename:   [0; 65],
        release:    [0; 65],
        version:    [0; 65],
        machine:    [0; 65],
        domainname: [0; 65]
    };

    let code: i32 = unsafe {
        uname(&mut buffer)
    };

    ensure!(code >= 0, "uname调用失败: {}", code);

    print!("{}", CStr::from_bytes_until_nul(&buffer.release).unwrap().to_string_lossy().into_owned());

    Ok(())
}

pub fn fingerprint() -> anyhow::Result<()> {
    print_prop("ro.build.fingerprint")
}