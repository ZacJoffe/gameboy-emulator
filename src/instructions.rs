use std::convert::From;

pub enum Instruction {
    ADD(ArithTarget),
    ADC(ArithTarget),
    ADDHL(AddHLTarget),
    SUB(ArithTarget),
    SBC(ArithTarget),
    AND(ArithTarget),
    OR(ArithTarget),
    XOR(ArithTarget),
    CP(ArithTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),

    CCF,
    SCF,

    RRA,
    RLA,
    RRCA,
    RLCA,

    CPL,

    BIT(BitPosition, PrefixTarget),
    SET(BitPosition, PrefixTarget),
    RES(BitPosition, PrefixTarget),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget)
}

pub enum ArithTarget {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum AddHLTarget {
    BC, DE, HL, SP
}

pub enum IncDecTarget {
    A, B, C, D, E, H, L, HLI, BC, DE, HL, SP
}

#[derive(Copy, Clone)]
pub enum PrefixTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum BitPosition {
    B0, B1, B2, B3, B4, B5, B6, B7
}

// useful for bit shifting
impl From<BitPosition> for u8 {
    fn from(pos: BitPosition) -> u8 {
        match pos {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7,
        }
    }
}
