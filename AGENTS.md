# AGENTS.md

## 1. 项目定位与最高裁决

本项目是一个 **AI 原生、跨平台、本地桌面的工程工作台**，服务研发、测试、联调、调试、诊断和运维等真实软件工程活动。

功能可以广泛，但必须形成统一工程体验，而不是无边界堆积功能。

本项目不是：

- 通用运行时平台；
- 第三方插件平台；
- 插件市场；
- 通用自动化平台。

技术服务于产品。不得因为 Rust、Tauri、WebAssembly 或 AI 具有通用能力，就主动扩大产品边界。

本文件是项目的**最高开发裁决规范**。README 负责项目介绍；详细架构、领域设计和专题方案放入 `docs/` 等文档。

除非用户明确修改已冻结规则，否则所有设计、选型、依赖、编码、重构和接口调整都必须遵守本文件。

---

## 2. 全量交叉审计

任何工程变更都必须与全部既有规则和冻结结论一起审计：

1. **哲学统一**：符合项目长期价值取向；
2. **语义一致**：同一概念保持唯一、稳定含义；
3. **逻辑自洽**：新旧设计之间不存在未解决冲突；
4. **真实有效**：建立在真实技术能力和工程约束上；
5. **完整可靠**：正常、异常、资源和生命周期路径完整；
6. **技术栈原生**：符合所采用语言、框架、协议和生态本身的设计哲学、惯用法及官方最佳实践。

项目规则可以比上游技术更严格，例如第一方禁止 unsafe；但不得无理由逆着上游生态设计。

确有产品级理由需要偏离惯用方案时，必须：

- 明确知道正在偏离；
- 说明真实收益；
- 将偏离限制在最小范围；
- 不把个人习惯包装成最佳实践。

发现冲突必须先解决。禁止通过第二套机制、长期兼容层、特殊分支、silent fallback 或永久 feature flag 掩盖根本矛盾。

---

## 3. 未明确问题的决策顺序

遇到本文件没有直接答案的问题时：

1. 明确真实需求和产品语义；
2. 阅读相关代码、配置和专题文档；
3. 检查已有实现与冻结结论；
4. 识别所用技术栈的原生解决方式；
5. 优先寻找成熟生态能力；
6. 对易变技术核对当前官方一手资料；
7. 比较安全、性能、跨平台性、维护性和长期方向；
8. 检查是否产生重复语义、无价值抽象或平台化设计；
9. 完成全量交叉审计；
10. 再实施整体最优方案。

不得为了尚不存在的未来需求提前建设复杂基础设施。

---

## 4. 技术策略：前沿优先，价值驱动

项目主动采用代表未来方向的新技术、协议和组件。

实验性、预览版和快速演进技术，只要在架构语义、性能、安全、开发体验或长期方向上提供明确价值，就可以主动采用。

激进不等于无条件追逐最大版本号。

快速演进的关键依赖应：

- 必要时精确锁定版本；
- 用薄边界隔离不稳定 API；
- 避免实验 API 污染核心领域模型；
- 主动跟踪和升级；
- 升级后执行真实验证。

当前核心技术方向包括：

- Rust；
- Tauri；
- WebAssembly Component Model；
- WASI 0.3 与现代异步 Component 语义；
- AI 原生产品能力。

尚未冻结的具体 crate、前端框架、AI 框架、模型提供商和数据库等不得被 Agent 擅自提升为长期架构承诺。

---

## 5. Rust 必须按 Rust 的方式设计

所有第一方 Rust 代码必须遵循 Rust 的语言哲学和惯用设计，而不是机械搬用 Java、C++、Go、TypeScript 或传统企业框架模式。

优先使用：

- ownership 和 borrowing 表达资源关系；
- RAII 管理资源生命周期；
- `Result` / `Option` 表达失败与缺失；
- enum 表达有限状态；
- newtype 表达领域身份和约束；
- trait 表达真实共享行为和抽象边界；
- 泛型、关联类型和 `impl Trait` 表达静态多态；
- 模块可见性表达封装；
- 类型系统使非法状态尽量难以表达。

### 所有权优先

不要为了绕过 borrow checker 下意识：

- `.clone()`；
- `Arc`；
- `Mutex`；
- `RwLock`；
- interior mutability。

先判断正确的 ownership 和 borrowing 关系。

当共享所有权、并发共享或内部可变性确实属于问题语义时，应自然使用对应 Rust 原语，不得为了形式上的“零成本”制造反而更复杂的设计。

