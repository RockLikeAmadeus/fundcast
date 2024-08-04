#[cfg(test)]
mod tests {
    use super::*

    #[test]
    fn adding_currency_values_gives_expected_result() {
        let two_dollars_fifty_one = Dollars::new(2, 51);
        let five_dollars_thirty_three = Dollars::new(5, 33);
        // Probably use the *currencies* rust crate.
        let sum = two_dollars_fifty_one + five_dollars_thirty_three;
        assert_eq(sum, Dollars::new(7, 84));
    }
}