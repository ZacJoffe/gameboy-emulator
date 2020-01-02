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
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),

    NOP
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

impl Instruction {
    fn disassemble(byte: u8, is_prefixed: bool) -> Option<Instruction> {
        if is_prefixed {
            Instruction::disassemble_prefixed(byte)
        } else {
            Instruction::disassemble_not_prefixed(byte)
        }
    }

    // todo when instruction enum is completely filled
    fn disassemble_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),

            _ => None
        }
    }

    // todo: check over
    fn disassemble_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            0x01 => Some(Instruction::RLC(PrefixTarget::C)),
            0x02 => Some(Instruction::RLC(PrefixTarget::D)),
            0x03 => Some(Instruction::RLC(PrefixTarget::E)),
            0x04 => Some(Instruction::RLC(PrefixTarget::H)),
            0x05 => Some(Instruction::RLC(PrefixTarget::L)),
            0x06 => Some(Instruction::RLC(PrefixTarget::HLI)),
            0x07 => Some(Instruction::RLC(PrefixTarget::A)),

            0x08 => Some(Instruction::RRC(PrefixTarget::B)),
            0x09 => Some(Instruction::RRC(PrefixTarget::C)),
            0x0a => Some(Instruction::RRC(PrefixTarget::D)),
            0x0b => Some(Instruction::RRC(PrefixTarget::E)),
            0x0c => Some(Instruction::RRC(PrefixTarget::H)),
            0x0d => Some(Instruction::RRC(PrefixTarget::L)),
            0x0e => Some(Instruction::RRC(PrefixTarget::HLI)),
            0x0f => Some(Instruction::RRC(PrefixTarget::A)),

            0x10 => Some(Instruction::RL(PrefixTarget::B)),
            0x11 => Some(Instruction::RL(PrefixTarget::C)),
            0x12 => Some(Instruction::RL(PrefixTarget::D)),
            0x13 => Some(Instruction::RL(PrefixTarget::E)),
            0x14 => Some(Instruction::RL(PrefixTarget::H)),
            0x15 => Some(Instruction::RL(PrefixTarget::L)),
            0x16 => Some(Instruction::RL(PrefixTarget::HLI)),
            0x17 => Some(Instruction::RL(PrefixTarget::A)),

            0x18 => Some(Instruction::RR(PrefixTarget::B)),
            0x19 => Some(Instruction::RR(PrefixTarget::C)),
            0x1a => Some(Instruction::RR(PrefixTarget::D)),
            0x1b => Some(Instruction::RR(PrefixTarget::E)),
            0x1c => Some(Instruction::RR(PrefixTarget::H)),
            0x1d => Some(Instruction::RR(PrefixTarget::L)),
            0x1e => Some(Instruction::RR(PrefixTarget::HLI)),
            0x1f => Some(Instruction::RR(PrefixTarget::A)),

            0x20 => Some(Instruction::SLA(PrefixTarget::B)),
            0x21 => Some(Instruction::SLA(PrefixTarget::C)),
            0x22 => Some(Instruction::SLA(PrefixTarget::D)),
            0x23 => Some(Instruction::SLA(PrefixTarget::E)),
            0x24 => Some(Instruction::SLA(PrefixTarget::H)),
            0x25 => Some(Instruction::SLA(PrefixTarget::L)),
            0x26 => Some(Instruction::SLA(PrefixTarget::HLI)),
            0x27 => Some(Instruction::SLA(PrefixTarget::A)),

            0x28 => Some(Instruction::SRA(PrefixTarget::B)),
            0x29 => Some(Instruction::SRA(PrefixTarget::C)),
            0x2a => Some(Instruction::SRA(PrefixTarget::D)),
            0x2b => Some(Instruction::SRA(PrefixTarget::E)),
            0x2c => Some(Instruction::SRA(PrefixTarget::H)),
            0x2d => Some(Instruction::SRA(PrefixTarget::L)),
            0x2e => Some(Instruction::SRA(PrefixTarget::HLI)),
            0x2f => Some(Instruction::SRA(PrefixTarget::A)),

            0x30 => Some(Instruction::SWAP(PrefixTarget::B)),
            0x31 => Some(Instruction::SWAP(PrefixTarget::C)),
            0x32 => Some(Instruction::SWAP(PrefixTarget::D)),
            0x33 => Some(Instruction::SWAP(PrefixTarget::E)),
            0x34 => Some(Instruction::SWAP(PrefixTarget::H)),
            0x35 => Some(Instruction::SWAP(PrefixTarget::L)),
            0x36 => Some(Instruction::SWAP(PrefixTarget::HLI)),
            0x37 => Some(Instruction::SWAP(PrefixTarget::A)),

            0x38 => Some(Instruction::SRL(PrefixTarget::B)),
            0x39 => Some(Instruction::SRL(PrefixTarget::C)),
            0x3a => Some(Instruction::SRL(PrefixTarget::D)),
            0x3b => Some(Instruction::SRL(PrefixTarget::E)),
            0x3c => Some(Instruction::SRL(PrefixTarget::H)),
            0x3d => Some(Instruction::SRL(PrefixTarget::L)),
            0x3e => Some(Instruction::SRL(PrefixTarget::HLI)),
            0x3f => Some(Instruction::SRL(PrefixTarget::A)),

            0x40 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::B)),
            0x41 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::C)),
            0x42 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::D)),
            0x43 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::E)),
            0x44 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::H)),
            0x45 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::L)),
            0x46 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::HLI)),
            0x47 => Some(Instruction::BIT(BitPosition::B0, PrefixTarget::A)),

            0x48 => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::B)),
            0x49 => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::C)),
            0x4a => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::D)),
            0x4b => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::E)),
            0x4c => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::H)),
            0x4d => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::L)),
            0x4e => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::HLI)),
            0x4f => Some(Instruction::BIT(BitPosition::B1, PrefixTarget::A)),

            0x50 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::B)),
            0x51 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::C)),
            0x52 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::D)),
            0x53 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::E)),
            0x54 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::H)),
            0x55 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::L)),
            0x56 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::HLI)),
            0x57 => Some(Instruction::BIT(BitPosition::B2, PrefixTarget::A)),

            0x58 => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::B)),
            0x59 => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::C)),
            0x5a => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::D)),
            0x5b => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::E)),
            0x5c => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::H)),
            0x5d => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::L)),
            0x5e => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::HLI)),
            0x5f => Some(Instruction::BIT(BitPosition::B3, PrefixTarget::A)),

            0x60 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::B)),
            0x61 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::C)),
            0x62 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::D)),
            0x63 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::E)),
            0x64 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::H)),
            0x65 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::L)),
            0x66 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::HLI)),
            0x67 => Some(Instruction::BIT(BitPosition::B4, PrefixTarget::A)),

            0x68 => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::B)),
            0x69 => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::C)),
            0x6a => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::D)),
            0x6b => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::E)),
            0x6c => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::H)),
            0x6d => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::L)),
            0x6e => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::HLI)),
            0x6f => Some(Instruction::BIT(BitPosition::B5, PrefixTarget::A)),

            0x70 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::B)),
            0x71 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::C)),
            0x72 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::D)),
            0x73 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::E)),
            0x74 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::H)),
            0x75 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::L)),
            0x76 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::HLI)),
            0x77 => Some(Instruction::BIT(BitPosition::B6, PrefixTarget::A)),

            0x78 => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::B)),
            0x79 => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::C)),
            0x7a => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::D)),
            0x7b => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::E)),
            0x7c => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::H)),
            0x7d => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::L)),
            0x7e => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::HLI)),
            0x7f => Some(Instruction::BIT(BitPosition::B7, PrefixTarget::A)),

            0x80 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::B)),
            0x81 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::C)),
            0x82 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::D)),
            0x83 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::E)),
            0x84 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::H)),
            0x85 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::L)),
            0x86 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::HLI)),
            0x87 => Some(Instruction::RES(BitPosition::B0, PrefixTarget::A)),

            0x88 => Some(Instruction::RES(BitPosition::B1, PrefixTarget::B)),
            0x89 => Some(Instruction::RES(BitPosition::B1, PrefixTarget::C)),
            0x8a => Some(Instruction::RES(BitPosition::B1, PrefixTarget::D)),
            0x8b => Some(Instruction::RES(BitPosition::B1, PrefixTarget::E)),
            0x8c => Some(Instruction::RES(BitPosition::B1, PrefixTarget::H)),
            0x8d => Some(Instruction::RES(BitPosition::B1, PrefixTarget::L)),
            0x8e => Some(Instruction::RES(BitPosition::B1, PrefixTarget::HLI)),
            0x8f => Some(Instruction::RES(BitPosition::B1, PrefixTarget::A)),

            0x90 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::B)),
            0x91 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::C)),
            0x92 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::D)),
            0x93 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::E)),
            0x94 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::H)),
            0x95 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::L)),
            0x96 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::HLI)),
            0x97 => Some(Instruction::RES(BitPosition::B2, PrefixTarget::A)),

            0x98 => Some(Instruction::RES(BitPosition::B3, PrefixTarget::B)),
            0x99 => Some(Instruction::RES(BitPosition::B3, PrefixTarget::C)),
            0x9a => Some(Instruction::RES(BitPosition::B3, PrefixTarget::D)),
            0x9b => Some(Instruction::RES(BitPosition::B3, PrefixTarget::E)),
            0x9c => Some(Instruction::RES(BitPosition::B3, PrefixTarget::H)),
            0x9d => Some(Instruction::RES(BitPosition::B3, PrefixTarget::L)),
            0x9e => Some(Instruction::RES(BitPosition::B3, PrefixTarget::HLI)),
            0x9f => Some(Instruction::RES(BitPosition::B3, PrefixTarget::A)),

            0xa0 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::B)),
            0xa1 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::C)),
            0xa2 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::D)),
            0xa3 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::E)),
            0xa4 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::H)),
            0xa5 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::L)),
            0xa6 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::HLI)),
            0xa7 => Some(Instruction::RES(BitPosition::B4, PrefixTarget::A)),

            0xa8 => Some(Instruction::RES(BitPosition::B5, PrefixTarget::B)),
            0xa9 => Some(Instruction::RES(BitPosition::B5, PrefixTarget::C)),
            0xaa => Some(Instruction::RES(BitPosition::B5, PrefixTarget::D)),
            0xab => Some(Instruction::RES(BitPosition::B5, PrefixTarget::E)),
            0xac => Some(Instruction::RES(BitPosition::B5, PrefixTarget::H)),
            0xad => Some(Instruction::RES(BitPosition::B5, PrefixTarget::L)),
            0xae => Some(Instruction::RES(BitPosition::B5, PrefixTarget::HLI)),
            0xaf => Some(Instruction::RES(BitPosition::B5, PrefixTarget::A)),

            0xb0 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::B)),
            0xb1 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::C)),
            0xb2 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::D)),
            0xb3 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::E)),
            0xb4 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::H)),
            0xb5 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::L)),
            0xb6 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::HLI)),
            0xb7 => Some(Instruction::RES(BitPosition::B6, PrefixTarget::A)),

            0xb8 => Some(Instruction::RES(BitPosition::B7, PrefixTarget::B)),
            0xb9 => Some(Instruction::RES(BitPosition::B7, PrefixTarget::C)),
            0xba => Some(Instruction::RES(BitPosition::B7, PrefixTarget::D)),
            0xbb => Some(Instruction::RES(BitPosition::B7, PrefixTarget::E)),
            0xbc => Some(Instruction::RES(BitPosition::B7, PrefixTarget::H)),
            0xbd => Some(Instruction::RES(BitPosition::B7, PrefixTarget::L)),
            0xbe => Some(Instruction::RES(BitPosition::B7, PrefixTarget::HLI)),
            0xbf => Some(Instruction::RES(BitPosition::B7, PrefixTarget::A)),

            0xc0 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::B)),
            0xc1 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::C)),
            0xc2 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::D)),
            0xc3 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::E)),
            0xc4 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::H)),
            0xc5 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::L)),
            0xc6 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::HLI)),
            0xc7 => Some(Instruction::SET(BitPosition::B0, PrefixTarget::A)),

            0xc8 => Some(Instruction::SET(BitPosition::B1, PrefixTarget::B)),
            0xc9 => Some(Instruction::SET(BitPosition::B1, PrefixTarget::C)),
            0xca => Some(Instruction::SET(BitPosition::B1, PrefixTarget::D)),
            0xcb => Some(Instruction::SET(BitPosition::B1, PrefixTarget::E)),
            0xcc => Some(Instruction::SET(BitPosition::B1, PrefixTarget::H)),
            0xcd => Some(Instruction::SET(BitPosition::B1, PrefixTarget::L)),
            0xce => Some(Instruction::SET(BitPosition::B1, PrefixTarget::HLI)),
            0xcf => Some(Instruction::SET(BitPosition::B1, PrefixTarget::A)),

            0xd0 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::B)),
            0xd1 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::C)),
            0xd2 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::D)),
            0xd3 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::E)),
            0xd4 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::H)),
            0xd5 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::L)),
            0xd6 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::HLI)),
            0xd7 => Some(Instruction::SET(BitPosition::B2, PrefixTarget::A)),

            0xd8 => Some(Instruction::SET(BitPosition::B3, PrefixTarget::B)),
            0xd9 => Some(Instruction::SET(BitPosition::B3, PrefixTarget::C)),
            0xda => Some(Instruction::SET(BitPosition::B3, PrefixTarget::D)),
            0xdb => Some(Instruction::SET(BitPosition::B3, PrefixTarget::E)),
            0xdc => Some(Instruction::SET(BitPosition::B3, PrefixTarget::H)),
            0xdd => Some(Instruction::SET(BitPosition::B3, PrefixTarget::L)),
            0xde => Some(Instruction::SET(BitPosition::B3, PrefixTarget::HLI)),
            0xdf => Some(Instruction::SET(BitPosition::B3, PrefixTarget::A)),

            0xe0 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::B)),
            0xe1 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::C)),
            0xe2 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::D)),
            0xe3 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::E)),
            0xe4 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::H)),
            0xe5 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::L)),
            0xe6 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::HLI)),
            0xe7 => Some(Instruction::SET(BitPosition::B4, PrefixTarget::A)),

            0xe8 => Some(Instruction::SET(BitPosition::B5, PrefixTarget::B)),
            0xe9 => Some(Instruction::SET(BitPosition::B5, PrefixTarget::C)),
            0xea => Some(Instruction::SET(BitPosition::B5, PrefixTarget::D)),
            0xeb => Some(Instruction::SET(BitPosition::B5, PrefixTarget::E)),
            0xec => Some(Instruction::SET(BitPosition::B5, PrefixTarget::H)),
            0xed => Some(Instruction::SET(BitPosition::B5, PrefixTarget::L)),
            0xee => Some(Instruction::SET(BitPosition::B5, PrefixTarget::HLI)),
            0xef => Some(Instruction::SET(BitPosition::B5, PrefixTarget::A)),

            0xf0 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::B)),
            0xf1 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::C)),
            0xf2 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::D)),
            0xf3 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::E)),
            0xf4 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::H)),
            0xf5 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::L)),
            0xf6 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::HLI)),
            0xf7 => Some(Instruction::SET(BitPosition::B6, PrefixTarget::A)),

            0xf8 => Some(Instruction::SET(BitPosition::B7, PrefixTarget::B)),
            0xf9 => Some(Instruction::SET(BitPosition::B7, PrefixTarget::C)),
            0xfa => Some(Instruction::SET(BitPosition::B7, PrefixTarget::D)),
            0xfb => Some(Instruction::SET(BitPosition::B7, PrefixTarget::E)),
            0xfc => Some(Instruction::SET(BitPosition::B7, PrefixTarget::H)),
            0xfd => Some(Instruction::SET(BitPosition::B7, PrefixTarget::L)),
            0xfe => Some(Instruction::SET(BitPosition::B7, PrefixTarget::HLI)),
            0xff => Some(Instruction::SET(BitPosition::B7, PrefixTarget::A)),
        }
    }
}
