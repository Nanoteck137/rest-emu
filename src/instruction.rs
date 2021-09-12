use crate::Register;

#[derive(Copy, Clone, Debug)]
pub enum Type {
    R,
    I,
    S,
    B,
    U,
    J
}

#[derive(Debug)]
pub enum Instruction {
    // 0b0110111
    Lui { rd: Register, imm: i32 },

    // 0b0010111
    Auipc { rd: Register, imm: i32 },

    // 0b1101111
    Jal { rd: Register, imm: i32 },

    // 0b1100111
    Jalr { rd: Register, rs1: Register, imm: i32 },

    // 0b1100011
    Beq  { rs1: Register, rs2: Register, imm: i32 },
    Bne  { rs1: Register, rs2: Register, imm: i32 },
    Blt  { rs1: Register, rs2: Register, imm: i32 },
    Bge  { rs1: Register, rs2: Register, imm: i32 },
    Bltu { rs1: Register, rs2: Register, imm: i32 },
    Bgeu { rs1: Register, rs2: Register, imm: i32 },

    // 0b0000011
    Lb  { rd: Register, rs1: Register, imm: i32 },
    Lh  { rd: Register, rs1: Register, imm: i32 },
    Lw  { rd: Register, rs1: Register, imm: i32 },
    Lbu { rd: Register, rs1: Register, imm: i32 },
    Lhu { rd: Register, rs1: Register, imm: i32 },
    Lwu { rd: Register, rs1: Register, imm: i32 },
    Ld  { rd: Register, rs1: Register, imm: i32 },

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
    // M Extention
    Mul    { rd: Register, rs1: Register, rs2: Register },
    Mulh   { rd: Register, rs1: Register, rs2: Register },
    Mulhsu { rd: Register, rs1: Register, rs2: Register },
    Mulhu  { rd: Register, rs1: Register, rs2: Register },
    Div    { rd: Register, rs1: Register, rs2: Register },
    Divu   { rd: Register, rs1: Register, rs2: Register },
    Rem    { rd: Register, rs1: Register, rs2: Register },
    Remu   { rd: Register, rs1: Register, rs2: Register },

    // 0b0111011
    Addw { rd: Register, rs1: Register, rs2: Register },
    Subw { rd: Register, rs1: Register, rs2: Register },
    Sllw { rd: Register, rs1: Register, rs2: Register },
    Srlw { rd: Register, rs1: Register, rs2: Register },
    Sraw { rd: Register, rs1: Register, rs2: Register },
    // M Extention
    Mulw  { rd: Register, rs1: Register, rs2: Register },
    Divw  { rd: Register, rs1: Register, rs2: Register },
    Divuw { rd: Register, rs1: Register, rs2: Register },
    Remw  { rd: Register, rs1: Register, rs2: Register },
    Remuw { rd: Register, rs1: Register, rs2: Register },

    // 0b0001111
    Fence { rd: Register, rs1: Register, imm: i32 },

    // 0b1110011
    Ecall,
    Ebreak,
    Csrrw  { rd: Register, rs1: Register, csr: u16 },
    Csrrs  { rd: Register, rs1: Register, csr: u16 },
    Csrrc  { rd: Register, rs1: Register, csr: u16 },
    Csrrwi { rd: Register, uimm: u32, csr: u16 },
    Csrrsi { rd: Register, uimm: u32, csr: u16 },
    Csrrci { rd: Register, uimm: u32, csr: u16 },

    // C Extention
    Hint,

    // Quad 0
    CAddi4spn { rd:  Register, nzuimm: u32 },
    CFld      { rd:  Register, rs1: Register, uimm: u32 },
    CLw       { rd:  Register, rs1: Register, uimm: u32 },
    CLd       { rd:  Register, rs1: Register, uimm: u32 },
    CFsd      { rs1: Register, rs2: Register, uimm: u32 },
    CSw       { rs1: Register, rs2: Register, uimm: u32 },
    CSd       { rs1: Register, rs2: Register, uimm: u32 },

