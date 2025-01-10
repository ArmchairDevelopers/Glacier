use crate::domain::DomainStore;
use crate::index::stack::DomainAssetIndexStack;

pub struct DomainStoreStack {
    pub stack: Vec<DomainStore>,
    pub index_stack: DomainAssetIndexStack,
}

impl DomainStoreStack {
    pub fn new() -> Self {
        Self { stack: Vec::new(), index_stack: DomainAssetIndexStack::new() }
    }

    pub fn push(&mut self, store: DomainStore) {
        self.stack.push(store);
    }

    pub fn load
}
