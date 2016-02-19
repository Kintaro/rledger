use std::ops::Div;
use std::rc::Rc;
use commodity::Commodity;
use decimal::Decimal;
use quantity::Quantity;
use price::Price;

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

struct MixedAmount(Vec<Amount>);

impl Amount {
    pub fn new() -> Amount {
        Amount {
            commodity: Commodity::new(String::from("")),
            quantity: Quantity(Decimal::new()),
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
        self.quantity.0 < Decimal::new()
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
}
