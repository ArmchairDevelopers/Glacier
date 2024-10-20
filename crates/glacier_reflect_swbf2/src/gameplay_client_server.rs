use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_gameplay_client_server_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERADMINISTRATIONRESTARTREQUIREDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPEERLOADLEVELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERPEERINITIALIZEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLEVELLOADNEXTLEVELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLEVELLOADNEXTROUNDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSUBLEVELONSTREAMEDINMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLEVELCOMPLETEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLEVELSTARTEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLEVELSPAWNENTITIESENDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLEVELSPAWNENTITIESBEGINMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERINPUTDEACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERINPUTREACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOMPONENTENTRYCOMPONENTPROCESSEDINPUTMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOMPONENTENTRYONPLAYEREXITSMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOMPONENTENTRYONPLAYERENTERSMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCOMPONENTENTRYONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCONNECTIONEXITMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCONNECTIONINITMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCLIENTCONNECTIONREMOVEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERCLIENTCONNECTIONCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERADMINBANPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(SERVERADMINSETSERVERNAMEMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSCRIPTTICKMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTOPMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERLOADLEVELMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERLEVELUNLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERUNLOADLEVELMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLEVELLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTOPPEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERAPPLYCONFIGURATIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERRESETCONFIGURATIONMESSAGE_TYPE_INFO);
    registry.register_type(SERVERSTARTEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERLOADGAMEMESSAGE_TYPE_INFO);
    registry.register_type(SAVEGAMEWRITTENTOMEDIAMESSAGE_TYPE_INFO);
    registry.register_type(SAVEGAMEGENERATEDMESSAGE_TYPE_INFO);
    registry.register_type(STATDISABLEMESSAGE_TYPE_INFO);
    registry.register_type(STATENABLEMESSAGE_TYPE_INFO);
    registry.register_type(SOUNDSETTINGS_TYPE_INFO);
    registry.register_type(SOUNDSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSETTINGS_TYPE_INFO);
    registry.register_type(CLIENTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSPAWNSPAWNEDORUNSPAWNEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEVELFINALIZEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEVELSPAWNDEBUGENTITIESMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONNECTIONUNLOADLEVELMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONNECTIONLINKLEVELMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONNECTIONLOADLEVELMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONNECTIONINITIALIZEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLACEHOLDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLACEHOLDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPHYSICSENTITYWITHPOSEPROVIDER_TYPE_INFO);
    registry.register_type(CLIENTPHYSICSENTITYWITHPOSEPROVIDER_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPHYSICSENTITY_TYPE_INFO);
    registry.register_type(CLIENTPHYSICSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMECOMPONENTENTITY_TYPE_INFO);
    registry.register_type(CLIENTGAMECOMPONENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTGAMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLEENTITY_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTENTRYCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTENTRYCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYEREXTENT_TYPE_INFO);
    registry.register_type(CLIENTPLAYEREXTENT_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEMANAGER_TYPE_INFO);
    registry.register_type(ONLINEMANAGER_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCONNECTION_TYPE_INFO);
    registry.register_type(CLIENTCONNECTION_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSPAWNENTITY_TYPE_INFO);
    registry.register_type(CLIENTSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYEREVENT_TYPE_INFO);
    registry.register_type(CLIENTPLAYEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDESTRUCTIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYERUPDATECAMERACOMPONENTMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERSWITCHTEAMMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERREQUESTCAMERACHANGEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERLOCALSETMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYEREXITENTRYMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERENTERENTRYMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERDELETEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERCONNECTMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPLAYERCHANGEDPLAYERVIEWMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSETSERVERPASSWORDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTWANTFULLSCREENMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEFTREMOTESERVERMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTDISCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTABORTCUTSCENEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEVELLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEVELLOADPROGRESSMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEVELDESCRIPTIONLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEVELUNLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLEVELUNLOADSTARTEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTQUITTINGFINISHEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTQUITTINGSTARTEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSTARTUPFINISHEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLOADLEVELMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTLOADLEVELREQUESTEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTENTEREDINGAMEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTENTERHUDINGAMEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTEXITGAMEMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTHOSTMIGRATIONMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTEXITTOMENUMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTRETURNTOMENUMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSTARTMULTIPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTCONTINUESINGLEPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTSTARTEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTJOINSERVERJOBMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTPEERNETWORKREMOVEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(CLIENTJOINMULTIPLAYERMESSAGEBASE_TYPE_INFO);
    registry.register_type(CLIENTRESTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO);
    registry.register_type(CLIENTSTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO);
    registry.register_type(SERVERDESTRUCTIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERGAMECOMPONENT_TYPE_INFO);
    registry.register_type(SERVERGAMECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMETERCOMPONENT_TYPE_INFO);
    registry.register_type(SHADERPARAMETERCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPARTCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTPARTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPART_TYPE_INFO);
    registry.register_type(CLIENTPART_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBONECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTBONECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSUBVIEW_TYPE_INFO);
    registry.register_type(CLIENTSUBVIEW_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdministrationRestartRequiredMessage {
}

pub const SERVERADMINISTRATIONRESTARTREQUIREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationRestartRequiredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationRestartRequiredMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINISTRATIONRESTARTREQUIREDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPeerLoadLevelMessage {
}

pub const SERVERPEERLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPeerLoadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPeerLoadLevelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPEERLOADLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerPeerInitializedMessage {
}

pub const SERVERPEERINITIALIZEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPeerInitializedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPeerInitializedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERPEERINITIALIZEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelLoadNextLevelMessage {
}

pub const SERVERLEVELLOADNEXTLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelLoadNextLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelLoadNextLevelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELLOADNEXTLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelLoadNextRoundMessage {
}

pub const SERVERLEVELLOADNEXTROUNDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelLoadNextRoundMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelLoadNextRoundMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELLOADNEXTROUNDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSubLevelOnStreamedInMessage {
}

pub const SERVERSUBLEVELONSTREAMEDINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelOnStreamedInMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSubLevelOnStreamedInMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSUBLEVELONSTREAMEDINMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelCompletedMessage {
}

pub const SERVERLEVELCOMPLETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelCompletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelCompletedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELCOMPLETEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelStartedMessage {
}

pub const SERVERLEVELSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelStartedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELSTARTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelSpawnEntitiesEndMessage {
}

pub const SERVERLEVELSPAWNENTITIESENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelSpawnEntitiesEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelSpawnEntitiesEndMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELSPAWNENTITIESENDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelSpawnEntitiesBeginMessage {
}

pub const SERVERLEVELSPAWNENTITIESBEGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelSpawnEntitiesBeginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelSpawnEntitiesBeginMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELSPAWNENTITIESBEGINMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSoundVoiceOverFinishedMessage {
}

pub const SERVERSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoundVoiceOverFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoundVoiceOverFinishedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerInputDeactivateInputRestrictionMessage {
}

pub const SERVERINPUTDEACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputDeactivateInputRestrictionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerInputDeactivateInputRestrictionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERINPUTDEACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerInputReactivateInputRestrictionMessage {
}

pub const SERVERINPUTREACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputReactivateInputRestrictionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerInputReactivateInputRestrictionMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERINPUTREACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerComponentEntryComponentProcessedInputMessage {
}

pub const SERVERCOMPONENTENTRYCOMPONENTPROCESSEDINPUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryComponentProcessedInputMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryComponentProcessedInputMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOMPONENTENTRYCOMPONENTPROCESSEDINPUTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerComponentEntryOnPlayerExitsMessage {
}

pub const SERVERCOMPONENTENTRYONPLAYEREXITSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryOnPlayerExitsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryOnPlayerExitsMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOMPONENTENTRYONPLAYEREXITSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerComponentEntryOnPlayerEntersMessage {
}

pub const SERVERCOMPONENTENTRYONPLAYERENTERSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryOnPlayerEntersMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryOnPlayerEntersMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOMPONENTENTRYONPLAYERENTERSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerComponentEntryOnUnspawnMessage {
}

pub const SERVERCOMPONENTENTRYONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryOnUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCOMPONENTENTRYONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerControllableUnspawnDoneMessage {
}

pub const SERVERCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableUnspawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableUnspawnDoneMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerControllableSpawnDoneMessage {
}

pub const SERVERCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableSpawnDoneMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerConnectionExitMessage {
}

pub const SERVERCONNECTIONEXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnectionExitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerConnectionExitMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCONNECTIONEXITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerConnectionInitMessage {
}

pub const SERVERCONNECTIONINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnectionInitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerConnectionInitMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCONNECTIONINITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerClientConnectionRemovedMessage {
}

pub const SERVERCLIENTCONNECTIONREMOVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClientConnectionRemovedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClientConnectionRemovedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCLIENTCONNECTIONREMOVEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerClientConnectionConnectedMessage {
}

pub const SERVERCLIENTCONNECTIONCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClientConnectionConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClientConnectionConnectedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERCLIENTCONNECTIONCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdminBanPlayerMessage {
}

pub const SERVERADMINBANPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdminBanPlayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdminBanPlayerMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINBANPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdminSetServerNameMessage {
}

pub const SERVERADMINSETSERVERNAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdminSetServerNameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdminSetServerNameMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINSETSERVERNAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerScriptTickMessage {
}

pub const SERVERSCRIPTTICKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerScriptTickMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerScriptTickMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSCRIPTTICKMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStopMessageBase {
}

pub const SERVERSTOPMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStopMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerStopMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERSTOPMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLoadLevelMessageBase {
}

pub const SERVERLOADLEVELMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLoadLevelMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerLoadLevelMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERLOADLEVELMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelUnloadedMessage {
}

pub const SERVERLEVELUNLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelUnloadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelUnloadedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELUNLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerUnloadLevelMessage {
}

pub const SERVERUNLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnloadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerUnloadLevelMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERUNLOADLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLevelLoadedMessage {
}

pub const SERVERLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelLoadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLEVELLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStoppedMessage {
}

pub const SERVERSTOPPEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStoppedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerStoppedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSTOPPEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerApplyConfigurationMessage {
}

pub const SERVERAPPLYCONFIGURATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerApplyConfigurationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerApplyConfigurationMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERAPPLYCONFIGURATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerResetConfigurationMessage {
}

pub const SERVERRESETCONFIGURATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerResetConfigurationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerResetConfigurationMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERRESETCONFIGURATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStartedMessage {
}

pub const SERVERSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerStartedMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERSTARTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerLoadGameMessage {
}

pub const SERVERLOADGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLoadGameMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLoadGameMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERLOADGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SaveGameWrittenToMediaMessage {
}

pub const SAVEGAMEWRITTENTOMEDIAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameWrittenToMediaMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SaveGameWrittenToMediaMessage {
    fn type_info() -> &'static TypeInfo {
        SAVEGAMEWRITTENTOMEDIAMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SaveGameGeneratedMessage {
}

pub const SAVEGAMEGENERATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameGeneratedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SaveGameGeneratedMessage {
    fn type_info() -> &'static TypeInfo {
        SAVEGAMEGENERATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatDisableMessage {
}

pub const STATDISABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatDisableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StatDisableMessage {
    fn type_info() -> &'static TypeInfo {
        STATDISABLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatEnableMessage {
}

pub const STATENABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatEnableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StatEnableMessage {
    fn type_info() -> &'static TypeInfo {
        STATENABLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoundSettings {
}

pub const SOUNDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SOUNDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundSettings {
    fn type_info() -> &'static TypeInfo {
        SOUNDSETTINGS_TYPE_INFO
    }
}


pub const SOUNDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("SoundSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClientSettings {
    pub is_spectator: bool,
    pub allow_video_recording: bool,
    pub debris_cluster_enabled: bool,
    pub vegetation_enabled: bool,
    pub force_enabled: bool,
    pub world_render_enabled: bool,
    pub terrain_enabled: bool,
    pub water_physics_enabled: bool,
    pub overgrowth_enabled: bool,
    pub effects_enabled: bool,
    pub auto_increment_pad_index: bool,
    pub lip_sync_enabled: bool,
    pub pause_game_on_start_up: bool,
    pub skip_fast_level_load: bool,
    pub screenshot_to_file: bool,
    pub screenshot_filename: String,
    pub screenshot_suffix: String,
    pub load_menu: bool,
    pub debug_menu_on_l_thumb: bool,
    pub screenshot_comparisons_enable: bool,
    pub render_tags: bool,
    pub team: u32,
    pub spawn_point_index: i32,
    pub server_ip: String,
    pub secondary_server_ip: String,
    pub scheme0_flip_y: bool,
    pub scheme1_flip_y: bool,
    pub scheme2_flip_y: bool,
    pub aim_scale: f32,
    pub havok_visual_debugger: bool,
    pub havok_capture_to_file: bool,
    pub show_build_id: bool,
    pub extract_persistence_information: bool,
    pub enable_rest_tool: bool,
    pub local_vehicle_simulation_enabled: bool,
    pub auto_unspawn_dynamic_objects: bool,
    pub incoming_frequency: f32,
    pub outgoing_frequency: f32,
    pub incoming_rate: u32,
    pub outgoing_rate: u32,
    pub loading_timeout: f32,
    pub loaded_timeout: f32,
    pub ingame_timeout: f32,
    pub quit_game_on_server_disconnect: bool,
    pub lua_option_set_enable: bool,
    pub cpu_quality: f32,
    pub instance_path: String,
    pub audio_system_guid: super::core::Guid,
    pub frame_history_time_warn_scale: f32,
    pub fast_exit: bool,
}

pub const CLIENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IsSpectator",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, is_spectator),
            },
            FieldInfoData {
                name: "AllowVideoRecording",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, allow_video_recording),
            },
            FieldInfoData {
                name: "DebrisClusterEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, debris_cluster_enabled),
            },
            FieldInfoData {
                name: "VegetationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, vegetation_enabled),
            },
            FieldInfoData {
                name: "ForceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, force_enabled),
            },
            FieldInfoData {
                name: "WorldRenderEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, world_render_enabled),
            },
            FieldInfoData {
                name: "TerrainEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, terrain_enabled),
            },
            FieldInfoData {
                name: "WaterPhysicsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, water_physics_enabled),
            },
            FieldInfoData {
                name: "OvergrowthEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, overgrowth_enabled),
            },
            FieldInfoData {
                name: "EffectsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, effects_enabled),
            },
            FieldInfoData {
                name: "AutoIncrementPadIndex",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, auto_increment_pad_index),
            },
            FieldInfoData {
                name: "LipSyncEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, lip_sync_enabled),
            },
            FieldInfoData {
                name: "PauseGameOnStartUp",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, pause_game_on_start_up),
            },
            FieldInfoData {
                name: "SkipFastLevelLoad",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, skip_fast_level_load),
            },
            FieldInfoData {
                name: "ScreenshotToFile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, screenshot_to_file),
            },
            FieldInfoData {
                name: "ScreenshotFilename",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, screenshot_filename),
            },
            FieldInfoData {
                name: "ScreenshotSuffix",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, screenshot_suffix),
            },
            FieldInfoData {
                name: "LoadMenu",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, load_menu),
            },
            FieldInfoData {
                name: "DebugMenuOnLThumb",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, debug_menu_on_l_thumb),
            },
            FieldInfoData {
                name: "ScreenshotComparisonsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, screenshot_comparisons_enable),
            },
            FieldInfoData {
                name: "RenderTags",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, render_tags),
            },
            FieldInfoData {
                name: "Team",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, team),
            },
            FieldInfoData {
                name: "SpawnPointIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, spawn_point_index),
            },
            FieldInfoData {
                name: "ServerIp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, server_ip),
            },
            FieldInfoData {
                name: "SecondaryServerIp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, secondary_server_ip),
            },
            FieldInfoData {
                name: "Scheme0FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, scheme0_flip_y),
            },
            FieldInfoData {
                name: "Scheme1FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, scheme1_flip_y),
            },
            FieldInfoData {
                name: "Scheme2FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, scheme2_flip_y),
            },
            FieldInfoData {
                name: "AimScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, aim_scale),
            },
            FieldInfoData {
                name: "HavokVisualDebugger",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, havok_visual_debugger),
            },
            FieldInfoData {
                name: "HavokCaptureToFile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, havok_capture_to_file),
            },
            FieldInfoData {
                name: "ShowBuildId",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, show_build_id),
            },
            FieldInfoData {
                name: "ExtractPersistenceInformation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, extract_persistence_information),
            },
            FieldInfoData {
                name: "EnableRestTool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, enable_rest_tool),
            },
            FieldInfoData {
                name: "LocalVehicleSimulationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, local_vehicle_simulation_enabled),
            },
            FieldInfoData {
                name: "AutoUnspawnDynamicObjects",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, auto_unspawn_dynamic_objects),
            },
            FieldInfoData {
                name: "IncomingFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, incoming_frequency),
            },
            FieldInfoData {
                name: "OutgoingFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, outgoing_frequency),
            },
            FieldInfoData {
                name: "IncomingRate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, incoming_rate),
            },
            FieldInfoData {
                name: "OutgoingRate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, outgoing_rate),
            },
            FieldInfoData {
                name: "LoadingTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, loading_timeout),
            },
            FieldInfoData {
                name: "LoadedTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, loaded_timeout),
            },
            FieldInfoData {
                name: "IngameTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, ingame_timeout),
            },
            FieldInfoData {
                name: "QuitGameOnServerDisconnect",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, quit_game_on_server_disconnect),
            },
            FieldInfoData {
                name: "LuaOptionSetEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, lua_option_set_enable),
            },
            FieldInfoData {
                name: "CpuQuality",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, cpu_quality),
            },
            FieldInfoData {
                name: "InstancePath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, instance_path),
            },
            FieldInfoData {
                name: "AudioSystemGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, audio_system_guid),
            },
            FieldInfoData {
                name: "FrameHistoryTimeWarnScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, frame_history_time_warn_scale),
            },
            FieldInfoData {
                name: "FastExit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClientSettings, fast_exit),
            },
        ],
    }),
    array_type: Some(CLIENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientSettings {
    fn type_info() -> &'static TypeInfo {
        CLIENTSETTINGS_TYPE_INFO
    }
}


