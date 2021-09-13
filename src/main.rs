//! Rest-EMU is a RISC-V emulator so we can emulate and define a custom
//! RISC-V cpu

// TODO(patrik):
//   - For rust to compile for RV64 we need to have the GC extentions
//     implement: G = I M A F D Zicsr Zifencei
//                    x x x     xxxxx
//   - Implement the M extentions (done)
//   - Implement the A extentions
//   - Implement the F extentions
//   - Implement the D extentions
//   - Implement the Zifencei extentions

mod instruction;
mod mmu;
mod cpu;

use mmu::Mmu;
use cpu::{ Core, Register };

fn load_binary_program(mmu: &mut Mmu) {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("test/rust-test.bin")
        .expect("Failed to load test binary");
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to load test binary");

    for index in 0..data.len() {
        let value = data[index];
        mmu.write_u8(index as u64, value);
    }
}

fn main() {
    let mut mmu = Mmu::new(1 * 1024 * 1024);
    load_binary_program(&mut mmu);

    let mut core = Core::new(mmu);
    core.set_reg(Register::Ra, 0xffff1337);
    core.set_reg(Register::Sp, 1 * 1024 * 1024);

    core.set_reg(Register::A0, 123);
    core.set_reg(Register::A1, 321);

    core.write_csr(0xfff, 0b111);

    loop {
        let res = core.step();
        // println!("Exit: {:#?}", res);

        if core.reg(Register::Pc) == 0xffff1337 {
            break;
        }
    }

    println!("{:#x?}", core);

    let value = core.read_csr(0xfff);
    println!("CSR Reg: {:#b}", value);
}
