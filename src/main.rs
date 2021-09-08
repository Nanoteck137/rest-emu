//! Rest-EMU is a RISC-V emulator so we can emulate and define a custom
//! RISC-V cpu

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
struct RType {
    funct7: u32,
    funct3: u32,
    rd: Register,
    rs1: Register,
    rs2: Register,
}

impl From<u32> for RType {
    fn from(value: u32) -> Self {
        let funct7 = (value >> 25) & 0b1111111;

        let rs2 = Register::from((value >> 20) & 0b11111);
        let rs1 = Register::from((value >> 15) & 0b11111);

        let funct3 = (value >> 12) & 0b111;

        let rd = Register::from((value >> 7) & 0b11111);

        Self {
            funct7,
            funct3,
            rd,
            rs1,
            rs2
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

#[derive(Debug)]
struct SType {
    imm: i32,
    funct3: u32,
    rs1: Register,
    rs2: Register
}

impl From<u32> for SType {
    fn from(value: u32) -> Self {
        let imm115 = (value >> 25) & 0b1111111;
        let imm40 = (value >> 7) & 0b11111;

        let imm = (imm115 << 5) | imm40;
        let imm = ((imm as i32) << 20) >> 20;

        let funct3 = (value >> 12) & 0b111;
        let rs1 = Register::from((value >> 15) & 0b11111);
        let rs2 = Register::from((value >> 20) & 0b11111);

        Self {
            imm,
            funct3,
            rs1,
            rs2
        }
    }
}

#[derive(Debug)]
struct BType {
    imm: i32,
    funct3: u32,
    rs1: Register,
    rs2: Register
}

impl From<u32> for BType {
    fn from(value: u32) -> Self {
        let imm12 = (value >> 31) & 0b1;
        let imm105 = (value >> 25) & 0b111111;
        let imm41 = (value >> 8) & 0b1111;
        let imm11 = (value >> 7) & 0b1;

        let imm = (imm12 >> 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);
        let imm = ((imm as i32) << 19) >> 19;

        let rs1 = Register::from((value >> 20) & 0b11111);
        let rs2 = Register::from((value >> 15) & 0b11111);

        let funct3 = (value >> 12) & 0b111;

        Self {
            imm,
            funct3,
            rs1,
            rs2
        }
    }
}

#[derive(Debug)]
struct UType {
    imm: i32,
    rd: Register
}

impl From<u32> for UType {
    fn from(value: u32) -> Self {
        let imm = (value & !0xfff) as i32;
        let rd = Register::from((value >> 7) & 0b11111);

        Self {
            imm,
            rd
        }
    }
}

#[derive(Debug)]
struct JType {
    imm: i32,
    rd: Register,
}

impl From<u32> for JType {
    fn from(value: u32) -> Self {
        let imm20 = (value >> 31) & 0b1;
        let imm101 = (value >> 21) & 0b1111111111;
        let imm11 = (value >> 20) & 0b1;
        let imm1912 = (value >> 12) & 0b11111111;

        let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) |
            (imm101 << 1);
        let imm = ((imm as i32) << 11) >> 11;

        let rd = Register::from((value >> 7) & 0b11111);

        Self {
            imm,
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
        let current_pc = self.reg(Register::Pc);

        let inst = self.fetch_u32();
        let inst = Instruction::decode(inst);
        println!("Instruction: {:?}", inst);

        match inst {
            Instruction::Lui { rd, imm } => {
                self.set_reg(rd, imm as i64 as u64);
            },

            Instruction::Auipc { rd, imm } => {
                let value = (imm as i64 as u64).wrapping_add(current_pc);
                self.set_reg(rd, value);
            },

            Instruction::Jal { rd, imm } => {
                let target = current_pc.wrapping_add(imm as i64 as u64);
                let return_addr = self.reg(Register::Pc);

                self.set_reg(rd, return_addr);
                self.set_reg(Register::Pc, target);
            },

            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let return_addr = self.reg(Register::Pc);
                self.set_reg(rd, return_addr);
                self.set_reg(Register::Pc, target);
            },

            // Beq { rs1: Register, rs2: Register, imm: i32 } => {},
            // Bne { rs1: Register, rs2: Register, imm: i32 } => {},
            // Blt { rs1: Register, rs2: Register, imm: i32 } => {},
            // Bge { rs1: Register, rs2: Register, imm: i32 } => {},
            // Bltu { rs1: Register, rs2: Register, imm: i32 } => {},
            // Bgeu { rs1: Register, rs2: Register, imm: i32 } => {},

            Instruction::Lb { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u8(addr);
                self.set_reg(rd, value as i8 as i64 as u64);
            },

            Instruction::Lh { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u16(addr);
                self.set_reg(rd, value as i16 as i64 as u64);
            },

            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u32(addr);
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Lbu { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u8(addr);
                self.set_reg(rd, value as u64);
            },

            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u16(addr);
                self.set_reg(rd, value as u64);
            },

            Instruction::Lwu { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u32(addr);
                self.set_reg(rd, value as u64);
            },

            Instruction::Ld { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u64(addr);
                self.set_reg(rd, value as i64 as u64);
            },

            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2) as u8;
                self.mmu.write_u8(addr, value);
            },

            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2) as u16;
                self.mmu.write_u16(addr, value);
            },

            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2) as u32;
                self.mmu.write_u32(addr, value);
            },

            Instruction::Sd { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2);
                self.mmu.write_u64(addr, value);
            },

            Instruction::Addi { rd, rs1, imm } => {
                let value = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                self.set_reg(rd, value);
            },

            Instruction::Slti { rd, rs1, imm } => {
                let rs1 = self.reg(rs1) as i64;
                let imm = imm as i64;

                if rs1 < imm {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }
            },
            Instruction::Sltiu { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                if rs1 < imm {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }
            },

            Instruction::Xori { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                let value = rs1 ^ imm;
                self.set_reg(rd, value);
            },

            Instruction::Ori { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                let value = rs1 | imm;
                self.set_reg(rd, value);
            },

            Instruction::Andi { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                let value = rs1 & imm;
                self.set_reg(rd, value);
            },

            Instruction::Slli { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1);

                let value = rs1 << shamt;
                self.set_reg(rd, value);
            },

            Instruction::Srli { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1);

                let value = rs1 >> shamt;
                self.set_reg(rd, value);
            },

            Instruction::Srai { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as i64;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as u64);
            },

            Instruction::Addiw { rd, rs1, imm } => {
                let rs1 = self.reg(rs1) as u32;
                let imm = imm as u32;

                let value = rs1.wrapping_add(imm);
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Slliw { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as u32;

                let value = rs1 << shamt;
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Srliw { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as u32;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Sraiw { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as u32 as i32;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as i64 as u64);
            },

            Instruction::Add { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1.wrapping_add(rs2);
                self.set_reg(rd, value);
            },

            Instruction::Sub { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1.wrapping_sub(rs2);
                self.set_reg(rd, value);
            },

            Instruction::Sll { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let shamt = rs2 & 0b111111;

                let value = rs1 << shamt;
                self.set_reg(rd, value);
            },

            Instruction::Slt { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as i64;
                let rs2 = self.reg(rs2) as i64;

                if rs1 < rs2 {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }
            },

            Instruction::Sltu { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                if rs1 < rs2 {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }
            },

            Instruction::Xor { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1 ^ rs2;
                self.set_reg(rd, value);
            },

            Instruction::Srl { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);
                let shamt = rs2 & 0b111111;

                let value = rs1 >> shamt;
                self.set_reg(rd, value);
            },

            Instruction::Sra { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as i64;
                let rs2 = self.reg(rs2);
                let shamt = rs2 & 0b111111;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as u64);
            },

            Instruction::Or { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1 | rs2;
                self.set_reg(rd, value as u64);
            },

            Instruction::And { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1 & rs2;
                self.set_reg(rd, value as u64);
            },

            Instruction::Addw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;
                let value = rs1.wrapping_add(rs2);
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Subw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;
                let value = rs1.wrapping_sub(rs2);
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Sllw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;

                let shamt = rs2 & 0b11111;
                let value = (rs1 << shamt);
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Srlw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;

                let shamt = rs2 & 0b11111;
                let value = (rs1 >> shamt);
                self.set_reg(rd, value as i32 as i64 as u64);
            },

            Instruction::Sraw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2);

                let shamt = rs2 & 0b11111;
                let value = (rs1 >> shamt);
                self.set_reg(rd, value as i64 as u64);
            },

            // Fence { rd: Register, rs1: Register, imm: i32 } => {},

            // Ecall,
            // Ebreak,

            Instruction::Undefined(inst) => {
                panic!("Undefined Instruction: {:#x}", inst);
            }

            _ => unimplemented!("Unimplemented instruction: {:?}", inst),
        }

        return;
        let inst = 0;
        let opcode = inst & 0b1111111;

        match opcode {
            0b0110111 => {
                // LUI

                let inst = UType::from(inst);
                println!("Inst: {:#x?}", inst);

                self.set_reg(inst.rd, inst.imm as i64 as u64);
            }

            0b1100111 => {
                // JALR

                let inst = IType::from(inst);
                println!("Inst: {:#x?}", inst);

                let target = self.reg(inst.rs1)
                    .wrapping_add(inst.imm as i64 as u64);

                let return_addr = self.reg(Register::Pc);
                self.set_reg(inst.rd, return_addr);
                self.set_reg(Register::Pc, target);
            }

            0b0000011 => {
                let inst = IType::from(inst);

                let addr = self.reg(inst.rs1)
                    .wrapping_add(inst.imm as i64 as u64);

                match inst.funct3 {
                    0b010 => {
                        // LW

                        let val = self.mmu.read_u32(addr);
                        self.set_reg(inst.rd, val as i64 as u64);
                    }

                    0b110 => {
                        // LWU

                        let val = self.mmu.read_u32(addr);
                        self.set_reg(inst.rd, val as u64)
                    }

                    0b011 => {
                        // LD
                        let val = self.mmu.read_u64(addr);
                        self.set_reg(inst.rd, val as i64 as u64);
                    }

                    _ => panic!("Unimplemented funct3 '0b{:03b}' at 0x{:016x}",
                                inst.funct3, current_pc),
                }
            }

            0b0100011 => {
                let inst = SType::from(inst);

                let addr = self.reg(inst.rs1)
                    .wrapping_add(inst.imm as i64 as u64);
                println!("Address: {:#x?}", addr);

                match inst.funct3 {
                    0b010 => {
                        // SW

                        let val = self.reg(inst.rs2) as u32;
                        self.mmu.write_u32(addr, val);
                    }

                    0b011 => {
                        // SD
                        let val = self.reg(inst.rs2);
                        self.mmu.write_u64(addr, val);
                    }

                    _ => panic!("Unimplemented funct3 '0b{:03b}' at 0x{:016x}",
                                inst.funct3, current_pc),
                }
            },

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
                    _ => panic!("Unimplemented funct3 '0b{:03b}' at 0x{:016x}",
                                inst.funct3, current_pc),
                }
            }

            0b0011011 => {
                let inst = IType::from(inst);

                match inst.funct3 {
                    0b000 => {
                        // ADDIW

                        let rs1 = self.reg(inst.rs1) as u32;
                        let imm = inst.imm as u32;

                        let val = rs1.wrapping_add(imm) as i32 as i64 as u64;
                        self.set_reg(inst.rd, val);
                    }
                    _ => panic!("Unimplemented funct3 '0b{:03b}' at 0x{:016x}",
                                inst.funct3, current_pc),
                }
            }

            0b0111011 => {
                let inst = RType::from(inst);

                let rs1 = self.reg(inst.rs1) as u32;
                let rs2 = self.reg(inst.rs2) as u32;

                match (inst.funct7, inst.funct3) {
                    (0b0000000, 0b000) => {
                        // ADDW

                        let value = rs1.wrapping_add(rs2);
                        self.set_reg(inst.rd, value as i32 as i64 as u64);
                    }

                    _ => panic!("Unimplemented funct3 '0b{:03b}' funct7 '0b{:07b} at 0x{:016x}",
                                inst.funct3, inst.funct7, current_pc),
                }
            }

            _ => panic!("Unknown opcode '0b{:07b}' at 0x{:016x}",
                        opcode, current_pc),
        }
        println!("Executing inst: {:#x} Opcode: 0b{:07b}", inst, opcode);
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

