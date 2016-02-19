use std::ops::Mul;
use decimal::Decimal;

#[derive(Clone, PartialEq, Eq)]
pub struct Quantity(pub Decimal);

impl Mul for Quantity {
    type Output = Quantity;

    fn mul(self, Quantity(r): Quantity) -> Quantity {
        Quantity(self.0 * r)
    }
}
