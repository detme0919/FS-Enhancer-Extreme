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
        FORGE_STORE,
        FSMODDIR,
        FSEEMODDIR,
        MAIN_MODULE
    },
    util_functions::{
        pidof,
        sigkill
    }
};

use std::{
    process,
    process::Stdio
};

fn other_intercept() {
    if MAIN_MODULE.identity != FORGE_STORE {
        println!("Not support main module other than ForgeStore");
        process::exit(1)
    }
}

pub fn fs_state() {
    other_intercept();

    if let Some(pid) = pidof("forgestore") {
        println!("running|{}", pid)
    } else {
        println!("not running")
    }
}

pub fn fs_stop() {
    other_intercept();

    if let Some(pid) = pidof("forgestore") {
        if sigkill(pid).is_ok() {
            println!("stopped|{}", pid)
        } else {
            println!("failure");
            process::exit(1)
        }
    } else {
        println!("not running")
    }
}

pub fn fs_start() {
    other_intercept();

    let Ok(daemon) = process::Command::new(format!("{}/daemon", FSMODDIR)).arg(FSMODDIR)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    else {
        println!("failure");
        process::exit(1)
    };

    println!("success|{}", daemon.id())
}

pub fn fs_restart() {
    fs_stop();
    fs_start()
}

pub fn fsee_state() {
    if let Some(pid) = pidof("fsees") {
        println!("running|{}", pid)
    } else {
        println!("not running")
    }
}

pub fn fsee_stop() {
    if let Some(pid) = pidof("fsees") {
        if sigkill(pid).is_ok() {
            println!("stopped|{}", pid)
        } else {
            println!("failure");
            process::exit(1)
        }
    } else {
        println!("not running")
    }
}

pub fn fsee_start() {
    let Ok(daemon) = process::Command::new(format!("{}/bin/fsees", FSEEMODDIR))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    else {
        println!("failure");
        process::exit(1)
    };

    println!("success|{}", daemon.id())
}

pub fn fsee_restart() {
    fsee_stop();
    fsee_start()
}