all:
	riscv64-unknown-elf-gcc -march=rv64imc -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-gcc -march=rv64imc -S -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-objcopy -O binary a.out test.bin

