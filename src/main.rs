//! Rest-EMU is a RISC-V emulator so we can emulate and define a custom
//! RISC-V cpu

// TODO(patrik):
//   - For rust to compile for RV64 we need to have the GC extentions
//     implement: G = I M A F D Zicsr Zifencei
//                    x x x     xxxxx
//   - Implement the M extentions (done)
//   - Implement the A extentions (almost done)
//   - Implement the F extentions
//   - Implement the D extentions
//   - Implement the Zifencei extentions

mod instruction;
mod mmu;
mod cpu;

use mmu::Mmu;
use cpu::{ Core, Register };
use cpu::{ CoreState, CoreStateFunctions, PrivilegeLevel };

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

fn custom_write_csr(core_state: &mut CoreState, csr: u16, value: u64) {
    core_state.csr_registers[csr as usize] = value;
}

fn custom_read_csr(core_state: &CoreState, csr: u16) -> u64 {
    core_state.csr_registers[csr as usize]
}

fn custom_write_privilege_level(core_state: &mut CoreState,
                                privilege_level: PrivilegeLevel)
{
}

fn custom_read_privilege_level(core_state: &CoreState) -> PrivilegeLevel {
    core_state.privilege_level
}

fn main() {
    let mut mmu = Mmu::new(1 * 1024 * 1024);
    load_binary_program(&mut mmu);

    let entry = std::fs::read_to_string("test/rust-test.entry")
        .expect("Failed to find a entry file");
    let entry = entry.trim_end_matches('\n');
    let entry = u64::from_str_radix(&entry[2..], 16)
        .expect("Failed to parse entry to int");

    let core_state_funcs = CoreStateFunctions {
        write_csr: custom_write_csr,
        read_csr: custom_read_csr,

        write_privilege_level: custom_write_privilege_level,
        read_privilege_level: custom_read_privilege_level,
    };

    let core_state = CoreState::new(core_state_funcs);

    let mut core = Core::new(core_state, mmu);
    core.set_reg(Register::Pc, entry);
    core.set_reg(Register::Ra, 0xffff1337);
    core.set_reg(Register::Sp, 1 * 1024 * 1024);

    core.set_reg(Register::A0, 123);
    // core.set_reg(Register::A1, 321);

    core.write_csr(0xfff, 0b111);

    loop {
        let res = core.step();
        println!("Exit: {:#?}", res);

        // TODO(patrik):
        // check_devices_for_interrupts();
        // check_interrupts();
        // send_interrupt_to_core();

        // core.trap(pc);

        if core.reg(Register::Pc) == 0xffff1337 {
            break;
        }
    }

    println!("{:#x?}", core);

    let value = core.read_csr(0xfff);
    println!("CSR Reg: {:#b}", value);

    let value = core.mmu.read_u32(0x36c);
    println!("Value: {}", value);
}
