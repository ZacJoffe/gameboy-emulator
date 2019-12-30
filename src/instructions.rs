pub enum Instruction {
    ADD(ArithTarget),
    ADC(ArithTarget),
    ADDHL(AddHLTarget),
    SUB(ArithTarget),
    SBC(ArithTarget),
    AND(ArithTarget),
    OR(ArithTarget),
    XOR(ArithTarget),
    CP(ArithTarget)
}

pub enum ArithTarget {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum AddHLTarget {
    BC, DE, HL, SP
}
