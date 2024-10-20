use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PeerServerBackendData {
    pub create_parameters: PeerCreateGameParameters,
}

pub const PEERSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerServerBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CreateParameters",
                flags: MemberInfoFlags::new(0),
                field_type: PEERCREATEGAMEPARAMETERS_TYPE_INFO,
                rust_offset: offset_of!(PeerServerBackendData, create_parameters),
            },
        ],
    }),
    array_type: Some(PEERSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PeerServerBackendData {
    fn type_info() -> &'static TypeInfo {
        PEERSERVERBACKENDDATA_TYPE_INFO
    }
}


pub const PEERSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerServerBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerServerBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetClientHostMigrationDataMessageBase {
}

pub const PRESENCEGETCLIENTHOSTMIGRATIONDATAMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetClientHostMigrationDataMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGetClientHostMigrationDataMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETCLIENTHOSTMIGRATIONDATAMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceServerPeerNotificationMessageBase {
}

pub const PRESENCESERVERPEERNOTIFICATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServerPeerNotificationMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceServerPeerNotificationMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCESERVERPEERNOTIFICATIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceServerPeerRequestMessageBase {
}

pub const PRESENCESERVERPEERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServerPeerRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceServerPeerRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCESERVERPEERREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePeerGameRequestMessageBase {
}

pub const PRESENCEPEERGAMEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerGameRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePeerGameRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPEERGAMEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePeerGameMessageBase {
}

pub const PRESENCEPEERGAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerGameMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePeerGameMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPEERGAMEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceHostMigrationRestoreFromSnapshotMessage {
}

pub const PRESENCEHOSTMIGRATIONRESTOREFROMSNAPSHOTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationRestoreFromSnapshotMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationRestoreFromSnapshotMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONRESTOREFROMSNAPSHOTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceHostMigrationStoreDataForCheckpointMessage {
}

pub const PRESENCEHOSTMIGRATIONSTOREDATAFORCHECKPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationStoreDataForCheckpointMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationStoreDataForCheckpointMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONSTOREDATAFORCHECKPOINTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceHostMigrationMessage {
}

pub const PRESENCEHOSTMIGRATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceHostMigrationClearCheckpointDataMessage {
}

pub const PRESENCEHOSTMIGRATIONCLEARCHECKPOINTDATAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationClearCheckpointDataMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationClearCheckpointDataMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONCLEARCHECKPOINTDATAMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceHostMigrationCheckpointMessage {
}

pub const PRESENCEHOSTMIGRATIONCHECKPOINTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHostMigrationCheckpointMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceHostMigrationCheckpointMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCEHOSTMIGRATIONCHECKPOINTMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum HostMigrationMessageType {
    #[default]
    HostMigration_Host = 0,
    HostMigration_Client = 1,
}

pub const HOSTMIGRATIONMESSAGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HostMigrationMessageType",
    flags: MemberInfoFlags::new(49429),
    module: "Peer",
    data: TypeInfoData::Enum,
    array_type: Some(HOSTMIGRATIONMESSAGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HostMigrationMessageType {
    fn type_info() -> &'static TypeInfo {
        HOSTMIGRATIONMESSAGETYPE_TYPE_INFO
    }
}


pub const HOSTMIGRATIONMESSAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HostMigrationMessageType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("HostMigrationMessageType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PeerCreateGameParameters {
    pub base: super::game_management::GameParametersData,
    pub player_capacity: u32,
}

pub const PEERCREATEGAMEPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerCreateGameParameters",
    flags: MemberInfoFlags::new(73),
    module: "Peer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Base",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPARAMETERSDATA_TYPE_INFO,
                rust_offset: offset_of!(PeerCreateGameParameters, base),
            },
            FieldInfoData {
                name: "PlayerCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PeerCreateGameParameters, player_capacity),
            },
        ],
    }),
    array_type: Some(PEERCREATEGAMEPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PeerCreateGameParameters {
    fn type_info() -> &'static TypeInfo {
        PEERCREATEGAMEPARAMETERS_TYPE_INFO
    }
}


pub const PEERCREATEGAMEPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerCreateGameParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerCreateGameParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePeerServiceData {
}

pub const PRESENCEPEERSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPEERSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePeerServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPEERSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEPEERSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePeerServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PresencePeerServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PeerOnlineManager {
}

pub const PEERONLINEMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerOnlineManager",
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ONLINEMANAGER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PEERONLINEMANAGER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PeerOnlineManager {
    fn type_info() -> &'static TypeInfo {
        PEERONLINEMANAGER_TYPE_INFO
    }
}


pub const PEERONLINEMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PeerOnlineManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("PeerOnlineManager-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPeerService {
}

pub const CLIENTPEERSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerService",
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPEERSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPeerService {
    fn type_info() -> &'static TypeInfo {
        CLIENTPEERSERVICE_TYPE_INFO
    }
}


pub const CLIENTPEERSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("ClientPeerService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPeerGameManagementBackend {
}

pub const CLIENTPEERGAMEMANAGEMENTBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerGameManagementBackend",
    flags: MemberInfoFlags::new(101),
    module: "Peer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPEERGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPeerGameManagementBackend {
    fn type_info() -> &'static TypeInfo {
        CLIENTPEERGAMEMANAGEMENTBACKEND_TYPE_INFO
    }
}


pub const CLIENTPEERGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerGameManagementBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Peer",
    data: TypeInfoData::Array("ClientPeerGameManagementBackend-Array"),
    array_type: None,
    alignment: 8,
};


