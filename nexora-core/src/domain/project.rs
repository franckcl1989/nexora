//! Project：用户定义的工程上下文集合，工程活动发生的容器。

/// Project 的唯一标识。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectId(pub String);

/// Project：一个或多个代码仓库加上相关环境描述的工程上下文集合。
#[derive(Debug, Clone)]
pub struct Project {
    /// 唯一标识。
    pub id: ProjectId,
    /// 显示名称。
    pub name: String,
}