#[derive(Debug)]
enum Instruction {
    // 0b0110111
    Lui { rd: Register, imm: i32 },

    // 0b0010111
    Auipc { rd: Register, imm: i32 },

    // 0b1101111
    Jal { rd: Register, imm: i32 },

    // 0b1100111
    Jalr { rd: Register, rs1: Register, imm: i32 },

    // 0b1100011
    Beq { rs1: Register, rs2: Register, imm: i32 },
    Bne { rs1: Register, rs2: Register, imm: i32 },
    Blt { rs1: Register, rs2: Register, imm: i32 },
    Bge { rs1: Register, rs2: Register, imm: i32 },
    Bltu { rs1: Register, rs2: Register, imm: i32 },
    Bgeu { rs1: Register, rs2: Register, imm: i32 },

    // 0b0000011
    Lb { rd: Register, rs1: Register, imm: i32 },
    Lh { rd: Register, rs1: Register, imm: i32 },
    Lw { rd: Register, rs1: Register, imm: i32 },
    Lbu { rd: Register, rs1: Register, imm: i32 },
    Lhu { rd: Register, rs1: Register, imm: i32 },
    Lwu { rd: Register, rs1: Register, imm: i32 },
    Ld { rd: Register, rs1: Register, imm: i32 },

    // 0b0100011
    Sb { rs1: Register, rs2: Register, imm: i32 },
    Sh { rs1: Register, rs2: Register, imm: i32 },
    Sw { rs1: Register, rs2: Register, imm: i32 },
    Sd { rs1: Register, rs2: Register, imm: i32 },

