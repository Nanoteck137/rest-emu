all:
	riscv64-unknown-elf-gcc -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-gcc -S -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-objcopy -O binary a.out test.bin

