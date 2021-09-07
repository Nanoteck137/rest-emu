//! Rest-EMU is a RISC-V emulator so we can emulate and define a custom
//! RISC-V cpu

#![allow(dead_code)]

const NUM_REGISTERS: usize = 32;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Register {
    Zero, // x0
    Ra,   // x1
    Sp,   // x2
    Gp,   // x3
    Tp,   // x4
    T0,   // x5
    T1,   // x6
    T2,   // x7
    S0,   // x8
    S1,   // x9
    A0,   // x10
    A1,   // x11
    A2,   // x12
    A3,   // x13
    A4,   // x14
    A5,   // x15
    A6,   // x16
    A7,   // x17
    S2,   // x18
    S3,   // x19
    S4,   // x20
    S5,   // x21
    S6,   // x22
    S7,   // x23
    S8,   // x24
    S9,   // x25
    S10,  // x26
    S11,  // x27
    T3,   // x28
    T4,   // x29
    T5,   // x30
    T6    // x31
}

impl Register {
    fn index(&self) -> usize {
        match self {
            Register::Zero => 0,
            Register::Ra   => 1,
            Register::Sp   => 2,
            Register::Gp   => 3,
            Register::Tp   => 4,
            Register::T0   => 5,
            Register::T1   => 6,
            Register::T2   => 7,
            Register::S0   => 8,
            Register::S1   => 9,
            Register::A0   => 10,
            Register::A1   => 11,
            Register::A2   => 12,
            Register::A3   => 13,
            Register::A4   => 14,
            Register::A5   => 15,
            Register::A6   => 16,
            Register::A7   => 17,
            Register::S2   => 18,
            Register::S3   => 19,
            Register::S4   => 20,
            Register::S5   => 21,
            Register::S6   => 22,
            Register::S7   => 23,
            Register::S8   => 24,
            Register::S9   => 25,
            Register::S10  => 26,
            Register::S11  => 27,
            Register::T3   => 28,
            Register::T4   => 29,
            Register::T5   => 30,
            Register::T6   => 31,
        }
    }
}

#[derive(Debug)]
struct Core {
    registers: [u64; NUM_REGISTERS],
}

impl Core {
    fn new() -> Self {
        Self {
            registers: [0; NUM_REGISTERS],
        }
    }

    fn set_reg(&mut self, reg: Register, value: u64) {
        if reg != Register::Zero {
            self.registers[reg.index()] = value;
        }
    }

    fn reg(&self, reg: Register) -> u64 {
        self.registers[reg.index()]
    }
}

fn main() {
    let mut core = Core::new();
    core.set_reg(Register::S9, 0x1337);

    println!("Core: {:#x?}", core);
}
