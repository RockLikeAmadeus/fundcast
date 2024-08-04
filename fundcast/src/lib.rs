// Module declarations
mod currency;


pub struct Wallet {
    accounts: Vec<Account>,
}

pub struct Account {
    name: String,
    amount: String, // this needs to be an instance of a wrapper for currency
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
