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
        DISABLE,
        FORGE_STORE,
        TRICKY_STORE,
        OH_MY_KEYMINT,
        TEESIMULATOR,
        FSMODDIR,
        TSMODDIR,
        OMKMODDIR,
        TEESMODDIR
    },
    util_functions::read_to_string
};

use std::path::Path;

macro_rules! unify_process {
    ($result_collect:expr, $moddir:expr) => {{
        if Path::new(&format!("{}/disable", $moddir)).exists() {
            $result_collect.push(None);
            return
        }

        let prop_full_path = format!("{}/module.prop", $moddir);
        let prop_file = Path::new(&prop_full_path);

        if !prop_file.exists() {
            return
        }

        read_to_string(prop_file)
    }}
}

fn parse_version_code(content: &String) -> Option<u32> {
    content.lines().find_map(|line|
        line.strip_prefix("versionCode=").and_then(|vc_str|
            vc_str.parse().ok()
        )
    )
}

fn detect_process(result_collect: &mut Vec<Option<(&str, u32)>>, moddir: &str, identity: &'static str) {
    let Ok(content) = unify_process!(result_collect, moddir) else {
        return
    };

    if let Some(version_code) = parse_version_code(&content) {
        result_collect.push(
            Some((identity, version_code))
        )
    }
}

fn forge_store(result_collect: &mut Vec<Option<(&str, u32)>>) {
    detect_process(result_collect, FSMODDIR, FORGE_STORE)
}

fn tricky_store(result_collect: &mut Vec<Option<(&str, u32)>>) {
    let Ok(content) = unify_process!(result_collect, TSMODDIR) else {
        return
    };

    let Some(version_code) = parse_version_code(&content) else {
        return
    };

    if let Some(name) = content.lines().find_map(|line|
        line.strip_prefix("name=")
    ) {
        result_collect.push(
            Some((
                match name {
                    identity if identity.contains("Tricky") && identity.contains("Store") && identity.contains("OSS") => "TrickyStoreOSS",
                    identity if identity.contains("TEESimulator") && identity.contains("RS") => "TEESimulatorRS",
                    identity if identity.contains("TEESimulator") => "TEESimulatorLegacy",
                    _ => TRICKY_STORE
                },
                version_code
            ))
        )
    }
}

fn oh_my_keymint(result_collect: &mut Vec<Option<(&str, u32)>>) {
    detect_process(result_collect, OMKMODDIR, OH_MY_KEYMINT)
}

fn teesim(result_collect: &mut Vec<Option<(&str, u32)>>) {
    detect_process(result_collect, TEESMODDIR, TEESIMULATOR)
}

pub fn collect() -> Vec<(&'static str, u32)> {
    let mut result_collect: Vec<Option<(&str, u32)>> = Vec::new();

    forge_store(&mut result_collect);
    tricky_store(&mut result_collect);
    oh_my_keymint(&mut result_collect);
    teesim(&mut result_collect);

    if result_collect.is_empty() {
        return Vec::new()
    }

    if result_collect.iter().all(|result|
        result.is_none()
    ) {
        vec![(DISABLE, 0)]
    } else {
        result_collect.into_iter().flatten().collect()
    }
}