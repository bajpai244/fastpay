// in memory implementation of the state

use std::collections::HashMap;

use bytes::Bytes;

use crate::account::Account;
use crate::state::{State, StateError};

pub type AccountAddress = Bytes;

pub struct MemoryState {
    accounts: HashMap<AccountAddress, Account>,
}

impl State for MemoryState {
    fn get_account(&self, address: &AccountAddress) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    fn update_account(
        &mut self,
        address: &AccountAddress,
        account: Account,
    ) -> Result<(), StateError> {
        self.accounts.insert(address.clone(), account);
        Ok(())
    }
}
