use std::{fmt, ops::{BitOr, BitXorAssign, BitXor, BitOrAssign}};

pub enum Status {
    Negative,
    Overflow,
    Unused,
    Break,
    Decimal,
    InterruptDisable,
    Zero,
    Carry,
    Mix(u8)
}

impl BitXorAssign<Self> for Status {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Status::Mix(self.status_bit() ^ rhs.status_bit());
    }
}

impl BitXor<Self> for Status {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
       Status::Mix(self.status_bit() ^ rhs.status_bit())
    }
}

impl BitOrAssign<Self> for Status {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Status::Mix(self.status_bit() | rhs.status_bit());
    }
}

impl BitOr<Self> for Status {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Status::Mix(self.status_bit() | rhs.status_bit())
    }
}

impl Status {
    #[inline(always)]
    fn status_bit(&self) -> u8 {
        match self {
            Status::Negative => 1 << 7,
            Status::Overflow => 1 << 6,
            Status::Unused => 0,
            Status::Break => 1 << 4,
            Status::Decimal => 1 << 3,
            Status::InterruptDisable => 1 << 2,
            Status::Zero => 1 << 1,
            Status::Carry => 1,
            Status::Mix(val) => *val,
        }
    }
}

#[derive(Copy, Clone)]
pub struct StatusRegister(pub u8);

impl From<StatusRegister> for u8 {
    fn from(reg: StatusRegister) -> u8 {
        reg.0
    }
}

impl StatusRegister {
    #[inline(always)]
    pub fn contains(&self, flag: Status) -> bool {
        (self.0 & flag.status_bit()) != 0
    }

    #[inline(always)]
    pub fn flag_value(&self, flag: Status) -> u8 {
        self.0 & flag.status_bit()
    }

    #[inline(always)]
    pub fn toggle_flag(&mut self, flag: Status) {
        self.0 ^= flag.status_bit();
    }

    #[inline(always)]
    pub fn set_flag(&mut self, flag: Status) {
        self.0 |= flag.status_bit();
    }

    #[inline(always)]
    pub fn clear_flag(&mut self, flag: Status) {
        self.0 &= !flag.status_bit();
    }
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub sr: StatusRegister,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            sr: StatusRegister(0b00100000),
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A: {:#04x} -- X: {:#04x}\nY: {:#04x} -- SR: {:#04x}\nPC: {:#04x} -- SP: {:#04x}",
            self.a, self.x, self.y, self.sr.0, self.pc, self.sp)
    }
}
