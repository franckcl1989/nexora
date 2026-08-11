# Workspace 布局：nexora-core 与 nexora-app 两 crate 加独立前端目录

Nexora 采用 Cargo workspace，包含 `nexora-core`（纯领域，零 Tauri/WebView/wasmtime 依赖）与 `nexora-app`（Tauri 壳，唯一 adapter 汇聚地）两个 crate，前端代码放独立目录 `nexora-web`。core 与 app 分离保证领域模型不被框架类型污染；按需才拆，不按子域预拆 crate。
