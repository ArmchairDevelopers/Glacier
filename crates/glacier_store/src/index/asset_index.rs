use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use std::borrow::Cow;
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
    pub name: String,
    pub partition: Guid,
    pub primary_type_hash: u32,
    pub instances: Vec<DomainAssetIndexInstance>,
    pub bundles: HashSet<u32>,
    pub imports: HashSet<Guid>, // Partition IDs imported by the asset
    pub res_imports: HashSet<Guid> // Partition IDs imported by sandbox data
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DomainAssetIndexEntrySimplified {
    /// Lowercase name of the asset
    pub name: String,
    pub partition: Guid,
    pub primary_type_hash: u32,
}

pub struct DomainAssetIndex<'a> {
    parent: Option<Cow<'a, DomainAssetIndex<'a>>>,

    by_name: HashMap<String, Guid>, // Map name to partition GUID
    by_name_hash: HashMap<u32, Guid>, // Map name hash to partition GUID
    by_partition: HashMap<Guid, DomainAssetIndexEntry>, // Map partition GUID to entry
    by_type: HashMap<u32, HashSet<Guid>>, // Map instance type hash to partition GUIDs
}

impl<'a> DomainAssetIndex {
    pub fn new() -> Self {
        Self {
            parent: None,
            by_name: HashMap::new(),
            by_name_hash: HashMap::new(),
            by_partition: HashMap::new(),
            by_type: HashMap::new(),
        }
    }

    pub fn load(data: &[u8]) -> Result<Self, bincode::Error> {
        let entries: Vec<DomainAssetIndexEntry> = bincode::deserialize(data)?;
        Ok(Self::load_from_entries(entries))
    }

    pub fn load_from_entries(entries: Vec<DomainAssetIndexEntry>) -> Self {
        let mut by_name = HashMap::new();
        let mut by_name_hash = HashMap::new();
        let mut by_partition = HashMap::new();
        let mut by_type: HashMap<u32, HashSet<Guid>> = HashMap::new();

        for entry in entries {
            let name_lower = entry.name.to_lowercase();

            by_name.insert(name_lower.clone(), entry.partition);
            by_name_hash.insert(name_lower.hash_quick(), entry.partition);
            by_partition.insert(entry.partition, entry.clone());

            for instance in &entry.instances {
                by_type
                    .entry(instance.type_hash)
                    .or_default()
                    .insert(entry.partition);
            }
        }

        Self {
            parent: None,
            by_name,
            by_name_hash,
            by_partition,
            by_type,
        }
    }

    pub fn set_parent(&mut self, parent: Cow<'a, DomainAssetIndex<'a>>) {
        self.parent = Some(parent);
    }

    /// Add or update an entry
    pub fn upsert_entry(&mut self, entry: DomainAssetIndexEntry) {
        // Remove existing entry if it exists
        if let Some(existing_entry) = self.by_partition.remove(&entry.partition) {
            self.remove_from_indices(&existing_entry);
        }

        let name_lower = entry.name.to_lowercase();

        // Add to indices
        self.by_name.insert(name_lower.to_owned(), entry.partition);
        self.by_name_hash.insert(name_lower.hash_quick(), entry.partition);
        self.by_type
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
        let name_lower = entry.name.to_lowercase();

        self.by_name.remove(&name_lower);
        self.by_name_hash.remove(&name_lower.hash_quick());

        for instance in &entry.instances {
            if let Some(set) = self.by_type.get_mut(&instance.type_hash) {
                set.remove(&entry.partition);
                if set.is_empty() {
                    self.by_type.remove(&instance.type_hash);
                }
            }
        }
    }

    /// Get an entry by name
    pub fn get_by_name(&self, name: &str) -> Option<&DomainAssetIndexEntry> {
        let normalized_name = name.to_lowercase();

        if let Some(guid) = self.by_name.get(&normalized_name) {
            if let Some(entry) = self.by_partition.get(guid) {
                return Some(entry);
            }
        }

        self.parent
            .as_ref()
            .and_then(|parent| parent.get_by_name(name))
    }

    /// Get an entry by name hash
    pub fn get_by_name_hash(&self, name_hash: u32) -> Option<&DomainAssetIndexEntry> {
        if let Some(guid) = self.by_name_hash.get(&name_hash) {
            if let Some(entry) = self.by_partition.get(guid) {
                return Some(entry);
            }
        }

        self.parent
            .as_ref()
            .and_then(|parent| parent.get_by_name_hash(name_hash))
    }

    /// Get an entry by partition GUID
    pub fn get_by_partition_guid(&self, partition: &Guid) -> Option<&DomainAssetIndexEntry> {
        if let Some(partition) = self.by_partition.get(partition) {
            return Some(partition);
        }

        self.parent
            .as_ref()
            .and_then(|parent| parent.get_by_partition_guid(partition))
    }

    pub fn get_name_by_partition_guid(&self, partition: &Guid) -> Option<&str> {
        if let Some(entry) = self.get_by_partition_guid(partition) {
            return Some(&entry.name);
        }

        None
    }

    /// Get all entries by primary type hash
    pub fn get_by_instance_type_hash(&self, type_hash: u32) -> Vec<&DomainAssetIndexEntry> {
        let mut results: Vec<&DomainAssetIndexEntry> = self.by_type
            .get(&type_hash)
            .into_iter()
            .flat_map(|guids| guids.iter().filter_map(|guid| self.by_partition.get(guid)))
            .collect();

        if let Some(parent) = &self.parent {
            results.extend(parent.get_by_instance_type_hash(type_hash));
        }

        results
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
    index: Arc<RwLock<DomainAssetIndex<'static>>>,
    imported_partitions: RwLock<HashSet<Guid>>,
}

impl AssetIndexDbxWriterImportResolver {
    pub fn new(index: Arc<RwLock<DomainAssetIndex>>) -> Self {
        Self {
            index,
            imported_partitions: RwLock::new(HashSet::new()),
        }
    }

    pub fn imported_partitions(self) -> HashSet<Guid> {
        self.imported_partitions.into_inner()
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
