use colored::*;
use num_traits::{AsPrimitive, Num};
use regex::Regex;

use crate::error::{AssemblerError, Result as AssemblerResult};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Add { rs: u8, rt: u8 },
    Comp { rs: u8, rt: u8 },
    AddImm { rs: u8, imm: u16 },
    CompImm { rs: u8, imm: u16 },
    And { rs: u8, rt: u8 },
    Xor { rs: u8, rt: u8 },
    Sll { rs: u8, sh: u8 },
    Srl { rs: u8, sh: u8 },
    Sra { rs: u8, sh: u8 },
    Sllv { rs: u8, rt: u8 },
    Srlv { rs: u8, rt: u8 },
    Srav { rs: u8, rt: u8 },
    Lw { rt: u8, imm: u16, rs: u8 },
    Sw { rt: u8, imm: u16, rs: u8 },
    B { label: AbsLabel },
    Bl { label: AbsLabel },
    Br { rs: u8 },
    Bcy { label: RelLabel },
    Bncy { label: RelLabel },
    Bltz { rs: u8, label: RelLabel },
    Bz { rs: u8, label: RelLabel },
    Bnz { rs: u8, label: RelLabel },
}

impl Instruction {
    fn opcode(&self) -> u8 {
        match self {
            Self::Add { .. } => 0,
            Self::Comp { .. } => 1,
            Self::AddImm { .. } => 2,
            Self::CompImm { .. } => 3,
            Self::And { .. } => 4,
            Self::Xor { .. } => 5,
            Self::Sll { .. } => 6,
            Self::Srl { .. } => 7,
            Self::Sra { .. } => 10,
            Self::Sllv { .. } => 8,
            Self::Srlv { .. } => 9,
            Self::Srav { .. } => 11,
            Self::Lw { .. } => 12,
            Self::Sw { .. } => 13,
            Self::B { .. } => 14,
            Self::Bl { .. } => 19,
            Self::Br { .. } => 15,
            Self::Bcy { .. } => 20,
            Self::Bncy { .. } => 21,
            Self::Bltz { .. } => 16,
            Self::Bz { .. } => 17,
            Self::Bnz { .. } => 18,
        }
    }

