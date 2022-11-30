After Qemu 7.0.0, we can directly submit the kernel executable file os to Qemu without any metadata clipping. In this case, our kernel can also run normally. The specific method is: 
```
-device loader,file=path/to/os
```

Use the following command to discard the metadata in the kernel executable file to get the kernel image:
```
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/ophelia_os -O binary target/riscv64gc-unknown-none-elf/release/ophelia_os.bin
```

Run with:
```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./bootloader/rustsbi-qemu.bin \
    -device loader,file=./ophelia_os/target/riscv64gc-unknown-none-elf/debug/ophelia_os.bin,addr=0x80200000
```
