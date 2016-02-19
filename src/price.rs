use amount::Amount;

#[derive(Clone, PartialEq, Eq)]
pub enum Price {
    None,
    UnitPrice(Amount),
    TotalPrice(Amount)
}
