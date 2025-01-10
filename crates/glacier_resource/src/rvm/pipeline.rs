use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::rvm::RvmBlock;

#[derive(Debug, Serialize, Deserialize)]
pub struct RvmShaderSandboxData {
    blocks: HashMap<u64, RvmBlock>,
}

impl RvmShaderSandboxData {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, hash: u64, rvm_type: RvmBlock) {
        self.blocks.insert(hash, rvm_type);
    }

    pub fn get_block(&self, hash: u64) -> Option<&RvmBlock> {
        self.blocks.get(&hash)
    }

    pub fn has_block(&self, hash: u64) -> bool {
        self.blocks.contains_key(&hash)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&u64, &RvmBlock)> {
        self.blocks.iter()
    }
}
