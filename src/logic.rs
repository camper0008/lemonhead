use std::ops::{AddAssign, Mul, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Unit(i32);

impl Unit {
    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn from_milliunits(value: i32) -> Self {
        Self(value)
    }

    pub fn from_units(value: i32) -> Self {
        Self(value * 1000)
    }

    pub fn from_units_decimal(value: f64) -> Self {
        Self((value * 1000.0) as i32)
    }
}

impl Unit {
    pub fn abs(&self) -> Unit {
        Unit(self.0.abs())
    }
}

impl AddAssign for Unit {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Unit(self.0.mul(rhs.0))
    }
}

impl Sub for Unit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Unit(self.0.sub(rhs.0))
    }
}
