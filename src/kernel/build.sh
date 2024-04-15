#!/bin/bash

set -e

debug=0 qemu=0
target='x86_64-unknown-uefi'
buildDir="target/$target/debug"

for arg; do
    case $arg in
        '-d'|'--debug') debug=1;;
        '-q'|'--qemu') qemu=1;;
    esac
done

if (( debug )); then
    cargo build --target "$target" \
        -Z build-std=core,alloc \
        -Z build-std-features=compiler-builtins-mem
else
    cargo build --release --target "$target" \
        -Z build-std=core,alloc \
        -Z build-std-features=compiler-builtins-mem
fi

cargo run \
    --package disk_image \
    -- "$buildDir/kernel.efi"

(( qemu )) && qemu-system-x86_64 \
    -drive format=raw,file="$buildDir/kernel.gdt" \
    -bios OVMF_CODE-pure-efi.fd
