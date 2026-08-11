# nexora-core 采用 domain / application / ports 三层

`nexora-core` 内部分三层：`domain` 只放纯类型与不变量；`application` 放用例编排；`ports` 定义对外部可替换依赖的 trait 契约。依赖方向指向内部（domain 不依赖任何层），替换依赖时改动只落在 adapter。
