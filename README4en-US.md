# Forge Store Enhancer Extreme
Enhancer of ForgeStore, Extreme hiding of detection points from unlocking bootloader.

> [!TIP]  
> 「[简体中文](README.md)」

> [!IMPORTANT]  
> This module **specializes** in disguising the bootloader status, **rather than** passed Play Integrity.

## Requirements
- Has been installed one of them
  - [ForgeStore](https://github.com/raana-inf/ForgeStore)
  - [TrickyStoreOSS](https://github.com/beakthoven/TrickyStoreOSS)
  - [TrickyStore](https://github.com/5ec1cff/TrickyStore)
  - [TEESimulator-RS](https://github.com/Enginex0/TEESimulator-RS)
  - [TEESimulator(<= 3.2)](https://github.com/JingMatrix/TEESimulator)
- Mount system not OverlayFS

## Install
1. Flash this module and reboot.
2. Manual configuration (optional).
3. Enjoy!

## Features
### Main
- `libc::inotify*` real-time monitoring
  - Add a remove tag / Force delete to conflict module; Directly uninstall the conflict app when detected
  - Take over the ForgeStore module target.txt, with priority over any similar modules
- At device startup
  - Sync Custom Security Patch Level to prop
  - Get Correct VerifiedBootHash prop and reset
  - Reset bootloader unlock state Related prop to locked

### Other
- Avoid abnormal environments
- Display detailed dashboard in module description, Example:  
  `[MainModule: ✅ForgeStore (170), RootImplement: ✅APatch (11224), Integrity: ✅Verified, Service: ✅Running]`  
  `[MainModule: ❌Multiple - ForgeStore (170) | OhMyKeymint (157) | TEESimulator (34) | TrickyStoreOSS (155), RootImplement: ❌Multiple - APatch (11224) | KernelSU(32525), Integrity: ⚠️This build is unsigned, Service: ❌All service will not start]`
- Display zh-Hans or en-US based on the system language: User-visible part
  - Force en-US: Create `/data/adb/fs_enhancer_extreme/config/setting/force_english` empty file

### WebUI
- Provides Google Hardware Attestation Root Certificate signing keybox
- STUB

### CLI
- Execute in the terminal as root `/data/adb/modules/fs_enhancer_extreme/bin/fseec`
- Command List
  - Operation Forge Store service
    - `fsctl` `restart|start|stop|state`
  - Operation FS Enhancer Extreme service
    - `fseectl` `restart|start|stop|state`
  - Check running environment if normal from envcollect cache
    - `envcheck`
  - Check and directly uninstall conflict apps
    - `appcheck`
  - Check and add remove tag or force delete conflict modules
    - `modcheck/-b|--boot`
  - Through Bootloader unlock related prop detection
    - `passprop`
  - Automatically correct abnormal VerifiedBootHash prop
    - `passvbhash/-b|--boot`
  - Launch standalone WebUI app to id fs_enhancer_extreme
    - `startwebui`
  - Sync Security Patch Level from security_patch.txt to prop
    - `spsyncprop/-b|--boot`
  - Detect and cache all necessity runtime environments
    - `envcollect`
  - Refresh module decription line from envcollect cache
    - `descrefresh/-d|--debug`
  - Refresh main module scope list from user config
    - `listrefresh`
  - Keybox Manager
    - `keybox` `builtin|import<<<path`

> [!NOTE]
> ### WebUI Launch
>   - **KernelSU or APatch**
>     - Native support
>   - **Magisk** 
>     - Provide action button to navigate to [WebUI X Portable](https://github.com/MMRLApp/WebUI-X-Portable) or [KSUWebUIStandalone](https://github.com/5ec1cff/KsuWebUIStandalone)

## Build
### Environment
- JDK 21
- Node.js 24
- Android SDK
- Android NDK 29.0.14206865
- Rust Nightly 2026-01-01
- cargo-ndk

### Packaging
- Execute in the terminal `./gradlew zip`

## Translation
// Go to `fseew/src/json/` add `language.json` file and then create [Pull requests](https://github.com/XtrLumen/FS-Enhancer-Extreme/pulls) to contribute the WebUI translation.

## Feedback
Log directory path: `/data/adb/fs_enhancer_extreme/log|log.old`  
  Create [Issues](https://github.com/XtrLumen/FS-Enhancer-Extreme/issues) and attach log.

## Acknowledgement
- [Fluent2](https://storybooks.fluentui.dev/web-components)
  - webui Design System Library
- [APatch](https://github.com/bmax121/APatch)
  - fseec `cli.rs` References
- [NeoZygisk](https://github.com/JingMatrix/NeoZygisk)
  - fseec `magisk.rs` References
- [Android-Open-Source-Project](https://cs.android.com/android/platform/superproject)
  - fseec `util_functions.rs` `fn pidof` References
- [KeyAttestation](https://github.com/vvb2060/KeyAttestation)
  - VBMetaProvider `attestation/*` Direct Source

## Other
### Project address (For users downloading from sources other than GitHub to trace back from this README)
- https://github.com/XtrLumen/FS-Enhancer-Extreme

### Just for fun