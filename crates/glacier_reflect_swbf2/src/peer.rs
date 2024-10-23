use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_peer_types(registry: &mut TypeRegistry) {
    registry.register_type(PEERSERVERBACKENDDATA_TYPE_INFO);
    registry.register_type(PEERSERVERBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETCLIENTHOSTMIGRATIONDATAMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCESERVERPEERNOTIFICATIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCESERVERPEERREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPEERGAMEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPEERGAMEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEHOSTMIGRATIONRESTOREFROMSNAPSHOTMESSAGE_TYPE_INFO);
    registry.register_type(PRESENCEHOSTMIGRATIONSTOREDATAFORCHECKPOINTMESSAGE_TYPE_INFO);
    registry.register_type(PRESENCEHOSTMIGRATIONMESSAGE_TYPE_INFO);
    registry.register_type(PRESENCEHOSTMIGRATIONCLEARCHECKPOINTDATAMESSAGE_TYPE_INFO);
    registry.register_type(PRESENCEHOSTMIGRATIONCHECKPOINTMESSAGE_TYPE_INFO);
    registry.register_type(HOSTMIGRATIONMESSAGETYPE_TYPE_INFO);
    registry.register_type(HOSTMIGRATIONMESSAGETYPE_ARRAY_TYPE_INFO);
    registry.register_type(PEERCREATEGAMEPARAMETERS_TYPE_INFO);
    registry.register_type(PEERCREATEGAMEPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPEERSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEPEERSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PEERONLINEMANAGER_TYPE_INFO);
    registry.register_type(PEERONLINEMANAGER_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPEERSERVICE_TYPE_INFO);
    registry.register_type(CLIENTPEERSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPEERGAMEMANAGEMENTBACKEND_TYPE_INFO);
    registry.register_type(CLIENTPEERGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PeerServerBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub create_parameters: PeerCreateGameParameters,
}

pub trait PeerServerBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn create_parameters(&self) -> &PeerCreateGameParameters;
    fn create_parameters_mut(&mut self) -> &mut PeerCreateGameParameters;
}

impl PeerServerBackendDataTrait for PeerServerBackendData {
    fn create_parameters(&self) -> &PeerCreateGameParameters {
        &self.create_parameters
    }
    fn create_parameters_mut(&mut self) -> &mut PeerCreateGameParameters {
        &mut self.create_parameters
    }
}

impl super::online_shared::PresenceBackendDataTrait for PeerServerBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for PeerServerBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PeerServerBackendData {
}

