//! Rest-EMU is a RISC-V emulator so we can emulate and define a custom
//! RISC-V cpu

#![allow(dead_code)]

struct Mmu {
    ram: Vec<u8>,
}

impl Mmu {
    fn new(size: usize) -> Self {
        Self {
            ram: vec![0; size],
        }
    }

    fn write_u8(&mut self, addr: u64, value: u8) {
        self.ram[addr as usize] = value;
    }

    fn read_u8(&self, addr: u64) -> u8{
        self.ram[addr as usize]
    }

    fn write_u16(&mut self, addr: u64, value: u16) {
        self.write_u8(addr, (value & 0xff) as u8);
        self.write_u8(addr + 1, ((value >> 8) & 0xff) as u8);
    }

    fn read_u16(&self, addr: u64) -> u16 {
        self.read_u8(addr) as u16 | (self.read_u8(addr + 1) as u16) << 8
    }

    fn write_u32(&mut self, addr: u64, value: u32) {
        self.write_u8(addr + 0, ((value >> 0) & 0xff) as u8);
        self.write_u8(addr + 1, ((value >> 8) & 0xff) as u8);
        self.write_u8(addr + 2, ((value >> 16) & 0xff) as u8);
        self.write_u8(addr + 3, ((value >> 24) & 0xff) as u8);
    }

    fn read_u32(&self, addr: u64) -> u32 {
        self.read_u8(addr) as u32 |
            (self.read_u8(addr + 1) as u32) << 8 |
            (self.read_u8(addr + 2) as u32) << 16 |
            (self.read_u8(addr + 3) as u32) << 24
    }

    fn write_u64(&mut self, addr: u64, value: u64) {
        self.write_u8(addr + 0, ((value >> 0) & 0xff) as u8);
        self.write_u8(addr + 1, ((value >> 8) & 0xff) as u8);
        self.write_u8(addr + 2, ((value >> 16) & 0xff) as u8);
        self.write_u8(addr + 3, ((value >> 24) & 0xff) as u8);

        self.write_u8(addr + 4, ((value >> 32) & 0xff) as u8);
        self.write_u8(addr + 5, ((value >> 40) & 0xff) as u8);
        self.write_u8(addr + 6, ((value >> 48) & 0xff) as u8);
        self.write_u8(addr + 7, ((value >> 56) & 0xff) as u8);
    }

    fn read_u64(&self, addr: u64) -> u64 {
        self.read_u8(addr) as u64 |
            (self.read_u8(addr + 1) as u64) << 8 |
            (self.read_u8(addr + 2) as u64) << 16 |
            (self.read_u8(addr + 3) as u64) << 24 |
            (self.read_u8(addr + 4) as u64) << 32 |
            (self.read_u8(addr + 5) as u64) << 40 |
            (self.read_u8(addr + 6) as u64) << 48 |
            (self.read_u8(addr + 7) as u64) << 56
    }
}

const NUM_REGISTERS: usize = 33;

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
    T6,   // x31
    Pc,   // pc
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
            Register::Pc   => 32,
        }
    }
}

impl From<u32> for Register {
    fn from(value: u32) -> Self {
        match value {
            0 => Register::Zero,
            1 => Register::Ra,
            2 => Register::Sp,
            3 => Register::Gp,
            4 => Register::Tp,
            5 => Register::T0,
            6 => Register::T1,
            7 => Register::T2,
            8 => Register::S0,
            9 => Register::S1,
            10 => Register::A0,
            11 => Register::A1,
            12 => Register::A2,
            13 => Register::A3,
            14 => Register::A4,
            15 => Register::A5,
            16 => Register::A6,
            17 => Register::A7,
            18 => Register::S2,
            19 => Register::S3,
            20 => Register::S4,
            21 => Register::S5,
            22 => Register::S6,
            23 => Register::S7,
            24 => Register::S8,
            25 => Register::S9,
            26 => Register::S10,
            27 => Register::S11,
            28 => Register::T3,
            29 => Register::T4,
            30 => Register::T5,
            31 => Register::T6,
            32 => Register::Pc,

            _ => panic!("Unknown register: {}", value),
        }
    }
}

