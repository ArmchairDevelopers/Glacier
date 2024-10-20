use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_game_management_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERGAMEMANAGERADDQUEUEDPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERGAMEDESTRUCTINGMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERPLAYERREMOVEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERPLAYERJOINCOMPLETEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERPLAYERJOININGMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERUPDATECAPACITYMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERGAMEPARAMETERSCHANGEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERCHANGEGAMEPARAMETERSMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERRECONFIGURABLEGAMECREATEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERCREATINGRECONFIGURABLEGAMEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERGAMERESETMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERGAMECREATEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGERCREATINGGAMEMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGEMENTBACKENDDATA_TYPE_INFO);
    registry.register_type(SERVERGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPARAMETERSDATA_TYPE_INFO);
    registry.register_type(GAMEPARAMETERSDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEROLEINFORMATION_TYPE_INFO);
    registry.register_type(GAMEROLEINFORMATION_ARRAY_TYPE_INFO);
    registry.register_type(GAMEATTRIBUTEDATA_TYPE_INFO);
    registry.register_type(GAMEATTRIBUTEDATA_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPEER2PEERMODE_TYPE_INFO);
    registry.register_type(GAMEPEER2PEERMODE_ARRAY_TYPE_INFO);
    registry.register_type(GAMENETWORKTOPOLOGY_TYPE_INFO);
    registry.register_type(GAMENETWORKTOPOLOGY_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGAMEMANAGEMENTSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEGAMEMANAGEMENTSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGAMEMANAGEMENTBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGAMEQUEUEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEGAMENOTIFICATIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEGAMEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEGAMEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(GAMEPARAMETERS_TYPE_INFO);
    registry.register_type(GAMEPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCECREATEGAMEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCECREATEGAMEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMEMANAGEMENTSERVICE_TYPE_INFO);
    registry.register_type(CLIENTGAMEMANAGEMENTSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO);
    registry.register_type(CLIENTGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerAddQueuedPlayerMessage {
}

pub const SERVERGAMEMANAGERADDQUEUEDPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerAddQueuedPlayerMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerAddQueuedPlayerMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERADDQUEUEDPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerGameDestructingMessage {
}

pub const SERVERGAMEMANAGERGAMEDESTRUCTINGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameDestructingMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerGameDestructingMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMEDESTRUCTINGMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerPlayerRemovedMessageBase {
}

pub const SERVERGAMEMANAGERPLAYERREMOVEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerPlayerRemovedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerPlayerRemovedMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERPLAYERREMOVEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerPlayerJoinCompleteMessage {
}

pub const SERVERGAMEMANAGERPLAYERJOINCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerPlayerJoinCompleteMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerPlayerJoinCompleteMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERPLAYERJOINCOMPLETEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerPlayerJoiningMessageBase {
}

pub const SERVERGAMEMANAGERPLAYERJOININGMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerPlayerJoiningMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerPlayerJoiningMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERPLAYERJOININGMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerUpdateCapacityMessageBase {
}

pub const SERVERGAMEMANAGERUPDATECAPACITYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerUpdateCapacityMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerUpdateCapacityMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERUPDATECAPACITYMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerGameParametersChangedMessageBase {
}

pub const SERVERGAMEMANAGERGAMEPARAMETERSCHANGEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameParametersChangedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerGameParametersChangedMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMEPARAMETERSCHANGEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerChangeGameParametersMessageBase {
}

pub const SERVERGAMEMANAGERCHANGEGAMEPARAMETERSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerChangeGameParametersMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerChangeGameParametersMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERCHANGEGAMEPARAMETERSMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerReconfigurableGameCreatedMessage {
}

pub const SERVERGAMEMANAGERRECONFIGURABLEGAMECREATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerReconfigurableGameCreatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerReconfigurableGameCreatedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERRECONFIGURABLEGAMECREATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerCreatingReconfigurableGameMessage {
}

pub const SERVERGAMEMANAGERCREATINGRECONFIGURABLEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerCreatingReconfigurableGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerCreatingReconfigurableGameMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERCREATINGRECONFIGURABLEGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerGameResetMessageBase {
}

pub const SERVERGAMEMANAGERGAMERESETMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameResetMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerGameResetMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMERESETMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerGameCreatedMessageBase {
}

pub const SERVERGAMEMANAGERGAMECREATEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameCreatedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerGameCreatedMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMECREATEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagerCreatingGameMessageBase {
}

pub const SERVERGAMEMANAGERCREATINGGAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerCreatingGameMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerCreatingGameMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGERCREATINGGAMEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameManagementBackendData {
    pub create_parameters: GameParametersData,
}

pub const SERVERGAMEMANAGEMENTBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagementBackendData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CreateParameters",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPARAMETERSDATA_TYPE_INFO,
                rust_offset: offset_of!(ServerGameManagementBackendData, create_parameters),
            },
        ],
    }),
    array_type: Some(SERVERGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ServerGameManagementBackendData {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEMANAGEMENTBACKENDDATA_TYPE_INFO
    }
}


