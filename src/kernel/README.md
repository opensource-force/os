<div align="center">
<h1>microkernel</h1>
</div>

## Specification
- [WASI](https://github.com/WebAssembly/WASI) - ABI

- [WASI](https://github.com/bytecodealliance/wasi) - Rust API

- [WASM](https://developer.mozilla.org/en-US/docs/WebAssembly#api_reference) - Mozilla docs
[Unix-like OS](http://scialex.github.io/reenix.pdf)

## Compiler
- [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)

- [LLVM](https://surma.dev/things/c-to-webassembly)

## Interpreter
- [Wasmi](https://github.com/wasmi-labs/wasmi)
- [wasmtime](https://github.com/bytecodealliance/wasmtime) (&Cranelift)
- [wasmer](https://github.com/wasmerio/wasmer)

## Resources
- [Kernel](https://os.phil-opp.com/minimal-rust-kernel) - Tutorial
- [RISC-V OS](https://osblog.stephenmarz.com) - Tutorial
- [Rust -> WASM](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) - Mozilla docs
- [WASI](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md) - Tutorial

## Examples
- [Rust OSDev](https://github.com/rust-osdev) - x86_64, UEFI, bootloader
- [coral](https://github.com/CharlyCst/coral) - Experimental WASM userspace kernel
- [kwast](https://github.com/kwast-os/kwast) - Microkernel running WASM bytecode in userspace
- [etheryal](https://github.com/KernelFreeze/etheryal-kernel) - Kernel with WASM runtime and WASI implementation
- [beryl](https://github.com/falkor11/Beryl) - Microkernel
- [poplar](https://github.com/IsaacWoods/poplar) - Microkernel
- [redshirt](https://github.com/tomaka/redshirt) - Kernel implementing decentralized IPFS executables with WASM