pub const CLIENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSpawnSpawnedOrUnSpawnedMessage {
}

pub const CLIENTSPAWNSPAWNEDORUNSPAWNEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpawnSpawnedOrUnSpawnedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSpawnSpawnedOrUnSpawnedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSPAWNSPAWNEDORUNSPAWNEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLevelFinalizedMessage {
}

pub const CLIENTLEVELFINALIZEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelFinalizedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelFinalizedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEVELFINALIZEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLevelSpawnDebugEntitiesMessage {
}

pub const CLIENTLEVELSPAWNDEBUGENTITIESMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelSpawnDebugEntitiesMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelSpawnDebugEntitiesMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEVELSPAWNDEBUGENTITIESMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableUnspawnDoneMessage {
}

pub const CLIENTCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableUnspawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableUnspawnDoneMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableSpawnDoneMessage {
}

pub const CLIENTCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableSpawnDoneMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientConnectionUnloadLevelMessage {
}

pub const CLIENTCONNECTIONUNLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionUnloadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionUnloadLevelMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONNECTIONUNLOADLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientConnectionLinkLevelMessage {
}

pub const CLIENTCONNECTIONLINKLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionLinkLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionLinkLevelMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONNECTIONLINKLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientConnectionLoadLevelMessage {
}

pub const CLIENTCONNECTIONLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionLoadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionLoadLevelMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONNECTIONLOADLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientConnectionInitializedMessage {
}

pub const CLIENTCONNECTIONINITIALIZEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionInitializedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionInitializedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONNECTIONINITIALIZEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlaceHolderEntity {
}

pub const CLIENTPLACEHOLDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlaceHolderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLACEHOLDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlaceHolderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLACEHOLDERENTITY_TYPE_INFO
    }
}


pub const CLIENTPLACEHOLDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlaceHolderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPlaceHolderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPhysicsEntityWithPoseProvider {
}

pub const CLIENTPHYSICSENTITYWITHPOSEPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntityWithPoseProvider",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHYSICSENTITYWITHPOSEPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhysicsEntityWithPoseProvider {
    fn type_info() -> &'static TypeInfo {
        CLIENTPHYSICSENTITYWITHPOSEPROVIDER_TYPE_INFO
    }
}


pub const CLIENTPHYSICSENTITYWITHPOSEPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntityWithPoseProvider-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPhysicsEntityWithPoseProvider-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPhysicsEntity {
}

pub const CLIENTPHYSICSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHYSICSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhysicsEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPHYSICSENTITY_TYPE_INFO
    }
}


pub const CLIENTPHYSICSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPhysicsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameComponentEntity {
}

pub const CLIENTGAMECOMPONENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponentEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMECOMPONENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameComponentEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMECOMPONENTENTITY_TYPE_INFO
    }
}


pub const CLIENTGAMECOMPONENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientGameComponentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameComponent {
}

pub const CLIENTGAMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTGAMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientGameComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableEntityCreatedEntityInfo {
}

pub const CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntityCreatedEntityInfo",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientControllableEntityCreatedEntityInfo {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_TYPE_INFO
    }
}


pub const CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntityCreatedEntityInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientControllableEntityCreatedEntityInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableEntity {
}

pub const CLIENTCONTROLLABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONTROLLABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientControllableEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLEENTITY_TYPE_INFO
    }
}


pub const CLIENTCONTROLLABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientControllableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEntryComponent {
}

pub const CLIENTENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntryComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTENTRYCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientEntryComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientControllableHealthComponent {
}

pub const CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientControllableHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientControllableHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerExtent {
}

pub const CLIENTPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerExtent {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYEREXTENT_TYPE_INFO
    }
}


pub const CLIENTPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPlayerExtent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineManager {
}

pub const ONLINEMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineManager",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ONLINEMANAGER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnlineManager {
    fn type_info() -> &'static TypeInfo {
        ONLINEMANAGER_TYPE_INFO
    }
}


