use std::{collections::HashMap, sync::Arc};

use glacier_util::guid::Guid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct DomainAssetIndexEntry {
    /// Lowercase name of the asset
    pub name: String,
    pub partition: Guid,
    pub instances: Vec<Guid>,
    pub primary_type_hash: u32,
    pub bundles: Vec<u32>,
    pub imports: Vec<Guid>, // Partition IDs
}

#[derive(Default)]
pub struct DomainAssetIndex {
    by_name: HashMap<String, Arc<DomainAssetIndexEntry>>,
    by_partition_guid: HashMap<Guid, Arc<DomainAssetIndexEntry>>,
}

impl DomainAssetIndex {
    pub fn load(data: String) -> Result<Self, serde_json::Error> {
        let entries: Vec<DomainAssetIndexEntry> = serde_json::from_str(&data)?;

        let mut by_name = HashMap::new();
        let mut by_partition_guid = HashMap::new();

        for entry in entries {
            let entry = Arc::new(entry);
            by_name.insert(entry.name.clone(), entry.clone());
            by_partition_guid.insert(entry.partition, entry);
        }

        Ok(Self {
            by_name,
            by_partition_guid,
        })
    }

    pub async fn save(&self) -> Result<String, serde_json::Error> {
        let entries: Vec<DomainAssetIndexEntry> =
            self.by_name.values().map(|e| (**e).clone()).collect();
        Ok(serde_json::to_string(&entries)?)
    }

    pub async fn insert(&mut self, entry: DomainAssetIndexEntry) {
        let entry = Arc::new(entry);
        self.by_name.insert(entry.name.clone(), entry.clone());
        self.by_partition_guid.insert(entry.partition, entry);
    }

    pub fn by_name(&self, name: &str) -> Option<&Arc<DomainAssetIndexEntry>> {
        self.by_name.get(&name.to_lowercase())
    }

    pub fn by_partition_guid(&self, guid: &Guid) -> Option<&Arc<DomainAssetIndexEntry>> {
        self.by_partition_guid.get(guid)
    }
}
