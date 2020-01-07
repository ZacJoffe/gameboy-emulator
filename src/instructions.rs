use std::convert::From;

pub enum Instruction {
    ADD(ArithTarget),
    ADC(ArithTarget),
    ADDHL(AddHLTarget),
    ADDSP,
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
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),

    JP(JumpTest),
    JR(JumpTest),
    JPHLI,

    LD(LoadType),

    PUSH(StackTarget),
    POP(StackTarget),

    CALL(JumpTest),
    RET(JumpTest),
    RST(RstTarget),
    RETI,

    NOP,
    HALT,
    DAA,
    STOP,
    DI,
    EI,
}

#[derive(Copy, Clone)]
pub enum ArithTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum AddHLTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone)]
pub enum PrefixTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum BitPosition {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
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

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Unconditional,
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Copy, Clone)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum LoadIndirectTarget {
    // note that CI is more like (0xff00 + C)
    BCI,
    DEI,
    HLIPLUS,
    HLIMINUS,
    WORDI,
    CI,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    AFromIndirect(LoadIndirectTarget),
    IndirectFromA(LoadIndirectTarget),
    AFromA8,
    A8FromA,
    HLFromSP,
    SPFromHL,
    IndirectFromSP
}

pub enum StackTarget {
    AF,
    BC,
    DE,
    HL,
}

pub enum RstTarget {
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
}

impl Instruction {
    pub fn disassemble(byte: u8, is_prefixed: bool) -> Option<Instruction> {
        if is_prefixed {
            Some(Instruction::disassemble_prefixed(byte))
        } else {
            Instruction::disassemble_not_prefixed(byte)
        }
    }