    // Quad 1
    CNop,
    CAddi     { reg: Register, nzimm: i32 },
    CAddiw    { reg: Register, imm: i32 },
    CLi       { rd:  Register, imm: i32 },
    CAddi16sp { nzimm: i32 },
    CLui      { rd:  Register, nzimm: i32 },
    CAndi     { reg: Register, imm: i32 },
    CSub      { reg: Register, rs2: Register },
    CXor      { reg: Register, rs2: Register },
    COr       { reg: Register, rs2: Register },
    CAnd      { reg: Register, rs2: Register },
    CSubw     { reg: Register, rs2: Register },
    CAddw     { reg: Register, rs2: Register },
    CJ        { imm: i32 },
    CBeqz     { rs1: Register, imm: i32 },
    CBnez     { rs1: Register, imm: i32 },

    // Quad 2
    CSlli  { reg: Register, nzuimm: u32 },
    CFldsp { rd:  Register, uimm:   u32 },
    CLwsp  { rd:  Register, uimm:   u32 },
    CLdsp  { rd:  Register, uimm:   u32 },
    CJr    { rs1: Register },
    CMv    { rd:  Register, rs2: Register },
    CEbreak,
    CJalr  { rs1: Register },
    CAdd   { reg: Register, rs2: Register },
    CFsdsp { rs2: Register, uimm: u32 },
    CSwsp  { rs2: Register, uimm: u32 },
    CSdsp  { rs2: Register, uimm: u32 },

    Undefined(u32),
    UndefinedCompressed(u16),
}

impl Instruction {
    pub fn decode(inst: u32) -> Self {
        let opcode = inst & 0b1111111;

        if let Some(typ) = Self::decode_type(opcode) {
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

    pub fn decode_type(opcode: u32) -> Option<Type> {
         TYPE_MAPPING_TABLE[opcode as usize]
    }

    fn reg_from_prime(prime: u16) -> Register {
        match prime {
            0b000 => Register::S0,
            0b001 => Register::S1,
            0b010 => Register::A0,
            0b011 => Register::A1,
            0b100 => Register::A2,
            0b101 => Register::A3,
            0b110 => Register::A4,
            0b111 => Register::A5,

            _ => unreachable!(),
        }
    }

    pub fn decode_compressed(inst: u16) -> Self {
        let quad = inst & 0b11;
        return match quad {
            0b00 => {
                let funct3 = (inst >> 13) & 0b111;

                return match funct3 {
                    0b000 => {
                        let nzuimm45 = (inst >> 11) & 0b11;
                        let nzuimm69 = (inst >> 7)  & 0b1111;
                        let nzuimm2  = (inst >> 6) & 0b1;
                        let nzuimm3  = (inst >> 5) & 0b1;
                        let nzuimm = nzuimm69 << 6 | nzuimm45 << 4 |
                            nzuimm3 << 3 | nzuimm2 << 2;
                        let rd = (inst >> 2) & 0b111;

                        if nzuimm == 0 && rd == 0 {
                            // NOTE(patrik): Illegal Instruction
                            Instruction::UndefinedCompressed(inst)
                        } else {
                            let rd = Self::reg_from_prime(rd);
                            let nzuimm = nzuimm as u32;
                            Instruction::CAddi4spn { rd, nzuimm }
                        }
                    },

                    0b001 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm67 = (inst >> 5) & 0b11;
                        let uimm = uimm67 << 6 | uimm35 << 3;
                        let uimm = uimm as u32;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);
                        let rd = Self::reg_from_prime((inst >> 2) & 0b111);

                        Instruction::CFld { rd, rs1, uimm }
                    },

                    0b010 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm2 = (inst >> 6) & 0b1;
                        let uimm6 = (inst >> 5) & 0b1;
                        let uimm = uimm6 << 6 | uimm35 << 3 | uimm2 << 2;
                        let uimm = uimm as u32;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);
                        let rd = Self::reg_from_prime((inst >> 2) & 0b111);

                        Instruction::CLw { rd, rs1, uimm }
                    },

                    0b011 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm67 = (inst >> 5) & 0b11;
                        let uimm = uimm67 << 6 | uimm35 << 3;
                        let uimm = uimm as u32;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);
                        let rd = Self::reg_from_prime((inst >> 2) & 0b111);

                        Instruction::CLd { rd, rs1, uimm }
                    }

