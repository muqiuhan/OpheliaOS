OS=ophelia_os
CARGO_BINUTILS=
DEBUGGER=gdb
TARGET=./target/riscv64gc-unknown-none-elf/release/$(OS)
BOOTLOADER=./bootloader/rustsbi-qemu.bin
BIN=./$(OS).bin

run: build
	@echo -e "NOTE: Press C-a x to exit QEMU\n"
	@qemu-system-riscv64 -machine virt \
			-nographic -bios $(BOOTLOADER) \
			-device loader,file=$(BIN),addr=0x80200000

run.debug: build
    @echo -e "NOTE: Press C-a x to exit QEMU\n"
	@qemu-system-riscv64 -machine virt \
			-nographic -bios $(BOOTLOADER) \
			-device loader,file=$(BIN),addr=0x80200000 \
			-s -S
build:
	@cargo build --release
	@$(CARGO_BINUTILS)rust-objcopy --strip-all $(TARGET) -O binary $(BIN)

fmt:
	@cargo fmt

clean:
	@cargo clean
	@rm $(BIN)
