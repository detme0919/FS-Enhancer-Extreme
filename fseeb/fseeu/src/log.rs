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

use crate::define::{
    FSEELOG,
    PID
};

use std::{
    mem,
    io::Write,
    fs::OpenOptions
};

use libc::{
    tm,
    gettid,
    timespec,
    localtime_r,
    clock_gettime,
    CLOCK_REALTIME
};

fn write(msg: String) {
    OpenOptions::new().create(true).append(true).open(FSEELOG).and_then(|mut content|
        content.write_all(
            msg.as_bytes()
        )
    ).ok();
}

pub fn output(level: char, tag: &str, msg: &str) {
    let (ts, tm, tid) = unsafe {
        let mut ts: timespec = mem::zeroed();
        clock_gettime(CLOCK_REALTIME, &mut ts);

        let mut tm: tm = mem::zeroed();
        localtime_r(&ts.tv_sec, &mut tm);

        (ts, tm, gettid())
    };
    write(format!(
        "{:02}-{:02} {:02}:{:02}:{:02}.{:03}  {}  {} {} [FSEE]  : <{}> {}\n",
        tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min, tm.tm_sec, ts.tv_nsec / 1_000_000,
        *PID, tid, level, tag, msg
    ))
}

pub fn output_raw(raw: &str) {
    write(
        format!("{}\n", raw)
    )
}