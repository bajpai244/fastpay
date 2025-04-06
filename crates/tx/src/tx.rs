use alloy::primitives::PrimitiveSignature;
use bytes::{Bytes, BytesMut};
use sha3::{Digest, Keccak256};
use state::state::AccountAddress;

pub enum Tx {
    Transfer {
        from: AccountAddress,
        // TODO: we want to allow transfer to multiple addresses, this later on needs to be an array
        to: AccountAddress,
        amount: u64,
        signature: Option<PrimitiveSignature>,
    },
}

impl Tx {
    pub fn new(
        from: AccountAddress,
        to: AccountAddress,
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
                value.extend_from_slice(&from);
                value.extend_from_slice(&to);
                value.extend_from_slice(&amount.to_be_bytes());
                value.freeze()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_new_transfer() {
        let from = Bytes::from_static(&[1; 32]);
        let to = Bytes::from_static(&[2; 32]);
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
        let from = Bytes::from_static(&[1; 32]);
        let to = Bytes::from_static(&[2; 32]);
        let amount = 100u64;

        let tx = Tx::new(from, to, amount, None);
        assert!(tx.is_transfer());
    }

    #[test]
    fn test_to_bytes() {
        let from = Bytes::from_static(&[1; 32]);
        let to = Bytes::from_static(&[2; 32]);
        let amount = 100u64;

        let tx = Tx::new(from.clone(), to.clone(), amount, None);
        let bytes = tx.to_bytes();

        // Expected length: 32 (from) + 32 (to) + 8 (amount) = 72 bytes
        assert_eq!(bytes.len(), 72);

        // Verify from address
        assert_eq!(&bytes[0..32], &from);
        // Verify to address
        assert_eq!(&bytes[32..64], &to);
        // Verify amount
        assert_eq!(&bytes[64..72], &amount.to_be_bytes());
    }

    #[test]
    fn test_tx_hash() {
        let from = Bytes::from_static(&[1; 32]);
        let to = Bytes::from_static(&[2; 32]);
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
