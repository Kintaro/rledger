use std::ops::Div;
use std::ops::Mul;
use std::cmp::max;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decimal(pub isize, pub usize);

impl Decimal {
    pub fn new() -> Decimal {
        Decimal(0, 0)
    }

    fn shift_to_precision(self, precision: usize) -> Decimal {
        if precision < self.1 {
            self
        } else {
            let val = 10usize.pow(precision as u32) as isize;
            Decimal(val * self.0, precision)
        }
    }
}

impl Div for Decimal {
    type Output = Decimal;

    fn div(self, rhs: Decimal) -> Decimal {
        let p = max(self.1, rhs.1);
        let a = self.shift_to_precision(p);
        let b = rhs.shift_to_precision(p);
        Decimal(a.0 / b.0, p)
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Decimal) -> Decimal {
        let p = max(self.1, rhs.1);
        let a = self.shift_to_precision(p);
        let b = rhs.shift_to_precision(p);
        Decimal(a.0 * b.0, p)
    }
}
