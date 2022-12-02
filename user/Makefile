TARGET := riscv64gc-unknown-none-elf
QEMU := qemu-system-riscv64
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

elf:
	@cargo build --release
	@echo $(APPS)
	@echo $(ELFS)

build: elf