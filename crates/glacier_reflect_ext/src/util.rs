use glacier_reflect::{data_container::DataContainerCore, type_info::LockedTypeObject};
use glacier_util::guid::Guid;

pub async fn get_dc_partition_guid(obj: &LockedTypeObject) -> Guid {
    let obj = obj.lock().await;
    obj.data_container_core().expect("Object has no DataContainerCore").partition_guid
}

pub async fn get_dc_instance_guid(obj: &LockedTypeObject) -> Guid {
    let obj = obj.lock().await;
    obj.data_container_core().expect("Object has no DataContainerCore").instance_guid
}
