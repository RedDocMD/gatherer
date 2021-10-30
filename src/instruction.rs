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

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        todo!("Implement instruction decode")
    }
}

pub struct AbsLabel {
    name: String,
    addr: Option<u32>,
}

impl AbsLabel {
    fn new(name: String) -> Self {
        Self { name, addr: None }
    }
}

pub struct RelLabel {
    name: String,
    addr: Option<u16>,
}

impl RelLabel {
    fn new(name: String) -> Self {
        Self { name, addr: None }
    }
}
