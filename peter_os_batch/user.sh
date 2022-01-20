qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios bootloader/rustsbi-qemu.bin \
    -device loader,file=user/target/riscv64gc-unknown-none-elf/release/hello_world,addr=0x80400000