    // 0b0010011
    Addi   { rd: Register, rs1: Register, imm: i32 },
    Slti   { rd: Register, rs1: Register, imm: i32 },
    Sltiu  { rd: Register, rs1: Register, imm: i32 },
    Xori   { rd: Register, rs1: Register, imm: i32 },
    Ori    { rd: Register, rs1: Register, imm: i32 },
    Andi   { rd: Register, rs1: Register, imm: i32 },
    Slli   { rd: Register, rs1: Register, shamt: i32 },
    Srli   { rd: Register, rs1: Register, shamt: i32 },
    Srai   { rd: Register, rs1: Register, shamt: i32 },

    // 0b0011011
    Addiw { rd: Register, rs1: Register, imm: i32 },
    Slliw { rd: Register, rs1: Register, shamt: i32 },
    Srliw { rd: Register, rs1: Register, shamt: i32 },
    Sraiw { rd: Register, rs1: Register, shamt: i32 },

    // 0b0110011
    Add  { rd: Register, rs1: Register, rs2: Register },
    Sub  { rd: Register, rs1: Register, rs2: Register },
    Sll  { rd: Register, rs1: Register, rs2: Register },
    Slt  { rd: Register, rs1: Register, rs2: Register },
    Sltu { rd: Register, rs1: Register, rs2: Register },
    Xor  { rd: Register, rs1: Register, rs2: Register },
    Srl  { rd: Register, rs1: Register, rs2: Register },
    Sra  { rd: Register, rs1: Register, rs2: Register },
    Or   { rd: Register, rs1: Register, rs2: Register },
    And  { rd: Register, rs1: Register, rs2: Register },

