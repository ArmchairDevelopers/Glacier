use glacier_util::guid::Guid;

#[derive(Clone, Debug, Default)]
pub struct DataContainerCore {
    pub partition_guid: Guid,
    pub instance_guid: Guid,
}