#[derive(Debug)]
struct IType {
    imm: i32,
    rs1: Register,
    funct3: u32,
    rd: Register
}

impl From<u32> for IType {
    fn from(value: u32) -> Self {
        let imm = (value as i32) >> 20;

        let rs1 = Register::from((value >> 15) & 0b11111);
        let funct3 = (value >> 12) & 0b111;
        let rd = Register::from((value >> 7) & 0b11111);

        Self {
            imm,
            rs1,
            funct3,
            rd
        }
    }
}

struct Core {
    registers: [u64; NUM_REGISTERS],

    mmu: Mmu
}

impl Core {
    fn new(mmu: Mmu) -> Self {
        Self {
            registers: [0; NUM_REGISTERS],

            mmu
        }
    }

    fn step(&mut self) {
        let inst = self.fetch_u32();
        let opcode = inst & 0b1111111;

        match opcode {
            0b0010011 => {
                let inst = IType::from(inst);
                println!("Instruction: {:#x?}", inst);

                match inst.funct3 {
                    0b000 => {
                        // ADDI

                        let imm = inst.imm as i64 as u64;
                        let rs1 = self.reg(inst.rs1);
                        self.set_reg(inst.rd, rs1.wrapping_add(imm));
                    }
                    _ => panic!("Unimplemented funct3 '0b{:03b}'", inst.funct3),
                }
            }

            _ => panic!("Unknown opcode '{:#b}'", opcode),
        }
        println!("Executing inst: {:#x} Opcode: {:#b}", inst, opcode);
    }

    fn fetch_u32(&mut self) -> u32 {
        let pc = self.reg(Register::Pc);
        let result = self.mmu.read_u32(pc);
        self.set_reg(Register::Pc, pc + 4);

        result
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

impl std::fmt::Debug for Core {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "z0 {:016x} ra {:016x}  sp {:016x}  gp {:016x}\n",
               self.reg(Register::Zero), self.reg(Register::Ra),
               self.reg(Register::Sp), self.reg(Register::Gp))?;
        write!(f, "tp {:016x} t0 {:016x}  t1 {:016x}  t2 {:016x}\n",
               self.reg(Register::Tp), self.reg(Register::T0),
               self.reg(Register::T1), self.reg(Register::T2))?;
        write!(f, "s0 {:016x} s1 {:016x}  a0 {:016x}  a1 {:016x}\n",
               self.reg(Register::S0), self.reg(Register::S1),
               self.reg(Register::A0), self.reg(Register::A1))?;
        write!(f, "a2 {:016x} a3 {:016x}  a4 {:016x}  a5 {:016x}\n",
               self.reg(Register::A2), self.reg(Register::A3),
               self.reg(Register::A4), self.reg(Register::A5))?;
        write!(f, "a6 {:016x} a7 {:016x}  s2 {:016x}  s3 {:016x}\n",
               self.reg(Register::A6), self.reg(Register::A7),
               self.reg(Register::S2), self.reg(Register::S3))?;
        write!(f, "s4 {:016x} s5 {:016x}  s6 {:016x}  s7 {:016x}\n",
               self.reg(Register::S4), self.reg(Register::S5),
               self.reg(Register::S6), self.reg(Register::S7))?;
        write!(f, "s8 {:016x} s9 {:016x} s10 {:016x} s11 {:016x}\n",
               self.reg(Register::S8), self.reg(Register::S9),
               self.reg(Register::S10), self.reg(Register::S11))?;
        write!(f, "t3 {:016x} t4 {:016x}  t5 {:016x}  t6 {:016x}\n",
               self.reg(Register::T3), self.reg(Register::T4),
               self.reg(Register::T5), self.reg(Register::T6))?;

        write!(f, "pc {:016x}", self.reg(Register::Pc))?;

        Ok(())
    }
}

fn load_binary_program(mmu: &mut Mmu) {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("test/test.bin")
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
    core.set_reg(Register::Sp, 1 * 1024 * 1024);

    core.step();

    println!("{:#x?}", core);
}