    // 0b0111011
    Addw { rd: Register, rs1: Register, rs2: Register },
    Subw { rd: Register, rs1: Register, rs2: Register },
    Sllw { rd: Register, rs1: Register, rs2: Register },
    Srlw { rd: Register, rs1: Register, rs2: Register },
    Sraw { rd: Register, rs1: Register, rs2: Register },

    // 0b0001111
    Fence { rd: Register, rs1: Register, imm: i32 },

    // 0b1110011
    Ecall,
    Ebreak,

    Undefined(u32),
}

impl Instruction {
    fn decode(inst: u32) -> Self {
        let opcode = inst & 0b1111111;

        if let Some(typ) = TYPE_MAPPING_TABLE[opcode as usize] {
            return match typ {
                Type::R => Self::decode_r(inst, opcode),
                Type::I => Self::decode_i(inst, opcode),
                Type::S => Self::decode_s(inst, opcode),
                Type::B => Self::decode_b(inst, opcode),
                Type::U => Self::decode_u(inst, opcode),
                Type::J => Self::decode_j(inst, opcode),
            }
        }

        Instruction::Undefined(inst)
    }

    fn decode_r(original_inst: u32, opcode: u32) -> Self {
        let inst = RType::from(original_inst);
        let rd = inst.rd;
        let rs1 = inst.rs1;
        let rs2 = inst.rs2;

        return match opcode {
            0b0110011 => {
                match (inst.funct7, inst.funct3) {
                    (0b0000000, 0b000) => Instruction::Add  { rd, rs1, rs2 },
                    (0b0100000, 0b000) => Instruction::Sub  { rd, rs1, rs2 },
                    (0b0000000, 0b001) => Instruction::Sll  { rd, rs1, rs2 },
                    (0b0000000, 0b010) => Instruction::Slt  { rd, rs1, rs2 },
                    (0b0000000, 0b011) => Instruction::Sltu { rd, rs1, rs2 },
                    (0b0000000, 0b100) => Instruction::Xor  { rd, rs1, rs2 },
                    (0b0000000, 0b101) => Instruction::Srl  { rd, rs1, rs2 },
                    (0b0100000, 0b101) => Instruction::Sra  { rd, rs1, rs2 },
                    (0b0000000, 0b110) => Instruction::Or   { rd, rs1, rs2 },
                    (0b0000000, 0b111) => Instruction::And  { rd, rs1, rs2 },

                    _ => Instruction::Undefined(original_inst)
                }
            }

            0b0111011 => {
                match (inst.funct7, inst.funct3) {
                    (0b0000000, 0b000) => Instruction::Addw { rd, rs1, rs2 },
                    (0b0100000, 0b000) => Instruction::Subw { rd, rs1, rs2 },
                    (0b0000000, 0b001) => Instruction::Sllw { rd, rs1, rs2 },
                    (0b0000000, 0b101) => Instruction::Srlw { rd, rs1, rs2 },
                    (0b0100000, 0b101) => Instruction::Sraw { rd, rs1, rs2 },

                    _ => Instruction::Undefined(original_inst)
                }
            }

            _ => Instruction::Undefined(original_inst)
        }
    }

