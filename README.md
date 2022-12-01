<div align="center">

# OpheliaOS

<p> An open source riscv operating system kernel written in Rust </p>

</div>

## Build and Run
> OpheliaOS is developed using Rust and currently runs under qemu.

### Dependencies

#### Environment

- For Rust, OpheliaOS must be built under the nightly version of rust, the current version is `rustc 1.67.0-nightly (2585bcea0 2022-11-28)`
- For Qemu, OpheliaOS runs on `qemu-system-riscv64` version 7.0 and above, the current version is `QEMU emulator version 7.1.0`

#### Rust Toolchain
- Requires cross compilation with riscv64gc-unknown-none-elf: `rustup target add riscv64gc-unknown-none-elf`
- (Optional) Use cargo-binutils to manipulate binaries: `cargo install cargo-binutils`
- Other necessities:
    1. `rustup component add llvm-tools-preview`
    2. `rustup component add rust-src`

### Structure
```
├── bootloader
├── os
│   ├── Cargo.toml
│   ├── Makefile
│   ├── README.md
│   ├── src
└── user
    ├── Cargo.toml
    ├── Makefile
    ├── src
```

`bootloader`: Executable file of rustsbi \
`os`: The core of OpheliaOS \
`user`: Batch processing system of OpheliaOS

### Build
As you can see, the source code of OpheliaOS is scattered in different modules, but there is a `Makefile` under each module, which means that each module of OpheliaOS can be built separately by executing `make` in the corresponding directory.

The `os` directory is a special directory, which is the complete core of OpheliaOS, so executing `make` in the os directory will get a completed OpheliaOS core.

### Run
OpheliaOS currently runs on qemu and can be run by executing `make run` in the `os` directory.

# Comminicate
The operating system is a very large and profound field. Rust is an interesting and modern language. There will be many difficulties in the development of this project. This is a lonely road, so we created a group for discuss:
- Telegram : https://t.me/xfi_cn

# Acknowledgements
- Thanks to the [rCore-OS community](http://rcore-os.cn/) for their detailed and friendly [tutorials](http://rcore-os.cn/rCore-Tutorial-Book-v3/) that helped me with this project

# License
The MIT License (MIT)

Copyright (c) 2022 Muqiu Han

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.