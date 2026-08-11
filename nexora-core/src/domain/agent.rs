//! Agent：与用户协作的 AI 协作者，复用类型化产品能力。

/// Agent 的唯一标识。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentId(pub String);

/// Agent：与用户协作的 AI 协作者。
#[derive(Debug, Clone)]
pub struct Agent {
    /// 唯一标识。
    pub id: AgentId,
    /// 显示名称。
    pub name: String,
}