    fn decode_i(original_inst: u32, opcode: u32) -> Self {
        let inst = IType::from(original_inst);
        let rd = inst.rd;
        let rs1 = inst.rs1;
        let imm = inst.imm;

        return match opcode {
            0b1100111 => {
                return match inst.funct3 {
                    0b000 => Instruction::Jalr { rd, rs1, imm },

                    _ => Instruction::Undefined(original_inst),
                };
            }

            0b0000011 => {
                return match inst.funct3 {
                    0b000 => Instruction::Lb { rd, rs1, imm },
                    0b001 => Instruction::Lh { rd, rs1, imm },
                    0b010 => Instruction::Lw { rd, rs1, imm },
                    0b100 => Instruction::Lbu { rd, rs1, imm },
                    0b101 => Instruction::Lhu { rd, rs1, imm },
                    0b110 => Instruction::Lwu { rd, rs1, imm },
                    0b011 => Instruction::Ld { rd, rs1, imm },

                    _ => Instruction::Undefined(original_inst),
                };
            }

            0b0010011 => {
                return match inst.funct3 {
                    0b000 => Instruction::Addi { rd, rs1, imm },
                    0b010 => Instruction::Slti { rd, rs1, imm },
                    0b011 => Instruction::Sltiu { rd, rs1, imm },
                    0b100 => Instruction::Xori { rd, rs1, imm },
                    0b110 => Instruction::Ori { rd, rs1, imm },
                    0b111 => Instruction::Andi { rd, rs1, imm },

                    0b001 => {
                        let shamt = inst.imm & 0b111111;
                        Instruction::Slli { rd, rs1, shamt }
                    },

                    0b101 => {
                        let shamt = inst.imm & 0b111111;
                        let mode = (inst.imm >> 6) & 0b111111;

                        return match mode {
                            0b000000 => Instruction::Srli { rd, rs1, shamt },
                            0b010000 => Instruction::Srai { rd, rs1, shamt },

                            _ => Instruction::Undefined(original_inst),
                        };
                    }

                    _ => Instruction::Undefined(original_inst),
                };
            }

            0b0011011 => {
                return match inst.funct3 {
                    0b000 => Instruction::Addiw { rd, rs1, imm },
                    0b001 => {
                        let shamt = inst.imm & 0b111111;
                        Instruction::Slliw { rd, rs1, shamt }
                    }
                    0b101 => {
                        let shamt = inst.imm & 0b111111;
                        let mode = (inst.imm >> 6) & 0b111111;

                        return match mode {
                            0b000000 => Instruction::Srliw { rd, rs1, shamt },
                            0b010000 => Instruction::Sraiw { rd, rs1, shamt },

                            _ => Instruction::Undefined(original_inst),
                        };
                    }
                    _ => Instruction::Undefined(original_inst),
                }
            }

            0b0001111 => {
                return match inst.funct3 {
                    0b000 => Instruction::Fence { rd, rs1, imm },

                    _ => Instruction::Undefined(original_inst),
                };
            }

            0b1110011 => {
                return if imm == 0 {
                    Instruction::Ecall
                } else if imm == 1 {
                    Instruction::Ebreak
                } else {
                    Instruction::Undefined(original_inst)
                };
            }

            _ => Instruction::Undefined(original_inst),
        }
    }

