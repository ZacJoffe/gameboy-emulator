use std::convert::From;

struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
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
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags::new(),
            h: 0,
            l: 0
        }
    }

    fn get_af(&self) -> u16 { (self.a as u16) << 8 | u8::from(self.f) as u16 }

    fn set_af(&mut self, value: u16) {
        let a = ((value & 0xff00) >> 8) as u8;
        let f = Flags::from((value & 0x00ff) as u8);

        self.a = a;
        self.f = f;
    }

    fn get_bc(&self) -> u16 { (self.b as u16) << 8 | self.c as u16 }

    fn set_bc(&mut self, value: u16) {
        let b = ((value & 0xff00) >> 8) as u8;
        let c = (value & 0x00ff) as u8;

        self.b = b;
        self.c = c;
    }

    fn get_de(&self) -> u16 { (self.d as u16) << 8 | self.e as u16 }

    fn set_de(&mut self, value: u16) {
        let d = ((value & 0xff00) >> 8) as u8;
        let e = (value & 0x00ff) as u8;

        self.d = d;
        self.e = e;
    }

    fn get_hl(&self) -> u16 { (self.h as u16) << 8 | self.l as u16 }

    fn set_hl(&mut self, value: u16) {
        let h = ((value & 0xff00) >> 8) as u8;
        let l = (value & 0x00ff) as u8;

        self.h = h;
        self.l = l;
    }
}


struct CPU {
    registers: Registers,
}

enum Instruction {
    ADD(ArithTarget)
}

enum ArithTarget {
    A, B, C, D, E, H, L
}

impl CPU {
    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::ADD(target) => {
                match target {
                    ArithTarget::C => {
                        // add c to a
                        let value = self.registers.c;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (result, did_overflow) = self.registers.a.overflowing_add(value);

        // set zero flag if the result is equal to 0
        self.registers.f.zero = result == 0;

        // set subtract flag to false as this operation is an addition
        self.registers.f.subtract = false;

        // set carry flag if there was an overflow
        self.registers.f.carry = did_overflow;

        // set the half_carry flag if there was a carry to the upper nibble of a
        self.registers.f.half_carry = (self.registers.a & 0xf) + (value & 0xf) > 0xf;

        // return the result of the addition
        result
    }
}
