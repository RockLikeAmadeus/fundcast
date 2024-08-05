//use currencies::currency::*;

pub enum Currency {
    NONE,
    USD,
}

pub trait Money {
    // constructors
    fn new(major_value: i32, minor_value: i32, currency: Currency) -> Self;
    fn from_str(_amount: &str) -> Self;

    // fields
    fn currency() -> Currency;
    fn major_value() -> i32;
    fn minor_value() -> i32;
    fn as_string() -> &'static str;
}

struct CurrencyWrapper {
    // amount: Amount;
}

impl CurrencyWrapper {
    fn new(major_value: i32, minor_value: i32, currency: Currency) -> CurrencyWrapper {
        let res = CurrencyWrapper {
            // currency: currency,
            // major_value: major_value,
            // minor_value: minor_value,
        };
        res
    }

    fn from_str(_amount: &str) -> CurrencyWrapper {
        let res = CurrencyWrapper {
            // currency: Currency::NONE,
            // major_value: 0,
            // minor_value: 0,
        };
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /* Test object construction*/

    #[test]
    fn constructing_dollar_amount_via_integers_yields_correct_object() {
        let cash: Money = CurrenciesWrapper::new(2, 51, Currency::USD);
        assert_eq!(cash.major_value, 2);
        assert_eq!(cash.minor_value, 51);
    }

    #[test]
    fn constructing_dollar_amount_via_string_yields_correct_object() {
        let cash = Money::from_str("$2.51");
        assert_eq!(cash.major_value, 2);
        assert_eq!(cash.minor_value, 51);
    }

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