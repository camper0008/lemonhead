use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Unit(i32);

impl Unit {
    pub const fn milliunits(&self) -> i32 {
        self.0
    }

    pub const fn decimal(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    pub const fn new(value: i32) -> Self {
        Self(value * 1000)
    }

    pub const fn new_decimal(value: f64) -> Self {
        Self((value * 1000.0) as i32)
    }
}

impl From<i32> for Unit {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<f64> for Unit {
    fn from(value: f64) -> Self {
        Self::new_decimal(value)
    }
}

impl Unit {
    pub const fn abs(&self) -> Unit {
        Unit(self.0.abs())
    }
}

impl AddAssign for Unit {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add for Unit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Unit(self.0.add(rhs.0))
    }
}

impl Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.decimal() * rhs.decimal()).into()
    }
}

impl Sub for Unit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Unit(self.0.sub(rhs.0))
    }
}
