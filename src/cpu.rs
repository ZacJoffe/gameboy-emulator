use crate::instructions::*;
use crate::memory_bus::MemoryBus;
use crate::registers::Registers;

struct CPU {
    pc: u16,
    sp: u16,
    registers: Registers,
    bus: MemoryBus,
    interrupts: bool,
    is_halted: bool,
}

impl CPU {
    fn fetch_execute(&mut self) {
        let mut instr_byte = self.bus.read_byte(self.pc);
        let is_prefixed = instr_byte == 0xcb;

        if is_prefixed {
            // instr_byte = self.bus.read_byte(self.pc + 1);
            instr_byte = self.read_next_byte();
        }

        let next_pc = if let Some(instr) = Instruction::disassemble(instr_byte, is_prefixed) {
            self.execute(instr)
        } else {
            panic!(
                "Unknown instruction! 0x{}{:x}",
                if is_prefixed { "cb" } else { "" },
                instr_byte
            );
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instr: Instruction) -> u16 {
        match instr {
            Instruction::ADD(target) => {
                self.add(target);

                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::ADC(target) => {
                // do an ADD, then add the carry
                self.add(target);
                self.add_a(self.registers.f.carry as u8);

                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::ADDHL(target) => {
                let value = match target {
                    AddHLTarget::BC => self.registers.get_bc(),
                    AddHLTarget::DE => self.registers.get_de(),
                    AddHLTarget::HL => self.registers.get_hl(),
                    AddHLTarget::SP => self.sp,
                };
                let result = self.add_hl(value);
                self.registers.set_hl(result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDSP => {
                let value = ((self.read_next_byte() as i8) as i16) as u16;
                let result = self.sp.wrapping_add(value);

                self.registers.f.set(Some(false), Some(false), Some((self.sp * 0xf) + (value & 0xf) > 0xf), Some((self.sp & 0xff) + (value & 0xff) > 0xff));
                self.pc.wrapping_add(2)
            }
            Instruction::SUB(target) => {
                self.sub(target);
                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::SBC(target) => {
                // do a SUB, then add the carry
                self.sub(target);
                self.sub_a(self.registers.f.carry as u8);
                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::AND(target) => {
                self.and(target);
                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::OR(target) => {
                self.or(target);
                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::XOR(target) => {
                self.xor(target);
                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::CP(target) => {
                self.cp(target);
                match target {
                    ArithTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::INC(target) => {
                self.inc(target);
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                self.dec(target);
                self.pc.wrapping_add(1)
            }
            Instruction::CCF => {
                /*
                // reset subtract and half_carry flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                // toggle carry flag
                self.registers.f.carry = if self.registers.f.carry { false } else { true };
                */

                self.registers.f.set(
                    None,
                    Some(false),
                    Some(false),
                    Some(!self.registers.f.carry),
                );
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                /*
                // reset subtract and half_carry flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                // set carry flag
                self.registers.f.carry = true;
                */

                self.registers
                    .f
                    .set(None, Some(false), Some(false), Some(true));
                self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                // get LSB of register a
                let new_carry = self.registers.a & 0x1;

                // rotate right through carry
                self.registers.a >>= 1;
                self.registers.a |= if self.registers.f.carry {
                    0x1 << 7
                } else {
                    0x0
                };

                /*
                // set carry flag to the LSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */
                self.registers
                    .f
                    .set(Some(new_carry != 0), Some(false), Some(false), Some(false));
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                // get MSB of register a
                let new_carry = (self.registers.a & 0x80) >> 7;

                // rotate left through carry
                self.registers.a <<= 1;
                self.registers.a |= if self.registers.f.carry { 0x1 } else { 0x0 };

                /*
                // set carry flag to the MSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers
                    .f
                    .set(Some(new_carry != 0), Some(false), Some(false), Some(false));
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                // get LSB of register a
                let new_carry = self.registers.a & 0x1;

                // rotate right
                self.registers.a = self.registers.a.rotate_right(1);

                /*
                // set carry flag to the LSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers
                    .f
                    .set(Some(new_carry != 0), Some(false), Some(false), Some(false));
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                // get MSB of register a
                let new_carry = (self.registers.a & 0x80) >> 7;

                // rotate left
                self.registers.a = self.registers.a.rotate_left(1);

                /*
                // set carry flag to the MSB of register a before rotate
                self.registers.f.carry = new_carry != 0;

                // reset other flags
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers
                    .f
                    .set(Some(new_carry != 0), Some(false), Some(false), Some(false));
                self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                // flip all bits of data in register a
                self.registers.a = !self.registers.a;

                /*
                // set subtract and half_carry flags, don't touch the others
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
                */

                self.registers.f.set(None, Some(true), Some(true), None);
                self.pc.wrapping_add(1)
            }
            Instruction::BIT(pos, target) => {
                // get shift value in variable so we don't move twice
                let shift_value = u8::from(pos);

                // mask it with 1 shifted left to the correct position,
                // and shift it back to get the value of that bit
                let bit =
                    (self.get_register_from_prefix(target) & (0x1 << shift_value)) >> shift_value;

                /*
                // set zero if flag if the bit is 0
                self.registers.f.zero = bit == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                */

                self.registers
                    .f
                    .set(Some(bit == 0), Some(false), Some(true), None);
                self.pc.wrapping_add(2)
            }
            Instruction::SET(pos, target) => {
                // shift 0x1 to the required bit position for the or operation
                let bit_set = 0x1 << u8::from(pos);

                match target {
                    PrefixTarget::A => self.registers.a |= bit_set,
                    PrefixTarget::B => self.registers.b |= bit_set,
                    PrefixTarget::C => self.registers.c |= bit_set,
                    PrefixTarget::D => self.registers.d |= bit_set,
                    PrefixTarget::E => self.registers.e |= bit_set,
                    PrefixTarget::H => self.registers.h |= bit_set,
                    PrefixTarget::L => self.registers.l |= bit_set,
                    PrefixTarget::HLI => self.bus.set_byte(
                        self.registers.get_hl(),
                        self.bus.read_byte(self.registers.get_hl()) | bit_set,
                    ),
                }
                self.pc.wrapping_add(2)
            }
            Instruction::RES(pos, target) => {
                // rotate 0xfe
                let base: u8 = 0b1111_1110;
                let bit_mask = base.rotate_left(u8::from(pos) as u32);

                match target {
                    PrefixTarget::A => self.registers.a &= bit_mask,
                    PrefixTarget::B => self.registers.b &= bit_mask,
                    PrefixTarget::C => self.registers.c &= bit_mask,
                    PrefixTarget::D => self.registers.d &= bit_mask,
                    PrefixTarget::E => self.registers.e &= bit_mask,
                    PrefixTarget::H => self.registers.h &= bit_mask,
                    PrefixTarget::L => self.registers.l &= bit_mask,
                    PrefixTarget::HLI => self.bus.set_byte(
                        self.registers.get_hl(),
                        self.bus.read_byte(self.registers.get_hl()) & bit_mask,
                    ),
                }
                self.pc.wrapping_add(2)
            }
            Instruction::SRL(target) => {
                // note that this opcode does a logical shift right,
                // meaning that the MSB is discarded in the shift
                //
                // by default, rust will do a logical shift with u8,
                // so we do not need to do anything special
                let value = self.get_register_from_prefix(target);

                // shift right
                let result = value >> 1;

                /*
                // put LSB of register before shift into carry flag
                self.registers.f.carry = (value & 0x1) != 0;

                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some((value & 0x1) != 0),
                );

                self.set_register_from_prefix(target, value);
                self.pc.wrapping_add(2)
            }
            Instruction::RR(target) => {
                let value = self.get_register_from_prefix(target);

                // get LSB of target
                let new_carry = value & 0x1;

                // shift value right and set the MSB to the value of the carry flag
                let result = (value >> 1)
                    | if self.registers.f.carry {
                        0x1 << 7
                    } else {
                        0x0
                    };

                /*
                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some(new_carry != 0),
                );

                self.set_register_from_prefix(target, value);
                self.pc.wrapping_add(2)
            }
            Instruction::RL(target) => {
                let value = self.get_register_from_prefix(target);

                // get MSB of target
                let new_carry = (value & 0x80) >> 7;

                // shift value right and set the MSB to the value of the carry flag
                let result = (value << 1) | if self.registers.f.carry { 0x1 } else { 0x0 };

                /*
                // set flags
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some(new_carry != 0),
                );

                self.set_register_from_prefix(target, value);
                self.pc.wrapping_add(2)
            }
            Instruction::RRC(target) => {
                let value = self.get_register_from_prefix(target);

                // get LSB of target
                let new_carry = value & 0x1;

                // rotate the value right
                let result = value.rotate_right(1);

                /*
                // set flags
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some(new_carry != 0),
                );

                // set the flag to the new value
                self.set_register_from_prefix(target, result);
                self.pc.wrapping_add(2)
            }
            Instruction::RLC(target) => {
                let value = self.get_register_from_prefix(target);

                // get MSB of target
                let new_carry = (value & 0x80) >> 7;

                // rotate the value left
                let result = value.rotate_left(1);

                /*
                // set flags
                self.registers.f.zero = result == 0;
                self.registers.f.carry = new_carry != 0;

                // reset flags
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some(new_carry != 0),
                );

                // set to value rotated right
                self.set_register_from_prefix(target, result);
                self.pc.wrapping_add(2)
            }
            Instruction::SRA(target) => {
                // note this instruction needs to do an arithmetic shift
                // thus, we need to preserve the MSB
                //
                // in rust, shifting a u8 is automatically logical
                let value = self.get_register_from_prefix(target);

                // get the MSB of value
                let msb = (value & 0x80) >> 7;

                // if the MSB is 1, then shift right and or it with 0x80 to set the new MSB
                // otherwise, just shift right and introduce the 0 normally
                let result = if msb != 0 {
                    (value >> 1) | 0x80
                } else {
                    value >> 1
                };

                /*
                // put LSB of register before shift into carry flag
                self.registers.f.carry = (value & 0x1) != 0;

                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some((value & 0x1) != 0),
                );

                self.set_register_from_prefix(target, result);
                self.pc.wrapping_add(2)
            }
            Instruction::SLA(target) => {
                let value = self.get_register_from_prefix(target);

                // shift left
                let result = value << 1;

                /*
                // put MSB of register before shift into carry flag
                self.registers.f.carry = (value & 0x80) >> 7 != 0;

                // set flags accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some(false),
                    Some((value & 0x80) >> 7 != 0),
                );

                self.set_register_from_prefix(target, result);
                self.pc.wrapping_add(2)
            }
            Instruction::SWAP(target) => {
                let value = self.get_register_from_prefix(target);

                // get upper and lower nibbles of the value
                let upper = (value & 0xf0) >> 4;
                let lower = value & 0xf;

                // combine the lower and upper nibbles to perform the swap
                let result = (lower << 4) | upper;

                /*
                // set registers accordingly
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.carry = false;
                self.registers.f.half_carry = false;
                */

                self.registers
                    .f
                    .set(Some(result == 0), Some(false), Some(false), Some(false));

                self.set_register_from_prefix(target, result);
                self.pc.wrapping_add(2)
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Unconditional => true,
                };

                // return 3 or the address jumped to
                self.jp(jump_condition)
            }
            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Unconditional => true,
                };

                // return 3 or the address jumped to
                self.jr(jump_condition)
            }
            Instruction::JPHLI => self.bus.read_byte(self.registers.get_hl()) as u16,
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        // load the value form memory into the target
                        self.set_register_from_load_byte(
                            target,
                            self.get_register_from_load_byte(source),
                        );

                        // if the source is a d8, we need to add 2 to the pc
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::Word(target) => {
                        /*
                        // little endian
                        let upper_byte = self.bus.read_byte(self.pc + 2) as u16;
                        let lower_byte = self.read_next_byte() as u16;
                        let value = (upper_byte << 8) | lower_byte;
                        */
                        let value = self.read_next_word();

                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(value),
                            LoadWordTarget::DE => self.registers.set_de(value),
                            LoadWordTarget::HL => self.registers.set_hl(value),
                            LoadWordTarget::SP => self.sp = value,
                        };

                        // add 3 to the pc
                        self.pc.wrapping_add(3)
                    }
                    LoadType::AFromIndirect(target) => {
                        self.registers.a = match target {
                            LoadIndirectTarget::BCI => self.bus.read_byte(self.registers.get_bc()),
                            LoadIndirectTarget::DEI => self.bus.read_byte(self.registers.get_de()),
                            LoadIndirectTarget::HLIPLUS => {
                                // increment hl, then get the byte at address hl
                                self.registers
                                    .set_hl(self.registers.get_hl().wrapping_add(1));
                                self.bus.read_byte(self.registers.get_hl())
                            }
                            LoadIndirectTarget::HLIMINUS => {
                                // decrement hl, then get the byte at address hl
                                self.registers
                                    .set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.bus.read_byte(self.registers.get_hl())
                            }
                            LoadIndirectTarget::WORDI => self.bus.read_byte(self.read_next_word()),
                            LoadIndirectTarget::CI => {
                                self.bus.read_byte(0xff00 + (self.registers.c as u16))
                            }
                        };

                        // only the (word) load instruction adds 3 to the pc (to skip the word)
                        // the rest will just go to the next byte
                        match target {
                            LoadIndirectTarget::WORDI => self.pc.wrapping_add(3),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::IndirectFromA(target) => {
                        match target {
                            LoadIndirectTarget::BCI => {
                                self.bus.set_byte(self.registers.get_bc(), self.registers.a)
                            }
                            LoadIndirectTarget::DEI => {
                                self.bus.set_byte(self.registers.get_de(), self.registers.a)
                            }
                            LoadIndirectTarget::HLIPLUS => {
                                // increment hl, then get the byte at address hl
                                self.registers
                                    .set_hl(self.registers.get_hl().wrapping_add(1));
                                self.bus.set_byte(self.registers.get_hl(), self.registers.a);
                            }
                            LoadIndirectTarget::HLIMINUS => {
                                // decrement hl, then get the byte at address hl
                                self.registers
                                    .set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.bus.set_byte(self.registers.get_hl(), self.registers.a);
                            }
                            LoadIndirectTarget::WORDI => {
                                self.bus.set_byte(self.read_next_word(), self.registers.a)
                            }
                            LoadIndirectTarget::CI => self
                                .bus
                                .set_byte(0xff00 + (self.registers.c as u16), self.registers.a),
                        };

                        // only the (word) load instruction adds 3 to the pc (to skip the word)
                        // the rest will just go to the next byte
                        match target {
                            LoadIndirectTarget::WORDI => self.pc.wrapping_add(3),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::AFromA8 => {
                        // set register a to a value located at in the last byte of memory
                        self.registers.a =
                            self.bus.read_byte(0xff00 + (self.read_next_byte() as u16));
                        self.pc.wrapping_add(2)
                    }
                    LoadType::A8FromA => {
                        // store the value of register a into somewhere in the last byte of memory
                        self.bus
                            .set_byte(0xff00 + (self.read_next_byte() as u16), self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    LoadType::HLFromSP => {
                        let value = ((self.read_next_byte() as i8) as i16) as u16;
                        let result = self.sp.wrapping_add(value);

                        self.registers.f.set(Some(false), Some(false), Some((self.sp * 0xf) + (value & 0xf) > 0xf), Some((self.sp & 0xff) + (value & 0xff) > 0xff));
                        self.registers.set_hl(result);
                        self.pc.wrapping_add(2)
                    }
                    LoadType::SPFromHL => {
                        self.sp = self.registers.get_hl();
                        self.pc.wrapping_add(1)
                    }
                    LoadType::IndirectFromSP => {
                        self.bus.set_byte(self.read_next_word(), (self.sp & 0xff) as u8);
                        self.bus.set_byte(self.read_next_word() + 1, ((self.sp & 0xff00) >> 8) as u8);
                        self.pc.wrapping_add(3)
                    }
                }
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::AF => self.registers.get_af(),
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                };

                /*
                self.sp = self.sp.wrapping_sub(1);
                self.bus.set_byte(self.sp, ((value & 0xff00) >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.bus.set_byte(self.sp, (value & 0x00ff) as u8);
                */
                // push value onto stack
                self.push(value);

                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let result = self.pop();

                // set the registers to the value
                match target {
                    StackTarget::AF => self.registers.set_af(result),
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                };

                self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Unconditional => true,
                };

                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Unconditional => true,
                };

                self.ret(jump_condition)
            }
            Instruction::RST(target) => {
                self.push(self.pc.wrapping_add(1));

                // return one of these addresses
                match target {
                    RstTarget::X00 => 0x00,
                    RstTarget::X08 => 0x08,
                    RstTarget::X10 => 0x10,
                    RstTarget::X18 => 0x18,
                    RstTarget::X20 => 0x20,
                    RstTarget::X28 => 0x28,
                    RstTarget::X30 => 0x30,
                    RstTarget::X38 => 0x38,
                }
            }
            Instruction::RETI => {
                self.interrupts = true;
                self.pop()
            }
            Instruction::NOP => 1,
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            Instruction::DAA => {
                let value = self.registers.a;
                let mut carry = false;

                let result = if !self.registers.f.subtract {
                    let mut result = value;
                    if self.registers.f.carry || value > 0x99 {
                        carry = true;
                        result = result.wrapping_add(0x60);
                    }

                    if self.registers.f.half_carry || value & 0xf > 0x9 {
                        result = result.wrapping_add(0x6);
                    }

                    result
                } else if self.registers.f.carry {
                    carry = true;
                    if self.registers.f.half_carry {
                        value.wrapping_add(0x9a)
                    } else {
                        value.wrapping_add(0xa0)
                    }
                } else {
                    value
                };

                self.registers.f.set(Some(result == 0), None, Some(false), Some(carry));
                self.pc.wrapping_add(1)
            }
            Instruction::STOP => {
                panic!("STOP instruction!")
            }
            Instruction::DI => {
                self.interrupts = false;
                self.pc.wrapping_add(1)
            }
            Instruction::EI => {
                self.interrupts = true;
                self.pc.wrapping_add(1)
            }
        }
    }

    // reads the next byte in memory
    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    // reads the next word (16 bit number) in memory
    fn read_next_word(&self) -> u16 {
        // the gameboy's cpu is little endian
        // thus, the byte order for a word is stored in memory from least significant to most significant byte
        //
        // i.e. next_word = memory[pc + 2]memory[pc + 1]
        let lower_byte = self.read_next_byte() as u16;
        let upper_byte = self.bus.read_byte(self.pc + 2) as u16;

        // return the formed word
        (upper_byte << 8) | lower_byte
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
            IncDecTarget::BC => {
                self.registers.set_bc(self.registers.get_bc() + 1);
            }
            IncDecTarget::DE => {
                self.registers.set_de(self.registers.get_de() + 1);
            }
            IncDecTarget::HL => {
                self.registers.set_hl(self.registers.get_hl() + 1);
            }
            IncDecTarget::SP => {
                self.sp += 1;
            }
            IncDecTarget::A => {
                // let (result, _) = self.registers.a.overflowing_add(1);
                let result = self.registers.a.wrapping_add(1);

                // note: carry flag not affected
                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.a & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.registers.a & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.registers.a = result;
            }
            IncDecTarget::B => {
                let result = self.registers.b.wrapping_add(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.b & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.registers.b & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.registers.b = result;
            }
            IncDecTarget::C => {
                let result = self.registers.c.wrapping_add(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.c & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.registers.c & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.registers.c = result;
            }
            IncDecTarget::D => {
                let result = self.registers.d.wrapping_add(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.d & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.registers.d & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.registers.d = result;
            }
            IncDecTarget::E => {
                let result = self.registers.e.wrapping_add(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.e & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.registers.e & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.registers.e = result;
            }
            IncDecTarget::H => {
                let result = self.registers.h.wrapping_add(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.h & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.registers.h & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.registers.h = result;
            }
            IncDecTarget::L => {
                let result = self.registers.l.wrapping_add(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.l & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.registers.l & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.registers.l = result;
            }
            IncDecTarget::HLI => {
                // let result = self.bus.read_byte(self.registers.get_hl()) + 1;
                let result = self.bus.read_byte(self.registers.get_hl()).wrapping_add(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.bus.read_byte(self.registers.get_hl()) & 0xf) + (1 & 0xf) > 0xf;
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(false),
                    Some((self.bus.read_byte(self.registers.get_hl()) & 0xf) + (1 & 0xf) > 0xf),
                    None,
                );

                self.bus.set_byte(self.registers.get_hl(), result);
            }
        }
    }

    // DEC instruction
    fn dec(&mut self, target: IncDecTarget) {
        match target {
            IncDecTarget::BC => {
                self.registers.set_bc(self.registers.get_bc() - 1);
            }
            IncDecTarget::DE => {
                self.registers.set_de(self.registers.get_de() - 1);
            }
            IncDecTarget::HL => {
                self.registers.set_hl(self.registers.get_hl() - 1);
            }
            IncDecTarget::SP => {
                self.sp -= 1;
            }
            IncDecTarget::A => {
                let result = self.registers.a.wrapping_sub(1);

                // note: carry flag not affected
                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.a & 0xf) < (1 & 0xf);
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.registers.a & 0xf) < (1 & 0xf)),
                    None,
                );

                self.registers.a = result;
            }
            IncDecTarget::B => {
                let result = self.registers.b.wrapping_sub(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.b & 0xf) < (1 & 0xf);
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.registers.b & 0xf) < (1 & 0xf)),
                    None,
                );

                self.registers.b = result;
            }
            IncDecTarget::C => {
                let result = self.registers.c.wrapping_sub(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.c & 0xf) < (1 & 0xf);
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.registers.c & 0xf) < (1 & 0xf)),
                    None,
                );

                self.registers.c = result;
            }
            IncDecTarget::D => {
                let result = self.registers.d.wrapping_sub(1);

                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.d & 0xf) < (1 & 0xf);
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.registers.d & 0xf) < (1 & 0xf)),
                    None,
                );

                self.registers.d = result;
            }
            IncDecTarget::E => {
                let result = self.registers.e.wrapping_sub(1);

                // note: carry flag not affected
                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.e & 0xf) < (1 & 0xf);
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.registers.e & 0xf) < (1 & 0xf)),
                    None,
                );

                self.registers.e = result;
            }
            IncDecTarget::H => {
                let result = self.registers.h.wrapping_sub(1);

                // note: carry flag not affected
                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.h & 0xf) < (1 & 0xf);
                */

                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.registers.h & 0xf) < (1 & 0xf)),
                    None,
                );

                self.registers.h = result;
            }
            IncDecTarget::L => {
                let result = self.registers.l.wrapping_sub(1);

                // note: carry flag not affected
                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.registers.l & 0xf) < (1 & 0xf);
                */
                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.registers.l & 0xf) < (1 & 0xf)),
                    None,
                );

                self.registers.l = result;
            }
            IncDecTarget::HLI => {
                let result = self.bus.read_byte(self.registers.get_hl()).wrapping_sub(1);

                // note: carry flag not affected
                /*
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (self.bus.read_byte(self.registers.get_hl()) & 0xf) < (1 & 0xf);
                */
                self.registers.f.set(
                    Some(result == 0),
                    Some(true),
                    Some((self.bus.read_byte(self.registers.get_hl()) & 0xf) < (1 & 0xf)),
                    None,
                );

                self.bus.set_byte(self.registers.get_hl(), result);
            }
        }
    }

    // AND instruction
    fn and(&mut self, target: ArithTarget) {
        // set a to itself anded with the value of the target register
        self.registers.a &= self.get_register_from_arith(target);

        /*
        // set zero flag if the result of the and is equal to 0
        self.registers.f.zero = self.registers.a == 0;

        // set subtract flag to false as this is an and operation
        self.registers.f.subtract = false;

        // reset carry flag
        self.registers.f.carry = false;

        // set half_carry flag
        self.registers.f.half_carry = true;
        */
        self.registers.f.set(
            Some(self.registers.a == 0),
            Some(false),
            Some(true),
            Some(false),
        );
    }

    // OR instruction
    fn or(&mut self, target: ArithTarget) {
        // set a to itself ored with the value of the target register
        self.registers.a |= self.get_register_from_arith(target);

        /*
        // set zero flag if the result of the and is equal to 0
        self.registers.f.zero = self.registers.a == 0;

        // set subtract flag to false as this is an and operation
        self.registers.f.subtract = false;

        // reset carry flag
        self.registers.f.carry = false;

        // reset half_carry flag
        self.registers.f.half_carry = false;
        */
        self.registers.f.set(
            Some(self.registers.a == 0),
            Some(false),
            Some(false),
            Some(false),
        );
    }

    // XOR instruction
    fn xor(&mut self, target: ArithTarget) {
        // set a to itself xored with the value of the target register
        self.registers.a ^= self.get_register_from_arith(target);

        /*
        // set zero flag if the result of the and is equal to 0
        self.registers.f.zero = self.registers.a == 0;

        // set subtract flag to false as this is an and operation
        self.registers.f.subtract = false;

        // reset carry flag
        self.registers.f.carry = false;

        // reset half_carry flag
        self.registers.f.half_carry = false;
        */
        self.registers.f.set(
            Some(self.registers.a == 0),
            Some(false),
            Some(false),
            Some(false),
        );
    }

    // JP instruction
    fn jp(&self, jump: bool) -> u16 {
        if jump {
            /*
            // little endian
            let upper_byte = self.bus.read_byte(self.pc + 2) as u16;
            // let lower_byte = self.bus.read_byte(self.pc + 1) as u16;
            let lower_byte = self.read_next_byte() as u16;

            (upper_byte << 8) | lower_byte
            */
            // return the next word in memory as it's the new pc's address
            self.read_next_word()
        } else {
            // add 3
            self.pc.wrapping_add(3)
        }
    }

    // JR instruction
    fn jr(&self, jump: bool) -> u16 {
        let mut next_pc = self.pc.wrapping_add(2);

        if jump {
            // the next byte is interpreted as a signed integer
            //
            // if it's positive, then do a wrapping add
            // else, wrapping sub the magnitude of the offset
            // let offset = self.bus.read_byte(self.pc + 1) as i8;
            let offset = self.read_next_byte() as i8;
            next_pc = if offset >= 0 {
                next_pc.wrapping_add(offset as u16)
            } else {
                next_pc.wrapping_sub(offset.abs() as u16)
            };

            next_pc
        } else {
            // add 2
            next_pc
        }
    }

    // CALL instruction
    fn call(&mut self, jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);

        if jump {
            // push the address of the next instruction (i.e. the next pc value)
            // onto the stack, so that we can pop into the pc when RET is called
            self.push(next_pc);

            // the next byte of memory is the address of the start of the subroutine,
            // so return that
            self.read_next_word()
        } else {
            // return the next_pc
            next_pc
        }
    }

    // RET instruction
    fn ret(&mut self, jump: bool) -> u16 {
        if jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    // get register value from arith target
    fn get_register_from_arith(&self, target: ArithTarget) -> u8 {
        match target {
            ArithTarget::A => self.registers.a,
            ArithTarget::B => self.registers.b,
            ArithTarget::C => self.registers.c,
            ArithTarget::D => self.registers.d,
            ArithTarget::E => self.registers.e,
            ArithTarget::H => self.registers.h,
            ArithTarget::L => self.registers.l,
            ArithTarget::D8 => self.read_next_byte(),
            ArithTarget::HLI => self.bus.read_byte(self.registers.get_hl()),
        }
    }

    // get register value from prefix target
    fn get_register_from_prefix(&self, target: PrefixTarget) -> u8 {
        match target {
            PrefixTarget::A => self.registers.a,
            PrefixTarget::B => self.registers.b,
            PrefixTarget::C => self.registers.c,
            PrefixTarget::D => self.registers.d,
            PrefixTarget::E => self.registers.e,
            PrefixTarget::H => self.registers.h,
            PrefixTarget::L => self.registers.l,
            PrefixTarget::HLI => self.bus.read_byte(self.registers.get_hl()),
        }
    }

    // set a register from a prefix target to a value
    fn set_register_from_prefix(&mut self, target: PrefixTarget, value: u8) {
        match target {
            PrefixTarget::A => {
                self.registers.a = value;
            }
            PrefixTarget::B => {
                self.registers.b = value;
            }
            PrefixTarget::C => {
                self.registers.c = value;
            }
            PrefixTarget::D => {
                self.registers.d = value;
            }
            PrefixTarget::E => {
                self.registers.e = value;
            }
            PrefixTarget::H => {
                self.registers.h = value;
            }
            PrefixTarget::L => {
                self.registers.l = value;
            }
            PrefixTarget::HLI => {
                self.bus.set_byte(self.registers.get_hl(), value);
            }
        }
    }

    // get a register from a load byte source
    fn get_register_from_load_byte(&self, source: LoadByteSource) -> u8 {
        match source {
            LoadByteSource::A => self.registers.a,
            LoadByteSource::B => self.registers.b,
            LoadByteSource::C => self.registers.c,
            LoadByteSource::D => self.registers.d,
            LoadByteSource::E => self.registers.e,
            LoadByteSource::H => self.registers.h,
            LoadByteSource::L => self.registers.l,
            LoadByteSource::D8 => self.read_next_byte(),
            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
        }
    }

    // set a register from a load byte target to a value
    fn set_register_from_load_byte(&mut self, target: LoadByteTarget, value: u8) {
        match target {
            LoadByteTarget::A => self.registers.a = value,
            LoadByteTarget::B => self.registers.b = value,
            LoadByteTarget::C => self.registers.c = value,
            LoadByteTarget::D => self.registers.d = value,
            LoadByteTarget::E => self.registers.e = value,
            LoadByteTarget::H => self.registers.h = value,
            LoadByteTarget::L => self.registers.l = value,
            LoadByteTarget::HLI => self.bus.set_byte(self.registers.get_hl(), value),
        }
    }

    // add to register a and set flags accordingly
    fn add_a(&mut self, value: u8) -> u8 {
        let (result, did_overflow) = self.registers.a.overflowing_add(value);

        /*
        // set zero flag if the result is equal to 0
        self.registers.f.zero = result == 0;

        // set subtract flag to false as this operation is an addition
        self.registers.f.subtract = false;

        // set carry flag if there was an overflow
        self.registers.f.carry = did_overflow;

        // set the half_carry flag if there was a carry to the upper nibble of a
        self.registers.f.half_carry = (self.registers.a & 0xf) + (value & 0xf) > 0xf;
        */
        self.registers.f.set(
            Some(result == 0),
            Some(false),
            Some((self.registers.a & 0xf) + (value & 0xf) > 0xf),
            Some(did_overflow),
        );

        // return the result of the addition
        result
    }

    // add to registers hl and set flags accordingly
    fn add_hl(&mut self, value: u16) -> u16 {
        let (result, did_overflow) = self.registers.get_hl().overflowing_add(value);

        /*
        // zero flag not set

        // set subtract flag to false as this operation is an addition
        self.registers.f.subtract = false;

        // set carry flag if there was an overflow
        self.registers.f.carry = did_overflow;

        // set the half_carry flag if there was a carry to the upper nibble of hl
        self.registers.f.half_carry = (self.registers.get_hl() & 0xfff) + (value & 0xfff) > 0xfff;
        */

        self.registers.f.set(
            None,
            Some(false),
            Some((self.registers.get_hl() & 0xfff) + (value & 0xfff) > 0xfff),
            Some(did_overflow),
        );

        // return the result of the addition
        result
    }

    // sub from a, set flags accordingly
    fn sub_a(&mut self, value: u8) -> u8 {
        let (result, did_underflow) = self.registers.a.overflowing_sub(value);

        /*
        // set zero flag if the result is equal to 0
        self.registers.f.zero = result == 0;

        // set subtract flag to true as this operation is a subtraction
        self.registers.f.subtract = true;

        // set carry flag if there was a borrow
        self.registers.f.carry = did_underflow;

        // set the half_carry flag if there was a borrow from bit 4
        self.registers.f.half_carry = (self.registers.a & 0xf) < (value & 0xf);
        */
        self.registers.f.set(
            Some(result == 0),
            Some(true),
            Some((self.registers.a & 0xf) < (value & 0xf)),
            Some(did_underflow),
        );

        // return the result of the subtraction
        result
    }

    // push the value onto the stack (i.e. push word)
    //
    // the stack is full descending, so it grows "down" in memory
    // thus, we decrease the stack pointer when we push
    //
    // also, store using little endian
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.set_byte(self.sp, ((value & 0xff00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.set_byte(self.sp, (value & 0x00ff) as u8);
    }

    // pop word from stack
    //
    // the stack is full descending, so it grows "down" in memory
    // thus, we increase the stack pointer when we pop
    //
    // we account for endianness when we form the value
    fn pop(&mut self) -> u16 {
        let lower_byte = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let upper_byte = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (upper_byte << 8) | lower_byte
    }
}
