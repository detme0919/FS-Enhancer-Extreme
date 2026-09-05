# Forge Store Enhancer Extreme
ForgeStore增强, 极致隐藏由解锁引导加载程序产生的检测点.

> [!TIP]  
> 「[English](README4en-US.md)」

> [!IMPORTANT]  
> 本模块**专精**伪装引导加载程序状态，**而非**通过PlayIntegrity。

## 条件
- 已安装其中之一
  - [ForgeStore](https://github.com/raana-inf/ForgeStore)
  - [TrickyStoreOSS](https://github.com/beakthoven/TrickyStoreOSS)
  - [TrickyStore](https://github.com/5ec1cff/TrickyStore)
  - [TEESimulator-RS](https://github.com/Enginex0/TEESimulator-RS)
  - [TEESimulator(<= 3.2)](https://github.com/JingMatrix/TEESimulator)
- 挂载系统不是 OverlayFS

## 安装
1. 刷入模块并重新启动设备。
2. 手动配置(可选)。
3. 完成！

## 功能
### 主要
- `libc::inotify*`实时监控
  - 对冲突模块添加移除标签/强制删除；检测到冲突应用时直接卸载
  - 接管 ForgeStore 模块 target.txt ，优先级高于任何类似模块
- 设备启动时
  - 从自定义安全补丁级别配置重设对应 prop
  - 重设引导加载程序解锁状态相关 prop 为锁定值
  - 获取正确 VerifiedBootHash 以重设对应 prop

### 其他
- 规避异常环境
- 在模块描述显示详细仪表盘，例:  
  `[主模块: ✅ForgeStore (170), 根实现: ✅APatch (11224), 完整性: ✅通过验证, 服务: ✅运行中]`  
  `[主模块: ❌多重共存 - ForgeStore (170) | OhMyKeymint (157) | TEESimulator (34) | TrickyStoreOSS (155), 根实现: ❌多重共存 - APatch (11224) | KernelSU (32525), 完整性: ⚠️本次构建未签名, 服务: ❌所有服务将不会启动]`
- 根据系统语言分别显示 zh-Hans 或 en-US : 用户可见部分
  - 强制 en-US : 创建 `/data/adb/fs_enhancer_extreme/config/setting/force_english` 空文件

### WebUI
- 提供谷歌硬件认证根证书签名的 keybox
- 占位

### 命令行工具
- 于终端以Root身份执行 `/data/adb/modules/fs_enhancer_extreme/bin/fseec`
- 命令列表
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
> ### WebUI 启动
>   - **KernelSU 或 APatch**
>     - 原生支持
>   - **Magisk**
>     - 提供跳转到 [WebUI X Portable](https://github.com/MMRLApp/WebUI-X-Portable) 或 [KSUWebUIStandalone](https://github.com/5ec1cff/KsuWebUIStandalone) 的 Action 按钮

## 构建
### 环境
- JDK 21
- Node.js 24
- Android SDK
- Android NDK 29.0.14206865
- Rust Nightly 2026-01-01
- cargo-ndk

### 打包
- 于终端执行 `./gradlew zip`

## 翻译
// 前往 `fseew/src/json/` 添加 `language.json` 文件后创建 [Pull requests](https://github.com/XtrLumen/FS-Enhancer-Extreme/pulls) 来为 WebUI 的翻译做出贡献。

## 反馈
日志目录路径: `/data/adb/fs_enhancer_extreme/log|log.old`  
  创建 [Issues](https://github.com/XtrLumen/FS-Enhancer-Extreme/issues) 并附上日志。

## 致谢
- [Fluent2](https://storybooks.fluentui.dev/web-components)
  - webui 设计系统库
- [APatch](https://github.com/bmax121/APatch)
  - fseec `cli.rs` 参考来源
- [NeoZygisk](https://github.com/JingMatrix/NeoZygisk)
  - fseec `magisk.rs` 参考来源
- [Android-Open-Source-Project](https://cs.android.com/android/platform/superproject)
  - fseec `util_functions.rs` `fn pidof` 参考来源
- [KeyAttestation](https://github.com/vvb2060/KeyAttestation)
  - VBMetaProvider `attestation/*` 直接来源

## 其他
### 本项目地址(用于非Github下载的用户从本自述文件溯源)
- https://github.com/XtrLumen/FS-Enhancer-Extreme

### Just for fun