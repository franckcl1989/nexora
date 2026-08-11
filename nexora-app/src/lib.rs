#![forbid(unsafe_code)]
#![doc = "Nexora 应用壳：Tauri adapter 汇聚地。"]

pub use nexora_core;

/// 启动桌面应用。
///
/// # Panics
///
/// 应用上下文生成失败或应用运行失败时 panic。
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("failed to run tauri application");
}
