//! Platform 端口：文件系统、进程与系统能力契约（Tauri 之上可替换）。

/// 平台能力契约：当前为骨架，功能方法随迭代添加。
pub trait Platform: Send + Sync {}
