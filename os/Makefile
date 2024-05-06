# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# GDB wrapper to handle virtual path to core lib and types display in Rust
GDB_PATH := /home/dbydd/gitRepos/2024s-rcore-dbydd/gdb-14.2/build-rv64/bin/riscv64-unknown-elf-gdb
gdb := RUST_GDB=$(GDB_PATH) rust-gdb

# Emit asm code
OBJDUMP := rust-objdump --arch-name=riscv64 --disassemble

gdb:
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(ELF),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h "$(gdb) -ex 'file $(ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux swap-pane -U && \
		tmux -2 attach-session -d

# kernel:
# 	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(ELF),addr=$(KERNEL_ENTRY_PA)

disasm:
	@$(OBJDUMP) $(ELF) | bat -l asm

gdbserver:
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(ELF),addr=$(KERNEL_ENTRY_PA) -s -S

gdbclient: 
	@$(gdb) -ex 'file $(ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

kernel:
	cargo build --release

clean:
	cargo clean

run: kernel
	timeout --foreground 30s qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-kernel $(KERNEL_ELF)

.PHONY: build kernel clean run gdb disasm gdbserver gdbclient
