all:
	riscv64-unknown-elf-gcc -ffreestanding -nostdlib -Tlinker.ld -O0 main.c
	riscv64-unknown-elf-objcopy -O binary a.out test.bin

