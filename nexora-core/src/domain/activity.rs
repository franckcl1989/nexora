//! Activity：Project 内一次具体的工程活动（如一次诊断）。

/// Activity 的唯一标识。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivityId(pub String);

/// Activity：有明确开始与结束的工程活动，可被记录与恢复。
#[derive(Debug, Clone)]
pub struct Activity {
    /// 唯一标识。
    pub id: ActivityId,
    /// 所属 Project。
    pub project_id: crate::domain::project::ProjectId,
    /// 活动标题。
    pub title: String,
}
