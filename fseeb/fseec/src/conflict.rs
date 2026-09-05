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
        CONFLICT_APP,
        TAG_CONFLICT_MOD,
        DEL_CONFLICT_MOD,
        MODULESDIR,
        MODULESUPDATEDIR,
        SKIP_APPCHECK,
        SKIP_MODCHECK,
        DESC_CONFLICT_MOD
    },
    util_functions::{
        pm_uninstall,
        pm_path,
        override_description,
        create_file,
        delete_file,
        setting_get_positive
    },
    cli::Mode,
    bridge::log
};

use std::{
    fs,
    process,
    path::Path,
    thread::sleep,
    time::Duration
};

pub fn app_process() -> anyhow::Result<()> {
    for conflict_app in CONFLICT_APP {
        if pm_path(conflict_app, false)? {
            pm_uninstall(conflict_app);
        }
    }

    Ok(())
}

fn delete_path(path: impl AsRef<Path>) {
    fs::remove_dir_all(path).ok();
}

fn del_conflict_mod_process() {
    for del_conflict_mod in DEL_CONFLICT_MOD {
        let conflict_mod_full_path: String = format!("{}/{}", MODULESDIR, del_conflict_mod);
        if Path::new(&conflict_mod_full_path).exists() {
            log::info(format!("移除: {}", del_conflict_mod));
            drop(process::Command::new("sh").current_dir(&conflict_mod_full_path).arg("./uninstall.sh")
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .status());
            delete_path(conflict_mod_full_path);
        }
    }
}

fn tag_conflict_mod_process(boot_mode: bool) {
    for tag_conflict_mod in TAG_CONFLICT_MOD {
        let conflict_mod_full_path: String = format!("{}/{}", MODULESDIR, tag_conflict_mod);
        if Path::new(&conflict_mod_full_path).exists() {
            log::info(format!("处理: {}", tag_conflict_mod));
            override_description(&conflict_mod_full_path, *DESC_CONFLICT_MOD);
            create_file(format!("{}/disable", conflict_mod_full_path));
            create_file(format!("{}/remove", conflict_mod_full_path));
            if boot_mode {
                create_file(format!("{}/update", conflict_mod_full_path));
                delete_file(format!("{}/uninstall.sh", conflict_mod_full_path));
                delete_file(format!("{}/{}/uninstall.sh", MODULESUPDATEDIR, tag_conflict_mod));
            } else {
                delete_path(format!("{}/{}", MODULESUPDATEDIR, tag_conflict_mod));
            }
        }
    }
}

fn boot_process() {
    tag_conflict_mod_process(true);
}

fn daemon_process() {
    sleep(Duration::from_secs(2));
    tag_conflict_mod_process(false);
}

pub fn route(mode: Mode) {
    if mode.boot {
        boot_process();
    } else {
        daemon_process();
    }
    del_conflict_mod_process()
}

pub fn appcheck_on() {
    delete_file(SKIP_APPCHECK)
}

pub fn appcheck_off() {
    create_file(SKIP_APPCHECK)
}

pub fn appcheck_get() -> ! {
    setting_get_positive(SKIP_APPCHECK)
}

pub fn modcheck_on() {
    delete_file(SKIP_MODCHECK)
}

pub fn modcheck_off() {
    create_file(SKIP_MODCHECK)
}

pub fn modcheck_get() -> ! {
    setting_get_positive(SKIP_MODCHECK)
}