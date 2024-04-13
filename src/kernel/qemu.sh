#!/bin/bash

set -e

buildDir='target/x86_64-unknown-uefi/debug'

cargo build \
    --target x86_64-unknown-uefi \
    -Z build-std=core,alloc \
    -Z build-std-features=compiler-builtins-mem

cargo run \
    --package disk_image \
    -- "$buildDir/kernel.efi"

qemu-system-x86_64 \
    -drive format=raw,file="$buildDir/kernel.gdt" \
    -bios OVMF_CODE-pure-efi.fd