    fn opname(&self) -> &'static str {
        match self {
            Self::Add { .. } => "add",
            Self::Comp { .. } => "comp",
            Self::AddImm { .. } => "addi",
            Self::CompImm { .. } => "compi",
            Self::And { .. } => "and",
            Self::Xor { .. } => "xor",
            Self::Sll { .. } => "sll",
            Self::Srl { .. } => "srl",
            Self::Sra { .. } => "sra",
            Self::Sllv { .. } => "sllv",
            Self::Srlv { .. } => "srlv",
            Self::Srav { .. } => "srav",
            Self::Lw { .. } => "lw",
            Self::Sw { .. } => "sw",
            Self::B { .. } => "b",
            Self::Bl { .. } => "bl",
            Self::Br { .. } => "br",
            Self::Bcy { .. } => "bcy",
            Self::Bncy { .. } => "bncy",
            Self::Bltz { .. } => "bltz",
            Self::Bz { .. } => "bz",
            Self::Bnz { .. } => "bnz",
        }
    }

    pub fn encode(&self) -> AssemblerResult<u32> {
        let opcode = self.opcode();
        match self {
            Instruction::Add { rs, rt }
            | Instruction::Comp { rs, rt }
            | Instruction::And { rs, rt }
            | Instruction::Xor { rs, rt }
            | Instruction::Sllv { rs, rt }
            | Instruction::Srlv { rs, rt }
            | Instruction::Srav { rs, rt } => Ok(encode_itype(opcode, *rs, *rt, 0)),

            Instruction::AddImm { rs, imm } | Instruction::CompImm { rs, imm } => {
                Ok(encode_itype(opcode, *rs, 0, *imm))
            }

            Instruction::Sll { rs, sh }
            | Instruction::Srl { rs, sh }
            | Instruction::Sra { rs, sh } => Ok(encode_itype(opcode, *rs, *sh, 0)),

            Instruction::Lw { rt, imm, rs } | Instruction::Sw { rt, imm, rs } => {
                Ok(encode_itype(opcode, *rs, *rt, *imm))
            }

            Instruction::B { label } | Instruction::Bl { label } => match label.addr {
                Some(addr) => Ok(encode_jtype(opcode, addr)),
                None => Err(AssemblerError::FloatingLabel(label.name.clone())),
            },
            Instruction::Bcy { label } | Instruction::Bncy { label } => match label.addr {
                Some(addr) => Ok(encode_itype(opcode, 0, 0, addr)),
                None => Err(AssemblerError::FloatingLabel(label.name.clone())),
            },

            Instruction::Br { rs } => Ok(encode_itype(opcode, *rs, 0, 0)),
            Instruction::Bltz { rs, label }
            | Instruction::Bz { rs, label }
            | Instruction::Bnz { rs, label } => match label.addr {
                Some(addr) => Ok(encode_itype(opcode, *rs, 0, addr)),
                None => Err(AssemblerError::FloatingLabel(label.name.clone())),
            },
        }
    }

    pub fn has_abs_label(&self) -> bool {
        use Instruction::*;
        matches!(self, B { .. } | Bl { .. })
    }

    pub fn has_rel_label(&self) -> bool {
        use Instruction::*;
        matches!(
            self,
            Bcy { .. } | Bncy { .. } | Bltz { .. } | Bz { .. } | Bnz { .. }
        )
    }

    pub fn set_abs_addr(&mut self, addr: u32) {
        use Instruction::*;
        match self {
            B { label, .. } | Bl { label, .. } => label.addr = Some(addr),
            _ => {
                eprintln!(
                    "{} {}",
                    "attempting to set absolute address on".yellow(),
                    self.opname().yellow()
                );
            }
        }
    }

    pub fn set_rel_addr(&mut self, addr: u16) {
        use Instruction::*;
        match self {
            Bcy { label, .. }
            | Bncy { label, .. }
            | Bltz { label, .. }
            | Bz { label, .. }
            | Bnz { label, .. } => label.addr = Some(addr),
            _ => {
                eprintln!(
                    "{} {}",
                    "attempting to set relative address on".yellow(),
                    self.opname().yellow()
                );
            }
        }
    }

    pub fn get_label_name(&self) -> &String {
        use Instruction::*;
        match self {
            Bcy { label, .. }
            | Bncy { label, .. }
            | Bltz { label, .. }
            | Bz { label, .. }
            | Bnz { label, .. } => &label.name,

            B { label } | Bl { label } => &label.name,
            _ => unreachable!("didn't expect to get label of {}", self.opname()),
        }
    }

    pub fn from_str(instr: &str) -> AssemblerResult<Vec<Self>> {
        let mut instrs = Vec::new();
        let (comm, rest) = extract_command(instr)
            .ok_or_else(|| AssemblerError::OpcodeMissing(String::from(instr)))?;
        match comm {
            "push" => {
                let reg = register_from_str(rest)
                    .ok_or_else(|| AssemblerError::UnknownRegister(String::from(rest)))?;
                let sp = register_from_str("$sp").unwrap();
                instrs.push(Instruction::Sw {
                    rt: reg,
                    rs: sp,
                    imm: 0,
                });
                instrs.push(Instruction::AddImm {
                    rs: sp,
                    imm: (-4_i16 as u16),
                });
            }
            "pop" => {
                let reg = register_from_str(rest)
                    .ok_or_else(|| AssemblerError::UnknownRegister(String::from(rest)))?;
                let sp = register_from_str("$sp").unwrap();
                instrs.push(Instruction::AddImm { rs: sp, imm: 4 });
                instrs.push(Instruction::Lw {
                    rt: reg,
                    rs: sp,
                    imm: 0,
                });
            }
            "mov" => {
                let (dest, src) = parse_two_registers(rest)?;
                instrs.push(Instruction::Xor { rs: dest, rt: dest });
                instrs.push(Instruction::Add { rs: dest, rt: src });
            }
            _ => instrs.push(Instruction::try_from(instr)?),
        }
        Ok(instrs)
    }
}

fn encode_itype(opcode: u8, rs: u8, rt: u8, imm: u16) -> u32 {
    let mut instr = 0_u32;
    instr |= (opcode as u32) << 26;
    instr |= (rs as u32) << 21;
    instr |= (rt as u32) << 16;
    instr |= imm as u32;
    instr
}

fn encode_jtype(opcode: u8, addr: u32) -> u32 {
    let mut instr = 0_u32;
    instr |= (opcode as u32) << 26;
    instr |= addr;
    instr
}

impl TryFrom<&str> for Instruction {
    type Error = AssemblerError;

