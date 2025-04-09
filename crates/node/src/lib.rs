use state::{memory::MemoryState, state::State};
use tx::tx::Tx;
use vm::{VMError, VM};

pub struct Node {
    vm: VM,
}

impl Node {
    pub fn new(state: Box<dyn State>) -> Self {
        let vm = VM::new(state);
        Self { vm }
    }

    pub fn execute_tx(&mut self, tx: &Tx) -> Result<(), VMError> {
        self.vm.execute(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use alloy::signers::local::PrivateKeySigner;
    use alloy::signers::SignerSync;
    use state::account::Account;
    use wallet::Wallet;

    #[test]
    fn test_multiple_transactions_from_single_wallet() {
        // Create state and node
        let state = Box::new(MemoryState::new());
        let mut node = Node::new(state);

        // Create sender wallet with large initial balance
        let sender_wallet = Wallet::random();
        let sender_address = sender_wallet.address();
        let initial_balance = 1000;
        let sender_account = Account::new(sender_address, initial_balance);
        node.vm
            .state_mut()
            .update_account(&sender_address, sender_account)
            .unwrap();

        // Create recipient wallets
        let recipient1_wallet = Wallet::random();
        let recipient1_address = recipient1_wallet.address();
        let recipient2_wallet = Wallet::random();
        let recipient2_address = recipient2_wallet.address();
        let recipient3_wallet = Wallet::random();
        let recipient3_address = recipient3_wallet.address();

        // First transaction: 100 to recipient1
        let tx1 = Tx::new(sender_address, recipient1_address, 100, None);
        let signature1 = sender_wallet.sign_transaction(tx1.clone()).unwrap();
        let tx1 = Tx::new(sender_address, recipient1_address, 100, Some(signature1));

        // Execute first transaction
        let result = node.execute_tx(&tx1);
        assert!(result.is_ok());

        // Verify balances after first transaction
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance - 100);
        let recipient1_balance = node
            .vm
            .state()
            .get_account(&recipient1_address)
            .unwrap()
            .balance();
        assert_eq!(recipient1_balance, 100);

        // Second transaction: 200 to recipient2
        let tx2 = Tx::new(sender_address, recipient2_address, 200, None);
        let signature2 = sender_wallet.sign_transaction(tx2.clone()).unwrap();
        let tx2 = Tx::new(sender_address, recipient2_address, 200, Some(signature2));

        // Execute second transaction
        let result = node.execute_tx(&tx2);
        assert!(result.is_ok());

        // Verify balances after second transaction
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance - 100 - 200);
        let recipient2_balance = node
            .vm
            .state()
            .get_account(&recipient2_address)
            .unwrap()
            .balance();
        assert_eq!(recipient2_balance, 200);

        // Third transaction: 300 to recipient3
        let tx3 = Tx::new(sender_address, recipient3_address, 300, None);
        let signature3 = sender_wallet.sign_transaction(tx3.clone()).unwrap();
        let tx3 = Tx::new(sender_address, recipient3_address, 300, Some(signature3));

        // Execute third transaction
        let result = node.execute_tx(&tx3);
        assert!(result.is_ok());

        // Verify final balances
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance - 100 - 200 - 300);
        let recipient3_balance = node
            .vm
            .state()
            .get_account(&recipient3_address)
            .unwrap()
            .balance();
        assert_eq!(recipient3_balance, 300);

