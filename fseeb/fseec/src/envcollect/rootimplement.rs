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

use std::process;

pub fn apatch_invoke(collect_result: &mut Vec<(&str, u32)>) {
    if let Some(version_code) = process::Command::new("apd").arg("-V")
        .output().ok().and_then(|output|
            String::from_utf8(output.stdout).ok()
        ).and_then(|version_str|
            version_str.split_whitespace().nth(1)?.parse().ok()
        )
    {
        collect_result.push(("APatch", version_code));
    }
}

pub fn kernelsu_invoke(collect_result: &mut Vec<(&str, u32)>) {
    if let Some(version_code) = process::Command::new("/data/adb/ksu/bin/ksud").args(["debug", "version"])
        .output().ok().and_then(|output|
            String::from_utf8(output.stdout).ok()
        ).and_then(|version_str|
            version_str.split_whitespace().nth(2)?.parse().ok()
        ).filter(|&version|
            version > 0
        )
    {
        collect_result.push(("KernelSU", version_code));
    }
}

fn exec_magisk(arg: &str) -> Option<String> {
    process::Command::new("magisk").arg(arg)
        .output().ok().and_then(|output|
            String::from_utf8(output.stdout).ok()
        )
}

pub fn magisk_detect(collect_result: &mut Vec<(&str, u32)>) {
    let Some(version_code) = (
        try {
            exec_magisk("-V")?.trim().parse::<u32>().ok()?
        }
    ) else {
        return
    };

    if let Some(version_name) = exec_magisk("-v") {
        collect_result.push((
            match version_name {
                variant if variant.contains("alpha") => "MagiskAlpha",
                variant if variant.contains("kitsune") => "MagiskKitsune",
                _ => "MagiskOfficial"
            },
            version_code
        ))
    }
}