pub static PEERSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerServerBackendData",
    name_hash: 2916502166,
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(PeerServerBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PeerServerBackendData as Default>::default())),
            create_boxed: || Box::new(<PeerServerBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CreateParameters",
                name_hash: 4233299195,
                flags: MemberInfoFlags::new(0),
                field_type: "PeerCreateGameParameters",
                rust_offset: offset_of!(PeerServerBackendData, create_parameters),
            },
        ],
    }),
    array_type: Some(PEERSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PeerServerBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PEERSERVERBACKENDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PEERSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerServerBackendData-Array",
    name_hash: 4162944930,
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerServerBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceGetClientHostMigrationDataMessageBase {
}

pub trait PresenceGetClientHostMigrationDataMessageBaseTrait: TypeObject {
}

impl PresenceGetClientHostMigrationDataMessageBaseTrait for PresenceGetClientHostMigrationDataMessageBase {
}

pub static PRESENCEGETCLIENTHOSTMIGRATIONDATAMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetClientHostMigrationDataMessageBase",
    name_hash: 3592834705,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetClientHostMigrationDataMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceGetClientHostMigrationDataMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGetClientHostMigrationDataMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETCLIENTHOSTMIGRATIONDATAMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceServerPeerNotificationMessageBase {
}

pub trait PresenceServerPeerNotificationMessageBaseTrait: TypeObject {
}

impl PresenceServerPeerNotificationMessageBaseTrait for PresenceServerPeerNotificationMessageBase {
}

pub static PRESENCESERVERPEERNOTIFICATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServerPeerNotificationMessageBase",
    name_hash: 216376232,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceServerPeerNotificationMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceServerPeerNotificationMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceServerPeerNotificationMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESERVERPEERNOTIFICATIONMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceServerPeerRequestMessageBase {
}

pub trait PresenceServerPeerRequestMessageBaseTrait: TypeObject {
}

impl PresenceServerPeerRequestMessageBaseTrait for PresenceServerPeerRequestMessageBase {
}

pub static PRESENCESERVERPEERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServerPeerRequestMessageBase",
    name_hash: 2400360596,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceServerPeerRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceServerPeerRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceServerPeerRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESERVERPEERREQUESTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePeerGameRequestMessageBase {
}

pub trait PresencePeerGameRequestMessageBaseTrait: TypeObject {
}

impl PresencePeerGameRequestMessageBaseTrait for PresencePeerGameRequestMessageBase {
}

pub static PRESENCEPEERGAMEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerGameRequestMessageBase",
    name_hash: 3609031199,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePeerGameRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePeerGameRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePeerGameRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPEERGAMEREQUESTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePeerGameMessageBase {
}

pub trait PresencePeerGameMessageBaseTrait: TypeObject {
}

impl PresencePeerGameMessageBaseTrait for PresencePeerGameMessageBase {
}

pub static PRESENCEPEERGAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerGameMessageBase",
    name_hash: 435666830,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePeerGameMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePeerGameMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePeerGameMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPEERGAMEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceHostMigrationRestoreFromSnapshotMessage {
}

pub trait PresenceHostMigrationRestoreFromSnapshotMessageTrait: TypeObject {
}

impl PresenceHostMigrationRestoreFromSnapshotMessageTrait for PresenceHostMigrationRestoreFromSnapshotMessage {
}

pub static PRESENCEHOSTMIGRATIONRESTOREFROMSNAPSHOTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationRestoreFromSnapshotMessage",
    name_hash: 3723672425,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationRestoreFromSnapshotMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceHostMigrationRestoreFromSnapshotMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationRestoreFromSnapshotMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONRESTOREFROMSNAPSHOTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceHostMigrationStoreDataForCheckpointMessage {
}

pub trait PresenceHostMigrationStoreDataForCheckpointMessageTrait: TypeObject {
}

impl PresenceHostMigrationStoreDataForCheckpointMessageTrait for PresenceHostMigrationStoreDataForCheckpointMessage {
}

pub static PRESENCEHOSTMIGRATIONSTOREDATAFORCHECKPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationStoreDataForCheckpointMessage",
    name_hash: 1705674085,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationStoreDataForCheckpointMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceHostMigrationStoreDataForCheckpointMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationStoreDataForCheckpointMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONSTOREDATAFORCHECKPOINTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceHostMigrationMessage {
}

pub trait PresenceHostMigrationMessageTrait: TypeObject {
}

impl PresenceHostMigrationMessageTrait for PresenceHostMigrationMessage {
}

pub static PRESENCEHOSTMIGRATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationMessage",
    name_hash: 2561951867,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceHostMigrationMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceHostMigrationClearCheckpointDataMessage {
}

pub trait PresenceHostMigrationClearCheckpointDataMessageTrait: TypeObject {
}

impl PresenceHostMigrationClearCheckpointDataMessageTrait for PresenceHostMigrationClearCheckpointDataMessage {
}

pub static PRESENCEHOSTMIGRATIONCLEARCHECKPOINTDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationClearCheckpointDataMessage",
    name_hash: 3408745784,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationClearCheckpointDataMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceHostMigrationClearCheckpointDataMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationClearCheckpointDataMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONCLEARCHECKPOINTDATAMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceHostMigrationCheckpointMessage {
}

pub trait PresenceHostMigrationCheckpointMessageTrait: TypeObject {
}

impl PresenceHostMigrationCheckpointMessageTrait for PresenceHostMigrationCheckpointMessage {
}

pub static PRESENCEHOSTMIGRATIONCHECKPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationCheckpointMessage",
    name_hash: 3148510289,
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationCheckpointMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceHostMigrationCheckpointMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationCheckpointMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONCHECKPOINTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum HostMigrationMessageType {
    #[default]
    HostMigration_Host = 0,
    HostMigration_Client = 1,
}

pub static HOSTMIGRATIONMESSAGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HostMigrationMessageType",
    name_hash: 1971694746,
    flags: MemberInfoFlags::new(49429),
    module: "Peer",
    data: TypeInfoData::Enum,
    array_type: Some(HOSTMIGRATIONMESSAGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HostMigrationMessageType {
    fn type_info(&self) -> &'static TypeInfo {
        HOSTMIGRATIONMESSAGETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static HOSTMIGRATIONMESSAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HostMigrationMessageType-Array",
    name_hash: 1938952110,
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("HostMigrationMessageType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PeerCreateGameParameters {
    pub base: Option<LockedTypeObject /* super::game_management::GameParametersData */>,
    pub player_capacity: u32,
}

pub trait PeerCreateGameParametersTrait: TypeObject {
    fn base(&self) -> &Option<LockedTypeObject /* super::game_management::GameParametersData */>;
    fn base_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_management::GameParametersData */>;
    fn player_capacity(&self) -> &u32;
    fn player_capacity_mut(&mut self) -> &mut u32;
}

impl PeerCreateGameParametersTrait for PeerCreateGameParameters {
    fn base(&self) -> &Option<LockedTypeObject /* super::game_management::GameParametersData */> {
        &self.base
    }
    fn base_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_management::GameParametersData */> {
        &mut self.base
    }
    fn player_capacity(&self) -> &u32 {
        &self.player_capacity
    }
    fn player_capacity_mut(&mut self) -> &mut u32 {
        &mut self.player_capacity
    }
}

pub static PEERCREATEGAMEPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerCreateGameParameters",
    name_hash: 2286244375,
    flags: MemberInfoFlags::new(73),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PeerCreateGameParameters as Default>::default())),
            create_boxed: || Box::new(<PeerCreateGameParameters as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Base",
                name_hash: 2088806864,
                flags: MemberInfoFlags::new(0),
                field_type: "GameParametersData",
                rust_offset: offset_of!(PeerCreateGameParameters, base),
            },
            FieldInfoData {
                name: "PlayerCapacity",
                name_hash: 646509826,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PeerCreateGameParameters, player_capacity),
            },
        ],
    }),
    array_type: Some(PEERCREATEGAMEPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PeerCreateGameParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PEERCREATEGAMEPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PEERCREATEGAMEPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerCreateGameParameters-Array",
    name_hash: 3136240291,
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerCreateGameParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePeerServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
}