pub const SERVERGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagementBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("ServerGameManagementBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameParametersData {
    pub server_name: String,
    pub game_type: super::game_shared::PersistenceGameType,
    pub open_to_invites: bool,
    pub open_to_matchmaking: bool,
    pub open_to_join_by_player: bool,
    pub open_to_browsing: bool,
    pub ranked: bool,
    pub host_migratable: bool,
    pub join_in_progress_supported: bool,
    pub allow_any_reputation: bool,
    pub dynamic_reputation_requirement: bool,
    pub friends_bypass_closed_to_join_by_player: bool,
    pub queue_capacity: u32,
    pub r#mod: u32,
    pub max_spectators: u32,
    pub max_allowed_capacity: u32,
    pub default_max_players: u32,
    pub game_topology: GameNetworkTopology,
    pub peer_mode: GamePeer2PeerMode,
    pub voip_topology: GameNetworkTopology,
    pub attributes: Vec<GameAttributeData>,
    pub role_configuration: Vec<GameRoleInformation>,
    pub role: String,
    pub teams: i32,
}

pub const GAMEPARAMETERSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParametersData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ServerName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, server_name),
            },
            FieldInfoData {
                name: "GameType",
                flags: MemberInfoFlags::new(0),
                field_type: PERSISTENCEGAMETYPE_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, game_type),
            },
            FieldInfoData {
                name: "OpenToInvites",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, open_to_invites),
            },
            FieldInfoData {
                name: "OpenToMatchmaking",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, open_to_matchmaking),
            },
            FieldInfoData {
                name: "OpenToJoinByPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, open_to_join_by_player),
            },
            FieldInfoData {
                name: "OpenToBrowsing",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, open_to_browsing),
            },
            FieldInfoData {
                name: "Ranked",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, ranked),
            },
            FieldInfoData {
                name: "HostMigratable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, host_migratable),
            },
            FieldInfoData {
                name: "JoinInProgressSupported",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, join_in_progress_supported),
            },
            FieldInfoData {
                name: "AllowAnyReputation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, allow_any_reputation),
            },
            FieldInfoData {
                name: "DynamicReputationRequirement",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, dynamic_reputation_requirement),
            },
            FieldInfoData {
                name: "FriendsBypassClosedToJoinByPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, friends_bypass_closed_to_join_by_player),
            },
            FieldInfoData {
                name: "QueueCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, queue_capacity),
            },
            FieldInfoData {
                name: "Mod",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, r#mod),
            },
            FieldInfoData {
                name: "MaxSpectators",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, max_spectators),
            },
            FieldInfoData {
                name: "MaxAllowedCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, max_allowed_capacity),
            },
            FieldInfoData {
                name: "DefaultMaxPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, default_max_players),
            },
            FieldInfoData {
                name: "GameTopology",
                flags: MemberInfoFlags::new(0),
                field_type: GAMENETWORKTOPOLOGY_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, game_topology),
            },
            FieldInfoData {
                name: "PeerMode",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPEER2PEERMODE_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, peer_mode),
            },
            FieldInfoData {
                name: "VoipTopology",
                flags: MemberInfoFlags::new(0),
                field_type: GAMENETWORKTOPOLOGY_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, voip_topology),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEATTRIBUTEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, attributes),
            },
            FieldInfoData {
                name: "RoleConfiguration",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEROLEINFORMATION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, role_configuration),
            },
            FieldInfoData {
                name: "Role",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, role),
            },
            FieldInfoData {
                name: "Teams",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameParametersData, teams),
            },
        ],
    }),
    array_type: Some(GAMEPARAMETERSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameParametersData {
    fn type_info() -> &'static TypeInfo {
        GAMEPARAMETERSDATA_TYPE_INFO
    }
}


pub const GAMEPARAMETERSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParametersData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameParametersData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameRoleInformation {
    pub role_name: String,
    pub capacity: u32,
}

pub const GAMEROLEINFORMATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameRoleInformation",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RoleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameRoleInformation, role_name),
            },
            FieldInfoData {
                name: "Capacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameRoleInformation, capacity),
            },
        ],
    }),
    array_type: Some(GAMEROLEINFORMATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameRoleInformation {
    fn type_info() -> &'static TypeInfo {
        GAMEROLEINFORMATION_TYPE_INFO
    }
}


pub const GAMEROLEINFORMATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameRoleInformation-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameRoleInformation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameAttributeData {
    pub attribute: String,
    pub value: String,
}

