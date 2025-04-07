use alloy::primitives::{Address, PrimitiveSignature};
use bytes::{Bytes, BytesMut};
use sha3::{Digest, Keccak256};

#[derive(Clone)]
pub enum Tx {
    Transfer {
        from: Address,
        // TODO: we want to allow transfer to multiple addresses, this later on needs to be an array
        to: Address,
        amount: u64,
        signature: Option<PrimitiveSignature>,
    },
}

impl Tx {
    pub fn new(
        from: Address,
        to: Address,
        amount: u64,
        signature: Option<PrimitiveSignature>,
    ) -> Self {
        Self::Transfer {
            from,
            to,
            amount,
            signature,
        }
    }

    pub fn is_transfer(&self) -> bool {
        matches!(self, Self::Transfer { .. })
    }

    pub fn from(&self) -> Address {
        match self {
            Self::Transfer { from, .. } => from.clone(),
        }
    }

    pub fn to(&self) -> Address {
        match self {
            Self::Transfer { to, .. } => to.clone(),
        }
    }

    pub fn amount(&self) -> u64 {
        match self {
            Self::Transfer { amount, .. } => *amount,
        }
    }

    pub fn signature(&self) -> Option<PrimitiveSignature> {
        match self {
            Self::Transfer { signature, .. } => signature.clone(),
        }
    }

    pub fn tx_hash(&self) -> Bytes {
        let value = self.to_bytes();

        let mut hasher = Keccak256::new();
        hasher.update(value);

        let hash = Bytes::from(hasher.finalize().to_vec());

        hash
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut value = BytesMut::new();
        match self {
            Self::Transfer {
                from,
                to,
                amount,
                signature: _,
            } => {
                value.extend_from_slice(&from.to_vec());
                value.extend_from_slice(&to.to_vec());
                value.extend_from_slice(&amount.to_be_bytes());
                value.freeze()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::local::PrivateKeySigner;

    #[test]
    fn test_new_transfer() {
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();

        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();

        let amount = 100u64;

        let tx = Tx::new(from.clone(), to.clone(), amount, None);

        assert!(tx.is_transfer());

        let Tx::Transfer {
            from: f,
            to: t,
            amount: a,
            signature: s,
        } = tx;

        assert_eq!(f, from);
        assert_eq!(t, to);
        assert_eq!(a, amount);
        assert_eq!(s, None);
    }

    #[test]
    fn test_is_transfer() {
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();

        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();

        let amount = 100u64;

        let tx = Tx::new(from, to, amount, None);
        assert!(tx.is_transfer());
    }

    #[test]
    fn test_to_bytes() {
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();

        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();

        let amount = 100u64;

        let tx = Tx::new(from.clone(), to.clone(), amount, None);
        let bytes = tx.to_bytes();

        // Expected length: 20 (from) + 20 (to) + 8 (amount) = 48 bytes
        assert_eq!(bytes.len(), 48);

        // Verify from address
        assert_eq!(&bytes[0..20], &from.to_vec());
        // Verify to address
        assert_eq!(&bytes[20..40], &to.to_vec());
        // Verify amount
        assert_eq!(&bytes[40..48], &amount.to_be_bytes());
    }

    #[test]
    fn test_tx_hash() {
        let from_signer = PrivateKeySigner::random();
        let from = from_signer.address();

        let to_signer = PrivateKeySigner::random();
        let to = to_signer.address();

        let amount = 100u64;

        let tx = Tx::new(from.clone(), to.clone(), amount, None);
        let hash = tx.tx_hash();

        // Keccak256 hash should be 32 bytes
        assert_eq!(hash.len(), 32);

        // Hash should be deterministic
        let hash2 = tx.tx_hash();
        assert_eq!(hash, hash2);

        // Different transaction should have different hash
        let tx2 = Tx::new(from, to, amount + 1, None);
        let hash3 = tx2.tx_hash();
        assert_ne!(hash, hash3);
    }
}
