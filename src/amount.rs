use std::ops::Add;
use std::ops::Div;
use std::num::Zero;
use std::rc::Rc;
use commodity::Commodity;
use decimal::Decimal;
use quantity::Quantity;
use price::Price;
use std::iter::FromIterator;

#[derive(Clone, PartialEq, Eq)]
enum Side {
    Left,
    Right
}

#[derive(Clone, PartialEq, Eq)]
struct DigitGroupStyle(char, Vec<usize>);

#[derive(Clone, PartialEq, Eq)]
pub struct AmountStyle {
    commodity_side: Side,
    commodity_spaced: bool,
    precision: usize,
    decimal_point: Option<char>,
    digit_groups: Option<DigitGroupStyle>
}

impl AmountStyle {
    pub fn new() -> AmountStyle {
        AmountStyle {
            commodity_side: Side::Left,
            commodity_spaced: false,
            precision: 0,
            decimal_point: Option::None,
            digit_groups: Option::None
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Amount {
    commodity: Commodity,
    quantity: Quantity,
    price: Rc<Price>,
    style: AmountStyle
}

#[derive(Clone, PartialEq, Eq)]
pub struct MixedAmount(Vec<Amount>);

impl Amount {
    pub fn new() -> Amount {
        Amount {
            commodity: Commodity::new(String::from("")),
            quantity: Quantity(Decimal::new(0, 0)),
            price: Rc::new(Price::None),
            style: AmountStyle::new()
        }
    }

    pub fn cost(&self) -> Amount {
        let q = self.clone().quantity;
        let res = Rc::try_unwrap(self.price.clone());
        match res.ok().unwrap() {
            Price::None => self.clone(),
            Price::UnitPrice(a) => {
                let mut r = a;
                r.quantity = r.quantity * q;
                r
            },
            Price::TotalPrice(a) => {
                let mut r = a;
                r.quantity = r.quantity * q;
                r
            }
        }
    }

    pub fn is_negative(&self) -> bool {
        let z = Decimal::new(0, 0);
        self.quantity.0 < z
    }

    pub fn apply_op<F>(op: F, a: Amount, b: Amount) -> Amount where F : Fn(Quantity, Quantity) -> Quantity {
        Amount {
            commodity: b.commodity,
            style: AmountStyle::new(),
            price: Rc::new(Price::None),
            quantity: op(a.quantity, b.quantity)
        }
    }

    pub fn with_commodity(&self, commodity: Commodity) -> Amount {
        Amount {
            commodity: commodity,
            style: self.style.clone(),
            quantity: self.quantity.clone(),
            price: Rc::new(Price::None)
        }
    }
}

impl Div<Quantity> for Amount {
    type Output = Amount;
    fn div(self, Quantity(d): Quantity) -> Amount {
        let Quantity(q) = self.clone().quantity;
        let mut r = self.clone();
        r.quantity = Quantity(q / d);
        r
    }
}

impl MixedAmount {
    pub fn filter<F>(self, f: F) -> MixedAmount where F : Fn(&Amount) -> bool {
        MixedAmount(self.0.iter().filter(|&x| f(x)).map(|x| x.clone()).collect())
    }

    pub fn filter_by_commodity(&self, commodity: Commodity) -> MixedAmount {
        let r = match &self.0.iter().filter(|x| x.commodity == commodity).map(|x| x.clone()).collect::<Vec<Amount>>() {
            x if x == &vec!() => vec!(Amount::new()),
            _ => vec!()
        };
        MixedAmount(r)
    }

    pub fn cost(&self) -> MixedAmount {
        MixedAmount(self.0.iter().map(|x| x.cost()).collect())
    }
}

impl Zero for MixedAmount {
    fn zero() -> Self {
        MixedAmount(vec!())
    }
}

impl Add for MixedAmount {
    type Output = MixedAmount;

    fn add(self, rhs: MixedAmount) -> MixedAmount {
        MixedAmount(self.0.iter().chain(rhs.0.iter()).map(|x| x.clone()).collect())
    }
}

impl FromIterator<Amount> for MixedAmount {
    fn from_iter<I: IntoIterator<Item=Amount>>(iterable: I) -> Self {
        MixedAmount(Vec::from_iter(iterable))
    }
}
