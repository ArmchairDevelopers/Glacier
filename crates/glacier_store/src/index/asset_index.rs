use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use glacier_fs::dbx::writer::DbxWriterImportResolver;
use glacier_util::{guid::Guid, hash::QuickHashExt};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Deserialize, Serialize, Clone)]
pub struct DomainAssetIndexInstance {
    pub guid: Guid,
    pub type_hash: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DomainAssetIndexEntry {
    /// Lowercase name of the asset
    pub name: String,
    pub partition: Guid,
    pub primary_type_hash: u32,
    pub instances: Vec<DomainAssetIndexInstance>,
    pub bundles: Vec<u32>,
    pub imports: Vec<Guid>, // Partition IDs
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DomainAssetIndexEntrySimplified {
    /// Lowercase name of the asset
    pub name: String,
    pub partition: Guid,
    pub primary_type_hash: u32,
}

pub struct DomainAssetIndex {
    by_name: HashMap<String, Guid>, // Map name to partition GUID
    by_name_hash: HashMap<u32, Guid>, // Map name hash to partition GUID
    by_partition: HashMap<Guid, DomainAssetIndexEntry>, // Map partition GUID to entry
    by_primary_type: HashMap<u32, HashSet<Guid>>, // Map primary type hash to partition GUIDs
}

impl DomainAssetIndex {
    pub fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            by_name_hash: HashMap::new(),
            by_partition: HashMap::new(),
            by_primary_type: HashMap::new(),
        }
    }

    pub fn load(data: String) -> Result<Self, serde_json::Error> {
        let entries: Vec<DomainAssetIndexEntry> = serde_json::from_str(&data)?;

        let mut by_name = HashMap::new();
        let mut by_name_hash = HashMap::new();
        let mut by_partition = HashMap::new();
        let mut by_primary_type: HashMap<u32, HashSet<Guid>> = HashMap::new();

        for entry in entries {
            by_name.insert(entry.name.clone(), entry.partition);
            by_name_hash.insert(entry.name.hash_quick(), entry.partition);
            by_partition.insert(entry.partition, entry.clone());
            by_primary_type
                .entry(entry.primary_type_hash)
                .or_default()
                .insert(entry.partition);
        }

        Ok(Self {
            by_name,
            by_name_hash,
            by_partition,
            by_primary_type,
        })
    }

    /// Add or update an entry
    pub fn upsert_entry(&mut self, entry: DomainAssetIndexEntry) {
        // Remove existing entry if it exists
        if let Some(existing_entry) = self.by_partition.remove(&entry.partition) {
            self.remove_from_indices(&existing_entry);
        }

        // Add to indices
        self.by_name.insert(entry.name.clone(), entry.partition);
        self.by_name_hash.insert(entry.name.hash_quick(), entry.partition);
        self.by_primary_type
            .entry(entry.primary_type_hash)
            .or_default()
            .insert(entry.partition);
        self.by_partition.insert(entry.partition, entry);
    }

    /// Delete an entry by partition GUID and cascade
    pub fn delete_entry(&mut self, partition: &Guid) {
        if let Some(entry) = self.by_partition.remove(partition) {
            self.remove_from_indices(&entry);

            // Remove this partition GUID from others' imports. Temporarily disabled
            // for performance reasons; this can technically be done lazily.
            // for (_, other_entry) in &mut self.by_partition {
            //     other_entry.imports.retain(|import| import != partition);
            // }
        }
    }

    pub fn delete_entry_by_name(&mut self, name: &str) {
        if let Some(partition) = self.by_name.remove(name) {
            self.delete_entry(&partition);
        }
    }

    /// Efficiently remove an entry from the indices
    fn remove_from_indices(&mut self, entry: &DomainAssetIndexEntry) {
        self.by_name.remove(&entry.name);
        self.by_name_hash.remove(&entry.name.hash_quick());
        if let Some(set) = self.by_primary_type.get_mut(&entry.primary_type_hash) {
            set.remove(&entry.partition);
            if set.is_empty() {
                self.by_primary_type.remove(&entry.primary_type_hash);
            }
        }
    }

    /// Get an entry by name
    pub fn get_by_name(&self, name: &str) -> Option<&DomainAssetIndexEntry> {
        self.by_name
            .get(name)
            .and_then(|guid| self.by_partition.get(guid))
    }

    /// Get an entry by name hash
    pub fn get_by_name_hash(&self, name_hash: u32) -> Option<&DomainAssetIndexEntry> {
        self.by_name_hash
            .get(&name_hash)
            .and_then(|guid| self.by_partition.get(guid))
    }

    /// Get an entry by partition GUID
    pub fn get_by_partition_guid(&self, partition: &Guid) -> Option<&DomainAssetIndexEntry> {
        self.by_partition.get(partition)
    }

    pub fn get_name_by_partition_guid(&self, partition: &Guid) -> Option<&str> {
        self.by_partition
            .get(partition)
            .map(|entry| entry.name.as_str())
    }

    /// Get all entries by primary type hash
    pub fn get_by_primary_type_hash(&self, type_hash: u32) -> Vec<&DomainAssetIndexEntry> {
        self.by_primary_type
            .get(&type_hash)
            .into_iter()
            .flat_map(|guids| guids.iter().filter_map(|guid| self.by_partition.get(guid)))
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &DomainAssetIndexEntry> {
        self.by_partition.values()
    }

    pub fn values(&self) -> Vec<&DomainAssetIndexEntry> {
        self.by_partition.values().collect()
    }

    pub fn values_cloned(&self) -> Vec<DomainAssetIndexEntrySimplified> {
        self.by_partition
            .values()
            .map(|entry| DomainAssetIndexEntrySimplified {
                name: entry.name.to_owned(),
                partition: entry.partition,
                primary_type_hash: entry.primary_type_hash,
            })
            .collect()
    }
}

pub struct AssetIndexDbxWriterImportResolver {
    index: Arc<RwLock<DomainAssetIndex>>,
    imported_partitions: RwLock<HashSet<Guid>>,
}

impl AssetIndexDbxWriterImportResolver {
    pub fn new(index: Arc<RwLock<DomainAssetIndex>>) -> Self {
        Self {
            index,
            imported_partitions: RwLock::new(HashSet::new()),
        }
    }

    pub fn imported_partitions(self) -> Vec<Guid> {
        self.imported_partitions.into_inner().into_iter().collect()
    }
}

#[async_trait::async_trait]
impl DbxWriterImportResolver for AssetIndexDbxWriterImportResolver {
    async fn resolve_import_name(&self, partition_guid: &Guid) -> Option<String> {
        self.imported_partitions
            .write()
            .await
            .insert(partition_guid.clone());

        let index = self.index.read().await;
        let data = index.get_name_by_partition_guid(partition_guid)?;
        Some(data.to_owned())
    }
}
