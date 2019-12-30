use std::convert::From;

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }
    }
}

impl From<&Flags> for u8 {
    fn from(flag: &Flags) -> u8 {
        let zero = if flag.zero { 1 } else { 0 };
        let subtract = if flag.subtract { 1 } else { 0 };
        let half_carry = if flag.half_carry { 1 } else { 0 };
        let carry = if flag.carry { 1 } else { 0 };

        (zero << 7) | (subtract << 6) | (half_carry << 5) | (carry << 4)
    }
}

impl From<Flags> for u8 {
    fn from(flag: Flags) -> u8 {
        let zero = if flag.zero { 1 } else { 0 };
        let subtract = if flag.subtract { 1 } else { 0 };
        let half_carry = if flag.half_carry { 1 } else { 0 };
        let carry = if flag.carry { 1 } else { 0 };

        (zero << 7) | (subtract << 6) | (half_carry << 5) | (carry << 4)
    }
}


impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> 7) & 0b1) != 0;
        let subtract = ((byte >> 6) & 0b1) != 0;
        let half_carry = ((byte >> 5) & 0b1) != 0;
        let carry = ((byte >> 4) & 0b1) != 0;

        Flags {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
