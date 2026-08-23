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

mod define;
mod log;
mod verify;

use {
    log::{
        output,
        output_raw
    },
    verify::entry
};

use std::process;

#[unsafe(no_mangle)]
pub fn verify() -> Option<bool> {
    entry()
}
#[unsafe(no_mangle)]
pub fn log_info(tag: &str, msg: &str) {
    output('I', tag, msg)
}
#[unsafe(no_mangle)]
pub fn log_warn(tag: &str, msg: &str) {
    output('W', tag, msg)
}
#[unsafe(no_mangle)]
pub fn log_error(tag: &str, msg: &str) {
    output('E', tag, msg)
}
#[unsafe(no_mangle)]
pub fn log_debug(tag: &str, msg: &str) {
    output('D', tag, msg)
}
#[unsafe(no_mangle)]
pub fn log_raw(msg: &str) {
    output_raw(msg)
}
#[unsafe(no_mangle)]
pub fn sigsegv() {
    log_error("lib", "遭到篡改");
    unsafe {
        *(0xDEADBEEF as *mut u8) = u8::MIN
    }
    process::abort()
}