### Concrete type 优先

不要为每个实现机械创建 `FooService` / `FooServiceImpl` 一类接口与实现配对。

默认从具体类型开始。

只有确实存在：

- 多实现抽象；
- 通用行为契约；
- 编译期多态需求；
- 真实运行时多态需求；

时才引入 trait。

不得仅为了 mocking 创建没有产品或架构语义的 trait。

### 类型优先

禁止可以用明确类型表达的业务语义退化为：

- magic string；
- magic number；
- boolean flag soup；
- stringly typed 状态机。

公共和重要内部 API 应遵循 Rust API Guidelines 的命名、类型安全、错误语义和可预测性原则。

---

## 6. 第一方 Rust 100% Safe

所有第一方 Rust，包括正常源码、测试、示例和 build script，都不得包含 unsafe Rust。

第一方 crate 应启用：

```rust id="yw72gs"
#![forbid(unsafe_code)]
```

禁止第一方：

- `unsafe` block；
- `unsafe fn`；
- `unsafe trait`；
- 裸 FFI；
- 自行操作裸指针；
- 自行承担 syscall 等 unsafe 边界。

**没有例外。**

底层能力必须依赖 unsafe 时，优先采用成熟、维护可靠、提供安全抽象的第三方库。

第三方依赖内部可以使用经过封装和审查的 unsafe，但 unsafe 边界不得泄漏到第一方代码。

---

## 7. 强制 Library-first

任何通用能力在第一方实现之前，都必须先寻找并评估成熟生态方案。

只要已有库：

- 语义真正匹配；
- 维护可靠；
- 目标平台支持正确；
- API 符合对应生态惯例；
- 不违反 Safe Rust；
- 不引入与收益不相称的运行时机制；
- 不突破产品边界；

就必须优先使用。

即使自行实现只有极少代码，也不能仅以“简单”为理由重复造轮子。

依赖数量、源码行数和二进制体积本身不是优化目标。

第一方代码主要承担：

- 产品语义；
- 领域模型；
- 组合；
- 策略；
- 权限与约束；
- 必要胶水。

成熟库缺少某个 backend 时，依次优先考虑：

1. 官方扩展点；
2. 官方或成熟 adapter；
3. 其他成熟实现；
4. 向上游贡献；
5. 最后才考虑第一方实现。

禁止自行重新实现成熟生态已经解决的协议栈、解析器、密码学、AI 基础设施、编辑器、终端模拟器等通用能力。

---

## 8. 零成本抽象

抽象不得在底层操作本来需要的成本之外增加没有真实产品价值的运行时工作。

默认优先：

- 直接调用；
- 静态分发；
- 单态化；
- 编译期生成；
- 类型系统表达；
- 内联；
- RAII。

禁止仅为了“解耦”“扩展性”或架构形式无意义引入：

- 动态分发；
- 内部 JSON 序列化；
- 内部 HTTP/RPC；
- 消息总线；
- 通用事件总线；
- 字符串路由；
- service locator；
- 运行时注册中心；
- 多余 channel；
- 多余 allocation；
- 无必要 sidecar。

Tauri IPC、Component ABI、真实动态选择和真实异步通信属于问题本身要求时可以接受的成本。

零成本原则不得反过来破坏 Rust 惯用设计。

如果 `Arc`、trait object、allocation 或 channel 正确表达真实问题，就应使用，而不是为了形式上的静态化制造更复杂代码。

每增加一层抽象都必须能够回答：

> 它增加了什么运行时或认知成本？解决了什么真实问题？

回答不清楚时不得引入。

---

## 9. 单一语义源与单一所有权

**同一个业务概念只能有一个权威语义源、一个权威业务规则实现和一个权威状态所有者。**

不得因为存在 Rust、前端、AI、WASM、数据库或其他边界，就手工维护多套表达相同业务含义的模型、校验和状态。

核心领域模型不得依赖：

- Tauri 类型；
- WebView 类型；
- Wasmtime 类型；
- 生成的 WIT binding；
- AI provider SDK 类型；
- 数据库 driver 类型。

技术框架属于 adapter 边界，不得向内污染领域语义。

### Rust ↔ 前端

同一跨端契约必须选择一个权威来源，并由成熟工具生成或派生另一侧定义。

禁止长期人工同步：

```text id="9k8nbg"
Rust struct
+
TypeScript interface
+
手写 IPC DTO
```

