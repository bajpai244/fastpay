use alloy::primitives::PrimitiveSignature;
use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use alloy::signers::SignerSync;
use bytes::Bytes;
use tx::tx::Tx;

#[derive(Debug)]
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
        let signer = PrivateKeySigner::random();
        Self { signer }
    }

    pub fn address(&self) -> alloy::primitives::Address {
        self.signer.address()
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::hex;
    use alloy::signers::local::PrivateKeySigner;
    use tx::tx::Tx;

    #[test]
    fn test_wallet_new() {
        let signer = PrivateKeySigner::random();
        let wallet = Wallet::new(signer.clone());

        // Verify the signer is correctly stored
        assert_eq!(wallet.signer.address(), signer.address());
    }

    #[test]
    fn test_wallet_random() {
        let wallet = Wallet::random();

        // Verify we can create a random wallet without panicking
        assert!(wallet.signer.address().to_string().starts_with("0x"));

        // Verify the address is a valid hex string
        let address = wallet.signer.address().to_string();
        assert!(hex::decode(&address[2..]).is_ok()); // Skip "0x" prefix
    }

    #[test]
    fn test_sign_message() {
        let wallet = Wallet::random();
        let message = Bytes::from_static(b"Hello, World!");

        let signature = wallet.sign_message(message.clone()).unwrap();

        // Verify signature length (65 bytes for secp256k1: r (32 bytes) + s (32 bytes) + v (1 byte))
        assert_eq!(signature.as_bytes().len(), 65);

        // Verify we get the same signature for the same message
        let signature2 = wallet.sign_message(message).unwrap();
        assert_eq!(signature.as_bytes(), signature2.as_bytes());
    }

    #[test]
    fn test_sign_different_messages() {
        let wallet = Wallet::random();
        let message1 = Bytes::from_static(b"Hello, World!");
        let message2 = Bytes::from_static(b"Different message");

        let signature1 = wallet.sign_message(message1).unwrap();
        let signature2 = wallet.sign_message(message2).unwrap();

        // Different messages should produce different signatures
        assert_ne!(signature1.as_bytes(), signature2.as_bytes());
    }

    #[test]
    fn test_sign_transaction() {
        let wallet = Wallet::random();

        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();

        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();

        let amount = 100u64;

        let tx = Tx::new(from.clone(), to.clone(), amount, None);
        let signature = wallet.sign_transaction(tx).unwrap();

        // Verify signature length
        assert_eq!(signature.as_bytes().len(), 65);

        // Create a new transaction with the same parameters
        let tx2 = Tx::new(from, to, amount, None);
        let signature2 = wallet.sign_transaction(tx2).unwrap();

        // Verify we get the same signature for the same transaction
        assert_eq!(signature.as_bytes(), signature2.as_bytes());
    }

    #[test]
    fn test_sign_different_transactions() {
        let wallet = Wallet::random();

        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();

        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();

        let tx1 = Tx::new(from, to, 100, None);

        let tx2 = Tx::new(
            from, to, 200, // Different amount
            None,
        );

        let signature1 = wallet.sign_transaction(tx1).unwrap();
        let signature2 = wallet.sign_transaction(tx2).unwrap();

        // Different transactions should produce different signatures
        assert_ne!(signature1.as_bytes(), signature2.as_bytes());
    }

    #[test]
    fn test_different_wallets_different_signatures() {
        let wallet1 = Wallet::random();
        let wallet2 = Wallet::random();
        let message = Bytes::from_static(b"Hello, World!");

        let signature1 = wallet1.sign_message(message.clone()).unwrap();
        let signature2 = wallet2.sign_message(message).unwrap();

        // Different wallets should produce different signatures for the same message
        assert_ne!(signature1.as_bytes(), signature2.as_bytes());
    }
}
