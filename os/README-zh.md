## 应用程序与基本执行环境

在 Qemu 7.0.0 版本后，我们可以直接将内核可执行文件 os 提交给 Qemu 而不必进行任何元数据的裁剪工作，这种情况下我们的内核也能正常运行。其具体做法为：将 Qemu 的参数替换为 -device loader,file=path/to/os:
```
-device loader,file=path/to/os
```

使用如下命令可以丢弃内核可执行文件中的元数据得到内核镜像：
```
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
```

使用qemu运行:
```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```

基于 GDB 验证启动需要通过以下命令启动 Qemu 并加载 RustSBI 和内核镜像：
```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
	-s -S
```

-s 可以使 Qemu 监听本地 TCP 端口 1234 等待 GDB 客户端连接，而 -S 可以使 Qemu 在收到 GDB 的请求后再开始运行。因此，Qemu 暂时没有任何输出。注意，如果不想通过 GDB 对于 Qemu 进行调试而是直接运行 Qemu 的话，则要删掉最后一行的 -s -S。

打开另一个终端，启动一个 GDB 客户端连接到 Qemu ：
```
riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```

如果成功可以看到:
```
Reading symbols from target/riscv64gc-unknown-none-elf/release/os...
The target architecture is set to "riscv:rv64".
Remote debugging using localhost:1234
0x0000000000001000 in ?? ()
(gdb)
```

可以检查一下 Qemu 的启动固件的内容：
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
可以看到，当位于 0x1010 的指令执行完毕后，下一条待执行的指令位于 RustSBI 的入口，也即 0x80000000 ，这意味着我们即将把控制权转交给 RustSBI 。

检查控制权能否被移交给我们的内核:
```
(gdb) b *0x80200000
Breakpoint 1 at 0x80200000
(gdb) c
Continuing.

Breakpoint 1, 0x0000000080200000 in stext ()
```

检查内核第一条指令是否被正确执行:
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
这里`ra`是寄存器`x1`的别名，`p/d $x1`可以以十进制打印寄存器`x1`的值，它的结果正确。