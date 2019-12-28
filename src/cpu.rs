use std::convert::From;

struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
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

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8
}

impl Registers {
    fn get_bc(&self) -> u16 { (self.b as u16) << 8 | self.c as u16 }

    fn set_bc(&mut self, value: u16) {
        let b = ((value & 0xff00) >> 8) as u8;
        let c = (value & 0x00ff) as u8;

        self.b = b;
        self.c = c;
    }
}
