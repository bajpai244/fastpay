use state::{memory::MemoryState, state::State};
use vm::VM;

pub struct Node {
    vm: VM,
}

impl Node {
    pub fn new() -> Self {
        let state = Box::new(MemoryState::new());
        let vm = VM::new(state);
        Self { vm }
    }
}
