use crate::account::Account;
use alloy::primitives::Address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    AccountNotFound,
    AccountBalanceTooLow,
}

// State in fastpay is simple, it allows you to read & update accounts based on their address
pub trait State {
    fn get_account(&self, address: &Address) -> Option<Account>;

    fn update_account(&mut self, address: &Address, account: Account) -> Result<(), StateError>;
}
