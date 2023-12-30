use std::cmp;

use crate::expression::{Expression, UndefinedError};
use crate::traits::Simplify;
use crate::types::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational(i32, i32);

impl Simplify for Rational {
    fn simplify(self) -> Result<Expression, UndefinedError> {
        let gcd = self.gcd();
        match (self.0, self.1) {
            (_, 0)                         => Err(UndefinedError("Indeterminate form: k/0".to_string())),
            (n, d) if n % d == 0 => Ok(int!(n / d)),
            (n, d) if d < 0      => Ok(frac!(-n / gcd, -d / gcd)),
            (n, d)               => Ok(frac!(n / gcd, d / gcd)),
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
    pub fn new(num: i32, den: i32) -> Rational {
        Rational(num, den)
    }

    pub fn gcd(&self) -> i32 {
        let mut a = self.num();
        let mut b = self.den();

        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs()
    }

    pub fn num(&self) -> i32 {
        self.0
    }

    pub fn den(&self) -> i32 {
        self.1
    }
}

impl From<f32> for Rational {
    fn from(value: f32) -> Self {
        const EPSILON: f32 = 1e-10;
        let mut den = 1.0;

        while (value * den) % 1.0 > EPSILON {
            den *= 10.0;
        }

        let num = (value * den).round() as i32;
        let den = den as i32;
        Rational::new(num, den)
    }
}