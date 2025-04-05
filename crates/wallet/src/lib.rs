use alloy::primitives::PrimitiveSignature;
use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use alloy::signers::SignerSync;
use bytes::Bytes;
use tx::tx::Tx;

pub enum WalletError {
    SigningError(alloy::signers::Error),
}

pub struct Wallet<T> {
    signer: LocalSigner<T>,
}

impl Wallet<SigningKey> {
    pub fn new(signer: LocalSigner<SigningKey>) -> Self {
        Self { signer }
    }

    pub fn random() -> Self {
        let signer = LocalSigner::random();
        Self { signer }
    }

    pub fn sign_message(&self, message: Bytes) -> Result<PrimitiveSignature, WalletError> {
        let signature = self.signer.sign_message_sync(&message);

        match signature {
            Ok(signature) => Ok(signature),
            Err(e) => Err(WalletError::SigningError(e)),
        }
    }

    pub fn sign_transaction(&self, transaction: Tx) -> Result<PrimitiveSignature, WalletError> {
        let message = transaction.tx_hash();

        self.sign_message(message)
    }
}
