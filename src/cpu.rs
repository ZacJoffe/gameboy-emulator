use crate::registers::Registers;
use crate::memory_bus::MemoryBus;
use crate::instructions::*;

struct CPU {
    pc: u16,
    sp: u16,
    registers: Registers,
    bus: MemoryBus
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
            },
            Instruction::CP(target) => {
                self.cp(target);
            },
            Instruction::INC(target) => {

            }
        }
    }

    // reads the next byte in memory
    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    // ADD instruction
    fn add(&mut self, target: ArithTarget) {
        let value = self.get_register_from_arith(target);
        let result = self.add_a(value);
        self.registers.a = result;
    }

    // SUB instruction
    fn sub(&mut self, target: ArithTarget) {
        let value = self.get_register_from_arith(target);
        let result = self.sub_a(value);
        self.registers.a = result;
    }

    // CP instruction
    fn cp(&mut self, target: ArithTarget) {
        let value = self.get_register_from_arith(target);

        // set the flags accordingly
        self.sub_a(value);
    }

    fn inc(&mut self, target: IncDecTarget) {
        match target {
            IncDecTarget::BC => { self.registers.set_bc(self.registers.get_bc() + 1); },
            IncDecTarget::DE => { self.registers.set_de(self.registers.get_de() + 1); },
            IncDecTarget::HL => { self.registers.set_hl(self.registers.get_hl() + 1); },
            IncDecTarget::SP => { self.sp += 1; },
            IncDecTarget::A => {
                // let (result, _) = self.registers.a.overflowing_add(1);
                let result = self.registers.a.wrapping_add(1);

                // note: carry flag not affected
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.a & 0xf) + (1 & 0xf) > 0xf;

                self.registers.a = result;
            },
            IncDecTarget::B => {
                let result = self.registers.b.wrapping_add(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.b & 0xf) + (1 & 0xf) > 0xf;

                self.registers.b = result;
            },
            IncDecTarget::C => {
                let result = self.registers.c.wrapping_add(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.c & 0xf) + (1 & 0xf) > 0xf;

                self.registers.c = result;
            }
            IncDecTarget::D => {
                let result = self.registers.d.wrapping_add(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.d & 0xf) + (1 & 0xf) > 0xf;

                self.registers.d = result;
            },
            IncDecTarget::E => {
                let result = self.registers.e.wrapping_add(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.e & 0xf) + (1 & 0xf) > 0xf;

                self.registers.e = result;
            },
            IncDecTarget::H => {
                let result = self.registers.h.wrapping_add(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.h & 0xf) + (1 & 0xf) > 0xf;

                self.registers.h = result;
            },
            IncDecTarget::L => {
                let result = self.registers.l.wrapping_add(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.l & 0xf) + (1 & 0xf) > 0xf;

                self.registers.l = result;
            },
            IncDecTarget::HLI => {
                let result = self.bus.read_byte(self.registers.get_hl()) + 1;

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.bus.read_byte(self.registers.get_hl()) & 0xf) + (1 & 0xf) > 0xf;

                self.bus.set_byte(self.registers.get_hl(), result);
            }
        }
    }

    // AND instruction
    fn and(&mut self, target: ArithTarget) {
        // set a to itself anded with the value of the target register
        self.registers.a &= self.get_register_from_arith(target);

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
        self.registers.a |= self.get_register_from_arith(target);

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
        self.registers.a ^= self.get_register_from_arith(target);

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
    fn get_register_from_arith(&self, target: ArithTarget) -> u8 {
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
