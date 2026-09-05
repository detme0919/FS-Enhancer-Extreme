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
    util_functions::{
        getprop,
        read_to_string,
        write
    },
    bridge,
    envcollect
};

use std::{
    path::Path,
    sync::LazyLock
};

pub const CONFLICT_APP: &[&str] = &[
    "com.lingqian.appbl",
    "com.topmiaohan.hidebllist"
];

pub const TAG_CONFLICT_MOD: &[&str] = &[
    "Yurikey",
    "xiaocaiye",
    "vbmeta-fixer",
    "safetynet-fix",
    "playintegrity",
    "integrity_box",
    "SukiSU_module",
    "ShamikoManager",
    "Reset_BootHash",
    "Tricky_store-bm",
    "Hide_Bootloader",
    "extreme_hide_root",
    "ts_enhancer_extreme",
    "Tricky_Store-xiaoyi",
    "tricky_store_assistant",
    "extreme_hide_bootloader",
    "wjw_hiderootauxiliarymod"
];

pub const DEL_CONFLICT_MOD: &[&str] = &[
    "TA_utl",
    ".TA_utl",
    "Yamabukiko"
];

pub const MODULESDIR: &str = "/data/adb/modules";
pub const MODULESUPDATEDIR: &str = "/data/adb/modules_update";

pub const LOG_TAG: &str = "cli";
pub const VERSION_NAME: &str = env!("VERSION_NAME");
pub const BUILTIN_KEYBOX: &[u8] = include_bytes!("../asset/keybox.xml");

pub const DISABLE: &str = "Disable";
pub const UNKNOWN: &str = "Unknown";

pub const FORGE_STORE: &str = "ForgeStore";
pub const TRICKY_STORE: &str = "TrickyStore";
pub const OH_MY_KEYMINT: &str = "OhMyKeymint";
pub const TEESIMULATOR: &str = "TEESimulator";

pub const FSMODDIR: &str = "/data/adb/modules/forge_store";
pub const TSMODDIR: &str = "/data/adb/modules/tricky_store";
pub const OMKMODDIR: &str = "/data/adb/modules/oh_my_keymint";
pub const TEESMODDIR: &str = "/data/adb/modules/teesim";

pub const FSEEMODDIR: &str = "/data/adb/modules/fs_enhancer_extreme";

pub const MAIN_MODULE_ENV_FILE: &str = "/data/adb/fs_enhancer_extreme/main_module";
pub const ROOT_IMPLEMENT_ENV_FILE: &str = "/data/adb/fs_enhancer_extreme/root_implement";

pub const APPEND_FILE: &str = "/data/adb/fs_enhancer_extreme/config/append";
pub const REMOVE_FILE: &str = "/data/adb/fs_enhancer_extreme/config/remove";
pub const VBH_FILE: &str = "/data/adb/fs_enhancer_extreme/config/verifiedboothash";

pub const BLACK_LIST: &str = "/data/adb/fs_enhancer_extreme/config/setting/blacklist";
pub const FORCE_ENGLISH: &str = "/data/adb/fs_enhancer_extreme/config/setting/force_english";
pub const SKIP_APPCHECK: &str = "/data/adb/fs_enhancer_extreme/config/setting/skip_appcheck";
pub const SKIP_MODCHECK: &str = "/data/adb/fs_enhancer_extreme/config/setting/skip_modcheck";
pub const SKIP_SPSYNC: &str = "/data/adb/fs_enhancer_extreme/config/setting/skip_spsync";
pub const SKIP_VBHPASS: &str = "/data/adb/fs_enhancer_extreme/config/setting/skip_vbhpass";

pub struct Environment {
    pub multiple: bool,
    pub identity: String,
    pub version: u32
}