pub trait PresencePeerServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
}

impl PresencePeerServiceDataTrait for PresencePeerServiceData {
}

impl super::online_shared::PresenceServiceDataTrait for PresencePeerServiceData {
}

impl super::core::AssetTrait for PresencePeerServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresencePeerServiceData {
}

pub static PRESENCEPEERSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerServiceData",
    name_hash: 2064621747,
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresencePeerServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePeerServiceData as Default>::default())),
            create_boxed: || Box::new(<PresencePeerServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPEERSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePeerServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPEERSERVICEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PRESENCEPEERSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerServiceData-Array",
    name_hash: 3813051399,
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PresencePeerServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PeerOnlineManager {
    pub _glacier_base: super::gameplay_client_server::OnlineManager,
}

pub trait PeerOnlineManagerTrait: super::gameplay_client_server::OnlineManagerTrait {
}

impl PeerOnlineManagerTrait for PeerOnlineManager {
}

impl super::gameplay_client_server::OnlineManagerTrait for PeerOnlineManager {
}

pub static PEERONLINEMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerOnlineManager",
    name_hash: 4023605627,
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::ONLINEMANAGER_TYPE_INFO),
        super_class_offset: offset_of!(PeerOnlineManager, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PeerOnlineManager as Default>::default())),
            create_boxed: || Box::new(<PeerOnlineManager as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PEERONLINEMANAGER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PeerOnlineManager {
    fn type_info(&self) -> &'static TypeInfo {
        PEERONLINEMANAGER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PEERONLINEMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerOnlineManager-Array",
    name_hash: 2162504015,
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerOnlineManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientPeerService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientPeerServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientPeerServiceTrait for ClientPeerService {
}

impl super::online::PresenceServiceTrait for ClientPeerService {
}

pub static CLIENTPEERSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerService",
    name_hash: 736516643,
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientPeerService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPeerService as Default>::default())),
            create_boxed: || Box::new(<ClientPeerService as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPEERSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPeerService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPEERSERVICE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPEERSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerService-Array",
    name_hash: 3759119767,
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("ClientPeerService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientPeerGameManagementBackend {
    pub _glacier_base: super::game_management::ClientGameManagementBackend,
}

pub trait ClientPeerGameManagementBackendTrait: super::game_management::ClientGameManagementBackendTrait {
}

impl ClientPeerGameManagementBackendTrait for ClientPeerGameManagementBackend {
}

impl super::game_management::ClientGameManagementBackendTrait for ClientPeerGameManagementBackend {
}

impl super::online::PresenceBackendTrait for ClientPeerGameManagementBackend {
}

pub static CLIENTPEERGAMEMANAGEMENTBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerGameManagementBackend",
    name_hash: 2180565671,
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_management::CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(ClientPeerGameManagementBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPeerGameManagementBackend as Default>::default())),
            create_boxed: || Box::new(<ClientPeerGameManagementBackend as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPEERGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPeerGameManagementBackend {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPEERGAMEMANAGEMENTBACKEND_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLIENTPEERGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerGameManagementBackend-Array",
    name_hash: 345250835,
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("ClientPeerGameManagementBackend"),
    array_type: None,
    alignment: 8,
};


