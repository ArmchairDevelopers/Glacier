use crate::index::asset_index::{DomainAssetIndex, DomainAssetIndexEntry};
use glacier_util::guid::Guid;

pub struct DomainAssetIndexStack {
    stack: Vec<DomainAssetIndex>,
}

impl DomainAssetIndexStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, index: DomainAssetIndex) {
        self.stack.push(index);
    }

    /// Add or update an entry
    pub fn upsert_entry(&mut self, entry: DomainAssetIndexEntry) {
        if let Some(index) = self.stack.last_mut() {
            index.upsert_entry(entry);
        }
    }

    pub fn delete_entry(&mut self, partition: &Guid) {
        if let Some(index) = self.stack.last_mut() {
            index.delete_entry(partition);
        }
    }

    pub fn get_by_name(&self, name: &str) -> Option<&DomainAssetIndexEntry> {
        self.stack
            .iter()
            .rev()
            .find_map(|index| index.get_by_name(name))
    }

    pub fn get_by_name_hash(&self, name_hash: u32) -> Option<&DomainAssetIndexEntry> {
        self.stack
            .iter()
            .rev()
            .find_map(|index| index.get_by_name_hash(name_hash))
    }

    pub fn get_by_partition_guid(&self, partition: &Guid) -> Option<&DomainAssetIndexEntry> {
        self.stack
            .iter()
            .rev()
            .find_map(|index| index.get_by_partition_guid(partition))
    }

    pub fn get_name_by_partition_guid(&self, partition: &Guid) -> Option<&str> {
        self.stack
            .iter()
            .rev()
            .find_map(|index| index.get_name_by_partition_guid(partition))
    }

    pub fn get_by_primary_type_hash(&self, type_hash: u32) -> Vec<&DomainAssetIndexEntry> {
        self.stack
            .iter()
            .rev()
            .flat_map(|index| index.get_by_instance_type_hash(type_hash))
            .collect()
    }
}
