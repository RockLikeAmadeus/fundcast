//use currencies::currency::*;

pub enum Currency {
    NONE,
    USD,
}

pub struct Money {
    value: i64
}

impl Money {
    // Constructors
    pub fn new(total_value: i64) -> Money {
        let res = Money {
            value: total_value
        };
        res
    }

    pub fn from_str(_amount: &str) -> Money {
        let res = Money {
            value: 0
        };
        res
    }

    // Methods
    pub fn value_as_i64(&self) -> i64 {
        0
    }

    pub fn major_part(&self) -> i64 {
        0
    }

    pub fn minor_part(&self) -> i8 {
        0
    }

    pub fn as_string(&self) -> &'static str {
        ""
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /* Test object construction*/

    #[test]
    fn constructing_dollar_amount_via_integer_yields_correct_object() {
        let cash: Money = Money::new(251);
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

    // Test addition
    /*#[test]
    fn adding_currency_values_gives_expected_result() {
        let two_dollars_fifty_one = Dollars::new(2, 51);
        let five_dollars_thirty_three = Dollars::new(5, 33);
        // Probably use the *currencies* rust crate.
        let sum = two_dollars_fifty_one + five_dollars_thirty_three;
        assert_eq(sum, Dollars::new(7, 84));
    }*/
    // Test subtraction
    // Test multiplication with integers
    // Test division with integers
    // Test comparison operators 
    // Test string formatting

    
}