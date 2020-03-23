extern crate lib;

#[cfg(tests)]
mod tests {
    use lib;

    #[test]
    fn test1() {
        let list = List::new().push(1).push(2).push(3);
    }
}
