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

mod mainmodule;
mod rootimplement;

use crate::define::{
    UNKNOWN,
    MAIN_MODULE_ENV_FILE,
    ROOT_IMPLEMENT_ENV_FILE,
    Environment,
};

fn process_collect_result(collect_result: Vec<(&str, u32)>) -> Environment {
    match collect_result.len() {
        0 => Environment {
            multiple: false,
            identity: UNKNOWN.to_string(),
            version: 0,
        },
        1 => {
            let (identity, version_code) = collect_result[0];
            Environment {
                multiple: false,
                identity: identity.to_string(),
                version: version_code,
            }
        }
        _ => {
            let multiple_type: Vec<String> = collect_result
                .iter()
                .map(|(identity, version_code)| format!("{} ({})", identity, version_code))
                .collect();
            Environment {
                multiple: true,
                identity: multiple_type.join(" | "),
                version: u32::MAX,
            }
        }
    }
}

fn main_module_process() -> Environment {
    process_collect_result(
        mainmodule::collect()
    )
}

fn root_implement_process() -> Environment {
    let mut collect_result: Vec<(&str, u32)> = Vec::new();

    rootimplement::apatch_invoke(&mut collect_result);
    rootimplement::kernelsu_invoke(&mut collect_result);
    rootimplement::magisk_detect(&mut collect_result);

    process_collect_result(collect_result)
}

pub fn entry() {
    Environment::export(
        MAIN_MODULE_ENV_FILE,
        main_module_process()
    );
    Environment::export(
        ROOT_IMPLEMENT_ENV_FILE,
        root_implement_process()
    );
}