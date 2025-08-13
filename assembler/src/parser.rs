pub enum AInstruction {
    Number(u16),
    Symbol(String)
}

bitflags::bitflags! {
    pub struct DestMask: u8 {
        const A=0b100;
        const D=0b010;
        const M=0b001;
    }
}

pub enum Jump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP
}

pub enum Comp {
    pub mnemonic: &str;
    pub command: u8;
}

pub struct CInstruction {
    pub dest: DestMask,
    pub comp: Comp,
    pub jump: Jump
}

pub enum Line {
    A(AInstruction),
    C(CInstruction),
    Label(String),
    Empty
}

