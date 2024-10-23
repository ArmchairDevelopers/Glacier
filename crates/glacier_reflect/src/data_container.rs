use glacier_util::guid::Guid;

#[derive(Debug, Default)]
pub struct DataContainerCore {
    pub partition_guid: Guid,
    pub instance_guid: Guid,
    pub exported: bool,
}
