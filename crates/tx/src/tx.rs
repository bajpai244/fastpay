use bytes::{Bytes, BytesMut};
use sha3::{Digest, Keccak256};
use state::state::AccountAddress;

pub enum Tx {
    Transfer {
        from: AccountAddress,
        // TODO: we want to allow transfer to multiple addresses, this later on needs to be an array
        to: AccountAddress,
        amount: u64,
    },
}

impl Tx {
    pub fn new(from: AccountAddress, to: AccountAddress, amount: u64) -> Self {
        Self::Transfer { from, to, amount }
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
            Self::Transfer { from, to, amount } => {
                value.extend_from_slice(&from);
                value.extend_from_slice(&to);
                value.extend_from_slice(&amount.to_be_bytes());
                value.freeze()
            }
        }
    }
}
