all: c-test rust-test

rust-test:
	rustc main.rs -C link-args=-Tlinker.ld --target=riscv64imac-unknown-none-elf -o rust-test.elf
	rustc main.rs -C link-args=-Tlinker.ld --emit=asm --target=riscv64imac-unknown-none-elf -o rust-test.s
	riscv64-unknown-elf-objcopy -O binary rust-test.elf rust-test.bin

c-test:
	riscv64-unknown-elf-gcc -march=rv64imc -ffreestanding -nostdlib -Tlinker.ld main.c -o c-test.elf
	riscv64-unknown-elf-gcc -march=rv64imc -S -ffreestanding -nostdlib -Tlinker.ld main.c -o c-test.s
	riscv64-unknown-elf-objcopy -O binary c-test.elf c-test.bin

clean:
	rm c-test.elf c-test.bin c-test.s
	rm rust-test.elf rust-test.bin

.PHONY: c-test rust-test clean