前端可以拥有布局、选中、展开、焦点、光标、未提交表单等纯 UI ViewModel，但不得重新定义业务真相。

### Rust ↔ AI

AI Tool 的输入、输出和约束必须来自现有类型化产品能力。

禁止为 AI 单独维护第二套 DTO、校验和业务实现。

### Rust ↔ WASM

WIT 是 Component ABI 的权威契约。

Host / Guest bindings 应由生成工具产生，不得手工复制 WIT ABI。

WIT 不替代产品领域模型；领域模型与 ABI 类型具有真实差异时，通过显式 adapter 转换。

### 状态

每一类可变业务状态必须有唯一 authoritative owner。

禁止 Rust 与前端分别维护可独立修改的同一业务状态，再依靠双向同步维持一致。

缓存和派生状态可以存在，但必须能够从权威状态重新得到。

生成代码不得手工修改。

---

## 10. 边界必须薄、明确、类型化

Tauri command、AI Tool adapter、WIT adapter、数据库 adapter 等边界只负责：

- 输入解析；
- 边界特有验证；
- 类型转换；
- 调用权威产品能力；
- 输出转换。

业务规则不得散落在 adapter 中。

同一进程能直接类型化调用时，不得改成内部 JSON、RPC、事件消息或字符串 command。

跨边界接口应具有合理产品粒度，避免大量聊天式细碎调用。

大量或连续数据优先使用真正的 streaming，而不是高频轮询或无界累积。

错误在领域内部保持结构化，到展示边界才转换为用户消息；禁止业务逻辑依赖错误字符串匹配。

---

## 11. AI 原生原则

AI 是产品中的第一等能力，不是附加聊天框。

人与 AI 必须复用同一套类型化产品能力。

禁止为同一业务操作维护 UI 实现和 AI 实现两套路径。

AI Tool 应是既有产品 Action/能力的薄适配。

### AI 不是可信执行主体

模型输出属于**不确定、非可信的外部输入**。

任何：

- Tool arguments；
- structured output；
- identifier；
- path；
- command；
- resource selection；

都必须经过与普通用户输入相同的权威类型、领域和安全校验。

模型永远不是：

- 权限事实源；
- 安全策略执行者；
- Secret 授权者；
- 数据真实性证明；
- destructive action 的最终授权依据。

具有副作用或风险的 AI 动作必须经过统一策略和必要的人类确认。

### AI Infrastructure Library-first

不得自行重新实现成熟生态已经覆盖的：

- provider integration；
- agent loop；
- tool calling；
- structured output；
- streaming；
- MCP；
- 通用 session / agent infrastructure。

只有在产品确有需求时才建立 provider abstraction，不得为了理论上的多模型兼容提前设计最低公分母 AI 层。

### AI Context

发送给模型的上下文遵循最小必要原则。

不得默认发送：

- 整个工作区；
- 无关文件；
- 全量历史；
- Secret；
- token；
- credential；
- 私钥。

Prompt、tool description、system instruction、模型参数等会改变 AI 行为的内容属于产品行为资产，应纳入版本管理。

重大 AI 行为变更除普通测试外，还必须使用可重复的代表性场景进行评估，禁止凭单次对话感觉宣布效果提升。

---

## 12. Tauri 必须按其安全模型使用

Tauri 的 WebView 与 Rust Core 是真实的信任边界。

前端不得被视为拥有宿主权限的可信核心。

Rust Core 持有系统能力，并通过最小化、类型化的 Tauri command 暴露必要功能。

禁止设计万能入口，例如：

```text id="kzpf8j"
execute(action: string, payload: json)
```

来绕过明确的 capability 和类型边界。

必须使用 Tauri 的：

- capability；
- permission；
- scope；

将 WebView 能调用的宿主能力限制在真实需要范围。

远程内容不得无必要加载进拥有高权限能力的 WebView。

确需远程内容时必须隔离信任边界并最小化能力。

CSP 应尽量严格；不得为了开发方便长期放宽安全策略。

前端主要承担：

- 展示；
- 用户交互；
- 页面组合；
- UI-only 状态。

产品领域逻辑、安全规则和系统权限不得因为实现方便迁移到前端。

---

## 13. WebAssembly / Component / WASI 原则

WebAssembly Component 是**内部实现机制**，不是第三方插件体系。

不得为了“模块化”强制所有功能 WASM 化。

只有当以下能力产生真实价值时才使用 Component：

- 隔离；
- 可移植性；
- 独立执行；
- capability boundary；
- 组件组合；
- 明确的跨语言契约。

