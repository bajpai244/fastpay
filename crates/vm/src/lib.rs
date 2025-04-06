use state::state::State;
use tx::tx::Tx;

pub enum VMError {
    InvalidTransaction(String),
}

pub struct VM {
    state: Box<dyn State>,
}

impl VM {
    pub fn new(state: &dyn State) -> Self {
        Self { state }
    }

    pub fn execute(&self, tx: &Tx) -> Result<(), VMError> {
        Ok(())
    }
}