pub const ONLINEMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("OnlineManager-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientConnection {
}

pub const CLIENTCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnection",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONNECTIONPEER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientConnection {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONNECTION_TYPE_INFO
    }
}


pub const CLIENTCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientConnection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSpawnEntity {
}

pub const CLIENTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSPAWNENTITY_TYPE_INFO
    }
}


pub const CLIENTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerEvent {
}

pub const CLIENTPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PLAYEREVENTBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerEvent {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYEREVENT_TYPE_INFO
    }
}


pub const CLIENTPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPlayerEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDestructionComponent {
}

pub const CLIENTDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DESTRUCTIONCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDestructionComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTDESTRUCTIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientDestructionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerUpdateCameraComponentMessage {
}

pub const CLIENTPLAYERUPDATECAMERACOMPONENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerUpdateCameraComponentMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerUpdateCameraComponentMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERUPDATECAMERACOMPONENTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerSwitchTeamMessage {
}

pub const CLIENTPLAYERSWITCHTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerSwitchTeamMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerSwitchTeamMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERSWITCHTEAMMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerRequestCameraChangeMessage {
}

pub const CLIENTPLAYERREQUESTCAMERACHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerRequestCameraChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerRequestCameraChangeMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERREQUESTCAMERACHANGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerLocalSetMessage {
}

pub const CLIENTPLAYERLOCALSETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerLocalSetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerLocalSetMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERLOCALSETMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerExitEntryMessage {
}

pub const CLIENTPLAYEREXITENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerExitEntryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerExitEntryMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYEREXITENTRYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerEnterEntryMessage {
}

pub const CLIENTPLAYERENTERENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEnterEntryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerEnterEntryMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERENTERENTRYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerDeletedMessage {
}

pub const CLIENTPLAYERDELETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerDeletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerDeletedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERDELETEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerConnectMessage {
}

pub const CLIENTPLAYERCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerConnectMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerConnectMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERCONNECTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayerChangedPlayerViewMessage {
}

pub const CLIENTPLAYERCHANGEDPLAYERVIEWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerChangedPlayerViewMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerChangedPlayerViewMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYERCHANGEDPLAYERVIEWMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSetServerPasswordMessage {
}

pub const CLIENTSETSERVERPASSWORDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSetServerPasswordMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSetServerPasswordMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSETSERVERPASSWORDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWantFullscreenMessage {
}

pub const CLIENTWANTFULLSCREENMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWantFullscreenMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWantFullscreenMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTWANTFULLSCREENMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLeftRemoteServerMessage {
}

pub const CLIENTLEFTREMOTESERVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLeftRemoteServerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLeftRemoteServerMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEFTREMOTESERVERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDisconnectedMessage {
}

pub const CLIENTDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDisconnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientDisconnectedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTDISCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientConnectedMessage {
}

pub const CLIENTCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAbortCutsceneMessage {
}

pub const CLIENTABORTCUTSCENEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAbortCutsceneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientAbortCutsceneMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTABORTCUTSCENEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLevelLoadedMessage {
}

pub const CLIENTLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelLoadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEVELLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLevelLoadProgressMessage {
}

pub const CLIENTLEVELLOADPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelLoadProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelLoadProgressMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEVELLOADPROGRESSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLevelDescriptionLoadedMessage {
}

pub const CLIENTLEVELDESCRIPTIONLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelDescriptionLoadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelDescriptionLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEVELDESCRIPTIONLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLevelUnloadedMessage {
}

pub const CLIENTLEVELUNLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelUnloadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelUnloadedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEVELUNLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLevelUnloadStartedMessage {
}

pub const CLIENTLEVELUNLOADSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelUnloadStartedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelUnloadStartedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEVELUNLOADSTARTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientQuittingFinishedMessage {
}