新 Component 优先采用 Component Model、WASI 0.3 和现代 async 语义。

需要异步、持续数据或延迟结果时，应优先采用与语义匹配的：

- `async func`；
- `stream<T>`；
- `future<T>`；

而不是在 WIT 上重新发明轮询、JSON-RPC 或事件协议。

标准 WASI 已经提供的能力必须优先使用标准接口。

自定义 WIT 只承载：

- 真实产品领域能力；
- 标准 WASI 无法表达的宿主能力。

WIT 必须描述稳定语义，不得机械镜像 Rust 内部模块、struct 或 implementation detail。

接口应小而内聚，避免一个包含所有宿主能力的 God World。

具有身份和生命周期的跨 Component 对象应按 Component Model 的 resource 语义建模；纯值保持纯值。

Wasmtime 等 runtime-specific API 必须限制在宿主 adapter。

WASM guest 依赖必须以真实 target、真实 feature 和真实运行路径验证。

“支持 WASM”不能推导出“支持当前 WASI/Component 目标”。

当前没有第三方插件承诺，因此不得为了未知外部消费者提前背负稳定 ABI 和兼容包袱。

---

## 14. 跨平台与系统边界

Rust 默认承担：

- 产品领域逻辑；
- 原生系统能力；
- AI orchestration；
- WASM hosting；
- 安全与权限；
- 持久化；
- 本地与网络资源集成。

没有真实必要时不得引入 Node、Python、JVM 或其他后端 sidecar，也不得无必要引入第二套后端 runtime。

Linux、Windows 和 macOS 的真实差异必须隔离在明确的平台边界中。

禁止：

- 平台条件编译无序扩散到领域代码；
- 为统一 API 虚构某个平台不存在的能力；
- 用最低公分母抹去有意义的平台差异。

跨平台公共语义统一，真实平台差异显式。

---

## 15. 异步、并发与资源生命周期

异步设计优先遵循 structured concurrency 思想。

任何 task、connection、stream、subscription 和后台工作必须有明确所有者和生命周期。

禁止无约束 fire-and-forget 或 spawn 无人管理的长期任务。

根据真实需求处理：

- cancellation；
- timeout；
- bounded concurrency；
- backpressure；
- graceful shutdown；
- resource cleanup。

任务应随所属操作、会话或资源结束而取消或回收，除非它明确属于整个应用生命周期。

不得在 async executor 上直接执行明显的长时间阻塞操作。

共享状态不得默认设计成一个巨大的 `Arc<Mutex<AppState>>`。

先按领域和 ownership 拆分状态，再只在真实共享可变性边界使用同步原语。

处理大型 body、文件、日志、AI 输出等连续数据时，优先 streaming、有界缓冲和减少复制。

---

## 16. 错误、配置、持久化与失败语义

用户输入、网络、文件、数据库、WASM、AI provider 和外部环境产生的可恢复失败必须使用显式错误语义。

不得使用 `panic!`、`unwrap()` 或 `expect()` 处理用户或环境能够触发的正常失败。

panic 只用于真正违反不可恢复内部不变量的情况。

错误应保留：

- 根因；
- 必要上下文；
- 结构化类型。

禁止过早压扁为字符串。

### Silent fallback 禁止

要求明确的能力不可用时必须明确失败。

只有候选方案在产品语义上真正等价，并且 fallback 已被明确设计时，才允许自动回退。

不得通过 fallback 隐藏配置错误、权限错误、数据损坏、版本错误或不支持的平台能力。

### 持久化

用户持久数据属于长期资产。

不兼容 schema 或格式变更必须具有明确、可测试的迁移。

禁止：

- 静默删除旧数据；
- 解析失败后自动重置；
- 长期维护双格式以逃避迁移；
- 无版本格式无限演化。

### 配置

每个配置语义必须具有明确事实源。

存在多来源配置时，precedence 必须显式设计和测试，不得形成隐藏 fallback 和不可解释覆盖链。

---

## 17. 安全、隐私与可观测性

所有能力遵循最小权限。

Secret、token、credential 和私钥：

- 不进入普通日志；
- 不无必要明文持久化；
- 不默认发送给远程 AI；
- 不通过 debug、panic 或错误信息泄漏。

密码学、TLS、证书验证和凭据存储必须采用成熟安全库。

禁止自行实现密码学原语。

重要后台任务、跨边界操作和失败路径必须具有结构化可观测性。

长期代码不得依赖零散 `println!` 作为诊断机制。

