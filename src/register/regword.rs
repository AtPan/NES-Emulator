use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct RegisterWord(pub u16);

impl RegisterWord {
    pub fn as_ptr(&mut self) -> &mut u16 {
        &mut self.0
    }
}

impl Add<i8> for RegisterWord {
    type Output = Self;
    fn add(self, rhs: i8) -> Self::Output {
        let mut val = self.0 + ((rhs & 0x7f) as u16);
        if rhs < 0 {
            val -= 128;
        }
        RegisterWord(val)
    }
}

impl AddAssign<i8> for RegisterWord {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Add<Self> for RegisterWord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        RegisterWord(self.0 + rhs.0)
    }
}

impl Add<u16> for RegisterWord {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        RegisterWord(self.0 + (rhs as u16))
    }
}

impl AddAssign<Self> for RegisterWord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<u16> for RegisterWord {
    fn add_assign(&mut self, rhs: u16) {
        *self = *self + rhs;
    }
}

impl Sub<Self> for RegisterWord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        RegisterWord(self.0 - rhs.0)
    }
}

impl Sub<u16> for RegisterWord {
    type Output = Self;
    fn sub(self, rhs: u16) -> Self::Output {
        RegisterWord(self.0 - rhs)
    }
}

impl SubAssign for RegisterWord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<RegisterWord> for usize {
    fn from(reg: RegisterWord) -> Self {
        reg.0 as usize
    }
}

impl From<RegisterWord> for u16 {
    fn from(reg: RegisterWord) -> Self {
        reg.0 as u16
    }
}
