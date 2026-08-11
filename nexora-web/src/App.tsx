import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface PlatformInfo {
  os: string;
  arch: string;
}

export function App() {
  const [info, setInfo] = useState<PlatformInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<PlatformInfo>("platform_info")
      .then(setInfo)
      .catch((err) => setError(String(err)));
  }, []);

  return (
    <main>
      <h1>Nexora</h1>
      <p>AI 原生工程工作台（前端骨架）。</p>
      {error && <p style={{ color: "red" }}>IPC 调用失败：{error}</p>}
      {info && (
        <p>
          平台：{info.os} / {info.arch}
        </p>
      )}
    </main>
  );
}