pub const GAMEATTRIBUTEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAttributeData",
    flags: MemberInfoFlags::new(73),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Attribute",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameAttributeData, attribute),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(GameAttributeData, value),
            },
        ],
    }),
    array_type: Some(GAMEATTRIBUTEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAttributeData {
    fn type_info() -> &'static TypeInfo {
        GAMEATTRIBUTEDATA_TYPE_INFO
    }
}


pub const GAMEATTRIBUTEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAttributeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameAttributeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GamePeer2PeerMode {
    #[default]
    GamePeer2PeerMode_FullMesh = 0,
    GamePeer2PeerMode_PartialMesh = 1,
}

pub const GAMEPEER2PEERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePeer2PeerMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameManagement",
    data: TypeInfoData::Enum,
    array_type: Some(GAMEPEER2PEERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GamePeer2PeerMode {
    fn type_info() -> &'static TypeInfo {
        GAMEPEER2PEERMODE_TYPE_INFO
    }
}


pub const GAMEPEER2PEERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePeer2PeerMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GamePeer2PeerMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GameNetworkTopology {
    #[default]
    GameNetworkTopology_Disabled = 0,
    GameNetworkTopology_Peer2Peer = 1,
    GameNetworkTopology_PeerHosted = 2,
    GameNetworkTopology_DedicatedServer = 3,
}

pub const GAMENETWORKTOPOLOGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameNetworkTopology",
    flags: MemberInfoFlags::new(49429),
    module: "GameManagement",
    data: TypeInfoData::Enum,
    array_type: Some(GAMENETWORKTOPOLOGY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GameNetworkTopology {
    fn type_info() -> &'static TypeInfo {
        GAMENETWORKTOPOLOGY_TYPE_INFO
    }
}


pub const GAMENETWORKTOPOLOGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameNetworkTopology-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameNetworkTopology-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGameManagementServiceData {
    pub gamegroup_max_participants: i32,
    pub join_game_session_after_gamegroup: bool,
    pub default_game_role_name: String,
}

pub const PRESENCEGAMEMANAGEMENTSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementServiceData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GamegroupMaxParticipants",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PresenceGameManagementServiceData, gamegroup_max_participants),
            },
            FieldInfoData {
                name: "JoinGameSessionAfterGamegroup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PresenceGameManagementServiceData, join_game_session_after_gamegroup),
            },
            FieldInfoData {
                name: "DefaultGameRoleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PresenceGameManagementServiceData, default_game_role_name),
            },
        ],
    }),
    array_type: Some(PRESENCEGAMEMANAGEMENTSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceGameManagementServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGAMEMANAGEMENTSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEGAMEMANAGEMENTSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("PresenceGameManagementServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGameManagementBackendData {
    pub use_game_info_strategy: bool,
}

pub const PRESENCEGAMEMANAGEMENTBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementBackendData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UseGameInfoStrategy",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PresenceGameManagementBackendData, use_game_info_strategy),
            },
        ],
    }),
    array_type: Some(PRESENCEGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceGameManagementBackendData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGAMEMANAGEMENTBACKENDDATA_TYPE_INFO
    }
}


pub const PRESENCEGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("PresenceGameManagementBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGameQueueMessageBase {
}

pub const PRESENCEGAMEQUEUEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameQueueMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameQueueMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGAMEQUEUEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGameNotificationMessageBase {
}

pub const PRESENCEGAMENOTIFICATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameNotificationMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameNotificationMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGAMENOTIFICATIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGameMessageBase {
}

pub const PRESENCEGAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGAMEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGameRequestMessageBase {
}

pub const PRESENCEGAMEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGAMEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameParameters {
}

pub const GAMEPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParameters",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(GAMEPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameParameters {
    fn type_info() -> &'static TypeInfo {
        GAMEPARAMETERS_TYPE_INFO
    }
}


pub const GAMEPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceCreateGameRequestParameters {
}

pub const PRESENCECREATEGAMEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreateGameRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECREATEGAMEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceCreateGameRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCECREATEGAMEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCECREATEGAMEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreateGameRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("PresenceCreateGameRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameManagementService {
}

pub const CLIENTGAMEMANAGEMENTSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementService",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEMANAGEMENTSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameManagementService {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEMANAGEMENTSERVICE_TYPE_INFO
    }
}


pub const CLIENTGAMEMANAGEMENTSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementService-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("ClientGameManagementService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameManagementBackend {
}

pub const CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementBackend",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameManagementBackend {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO
    }
}


pub const CLIENTGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("ClientGameManagementBackend-Array"),
    array_type: None,
    alignment: 8,
};


