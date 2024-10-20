use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct PeerServerBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub create_parameters: PeerCreateGameParameters,
}

pub trait PeerServerBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn create_parameters(&self) -> &PeerCreateGameParameters;
}

impl PeerServerBackendDataTrait for PeerServerBackendData {
    fn create_parameters(&self) -> &PeerCreateGameParameters {
        &self.create_parameters
    }
}

impl super::online_shared::PresenceBackendDataTrait for PeerServerBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
}

impl super::core::AssetTrait for PeerServerBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PeerServerBackendData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PEERSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerServerBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PeerServerBackendData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CreateParameters",
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
}


pub static PEERSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerServerBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerServerBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGetClientHostMigrationDataMessageBase {
}

pub trait PresenceGetClientHostMigrationDataMessageBaseTrait: TypeObject {
}

impl PresenceGetClientHostMigrationDataMessageBaseTrait for PresenceGetClientHostMigrationDataMessageBase {
}

pub static PRESENCEGETCLIENTHOSTMIGRATIONDATAMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetClientHostMigrationDataMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetClientHostMigrationDataMessageBase as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresenceServerPeerNotificationMessageBase {
}

pub trait PresenceServerPeerNotificationMessageBaseTrait: TypeObject {
}

impl PresenceServerPeerNotificationMessageBaseTrait for PresenceServerPeerNotificationMessageBase {
}

pub static PRESENCESERVERPEERNOTIFICATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServerPeerNotificationMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceServerPeerNotificationMessageBase as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresenceServerPeerRequestMessageBase {
}

pub trait PresenceServerPeerRequestMessageBaseTrait: TypeObject {
}

impl PresenceServerPeerRequestMessageBaseTrait for PresenceServerPeerRequestMessageBase {
}

pub static PRESENCESERVERPEERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServerPeerRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceServerPeerRequestMessageBase as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresencePeerGameRequestMessageBase {
}

pub trait PresencePeerGameRequestMessageBaseTrait: TypeObject {
}

impl PresencePeerGameRequestMessageBaseTrait for PresencePeerGameRequestMessageBase {
}

pub static PRESENCEPEERGAMEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerGameRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePeerGameRequestMessageBase as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresencePeerGameMessageBase {
}

pub trait PresencePeerGameMessageBaseTrait: TypeObject {
}

impl PresencePeerGameMessageBaseTrait for PresencePeerGameMessageBase {
}

pub static PRESENCEPEERGAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerGameMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePeerGameMessageBase as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresenceHostMigrationRestoreFromSnapshotMessage {
}

pub trait PresenceHostMigrationRestoreFromSnapshotMessageTrait: TypeObject {
}

impl PresenceHostMigrationRestoreFromSnapshotMessageTrait for PresenceHostMigrationRestoreFromSnapshotMessage {
}

pub static PRESENCEHOSTMIGRATIONRESTOREFROMSNAPSHOTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationRestoreFromSnapshotMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationRestoreFromSnapshotMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresenceHostMigrationStoreDataForCheckpointMessage {
}

pub trait PresenceHostMigrationStoreDataForCheckpointMessageTrait: TypeObject {
}

impl PresenceHostMigrationStoreDataForCheckpointMessageTrait for PresenceHostMigrationStoreDataForCheckpointMessage {
}

pub static PRESENCEHOSTMIGRATIONSTOREDATAFORCHECKPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationStoreDataForCheckpointMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationStoreDataForCheckpointMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresenceHostMigrationMessage {
}

pub trait PresenceHostMigrationMessageTrait: TypeObject {
}

impl PresenceHostMigrationMessageTrait for PresenceHostMigrationMessage {
}

pub static PRESENCEHOSTMIGRATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresenceHostMigrationClearCheckpointDataMessage {
}

pub trait PresenceHostMigrationClearCheckpointDataMessageTrait: TypeObject {
}

impl PresenceHostMigrationClearCheckpointDataMessageTrait for PresenceHostMigrationClearCheckpointDataMessage {
}

pub static PRESENCEHOSTMIGRATIONCLEARCHECKPOINTDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationClearCheckpointDataMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationClearCheckpointDataMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresenceHostMigrationCheckpointMessage {
}

pub trait PresenceHostMigrationCheckpointMessageTrait: TypeObject {
}

impl PresenceHostMigrationCheckpointMessageTrait for PresenceHostMigrationCheckpointMessage {
}

pub static PRESENCEHOSTMIGRATIONCHECKPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationCheckpointMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHostMigrationCheckpointMessage as Default>::default())),
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
}


pub static HOSTMIGRATIONMESSAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HostMigrationMessageType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("HostMigrationMessageType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PeerCreateGameParameters {
    pub base: Option<Arc<Mutex<dyn super::game_management::GameParametersDataTrait>>>,
    pub player_capacity: u32,
}

pub trait PeerCreateGameParametersTrait: TypeObject {
    fn base(&self) -> &Option<Arc<Mutex<dyn super::game_management::GameParametersDataTrait>>>;
    fn player_capacity(&self) -> &u32;
}

impl PeerCreateGameParametersTrait for PeerCreateGameParameters {
    fn base(&self) -> &Option<Arc<Mutex<dyn super::game_management::GameParametersDataTrait>>> {
        &self.base
    }
    fn player_capacity(&self) -> &u32 {
        &self.player_capacity
    }
}

pub static PEERCREATEGAMEPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerCreateGameParameters",
    flags: MemberInfoFlags::new(73),
    module: "Peer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PeerCreateGameParameters as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Base",
                flags: MemberInfoFlags::new(0),
                field_type: "GameParametersData",
                rust_offset: offset_of!(PeerCreateGameParameters, base),
            },
            FieldInfoData {
                name: "PlayerCapacity",
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
}


pub static PEERCREATEGAMEPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerCreateGameParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerCreateGameParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::DataContainerTrait for PresencePeerServiceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESENCEPEERSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePeerServiceData as Default>::default())),
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
}


pub static PRESENCEPEERSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PresencePeerServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::ONLINEMANAGER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PeerOnlineManager as Default>::default())),
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
}


pub static PEERONLINEMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerOnlineManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerOnlineManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPeerService as Default>::default())),
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
}


pub static CLIENTPEERSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("ClientPeerService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_management::CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPeerGameManagementBackend as Default>::default())),
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
}


pub static CLIENTPEERGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerGameManagementBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("ClientPeerGameManagementBackend"),
    array_type: None,
    alignment: 8,
};


