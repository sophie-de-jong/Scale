use std::cmp;

use crate::expression::Expression;
use crate::traits::Simplify;
use crate::types::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational(pub i64, pub i64);

impl Simplify for Rational {
    fn simplify(self) -> Option<Expression> {
        let gcd = self.gcd();
        match (self.0, self.1) {
            (_, 0)                         => None,
            (n, d) if n % d == 0 => Some(int!(n / d)),
            (n, d) if d < 0      => Some(frac!(-n / gcd, -d / gcd)),
            (n, d)               => Some(frac!(n / gcd, d / gcd)),
        }
    }
}

impl cmp::Ord for Rational {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.num() * other.den()).cmp(&(self.den() * other.num()))
    }
}

impl cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Rational {
    pub fn gcd(&self) -> i64 {
        let (mut a, mut b) = self.as_tuple();
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs()
    }

    pub fn num(&self) -> i64 {
        self.0
    }

    pub fn den(&self) -> i64 {
        self.1
    }

    pub fn as_tuple(&self) -> (i64, i64) {
        (self.0, self.1)
    }
}