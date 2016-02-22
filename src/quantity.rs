use std::ops::Mul;
use decimal::Decimal;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Quantity(pub Decimal);

impl Quantity {
    pub fn abs(self) -> Quantity {
        Quantity(self.0.abs())
    }
}

impl Mul for Quantity {
    type Output = Quantity;

    fn mul(self, Quantity(r): Quantity) -> Quantity {
        Quantity(self.0 * r)
    }
}
