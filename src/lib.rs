#[macro_use]
extern crate log as logger;

mod iothub;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
