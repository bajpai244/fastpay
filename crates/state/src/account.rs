use alloy::primitives::Address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    address: Address,
    balance: u64,
}

impl Account {
    pub fn new(address: Address, balance: u64) -> Self {
        Self { address, balance }
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: u64) {
        self.balance = balance;
    }

    pub fn get_address(&self) -> Address {
        self.address.clone()
    }
}
