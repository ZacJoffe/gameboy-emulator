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

