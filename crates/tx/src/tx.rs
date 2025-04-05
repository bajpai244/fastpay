use bytes::Bytes;
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

        let value = Bytes ::new();
    }
}
