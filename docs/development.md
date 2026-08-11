# 开发指南：启动与验证

本文档记录 Nexora 的开发启动方式与已验证的工程链路。

## 目录结构

```
nexora/
├── nexora-core/     # 纯领域 crate（domain / application / ports 三层，零框架依赖）
├── nexora-app/      # Tauri 壳 crate（唯一 adapter 汇聚地，含 tauri.conf.json）
├── nexora-web/      # 前端（Vite + React + TS）
└── docs/            # 架构决策与专题文档
```

## 一键启动开发模式

```powershell
# 从 nexora-app 目录启动（tauri CLI 从当前目录向下查找 tauri.conf.json）
cd nexora-app
..\nexora-web\node_modules\.bin\tauri.cmd dev
```

完整链路：`beforeDevCommand`（启动 Vite，端口 1420）→ cargo 编译 → 打开应用窗口加载 `devUrl`。

## 手动两步启动（调试备选）

```powershell
# 终端 1：启动��端
cd nexora-web
npm run dev

# 终端 2：以 devUrl 模式运行应用
cd nexora-app
cargo run
```

## monorepo 目录语义（重要约束）

Tauri CLI 在 monorepo 下有两条**只向下搜索**的路径解析规则，容易踩坑：

1. **tauri.conf.json 定位**：从当前目录向下查找（默认深度 3）。从 `nexora-app` 目录启动可被直接命中。
2. **beforeDevCommand 的工作目录**：默认是 CLI 解析出的"前端目录"（含 package.json 的目录），**不是** tauri.conf.json 所在目录。本项目前端与 Rust 分离，必须用对象形式显式指定 cwd：

```jsonc
"build": {
  "beforeDevCommand": { "script": "npm run dev", "cwd": "../nexora-web" },
  "beforeBuildCommand": { "script": "npm run build", "cwd": "../nexora-web" }
}
```

- `cwd` 是相对 tauri.conf.json 所在目录（nexora-app）解析的。
- 若用字符串形式 `npm --prefix ../nexora-web run dev`，会被从解析出的前端目录（错误回退到 workspace 根）执行，导致路径高一层而 ENOENT 失败。**禁止回退到字符串形式**。

## 构建与验证

```powershell
# 前端类型检查 + 构建
cd nexora-web
npm run build        # tsc --noEmit && vite build

# Rust 全量检查（workspace 根）
cargo check --workspace
cargo clippy --workspace --all-targets
cargo test --workspace
```

验证 IPC 链路：启动后窗口应显示"平台：\<os\> / \<arch\>"（前端 `invoke("platform_info")` → `nexora-app/src/commands.rs`）。