                    0b101 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm67 = (inst >> 5) & 0b11;
                        let uimm = uimm67 << 6 | uimm35 << 3;
                        let uimm = uimm as u32;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);
                        let rs2 = Self::reg_from_prime((inst >> 2) & 0b111);

                        Instruction::CFsd { rs1, rs2, uimm }
                    }

                    0b110 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm2 = (inst >> 6) & 0b1;
                        let uimm6 = (inst >> 5) & 0b1;
                        let uimm = uimm6 << 6 | uimm35 << 3 | uimm2 << 2;
                        let uimm = uimm as u32;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);
                        let rs2 = Self::reg_from_prime((inst >> 2) & 0b111);

                        Instruction::CSw { rs1, rs2, uimm }
                    },

                    0b111 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm67 = (inst >> 5) & 0b11;
                        let uimm = uimm67 << 6 | uimm35 << 3;
                        let uimm = uimm as u32;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);
                        let rs2 = Self::reg_from_prime((inst >> 2) & 0b111);

                        Instruction::CSd { rs1, rs2, uimm }
                    }

                    _ => Instruction::UndefinedCompressed(inst),
                };
            },

            0b01 => {
                let funct3 = (inst >> 13) & 0b111;

                match funct3 {
                    0b000 => {
                        let nzimm5 = (inst >> 12) & 0b1;
                        let nzimm04 = (inst >> 2) & 0b11111;
                        let nzimm = nzimm5 << 5 | nzimm04;
                        let nzimm = ((nzimm as i32) << 26) >> 26;

                        let reg = Register::from((inst >> 7) & 0b11111);
                        return if reg == Register::Zero {
                            if nzimm != 0 {
                                Instruction::Hint
                            } else {
                                Instruction::CNop
                            }
                        } else {
                            if nzimm == 0 {
                                Instruction::Hint
                            } else {
                                Instruction::CAddi { reg, nzimm }
                            }
                        };
                    },

                    0b001 => {
                        let imm5 = (inst >> 12) & 0b1;
                        let imm04 = (inst >> 2) & 0b11111;
                        let imm = imm5 << 5 | imm04;
                        let imm = ((imm as i32) << 26) >> 26;

                        let reg = Register::from((inst >> 7) & 0b11111);

                        if reg == Register::Zero {
                            panic!("Reserved instruction");
                        } else {
                            Instruction::CAddiw { reg, imm }
                        }
                    }

                    0b010 => {
                        let imm5 = (inst >> 12) & 0b1;
                        let imm04 = (inst >> 2) & 0b11111;
                        let imm = imm5 << 5 | imm04;
                        let imm = ((imm as i32) << 26) >> 26;

                        let rd = Register::from((inst >> 7) & 0b11111);

                        if rd == Register::Zero {
                            Instruction::Hint
                        } else {
                            Instruction::CLi { rd, imm }
                        }
                    }

                    0b011 => {
                        let rd = Register::from((inst >> 7) & 0b11111);
                        if rd == Register::Sp {
                            let nzimm9  = (inst >> 12) & 0b1;
                            let nzimm4  = (inst >> 6)  & 0b1;
                            let nzimm6  = (inst >> 5)  & 0b1;
                            let nzimm87 = (inst >> 3)  & 0b11;
                            let nzimm5  = (inst >> 2)  & 0b1;
                            let nzimm = nzimm9 << 9 | nzimm87 << 7 |
                                nzimm6 << 6 | nzimm5 << 5 | nzimm4 << 4;
                            let nzimm = ((nzimm as i32) << 22) >> 22;

                            return if nzimm == 0 {
                                panic!("Reserved Instruction");
                            } else {
                                Instruction::CAddi16sp { nzimm }
                            };
                        } else {
                            return if rd == Register::Zero {
                                Instruction::Hint
                            } else {
                                let nzimm17   = ((inst >> 12) & 0b1) as u32;
                                let nzimm1612 = ((inst >> 2)  & 0b11111) as u32;

                                let nzimm = nzimm17 << 17 | nzimm1612 << 12;
                                let nzimm = ((nzimm as i32) << 14) >> 14;

                                // TODO(patrik): Check nzimm is correct
                                panic!();

                                return if nzimm == 0 {
                                    panic!("Reserved Instruction");
                                } else {
                                    Instruction::CLui { rd, nzimm }
                                };
                            };
                        }
                    }

                    0b100 => {
                        let funct2 = (inst >> 10) & 0b11;
                        return match funct2 {
                            0b00 => {
                                panic!();
                                Instruction::Hint
                            },

                            0b01 => {
                                panic!();
                                Instruction::Hint
                            },

                            0b10 => {
                                let imm5 = (inst >> 12) & 0b1;
                                let imm04 = (inst >> 2) & 0b11111;
                                let imm = imm5 << 5 | imm04;
                                let imm = ((imm as i32) << 26) >> 26;

                                let reg =
                                    Self::reg_from_prime((inst >> 7) & 0b111);

                                Instruction::CAndi { reg, imm }
                            },
                            0b11 => {
                                let funct2 = (inst >> 5) & 0b11;
                                let bit12 = (inst >> 12) & 0b1;

                                let reg =
                                    Self::reg_from_prime((inst >> 7) & 0b111);

                                let rs2 =
                                    Self::reg_from_prime((inst >> 2) & 0b111);

                                match (bit12, funct2) {
                                    (0, 0b00) =>
                                        Instruction::CSub { reg, rs2 },
                                    (0, 0b01) =>
                                        Instruction::CXor { reg, rs2 },
                                    (0, 0b10) =>
                                        Instruction::COr { reg, rs2 },
                                    (0, 0b11) =>
                                        Instruction::CAnd { reg, rs2 },

                                    (1, 0b00) =>
                                        Instruction::CSubw { reg, rs2 },
                                    (1, 0b01) =>
                                        Instruction::CAddw { reg, rs2 },
                                    (1, 0b10) => panic!("Reserved instruction"),
                                    (1, 0b11) => panic!("Reserved instruction"),

                                    _ => unreachable!(),
                                }
                            },

                            _ => unreachable!(),
                        };
                    },

                    0b101 => {
                        let imm11 = (inst >> 12) & 0b1;
                        let imm4  = (inst >> 11) & 0b1;
                        let imm89 = (inst >> 9)  & 0b11;
                        let imm10 = (inst >> 8)  & 0b1;
                        let imm6  = (inst >> 7)  & 0b1;
                        let imm7  = (inst >> 6)  & 0b1;
                        let imm13 = (inst >> 3)  & 0b111;
                        let imm5  = (inst >> 2)  & 0b1;

                        let imm = imm11 << 11 | imm10 << 10 | imm89 << 8 |
                            imm7 << 7 | imm6 << 6 | imm5 << 5 |
                            imm4 << 4 | imm13 << 1;
                        let imm = ((imm as i32) << 20) >> 20;

                        Instruction::CJ { imm }
                    }

                    0b110 => {
                        let imm8  = (inst >> 12) & 0b1;
                        let imm34 = (inst >> 10) & 0b11;
                        let imm67 = (inst >> 5)  & 0b11;
                        let imm12 = (inst >> 3)  & 0b11;
                        let imm5  = (inst >> 2)  & 0b1;

                        let imm = imm8 << 8 | imm67 << 6 | imm5 << 5 |
                            imm34 << 3 | imm12 << 1;
                        let imm = ((imm as i32) << 23) >> 23;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);

                        Instruction::CBeqz { rs1, imm }
                    }

                    0b111 => {
                        let imm8  = (inst >> 12) & 0b1;
                        let imm34 = (inst >> 10) & 0b11;
                        let imm67 = (inst >> 5)  & 0b11;
                        let imm12 = (inst >> 3)  & 0b11;
                        let imm5  = (inst >> 2)  & 0b1;

                        let imm = imm8 << 8 | imm67 << 6 | imm5 << 5 |
                            imm34 << 3 | imm12 << 1;
                        let imm = ((imm as i32) << 23) >> 23;

                        let rs1 = Self::reg_from_prime((inst >> 7) & 0b111);

                        Instruction::CBnez { rs1, imm }
                    }

                    _ => Instruction::UndefinedCompressed(inst),
                }
            },

            0b10 => {
                let funct3 = (inst >> 13) & 0b111;

                match funct3 {
                    0b000 => {
                        let nzuimm5  = (inst >> 12) & 0b1;
                        let nzuimm04 = (inst >> 2) & 0b11111;

                        let nzuimm = nzuimm5 << 5 | nzuimm04;
                        let nzuimm = nzuimm as u32;

                        let reg = Register::from((inst >> 7) & 0b11111);

                        return if reg == Register::Zero {
                            Instruction::Hint
                        } else if nzuimm == 0 {
                            Instruction::Hint
                        } else {
                            Instruction::CSlli { reg, nzuimm }
                        };
                    },

                    0b001 => {
                        let uimm5 = (inst >> 12) & 0b1;
                        let uimm34 = (inst >> 5) & 0b11;
                        let uimm68 = (inst >> 2) & 0b111;

                        let uimm = uimm68 << 6 | uimm5 << 5 | uimm34 << 3;
                        let uimm = uimm as u32;

                        let rd = Register::from((inst >> 7) & 0b11111);

                        Instruction::CFldsp { rd, uimm }
                    },

                    0b010 => {
                        let uimm5  = (inst >> 12) & 0b1;
                        let uimm24 = (inst >> 4) & 0b111;
                        let uimm67 = (inst >> 2) & 0b11;

                        let uimm = uimm67 << 6 | uimm5 << 5 | uimm24 << 2;
                        let uimm = uimm as u32;

                        let rd = Register::from((inst >> 7) & 0b11111);

                        if rd == Register::Zero {
                            panic!("Reserved instruction");
                        }

                        Instruction::CLwsp { rd, uimm }
                    },

                    0b011 => {
                        let uimm5  = (inst >> 12) & 0b1;
                        let uimm34 = (inst >> 5)  & 0b11;
                        let uimm68 = (inst >> 2)  & 0b111;

                        let uimm = uimm68 << 6 | uimm5 << 5 | uimm34 << 3;
                        let uimm = uimm as u32;

                        let rd = Register::from((inst >> 7) & 0b11111);

                        if rd == Register::Zero {
                            panic!("Reserved Instruction");
                        }

                        Instruction::CLdsp { rd, uimm }
                    },

                    0b100 => {
                        let bit12 = (inst >> 12) & 0b1;

                        let reg = (inst >> 7) & 0b11111;
                        let rs2 = (inst >> 2) & 0b11111;

                        return if bit12 == 0 {
                            return if rs2 == 0 {
                                let rs1 = Register::from(reg);

                                Instruction::CJr { rs1 }
                            } else {
                                let rd = Register::from(reg);
                                let rs2 = Register::from(rs2);

                                Instruction::CMv { rd, rs2 }
                            };
                        } else {
                            return if reg == 0 && rs2 == 0 {
                                Instruction::CEbreak
                            } else if rs2 == 0 {
                                let rs1 = Register::from(reg);
                                Instruction::CJalr { rs1 }
                            } else {
                                let reg = Register::from(reg);
                                let rs2 = Register::from(rs2);

                                return if reg == Register::Zero {
                                    Instruction::Hint
                                } else {
                                    Instruction::CAdd { reg, rs2 }
                                };
                            };
                        };
                    },

                    0b101 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm68 = (inst >> 7) & 0b111;

                        let uimm = uimm68 << 6 | uimm35 << 3;
                        let uimm = uimm as u32;

                        let rs2 = Register::from((inst >> 2) & 0b11111);

                        Instruction::CFsdsp { rs2, uimm }
                    },

                    0b110 => {
                        let uimm25 = (inst >> 9) & 0b1111;
                        let uimm67 = (inst >> 7) & 0b11;

                        let uimm = uimm67 << 6 | uimm25 << 2;
                        let uimm = uimm as u32;

                        let rs2 = Register::from((inst >> 2) & 0b11111);

                        Instruction::CSwsp { rs2, uimm }
                    },

                    0b111 => {
                        let uimm35 = (inst >> 10) & 0b111;
                        let uimm68 = (inst >> 7) & 0b111;

                        let uimm = uimm68 << 6 | uimm35 << 3;
                        let uimm = uimm as u32;

                        let rs2 = Register::from((inst >> 2) & 0b11111);

                        Instruction::CSdsp { rs2, uimm }
                    }

                    _ => unreachable!(),
                }
            },

            _ => Instruction::UndefinedCompressed(inst)
        };
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

                    // M Extention

                    (0b0000001, 0b000) => Instruction::Mul    { rd, rs1, rs2 },
                    (0b0000001, 0b001) => Instruction::Mulh   { rd, rs1, rs2 },
                    (0b0000001, 0b010) => Instruction::Mulhsu { rd, rs1, rs2 },
                    (0b0000001, 0b011) => Instruction::Mulhu  { rd, rs1, rs2 },
                    (0b0000001, 0b100) => Instruction::Div    { rd, rs1, rs2 },
                    (0b0000001, 0b101) => Instruction::Divu   { rd, rs1, rs2 },
                    (0b0000001, 0b110) => Instruction::Rem    { rd, rs1, rs2 },
                    (0b0000001, 0b111) => Instruction::Remu   { rd, rs1, rs2 },

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

                    // M Extention

                    (0b0000001, 0b000) => Instruction::Mulw  { rd, rs1, rs2 },
                    (0b0000001, 0b100) => Instruction::Divw  { rd, rs1, rs2 },
                    (0b0000001, 0b101) => Instruction::Divuw { rd, rs1, rs2 },
                    (0b0000001, 0b110) => Instruction::Remw  { rd, rs1, rs2 },
                    (0b0000001, 0b111) => Instruction::Remuw { rd, rs1, rs2 },

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
                let csr = (imm & 0b111111111111) as u16;
                let uimm = (original_inst >> 15) & 0b11111;

                return match inst.funct3 {
                    0b000 => {
                        return match imm & 0b111111111111 {
                            0 => Instruction::Ecall,
                            1 => Instruction::Ebreak,

                            _ => Instruction::Undefined(original_inst),
                        }
                    }

                    0b001 => Instruction::Csrrw { rd, rs1, csr },
                    0b010 => Instruction::Csrrs { rd, rs1, csr },
                    0b011 => Instruction::Csrrc { rd, rs1, csr },

                    0b101 => Instruction::Csrrwi { rd, uimm, csr },
                    0b110 => Instruction::Csrrsi { rd, uimm, csr },
                    0b111 => Instruction::Csrrci { rd, uimm, csr },

                    _ => Instruction::Undefined(original_inst),
                }
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

#[derive(Debug)]
pub struct RType {
    pub funct7: u32,
    pub funct3: u32,
    pub rd: Register,
    pub rs1: Register,
    pub rs2: Register,
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
pub struct IType {
    pub imm: i32,
    pub rs1: Register,
    pub funct3: u32,
    pub rd: Register
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
pub struct SType {
    pub imm: i32,
    pub funct3: u32,
    pub rs1: Register,
    pub rs2: Register
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
pub struct BType {
    pub imm: i32,
    pub funct3: u32,
    pub rs1: Register,
    pub rs2: Register
}

impl From<u32> for BType {
    fn from(value: u32) -> Self {
        let imm12 = (value >> 31) & 0b1;
        let imm105 = (value >> 25) & 0b111111;
        let imm41 = (value >> 8) & 0b1111;
        let imm11 = (value >> 7) & 0b1;

        let imm = (imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);
        let imm = ((imm as i32) << 19) >> 19;

        let rs1 = Register::from((value >> 15) & 0b11111);
        let rs2 = Register::from((value >> 20) & 0b11111);

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
pub struct UType {
    pub imm: i32,
    pub rd: Register
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
pub struct JType {
    pub imm: i32,
    pub rd: Register,
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

