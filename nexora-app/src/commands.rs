//! 类型化 Tauri 命令：应用壳对外暴露的最小能力边界。

use serde::Serialize;

/// 平台信息：验证 Rust ↔ 前端 IPC 链路的类型化返回。
#[derive(Debug, Serialize)]
pub struct PlatformInfo {
    /// 操作系统名称。
    pub os: &'static str,
    /// CPU 架构。
    pub arch: &'static str,
}

/// 返回当前平台信息（仅用于验证 IPC 链路，无业务含义）。
#[tauri::command]
#[must_use]
pub fn platform_info() -> PlatformInfo {
    PlatformInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
    }
}
