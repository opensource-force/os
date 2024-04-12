#!/bin/bash

cargo build --target x86_64-unknown-uefi -Z build-std=core -Z build-std-features=compiler-builtins-mem
cargo run --package disk_image -- target/x86_64-unknown-uefi/debug/kernel.efi
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-uefi/debug/kernel.gdt -bios OVMF_CODE-pure-efi.fd