    fn try_from(instr: &str) -> Result<Self, Self::Error> {
        let (comm, rest) = extract_command(instr)
            .ok_or_else(|| AssemblerError::OpcodeMissing(String::from(instr)))?;
        match comm {
            "add" => {
                let (rs, rt) = parse_two_registers(rest)?;
                Ok(Instruction::Add { rs, rt })
            }
            "comp" => {
                let (rs, rt) = parse_two_registers(rest)?;
                Ok(Instruction::Comp { rs, rt })
            }
            "addi" => {
                let (rs, imm) = parse_register_and_value::<u16>(rest)?;
                Ok(Instruction::AddImm { rs, imm })
            }
            "compi" => {
                let (rs, imm) = parse_register_and_value::<u16>(rest)?;
                Ok(Instruction::CompImm { rs, imm })
            }
            "and" => {
                let (rs, rt) = parse_two_registers(rest)?;
                Ok(Instruction::And { rs, rt })
            }
            "xor" => {
                let (rs, rt) = parse_two_registers(rest)?;
                Ok(Instruction::Xor { rs, rt })
            }
            "sll" => {
                let (rs, sh) = parse_register_and_value::<u8>(rest)?;
                Ok(Instruction::Sll { rs, sh })
            }
            "srl" => {
                let (rs, sh) = parse_register_and_value::<u8>(rest)?;
                Ok(Instruction::Srl { rs, sh })
            }
            "sra" => {
                let (rs, sh) = parse_register_and_value::<u8>(rest)?;
                Ok(Instruction::Sra { rs, sh })
            }
            "sllv" => {
                let (rs, rt) = parse_two_registers(rest)?;
                Ok(Instruction::Sllv { rs, rt })
            }
            "srlv" => {
                let (rs, rt) = parse_two_registers(rest)?;
                Ok(Instruction::Srlv { rs, rt })
            }
            "srav" => {
                let (rs, rt) = parse_two_registers(rest)?;
                Ok(Instruction::Srav { rs, rt })
            }
            "lw" => {
                let (rt, imm, rs) = parse_mem_access(rest)?;
                Ok(Instruction::Lw { rt, imm, rs })
            }
            "sw" => {
                let (rt, imm, rs) = parse_mem_access(rest)?;
                Ok(Instruction::Sw { rt, imm, rs })
            }
            "b" => Ok(Instruction::B {
                label: AbsLabel::from(rest.trim()),
            }),
            "bl" => Ok(Instruction::Bl {
                label: AbsLabel::from(rest.trim()),
            }),
            "br" => Ok(Instruction::Br {
                rs: register_from_str(rest)
                    .ok_or_else(|| AssemblerError::UnknownRegister(String::from(rest)))?,
            }),
            "bcy" => Ok(Instruction::Bcy {
                label: RelLabel::from(rest.trim()),
            }),
            "bncy" => Ok(Instruction::Bncy {
                label: RelLabel::from(rest.trim()),
            }),
            "bltz" => {
                let (rs, label) = parse_reg_label(rest)?;
                Ok(Instruction::Bltz { rs, label })
            }
            "bz" => {
                let (rs, label) = parse_reg_label(rest)?;
                Ok(Instruction::Bz { rs, label })
            }
            "bnz" => {
                let (rs, label) = parse_reg_label(rest)?;
                Ok(Instruction::Bnz { rs, label })
            }
            _ => Err(AssemblerError::UnknownInstruction(String::from(comm))),
        }
    }
}

fn parse_two_registers(rest: &str) -> AssemblerResult<(u8, u8)> {
    let regs_str: Vec<_> = rest.split(',').map(|x| x.trim()).collect();
    if regs_str.len() != 2 {
        return Err(AssemblerError::InvalidNoOfArgs(2, regs_str.len()));
    }
    let mut regs = Vec::with_capacity(2);
    for reg_str in regs_str {
        regs.push(
            register_from_str(reg_str)
                .ok_or_else(|| AssemblerError::UnknownRegister(String::from(reg_str)))?,
        );
    }
    Ok((regs[0], regs[1]))
}

