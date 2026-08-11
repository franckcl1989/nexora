//! Persistence 端口：存储适配契约（实现方如 `SQLite` 等可替换）。

/// 持久化契约：当前为骨架，功能方法随迭代添加。
pub trait Persistence: Send + Sync {}
