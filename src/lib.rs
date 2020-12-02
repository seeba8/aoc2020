mod day1;
mod day2;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("Test 123, Test 456", std::fs::read_to_string("resources/day0.txt").unwrap());
    }
}