    // todo when instruction enum is completely filled
    fn disassemble_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC))),
            0x02 => Some(Instruction::LD(LoadType::IndirectFromA(
                LoadIndirectTarget::BCI,
            ))),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
            0x07 => Some(Instruction::RLCA),
            0x08 => Some(Instruction::LD(LoadType::IndirectFromSP)),
            0x09 => Some(Instruction::ADDHL(AddHLTarget::HL)),
            0x0a => Some(Instruction::LD(LoadType::AFromIndirect(
                LoadIndirectTarget::BCI,
            ))),
            0x0b => Some(Instruction::DEC(IncDecTarget::BC)),
            0x0c => Some(Instruction::INC(IncDecTarget::C)),
            0x0d => Some(Instruction::DEC(IncDecTarget::C)),
            0x0e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D8,
            ))),
            0x0f => Some(Instruction::RRCA),

            0x10 => Some(Instruction::STOP),
            0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE))),
            0x12 => Some(Instruction::LD(LoadType::IndirectFromA(
                LoadIndirectTarget::DEI,
            ))),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D8,
            ))),
            0x17 => Some(Instruction::RLA),
            0x18 => Some(Instruction::JR(JumpTest::Unconditional)),
            0x19 => Some(Instruction::ADDHL(AddHLTarget::DE)),
            0x1a => Some(Instruction::LD(LoadType::AFromIndirect(
                LoadIndirectTarget::DEI,
            ))),
            0x1b => Some(Instruction::DEC(IncDecTarget::DE)),
            0x1c => Some(Instruction::INC(IncDecTarget::E)),
            0x1d => Some(Instruction::DEC(IncDecTarget::E)),
            0x1e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D8,
            ))),
            0x1f => Some(Instruction::RRA),

            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL))),
            0x22 => Some(Instruction::LD(LoadType::IndirectFromA(
                LoadIndirectTarget::HLIPLUS,
            ))),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x24 => Some(Instruction::INC(IncDecTarget::D)),
            0x25 => Some(Instruction::DEC(IncDecTarget::D)),
            0x26 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D8,
            ))),
            0x27 => Some(Instruction::DAA),
            0x28 => Some(Instruction::JR(JumpTest::Zero)),
            0x29 => Some(Instruction::ADDHL(AddHLTarget::HL)),
            0x2a => Some(Instruction::LD(LoadType::AFromIndirect(
                LoadIndirectTarget::HLIPLUS,
            ))),
            0x2b => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2c => Some(Instruction::INC(IncDecTarget::L)),
            0x2d => Some(Instruction::DEC(IncDecTarget::L)),
            0x2e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D8,
            ))),
            0x2f => Some(Instruction::CPL),

            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(
                LoadIndirectTarget::HLIMINUS,
            ))),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),
            0x34 => Some(Instruction::INC(IncDecTarget::HLI)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HLI)),
            0x36 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::D8,
            ))),
            0x37 => Some(Instruction::SCF),
            0x38 => Some(Instruction::JR(JumpTest::Carry)),
            0x39 => Some(Instruction::ADDHL(AddHLTarget::SP)),
            0x3a => Some(Instruction::LD(LoadType::AFromIndirect(
                LoadIndirectTarget::HLIMINUS,
            ))),
            0x3b => Some(Instruction::DEC(IncDecTarget::SP)),
            0x3c => Some(Instruction::INC(IncDecTarget::A)),
            0x3d => Some(Instruction::DEC(IncDecTarget::A)),
            0x3e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D8,
            ))),
            0x3f => Some(Instruction::CCF),

            0x40 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x41 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x42 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D,
            ))),
            0x43 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::E,
            ))),
            0x44 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::H,
            ))),
            0x45 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::L,
            ))),
            0x46 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::HLI,
            ))),
            0x47 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::A,
            ))),
            0x48 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::B,
            ))),
            0x49 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::C,
            ))),
            0x4a => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D,
            ))),
            0x4b => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::E,
            ))),
            0x4c => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::H,
            ))),
            0x4d => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::L,
            ))),
            0x4e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::HLI,
            ))),
            0x4f => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::A,
            ))),

            0x50 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::C,
            ))),
            0x51 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::C,
            ))),
            0x52 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D,
            ))),
            0x53 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::E,
            ))),
            0x54 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::H,
            ))),
            0x55 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::L,
            ))),
            0x56 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::HLI,
            ))),
            0x57 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::A,
            ))),
            0x58 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::B,
            ))),
            0x59 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::C,
            ))),
            0x5a => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D,
            ))),
            0x5b => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::E,
            ))),
            0x5c => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::H,
            ))),
            0x5d => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::L,
            ))),
            0x5e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::HLI,
            ))),
            0x5f => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::A,
            ))),

            0x60 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::C,
            ))),
            0x61 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::C,
            ))),
            0x62 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D,
            ))),
            0x63 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::E,
            ))),
            0x64 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::H,
            ))),
            0x65 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::L,
            ))),
            0x66 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::HLI,
            ))),
            0x67 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::A,
            ))),
            0x68 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::B,
            ))),
            0x69 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::C,
            ))),
            0x6a => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D,
            ))),
            0x6b => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::E,
            ))),
            0x6c => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::H,
            ))),
            0x6d => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::L,
            ))),
            0x6e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::HLI,
            ))),
            0x6f => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::A,
            ))),

            0x70 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::C,
            ))),
            0x71 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::C,
            ))),
            0x72 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::D,
            ))),
            0x73 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::E,
            ))),
            0x74 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::H,
            ))),
            0x75 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::L,
            ))),
            0x76 => Some(Instruction::HALT),
            0x77 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::A,
            ))),
            0x78 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::B,
            ))),
            0x79 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::C,
            ))),
            0x7a => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D,
            ))),
            0x7b => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::E,
            ))),
            0x7c => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::H,
            ))),
            0x7d => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::L,
            ))),
            0x7e => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLI,
            ))),
            0x7f => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::A,
            ))),

            0x80 => Some(Instruction::ADD(ArithTarget::B)),
            0x81 => Some(Instruction::ADD(ArithTarget::C)),
            0x82 => Some(Instruction::ADD(ArithTarget::D)),
            0x83 => Some(Instruction::ADD(ArithTarget::E)),
            0x84 => Some(Instruction::ADD(ArithTarget::H)),
            0x85 => Some(Instruction::ADD(ArithTarget::L)),
            0x86 => Some(Instruction::ADD(ArithTarget::HLI)),
            0x87 => Some(Instruction::ADD(ArithTarget::A)),
            0x88 => Some(Instruction::ADD(ArithTarget::B)),
            0x89 => Some(Instruction::ADD(ArithTarget::C)),
            0x8a => Some(Instruction::ADD(ArithTarget::D)),
            0x8b => Some(Instruction::ADD(ArithTarget::E)),
            0x8c => Some(Instruction::ADD(ArithTarget::H)),
            0x8d => Some(Instruction::ADD(ArithTarget::L)),
            0x8e => Some(Instruction::ADD(ArithTarget::HLI)),
            0x8f => Some(Instruction::ADD(ArithTarget::A)),

            0x90 => Some(Instruction::SUB(ArithTarget::B)),
            0x91 => Some(Instruction::SUB(ArithTarget::C)),
            0x92 => Some(Instruction::SUB(ArithTarget::D)),
            0x93 => Some(Instruction::SUB(ArithTarget::E)),
            0x94 => Some(Instruction::SUB(ArithTarget::H)),
            0x95 => Some(Instruction::SUB(ArithTarget::L)),
            0x96 => Some(Instruction::SUB(ArithTarget::HLI)),
            0x97 => Some(Instruction::SBC(ArithTarget::A)),
            0x98 => Some(Instruction::SBC(ArithTarget::B)),
            0x99 => Some(Instruction::SBC(ArithTarget::C)),
            0x9a => Some(Instruction::SBC(ArithTarget::D)),
            0x9b => Some(Instruction::SBC(ArithTarget::E)),
            0x9c => Some(Instruction::SBC(ArithTarget::H)),
            0x9d => Some(Instruction::SBC(ArithTarget::L)),
            0x9e => Some(Instruction::SBC(ArithTarget::HLI)),
            0x9f => Some(Instruction::SBC(ArithTarget::A)),

            0xa0 => Some(Instruction::AND(ArithTarget::B)),
            0xa1 => Some(Instruction::AND(ArithTarget::C)),
            0xa2 => Some(Instruction::AND(ArithTarget::D)),
            0xa3 => Some(Instruction::AND(ArithTarget::E)),
            0xa4 => Some(Instruction::AND(ArithTarget::H)),
            0xa5 => Some(Instruction::AND(ArithTarget::L)),
            0xa6 => Some(Instruction::AND(ArithTarget::HLI)),
            0xa7 => Some(Instruction::AND(ArithTarget::A)),
            0xa8 => Some(Instruction::XOR(ArithTarget::B)),
            0xa9 => Some(Instruction::XOR(ArithTarget::C)),
            0xaa => Some(Instruction::XOR(ArithTarget::D)),
            0xab => Some(Instruction::XOR(ArithTarget::E)),
            0xac => Some(Instruction::XOR(ArithTarget::H)),
            0xad => Some(Instruction::XOR(ArithTarget::L)),
            0xae => Some(Instruction::XOR(ArithTarget::HLI)),
            0xaf => Some(Instruction::XOR(ArithTarget::A)),

            0xb0 => Some(Instruction::OR(ArithTarget::B)),
            0xb1 => Some(Instruction::OR(ArithTarget::C)),
            0xb2 => Some(Instruction::OR(ArithTarget::D)),
            0xb3 => Some(Instruction::OR(ArithTarget::E)),
            0xb4 => Some(Instruction::OR(ArithTarget::H)),
            0xb5 => Some(Instruction::OR(ArithTarget::L)),
            0xb6 => Some(Instruction::OR(ArithTarget::HLI)),
            0xb7 => Some(Instruction::OR(ArithTarget::A)),
            0xb8 => Some(Instruction::CP(ArithTarget::B)),
            0xb9 => Some(Instruction::CP(ArithTarget::C)),
            0xba => Some(Instruction::CP(ArithTarget::D)),
            0xbb => Some(Instruction::CP(ArithTarget::E)),
            0xbc => Some(Instruction::CP(ArithTarget::H)),
            0xbd => Some(Instruction::CP(ArithTarget::L)),
            0xbe => Some(Instruction::CP(ArithTarget::HLI)),
            0xbf => Some(Instruction::CP(ArithTarget::A)),

            0xc0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xc1 => Some(Instruction::POP(StackTarget::BC)),
            0xc2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xc3 => Some(Instruction::JP(JumpTest::Unconditional)),
            0xc4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xc5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xc6 => Some(Instruction::ADD(ArithTarget::D8)),
            0xc7 => Some(Instruction::RST(RstTarget::X00)),
            0xc8 => Some(Instruction::RET(JumpTest::Zero)),
            0xc9 => Some(Instruction::RET(JumpTest::Unconditional)),
            0xca => Some(Instruction::JP(JumpTest::Zero)),
            // 0xcb =>
            0xcc => Some(Instruction::CALL(JumpTest::Zero)),
            0xcd => Some(Instruction::CALL(JumpTest::Unconditional)),
            0xce => Some(Instruction::ADC(ArithTarget::D8)),
            0xcf => Some(Instruction::RST(RstTarget::X08)),

            0xd0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xd1 => Some(Instruction::POP(StackTarget::DE)),
            0xd2 => Some(Instruction::JP(JumpTest::NotCarry)),

            0xd4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xd5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xd6 => Some(Instruction::SUB(ArithTarget::D8)),
            0xd7 => Some(Instruction::RST(RstTarget::X10)),
            0xd8 => Some(Instruction::RET(JumpTest::Carry)),
            0xd9 => Some(Instruction::RETI),
            0xda => Some(Instruction::JP(JumpTest::Carry)),

            0xdc => Some(Instruction::CALL(JumpTest::Carry)),

            0xde => Some(Instruction::SBC(ArithTarget::D8)),
            0xdf => Some(Instruction::RST(RstTarget::X18)),

            0xe0 => Some(Instruction::LD(LoadType::A8FromA)),
            0xe1 => Some(Instruction::POP(StackTarget::HL)),
            0xe2 => Some(Instruction::LD(LoadType::IndirectFromA(LoadIndirectTarget::CI))),

            0xe5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xe6 => Some(Instruction::AND(ArithTarget::D8)),
            0xe7 => Some(Instruction::RST(RstTarget::X20)),
            0xe8 => Some(Instruction::ADDSP),
            0xe9 => Some(Instruction::JPHLI),
            0xea => Some(Instruction::LD(LoadType::IndirectFromA(LoadIndirectTarget::WORDI))),

            0xee => Some(Instruction::XOR(ArithTarget::D8)),
            0xef => Some(Instruction::RST(RstTarget::X28)),

            0xf0 => Some(Instruction::LD(LoadType::AFromA8)),
            0xf1 => Some(Instruction::POP(StackTarget::AF)),
            0xf2 => Some(Instruction::LD(LoadType::AFromIndirect(LoadIndirectTarget::CI))),
            0xf3 => Some(Instruction::DI),
            0xf5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xf6 => Some(Instruction::OR(ArithTarget::D8)),
            0xf7 => Some(Instruction::RST(RstTarget::X30)),
            0xf8 => Some(Instruction::LD(LoadType::HLFromSP)),
            0xf9 => Some(Instruction::LD(LoadType::SPFromHL)),
            0xfa => Some(Instruction::LD(LoadType::AFromIndirect(LoadIndirectTarget::WORDI))),
            0xfb => Some(Instruction::EI),
            0xfe => Some(Instruction::CP(ArithTarget::D8)),
            0xff => Some(Instruction::RST(RstTarget::X38)),


            _ => None,
        }
    }

    // todo: check over
    fn disassemble_prefixed(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::RLC(PrefixTarget::B),
            0x01 => Instruction::RLC(PrefixTarget::C),
            0x02 => Instruction::RLC(PrefixTarget::D),
            0x03 => Instruction::RLC(PrefixTarget::E),
            0x04 => Instruction::RLC(PrefixTarget::H),
            0x05 => Instruction::RLC(PrefixTarget::L),
            0x06 => Instruction::RLC(PrefixTarget::HLI),
            0x07 => Instruction::RLC(PrefixTarget::A),

            0x08 => Instruction::RRC(PrefixTarget::B),
            0x09 => Instruction::RRC(PrefixTarget::C),
            0x0a => Instruction::RRC(PrefixTarget::D),
            0x0b => Instruction::RRC(PrefixTarget::E),
            0x0c => Instruction::RRC(PrefixTarget::H),
            0x0d => Instruction::RRC(PrefixTarget::L),
            0x0e => Instruction::RRC(PrefixTarget::HLI),
            0x0f => Instruction::RRC(PrefixTarget::A),

            0x10 => Instruction::RL(PrefixTarget::B),
            0x11 => Instruction::RL(PrefixTarget::C),
            0x12 => Instruction::RL(PrefixTarget::D),
            0x13 => Instruction::RL(PrefixTarget::E),
            0x14 => Instruction::RL(PrefixTarget::H),
            0x15 => Instruction::RL(PrefixTarget::L),
            0x16 => Instruction::RL(PrefixTarget::HLI),
            0x17 => Instruction::RL(PrefixTarget::A),

            0x18 => Instruction::RR(PrefixTarget::B),
            0x19 => Instruction::RR(PrefixTarget::C),
            0x1a => Instruction::RR(PrefixTarget::D),
            0x1b => Instruction::RR(PrefixTarget::E),
            0x1c => Instruction::RR(PrefixTarget::H),
            0x1d => Instruction::RR(PrefixTarget::L),
            0x1e => Instruction::RR(PrefixTarget::HLI),
            0x1f => Instruction::RR(PrefixTarget::A),

            0x20 => Instruction::SLA(PrefixTarget::B),
            0x21 => Instruction::SLA(PrefixTarget::C),
            0x22 => Instruction::SLA(PrefixTarget::D),
            0x23 => Instruction::SLA(PrefixTarget::E),
            0x24 => Instruction::SLA(PrefixTarget::H),
            0x25 => Instruction::SLA(PrefixTarget::L),
            0x26 => Instruction::SLA(PrefixTarget::HLI),
            0x27 => Instruction::SLA(PrefixTarget::A),

            0x28 => Instruction::SRA(PrefixTarget::B),
            0x29 => Instruction::SRA(PrefixTarget::C),
            0x2a => Instruction::SRA(PrefixTarget::D),
            0x2b => Instruction::SRA(PrefixTarget::E),
            0x2c => Instruction::SRA(PrefixTarget::H),
            0x2d => Instruction::SRA(PrefixTarget::L),
            0x2e => Instruction::SRA(PrefixTarget::HLI),
            0x2f => Instruction::SRA(PrefixTarget::A),

            0x30 => Instruction::SWAP(PrefixTarget::B),
            0x31 => Instruction::SWAP(PrefixTarget::C),
            0x32 => Instruction::SWAP(PrefixTarget::D),
            0x33 => Instruction::SWAP(PrefixTarget::E),
            0x34 => Instruction::SWAP(PrefixTarget::H),
            0x35 => Instruction::SWAP(PrefixTarget::L),
            0x36 => Instruction::SWAP(PrefixTarget::HLI),
            0x37 => Instruction::SWAP(PrefixTarget::A),

            0x38 => Instruction::SRL(PrefixTarget::B),
            0x39 => Instruction::SRL(PrefixTarget::C),
            0x3a => Instruction::SRL(PrefixTarget::D),
            0x3b => Instruction::SRL(PrefixTarget::E),
            0x3c => Instruction::SRL(PrefixTarget::H),
            0x3d => Instruction::SRL(PrefixTarget::L),
            0x3e => Instruction::SRL(PrefixTarget::HLI),
            0x3f => Instruction::SRL(PrefixTarget::A),

            0x40 => Instruction::BIT(BitPosition::B0, PrefixTarget::B),
            0x41 => Instruction::BIT(BitPosition::B0, PrefixTarget::C),
            0x42 => Instruction::BIT(BitPosition::B0, PrefixTarget::D),
            0x43 => Instruction::BIT(BitPosition::B0, PrefixTarget::E),
            0x44 => Instruction::BIT(BitPosition::B0, PrefixTarget::H),
            0x45 => Instruction::BIT(BitPosition::B0, PrefixTarget::L),
            0x46 => Instruction::BIT(BitPosition::B0, PrefixTarget::HLI),
            0x47 => Instruction::BIT(BitPosition::B0, PrefixTarget::A),

            0x48 => Instruction::BIT(BitPosition::B1, PrefixTarget::B),
            0x49 => Instruction::BIT(BitPosition::B1, PrefixTarget::C),
            0x4a => Instruction::BIT(BitPosition::B1, PrefixTarget::D),
            0x4b => Instruction::BIT(BitPosition::B1, PrefixTarget::E),
            0x4c => Instruction::BIT(BitPosition::B1, PrefixTarget::H),
            0x4d => Instruction::BIT(BitPosition::B1, PrefixTarget::L),
            0x4e => Instruction::BIT(BitPosition::B1, PrefixTarget::HLI),
            0x4f => Instruction::BIT(BitPosition::B1, PrefixTarget::A),

            0x50 => Instruction::BIT(BitPosition::B2, PrefixTarget::B),
            0x51 => Instruction::BIT(BitPosition::B2, PrefixTarget::C),
            0x52 => Instruction::BIT(BitPosition::B2, PrefixTarget::D),
            0x53 => Instruction::BIT(BitPosition::B2, PrefixTarget::E),
            0x54 => Instruction::BIT(BitPosition::B2, PrefixTarget::H),
            0x55 => Instruction::BIT(BitPosition::B2, PrefixTarget::L),
            0x56 => Instruction::BIT(BitPosition::B2, PrefixTarget::HLI),
            0x57 => Instruction::BIT(BitPosition::B2, PrefixTarget::A),

            0x58 => Instruction::BIT(BitPosition::B3, PrefixTarget::B),
            0x59 => Instruction::BIT(BitPosition::B3, PrefixTarget::C),
            0x5a => Instruction::BIT(BitPosition::B3, PrefixTarget::D),
            0x5b => Instruction::BIT(BitPosition::B3, PrefixTarget::E),
            0x5c => Instruction::BIT(BitPosition::B3, PrefixTarget::H),
            0x5d => Instruction::BIT(BitPosition::B3, PrefixTarget::L),
            0x5e => Instruction::BIT(BitPosition::B3, PrefixTarget::HLI),
            0x5f => Instruction::BIT(BitPosition::B3, PrefixTarget::A),

            0x60 => Instruction::BIT(BitPosition::B4, PrefixTarget::B),
            0x61 => Instruction::BIT(BitPosition::B4, PrefixTarget::C),
            0x62 => Instruction::BIT(BitPosition::B4, PrefixTarget::D),
            0x63 => Instruction::BIT(BitPosition::B4, PrefixTarget::E),
            0x64 => Instruction::BIT(BitPosition::B4, PrefixTarget::H),
            0x65 => Instruction::BIT(BitPosition::B4, PrefixTarget::L),
            0x66 => Instruction::BIT(BitPosition::B4, PrefixTarget::HLI),
            0x67 => Instruction::BIT(BitPosition::B4, PrefixTarget::A),

            0x68 => Instruction::BIT(BitPosition::B5, PrefixTarget::B),
            0x69 => Instruction::BIT(BitPosition::B5, PrefixTarget::C),
            0x6a => Instruction::BIT(BitPosition::B5, PrefixTarget::D),
            0x6b => Instruction::BIT(BitPosition::B5, PrefixTarget::E),
            0x6c => Instruction::BIT(BitPosition::B5, PrefixTarget::H),
            0x6d => Instruction::BIT(BitPosition::B5, PrefixTarget::L),
            0x6e => Instruction::BIT(BitPosition::B5, PrefixTarget::HLI),
            0x6f => Instruction::BIT(BitPosition::B5, PrefixTarget::A),

            0x70 => Instruction::BIT(BitPosition::B6, PrefixTarget::B),
            0x71 => Instruction::BIT(BitPosition::B6, PrefixTarget::C),
            0x72 => Instruction::BIT(BitPosition::B6, PrefixTarget::D),
            0x73 => Instruction::BIT(BitPosition::B6, PrefixTarget::E),
            0x74 => Instruction::BIT(BitPosition::B6, PrefixTarget::H),
            0x75 => Instruction::BIT(BitPosition::B6, PrefixTarget::L),
            0x76 => Instruction::BIT(BitPosition::B6, PrefixTarget::HLI),
            0x77 => Instruction::BIT(BitPosition::B6, PrefixTarget::A),

            0x78 => Instruction::BIT(BitPosition::B7, PrefixTarget::B),
            0x79 => Instruction::BIT(BitPosition::B7, PrefixTarget::C),
            0x7a => Instruction::BIT(BitPosition::B7, PrefixTarget::D),
            0x7b => Instruction::BIT(BitPosition::B7, PrefixTarget::E),
            0x7c => Instruction::BIT(BitPosition::B7, PrefixTarget::H),
            0x7d => Instruction::BIT(BitPosition::B7, PrefixTarget::L),
            0x7e => Instruction::BIT(BitPosition::B7, PrefixTarget::HLI),
            0x7f => Instruction::BIT(BitPosition::B7, PrefixTarget::A),

            0x80 => Instruction::RES(BitPosition::B0, PrefixTarget::B),
            0x81 => Instruction::RES(BitPosition::B0, PrefixTarget::C),
            0x82 => Instruction::RES(BitPosition::B0, PrefixTarget::D),
            0x83 => Instruction::RES(BitPosition::B0, PrefixTarget::E),
            0x84 => Instruction::RES(BitPosition::B0, PrefixTarget::H),
            0x85 => Instruction::RES(BitPosition::B0, PrefixTarget::L),
            0x86 => Instruction::RES(BitPosition::B0, PrefixTarget::HLI),
            0x87 => Instruction::RES(BitPosition::B0, PrefixTarget::A),

            0x88 => Instruction::RES(BitPosition::B1, PrefixTarget::B),
            0x89 => Instruction::RES(BitPosition::B1, PrefixTarget::C),
            0x8a => Instruction::RES(BitPosition::B1, PrefixTarget::D),
            0x8b => Instruction::RES(BitPosition::B1, PrefixTarget::E),
            0x8c => Instruction::RES(BitPosition::B1, PrefixTarget::H),
            0x8d => Instruction::RES(BitPosition::B1, PrefixTarget::L),
            0x8e => Instruction::RES(BitPosition::B1, PrefixTarget::HLI),
            0x8f => Instruction::RES(BitPosition::B1, PrefixTarget::A),

            0x90 => Instruction::RES(BitPosition::B2, PrefixTarget::B),
            0x91 => Instruction::RES(BitPosition::B2, PrefixTarget::C),
            0x92 => Instruction::RES(BitPosition::B2, PrefixTarget::D),
            0x93 => Instruction::RES(BitPosition::B2, PrefixTarget::E),
            0x94 => Instruction::RES(BitPosition::B2, PrefixTarget::H),
            0x95 => Instruction::RES(BitPosition::B2, PrefixTarget::L),
            0x96 => Instruction::RES(BitPosition::B2, PrefixTarget::HLI),
            0x97 => Instruction::RES(BitPosition::B2, PrefixTarget::A),

            0x98 => Instruction::RES(BitPosition::B3, PrefixTarget::B),
            0x99 => Instruction::RES(BitPosition::B3, PrefixTarget::C),
            0x9a => Instruction::RES(BitPosition::B3, PrefixTarget::D),
            0x9b => Instruction::RES(BitPosition::B3, PrefixTarget::E),
            0x9c => Instruction::RES(BitPosition::B3, PrefixTarget::H),
            0x9d => Instruction::RES(BitPosition::B3, PrefixTarget::L),
            0x9e => Instruction::RES(BitPosition::B3, PrefixTarget::HLI),
            0x9f => Instruction::RES(BitPosition::B3, PrefixTarget::A),

            0xa0 => Instruction::RES(BitPosition::B4, PrefixTarget::B),
            0xa1 => Instruction::RES(BitPosition::B4, PrefixTarget::C),
            0xa2 => Instruction::RES(BitPosition::B4, PrefixTarget::D),
            0xa3 => Instruction::RES(BitPosition::B4, PrefixTarget::E),
            0xa4 => Instruction::RES(BitPosition::B4, PrefixTarget::H),
            0xa5 => Instruction::RES(BitPosition::B4, PrefixTarget::L),
            0xa6 => Instruction::RES(BitPosition::B4, PrefixTarget::HLI),
            0xa7 => Instruction::RES(BitPosition::B4, PrefixTarget::A),

            0xa8 => Instruction::RES(BitPosition::B5, PrefixTarget::B),
            0xa9 => Instruction::RES(BitPosition::B5, PrefixTarget::C),
            0xaa => Instruction::RES(BitPosition::B5, PrefixTarget::D),
            0xab => Instruction::RES(BitPosition::B5, PrefixTarget::E),
            0xac => Instruction::RES(BitPosition::B5, PrefixTarget::H),
            0xad => Instruction::RES(BitPosition::B5, PrefixTarget::L),
            0xae => Instruction::RES(BitPosition::B5, PrefixTarget::HLI),
            0xaf => Instruction::RES(BitPosition::B5, PrefixTarget::A),

            0xb0 => Instruction::RES(BitPosition::B6, PrefixTarget::B),
            0xb1 => Instruction::RES(BitPosition::B6, PrefixTarget::C),
            0xb2 => Instruction::RES(BitPosition::B6, PrefixTarget::D),
            0xb3 => Instruction::RES(BitPosition::B6, PrefixTarget::E),
            0xb4 => Instruction::RES(BitPosition::B6, PrefixTarget::H),
            0xb5 => Instruction::RES(BitPosition::B6, PrefixTarget::L),
            0xb6 => Instruction::RES(BitPosition::B6, PrefixTarget::HLI),
            0xb7 => Instruction::RES(BitPosition::B6, PrefixTarget::A),

            0xb8 => Instruction::RES(BitPosition::B7, PrefixTarget::B),
            0xb9 => Instruction::RES(BitPosition::B7, PrefixTarget::C),
            0xba => Instruction::RES(BitPosition::B7, PrefixTarget::D),
            0xbb => Instruction::RES(BitPosition::B7, PrefixTarget::E),
            0xbc => Instruction::RES(BitPosition::B7, PrefixTarget::H),
            0xbd => Instruction::RES(BitPosition::B7, PrefixTarget::L),
            0xbe => Instruction::RES(BitPosition::B7, PrefixTarget::HLI),
            0xbf => Instruction::RES(BitPosition::B7, PrefixTarget::A),

            0xc0 => Instruction::SET(BitPosition::B0, PrefixTarget::B),
            0xc1 => Instruction::SET(BitPosition::B0, PrefixTarget::C),
            0xc2 => Instruction::SET(BitPosition::B0, PrefixTarget::D),
            0xc3 => Instruction::SET(BitPosition::B0, PrefixTarget::E),
            0xc4 => Instruction::SET(BitPosition::B0, PrefixTarget::H),
            0xc5 => Instruction::SET(BitPosition::B0, PrefixTarget::L),
            0xc6 => Instruction::SET(BitPosition::B0, PrefixTarget::HLI),
            0xc7 => Instruction::SET(BitPosition::B0, PrefixTarget::A),

            0xc8 => Instruction::SET(BitPosition::B1, PrefixTarget::B),
            0xc9 => Instruction::SET(BitPosition::B1, PrefixTarget::C),
            0xca => Instruction::SET(BitPosition::B1, PrefixTarget::D),
            0xcb => Instruction::SET(BitPosition::B1, PrefixTarget::E),
            0xcc => Instruction::SET(BitPosition::B1, PrefixTarget::H),
            0xcd => Instruction::SET(BitPosition::B1, PrefixTarget::L),
            0xce => Instruction::SET(BitPosition::B1, PrefixTarget::HLI),
            0xcf => Instruction::SET(BitPosition::B1, PrefixTarget::A),

            0xd0 => Instruction::SET(BitPosition::B2, PrefixTarget::B),
            0xd1 => Instruction::SET(BitPosition::B2, PrefixTarget::C),
            0xd2 => Instruction::SET(BitPosition::B2, PrefixTarget::D),
            0xd3 => Instruction::SET(BitPosition::B2, PrefixTarget::E),
            0xd4 => Instruction::SET(BitPosition::B2, PrefixTarget::H),
            0xd5 => Instruction::SET(BitPosition::B2, PrefixTarget::L),
            0xd6 => Instruction::SET(BitPosition::B2, PrefixTarget::HLI),
            0xd7 => Instruction::SET(BitPosition::B2, PrefixTarget::A),

            0xd8 => Instruction::SET(BitPosition::B3, PrefixTarget::B),
            0xd9 => Instruction::SET(BitPosition::B3, PrefixTarget::C),
            0xda => Instruction::SET(BitPosition::B3, PrefixTarget::D),
            0xdb => Instruction::SET(BitPosition::B3, PrefixTarget::E),
            0xdc => Instruction::SET(BitPosition::B3, PrefixTarget::H),
            0xdd => Instruction::SET(BitPosition::B3, PrefixTarget::L),
            0xde => Instruction::SET(BitPosition::B3, PrefixTarget::HLI),
            0xdf => Instruction::SET(BitPosition::B3, PrefixTarget::A),

            0xe0 => Instruction::SET(BitPosition::B4, PrefixTarget::B),
            0xe1 => Instruction::SET(BitPosition::B4, PrefixTarget::C),
            0xe2 => Instruction::SET(BitPosition::B4, PrefixTarget::D),
            0xe3 => Instruction::SET(BitPosition::B4, PrefixTarget::E),
            0xe4 => Instruction::SET(BitPosition::B4, PrefixTarget::H),
            0xe5 => Instruction::SET(BitPosition::B4, PrefixTarget::L),
            0xe6 => Instruction::SET(BitPosition::B4, PrefixTarget::HLI),
            0xe7 => Instruction::SET(BitPosition::B4, PrefixTarget::A),

            0xe8 => Instruction::SET(BitPosition::B5, PrefixTarget::B),
            0xe9 => Instruction::SET(BitPosition::B5, PrefixTarget::C),
            0xea => Instruction::SET(BitPosition::B5, PrefixTarget::D),
            0xeb => Instruction::SET(BitPosition::B5, PrefixTarget::E),
            0xec => Instruction::SET(BitPosition::B5, PrefixTarget::H),
            0xed => Instruction::SET(BitPosition::B5, PrefixTarget::L),
            0xee => Instruction::SET(BitPosition::B5, PrefixTarget::HLI),
            0xef => Instruction::SET(BitPosition::B5, PrefixTarget::A),

            0xf0 => Instruction::SET(BitPosition::B6, PrefixTarget::B),
            0xf1 => Instruction::SET(BitPosition::B6, PrefixTarget::C),
            0xf2 => Instruction::SET(BitPosition::B6, PrefixTarget::D),
            0xf3 => Instruction::SET(BitPosition::B6, PrefixTarget::E),
            0xf4 => Instruction::SET(BitPosition::B6, PrefixTarget::H),
            0xf5 => Instruction::SET(BitPosition::B6, PrefixTarget::L),
            0xf6 => Instruction::SET(BitPosition::B6, PrefixTarget::HLI),
            0xf7 => Instruction::SET(BitPosition::B6, PrefixTarget::A),

            0xf8 => Instruction::SET(BitPosition::B7, PrefixTarget::B),
            0xf9 => Instruction::SET(BitPosition::B7, PrefixTarget::C),
            0xfa => Instruction::SET(BitPosition::B7, PrefixTarget::D),
            0xfb => Instruction::SET(BitPosition::B7, PrefixTarget::E),
            0xfc => Instruction::SET(BitPosition::B7, PrefixTarget::H),
            0xfd => Instruction::SET(BitPosition::B7, PrefixTarget::L),
            0xfe => Instruction::SET(BitPosition::B7, PrefixTarget::HLI),
            0xff => Instruction::SET(BitPosition::B7, PrefixTarget::A),
        }
    }
}
