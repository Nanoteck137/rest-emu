all:
	riscv64-unknown-elf-gcc -march=rv64im -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-gcc -march=rv64im -S -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-objcopy -O binary a.out test.bin

