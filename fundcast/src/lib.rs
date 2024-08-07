//use std::collections::*;
use money::Money;

// Module declarations
pub mod money;


pub struct Wallet {
    accounts: Vec<Account>,
}

impl Wallet {
    fn total_balance(&self) -> Money {
        let mut sum: Money = Money::new(0);
        for account in &self.accounts {
            sum += *account.balance();
        }
        sum
    }
}

pub struct Account {
    pub name: String,
    balance: Money,
}

impl Account {
    pub fn new(name: &str, balance: Money) -> Account {
        Account {
            name: name.to_string(),
            balance: balance,
        }
    }

    pub fn from_i64(name: &str, balance: i64) -> Account {
        Account {
            name: name.to_string(),
            balance: Money::new(balance),
        }
    }

    pub fn balance(&self) -> &Money {
        &self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_balance_equals_sum_of_account_balances() {
        let wallet = Wallet {
            accounts: vec!(
                Account::from_i64("Acc 1", 12345),
                Account::from_i64("Acc 2", 67890),
                Account::from_i64("Acc 3", 12)
            ),
        };
        assert_eq!(wallet.total_balance(), Money::new(80247));
    }

    #[test]
    fn validate_account_constructor() {
        let account = Account::new("Chase Checking", Money::new(1_337_13));
        assert_eq!(account.name, "Chase Checking");
        assert_eq!(account.balance().major_part(), 1_337);
        assert_eq!(account.balance().minor_part(), 13);
    }

    #[test]
    fn validate_account_from_i64() {
        let account = Account::from_i64("Chase Checking", 1_337_13);
        assert_eq!(account.name, "Chase Checking");
        assert_eq!(account.balance().major_part(), 1_337);
        assert_eq!(account.balance().minor_part(), 13);
    }
}
