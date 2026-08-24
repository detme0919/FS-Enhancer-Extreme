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
    SYNC_LEN,
    FSEEMODDIR
};

use std::{
    fs,
    fs::File,
    io::Read,
    path::Path
};

use blake3::Hasher;
use arrayvec::ArrayString;
use ed25519_compact::{
    PublicKey,
    Signature
};

pub fn entry() -> Option<bool> {
    let base_path = Path::new(FSEEMODDIR);

    let raana_bytes: Vec<u8> = if let Ok(data) = fs::read(base_path.join("other/raana")) {
        if data.is_empty() {
            return None
        } else {
            data
        }
    } else {
        return Some(false)
    };

    let files: [&str; SYNC_LEN] = [
        "bin/fseec",
        "bin/fsees",
        "lib/libutils.so",
        "other/module.base",
        "other/provider.apk",
        "script/state.sh",
        "script/util_functions.sh",
        if base_path.join("action.sh").exists() {
            "action.sh"
        } else {
            "script/action.sh"
        },
        "post-fs-data.sh",
        "service.sh",
        "uninstall.sh"
    ];

    let mut rebuild_checksum = ArrayString::<{SYNC_LEN * 64}>::new();

    for file in files {
        let mut file = if let Ok(exists_continue) = File::open(base_path.join(file)) {
            exists_continue
        } else {
            return Some(false)
        };

        let mut hasher = Hasher::new();
        let mut buffer = [0u8; 4096];
        loop {
            let size = match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(success) => success,
                _ => return Some(false)
            };
            hasher.update(&buffer[..size]);
        }

        rebuild_checksum.push_str(
            &hasher.finalize().to_hex()
        )
    }

    let mut public_key_bytes = [0u8; 32];
    public_key_bytes[0..16].copy_from_slice(&raana_bytes[16..32]);
    public_key_bytes[16..32].copy_from_slice(&raana_bytes[64..80]);

    let final_data = rebuild_checksum.as_bytes();

    let mut sign_bytes = [0u8; 64];
    sign_bytes[0..16].copy_from_slice(&raana_bytes[0..16]);
    sign_bytes[16..48].copy_from_slice(&raana_bytes[32..64]);
    sign_bytes[48..64].copy_from_slice(&raana_bytes[80..96]);

    let final_sign = Signature::new(sign_bytes);

    Some(
        PublicKey::new(public_key_bytes).verify(&final_data, &final_sign).is_ok()
    )
}