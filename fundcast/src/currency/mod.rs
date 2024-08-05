enum Currency {
    USD,
}

struct Money {
    currency: Currency,
    major_value: i32,
    minor_value: i32,
}

impl Money {
    fn new(major_value: i32, minor_value: i32, currency: Currency) -> Money {
        let res = Money {
            currency: currency,
            major_value: major_value,
            minor_value: minor_value,
        };
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn adding_currency_values_gives_expected_result() {
        let two_dollars_fifty_one = Dollars::new(2, 51);
        let five_dollars_thirty_three = Dollars::new(5, 33);
        // Probably use the *currencies* rust crate.
        let sum = two_dollars_fifty_one + five_dollars_thirty_three;
        assert_eq(sum, Dollars::new(7, 84));
    }*/

    // Test object construction

    // #[test]
    // fn constructing_currency_via_string_yields_correct_object() {
    //     let two_dollars_fifty_one = Money::new("$2.51")
    //     assert_eq!(two_dollars_fifty_one.major_value, 2);
    //     assert_eq!(two_dollars_fifty_one.minor_value, 51);
    // }

    #[test]
    fn constructing_currency_via_integers_yields_correct_object() {
        let two_dollars_fifty_one = Money::new(2, 51, Currency::USD);
        assert_eq!(two_dollars_fifty_one.major_value, 2);
        assert_eq!(two_dollars_fifty_one.minor_value, 51);
    }

    // Test addition
    // Test subtraction
    // Test multiplication with integers
    // Test division with integers
    // Test comparison operators 
    // Test string formatting

    
}