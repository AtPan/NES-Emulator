use std::fmt;

use crate::opcode::{Instruction, MemAddressMode, AugmentOpcodeInfo, OPCODES};
use crate::{register::*, cpu};
use crate::memory::{Memory, Endian};
use crate::opcode::*;

pub enum ProcState {
    Idle,
    Execute,
    MemoryRead,
    MemoryWrite,
}

pub struct Processor {
    pub registers: Registers,
    pub memory: Memory,
    pub state: ProcState,
}

impl fmt::Display for ProcState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ProcState::Idle => "Idle",
            ProcState::Execute => "Executing",
            ProcState::MemoryRead => "Reading RAM",
            ProcState::MemoryWrite => "Writing RAM",
        })
    }
}

impl fmt::Display for Processor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Processor:\n{}\n{}", self.registers, self.state)
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: Registers::new(),
            /* Maybe switch to 0x800 limit, and add secondary memory storage.
             * Current setup could be susceptible to memory fooling and emulation detection
             */
            memory: Memory::new(0x10000, Endian::Little),
            state: ProcState::Idle,
        }
    }

    pub fn execute_instruction(&mut self, inst: Instruction) {
        let op = inst.0;

        match op {
            Opcode::ADC => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.add_with_carry(val);
                }
            },
            Opcode::AND => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.and(val);
                }
            },
            Opcode::ASL => {
                let target = match inst.1 {
                    AugmentOpcodeInfo::Address(addr) | AugmentOpcodeInfo::MemoryByte(addr) =>
                        self.memory.as_mut(addr as usize),

                    _ => &mut self.registers.a.0,
                };
                cpu::shift_left(&mut self.registers.sr, target);
            },
            Opcode::BCC => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if !self.registers.sr.contains(Status::Carry) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::BCS => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if self.registers.sr.contains(Status::Carry) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::BEQ => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if self.registers.sr.contains(Status::Zero) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::BIT => {
                if let Some(addr) = cpu::decode_augmented_addr(inst.1) {
                    self.bit(self.memory.read_byte(addr as usize));
                }
            },
            Opcode::BMI => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if self.registers.sr.contains(Status::Negative) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::BNE => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if !self.registers.sr.contains(Status::Zero) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::BPL => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if !self.registers.sr.contains(Status::Negative) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::BRK => {
                self.registers.sr.set_flag(Status::InterruptDisable);
                self.memory.write_word(u8::from(self.registers.sp) as usize, u16::from(self.registers.pc) + 2);
                self.memory.write_byte(u8::from(self.registers.sp + 2) as usize, u8::from(self.registers.sr));
                self.registers.sp += 3;
            },
            Opcode::BVC => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if !self.registers.sr.contains(Status::Overflow) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::BVS => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    if self.registers.sr.contains(Status::Overflow) { cpu::branch(&mut self.registers.pc, val); }
                }
            },
            Opcode::CLC => self.registers.sr.clear_flag(Status::Carry),
            Opcode::CLD => self.registers.sr.clear_flag(Status::Decimal),
            Opcode::CLI => self.registers.sr.clear_flag(Status::InterruptDisable),
            Opcode::CLV => self.registers.sr.clear_flag(Status::Overflow),
            Opcode::CMP => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.compare_with_accumulator(val);
                }
            },
            Opcode::CPX => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.compare_with_x(val);
                }
            },
            Opcode::CPY => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.compare_with_y(val);
                }
            },
            Opcode::DEC => {
                if let AugmentOpcodeInfo::Address(addr) = inst.1 {
                    let target = self.memory.as_mut(addr as usize);
                    cpu::load_u8_memory(&mut self.registers.sr, target, *target - 1);
                }
            },
            Opcode::DEX => self.decrement_x(),
            Opcode::DEY => self.decrement_y(),
            Opcode::EOR => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.xor_accumulator(val);
                }
            },
            Opcode::INC => {
                if let AugmentOpcodeInfo::Address(addr) = inst.1 {
                    let target = self.memory.as_mut(addr as usize);
                    cpu::load_u8_memory(&mut self.registers.sr, target, *target - 1);
                }
            },
            Opcode::INX => self.increment_x(),
            Opcode::INY => self.increment_y(),
            Opcode::JMP => {
                if let AugmentOpcodeInfo::Address(addr) = inst.1 {
                    self.jump(addr);
                }
            },
            Opcode::JSR => {
                if let AugmentOpcodeInfo::Address(addr) = inst.1 {
                    self.jump_save_return(addr);
                }
            },
            Opcode::LDA => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, val);
                }
            },
            Opcode::LDX => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.x, val);
                }
            },
            Opcode::LDY => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.y, val);
                }
            },
            Opcode::LSR => {
                let target = match inst.1 {
                    AugmentOpcodeInfo::Address(addr) => self.memory.as_mut(addr as usize),
                    _ => &mut self.registers.a.0,
                };
                cpu::shift_right(&mut self.registers.sr, target);
            },
            Opcode::NOP => {},
            Opcode::ORA => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.or(val);
                }
            },
            Opcode::PHA => self.push_accumulator(),
            Opcode::PHP => self.push_status(),
            Opcode::PLA => self.pop_accumulator(),
            Opcode::PLP => self.pop_status(),
            Opcode::ROL => {
                let target = match inst.1 {
                    AugmentOpcodeInfo::Address(addr) => self.memory.as_mut(addr as usize),
                    _ => &mut self.registers.a.0,
                };
                let val = *target >> 7;

                cpu::shift_left(&mut self.registers.sr, target);
                *target |= val;
            },
            Opcode::ROR => {
                let target = match inst.1 {
                    AugmentOpcodeInfo::Address(addr) => self.memory.as_mut(addr as usize),
                    _ => &mut self.registers.a.0,
                };
                let val = *target << 7;

                cpu::shift_right(&mut self.registers.sr, target);
                *target |= val;
            },
            Opcode::RTI => {
                self.registers.sr = StatusRegister(self.memory.read_byte(u8::from(self.registers.sp) as usize));
                self.registers.pc = RegisterWord(self.memory.read_word(u8::from(self.registers.sp + 1) as usize));
                self.registers.sp += 3;
            },
            Opcode::RTS => {
                self.registers.pc = RegisterWord(self.memory.read_word(u8::from(self.registers.sp) as usize));
                self.registers.sp += 2;
            },
            Opcode::SBC => {
                if let Some(val) = cpu::decode_augmented_u8(&self.memory, inst.1) {
                    self.subtract_with_carry(val);
                }
            },
            Opcode::SEC => self.registers.sr.set_flag(Status::Carry),
            Opcode::SED => self.registers.sr.set_flag(Status::Decimal),
            Opcode::SEI => self.registers.sr.set_flag(Status::InterruptDisable),
            Opcode::STA => {
                if let Some(addr) = cpu::decode_augmented_addr(inst.1) {
                    self.memory.write_byte(addr as usize, u8::from(self.registers.a));
                }
            },
            Opcode::STX => {
                if let Some(addr) = cpu::decode_augmented_addr(inst.1) {
                    self.memory.write_byte(addr as usize, u8::from(self.registers.x));
                }
            },
            Opcode::STY => {
                if let Some(addr) = cpu::decode_augmented_addr(inst.1) {
                    self.memory.write_byte(addr as usize, u8::from(self.registers.y));
                }
            },
            Opcode::TAX => self.registers.x = self.registers.a,
            Opcode::TAY => self.registers.y = self.registers.a,
            Opcode::TSX => cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.x, u8::from(self.registers.sp)),
            Opcode::TXA => self.registers.a = self.registers.x,
            Opcode::TXS => self.registers.sr = StatusRegister(u8::from(self.registers.x)),
            Opcode::TYA => self.registers.a = self.registers.y,
        };
    }

    pub fn read_next_instruction(&mut self) -> Option<Instruction> {
        let op = self.memory.read_byte(usize::from(self.registers.pc));
        self.registers.pc += 1_u16;

        match OPCODES[op as usize] {
            Some((code, mode)) => {
                let aug_info = match mode {
                    MemAddressMode::Implied => AugmentOpcodeInfo::Implied,
                    MemAddressMode::Accumulator => AugmentOpcodeInfo::Accumulator,
                    MemAddressMode::Relative |
                    MemAddressMode::Immediate => {
                        self.registers.pc += 1_u16;
                        AugmentOpcodeInfo::Immediate(self.memory.read_byte(usize::from(self.registers.pc - 1)))
                    },
                    MemAddressMode::Absolute => {
                        self.registers.pc += 2_u16;
                        AugmentOpcodeInfo::Address(self.memory.read_word(usize::from(self.registers.pc - 2)))
                    },
                    MemAddressMode::ZeroPage => {
                        self.registers.pc += 1_u16;
                        AugmentOpcodeInfo::Address(self.memory.read_byte(usize::from(self.registers.pc - 1)) as u16)
                    },
                    MemAddressMode::Indirect => {
                        self.registers.pc += 2_u16;
                        AugmentOpcodeInfo::MemoryWord(self.memory.read_word(usize::from(self.registers.pc - 2)))
                    },
                    MemAddressMode::AbsoluteIndexedX => {
                        self.registers.pc += 2_u16;
                        AugmentOpcodeInfo::Address(self.memory.read_word(usize::from(self.registers.pc - 2))
                            + u8::from(self.registers.x) as u16)
                    },
                    MemAddressMode::AbsoluteIndexedY => {
                        self.registers.pc += 2_u16;
                        AugmentOpcodeInfo::Address(self.memory.read_word(usize::from(self.registers.pc - 2))
                            + u8::from(self.registers.y) as u16)
                    },
                    MemAddressMode::ZeroPageIndexedX => {
                        self.registers.pc += 1_u16;
                        AugmentOpcodeInfo::Address(0x00ff &
                            (self.memory.read_byte(usize::from(self.registers.pc - 1)) +
                             self.registers.x) as u16)
                    },
                    MemAddressMode::ZeroPageIndexedY => {
                        self.registers.pc += 1_u16;
                        AugmentOpcodeInfo::Address(0x00ff &
                            (self.memory.read_byte(usize::from(self.registers.pc - 1)) + self.registers.y) as u16)
                    },
                    MemAddressMode::IndirectIndexedX => {
                        self.registers.pc += 1_u16;
                        AugmentOpcodeInfo::Address(self.memory.read_word(
                                (self.memory.read_byte(usize::from(self.registers.pc - 1)) +
                                self.registers.x) as usize))
                    },
                    MemAddressMode::IndirectIndexedY => {
                        self.registers.pc += 1_u16;
                        AugmentOpcodeInfo::Address(self.memory.read_word((
                                    self.memory.read_word(usize::from(self.registers.pc - 1))
                                    ) as usize)
                            + u8::from(self.registers.y) as u16)
                    },
                };
                Some((code, aug_info))
            },
            None => None,
        }
    }

    pub fn add_with_carry(&mut self, val: u8) {
        let mut set = Status::Mix(0);
        let mut clear = Status::Zero | Status::Negative | Status::Overflow;

        let accumulator = self.registers.a;
        let val = val + self.registers.sr.flag_value(Status::Carry);

        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, u8::from(accumulator) + val);

        if self.registers.sr.contains(Status::Decimal) {
            let mut nibble_low = self.registers.a & 0xf;
            let mut nibble_high = self.registers.a >> 4;

            if nibble_low > 9 {
                nibble_low -= 10;
                nibble_high += 1;
            }
            if nibble_high > 9 {
                nibble_high -= 10;
            }

            self.registers.a = (nibble_high << 4) | nibble_low;
        }

        /* Flag Checks */
        if self.registers.a < accumulator || self.registers.a < val {
            set |= Status::Carry;
            clear ^= Status::Carry;
        }

        if self.registers.a & 0x80 != ((accumulator & 0x80) | (val & 0x80)) {
            set |= Status::Overflow;
            clear ^= Status::Overflow;
        }

        self.registers.sr.set_flag(set);
        self.registers.sr.clear_flag(clear);
    }

    pub fn and(&mut self, val: u8) {
        let val = u8::from(self.registers.a) & val;

        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, val);
    }

    pub fn bit(&mut self, val: u8) {
        let mut set = Status::Mix(0);
        let mut clear = Status::Zero | Status::Overflow | Status::Negative;

        if val & 0x80 == 0x80 {
            set |= Status::Negative;
            clear ^= Status::Negative;
        }

        if val & 0x40 == 0x40 {
            set |= Status::Overflow;
            clear ^= Status::Overflow;
        }

        if self.registers.a & val == 0 {
            set |= Status::Zero;
            clear ^= Status::Zero;
        }

        self.registers.sr.clear_flag(clear);
        self.registers.sr.set_flag(set);
    }

    pub fn force_break(&mut self) {
        self.registers.sr.set_flag(Status::InterruptDisable);
        self.memory.write_word(usize::from(self.registers.sp - 2), u16::from(self.registers.pc) + 2);
        self.memory.write_byte(usize::from(self.registers.sp - 3), self.registers.sr.0);
    }

    pub fn compare_with_accumulator(&mut self, val: u8) {
        cpu::compare_u8(&mut self.registers.sr, &self.registers.a, val);
    }

    pub fn compare_with_x(&mut self, val: u8) {
        cpu::compare_u8(&mut self.registers.sr, &self.registers.x, val);
    }

    pub fn compare_with_y(&mut self, val: u8) {
        cpu::compare_u8(&mut self.registers.sr, &self.registers.y, val);
    }

    pub fn decrement(&mut self, val: &mut u8) {
        cpu::load_u8_memory(&mut self.registers.sr, val, *val - 1);

        if self.registers.sr.contains(Status::Decimal) {
            *val = cpu::format_bcd(*val);
        }
    }

    pub fn decrement_x(&mut self) {
        let val = u8::from(self.registers.x) - 1;
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.x, val);

        if self.registers.sr.contains(Status::Decimal) {
            self.registers.x = RegisterChar(cpu::format_bcd(u8::from(self.registers.x)));
        }
    }

    pub fn decrement_y(&mut self) {
        let val = u8::from(self.registers.y) - 1;
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.y, val);

        if self.registers.sr.contains(Status::Decimal) {
            self.registers.y = RegisterChar(cpu::format_bcd(u8::from(self.registers.y)));
        }
    }

    pub fn xor_accumulator(&mut self, val: u8) {
        let val = self.registers.a ^ val;
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, u8::from(val));
    }

    pub fn increment(&mut self, val: &mut u8) {
        cpu::load_u8_memory(&mut self.registers.sr, val, *val + 1);

        if self.registers.sr.contains(Status::Decimal) {
            *val = cpu::format_bcd(*val);
        }
    }

    pub fn increment_x(&mut self) {
        let val = self.registers.x + 1;
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.x, u8::from(val));

        if self.registers.sr.contains(Status::Decimal) {
            self.registers.x = RegisterChar(cpu::format_bcd(u8::from(self.registers.x)));
        }
    }

    pub fn increment_y(&mut self) {
        let val = self.registers.y + 1;
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.y, u8::from(val));

        if self.registers.sr.contains(Status::Decimal) {
            self.registers.y = RegisterChar(cpu::format_bcd(u8::from(self.registers.y)));
        }
    }

    pub fn jump(&mut self, addr: u16) {
        self.registers.pc = RegisterWord(addr);
    }

    pub fn jump_save_return(&mut self, addr: u16) {
        self.registers.sp -= 2;
        self.memory.write_word(usize::from(self.registers.sp), u16::from(self.registers.pc));
        self.registers.pc = RegisterWord(addr);
    }

    pub fn or(&mut self, val: u8) {
        let val = self.registers.a | val;
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, u8::from(val));
    }

    pub fn push_accumulator(&mut self) {
        self.registers.sp -= 1;
        self.memory.write_byte(usize::from(self.registers.sp), u8::from(self.registers.a));
    }

    pub fn push_status(&mut self) {
        self.registers.sp -= 1;
        self.registers.sr.set_flag(Status::Break | Status::Mix(1 << 5));
        self.memory.write_byte(usize::from(self.registers.sp), self.registers.sr.0);
    }

    pub fn pop_accumulator(&mut self) {
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a,
            self.memory.read_byte(usize::from(self.registers.sp))
        );
        self.registers.sp += 1;
    }

    pub fn pop_status(&mut self) {
        self.registers.sr = StatusRegister(self.memory.read_byte(usize::from(self.registers.sp)));
        self.registers.sp += 1;
        self.registers.sr.clear_flag(Status::Mix(1 << 5));
    }

    pub fn rotate_left(&mut self, val: &mut u8) {
        if *val & 0x80 == 0x80 {
            self.registers.sr.set_flag(Status::Carry);
        }
        else {
            self.registers.sr.clear_flag(Status::Carry);
        }

        let rot_val = (*val << 1) | (*val >> 7);
        cpu::load_u8_memory(&mut self.registers.sr, val, rot_val);
    }

    pub fn rotate_right(&mut self, val: &mut u8) {
        if *val & 1 == 1 {
            self.registers.sr.set_flag(Status::Carry);
        }
        else {
            self.registers.sr.clear_flag(Status::Carry);
        }

        let rot_val = (*val >> 1) | (*val << 7);
        cpu::load_u8_memory(&mut self.registers.sr, val, rot_val);
    }

    pub fn return_from_interrupt(&mut self) {
        self.registers.sr = StatusRegister(self.memory.read_byte(usize::from(self.registers.sp)));
        self.registers.pc = RegisterWord(self.memory.read_word(1 + usize::from(self.registers.sp)));
        self.registers.sp += 3;
    }

    pub fn return_from_subroutine(&mut self) {
        self.registers.pc = RegisterWord(self.memory.read_word(usize::from(self.registers.sp)));
        self.registers.sp += 2;
    }

    pub fn subtract_with_carry(&mut self, val: u8) {
        let mut set = Status::Mix(0);
        let mut clear = Status::Carry | Status::Overflow;

        let diff = self.registers.a - val - self.registers.sr.flag_value(Status::Carry);

        if self.registers.sr.contains(Status::Decimal) {
            let mut nibble_low = diff & 0xf;
            let mut nibble_high = diff >> 4;

            if nibble_low > 9 {
                nibble_low -= 10;
                nibble_high += 1;
            }
            if nibble_high > 9 {
                nibble_high -= 10;
            }

            self.registers.a = (nibble_high << 4) | nibble_low;
        }

        if diff < self.registers.a || diff < val {
            set |= Status::Carry;
            clear ^= Status::Carry;
        }
        if diff & 0x80 != ((self.registers.a & 0x80) | (val & 0x80)) {
            set |= Status::Overflow;
            clear ^= Status::Overflow;
        }

        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, u8::from(diff));

        self.registers.sr.set_flag(set);
        self.registers.sr.clear_flag(clear);
    }

    pub fn store_accumulator(&mut self, addr: u16) {
        self.memory.write_byte(addr as usize, u8::from(self.registers.a));
    }

    pub fn store_x(&mut self, addr: u16) {
        self.memory.write_byte(addr as usize, u8::from(self.registers.x));
    }

    pub fn store_y(&mut self, addr: u16) {
        self.memory.write_byte(addr as usize, u8::from(self.registers.y));
    }

    #[inline(always)]
    pub fn load_accumulator_in_x(&mut self) {
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.x, u8::from(self.registers.a));
    }

    #[inline(always)]
    pub fn load_accumulator_in_y(&mut self) {
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.y, u8::from(self.registers.a));
    }

    #[inline(always)]
    pub fn load_stackpointer_in_x(&mut self) {
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.x, u8::from(self.registers.sp));
    }

    #[inline(always)]
    pub fn load_x_in_accumulator(&mut self) {
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, u8::from(self.registers.x));
    }

    #[inline(always)]
    pub fn load_x_in_stackpointer(&mut self) {
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.sp, u8::from(self.registers.x));
    }

    #[inline(always)]
    pub fn load_y_in_accumulator(&mut self) {
        cpu::load_u8_register(&mut self.registers.sr, &mut self.registers.a, u8::from(self.registers.y));
    }
}
