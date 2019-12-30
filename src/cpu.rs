use std::convert::From;

struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
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

struct MemoryBus {
    memory: [u8; 0xffff]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize];
    }
}

struct CPU {
    pc: u16,
    sp: u16,
    registers: Registers,
    bus: MemoryBus
}


enum Instruction {
    ADD(ArithTarget),
    ADC(ArithTarget),
    ADDHL(AddHLTarget),
    SUB(ArithTarget),
    SBC(ArithTarget),
    AND(ArithTarget),
    OR(ArithTarget),
    XOR(ArithTarget)
}

enum ArithTarget {
    A, B, C, D, E, H, L, D8, HLI
}

enum AddHLTarget {
    BC, DE, HL, SP
}

impl CPU {
    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::ADD(target) => {
                self.add(target);
            }
            Instruction::ADC(target) => {
                // do an ADD, then add the carry
                self.add(target);
                self.add_a(self.registers.f.carry as u8);
            }
            Instruction::ADDHL(target) => {
                let value = match target {
                    AddHLTarget::BC => { self.registers.get_bc() },
                    AddHLTarget::DE => { self.registers.get_de() },
                    AddHLTarget::HL => { self.registers.get_hl() },
                    AddHLTarget::SP => { self.sp },
                };
                let result = self.add_hl(value);
                self.registers.set_hl(result);
            },
            Instruction::SUB(target) => {
                self.sub(target);
            },
            Instruction::SBC(target) => {
                 // do a SUB, then add the carry
                self.sub(target);
                self.sub_a(self.registers.f.carry as u8);
            },
            Instruction::AND(target) => {
                self.and(target);
            },
            Instruction::OR(target) => {
                self.or(target);
            },
            Instruction::XOR(target) => {
                self.xor(target);
            }
            _ => {}
        }
    }

    // reads the next byte in memory
    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    // ADD instruction
    fn add(&mut self, target: ArithTarget) {
        /*
        match target {
            ArithTarget::HLI => {

            },
            ArithTarget::D8 => {

            },
            ArithTarget::A => {
                let value = self.registers.a;
                let result = self.add_a(value);
                self.registers.a = result;
            },
            ArithTarget::B => {
                let value = self.registers.b;
                let result = self.add_a(value);
                self.registers.a = result;
            },
            ArithTarget::C => {
                let value = self.registers.c;
                let result = self.add_a(value);
                self.registers.a = result;
            },
            ArithTarget::D => {
                let value = self.registers.d;
                let result = self.add_a(value);
                self.registers.a = result;
            },
            ArithTarget::E => {
                let value = self.registers.e;
                let result = self.add_a(value);
                self.registers.a = result;
            },
            ArithTarget::H => {
                let value = self.registers.h;
                let result = self.add_a(value);
                self.registers.a = result;
            },
            ArithTarget::L => {
                let value = self.registers.l;
                let result = self.add_a(value);
                self.registers.a = result;
            }
        }
        */
        let value = self.get_register(target);
        let result = self.add_a(value);
        self.registers.a = result;
    }

    // SUB instruction
    fn sub(&mut self, target:ArithTarget) {
        /*
        match target {
            ArithTarget::HLI => {

            },
            ArithTarget::D8 => {

            },
            ArithTarget::A => {
                let value = self.registers.a;
                let result = self.sub_a(value);
                self.registers.a = result;
            },
            ArithTarget::B => {
                let value = self.registers.b;
                let result = self.sub_a(value);
                self.registers.a = result;
            },
            ArithTarget::C => {
                let value = self.registers.c;
                let result = self.sub_a(value);
                self.registers.a = result;
            },
            ArithTarget::D => {
                let value = self.registers.d;
                let result = self.sub_a(value);
                self.registers.a = result;
            },
            ArithTarget::E => {
                let value = self.registers.e;
                let result = self.sub_a(value);
                self.registers.a = result;
            },
            ArithTarget::H => {
                let value = self.registers.h;
                let result = self.sub_a(value);
                self.registers.a = result;
            },
            ArithTarget::L => {
                let value = self.registers.l;
                let result = self.sub_a(value);
                self.registers.a = result;
            }
        }
        */
        let value = self.get_register(target);
        let result = self.sub_a(value);
        self.registers.a = result;
    }

    // AND instruction
    fn and(&mut self, target: ArithTarget) {
        // set a to itself anded with the value of the target register
        self.registers.a &= self.get_register(target);

        // set zero flag if the result of the and is equal to 0
        self.registers.f.zero = self.registers.a == 0;

        // set subtract flag to false as this is an and operation
        self.registers.f.subtract = false;

        // reset carry flag
        self.registers.f.carry = false;

        // set half_carry flag
        self.registers.f.half_carry = true;
    }

    // OR instruction
    fn or(&mut self, target: ArithTarget) {
        // set a to itself ored with the value of the target register
        self.registers.a |= self.get_register(target);

        // set zero flag if the result of the and is equal to 0
        self.registers.f.zero = self.registers.a == 0;

        // set subtract flag to false as this is an and operation
        self.registers.f.subtract = false;

        // reset carry flag
        self.registers.f.carry = false;

        // reset half_carry flag
        self.registers.f.half_carry = false;
    }

    // XOR instruction
    fn xor(&mut self, target: ArithTarget) {
        // set a to itself xored with the value of the target register
        self.registers.a ^= self.get_register(target);

        // set zero flag if the result of the and is equal to 0
        self.registers.f.zero = self.registers.a == 0;

        // set subtract flag to false as this is an and operation
        self.registers.f.subtract = false;

        // reset carry flag
        self.registers.f.carry = false;

        // reset half_carry flag
        self.registers.f.half_carry = false;
    }

    // get register value from arith target
    fn get_register(&self, target: ArithTarget) -> u8 {
        match target {
            ArithTarget::HLI => { self.bus.read_byte(self.registers.get_hl()) },
            ArithTarget::D8 => { self.read_next_byte() },
            ArithTarget::A => { self.registers.a },
            ArithTarget::B => { self.registers.b },
            ArithTarget::C => { self.registers.c },
            ArithTarget::D => { self.registers.d },
            ArithTarget::E => { self.registers.e },
            ArithTarget::H => { self.registers.h },
            ArithTarget::L => { self.registers.l }
        }
    }

    // add to register a and set flags accordingly
    fn add_a(&mut self, value: u8) -> u8 {
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

    // add to registers hl and set flags accordingly
    fn add_hl(&mut self, value: u16) -> u16 {
        let (result, did_overflow) = self.registers.get_hl().overflowing_add(value);

        // zero flag not set

        // set subtract flag to false as this operation is an addition
        self.registers.f.subtract = false;

        // set carry flag if there was an overflow
        self.registers.f.carry = did_overflow;

        // set the half_carry flag if there was a carry to the upper nibble of hl
        self.registers.f.half_carry = (self.registers.get_hl() & 0xfff) + (value & 0xfff) > 0xfff;

        // return the result of the addition
        result
    }

    // sub from a, set flags accordingly
    fn sub_a(&mut self, value: u8) -> u8 {
        let (result, did_underflow) = self.registers.a.overflowing_sub(value);

        // set zero flag if the result is equal to 0
        self.registers.f.zero = result == 0;

        // set subtract flag to true as this operation is a subtraction
        self.registers.f.subtract = false;

        // set carry flag if there was a borrow
        self.registers.f.carry = did_underflow;

        // set the half_carry flag if there was a borrow from bit 4
        self.registers.f.half_carry = (self.registers.a & 0xf) < (value & 0xf);

        // return the result of the subtraction
        result
    }
}
