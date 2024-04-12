<div align="center">
<h1>microkernel</h1>
</div>

## Build & Execution
Compile, create disk image & load into Qemu

`./qemu.sh`/`bash qemu.sh` or execute the commands within said script individually

## Resources
### x86_64
1. [uefi-rs](https://rust-osdev.github.io/uefi-rs/HEAD/): up-to-date ref; handles & protocols, drivers, GUID/tables, etc
2. [Phil-OS](https://github.com/phil-opp/blog_os/blob/edition-3/blog/content/edition-3/posts/02-booting/uefi/index.md): UEFI, target, disk image, elf binary, qemu, etc
3. [Phil-OS](https://os.phil-opp.com): BIOS, bootloader, paging, allocator, etc

### RSIC-V
- [RISC-V OS](https://osblog.stephenmarz.com) - allocation, interrupts, system calls, etc

---

### WASI/WASM
- [wasmtime](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md): tutorial
- [Mozilla](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm): tutorial
- [ABI](https://github.com/WebAssembly/WASI): spec
- [Rust API](https://github.com/bytecodealliance/wasi): spec
- [Mozilla](https://developer.mozilla.org/en-US/docs/WebAssembly#api_reference): spec
- [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift): compiler
- [LLVM](https://surma.dev/things/c-to-webassembly): compiler
- [Wasmi](https://github.com/wasmi-labs/wasmi): interpreter
- [wasmtime](https://github.com/bytecodealliance/wasmtime): interpreter
- [wasmer](https://github.com/wasmerio/wasmer): interpreter

## Implementations
- [reenix](http://scialex.github.io/reenix.pdf): Unix-like OS
- [coral](https://github.com/CharlyCst/coral): Experimental WASM userspace kernel
- [kwast](https://github.com/kwast-os/kwast): Microkernel running WASM bytecode in userspace
- [etheryal](https://github.com/KernelFreeze/etheryal-kernel): Kernel with WASM runtime and WASI implementation
- [beryl](https://github.com/falkor11/Beryl): Microkernel
- [poplar](https://github.com/IsaacWoods/poplar): Microkernel
- [redshirt](https://github.com/tomaka/redshirt): Kernel implementing decentralized IPFS executables with WASM