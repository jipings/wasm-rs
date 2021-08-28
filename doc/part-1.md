## Wasm介绍和环境准备

[Wasm官网](https://webassembly.org/)

[Wasm wiki](https://zh.wikipedia.org/wiki/WebAssembly)

相关规范
WebAssembly Core Specification

WebAssembly JavaScript Interface Specification

https://github.com/WebAssembly

[wasm api](https://developer.mozilla.org/zh-CN/docs/WebAssembly)

asm.js

编译器包含：

* 前端（Front End）
主要负责预处理、词法分析、语法分析、语义分析，生成便于后续处理的中间表示（Intermediate Representation，IR）

* 中端（Middle End）
中端对IR进行分析和优化

* 后端（Back End）
后端生成目标代码，把IR转换成平台相关的汇编代码，最终由汇编器译为机器码

技术特点

1.  技术规范

2. 模块
模块是 Wasm 程序编译、传输和加载的单位
Wasm 规范定义了两种模块格式：二进制格式和文本格式

3. 指令集
wasm 也采用栈虚拟机和字节码

4. 验证
wasm 模块必须安全可靠，不允许有任何恶意行为。

安装rust
安装 wasm32

> rustup target add wasm32-unknown-unknown

安装 WABT， 二进制工具箱（WebAssembly Binary Toolkit），包括 WAT 汇编器 wat2wasm、反汇编器 wasm2wat、Wasm 二进制格式查看工具 wasm-objdump、二进制格式验证工具 wasm-validate 等。

> brew install WABT

编译为wasm文件
> wat2wasm xxx.wat 

> wasm-objdump -h xxx.wasm # 打印段头信息
> wasm-objdump -x xxx.wasm # 打印详细信息



