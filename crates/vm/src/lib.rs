use alloy::primitives::Address;
use state::{account::Account, state::State};
use tx::tx::Tx;

pub enum VMError {
    InvalidTransaction(String),
}

pub struct VM {
    state: Box<dyn State>,
}

impl VM {
    pub fn new(state: Box<dyn State>) -> Self {
        Self { state }
    }

    pub fn execute(&mut self, tx: &Tx) -> Result<(), VMError> {
        let from = tx.from();
        let to = tx.to();
        let amount = tx.amount();

        let signature = tx.signature();

        if signature.is_none() {
            return Err(VMError::InvalidTransaction(
                "Transaction has no signature".to_string(),
            ));
        }

        let signature = signature.unwrap();

        let recovered_address = signature.recover_address_from_msg(tx.tx_hash());

        // TODO: ideally we need to wrap this error in VM error
        if recovered_address.is_err() {
            return Err(VMError::InvalidTransaction(
                "Transaction signature is invalid".to_string(),
            ));
        }

        let recovered_address = recovered_address.unwrap();

        if recovered_address != from {
            return Err(VMError::InvalidTransaction(
                "Transaction signature is invalid".to_string(),
            ));
        }

        let from_account = self.state.get_account(&from);

        if from_account.is_none() {
            return Err(VMError::InvalidTransaction(
                "Transaction sender account does not exist".to_string(),
            ));
        }

        let from_account = from_account.unwrap();
        let from_balance = from_account.balance();

        if from_balance < amount {
            return Err(VMError::InvalidTransaction(
                "Transaction sender account does not have enough balance".to_string(),
            ));
        }

        let updated_from_account = Account::new(from, from_balance - amount);
        match self.state.update_account(&from, updated_from_account) {
            Ok(_) => (),
            Err(_) => {
                return Err(VMError::InvalidTransaction(
                    "Transaction sender account does not have enough balance".to_string(),
                ));
            }
        };

        let to_account_exists = self.state.get_account(&to).is_none();

        if to_account_exists {
            let to_account = Account::new(to, amount);
            let update_result = self.state.update_account(&to, to_account);

            if update_result.is_err() {
                return Err(VMError::InvalidTransaction(
                    "Transaction sender account does not have enough balance".to_string(),
                ));
            };
        } else {
            let to_account = self.state.get_account(&to).unwrap();
            let to_balance = to_account.balance();

            let updated_to_account = Account::new(to, to_balance + amount);
            let update_result = self.state.update_account(&to, updated_to_account);

            if update_result.is_err() {
                return Err(VMError::InvalidTransaction(
                    "Transaction sender account does not have enough balance".to_string(),
                ));
            };
        };

        Ok(())
    }

    pub fn state(&self) -> &Box<dyn State> {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut Box<dyn State> {
        &mut self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::local::PrivateKeySigner;
    use alloy::signers::SignerSync;
    use state::memory::MemoryState;

    #[test]
    fn test_vm_constructor() {
        let state = Box::new(MemoryState::new());
        let vm = VM::new(state);
        assert!(vm.state.get_account(&Address::ZERO).is_none());
    }

    #[test]
    fn test_execute_valid_transaction() {
        let mut state = MemoryState::new();
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();
        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();
        let initial_balance = 100;

        // Create and add sender account
        let from_account = Account::new(from, initial_balance);
        state.update_account(&from, from_account).unwrap();

        let vm = VM::new(Box::new(state));
        let mut vm = vm;

        // Create a valid transaction
        let tx = Tx::new(from, to, 50, None);
        let tx_hash = tx.tx_hash();
        let signature = from_signer.sign_message_sync(&tx_hash).unwrap();
        let tx = Tx::new(from, to, 50, Some(signature));

        // Execute transaction
        let result = vm.execute(&tx);
        assert!(result.is_ok());

        // Verify balances
        let from_account = vm.state.get_account(&from).unwrap();
        let to_account = vm.state.get_account(&to).unwrap();
        assert_eq!(from_account.balance(), initial_balance - 50);
        assert_eq!(to_account.balance(), 50);
    }

    #[test]
    fn test_execute_insufficient_balance() {
        let mut state = MemoryState::new();
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();
        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();
        let initial_balance = 30;

        // Create and add sender account with insufficient balance
        let from_account = Account::new(from, initial_balance);
        state.update_account(&from, from_account).unwrap();

        let vm = VM::new(Box::new(state));
        let mut vm = vm;

        // Create a transaction with amount > balance
        let tx = Tx::new(from, to, 50, None);
        let tx_hash = tx.tx_hash();
        let signature = from_signer.sign_message_sync(&tx_hash).unwrap();
        let tx = Tx::new(from, to, 50, Some(signature));

        // Execute transaction
        let result = vm.execute(&tx);
        assert!(result.is_err());
        match result.unwrap_err() {
            VMError::InvalidTransaction(msg) => {
                assert!(msg.contains("does not have enough balance"));
            }
        }
    }

    #[test]
    fn test_execute_invalid_signature() {
        let mut state = MemoryState::new();
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();
        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();
        let initial_balance = 100;

        // Create and add sender account
        let from_account = Account::new(from, initial_balance);
        state.update_account(&from, from_account).unwrap();

        let vm = VM::new(Box::new(state));
        let mut vm = vm;

        // Create a transaction with invalid signature
        let tx = Tx::new(from, to, 50, None);
        let tx_hash = tx.tx_hash();
        let wrong_signer = PrivateKeySigner::random();
        let signature = wrong_signer.sign_message_sync(&tx_hash).unwrap();
        let tx = Tx::new(from, to, 50, Some(signature));

        // Execute transaction
        let result = vm.execute(&tx);
        assert!(result.is_err());
        match result.unwrap_err() {
            VMError::InvalidTransaction(msg) => {
                assert!(msg.contains("signature is invalid"));
            }
        }
    }

    #[test]
    fn test_execute_nonexistent_sender() {
        let state = MemoryState::new();
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();
        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();

        let vm = VM::new(Box::new(state));
        let mut vm = vm;

        // Create a transaction from non-existent account
        let tx = Tx::new(from, to, 50, None);
        let tx_hash = tx.tx_hash();
        let signature = from_signer.sign_message_sync(&tx_hash).unwrap();
        let tx = Tx::new(from, to, 50, Some(signature));

        // Execute transaction
        let result = vm.execute(&tx);
        assert!(result.is_err());
        match result.unwrap_err() {
            VMError::InvalidTransaction(msg) => {
                assert!(msg.contains("sender account does not exist"));
            }
        }
    }
}
