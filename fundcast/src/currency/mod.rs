#[cfg(test)]
mod tests {
    use super::*

    #[test]
    fn adding_currency_values_gives_expected_result() {
        let two_dollars_fifty_one = Dollars::new(2, 51);
        let five_dollars_thirty_three = Dollars::new(5, 33);
        let sum = two_dollars_fifty_one.plus(five_dollars_thirty_three);
        assert_eq(sum, Dollars::new(7, 84));
    }
}