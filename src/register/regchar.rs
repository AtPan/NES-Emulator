use std::ops::{BitXor, Add, BitAnd, Shr, Sub, SubAssign, AddAssign, BitAndAssign, BitOr, BitOrAssign, Shl};

#[derive(Eq, Copy, Clone)]
pub struct RegisterChar(pub u8);

impl RegisterChar {
    pub fn as_ptr(&mut self) -> &mut u8 {
        &mut self.0
    }
}

impl Add<Self> for RegisterChar {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        RegisterChar(self.0 + rhs.0)
    }
}

impl AddAssign<Self> for RegisterChar {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add<u8> for RegisterChar {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        RegisterChar(self.0 + rhs)
    }
}

impl AddAssign<u8> for RegisterChar {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl BitOr<Self> for RegisterChar {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        RegisterChar(self.0 | rhs.0)
    }
}

impl BitOrAssign for RegisterChar {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitOr<u8> for RegisterChar {
    type Output = Self;
    fn bitor(self, rhs: u8) -> Self::Output {
        RegisterChar(self.0 | rhs)
    }
}

impl BitXor<u8> for RegisterChar {
    type Output = Self;
    fn bitxor(self, rhs: u8) -> Self::Output {
        RegisterChar(self.0 ^ rhs)
    }
}

impl BitAnd<Self> for RegisterChar {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        RegisterChar(self.0 & rhs.0)
    }
}

impl BitAnd<u8> for RegisterChar {
    type Output = Self;
    fn bitand(self, rhs: u8) -> Self::Output {
        RegisterChar(self.0 & rhs)
    }
}

impl BitAndAssign<Self> for RegisterChar {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitAndAssign<u8> for RegisterChar {
    fn bitand_assign(&mut self, rhs: u8) {
        *self = *self & rhs;
    }
}

impl Shl<Self> for RegisterChar {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        RegisterChar(self.0 << rhs.0)
    }
}

impl Shl<i32> for RegisterChar {
    type Output = Self;
    fn shl(self, rhs: i32) -> Self::Output {
        RegisterChar(self.0 << rhs)
    }
}

impl Shr<Self> for RegisterChar {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        RegisterChar(self.0 >> rhs.0)
    }
}

impl Shr<i32> for RegisterChar {
    type Output = Self;
    fn shr(self, rhs: i32) -> Self::Output {
        RegisterChar(self.0 >> rhs)
    }
}

impl Sub<Self> for RegisterChar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        RegisterChar(self.0 - rhs.0)
    }
}

impl SubAssign<Self> for RegisterChar {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub<u8> for RegisterChar {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        RegisterChar(self.0 - rhs)
    }
}

impl SubAssign<u8> for RegisterChar {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl PartialEq<Self> for RegisterChar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for RegisterChar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd<Self> for RegisterChar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<u8> for RegisterChar {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for RegisterChar {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

impl From<RegisterChar> for usize {
    fn from(reg: RegisterChar) -> Self {
        reg.0 as usize
    }
}

/*
impl From<RegisterChar> for u16 {
    fn from(reg: RegisterChar) -> Self {
        reg.0 as u16
    }
}
*/

impl From<RegisterChar> for u8 {
    fn from(reg: RegisterChar) -> Self {
        reg.0 as u8
    }
}

impl Add<RegisterChar> for u8 {
    type Output = Self;
    fn add(self, rhs: RegisterChar) -> Self::Output {
        self + rhs.0
    }
}