fn parse_register_and_value<T>(rest: &str) -> AssemblerResult<(u8, T)>
where
    T: Num + AsPrimitive<i32>,
    i32: AsPrimitive<T>,
{
    let things_str: Vec<_> = rest.split(',').map(|x| x.trim()).collect();
    if things_str.len() != 2 {
        return Err(AssemblerError::InvalidNoOfArgs(2, things_str.len()));
    }
    let reg = register_from_str(things_str[0])
        .ok_or_else(|| AssemblerError::UnknownRegister(String::from(things_str[0])))?;
    let (sign, num_str) = parse_sign(things_str[1]);
    let (radix, num_str) = parse_radix(num_str);
    let val = T::from_str_radix(num_str, radix)
        .map_err(|_| AssemblerError::InvalidNumber(String::from(num_str)))?;
    Ok((reg, sign.to_sign(val)))
}

fn parse_radix(num: &str) -> (u32, &str) {
    if num.len() < 2 {
        (10, num)
    } else {
        match &num[..2] {
            "0x" => (16, &num[2..]),
            "0b" => (2, &num[2..]),
            "0o" => (8, &num[2..]),
            _ => (10, num),
        }
    }
}

enum Sign {
    Positive,
    Negative,
}

impl Sign {
    fn to_sign<T>(&self, val: T) -> T
    where
        T: AsPrimitive<i32>,
        i32: AsPrimitive<T>,
    {
        let signed_val: i32 = val.as_();
        let after_sign = match self {
            Sign::Positive => signed_val,
            Sign::Negative => -signed_val,
        };
        after_sign.as_()
    }
}

fn parse_sign(num: &str) -> (Sign, &str) {
    let num_bytes = num.as_bytes();
    if num_bytes[0] == b'+' {
        (Sign::Positive, &num[1..])
    } else if num_bytes[0] == b'-' {
        (Sign::Negative, &num[1..])
    } else {
        (Sign::Positive, num)
    }
}

fn parse_mem_access(rest: &str) -> AssemblerResult<(u8, u16, u8)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\$[a-z0-9]{2}) *, *([^(]+)\((\$[a-z0-9]{2})\)").unwrap();
    }
    let caps = RE
        .captures(rest)
        .ok_or_else(|| AssemblerError::InvalidInstruction(String::from(rest)))?;
    let rt = register_from_str(&caps[1])
        .ok_or_else(|| AssemblerError::UnknownRegister(String::from(&caps[1])))?;
    let (sign, num_str) = parse_sign(&caps[2]);
    let (radix, num_str) = parse_radix(num_str);
    let imm = u16::from_str_radix(num_str, radix)
        .map_err(|_| AssemblerError::InvalidNumber(String::from(num_str)))?;
    let rs = register_from_str(&caps[3])
        .ok_or_else(|| AssemblerError::UnknownRegister(String::from(&caps[3])))?;
    Ok((rt, sign.to_sign(imm), rs))
}

fn parse_reg_label(rest: &str) -> AssemblerResult<(u8, RelLabel)> {
    let things_str: Vec<_> = rest.split(',').map(|x| x.trim()).collect();
    if things_str.len() != 2 {
        return Err(AssemblerError::InvalidNoOfArgs(2, things_str.len()));
    }
    let reg = register_from_str(things_str[0])
        .ok_or_else(|| AssemblerError::UnknownRegister(String::from(things_str[0])))?;
    let label = RelLabel::from(things_str[1]);
    Ok((reg, label))
}

fn register_from_str(reg: &str) -> Option<u8> {
    match reg {
        "$zero" => Some(0),
        "$at" => Some(1),
        "$v0" => Some(2),
        "$v1" => Some(3),
        "$a0" => Some(4),
        "$a1" => Some(5),
        "$a2" => Some(6),
        "$a3" => Some(7),
        "$t0" => Some(8),
        "$t1" => Some(9),
        "$t2" => Some(10),
        "$t3" => Some(11),
        "$t4" => Some(12),
        "$t5" => Some(13),
        "$t6" => Some(14),
        "$t7" => Some(15),
        "$s0" => Some(16),
        "$s1" => Some(17),
        "$s2" => Some(18),
        "$s3" => Some(19),
        "$s4" => Some(20),
        "$s5" => Some(21),
        "$s6" => Some(22),
        "$s7" => Some(23),
        "$t8" => Some(24),
        "$t9" => Some(25),
        "$k0" => Some(26),
        "$k1" => Some(27),
        "$gp" => Some(28),
        "$sp" => Some(29),
        "$fp" => Some(30),
        "$ra" => Some(31),
        _ => None,
    }
}

fn extract_command(instr: &str) -> Option<(&str, &str)> {
    let blank_idx = match instr.find(' ') {
        Some(idx) => idx,
        None => return None,
    };
    Some((&instr[0..blank_idx], &instr[blank_idx + 1..]))
}

