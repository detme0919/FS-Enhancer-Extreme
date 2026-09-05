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
        APPEND_FILE,
        REMOVE_FILE,
        BLACK_LIST,
        FINAL_MAIN_MODULE_CONFIG
    },
    util_functions::{
        pm_list,
        read_to_string,
        write,
        create_file,
        delete_file
    }
};

use std::collections::HashSet;
use std::path::Path;
use std::process;

pub fn refresh() -> anyhow::Result<()> {
    let result: String = pm_list("-3")?;

    let packages: HashSet<&str> = result.lines().filter_map(|line|
        line.strip_prefix("package:")
    ).collect();

    let usr: HashSet<String> = if let Ok(file) = read_to_string(REMOVE_FILE) {
        file.lines().map(|content|
            content.to_string()
        ).collect()
    } else {
        HashSet::new()
    };

    let mut output: String = String::new();
    for pkg in packages.iter().filter(|package|
        !usr.contains(**package)
    ) {
        output.push_str(pkg);
        output.push('\n')
    }

    if let Ok(sys) = read_to_string(APPEND_FILE) {
        output.push_str(&sys)
    }

    let target = format!("{}/target.txt", *FINAL_MAIN_MODULE_CONFIG);
    write(target, output, false);

    Ok(())
}

pub fn blacklist_on() {
    create_file(BLACK_LIST)
}

pub fn blacklist_off() {
    delete_file(BLACK_LIST)
}

pub fn blacklist_get() -> ! {
    if Path::new(BLACK_LIST).exists() {
        process::exit(1)
    } else {
        process::exit(0)
    }
}

pub fn blacklist_to_whitelist() -> anyhow::Result<()> {
    Ok(())
}

pub fn whitelist_to_blacklist() -> anyhow::Result<()> {
    Ok(())
}