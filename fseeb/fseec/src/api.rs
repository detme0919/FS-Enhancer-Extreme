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
        FSEEMODDIR,
        Environment,
        MAIN_MODULE,
        ROOT_IMPLEMENT,
        VERIFY
    },
    util_functions::{
        getprop,
        pm_list,
        read_to_string
    }
};

use std::ffi::{
    CStr,
    c_char
};

use anyhow::ensure;

pub fn version_name() -> anyhow::Result<()> {
    let content: String = read_to_string(format!("{}/module.prop", FSEEMODDIR))?;

    let version_name: &str = content.lines().find_map(|line|
        line.strip_prefix("version=v")
    ).unwrap();

    print!("{}", version_name);

    Ok(())
}

fn print_identity(env: &Environment) {
    if env.multiple {
        print!("{}", env.identity)
    } else {
        print!("{}[{}]", env.identity, env.version)
    }
}

pub fn main_module() {
    print_identity(&MAIN_MODULE)
}

pub fn root_implement() {
    print_identity(&ROOT_IMPLEMENT)
}

pub fn integrity_status() {
    let status = match *VERIFY {
        Some(true) => 2,
        None => 1,
        Some(false) => 0
    };

    print!("{}", status)
}

pub fn getenforce() -> anyhow::Result<()> {
    let status: String = read_to_string("/sys/fs/selinux/enforce")?;

    print!("{}", status.trim());

    Ok(())
}

fn print_prop(arg: &str) {
    let result: String = getprop(arg);

    print!("{}", result)
}

pub fn android_version() -> anyhow::Result<()> {
    let release: String = getprop("ro.build.version.release");
    let sdk: String = getprop("ro.build.version.sdk");

    print!("{}[API {}]", release, sdk);

    Ok(())
}

pub fn device_arch() {
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

pub fn fingerprint() {
    print_prop("ro.build.fingerprint")
}

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