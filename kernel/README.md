__build with `cargo build --release`__

## Application and Basic Execution Environment

After Qemu 7.0.0, we can directly submit the kernel executable file os to Qemu without any metadata clipping. In this case, our kernel can also run normally. The specific method is: 
```
-device loader,file=path/to/os
```

Use the following command to discard the metadata in the kernel executable file to get the kernel image:
```
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
```

Run with:
```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```

Start Qemu and load RustSBI and the kernel image with the following command:
```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
	-s -S
```

`-s` can make Qemu listen to local TCP port 1234 waiting for GDB client connection, and `-S` can make Qemu start running after receiving GDB's request. Therefore, Qemu has no output for now. Note that if you do not want to debug Qemu through GDB but run Qemu directly, you need to delete `-s -S` in the last line.

Start a GDB client connected to Qemu:
```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
	-s -S
```

`-s` can make Qemu listen to local TCP port 1234 waiting for GDB client connection, and `-S` can make Qemu start running after receiving GDB's request. Therefore, Qemu has no output for now. Note that if you do not want to debug Qemu through GDB but run Qemu directly, you need to delete `-s -S` in the last line.

Open another terminal and start a GDB client to connect to Qemu:
```
riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```

If successful you can see:
```
Reading symbols from target/riscv64gc-unknown-none-elf/release/os...
The target architecture is set to "riscv:rv64".
Remote debugging using localhost:1234
0x0000000000001000 in ?? ()
(gdb)
```

You can check the contents of Qemu's boot firmware:
```
(gdb) x/10i $pc
=> 0x1000:      auipc   t0,0x0
   0x1004:      addi    a2,t0,40
   0x1008:      csrr    a0,mhartid
   0x100c:      ld      a1,32(t0)
   0x1010:      ld      t0,24(t0)
   0x1014:      jr      t0
   0x1018:      unimp
   0x101a:      0x8000
   0x101c:      unimp
   0x101e:      unimp
(gdb) si
0x0000000000001004 in ?? ()
(gdb) si
0x0000000000001008 in ?? ()
(gdb) si
0x000000000000100c in ?? ()
(gdb) si
0x0000000000001010 in ?? ()
(gdb) si
0x0000000000001014 in ?? ()
(gdb) p/x $t0
$2 = 0x80000000
(gdb) si
0x0000000080000000 in ?? ()
```

It can be seen that when the instruction at 0x1010 is executed, the next instruction to be executed is at the entry of RustSBI, which is 0x80000000, which means that we are about to transfer control to RustSBI.

Check if control can be handed over to our kernel:
```
(gdb) b *0x80200000
Breakpoint 1 at 0x80200000
(gdb) c
Continuing.

Breakpoint 1, 0x0000000080200000 in stext ()
```

Check whether the first instruction of the kernel is executed correctly:
```
(gdb) x/5i $pc
=> 0x80200000 <stext>:  li      ra,100
   0x80200004:  unimp
   0x80200006:  unimp
   0x80200008:  unimp
   0x8020000a:  unimp
(gdb) si
0x0000000080200004 in ?? ()
(gdb) p/d $x1
$3 = 100
(gdb) p/x $sp
$4 = 0x0
```

Here `ra` is an alias for the register `x1`, `p/d $x1` can print the value of the register `x1` in decimal, and its result is correct.
