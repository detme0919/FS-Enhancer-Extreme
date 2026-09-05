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
        UNKNOWN,
        OH_MY_KEYMINT,
        TEESIMULATOR,
        FSEEMODDIR,
        VERIFY,
        ROOT_IMPLEMENT,
        MAIN_MODULE,
        ENV_ABNORMAL,
        DESC_BASE,
        DESC_MAIN_MODULE_NOT_INSTALL,
        DESC_MULTIPLE_PREFIX,
        DESC_DISABLE,
        DESC_MAIN_MODULE,
        DESC_ROOT_IMPL,
        DESC_INTEGRITY,
        DESC_SERVICE,
        DESC_INTEGRITY_SUCCESS,
        DESC_INTEGRITY_WARNING,
        DESC_INTEGRITY_ERROR,
        DESC_SERVICE_SUCCESS,
        DESC_SERVICE_FAILURE,
        DESC_SERVICE_NOT_START,
    },
    util_functions::{
        pidof,
        override_description
    },
    bridge::log,
    envcollect
};

use std::{
    fs,
    path::Path
};

use clap::Args;

#[derive(Args)]
pub struct Mode {
    /// Force recollect environment
    #[arg(short, long)]
    debug: bool
}

pub fn refresh(mode: Mode) -> anyhow::Result<()> {
    if mode.debug {
        envcollect::entry()
    }

    let base_path = Path::new(FSEEMODDIR);
    if let Err(error) = try {
        let data = fs::read(base_path.join("module.prop"))?;
        if data.iter().all(|bytes|
            *bytes == 0
        ) {
            fs::copy(base_path.join("other/module.base"), base_path.join("module.prop"))?;
        }
    } {
        log::error(format!("失败: {}", error));
        panic!("失败: {}", error)
    }

    let full_environment: String = if Path::new(&format!("{}/disable", FSEEMODDIR)).exists() {
        format!("❌{}", *DESC_DISABLE)
    } else {
        let (main_module_prefix, main_module_identity): (&str, &str) = if MAIN_MODULE.multiple {
            ("❌", &format!("{} - {}", *DESC_MULTIPLE_PREFIX, MAIN_MODULE.identity))
        } else if MAIN_MODULE.identity == UNKNOWN {
            ("❌", *DESC_MAIN_MODULE_NOT_INSTALL)
        } else if MAIN_MODULE.identity == DISABLE {
            ("❌", *DESC_DISABLE)
        } else {
            (
                if matches!(MAIN_MODULE.identity.as_str(), OH_MY_KEYMINT | TEESIMULATOR) {
                    "❌"
                } else {
                    "✅"
                },
                &format!("{} ({})", MAIN_MODULE.identity, MAIN_MODULE.version)
            )
        };

        let (root_implement_prefix, root_implement_identity): (&str, &String) = if ROOT_IMPLEMENT.multiple {
            ("❌", &format!("{} - {}", *DESC_MULTIPLE_PREFIX, ROOT_IMPLEMENT.identity))
        } else if ROOT_IMPLEMENT.identity == UNKNOWN {
            ("⚠️", &ROOT_IMPLEMENT.identity)
        } else {
            ("✅", &format!("{} ({})", ROOT_IMPLEMENT.identity, ROOT_IMPLEMENT.version))
        };

        let (integrity_prefix, integrity_state) = match *VERIFY {
            Some(true) => ("✅", *DESC_INTEGRITY_SUCCESS),
            Some(false) => ("❌", *DESC_INTEGRITY_ERROR),
            None => ("⚠️", *DESC_INTEGRITY_WARNING)
        };

        let (daemon_prefix, daemon_state) = if *ENV_ABNORMAL {
            ("❌", *DESC_SERVICE_NOT_START)
        } else {
            if pidof("fsees") == None {
                ("❌", *DESC_SERVICE_FAILURE)
            } else {
                ("✅", *DESC_SERVICE_SUCCESS)
            }
        };

        format!(
            "{}{}{}, {}{}{}, {}{}{}, {}{}{}",
            *DESC_MAIN_MODULE, main_module_prefix, main_module_identity,
            *DESC_ROOT_IMPL, root_implement_prefix, root_implement_identity,
            *DESC_INTEGRITY, integrity_prefix, integrity_state,
            *DESC_SERVICE, daemon_prefix, daemon_state
        )
    };

    override_description(FSEEMODDIR, format!("[{}] {}", full_environment, *DESC_BASE));

    println!("[{}]", full_environment);

    Ok(())
}