impl Environment {
    pub fn export(env_file: &str, env: Environment) {
        write(env_file, format!("{}\n{}\n{}", env.multiple, env.identity, env.version), false);

        println!(
            "{{Multiple: \"{}\", Identity: \"{}\", VersionCode: \"{}\"}}",
            env.multiple, env.identity, env.version,
        )
    }
    fn import(env_file: &str) -> Self {
        let content = if let Ok(exists_continue) = read_to_string(env_file) {
            exists_continue
        } else {
            envcollect::entry();
            read_to_string(env_file).unwrap()
        };

        Self {
            multiple: content.lines().nth(0)
                .unwrap().parse()
                .unwrap(),
            identity: content.lines().nth(1)
                .unwrap().to_string(),
            version: content.lines().nth(2)
                .unwrap().parse()
                .unwrap()
        }
    }
}

pub static ROOT_IMPLEMENT: LazyLock<Environment> = LazyLock::new(||
    Environment::import(ROOT_IMPLEMENT_ENV_FILE)
);
pub static MAIN_MODULE: LazyLock<Environment> = LazyLock::new(||
    Environment::import(MAIN_MODULE_ENV_FILE)
);

pub static VERIFY: LazyLock<Option<bool>> = LazyLock::new(||
    bridge::verify()
);

pub static ENV_ABNORMAL: LazyLock<bool> = LazyLock::new(||
    *VERIFY == Some(false) || MAIN_MODULE.multiple || matches!(MAIN_MODULE.identity.as_str(), UNKNOWN | DISABLE | OH_MY_KEYMINT | TEESIMULATOR)
);

pub static FINAL_MAIN_MODULE_CONFIG: LazyLock<&str> = LazyLock::new(||
    if MAIN_MODULE.identity == FORGE_STORE {
        "/data/adb/forge_store"
    } else {
        "/data/adb/tricky_store"
    }
);

pub static IS_ZHCN: LazyLock<bool> = LazyLock::new(||
    !Path::new(FORCE_ENGLISH).exists() && (getprop("persist.sys.locale").contains("zh") || getprop("ro.product.locale").contains("zh"))
);

pub static DESC_CONFLICT_MOD: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "此模块与 FS-Enhancer-Extreme 证实冲突, 已被添加移除标签, 将在设备下一次启动时被移除."
    } else {
        "This module has been confirmed to conflict with the FS-Enhancer-Extreme. Has been tagged for remove, Will be removed upon the devide next boot."
    }
);
pub static DESC_BASE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "ForgeStore 增强, 极致隐藏由解锁引导加载程序产生的检测点."
    } else {
        "Enhancer of ForgeStore, Extreme hiding of detection points from unlocking bootloader."
    }
);

pub static DESC_MAIN_MODULE_NOT_INSTALL: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "未安装"
    } else {
        "Not installed"
    }
);
pub static DESC_MULTIPLE_PREFIX: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "多重共存"
    } else {
        "Multiple"
    }
);
pub static DESC_DISABLE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "被禁用"
    } else {
        "Disabled"
    }
);

pub static DESC_MAIN_MODULE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "主模块: "
    } else {
        "MainModule: "
    }
);
pub static DESC_ROOT_IMPL: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "根实现: "
    } else {
        "RootImplement: "
    }
);
pub static DESC_INTEGRITY: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "完整性: "
    } else {
        "Integrity: "
    }
);
pub static DESC_SERVICE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "服务: "
    } else {
        "Service: "
    }
);

pub static DESC_INTEGRITY_SUCCESS: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "通过验证"
    } else {
        "Verified"
    }
);
pub static DESC_INTEGRITY_WARNING: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "本次构建未签名"
    } else {
        "This build is unsigned"
    }
);
pub static DESC_INTEGRITY_ERROR: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "遭到篡改"
    } else {
        "Tampered with"
    }
);

pub static DESC_SERVICE_SUCCESS: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "运行中"
    } else {
        "Running"
    }
);
pub static DESC_SERVICE_FAILURE: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "无法启动"
    } else {
        "Cannot start"
    }
);
pub static DESC_SERVICE_NOT_START: LazyLock<&str> = LazyLock::new(||
    if *IS_ZHCN {
        "所有服务将不会启动"
    } else {
        "All service will not start"
    }
);