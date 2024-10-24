use glacier_util::guid::Guid;

#[derive(Debug, Default, Clone)]
pub struct DataContainerCore {
    /// Set manually to identify certain nodes in blueprints, etc.
    pub id: String,

    pub partition_guid: Guid,
    pub instance_guid: Guid,
    pub exported: bool,
}