pub const CLIENTQUITTINGFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuittingFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientQuittingFinishedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTQUITTINGFINISHEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientQuittingStartedMessage {
}

pub const CLIENTQUITTINGSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuittingStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientQuittingStartedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTQUITTINGSTARTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStartupFinishedMessage {
}

pub const CLIENTSTARTUPFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartupFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStartupFinishedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTARTUPFINISHEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLoadLevelMessage {
}

pub const CLIENTLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLoadLevelMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLoadLevelMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOADLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLoadLevelRequestedMessage {
}

pub const CLIENTLOADLEVELREQUESTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLoadLevelRequestedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLoadLevelRequestedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTLOADLEVELREQUESTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEnteredIngameMessage {
}

pub const CLIENTENTEREDINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEnteredIngameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientEnteredIngameMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTENTEREDINGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEnterHudIngameMessage {
}

pub const CLIENTENTERHUDINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEnterHudIngameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientEnterHudIngameMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTENTERHUDINGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientExitGameMessage {
}

pub const CLIENTEXITGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExitGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientExitGameMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTEXITGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientHostMigrationMessage {
}

pub const CLIENTHOSTMIGRATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHostMigrationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientHostMigrationMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTHOSTMIGRATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientExitToMenuMessage {
}

pub const CLIENTEXITTOMENUMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExitToMenuMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientExitToMenuMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTEXITTOMENUMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientReturnToMenuMessage {
}

pub const CLIENTRETURNTOMENUMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReturnToMenuMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientReturnToMenuMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTRETURNTOMENUMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStartMultiplayerMessage {
}

pub const CLIENTSTARTMULTIPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartMultiplayerMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStartMultiplayerMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTARTMULTIPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientContinueSingleplayerMessage {
}

pub const CLIENTCONTINUESINGLEPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientContinueSingleplayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientContinueSingleplayerMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONTINUESINGLEPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStartedMessage {
}

pub const CLIENTSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStartedMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTARTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientJoinServerJobMessage {
}

pub const CLIENTJOINSERVERJOBMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJoinServerJobMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientJoinServerJobMessage {
    fn type_info() -> &'static TypeInfo {
        CLIENTJOINSERVERJOBMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPeerNetworkRemovedMessageBase {
}

pub const CLIENTPEERNETWORKREMOVEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerNetworkRemovedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientPeerNetworkRemovedMessageBase {
    fn type_info() -> &'static TypeInfo {
        CLIENTPEERNETWORKREMOVEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientJoinMultiplayerMessageBase {
}

pub const CLIENTJOINMULTIPLAYERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJoinMultiplayerMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientJoinMultiplayerMessageBase {
    fn type_info() -> &'static TypeInfo {
        CLIENTJOINMULTIPLAYERMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientRestartSingleplayerMessageBase {
}

pub const CLIENTRESTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRestartSingleplayerMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientRestartSingleplayerMessageBase {
    fn type_info() -> &'static TypeInfo {
        CLIENTRESTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStartSingleplayerMessageBase {
}

pub const CLIENTSTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartSingleplayerMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientStartSingleplayerMessageBase {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDestructionComponent {
}

pub const SERVERDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DESTRUCTIONCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDestructionComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERDESTRUCTIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ServerDestructionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameComponent {
}

pub const SERVERGAMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMECOMPONENT_TYPE_INFO
    }
}


pub const SERVERGAMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ServerGameComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParameterComponent {
}

pub const SHADERPARAMETERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderParameterComponent {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMETERCOMPONENT_TYPE_INFO
    }
}


pub const SHADERPARAMETERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ShaderParameterComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPartComponent {
}

pub const CLIENTPARTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPARTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPartComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTPARTCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTPARTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPartComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPart {
}

pub const CLIENTPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPart",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PART_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPart {
    fn type_info() -> &'static TypeInfo {
        CLIENTPART_TYPE_INFO
    }
}


pub const CLIENTPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPart-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientGameHealthComponent {
}

pub const CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameHealthComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientGameHealthComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBoneComponent {
}

pub const CLIENTBONECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBONECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBoneComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTBONECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTBONECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientBoneComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSubView {
}

pub const CLIENTSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSubView {
    fn type_info() -> &'static TypeInfo {
        CLIENTSUBVIEW_TYPE_INFO
    }
}


pub const CLIENTSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientSubView-Array"),
    array_type: None,
    alignment: 8,
};


