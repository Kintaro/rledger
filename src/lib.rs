#![feature(zero_one)]
#![feature(iter_arith)]
pub mod amount;
pub mod decimal;
pub mod commodity;
pub mod posting;
pub mod price;
pub mod quantity;
pub mod transaction;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