        // Verify all recipient balances
        assert_eq!(
            node.vm
                .state()
                .get_account(&recipient1_address)
                .unwrap()
                .balance(),
            100
        );
        assert_eq!(
            node.vm
                .state()
                .get_account(&recipient2_address)
                .unwrap()
                .balance(),
            200
        );
        assert_eq!(
            node.vm
                .state()
                .get_account(&recipient3_address)
                .unwrap()
                .balance(),
            300
        );
    }

    #[test]
    fn test_insufficient_balance_after_multiple_transactions() {
        // Create state and node
        let state = Box::new(MemoryState::new());
        let mut node = Node::new(state);

        // Create sender wallet with initial balance
        let sender_wallet = Wallet::random();
        let sender_address = sender_wallet.address();
        let initial_balance = 100;
        let sender_account = Account::new(sender_address, initial_balance);
        node.vm
            .state_mut()
            .update_account(&sender_address, sender_account)
            .unwrap();

        // Create recipient wallet
        let recipient_wallet = Wallet::random();
        let recipient_address = recipient_wallet.address();

        // First transaction: 50 to recipient
        let tx1 = Tx::new(sender_address, recipient_address, 50, None);
        let signature1 = sender_wallet.sign_transaction(tx1.clone()).unwrap();
        let tx1 = Tx::new(sender_address, recipient_address, 50, Some(signature1));

        // Execute first transaction
        let result = node.execute_tx(&tx1);
        assert!(result.is_ok());

        // Verify balances after first transaction
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance - 50);
        let recipient_balance = node
            .vm
            .state()
            .get_account(&recipient_address)
            .unwrap()
            .balance();
        assert_eq!(recipient_balance, 50);

        // Second transaction: 60 to recipient (should fail due to insufficient balance)
        let tx2 = Tx::new(sender_address, recipient_address, 60, None);
        let signature2 = sender_wallet.sign_transaction(tx2.clone()).unwrap();
        let tx2 = Tx::new(sender_address, recipient_address, 60, Some(signature2));

        // Execute second transaction
        let result = node.execute_tx(&tx2);
        assert!(result.is_err());
        match result.unwrap_err() {
            VMError::InvalidTransaction(msg) => {
                assert!(msg.contains("does not have enough balance"));
            }
        }

        // Verify balances remain unchanged after failed transaction
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance - 50);
        let recipient_balance = node
            .vm
            .state()
            .get_account(&recipient_address)
            .unwrap()
            .balance();
        assert_eq!(recipient_balance, 50);
    }

    #[test]
    fn test_transaction_with_invalid_signature() {
        // Create state and node
        let state = Box::new(MemoryState::new());
        let mut node = Node::new(state);

        // Create sender wallet with initial balance
        let sender_wallet = Wallet::random();
        let sender_address = sender_wallet.address();
        let initial_balance = 100;
        let sender_account = Account::new(sender_address, initial_balance);
        node.vm
            .state_mut()
            .update_account(&sender_address, sender_account)
            .unwrap();

        // Create recipient wallet
        let recipient_wallet = Wallet::random();
        let recipient_address = recipient_wallet.address();

        // Create transaction with signature from wrong wallet
        let tx = Tx::new(sender_address, recipient_address, 50, None);
        let wrong_wallet = Wallet::random();
        let signature = wrong_wallet.sign_transaction(tx.clone()).unwrap();
        let tx = Tx::new(sender_address, recipient_address, 50, Some(signature));

        // Execute transaction
        let result = node.execute_tx(&tx);
        assert!(result.is_err());
        match result.unwrap_err() {
            VMError::InvalidTransaction(msg) => {
                assert!(msg.contains("signature is invalid"));
            }
        }

        // Verify balances remain unchanged
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance);
        assert!(node.vm.state().get_account(&recipient_address).is_none());
    }

    #[test]
    fn test_transaction_to_nonexistent_recipient() {
        // Create state and node
        let state = Box::new(MemoryState::new());
        let mut node = Node::new(state);

        // Create sender wallet with initial balance
        let sender_wallet = Wallet::random();
        let sender_address = sender_wallet.address();
        let initial_balance = 100;
        let sender_account = Account::new(sender_address, initial_balance);
        node.vm
            .state_mut()
            .update_account(&sender_address, sender_account)
            .unwrap();

        // Create recipient wallet (but don't add to state)
        let recipient_wallet = Wallet::random();
        let recipient_address = recipient_wallet.address();

        // Create and sign transaction
        let tx = Tx::new(sender_address, recipient_address, 50, None);
        let signature = sender_wallet.sign_transaction(tx.clone()).unwrap();
        let tx = Tx::new(sender_address, recipient_address, 50, Some(signature));

        // Execute transaction
        let result = node.execute_tx(&tx);
        assert!(result.is_ok());

        // Verify balances
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance - 50);
        let recipient_balance = node
            .vm
            .state()
            .get_account(&recipient_address)
            .unwrap()
            .balance();
        assert_eq!(recipient_balance, 50);
    }

    #[test]
    fn test_zero_amount_transaction() {
        // Create state and node
        let state = Box::new(MemoryState::new());
        let mut node = Node::new(state);

        // Create sender wallet with initial balance
        let sender_wallet = Wallet::random();
        let sender_address = sender_wallet.address();
        let initial_balance = 100;
        let sender_account = Account::new(sender_address, initial_balance);
        node.vm
            .state_mut()
            .update_account(&sender_address, sender_account)
            .unwrap();

        // Create recipient wallet
        let recipient_wallet = Wallet::random();
        let recipient_address = recipient_wallet.address();

        // Create and sign transaction with zero amount
        let tx = Tx::new(sender_address, recipient_address, 0, None);
        let signature = sender_wallet.sign_transaction(tx.clone()).unwrap();
        let tx = Tx::new(sender_address, recipient_address, 0, Some(signature));

        // Execute transaction
        let result = node.execute_tx(&tx);
        assert!(result.is_ok());

        // Verify balances remain unchanged
        let sender_balance = node
            .vm
            .state()
            .get_account(&sender_address)
            .unwrap()
            .balance();
        assert_eq!(sender_balance, initial_balance);
        let recipient_balance = node
            .vm
            .state()
            .get_account(&recipient_address)
            .unwrap()
            .balance();
        assert_eq!(recipient_balance, 0);
    }
}
