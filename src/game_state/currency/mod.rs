use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Currency {
    amount: i128,
}

pub const COPPER_PER_SILVER: i128 = 100;
pub const SILVER_PER_GOLD: i128 = 100;

pub const COPPER_PER_GOLD: i128 = COPPER_PER_SILVER * SILVER_PER_GOLD;

#[allow(dead_code)]
impl Currency {
    pub const fn from_copper(copper: i128) -> Self {
        Self { amount: copper }
    }

    pub const fn from_silver(silver: i128) -> Self {
        Self {
            amount: silver * COPPER_PER_SILVER,
        }
    }

    pub const fn from_gold(gold: i128) -> Self {
        Self {
            amount: gold * COPPER_PER_GOLD,
        }
    }

    pub fn from_copper_f64(copper: f64) -> Self {
        Self {
            amount: copper.round() as i128,
        }
    }

    pub const fn zero() -> Self {
        Self { amount: 0 }
    }

    pub const fn copper(&self) -> i128 {
        self.amount
    }

    pub const fn silver(&self) -> i128 {
        self.amount / COPPER_PER_SILVER
    }

    pub const fn gold(&self) -> i128 {
        self.amount / COPPER_PER_GOLD
    }

    pub const fn copper_of_silver(&self) -> i8 {
        (self.copper() % COPPER_PER_SILVER) as i8
    }

    pub const fn silver_of_gold(&self) -> i8 {
        (self.silver() % SILVER_PER_GOLD) as i8
    }

    pub const fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
        }
    }
}

impl ops::Add for Currency {
    type Output = Currency;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount + rhs.amount,
        }
    }
}

impl ops::AddAssign for Currency {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Currency {
    type Output = Currency;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount - rhs.amount,
        }
    }
}

impl ops::SubAssign for Currency {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Neg for Currency {
    type Output = Currency;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
        }
    }
}
