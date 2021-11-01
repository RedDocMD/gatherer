use num_traits::Num;

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
    Br { rs: u8 },
    Bltz { rs: u8, label: RelLabel },
    Bz { rs: u8, label: RelLabel },
    Bnz { rs: u8, label: RelLabel },
    Bl { label: AbsLabel },
    Bcy { label: RelLabel },
    Bncy { label: RelLabel },
}

impl TryFrom<&str> for Instruction {
    type Error = AssemblerError;

    fn try_from(instr: &str) -> Result<Self, Self::Error> {
        let (comm, rest) = extract_command(instr).ok_or(AssemblerError::OpcodeMissing)?;
        if comm == "add" {
            let (rs, rt) = parse_two_registers(rest)?;
            Ok(Instruction::Add { rs, rt })
        } else if comm == "comp" {
            let (rs, rt) = parse_two_registers(rest)?;
            Ok(Instruction::Comp { rs, rt })
        } else if comm == "addi" {
            let (rs, imm) = parse_register_and_value::<u16>(rest)?;
            Ok(Instruction::AddImm { rs, imm })
        } else if comm == "compi" {
            let (rs, imm) = parse_register_and_value::<u16>(rest)?;
            Ok(Instruction::CompImm { rs, imm })
        } else if comm == "and" {
            let (rs, rt) = parse_two_registers(rest)?;
            Ok(Instruction::And { rs, rt })
        } else if comm == "xor" {
            let (rs, rt) = parse_two_registers(rest)?;
            Ok(Instruction::Xor { rs, rt })
        } else if comm == "sll" {
            let (rs, sh) = parse_register_and_value::<u8>(rest)?;
            Ok(Instruction::Sll { rs, sh })
        } else if comm == "srl" {
            let (rs, sh) = parse_register_and_value::<u8>(rest)?;
            Ok(Instruction::Srl { rs, sh })
        } else if comm == "sra" {
            let (rs, sh) = parse_register_and_value::<u8>(rest)?;
            Ok(Instruction::Sra { rs, sh })
        } else if comm == "sllv" {
            let (rs, rt) = parse_two_registers(rest)?;
            Ok(Instruction::Sllv { rs, rt })
        } else if comm == "srlv" {
            let (rs, rt) = parse_two_registers(rest)?;
            Ok(Instruction::Srlv { rs, rt })
        } else if comm == "srav" {
            let (rs, rt) = parse_two_registers(rest)?;
            Ok(Instruction::Srav { rs, rt })
        } else {
            Err(AssemblerError::UnknownInstruction(String::from(comm)))
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
                .ok_or(AssemblerError::UnknownRegister(String::from(reg_str)))?,
        );
    }
    Ok((regs[0], regs[1]))
}

fn parse_register_and_value<T: Num>(rest: &str) -> AssemblerResult<(u8, T)> {
    let things_str: Vec<_> = rest.split(',').map(|x| x.trim()).collect();
    if things_str.len() != 2 {
        return Err(AssemblerError::InvalidNoOfArgs(2, things_str.len()));
    }
    let reg = register_from_str(things_str[0])
        .ok_or(AssemblerError::UnknownRegister(String::from(things_str[0])))?;
    let (radix, num_str) = parse_radix(things_str[1]);
    let val = T::from_str_radix(num_str, radix)
        .map_err(|_| AssemblerError::InvalidNumber(String::from(num_str)))?;
    Ok((reg, val))
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

impl AbsLabel {
    fn new(name: String) -> Self {
        Self { name, addr: None }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RelLabel {
    name: String,
    addr: Option<u16>,
}

impl RelLabel {
    fn new(name: String) -> Self {
        Self { name, addr: None }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reg_instr() {
        let instr = "add $t2, $a0";
        let parsed_instr = Instruction::try_from(instr);
        assert!(parsed_instr.is_ok());
        assert_eq!(parsed_instr.unwrap(), Instruction::Add { rs: 10, rt: 4 });
    }

    #[test]
    fn test_imm_instr() {
        let instr = "compi $t2, 0x20";
        let parsed_instr = Instruction::try_from(instr);
        assert!(parsed_instr.is_ok());
        assert_eq!(
            parsed_instr.unwrap(),
            Instruction::CompImm { rs: 10, imm: 32 }
        );
    }

    #[test]
    fn test_shamt_instr() {
        let instr = "sll $t2, 3";
        let parsed_instr = Instruction::try_from(instr);
        assert!(parsed_instr.is_ok());
        assert_eq!(parsed_instr.unwrap(), Instruction::Sll { rs: 10, sh: 3 });
    }
}
