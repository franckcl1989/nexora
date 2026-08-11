# Nexora

AI 原生、跨平台、本地桌面的工程工作台：服务研发、测试、联调、调试、诊断与运维等真实软件工程活动。本词汇表定义工程工作台的统一领域语言。

## 核心概念

**Project**:
用户定义的工程上下文集合：一个或多个代码仓库，加上相关环境描述。Project 是工程活动发生的容器，也是 AI 上下文的最小边界。
_Avoid_: Workspace（避免与 Cargo workspace 混淆）、Repository（仅指单个代码仓库）

**Activity**:
Project 内一次具体的工程活动（如一次调试会话、一次诊断）。有明确的开始与结束，可被记录与恢复。
_Avoid_: Task（与通用任务语义混淆）、Session（含义过泛）

**Agent**:
与用户协作的 AI 协作者。Agent 复用类型化产品能力，不是独立的可信执行主体。
_Avoid_: Assistant（弱化产品语义）、Chatbot

**Records**:
Activity 产生的结构化活动记录。可回放、复现、搜索，是产品"可记录复现"主张的载体。
_Avoid_: History（隐含被动删除）、Logs（仅指原始日志）

**Platform**:
工程工作台的系统能力层：权限、配置、持久化、跨平台系统能力。
_Avoid_: Infrastructure（范围过宽）
