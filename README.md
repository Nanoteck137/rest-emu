# rest-emu
Rest-EMU is a RISC-V emulator used for implementing and testing RISC-V harts/cores

### Compile Emulator
Tested on Rust version: 1.57.0-nightly

### Compile test code

Setup the RISC-V GNU Toolchain
    $ git clone https://github.com/riscv/riscv-gnu-toolchain
    $ cd riscv-gnu-toolchain
    $ ./configure --prefix=/opt/riscv --with-arch=rv64i --with-abi=lp64
    $ make
