use glacier_reflect::type_info::{LockedTypeObject, TypeInfo, TypeInfoData};
use glacier_util::guid::Guid;

#[derive(Default)]
pub struct PartitionInitData {
    pub guid: Guid,
    pub instances: Vec<LockedTypeObject>,
}

#[derive(Debug)]
pub struct DatabasePartition {
    guid: Guid,
    instances: Vec<LockedTypeObject>,
}

impl DatabasePartition {
    pub fn new(init_data: PartitionInitData) -> Self {
        Self {
            guid: init_data.guid,
            instances: init_data.instances,
        }
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

    pub async fn create_instance(&self, type_info: &'static TypeInfo) -> Option<LockedTypeObject> {
        if let TypeInfoData::Class(class_info) = &type_info.data {
            let instance = class_info.create();

            {
                let mut instance = instance.lock().await;
                let core = instance.data_container_core_mut().expect("Instance is not a DataContainer");
                core.partition_guid = self.guid().clone();
                core.instance_guid = Guid::random();
            }

            Some(instance)
        } else {
            None
        }
    }

    pub fn instances(&self) -> &[LockedTypeObject] {
        &self.instances
    }

    pub fn primary_instance(&self) -> Option<&LockedTypeObject> {
        self.instances.first()
    }
}

impl PartialEq for DatabasePartition {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
    }
}