#[derive(Debug, PartialEq, Eq)]
pub struct AbsLabel {
    name: String,
    addr: Option<u32>,
}

impl From<&str> for AbsLabel {
    fn from(s: &str) -> Self {
        Self {
            name: String::from(s),
            addr: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RelLabel {
    name: String,
    addr: Option<u16>,
}

impl From<&str> for RelLabel {
    fn from(s: &str) -> Self {
        Self {
            name: String::from(s),
            addr: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reg_instr() {
        let instr = "add $t2   ,     $a0";
        let parsed_instr = Instruction::try_from(instr);
        assert!(parsed_instr.is_ok());
        assert_eq!(parsed_instr.unwrap(), Instruction::Add { rs: 10, rt: 4 });
    }

    #[test]
    fn test_imm_instr() {
        let instr = "compi $t2   ,  0x20";
        let parsed_instr = Instruction::try_from(instr);
        assert!(parsed_instr.is_ok());
        assert_eq!(
            parsed_instr.unwrap(),
            Instruction::CompImm { rs: 10, imm: 32 }
        );
    }

    #[test]
    fn test_neg_imm() {
        let instr = "addi $t2, -0x10";
        let parsed_instr = Instruction::try_from(instr);
        assert!(parsed_instr.is_ok());
        assert_eq!(
            parsed_instr.unwrap(),
            Instruction::AddImm {
                rs: 10,
                imm: 0xFFF0
            }
        );
    }

    #[test]
    fn test_shamt_instr() {
        let instr = "sll $t2  ,   3";
        let parsed_instr = Instruction::try_from(instr);
        assert!(parsed_instr.is_ok());
        assert_eq!(parsed_instr.unwrap(), Instruction::Sll { rs: 10, sh: 3 });
    }

    #[test]
    fn test_mem_instr() {
        let load = "lw $t1,  16($t2)";
        let load_instr = Instruction::try_from(load);
        assert!(load_instr.is_ok());
        assert_eq!(
            load_instr.unwrap(),
            Instruction::Lw {
                rt: 9,
                imm: 16,
                rs: 10
            }
        );
        let store = "sw $t1  ,  16($t2)";
        let store_instr = Instruction::try_from(store);
        assert!(store_instr.is_ok());
        assert_eq!(
            store_instr.unwrap(),
            Instruction::Sw {
                rt: 9,
                imm: 16,
                rs: 10
            }
        );
    }

    #[test]
    fn test_branch() {
        let abs_jmp = "b Hell";
        let abs_jmp_instr = Instruction::try_from(abs_jmp);
        assert!(abs_jmp_instr.is_ok());
        assert_eq!(
            abs_jmp_instr.unwrap(),
            Instruction::B {
                label: AbsLabel {
                    name: String::from("Hell"),
                    addr: None
                }
            }
        );
        let cond_jmp = "bltz $s0, Else1";
        let cond_jmp_instr = Instruction::try_from(cond_jmp);
        assert!(cond_jmp_instr.is_ok());
        assert_eq!(
            cond_jmp_instr.unwrap(),
            Instruction::Bltz {
                rs: 16,
                label: RelLabel {
                    name: String::from("Else1"),
                    addr: None
                }
            }
        );
    }

    #[test]
    fn test_encoding() {
        let and_instr = Instruction::And { rs: 10, rt: 23 };
        let and_instr_word = and_instr.encode();
        assert!(and_instr_word.is_ok());
        assert_eq!(and_instr_word.unwrap(), 0x11570000);
        let add_instr = Instruction::AddImm { rs: 10, imm: 657 };
        let add_instr_word = add_instr.encode();
        assert!(add_instr_word.is_ok());
        assert_eq!(add_instr_word.unwrap(), 0x09400291);
        let lw_instr = Instruction::Lw {
            rs: 10,
            rt: 15,
            imm: 657,
        };
        let lw_instr_word = lw_instr.encode();
        assert!(lw_instr_word.is_ok());
        assert_eq!(lw_instr_word.unwrap(), 0x314F0291);
        let b_instr = Instruction::B {
            label: AbsLabel {
                name: String::from("L0"),
                addr: Some(0xA7FFF),
            },
        };
        let b_instr_word = b_instr.encode();
        assert!(b_instr_word.is_ok());
        assert_eq!(b_instr_word.unwrap(), 0x380A7FFF);
    }
}
