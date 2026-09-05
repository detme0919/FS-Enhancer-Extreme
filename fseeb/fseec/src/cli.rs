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
        VERSION_NAME,
        VERIFY,
        ENV_ABNORMAL
    },
    envcollect,
    api,
    bridge,
    conflict,
    ctl,
    description,
    keybox,
    passprop,
    securitypatch,
    verifiedboothash,
    scope,
    webui
};

use std::process;

use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Subcommand)]
enum Display {
    /// Return parsed version
    Pv,
    /// Return main module
    Mm,
    /// Return root implement
    Ri,
    /// Return integrity status
    Is,
    /// Return selinux status
    Ss,
    /// Return android version
    Av,
    /// Return device architecture
    Ac,
    /// Return kernel version
    Kv,
    /// Return system fingerprint
    Sf,
}

#[derive(Subcommand)]
enum SettingManager {
    /// Delete setting file
    On,
    /// Create setting file
    Off,
    /// Check setting file if exists
    Get
}

#[derive(Subcommand)]
enum ScopeOperate {
    /// Return system packages list
    Sys,
    /// Return third party packages list
    Usr,
    /// Whitelist to blacklist
    Wtb,
    /// Blacklist to whitelist
    Btw
}

#[derive(Subcommand)]
enum SettingOperate {
    /// Scope blacklist operate
    Blacklist {
        #[command(subcommand)]
        command: SettingManager
    },
    /// Boot process invoke passvbhash arg operate
    Vbhpass {
        #[command(subcommand)]
        command: SettingManager
    },
    /// Boot process invoke spsyncprop arg operate
    Spsync {
        #[command(subcommand)]
        command: SettingManager
    },
    /// Service invoke appcheck arg operate
    Appcheck {
        #[command(subcommand)]
        command: SettingManager
    },
    /// Service start modcheck thread operate
    Modcheck {
        #[command(subcommand)]
        command: SettingManager
    }
}

#[derive(Subcommand)]
enum Api {
    /// Necessity information
    Info {
        #[command(subcommand)]
        command: Display
    },
    /// Package list
    Scope {
        #[command(subcommand)]
        command: ScopeOperate
    },
    /// Operate api
    Setting {
        #[command(subcommand)]
        command: SettingOperate
    }
}

#[derive(Subcommand)]
enum Ctl {
    /// Restart service process
    Restart,
    /// Start   service process
    Start,
    /// Stop    service process
    Stop,
    /// Get     service running status
    State,
}

#[derive(Args)]
pub struct Mode {
    /// Patterns for boot process
    #[arg(short, long)]
    pub boot: bool
}

#[derive(Subcommand)]
enum KeyboxManager {
    /// Use built-in keybox
    Builtin,
    /// Use external keybox
    Import {
        /// Path to external keybox file
        path: String
    }
}

/// FS Enhancer Extreme CLI
#[derive(Parser)]
#[command(version = VERSION_NAME)]
enum Command {
    /// For webui invoke
    Api {
        #[command(subcommand)]
        command: Api
    },
    /// Operation Forge Store service
    Fsctl {
        #[command(subcommand)]
        command: Ctl
    },
    /// Operation FS Enhancer Extreme service
    Fseectl {
        #[command(subcommand)]
        command: Ctl
    },
    /// Check running environment if normal from envcollect cache
    Envcheck,
    /// Check and directly uninstall conflict apps
    Appcheck,
    /// Check and add remove tag or force delete conflict modules
    Modcheck(Mode),
    /// Through Bootloader unlock related prop detection
    Passprop,
    /// Automatically correct abnormal VerifiedBootHash prop
    Passvbhash(Mode),
    /// Launch standalone WebUI app to id fs_enhancer_extreme
    Startwebui,
    /// Sync Security Patch Level from security_patch.txt to prop
    Spsyncprop(Mode),
    /// Detect and cache all necessity runtime environments
    Envcollect,
    /// Refresh module decription line from envcollect cache
    Descrefresh(description::Mode),
    /// Refresh main module scope list from user config
    Listrefresh,
    /// Keybox Manager
    Keybox {
        #[command(subcommand)]
        command: KeyboxManager
    }
}

fn abnormal_exit() {
    if *ENV_ABNORMAL {
        println!("Abnormal Environment");
        process::exit(1)
    }
}