    fn decode_s(original_inst: u32, opcode: u32) -> Self {
        let inst = SType::from(original_inst);
        let rs1 = inst.rs1;
        let rs2 = inst.rs2;
        let imm = inst.imm;

        return match opcode {
            0b0100011 => {
                return match inst.funct3 {
                    0b000 => Instruction::Sb { rs1, rs2, imm },
                    0b001 => Instruction::Sh { rs1, rs2, imm },
                    0b010 => Instruction::Sw { rs1, rs2, imm },
                    0b011 => Instruction::Sd { rs1, rs2, imm },
                    _ => Instruction::Undefined(original_inst)
                };
            }

            _ => Instruction::Undefined(original_inst)
        };

    }

    fn decode_b(original_inst: u32, opcode: u32) -> Self {
        let inst = BType::from(original_inst);
        let rs1 = inst.rs1;
        let rs2 = inst.rs2;
        let imm = inst.imm;

        return match opcode {
            0b1100011 => {
                match inst.funct3 {
                    0b000 => Instruction::Beq { rs1, rs2, imm },
                    0b001 => Instruction::Bne { rs1, rs2, imm },
                    0b100 => Instruction::Blt { rs1, rs2, imm },
                    0b101 => Instruction::Bge { rs1, rs2, imm },
                    0b110 => Instruction::Bltu { rs1, rs2, imm },
                    0b111 => Instruction::Bgeu { rs1, rs2, imm },

                    _ => Instruction::Undefined(original_inst)
                }
            },

            _ => Instruction::Undefined(original_inst)
        };
    }

    fn decode_u(original_inst: u32, opcode: u32) -> Self {
        let inst = UType::from(original_inst);
        let rd = inst.rd;
        let imm = inst.imm;

        return match opcode {
            0b0110111 => Instruction::Lui { rd, imm },
            0b0010111 => Instruction::Auipc { rd, imm },

            _ => Instruction::Undefined(original_inst),
        };
    }

    fn decode_j(original_inst: u32, opcode: u32) -> Self {
        let inst = JType::from(original_inst);
        let imm = inst.imm;
        let rd = inst.rd;

        return match opcode {
            0b1101111 => Instruction::Jal { rd, imm },
            _ => Instruction::Undefined(original_inst),
        };
    }
}

#[derive(Copy, Clone, Debug)]
enum Type {
    R,
    I,
    S,
    B,
    U,
    J
}

