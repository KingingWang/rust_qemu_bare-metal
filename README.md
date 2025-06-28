# rust_qemu_bare-metal
This project is suitable for Rust beginners to experience ARM bare-metal development.

## 🇨🇳 中文介绍  
**专为Rust裸机开发初学者设计**的AArch64模板项目，具有以下特性：  
✅ 完全禁用标准库(`std`)，仅使用`core`和`alloc`  
✅ 预配置`println!`宏支持（无需OS依赖）  
✅ 生成可直接在QEMU运行的ELF文件  
✅ 精简启动流程
---
## 🇺🇸 English Introduction  
**Beginner-friendly template** for baremetal Rust development on AArch64:  
✅ No `std` support - uses only `core` and `alloc`  
✅ Pre-configured `println!` macro (no OS required)  
✅ Directly executable ELF on QEMU virt machine  
✅ Minimal boot sequence
---

## 🚀 使用方法 / Usage  
### 编译示例 / Build Example  

```bash
rustup target add aarch64-unknown-none-softfloat
cargo build --example hello
```

QEMU 运行 / Run in QEMU

```bash
qemu-system-aarch64 \
  -machine virt \
  -m 1024M \
  -cpu cortex-a53 \
  -nographic \
  -kernel ./target/aarch64-unknown-none-softfloat/debug/examples/hello
```

🖥️ 运行输出示例 / Example Output
```bash
Hello, world!
```

📂 项目结构 / Project Structure
```
.
├── aarch64-qemu.ld
├── Cargo.lock
├── Cargo.toml
├── examples
│   └── hello.rs
├── LICENSE
├── README.md
└── src
    ├── lib.rs
    ├── main.rs
    ├── panic.rs
    ├── start.s
    └── uart_console.rs
```
📜 许可证 / License
本项目采用 MIT许可证 - 详见 LICENSE 文件

This project is licensed under MIT License - see LICENSE

提示：首次使用需安装目标平台：

rustup target add aarch64-unknown-none-softfloat