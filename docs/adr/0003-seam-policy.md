# Seam 策略：Persistence / Platform / AI provider 三 trait 立即定义，WASM seam 暂缓

与易变依赖相接的边界立即定义 trait：Persistence（存储适配）、Platform（文件系统/进程/系统能力）、AI provider（模型提供商）。WASM host 与 WIT 契约暂缓：当前无任何 WASM 组件存在，不为尚未发生的需求预建基础设施；待第一个组件真实出现时再开 seam。
