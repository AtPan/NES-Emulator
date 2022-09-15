use std::cmp::Ordering;
use crate::register::*;
use crate::opcode::*;
use crate::memory::Memory;

pub fn branch(pc: &mut RegisterWord, offset: u8) {
    let offset: i8 = if offset & 0x80  == 0x80 { -128 } else { 0 } + ((offset & 0x7f) as i8);
    *pc += offset;
}

pub fn shift_right(status: &mut StatusRegister, target: &mut u8) {
    let mut set = Status::Mix(0);
    let mut clear = Status::Carry | Status::Zero | Status::Negative;

    if *target & 1 == 1 {
        set |= Status::Carry;
        clear ^= Status::Carry;
    }

    *target >>= 1;

    if *target == 0 {
        set |= Status::Zero;
        clear ^= Status::Zero;
    }

    status.set_flag(set);
    status.clear_flag(clear);
}

pub fn shift_left(status: &mut StatusRegister, target: &mut u8) {
    let mut set = Status::Mix(0);
    let mut clear = Status::Carry | Status::Zero | Status::Negative;

    if *target & 0x80 == 0x80 {
        set |= Status::Carry;
        clear ^= Status::Carry;
    }

    *target <<= 1;

    if *target & 0x80 == 0x80 {
        set |= Status::Negative;
        clear ^= Status::Negative;
    }
    else if *target == 0 {
        set |= Status::Zero;
        clear ^= Status::Zero;
    }

    status.set_flag(set);
    status.clear_flag(clear);
}

pub fn decode_augmented_addr(aug: AugmentOpcodeInfo) -> Option<u16> {
    match aug {
        AugmentOpcodeInfo::Address(addr) | AugmentOpcodeInfo::MemoryByte(addr) => Some(addr),

        _ => None,
    }
}

pub fn decode_augmented_u8(memory: &Memory, aug: AugmentOpcodeInfo) -> Option<u8> {
    match aug {
        AugmentOpcodeInfo::Immediate(val) => Some(val),
        AugmentOpcodeInfo::Address(addr) | AugmentOpcodeInfo::MemoryByte(addr) =>
            Some(memory.read_byte(addr as usize)),
        AugmentOpcodeInfo::MemoryWord(addr) =>
            Some(memory.read_byte(memory.read_word(addr as usize) as usize)),

        _ => None,
    }
}

pub fn format_bcd(val: u8) -> u8 {
    let mut nibble_low = val & 0xf;
    let mut nibble_high = val >> 4;

    if nibble_low > 9 {
        nibble_low -= 10;
        nibble_high += 1;
    }
    if nibble_high > 9 {
        nibble_high -= 10;
    }

    (nibble_high << 4) | nibble_low
}

pub fn compare_u8(status: &mut StatusRegister, reg: &RegisterChar, val: u8) {
    let mut set = Status::Mix(0);
    let mut clear = Status::Carry | Status::Zero | Status::Negative;

    match reg.cmp(&RegisterChar(val)) {
        Ordering::Less => {
            set |= Status::Negative;
            clear ^= Status::Negative;
        },
        Ordering::Equal => {
            set |= Status::Zero | Status::Carry;
            clear ^= Status::Zero | Status::Carry;
        },
        Ordering::Greater => {
            set |= Status::Zero;
            clear ^= Status::Zero;
        },
    };

    status.clear_flag(clear);
    status.set_flag(set);
}

pub fn load_u8_register(status: &mut StatusRegister, reg: &mut RegisterChar, val: u8) {
    load_u8_memory(status, &mut reg.0, val);
}

pub fn load_u8_memory(status: &mut StatusRegister, mem: &mut u8, val: u8) {
    let mut set = Status::Mix(0);
    let mut clear = Status::Negative | Status::Zero;

    *mem = val;

    if val & 0x80 == 0x80 {
        set |= Status::Negative;
        clear ^= Status::Negative;
    }
    else if val == 0 {
        set |= Status::Zero;
        clear ^= Status::Zero;
    }

    status.clear_flag(clear);
    status.set_flag(set);
}
