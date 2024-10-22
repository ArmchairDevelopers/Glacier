use glacier_reflect::type_info::LockedTypeObject;
use glacier_util::guid::Guid;

#[derive(Default)]
pub struct PartitionInitData {
    pub guid: Guid,
    pub instances: Vec<LockedTypeObject>,
}

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

    pub fn add_instance(&mut self, instance: LockedTypeObject) {
        self.instances.push(instance);
    }

    pub fn instances(&self) -> &[LockedTypeObject] {
        &self.instances
    }

    pub fn primary_instance(&self) -> Option<&LockedTypeObject> {
        self.instances.first()
    }
}
