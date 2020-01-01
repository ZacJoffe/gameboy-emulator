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
                self.inc(target);
            },
            Instruction::DEC(target) => {
                self.dec(target);
            },
            Instruction::CCF => {
                // CCF instruction
                // reset subtract and half_carry flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                // toggle carry flag
                self.registers.f.carry = if self.registers.f.carry { false } else { true };
            },
            Instruction::SCF => {
                // SCF instruction
                // reset subtract and half_carry flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                // set carry flag
                self.registers.f.carry = true;
            },
            Instruction::RRA => {
                // get LSB of register a
                let new_carry = self.registers.a & 0x1;

                // rotate right through carry
                self.registers.a >>= 1;
                self.registers.a |= if self.registers.f.carry { 0x1 << 7 } else { 0x0 };

                // set carry flag to the LSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::RLA => {
                // get MSB of register a
                let new_carry = (self.registers.a & 0x80) >> 7;

                // rotate left through carry
                self.registers.a <<= 1;
                self.registers.a |= if self.registers.f.carry { 0x1 } else { 0x0 };

                // set carry flag to the MSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::RRCA => {
                // get LSB of register a
                let new_carry = self.registers.a & 0x1;

                // rotate right
                self.registers.a = self.registers.a.rotate_right(1);

                // set carry flag to the LSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::RLCA => {
                // get MSB of register a
                let new_carry = (self.registers.a & 0x80) >> 7;

                // rotate left
                self.registers.a = self.registers.a.rotate_left(1);

                // set carry flag to the MSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::CPL => {
                // flip all bits of data in register a
                self.registers.a = !self.registers.a;

                // set subtract and half_carry flags, don't touch the others
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
            },
            Instruction::BIT(pos, target) => {
                // get shift value in variable so we don't move twice
                let shift_value = u8::from(pos);

                // mask it with 1 shifted left to the correct position,
                // and shift it back to get the value of that bit
                let bit = (self.get_register_from_prefix(target) & (0x1 << shift_value)) >> shift_value;

                // set zero if flag if the bit is 0
                self.registers.f.zero = bit == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
            },
            Instruction::SET(pos, target) => {
                // shift 0x1 to the required bit position for the or operation
                let bit_set = 0x1 << u8::from(pos);

                match target {
                    PrefixTarget::A => { self.registers.a |= bit_set },
                    PrefixTarget::B => { self.registers.b |= bit_set },
                    PrefixTarget::C => { self.registers.c |= bit_set },
                    PrefixTarget::D => { self.registers.d |= bit_set },
                    PrefixTarget::E => { self.registers.e |= bit_set },
                    PrefixTarget::H => { self.registers.h |= bit_set },
                    PrefixTarget::L => { self.registers.l |= bit_set },
                    PrefixTarget::HLI => { self.bus.set_byte(self.registers.get_hl(), self.bus.read_byte(self.registers.get_hl()) | bit_set) }
                }
            },
            Instruction::RES(pos, target) => {
                // rotate 0xfe
                let base: u8 = 0b1111_1110;
                let bit_mask = base.rotate_left(u8::from(pos) as u32);

                match target {
                    PrefixTarget::A => { self.registers.a &= bit_mask },
                    PrefixTarget::B => { self.registers.b &= bit_mask },
                    PrefixTarget::C => { self.registers.c &= bit_mask },
                    PrefixTarget::D => { self.registers.d &= bit_mask },
                    PrefixTarget::E => { self.registers.e &= bit_mask },
                    PrefixTarget::H => { self.registers.h &= bit_mask },
                    PrefixTarget::L => { self.registers.l &= bit_mask },
                    PrefixTarget::HLI => { self.bus.set_byte(self.registers.get_hl(), self.bus.read_byte(self.registers.get_hl()) & bit_mask) }
                }
            },
            Instruction::SRL(target) => {
                // note that this opcode does a logical shift right,
                // meaning that the MSB is discarded in the shift
                //
                // by default, rust will do a logical shift with u8,
                // so we do not need to do anything special
                let value = self.get_register_from_prefix(target);

                // put LSB of register before shift into carry flag
                self.registers.f.carry = (value & 0x1) != 0;

                // shift right
                let result = value >> 1;

                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                /*
                match target {
                    PrefixTarget::A => { self.registers.a = value; },
                    PrefixTarget::B => { self.registers.b = value; },
                    PrefixTarget::C => { self.registers.c = value; },
                    PrefixTarget::D => { self.registers.d = value; },
                    PrefixTarget::E => { self.registers.e = value; },
                    PrefixTarget::H => { self.registers.h = value; },
                    PrefixTarget::L => { self.registers.l = value; },
                    PrefixTarget::HLI => { self.bus.set_byte(self.registers.get_hl(), value); }
                }
                */
                self.set_register_from_prefix(target, value);
            },
            Instruction::RR(target) => {
                let value = self.get_register_from_prefix(target);

                // get LSB of target
                let new_carry = value & 0x1;

                // shift value right and set the MSB to the value of the carry flag
                let result = (value >> 1) | if self.registers.f.carry { 0x1 << 7 } else { 0x0 };

                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                self.set_register_from_prefix(target, value);
            },
            Instruction::RL(target) => {
                let value = self.get_register_from_prefix(target);

                // get MSB of target
                let new_carry = (value & 0x80) >> 7;

                // shift value right and set the MSB to the value of the carry flag
                let result = (value << 1) | if self.registers.f.carry { 0x1 } else { 0x0 };

                // set flags
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                self.set_register_from_prefix(target, value);
            },
            Instruction::RRC(target) => {
                let value = self.get_register_from_prefix(target);

                // get LSB of target
                let new_carry = value & 0x1;

                // rotate the value right
                let result = value.rotate_right(1);

                // set flags
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                // set the flag to the new value
                self.set_register_from_prefix(target, result);
            },
            Instruction::RLC(target) => {
                let value = self.get_register_from_prefix(target);

                // get MSB of target
                let new_carry = (value & 0x80) >> 7;

                // rotate the value left
                let result = value.rotate_left(1);

                // set flags
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                // set to value rotated right
                self.set_register_from_prefix(target, result);
            },
            Instruction::SRA(target) => {
                // note this instruction needs to do an arithmetic shift
                // thus, we need to preserve the MSB
                //
                // in rust, shifting a u8 is automatically logical
                let value = self.get_register_from_prefix(target);

                // put LSB of register before shift into carry flag
                self.registers.f.carry = (value & 0x1) != 0;

                // get the MSB of value
                let msb = (value & 0x80) >> 7;

                // if the MSB is 1, then shift right and or it with 0x80 to set the new MSB
                // otherwise, just shift right and introduce the 0 normally
                let result = if msb != 0 {
                    (value >> 1) | 0x80
                } else {
                    value >> 1
                };

                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                self.set_register_from_prefix(target, result);
            },
            Instruction::SLA(target) => {
                let value = self.get_register_from_prefix(target);

                // put MSB of register before shift into carry flag
                self.registers.f.carry = (value & 0x80) >> 7 != 0;

                // shift left
                let result = value << 1;

                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                self.set_register_from_prefix(target, result);
            },
            Instruction::SWAP(target) => {
                let value = self.get_register_from_prefix(target);

                // get upper and lower nibbles of the value
                let upper = (value & 0xf0) >> 4;
                let lower = (value & 0xf);

                // combine the lower and upper nibbles to perform the swap
                let result = (lower << 4) | upper;

                // set registers accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;

                self.set_register_from_prefix(target, result);
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

    // INC instruction
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
            },
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
                // let result = self.bus.read_byte(self.registers.get_hl()) + 1;
                let result = self.bus.read_byte(self.registers.get_hl()).wrapping_add(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.bus.read_byte(self.registers.get_hl()) & 0xf) + (1 & 0xf) > 0xf;

                self.bus.set_byte(self.registers.get_hl(), result);
            }
        }
    }

    // DEC instruction
    fn dec(&mut self, target: IncDecTarget) {
        match target {
            IncDecTarget::BC => { self.registers.set_bc(self.registers.get_bc() - 1); },
            IncDecTarget::DE => { self.registers.set_de(self.registers.get_de() - 1); },
            IncDecTarget::HL => { self.registers.set_hl(self.registers.get_hl() - 1); },
            IncDecTarget::SP => { self.sp -= 1; },
            IncDecTarget::A => {
                let result = self.registers.a.wrapping_sub(1);

                // note: carry flag not affected
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.a & 0xf) < (1 & 0xf);

                self.registers.a = result;
            },
            IncDecTarget::B => {
                let result = self.registers.b.wrapping_sub(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.b & 0xf) < (1 & 0xf);

                self.registers.b = result;
            },
            IncDecTarget::C => {
                let result = self.registers.c.wrapping_sub(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.c & 0xf) < (1 & 0xf);

                self.registers.c = result;
            },
            IncDecTarget::D => {
                let result = self.registers.d.wrapping_sub(1);

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.d & 0xf) < (1 & 0xf);

                self.registers.d = result;
            },
            IncDecTarget::E => {
                let result = self.registers.e.wrapping_sub(1);

                // note: carry flag not affected
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.e & 0xf) < (1 & 0xf);

                self.registers.e = result;
            },
            IncDecTarget::H => {
                let result = self.registers.h.wrapping_sub(1);

                // note: carry flag not affected
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.h & 0xf) < (1 & 0xf);

                self.registers.h = result;
            },
            IncDecTarget::L => {
                let result = self.registers.l.wrapping_sub(1);

                // note: carry flag not affected
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.l & 0xf) < (1 & 0xf);

                self.registers.l = result;
            },
            IncDecTarget::HLI => {
                let result = self.bus.read_byte(self.registers.get_hl()).wrapping_sub(1);

                // note: carry flag not affected
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.bus.read_byte(self.registers.get_hl()) & 0xf) < (1 & 0xf);

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
            ArithTarget::A => { self.registers.a },
            ArithTarget::B => { self.registers.b },
            ArithTarget::C => { self.registers.c },
            ArithTarget::D => { self.registers.d },
            ArithTarget::E => { self.registers.e },
            ArithTarget::H => { self.registers.h },
            ArithTarget::L => { self.registers.l }
            ArithTarget::D8 => { self.read_next_byte() },
            ArithTarget::HLI => { self.bus.read_byte(self.registers.get_hl()) },
        }
    }

    // get register value from prefix target
    fn get_register_from_prefix(&self, target: PrefixTarget) -> u8 {
        match target {
            PrefixTarget::A => { self.registers.a },
            PrefixTarget::B => { self.registers.b },
            PrefixTarget::C => { self.registers.c },
            PrefixTarget::D => { self.registers.d },
            PrefixTarget::E => { self.registers.e },
            PrefixTarget::H => { self.registers.h },
            PrefixTarget::L => { self.registers.l },
            PrefixTarget::HLI => { self.bus.read_byte(self.registers.get_hl()) }
        }
    }

    fn set_register_from_prefix(&mut self, target: PrefixTarget, value: u8) {
        match target {
            PrefixTarget::A => { self.registers.a = value; },
            PrefixTarget::B => { self.registers.b = value; },
            PrefixTarget::C => { self.registers.c = value; },
            PrefixTarget::D => { self.registers.d = value; },
            PrefixTarget::E => { self.registers.e = value; },
            PrefixTarget::H => { self.registers.h = value; },
            PrefixTarget::L => { self.registers.l = value; },
            PrefixTarget::HLI => { self.bus.set_byte(self.registers.get_hl(), value); }
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
        self.registers.f.subtract = true;

        // set carry flag if there was a borrow
        self.registers.f.carry = did_underflow;

        // set the half_carry flag if there was a borrow from bit 4
        self.registers.f.half_carry = (self.registers.a & 0xf) < (value & 0xf);

        // return the result of the subtraction
        result
    }
}
