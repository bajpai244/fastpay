use bytes::Bytes;

use crate::account::Account;

pub type AccountAddress = Bytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateError {
    AccountNotFound,
    AccountBalanceTooLow,
}

// State in fastpay is simple, it allows you to read & update accounts based on their address
pub trait State {
    fn get_account(&self, address: &AccountAddress) -> Option<Account>;

    fn update_account(
        &mut self,
        address: &AccountAddress,
        account: Account,
    ) -> Result<(), StateError>;
}
