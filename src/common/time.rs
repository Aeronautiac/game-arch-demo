use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::common::Fixed;

pub type TimeValue = Fixed;
pub type TimeBase = u128; // ns

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    ns: TimeBase,
}

impl Time {
    pub const ZERO: Time = Time { ns: 0 };

    pub fn from_nanos(nanos: TimeBase) -> Self {
        Time { ns: nanos }
    }

    pub fn from_micro(micro: TimeValue) -> Self {
        Self::from_nanos((micro * Fixed::from_num(1_000)).to_num())
    }

    pub fn from_ms(ms: TimeValue) -> Self {
        Self::from_nanos((ms * Fixed::from_num(1_000_000)).to_num())
    }

    pub fn from_sec(sec: TimeValue) -> Self {
        Self::from_nanos((sec * Fixed::from_num(1_000_000_000)).to_num())
    }

    pub fn pure(&self) -> TimeBase {
        self.ns
    }

    pub fn nano(&self) -> TimeValue {
        Fixed::from_num(self.ns)
    }

    pub fn micro(&self) -> TimeValue {
        self.nano() / 1_000
    }

    pub fn ms(&self) -> TimeValue {
        self.nano() / 1_000_000
    }

    pub fn sec(&self) -> TimeValue {
        self.nano() / 1_000_000_000
    }
}

impl Add for Time {
    type Output = Time;
    fn add(self, rhs: Self) -> Self::Output {
        Time::from_nanos(self.ns + rhs.ns)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.ns += rhs.ns;
    }
}

impl Sub for Time {
    type Output = Time;
    fn sub(self, rhs: Self) -> Self::Output {
        Time::from_nanos(self.ns - rhs.ns)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.ns -= rhs.ns;
    }
}
