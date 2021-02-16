#![feature(map_first_last)]
pub mod mon;
pub mod order;
pub mod poly;
pub mod ring;
pub mod var;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
