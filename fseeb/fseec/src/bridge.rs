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

use std::sync::LazyLock;

use libloading::Library;

static LIB_INSTANCE: LazyLock<Library> = LazyLock::new(||
    unsafe {
        Library::new("/data/adb/modules/fs_enhancer_extreme/lib/libutils.so").expect("加载失败")
    }
);

struct Pointers {
    verify:    fn() -> Option<bool>,
    sigsegv:   fn(),
    log_raw:   fn(&str),
    log_info:  fn(&str, &str),
    log_warn:  fn(&str, &str),
    log_error: fn(&str, &str),
    #[cfg(debug_assertions)]
    log_debug: fn(&str, &str)
}

impl Pointers {
    fn load_symbol<T: Copy>(function_name: &str) -> T {
        unsafe {
            *LIB_INSTANCE.get(
                function_name.as_bytes()
            ).expect("符号缺失")
        }
    }
    fn export() -> Self {
        Self {
            verify:    Self::load_symbol("verify"),
            sigsegv:   Self::load_symbol("sigsegv"),
            log_raw:   Self::load_symbol("log_raw"),
            log_info:  Self::load_symbol("log_info"),
            log_warn:  Self::load_symbol("log_warn"),
            log_error: Self::load_symbol("log_error"),
            #[cfg(debug_assertions)]
            log_debug: Self::load_symbol("log_debug")
        }
    }
}

static FN: LazyLock<Pointers> = LazyLock::new(||
    Pointers::export()
);

pub fn verify() -> Option<bool> {
    (FN.verify)()
}

pub fn sigsegv() {
    (FN.sigsegv)()
}

pub mod log {
    use crate::define::LOG_TAG;

    use super::FN;

    pub fn raw(msg: &str) {
        (FN.log_raw)(msg)
    }

    pub fn info(msg: &str) {
        (FN.log_info)(LOG_TAG, msg)
    }

    pub fn warn(msg: &str) {
        (FN.log_warn)(LOG_TAG, msg)
    }

    pub fn error(msg: &str) {
        (FN.log_error)(LOG_TAG, msg)
    }

    #[cfg(debug_assertions)]
    pub fn debug(msg: &str) {
        (FN.log_debug)(LOG_TAG, msg)
    }
}