qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/peter_os_batch.bin,addr=0x80400000