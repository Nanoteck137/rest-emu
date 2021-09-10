use crate::instruction::Instruction;
use crate::mmu::Mmu;

const MAX_REGISTERS: usize = 33;
const MAX_CSR_REGISTERS: usize = 4096;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Register {
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
    pub fn index(&self) -> usize {
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CoreExit {
    Success,
    Ecall,
    Ebreak
}

pub struct Core {
    registers: [u64; MAX_REGISTERS],
    csr_registers: [u64; MAX_CSR_REGISTERS],

    mmu: Mmu
}

impl Core {
    pub fn new(mmu: Mmu) -> Self {
        Self {
            registers: [0; MAX_REGISTERS],
            csr_registers: [0; MAX_CSR_REGISTERS],

            mmu
        }
    }

    pub fn step(&mut self) -> CoreExit {
        let current_pc = self.reg(Register::Pc);

        let inst = self.fetch_u32();
        let inst = Instruction::decode(inst);
        println!("Instruction: {:?}", inst);

        return match inst {
            Instruction::Lui { rd, imm } => {
                self.set_reg(rd, imm as i64 as u64);

                CoreExit::Success
            },

            Instruction::Auipc { rd, imm } => {
                let value = (imm as i64 as u64).wrapping_add(current_pc);
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Jal { rd, imm } => {
                let target = current_pc.wrapping_add(imm as i64 as u64);
                let return_addr = self.reg(Register::Pc);

                self.set_reg(rd, return_addr);
                self.set_reg(Register::Pc, target);

                CoreExit::Success
            },

            Instruction::Jalr { rd, rs1, imm } => {
                let target = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let return_addr = self.reg(Register::Pc);
                self.set_reg(rd, return_addr);
                self.set_reg(Register::Pc, target);

                CoreExit::Success
            },

            Instruction::Beq { rs1, rs2, imm } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);
                let imm = imm as i64 as u64;
                let target = current_pc.wrapping_add(imm);

                if rs1 == rs2 {
                    self.set_reg(Register::Pc, target);
                }

                CoreExit::Success
            },

            Instruction::Bne { rs1, rs2, imm } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);
                let imm = imm as i64 as u64;
                let target = current_pc.wrapping_add(imm);

                if rs1 != rs2 {
                    self.set_reg(Register::Pc, target);
                }

                CoreExit::Success
            },

            Instruction::Blt { rs1, rs2, imm } => {
                let rs1 = self.reg(rs1) as i64;
                let rs2 = self.reg(rs2) as i64;
                let imm = imm as i64 as u64;
                let target = current_pc.wrapping_add(imm);

                if rs1 < rs2 {
                    self.set_reg(Register::Pc, target);
                }

                CoreExit::Success
            },

            Instruction::Bge { rs1, rs2, imm } => {
                let rs1 = self.reg(rs1) as i64;
                let rs2 = self.reg(rs2) as i64;
                let imm = imm as i64 as u64;
                let target = current_pc.wrapping_add(imm);

                if rs1 >= rs2 {
                    self.set_reg(Register::Pc, target);
                }

                CoreExit::Success
            },

            Instruction::Bltu { rs1, rs2, imm } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);
                let imm = imm as i64 as u64;
                let target = current_pc.wrapping_add(imm);

                if rs1 < rs2 {
                    self.set_reg(Register::Pc, target);
                }

                CoreExit::Success
            },

            Instruction::Bgeu { rs1, rs2, imm } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);
                let imm = imm as i64 as u64;
                let target = current_pc.wrapping_add(imm);

                if rs1 >= rs2 {
                    self.set_reg(Register::Pc, target);
                }

                CoreExit::Success
            },

            Instruction::Lb { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u8(addr);
                self.set_reg(rd, value as i8 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Lh { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u16(addr);
                self.set_reg(rd, value as i16 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u32(addr);
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Lbu { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u8(addr);
                self.set_reg(rd, value as u64);

                CoreExit::Success
            },

            Instruction::Lhu { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u16(addr);
                self.set_reg(rd, value as u64);

                CoreExit::Success
            },

            Instruction::Lwu { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u32(addr);
                self.set_reg(rd, value as u64);

                CoreExit::Success
            },

            Instruction::Ld { rd, rs1, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.mmu.read_u64(addr);
                self.set_reg(rd, value as i64 as u64);

                CoreExit::Success
            },

            Instruction::Sb { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2) as u8;
                self.mmu.write_u8(addr, value);

                CoreExit::Success
            },

            Instruction::Sh { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2) as u16;
                self.mmu.write_u16(addr, value);

                CoreExit::Success
            },

            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2) as u32;
                self.mmu.write_u32(addr, value);

                CoreExit::Success
            },

            Instruction::Sd { rs1, rs2, imm } => {
                let addr = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);

                let value = self.reg(rs2);
                self.mmu.write_u64(addr, value);

                CoreExit::Success
            },

            Instruction::Addi { rd, rs1, imm } => {
                let value = self.reg(rs1)
                    .wrapping_add(imm as i64 as u64);
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Slti { rd, rs1, imm } => {
                let rs1 = self.reg(rs1) as i64;
                let imm = imm as i64;

                if rs1 < imm {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }

                CoreExit::Success
            },
            Instruction::Sltiu { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                if rs1 < imm {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }

                CoreExit::Success
            },

            Instruction::Xori { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                let value = rs1 ^ imm;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Ori { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                let value = rs1 | imm;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Andi { rd, rs1, imm } => {
                let rs1 = self.reg(rs1);
                let imm = imm as i64 as u64;

                let value = rs1 & imm;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Slli { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1);

                let value = rs1 << shamt;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Srli { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1);

                let value = rs1 >> shamt;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Srai { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as i64;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as u64);

                CoreExit::Success
            },

            Instruction::Addiw { rd, rs1, imm } => {
                let rs1 = self.reg(rs1) as u32;
                let imm = imm as u32;

                let value = rs1.wrapping_add(imm);
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Slliw { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as u32;

                let value = rs1 << shamt;
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Srliw { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as u32;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Sraiw { rd, rs1, shamt } => {
                let rs1 = self.reg(rs1) as u32 as i32;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as i64 as u64);

                CoreExit::Success
            },

            Instruction::Add { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1.wrapping_add(rs2);
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Sub { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1.wrapping_sub(rs2);
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Sll { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let shamt = rs2 & 0b111111;

                let value = rs1 << shamt;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Slt { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as i64;
                let rs2 = self.reg(rs2) as i64;

                if rs1 < rs2 {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }

                CoreExit::Success
            },

            Instruction::Sltu { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                if rs1 < rs2 {
                    self.set_reg(rd, 1);
                } else {
                    self.set_reg(rd, 0);
                }

                CoreExit::Success
            },

            Instruction::Xor { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1 ^ rs2;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Srl { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);
                let shamt = rs2 & 0b111111;

                let value = rs1 >> shamt;
                self.set_reg(rd, value);

                CoreExit::Success
            },

            Instruction::Sra { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as i64;
                let rs2 = self.reg(rs2);
                let shamt = rs2 & 0b111111;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as u64);

                CoreExit::Success
            },

            Instruction::Or { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1 | rs2;
                self.set_reg(rd, value as u64);

                CoreExit::Success
            },

            Instruction::And { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1);
                let rs2 = self.reg(rs2);

                let value = rs1 & rs2;
                self.set_reg(rd, value as u64);

                CoreExit::Success
            },

            Instruction::Addw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;

                let value = rs1.wrapping_add(rs2);
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Subw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;

                let value = rs1.wrapping_sub(rs2);
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Sllw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;
                let shamt = rs2 & 0b11111;

                let value = rs1 << shamt;
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Srlw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2) as u32;
                let shamt = rs2 & 0b11111;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as i32 as i64 as u64);

                CoreExit::Success
            },

            Instruction::Sraw { rd, rs1, rs2 } => {
                let rs1 = self.reg(rs1) as u32;
                let rs2 = self.reg(rs2);
                let shamt = rs2 & 0b11111;

                let value = rs1 >> shamt;
                self.set_reg(rd, value as i64 as u64);

                CoreExit::Success
            },

            // Fence { rd: Register, rs1: Register, imm: i32 } => {},

            Instruction::Ecall => CoreExit::Ecall,
            Instruction::Ebreak => CoreExit::Ebreak,

            Instruction::Csrrw { rd, rs1, csr } => {
                // NOTE(patrik): Doing this because the spec says that if the
                // Zero/x0 register is used for rd then don't read the csr
                // and if we change self.read_csr to have side effect we don't
                // want those to effect when the user if not reading the csr
                // but this current implementation dosen't have side effects
                // inside read_csr
                if rd != Register::Zero {
                    let value = self.read_csr(csr);
                    self.set_reg(rd, value);
                }

                let rs1 = self.reg(rs1);
                self.write_csr(csr, rs1);

                CoreExit::Success
            },

            Instruction::Csrrs { rd, rs1, csr } => {
                let value = self.read_csr(csr);
                self.set_reg(rd, value);

                // NOTE(patrik): If rs1 is the zero/x0 register then don't
                // write to the csr
                if rs1 != Register::Zero {
                    let rs1 = self.reg(rs1);
                    self.write_csr(csr, value | rs1);
                }

                CoreExit::Success
            },

            Instruction::Csrrc { rd, rs1, csr } => {
                let value = self.read_csr(csr);
                self.set_reg(rd, value);

                // NOTE(patrik): If rs1 is the zero/x0 register then don't
                // write to the csr
                if rs1 != Register::Zero {
                    let rs1 = self.reg(rs1);
                    self.write_csr(csr, value & !rs1);
                }

                CoreExit::Success
            },

            Instruction::Csrrwi { rd, uimm, csr } => {
                // NOTE(patrik): Doing this because the spec says that if the
                // Zero/x0 register is used for rd then don't read the csr
                // and if we change self.read_csr to have side effect we don't
                // want those to effect when the user if not reading the csr
                // but this current implementation dosen't have side effects
                // inside read_csr
                if rd != Register::Zero {
                    let value = self.read_csr(csr);
                    self.set_reg(rd, value);
                }

                self.write_csr(csr, uimm as u64);

                CoreExit::Success
            },

            Instruction::Csrrsi { rd, uimm, csr } => {
                let value = self.read_csr(csr);
                self.set_reg(rd, value);

                // NOTE(patrik): If uimm is 0 then don't write to csr
                if uimm != 0 {
                    let uimm = uimm as u64;
                    self.write_csr(csr, value | uimm);
                }

                CoreExit::Success
            },

            Instruction::Csrrci { rd, uimm, csr } => {
                let value = self.read_csr(csr);
                self.set_reg(rd, value);

                // NOTE(patrik): If uimm is 0 then don't write to csr
                if uimm != 0 {
                    let uimm = uimm as u64;
                    self.write_csr(csr, value & !uimm);
                }

                CoreExit::Success
            },

            Instruction::Undefined(inst) => {
                // TODO(patrik): Print more infomation about the inst
                //               like the type from the type table if it
                //               exists
                panic!("Undefined Instruction: {:#x}", inst);
            }

            _ => unimplemented!("Unimplemented instruction: {:?}", inst),
        };
    }

    pub fn write_csr(&mut self, csr: u16, value: u64) {
        let csr = csr as usize;

        if csr >= MAX_CSR_REGISTERS {
            panic!("write_csr: csr address is over the max 4096 limit: {}",
                   csr);
        }

        self.csr_registers[csr as usize] = value;
    }

    pub fn read_csr(&self, csr: u16) -> u64 {
        let csr = csr as usize;

        if csr >= MAX_CSR_REGISTERS {
            panic!("read_csr: csr address is over the max 4096 limit: {}",
                   csr);
        }

        self.csr_registers[csr as usize]
    }

    fn fetch_u32(&mut self) -> u32 {
        let pc = self.reg(Register::Pc);
        let result = self.mmu.read_u32(pc);
        self.set_reg(Register::Pc, pc + 4);

        result
    }

    pub fn set_reg(&mut self, reg: Register, value: u64) {
        if reg != Register::Zero {
            self.registers[reg.index()] = value;
        }
    }

    pub fn reg(&self, reg: Register) -> u64 {
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
