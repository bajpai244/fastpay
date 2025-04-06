// in memory implementation of the state

use std::collections::HashMap;

use alloy::primitives::Address;

use crate::account::Account;
use crate::state::{State, StateError};

pub struct MemoryState {
    accounts: HashMap<Address, Account>,
}

impl MemoryState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
}

impl State for MemoryState {
    fn get_account(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    fn update_account(&mut self, address: &Address, account: Account) -> Result<(), StateError> {
        self.accounts.insert(address.clone(), account);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::local::PrivateKeySigner;

    #[test]
    fn test_new_memory_state() {
        let state = MemoryState::new();
        assert!(state.accounts.is_empty());
    }

    #[test]
    fn test_get_nonexistent_account() {
        let state = MemoryState::new();
        let signer = PrivateKeySigner::random();
        let address = signer.address();
        assert_eq!(state.get_account(&address), None);
    }

    #[test]
    fn test_update_and_get_account() {
        let mut state = MemoryState::new();

        let signer = PrivateKeySigner::random();
        let address = signer.address();
        let account = Account::new(address.clone(), 100);

        // Update account
        state.update_account(&address, account.clone()).unwrap();

        // Get account and verify
        let retrieved = state.get_account(&address).unwrap();
        assert_eq!(retrieved.balance(), 100);
        assert_eq!(retrieved.get_address(), address);
    }

    #[test]
    fn test_update_account_multiple_times() {
        let mut state = MemoryState::new();
        let signer = PrivateKeySigner::random();
        let address = signer.address();

        // First update
        let account1 = Account::new(address.clone(), 100);
        state.update_account(&address, account1).unwrap();

        // Second update
        let account2 = Account::new(address.clone(), 200);
        state.update_account(&address, account2.clone()).unwrap();

        // Verify latest update
        let retrieved = state.get_account(&address).unwrap();
        assert_eq!(retrieved.balance(), 200);
    }

    #[test]
    fn test_multiple_accounts() {
        let mut state = MemoryState::new();

        // Add first account
        let signer1 = PrivateKeySigner::random();
        let address1 = signer1.address();
        let account1 = Account::new(address1.clone(), 100);
        state.update_account(&address1, account1).unwrap();

        // Add second account
        let signer2 = PrivateKeySigner::random();
        let address2 = signer2.address();
        let account2 = Account::new(address2.clone(), 200);
        state.update_account(&address2, account2).unwrap();

        // Verify both accounts
        assert_eq!(state.get_account(&address1).unwrap().balance(), 100);
        assert_eq!(state.get_account(&address2).unwrap().balance(), 200);
    }
}
