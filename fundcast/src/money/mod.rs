use std::ops;

/*pub enum Currency {
    NONE,
    USD,
}*/

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Money {
    value: i64,
}

impl Money {
    // Constructors
    pub fn new(total_value: i64) -> Money {
        let res = Money { value: total_value };
        res
    }

    pub fn _from_str(_amount: &str) -> Money {
        let res = Money { value: 0 };
        res
    }

    // Methods
    pub fn value_as_i64(&self) -> i64 {
        self.value
    }

    pub fn major_part(&self) -> i64 {
        &self.value / 100
    }

    pub fn minor_part(&self) -> i8 {
        (&self.value % 100).try_into().unwrap()
    }

    pub fn _as_string(&self) -> &'static str {
        ""
    }
}

impl ops::Add<Money> for Money {
    type Output = Money;

    fn add(self, rhs: Money) -> Money {
        Money::new(self.value + rhs.value)
    }
}

impl ops::AddAssign<Money> for Money {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value + other.value,
        }
    }
}

impl ops::Sub<Money> for Money {
    type Output = Money;

    fn sub(self, rhs: Money) -> Money {
        Money::new(self.value - rhs.value)
    }
}

impl ops::SubAssign<Money> for Money {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value - other.value,
        }
    }
}

impl ops::Mul<i64> for Money {
    type Output = Money;

    fn mul(self, rhs: i64) -> Money {
        Money::new(self.value * rhs)
    }
}

impl ops::Div<i64> for Money {
    type Output = Money;

    fn div(self, rhs: i64) -> Money {
        Money::new(self.value / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Test object construction*/

    #[test]
    fn constructing_dollar_amount_via_integer_yields_correct_object() {
        let cash: Money = Money::new(251);
        assert_eq!(cash.value_as_i64(), 251);
        assert_eq!(cash.major_part(), 2);
        assert_eq!(cash.minor_part(), 51);
    }

    // #[test] // Implement later!!
    // fn constructing_dollar_amount_via_string_yields_correct_object() {
    //     let cash = Money::from_str("$2.51");
    //     assert_eq!(cash.major_value, 2);
    //     assert_eq!(cash.minor_value, 51);
    // }

    // Test cash! macro for simple construction

    /* Test addition */
    #[test]
    fn adding_currency_values_gives_expected_result() {
        let two_dollars_fifty_one = Money::new(251);
        let five_dollars_thirty_three = Money::new(533);
        let sum = two_dollars_fifty_one + five_dollars_thirty_three;
        assert_eq!(sum, Money::new(784));
    }

    #[test]
    fn add_assigning_currency_values_gives_expected_result() {
        let mut cash1 = Money::new(251);
        let cash2 = Money::new(533);
        cash1 += cash2;
        assert_eq!(cash1, Money::new(784));
    }

    /* Test subtraction */
    #[test]
    fn subtracting_currency_values_gives_expected_result() {
        let two_dollars_fifty_one = Money::new(251);
        let five_dollars_thirty_three = Money::new(533);
        let difference = five_dollars_thirty_three - two_dollars_fifty_one;
        assert_eq!(difference, Money::new(282));
    }

    #[test]
    fn sub_assigning_currency_values_gives_expected_result() {
        let mut cash1 = Money::new(251);
        let cash2 = Money::new(533);
        cash1 -= cash2;
        assert_eq!(cash1, Money::new(-282));
    }

    #[test]
    fn subtracting_currency_values_gives_expected_negative_result() {
        let two_dollars_fifty_one = Money::new(251);
        let five_dollars_thirty_three = Money::new(533);
        let difference = two_dollars_fifty_one - five_dollars_thirty_three;
        assert_eq!(difference, Money::new(-282));
    }

    /* Test multiplication with integers */
    #[test]
    fn multiplying_currency_by_integer_gives_expected_result() {
        let two_dollars_fifty_one = Money::new(251);
        let product = two_dollars_fifty_one * 99;
        assert_eq!(product, Money::new(24849));
    }

    /* Test division with integers */

    #[test]
    fn dividing_currency_by_integer_gives_expected_exact_result() {
        let two_dollars_fifty_two = Money::new(252);
        let result = two_dollars_fifty_two / 2;
        assert_eq!(result, Money::new(126));
    }

    #[test]
    fn dividing_currency_by_integer_truncates() {
        let two_dollars_fifty_two = Money::new(252);
        let result = two_dollars_fifty_two / 5;
        assert_eq!(result, Money::new(50));
    }

    /* Test comparison operators */
    /* Test string formatting */
}