Tracing / logging 应能够在需要时关联：

- operation；
- task；
- request；
- Component invocation；
- AI run；
- error chain。

可观测性不得成为 Secret 或大块敏感 payload 的泄漏渠道。

---

## 18. Cargo、依赖与供应链

本项目是应用项目，必须维护并提交可复现的 `Cargo.lock`。

Workspace 中重复依赖和 lint 配置应优先集中管理，避免成员 crate 漂移。

只启用真实需要的 dependency feature，并审查 default features。

Cargo feature 必须表示真实可组合能力，并应遵循 additive 语义。

不得把 feature 当作：

- 环境选择器；
- 隐藏配置系统；
- 互相冲突的产品模式；

除非确有无法避免的上游约束并经过明确审计。

新增依赖必须评估：

- 语义匹配；
- 维护状态；
- 安全记录；
- License；
- 平台支持；
- Rust/WASI target 支持；
- feature 集；
- native build requirement；
- 是否增加额外 runtime；
- 是否引入不必要系统库。

快速变化依赖应精确管理版本。

Git dependency 如确有必要，应固定到明确 revision，不得依赖漂移分支。

依赖安全公告、License 和供应链风险应使用成熟工具持续检查，而不是人工长期维护名单。

---

## 19. 格式、Lint、测试与验证

Rust 格式以 `rustfmt` 为准，不得建立与其对抗的手工格式体系。

Clippy 和编译器 lint 是基础质量门槛。

禁止为了让 CI 变绿进行大范围 `allow`。

确需允许 lint 时，应：

- 范围尽可能小；
- 原因明确；
- 保留对应语义。

测试应优先验证产品行为和契约，而不是锁死 implementation detail。

不得为了方便 mocking 而制造无真实抽象价值的 trait 和层次。

修复 bug 时，在适合情况下必须增加能复现原问题的回归测试。

任何变更都必须执行与影响范围匹配的真实验证，包括适用的：

- format；
- compile/check；
- clippy/static analysis；
- unit test；
- integration test；
- frontend type/build validation；
- WASM Component build/validation；
- 跨边界 contract test；
- 平台条件验证；
- AI 行为评估；
- benchmark / profiling。

性能结论必须来自真实测量，不能来自直觉。

未经实际验证不得宣称完成。

无法执行某项验证时，必须明确说明未验证内容及原因。

---

## 20. 修改纪律与完成标准

修改应聚焦当前任务，禁止顺手进行无关大面积重构。

但如果任务暴露了根本设计错误，不得为了保持小 diff 而继续叠加错误架构。

应修正正确边界，而不是永久增加：

- compatibility shim；
- dual path；
- dead code；
- 临时 adapter；
- TODO architecture；
- 无意义 feature flag。

内部 API 当前没有外部兼容承诺时，应优先保持正确和简洁，而不是维护历史错误。

任务只有同时满足以下条件才算完成：

- 满足真实需求；
- 没有突破产品边界；
- 符合所用技术栈的原生哲学；
- 第一方 Rust 保持 100% Safe；
- 已执行 Library-first；
- 没有形成第二套业务语义或状态真相；
- 没有新增无价值运行时成本；
- 权限、状态和资源生命周期明确；
- 错误与失败语义完整；
- AI 和 Secret 边界正确；
- 已完成与影响范围匹配的真实验证；
- 必要文档已同步；
- 已通过全量交叉审计。

**局部能运行不等于整体正确。**

---

## 21. OpenCode 与文档

根 `AGENTS.md` 必须保持**最小充分、导航优先**。

这里只保留：

- 最高裁决规则；
- 长期产品边界；
- 普遍适用的工程原则；
- 缺失后容易导致错误决策的约束；
- 仓库形成后真正需要长期记住的 canonical build/test 入口。

详细架构、领域设计、专题约束和长篇背景下沉到 `docs/`，并按当前任务需要读取。

不得通过 `opencode.json` 的全局 instructions 把大量专题文档永久加载到每次上下文。

重复、稳定、适合按需执行的工作流优先使用 OpenCode Skill 等按需机制，而不是继续膨胀根规则。

同一规则只能有一个权威事实源。

新增根规则前必须回答：

> 如果 Agent 在绝大多数会话中不知道这条规则，是否很可能做出错误决策？

如果答案是否定的，应下沉到专题文档或 Skill。

`AGENTS.md` 是项目开发的**地图、宪法和裁决入口**，不是完整百科全书。