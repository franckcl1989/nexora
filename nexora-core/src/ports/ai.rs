//! AI 端口：AI 模型提供商契约（rig/genai 等可替换）。

/// AI 提供商契约：当前为骨架，功能方法随迭代添加。
pub trait AiProvider: Send + Sync {}
