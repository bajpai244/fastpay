use alloy::primitives::Address;
use state::state::State;
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

    pub fn execute(&self, tx: &Tx) -> Result<(), VMError> {
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

        Ok(())
    }
}
