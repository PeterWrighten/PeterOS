# Build


```shell
$ chmod 777 ter.sh # authoricate permission to ter.sh

$ cargo build --release

$ cd target/riscv64gc-unknown-none-elf/release/

$ rust-objcopy --strip-all rcore-peter-os -O binary rcore-peter-os.bin

$ ./ter.sh # execute shell instr
```