static TYPE_MAPPING_TABLE: [Option<Type>; 128] = [
    None,          // 0b0000000
    None,          // 0b0000001
    None,          // 0b0000010
    Some(Type::I), // 0b0000011
    None,          // 0b0000100
    None,          // 0b0000101
    None,          // 0b0000110
    None,          // 0b0000111
    None,          // 0b0001000
    None,          // 0b0001001
    None,          // 0b0001010
    None,          // 0b0001011
    None,          // 0b0001100
    None,          // 0b0001101
    None,          // 0b0001110
    Some(Type::I), // 0b0001111
    None,          // 0b0010000
    None,          // 0b0010001
    None,          // 0b0010010
    Some(Type::I), // 0b0010011
    None,          // 0b0010100
    None,          // 0b0010101
    None,          // 0b0010110
    Some(Type::U), // 0b0010111
    None,          // 0b0011000
    None,          // 0b0011001
    None,          // 0b0011010
    Some(Type::I), // 0b0011011
    None,          // 0b0011100
    None,          // 0b0011101
    None,          // 0b0011110
    None,          // 0b0011111
    None,          // 0b0100000
    None,          // 0b0100001
    None,          // 0b0100010
    Some(Type::S), // 0b0100011
    None,          // 0b0100100
    None,          // 0b0100101
    None,          // 0b0100110
    None,          // 0b0100111
    None,          // 0b0101000
    None,          // 0b0101001
    None,          // 0b0101010
    None,          // 0b0101011
    None,          // 0b0101100
    None,          // 0b0101101
    None,          // 0b0101110
    None,          // 0b0101111
    None,          // 0b0110000
    None,          // 0b0110001
    None,          // 0b0110010
    Some(Type::R), // 0b0110011
    None,          // 0b0110100
    None,          // 0b0110101
    None,          // 0b0110110
    Some(Type::U), // 0b0110111
    None,          // 0b0111000
    None,          // 0b0111001
    None,          // 0b0111010
    Some(Type::R), // 0b0111011
    None,          // 0b0111100
    None,          // 0b0111101
    None,          // 0b0111110
    None,          // 0b0111111
    None,          // 0b1000000
    None,          // 0b1000001
    None,          // 0b1000010
    None,          // 0b1000011
    None,          // 0b1000100
    None,          // 0b1000101
    None,          // 0b1000110
    None,          // 0b1000111
    None,          // 0b1001000
    None,          // 0b1001001
    None,          // 0b1001010
    None,          // 0b1001011
    None,          // 0b1001100
    None,          // 0b1001101
    None,          // 0b1001110
    None,          // 0b1001111
    None,          // 0b1010000
    None,          // 0b1010001
    None,          // 0b1010010
    None,          // 0b1010011
    None,          // 0b1010100
    None,          // 0b1010101
    None,          // 0b1010110
    None,          // 0b1010111
    None,          // 0b1011000
    None,          // 0b1011001
    None,          // 0b1011010
    None,          // 0b1011011
    None,          // 0b1011100
    None,          // 0b1011101
    None,          // 0b1011110
    None,          // 0b1011111
    None,          // 0b1100000
    None,          // 0b1100001
    None,          // 0b1100010
    Some(Type::B), // 0b1100011
    None,          // 0b1100100
    None,          // 0b1100101
    None,          // 0b1100110
    Some(Type::I), // 0b1100111
    None,          // 0b1101000
    None,          // 0b1101001
    None,          // 0b1101010
    None,          // 0b1101011
    None,          // 0b1101100
    None,          // 0b1101101
    None,          // 0b1101110
    Some(Type::J), // 0b1101111
    None,          // 0b1110000
    None,          // 0b1110001
    None,          // 0b1110010
    Some(Type::I), // 0b1110011
    None,          // 0b1110100
    None,          // 0b1110101
    None,          // 0b1110110
    None,          // 0b1110111
    None,          // 0b1111000
    None,          // 0b1111001
    None,          // 0b1111010
    None,          // 0b1111011
    None,          // 0b1111100
    None,          // 0b1111101
    None,          // 0b1111110
    None,          // 0b1111111
];

fn main() {
    let mut mmu = Mmu::new(1 * 1024 * 1024);
    load_binary_program(&mut mmu);

    let mut core = Core::new(mmu);
    core.set_reg(Register::Ra, 0xffff1337);
    core.set_reg(Register::Sp, 1 * 1024 * 1024);

    loop {
        core.step();

        if core.reg(Register::Pc) == 0xffff1337 {
            break;
        }
    }

    println!("{:#x?}", core);
}
