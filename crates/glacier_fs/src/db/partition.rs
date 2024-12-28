use glacier_reflect::type_info::{LockedTypeObject, TypeInfo, TypeInfoData};
use glacier_util::guid::Guid;

#[derive(Default)]
pub struct PartitionInitData {
    pub name: String,
    pub guid: Guid,
    pub instances: Vec<LockedTypeObject>,
}

#[derive(Debug)]
pub struct DatabasePartition {
    name: String,
    guid: Guid,
    instances: Vec<LockedTypeObject>,
}

impl DatabasePartition {
    pub fn new(init_data: PartitionInitData) -> Self {
        Self {
            name: init_data.name,
            guid: init_data.guid,
            instances: init_data.instances,
        }
    }

    pub fn new_empty(name: String, guid: Guid) -> Self {
        Self {
            name,
            guid,
            instances: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn guid(&self) -> &Guid {
        &self.guid
    }

    /// Sets the instance's partition guid to this partition and adds it to the partition's instance list
    pub async fn add_instance(&mut self, instance: LockedTypeObject) {
        {
            let mut instance = instance.lock().await;
            let core = instance.data_container_core_mut().expect("Instance is not a DataContainer");
            core.partition_guid = self.guid().clone();
        }

        self.instances.push(instance);
    }

    pub async fn create_instance_with_id(&self, guid: Guid, type_info: &'static TypeInfo) -> Option<LockedTypeObject> {
        if let TypeInfoData::Class(class_info) = &type_info.data {
            let instance = class_info.create();

            {
                let mut instance = instance.lock().await;
                let core = instance.data_container_core_mut().expect("Instance is not a DataContainer");
                core.partition_guid = self.guid().clone();
                core.instance_guid = guid;
            }

            Some(instance)
        } else {
            None
        }
    }

    pub async fn create_instance(&self, type_info: &'static TypeInfo) -> Option<LockedTypeObject> {
        self.create_instance_with_id(Guid::random(), type_info).await
    }

    pub fn instances(&self) -> &[LockedTypeObject] {
        &self.instances
    }

    pub fn primary_instance(&self) -> Option<&LockedTypeObject> {
        self.instances.first()
    }

    pub async fn instance_by_guid(&self, guid: &Guid) -> Option<&LockedTypeObject> {
        for instance in &self.instances {
            let locked_instance = instance.lock().await;
            let core = locked_instance.data_container_core().expect("Instance is not a DataContainer");

            if core.instance_guid == *guid {
                return Some(instance);
            }
        }

        None
    }
}

impl PartialEq for DatabasePartition {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
    }
}
