#[derive(Clone, PartialEq, Eq)]
pub struct Commodity(String);

impl Commodity {
    pub fn new(commodity: String) -> Commodity {
        return Commodity(commodity);
    }

    pub fn conversion_rate(Commodity(a): Commodity, Commodity(b) : Commodity) -> f64 {
        return 1.0;
    }
}
