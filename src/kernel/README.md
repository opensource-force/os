<div align="center">
<h1>microkernel</h1>
</div>

A microkernel is a modular and extensible kernel that leverages IPC, allowing process addresses to be isolated from each other and of course the kernel itself. It pushes most components into userland and focuses on keeping the kernel itself as minimal as functionally possible; Meanwhile emphasizing security and simplified testing

```
               +----------+
        +------| Hardware |
+-------|+     +----|-----+
|        |     +----|----+
| Kernel |--+--| Daemons |
|        |  |  +----|----+
+--------+ IPC +----|-----+
               | Software |
               +----------+
```

In microkernel architecture IPC, file systems and networking are implemented as daemons or services

### Pros
- Stabability: Reduced amount of code and kernel isolation
- Modularity: Seperates core kernel functionality from services
- Security: The modular design keeps services from affecting other system components and promotes seperate permissions
- Debugging: Fewer and isolated components allows easier testing
- Portability: Being minimalistic, it can run on more hardware

### Cons
- Performance: IPC overhead
- Complexity: The seperation of components and IPC makes things less simple

*Compared to a monolithic kernel*

## Interrupts
Both hardware and software interrputs cause the CPU to stop execution, save state, jump to a defined handler and resume execution after. The difference being that hardware interrupts are sent by some external device. When external events trigger an interrupt an interrupt service routine (ISR) is called

```
+------------+   +----------+
|            |<--| Hardware |
|            |   +----------+
| Interrupt  |
| Controller |      +-----+
|            |----->| CPU |
|            |      +-----+
+------------+
```

*x86 architecture systems are interrupt driven*

### CPU Exceptions
Exceptions are a type of interrupt sent by the CPU when an exception type occurs; Such as a double fault or illegal storage access

## Components
### Memory Management
Responsible for allocating, managing and ensuring processes have sufficent memory

### Process Management
Handles resources and the creation, termination and scheduling of processes. Manages the execution of user programs and fair use of system resources

### Inter-Process Communication (IPC)
Facilitates communication between processes running in user-space and kernel components. An IPC handles, remote procedure calls (RPC), message-passing, shared memory and more

## Allocator Types
### Bump (stack) allocator
A bump allocator is a simple linear memory allocator that tracks the number of allocations and allocated bytes

Concept

A bump allocator sequentially allocates memory; Initially tracking the start address and bumping it by allocation size, effectively defining the boundary between used and unused memory. Allocations are contiguous and therefore deallocations aren't tracked but instead freed by the entire heap

Pros
- Simple and fast allocations
- Predictable memory layout
- No bookkeeping

Cons
- Deallocated memory can only be utilized after ALL allocations are freed

### Linked List (pool) allocator
A linked list allocator is a dynamic, linear memory allocation strategy that utilizes a linked list data structure. Unlike a bump allocator, which allocates memory sequentially, a linked list allocator divides memory into blocks of varying size and uses a linked list to keep track of available memory blocks

Concept

Initially the entire heap is represented as one whole block. As allocations are made, the heap is divided into non-contiguous blocks. Within each free block is the size of the memory block and a pointer to the next unallocated memory block. To track all free blocks, only a pointer to the initial (head) free block is required

Pros
- Handles flexible, frequent allocation and deallocations
- Enables memory resuse without all allocations freed

Cons
- Susceptible to fragmentation; Where memory can be permanently divided into smaller blocks
- Bookkeping

### Fixed(-size) block allocator
A fixed-size block allocator is similar to a linked list allocator but with the main difference being that it stores a new linked list for each block size. Blocks of varying sizes are made to meet allocation request and since each block size defines a new list (head), the head name itself defines the block size, e.g. head_16

Pros
- Handles flexible, frequent allocation and deallocations
- Enables memory resuse without all allocations freed
- Fast, no traversal on allocations or deallocations

Cons
- Memory waste by roundup
- Susceptible to fragmentation

## Build & Execution
Build kernel, create disk image & load into Qemu

`./qemu.sh`/`bash qemu.sh` or execute the commands within the script individually

## Resources
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