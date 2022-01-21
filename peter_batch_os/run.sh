qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/peter_batch_os.bin,addr=0x80400000