all:
	riscv64-unknown-elf-gcc -O1 -march=rv64i -mabi=lp64 -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-gcc -O1 -S -ffreestanding -nostdlib -Tlinker.ld main.c
	riscv64-unknown-elf-objcopy -O binary a.out test.bin