macro_rules! promise {
    ($function:path) => ({
        $function();

        Ok(())
    });
    ($function:path, $arg:expr) => ({
        $function($arg);

        Ok(())
    })
}

macro_rules! guard {
    (match $($command:tt)*) => ({
        abnormal_exit();
        match $($command)*
    });
    ($function:path) => ({
        abnormal_exit();
        $function()
    });
    ($function:path, $arg:expr) => ({
        abnormal_exit();
        $function($arg)
    })
}

pub fn entry() -> anyhow::Result<()> {
    let args = Command::parse();

    if !matches!(args, Command::Api {..} | Command::Envcheck | Command::Envcollect | Command::Descrefresh(..)) && *VERIFY == Some(false) {
        bridge::sigsegv()
    }

    match args {
        Command::Api {command} => match command {
            Api::Info {command} => match command {
                Display::Pv => api::version_name(),
                Display::Mm => promise!(api::main_module),
                Display::Ri => promise!(api::root_implement),
                Display::Is => promise!(api::integrity_status),
                Display::Ss => api::getenforce(),
                Display::Av => api::android_version(),
                Display::Ac => promise!(api::device_arch),
                Display::Kv => api::kernel_version(),
                Display::Sf => promise!(api::fingerprint)
            }
            Api::Scope {command} => match command {
                ScopeOperate::Sys => api::system_package(),
                ScopeOperate::Usr => api::user_package(),
                ScopeOperate::Wtb => scope::whitelist_to_blacklist(),
                ScopeOperate::Btw => scope::blacklist_to_whitelist(),
            }
            Api::Setting {command} => match command {
                SettingOperate::Blacklist {command} => match command {
                    SettingManager::On => promise!(scope::blacklist_on),
                    SettingManager::Off => promise!(scope::blacklist_off),
                    SettingManager::Get => scope::blacklist_get()
                }
                SettingOperate::Vbhpass {command} => match command {
                    SettingManager::On => promise!(verifiedboothash::vbhpass_on),
                    SettingManager::Off => promise!(verifiedboothash::vbhpass_off),
                    SettingManager::Get => verifiedboothash::vbhpass_get()
                }
                SettingOperate::Spsync {command} => match command {
                    SettingManager::On => promise!(securitypatch::spsync_on),
                    SettingManager::Off => promise!(securitypatch::spsync_off),
                    SettingManager::Get => securitypatch::spsync_get()
                }
                SettingOperate::Appcheck {command} => match command {
                    SettingManager::On => promise!(conflict::appcheck_on),
                    SettingManager::Off => promise!(conflict::appcheck_off),
                    SettingManager::Get => conflict::appcheck_get()
                }
                SettingOperate::Modcheck {command} => match command {
                    SettingManager::On => promise!(conflict::modcheck_on),
                    SettingManager::Off => promise!(conflict::modcheck_off),
                    SettingManager::Get => conflict::modcheck_get()
                }
            }
        }
        Command::Fsctl {command} => guard!(
            match command {
                Ctl::Restart => promise!(ctl::fs_restart),
                Ctl::Start => promise!(ctl::fs_start),
                Ctl::Stop => promise!(ctl::fs_stop),
                Ctl::State => promise!(ctl::fs_state)
            }
        ),
        Command::Fseectl {command} => match command {
            Ctl::Restart => promise!(ctl::fsee_restart),
            Ctl::Start => promise!(ctl::fsee_start),
            Ctl::Stop => promise!(ctl::fsee_stop),
            Ctl::State => promise!(ctl::fsee_state)
        }
        Command::Envcheck => promise!(abnormal_exit),
        Command::Appcheck => conflict::app_process(),
        Command::Modcheck(mode) => promise!(conflict::route, mode),
        Command::Passprop => passprop::entry(),
        Command::Passvbhash(mode) => guard!(verifiedboothash::pass, mode),
        Command::Startwebui => guard!(webui::start),
        Command::Spsyncprop(mode) => guard!(securitypatch::sync, mode),
        Command::Envcollect => promise!(envcollect::entry),
        Command::Descrefresh(mode) => description::refresh(mode),
        Command::Listrefresh => guard!(scope::refresh),
        Command::Keybox {command} => guard!(
            match command {
                KeyboxManager::Builtin => keybox::extract(),
                KeyboxManager::Import {path} => keybox::transfer(path)
            }
        )
    }
}