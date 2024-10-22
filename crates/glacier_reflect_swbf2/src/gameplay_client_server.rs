use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ServerAdministrationRestartRequiredMessage {
}

pub trait ServerAdministrationRestartRequiredMessageTrait: TypeObject {
}

impl ServerAdministrationRestartRequiredMessageTrait for ServerAdministrationRestartRequiredMessage {
}

pub static SERVERADMINISTRATIONRESTARTREQUIREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationRestartRequiredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdministrationRestartRequiredMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationRestartRequiredMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINISTRATIONRESTARTREQUIREDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerPeerLoadLevelMessage {
}

pub trait ServerPeerLoadLevelMessageTrait: TypeObject {
}

impl ServerPeerLoadLevelMessageTrait for ServerPeerLoadLevelMessage {
}

pub static SERVERPEERLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPeerLoadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPeerLoadLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPeerLoadLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPEERLOADLEVELMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerPeerInitializedMessage {
}

pub trait ServerPeerInitializedMessageTrait: TypeObject {
}

impl ServerPeerInitializedMessageTrait for ServerPeerInitializedMessage {
}

pub static SERVERPEERINITIALIZEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerPeerInitializedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerPeerInitializedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerPeerInitializedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERPEERINITIALIZEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelLoadNextLevelMessage {
}

pub trait ServerLevelLoadNextLevelMessageTrait: TypeObject {
}

impl ServerLevelLoadNextLevelMessageTrait for ServerLevelLoadNextLevelMessage {
}

pub static SERVERLEVELLOADNEXTLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelLoadNextLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelLoadNextLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelLoadNextLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELLOADNEXTLEVELMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelLoadNextRoundMessage {
}

pub trait ServerLevelLoadNextRoundMessageTrait: TypeObject {
}

impl ServerLevelLoadNextRoundMessageTrait for ServerLevelLoadNextRoundMessage {
}

pub static SERVERLEVELLOADNEXTROUNDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelLoadNextRoundMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelLoadNextRoundMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelLoadNextRoundMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELLOADNEXTROUNDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerSubLevelOnStreamedInMessage {
}

pub trait ServerSubLevelOnStreamedInMessageTrait: TypeObject {
}

impl ServerSubLevelOnStreamedInMessageTrait for ServerSubLevelOnStreamedInMessage {
}

pub static SERVERSUBLEVELONSTREAMEDINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSubLevelOnStreamedInMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSubLevelOnStreamedInMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSubLevelOnStreamedInMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSUBLEVELONSTREAMEDINMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelCompletedMessage {
}

pub trait ServerLevelCompletedMessageTrait: TypeObject {
}

impl ServerLevelCompletedMessageTrait for ServerLevelCompletedMessage {
}

pub static SERVERLEVELCOMPLETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelCompletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelCompletedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelCompletedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELCOMPLETEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelStartedMessage {
}

pub trait ServerLevelStartedMessageTrait: TypeObject {
}

impl ServerLevelStartedMessageTrait for ServerLevelStartedMessage {
}

pub static SERVERLEVELSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelStartedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelStartedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELSTARTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelSpawnEntitiesEndMessage {
}

pub trait ServerLevelSpawnEntitiesEndMessageTrait: TypeObject {
}

impl ServerLevelSpawnEntitiesEndMessageTrait for ServerLevelSpawnEntitiesEndMessage {
}

pub static SERVERLEVELSPAWNENTITIESENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelSpawnEntitiesEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelSpawnEntitiesEndMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelSpawnEntitiesEndMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELSPAWNENTITIESENDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelSpawnEntitiesBeginMessage {
}

pub trait ServerLevelSpawnEntitiesBeginMessageTrait: TypeObject {
}

impl ServerLevelSpawnEntitiesBeginMessageTrait for ServerLevelSpawnEntitiesBeginMessage {
}

pub static SERVERLEVELSPAWNENTITIESBEGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelSpawnEntitiesBeginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelSpawnEntitiesBeginMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelSpawnEntitiesBeginMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELSPAWNENTITIESBEGINMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerSoundVoiceOverFinishedMessage {
}

pub trait ServerSoundVoiceOverFinishedMessageTrait: TypeObject {
}

impl ServerSoundVoiceOverFinishedMessageTrait for ServerSoundVoiceOverFinishedMessage {
}

pub static SERVERSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSoundVoiceOverFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSoundVoiceOverFinishedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerSoundVoiceOverFinishedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSOUNDVOICEOVERFINISHEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerInputDeactivateInputRestrictionMessage {
}

pub trait ServerInputDeactivateInputRestrictionMessageTrait: TypeObject {
}

impl ServerInputDeactivateInputRestrictionMessageTrait for ServerInputDeactivateInputRestrictionMessage {
}

pub static SERVERINPUTDEACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputDeactivateInputRestrictionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerInputDeactivateInputRestrictionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerInputDeactivateInputRestrictionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERINPUTDEACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerInputReactivateInputRestrictionMessage {
}

pub trait ServerInputReactivateInputRestrictionMessageTrait: TypeObject {
}

impl ServerInputReactivateInputRestrictionMessageTrait for ServerInputReactivateInputRestrictionMessage {
}

pub static SERVERINPUTREACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerInputReactivateInputRestrictionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerInputReactivateInputRestrictionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerInputReactivateInputRestrictionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERINPUTREACTIVATEINPUTRESTRICTIONMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerComponentEntryComponentProcessedInputMessage {
}

pub trait ServerComponentEntryComponentProcessedInputMessageTrait: TypeObject {
}

impl ServerComponentEntryComponentProcessedInputMessageTrait for ServerComponentEntryComponentProcessedInputMessage {
}

pub static SERVERCOMPONENTENTRYCOMPONENTPROCESSEDINPUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryComponentProcessedInputMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerComponentEntryComponentProcessedInputMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryComponentProcessedInputMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOMPONENTENTRYCOMPONENTPROCESSEDINPUTMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerComponentEntryOnPlayerExitsMessage {
}

pub trait ServerComponentEntryOnPlayerExitsMessageTrait: TypeObject {
}

impl ServerComponentEntryOnPlayerExitsMessageTrait for ServerComponentEntryOnPlayerExitsMessage {
}

pub static SERVERCOMPONENTENTRYONPLAYEREXITSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryOnPlayerExitsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerComponentEntryOnPlayerExitsMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryOnPlayerExitsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOMPONENTENTRYONPLAYEREXITSMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerComponentEntryOnPlayerEntersMessage {
}

pub trait ServerComponentEntryOnPlayerEntersMessageTrait: TypeObject {
}

impl ServerComponentEntryOnPlayerEntersMessageTrait for ServerComponentEntryOnPlayerEntersMessage {
}

pub static SERVERCOMPONENTENTRYONPLAYERENTERSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryOnPlayerEntersMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerComponentEntryOnPlayerEntersMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryOnPlayerEntersMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOMPONENTENTRYONPLAYERENTERSMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerComponentEntryOnUnspawnMessage {
}

pub trait ServerComponentEntryOnUnspawnMessageTrait: TypeObject {
}

impl ServerComponentEntryOnUnspawnMessageTrait for ServerComponentEntryOnUnspawnMessage {
}

pub static SERVERCOMPONENTENTRYONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerComponentEntryOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerComponentEntryOnUnspawnMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerComponentEntryOnUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCOMPONENTENTRYONUNSPAWNMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerControllableUnspawnDoneMessage {
}

pub trait ServerControllableUnspawnDoneMessageTrait: TypeObject {
}

impl ServerControllableUnspawnDoneMessageTrait for ServerControllableUnspawnDoneMessage {
}

pub static SERVERCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableUnspawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerControllableUnspawnDoneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableUnspawnDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerControllableSpawnDoneMessage {
}

pub trait ServerControllableSpawnDoneMessageTrait: TypeObject {
}

impl ServerControllableSpawnDoneMessageTrait for ServerControllableSpawnDoneMessage {
}

pub static SERVERCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerControllableSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerControllableSpawnDoneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerControllableSpawnDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerConnectionExitMessage {
}

pub trait ServerConnectionExitMessageTrait: TypeObject {
}

impl ServerConnectionExitMessageTrait for ServerConnectionExitMessage {
}

pub static SERVERCONNECTIONEXITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnectionExitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerConnectionExitMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerConnectionExitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONNECTIONEXITMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerConnectionInitMessage {
}

pub trait ServerConnectionInitMessageTrait: TypeObject {
}

impl ServerConnectionInitMessageTrait for ServerConnectionInitMessage {
}

pub static SERVERCONNECTIONINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerConnectionInitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerConnectionInitMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerConnectionInitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCONNECTIONINITMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerClientConnectionRemovedMessage {
}

pub trait ServerClientConnectionRemovedMessageTrait: TypeObject {
}

impl ServerClientConnectionRemovedMessageTrait for ServerClientConnectionRemovedMessage {
}

pub static SERVERCLIENTCONNECTIONREMOVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClientConnectionRemovedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerClientConnectionRemovedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClientConnectionRemovedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCLIENTCONNECTIONREMOVEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerClientConnectionConnectedMessage {
}

pub trait ServerClientConnectionConnectedMessageTrait: TypeObject {
}

impl ServerClientConnectionConnectedMessageTrait for ServerClientConnectionConnectedMessage {
}

pub static SERVERCLIENTCONNECTIONCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerClientConnectionConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerClientConnectionConnectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerClientConnectionConnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCLIENTCONNECTIONCONNECTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerAdminBanPlayerMessage {
}

pub trait ServerAdminBanPlayerMessageTrait: TypeObject {
}

impl ServerAdminBanPlayerMessageTrait for ServerAdminBanPlayerMessage {
}

pub static SERVERADMINBANPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdminBanPlayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdminBanPlayerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdminBanPlayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINBANPLAYERMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerAdminSetServerNameMessage {
}

pub trait ServerAdminSetServerNameMessageTrait: TypeObject {
}

impl ServerAdminSetServerNameMessageTrait for ServerAdminSetServerNameMessage {
}

pub static SERVERADMINSETSERVERNAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdminSetServerNameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdminSetServerNameMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdminSetServerNameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINSETSERVERNAMEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerScriptTickMessage {
}

pub trait ServerScriptTickMessageTrait: TypeObject {
}

impl ServerScriptTickMessageTrait for ServerScriptTickMessage {
}

pub static SERVERSCRIPTTICKMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerScriptTickMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerScriptTickMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerScriptTickMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSCRIPTTICKMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerStopMessageBase {
}

pub trait ServerStopMessageBaseTrait: TypeObject {
}

impl ServerStopMessageBaseTrait for ServerStopMessageBase {
}

pub static SERVERSTOPMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStopMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStopMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerStopMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTOPMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLoadLevelMessageBase {
}

pub trait ServerLoadLevelMessageBaseTrait: TypeObject {
}

impl ServerLoadLevelMessageBaseTrait for ServerLoadLevelMessageBase {
}

pub static SERVERLOADLEVELMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLoadLevelMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLoadLevelMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerLoadLevelMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLOADLEVELMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelUnloadedMessage {
}

pub trait ServerLevelUnloadedMessageTrait: TypeObject {
}

impl ServerLevelUnloadedMessageTrait for ServerLevelUnloadedMessage {
}

pub static SERVERLEVELUNLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelUnloadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelUnloadedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelUnloadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELUNLOADEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerUnloadLevelMessage {
}

pub trait ServerUnloadLevelMessageTrait: TypeObject {
}

impl ServerUnloadLevelMessageTrait for ServerUnloadLevelMessage {
}

pub static SERVERUNLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerUnloadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerUnloadLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerUnloadLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERUNLOADLEVELMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLevelLoadedMessage {
}

pub trait ServerLevelLoadedMessageTrait: TypeObject {
}

impl ServerLevelLoadedMessageTrait for ServerLevelLoadedMessage {
}

pub static SERVERLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLevelLoadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLevelLoadedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLevelLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLEVELLOADEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerStoppedMessage {
}

pub trait ServerStoppedMessageTrait: TypeObject {
}

impl ServerStoppedMessageTrait for ServerStoppedMessage {
}

pub static SERVERSTOPPEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStoppedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStoppedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerStoppedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTOPPEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerApplyConfigurationMessage {
}

pub trait ServerApplyConfigurationMessageTrait: TypeObject {
}

impl ServerApplyConfigurationMessageTrait for ServerApplyConfigurationMessage {
}

pub static SERVERAPPLYCONFIGURATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerApplyConfigurationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerApplyConfigurationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerApplyConfigurationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAPPLYCONFIGURATIONMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerResetConfigurationMessage {
}

pub trait ServerResetConfigurationMessageTrait: TypeObject {
}

impl ServerResetConfigurationMessageTrait for ServerResetConfigurationMessage {
}

pub static SERVERRESETCONFIGURATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerResetConfigurationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerResetConfigurationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerResetConfigurationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERRESETCONFIGURATIONMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerStartedMessage {
}

pub trait ServerStartedMessageTrait: TypeObject {
}

impl ServerStartedMessageTrait for ServerStartedMessage {
}

pub static SERVERSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStartedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerStartedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTARTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerLoadGameMessage {
}

pub trait ServerLoadGameMessageTrait: TypeObject {
}

impl ServerLoadGameMessageTrait for ServerLoadGameMessage {
}

pub static SERVERLOADGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerLoadGameMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerLoadGameMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerLoadGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERLOADGAMEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct SaveGameWrittenToMediaMessage {
}

pub trait SaveGameWrittenToMediaMessageTrait: TypeObject {
}

impl SaveGameWrittenToMediaMessageTrait for SaveGameWrittenToMediaMessage {
}

pub static SAVEGAMEWRITTENTOMEDIAMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameWrittenToMediaMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SaveGameWrittenToMediaMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SaveGameWrittenToMediaMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SAVEGAMEWRITTENTOMEDIAMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct SaveGameGeneratedMessage {
}

pub trait SaveGameGeneratedMessageTrait: TypeObject {
}

impl SaveGameGeneratedMessageTrait for SaveGameGeneratedMessage {
}

pub static SAVEGAMEGENERATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SaveGameGeneratedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SaveGameGeneratedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SaveGameGeneratedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SAVEGAMEGENERATEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct StatDisableMessage {
}

pub trait StatDisableMessageTrait: TypeObject {
}

impl StatDisableMessageTrait for StatDisableMessage {
}

pub static STATDISABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatDisableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StatDisableMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StatDisableMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STATDISABLEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct StatEnableMessage {
}

pub trait StatEnableMessageTrait: TypeObject {
}

impl StatEnableMessageTrait for StatEnableMessage {
}

pub static STATENABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatEnableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StatEnableMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StatEnableMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STATENABLEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct SoundSettings {
    pub _glacier_base: super::core::DataContainer,
}

pub trait SoundSettingsTrait: super::core::DataContainerTrait {
}

impl SoundSettingsTrait for SoundSettings {
}

impl super::core::DataContainerTrait for SoundSettings {
}

pub static SOUNDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SoundSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SOUNDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundSettings {
    fn type_info(&self) -> &'static TypeInfo {
        SOUNDSETTINGS_TYPE_INFO
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


pub static SOUNDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("SoundSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSettings {
    pub _glacier_base: super::core::SystemSettings,
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
    pub audio_system_guid: glacier_util::guid::Guid,
    pub frame_history_time_warn_scale: f32,
    pub fast_exit: bool,
}

pub trait ClientSettingsTrait: super::core::SystemSettingsTrait {
    fn is_spectator(&self) -> &bool;
    fn is_spectator_mut(&mut self) -> &mut bool;
    fn allow_video_recording(&self) -> &bool;
    fn allow_video_recording_mut(&mut self) -> &mut bool;
    fn debris_cluster_enabled(&self) -> &bool;
    fn debris_cluster_enabled_mut(&mut self) -> &mut bool;
    fn vegetation_enabled(&self) -> &bool;
    fn vegetation_enabled_mut(&mut self) -> &mut bool;
    fn force_enabled(&self) -> &bool;
    fn force_enabled_mut(&mut self) -> &mut bool;
    fn world_render_enabled(&self) -> &bool;
    fn world_render_enabled_mut(&mut self) -> &mut bool;
    fn terrain_enabled(&self) -> &bool;
    fn terrain_enabled_mut(&mut self) -> &mut bool;
    fn water_physics_enabled(&self) -> &bool;
    fn water_physics_enabled_mut(&mut self) -> &mut bool;
    fn overgrowth_enabled(&self) -> &bool;
    fn overgrowth_enabled_mut(&mut self) -> &mut bool;
    fn effects_enabled(&self) -> &bool;
    fn effects_enabled_mut(&mut self) -> &mut bool;
    fn auto_increment_pad_index(&self) -> &bool;
    fn auto_increment_pad_index_mut(&mut self) -> &mut bool;
    fn lip_sync_enabled(&self) -> &bool;
    fn lip_sync_enabled_mut(&mut self) -> &mut bool;
    fn pause_game_on_start_up(&self) -> &bool;
    fn pause_game_on_start_up_mut(&mut self) -> &mut bool;
    fn skip_fast_level_load(&self) -> &bool;
    fn skip_fast_level_load_mut(&mut self) -> &mut bool;
    fn screenshot_to_file(&self) -> &bool;
    fn screenshot_to_file_mut(&mut self) -> &mut bool;
    fn screenshot_filename(&self) -> &String;
    fn screenshot_filename_mut(&mut self) -> &mut String;
    fn screenshot_suffix(&self) -> &String;
    fn screenshot_suffix_mut(&mut self) -> &mut String;
    fn load_menu(&self) -> &bool;
    fn load_menu_mut(&mut self) -> &mut bool;
    fn debug_menu_on_l_thumb(&self) -> &bool;
    fn debug_menu_on_l_thumb_mut(&mut self) -> &mut bool;
    fn screenshot_comparisons_enable(&self) -> &bool;
    fn screenshot_comparisons_enable_mut(&mut self) -> &mut bool;
    fn render_tags(&self) -> &bool;
    fn render_tags_mut(&mut self) -> &mut bool;
    fn team(&self) -> &u32;
    fn team_mut(&mut self) -> &mut u32;
    fn spawn_point_index(&self) -> &i32;
    fn spawn_point_index_mut(&mut self) -> &mut i32;
    fn server_ip(&self) -> &String;
    fn server_ip_mut(&mut self) -> &mut String;
    fn secondary_server_ip(&self) -> &String;
    fn secondary_server_ip_mut(&mut self) -> &mut String;
    fn scheme0_flip_y(&self) -> &bool;
    fn scheme0_flip_y_mut(&mut self) -> &mut bool;
    fn scheme1_flip_y(&self) -> &bool;
    fn scheme1_flip_y_mut(&mut self) -> &mut bool;
    fn scheme2_flip_y(&self) -> &bool;
    fn scheme2_flip_y_mut(&mut self) -> &mut bool;
    fn aim_scale(&self) -> &f32;
    fn aim_scale_mut(&mut self) -> &mut f32;
    fn havok_visual_debugger(&self) -> &bool;
    fn havok_visual_debugger_mut(&mut self) -> &mut bool;
    fn havok_capture_to_file(&self) -> &bool;
    fn havok_capture_to_file_mut(&mut self) -> &mut bool;
    fn show_build_id(&self) -> &bool;
    fn show_build_id_mut(&mut self) -> &mut bool;
    fn extract_persistence_information(&self) -> &bool;
    fn extract_persistence_information_mut(&mut self) -> &mut bool;
    fn enable_rest_tool(&self) -> &bool;
    fn enable_rest_tool_mut(&mut self) -> &mut bool;
    fn local_vehicle_simulation_enabled(&self) -> &bool;
    fn local_vehicle_simulation_enabled_mut(&mut self) -> &mut bool;
    fn auto_unspawn_dynamic_objects(&self) -> &bool;
    fn auto_unspawn_dynamic_objects_mut(&mut self) -> &mut bool;
    fn incoming_frequency(&self) -> &f32;
    fn incoming_frequency_mut(&mut self) -> &mut f32;
    fn outgoing_frequency(&self) -> &f32;
    fn outgoing_frequency_mut(&mut self) -> &mut f32;
    fn incoming_rate(&self) -> &u32;
    fn incoming_rate_mut(&mut self) -> &mut u32;
    fn outgoing_rate(&self) -> &u32;
    fn outgoing_rate_mut(&mut self) -> &mut u32;
    fn loading_timeout(&self) -> &f32;
    fn loading_timeout_mut(&mut self) -> &mut f32;
    fn loaded_timeout(&self) -> &f32;
    fn loaded_timeout_mut(&mut self) -> &mut f32;
    fn ingame_timeout(&self) -> &f32;
    fn ingame_timeout_mut(&mut self) -> &mut f32;
    fn quit_game_on_server_disconnect(&self) -> &bool;
    fn quit_game_on_server_disconnect_mut(&mut self) -> &mut bool;
    fn lua_option_set_enable(&self) -> &bool;
    fn lua_option_set_enable_mut(&mut self) -> &mut bool;
    fn cpu_quality(&self) -> &f32;
    fn cpu_quality_mut(&mut self) -> &mut f32;
    fn instance_path(&self) -> &String;
    fn instance_path_mut(&mut self) -> &mut String;
    fn audio_system_guid(&self) -> &glacier_util::guid::Guid;
    fn audio_system_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn frame_history_time_warn_scale(&self) -> &f32;
    fn frame_history_time_warn_scale_mut(&mut self) -> &mut f32;
    fn fast_exit(&self) -> &bool;
    fn fast_exit_mut(&mut self) -> &mut bool;
}

impl ClientSettingsTrait for ClientSettings {
    fn is_spectator(&self) -> &bool {
        &self.is_spectator
    }
    fn is_spectator_mut(&mut self) -> &mut bool {
        &mut self.is_spectator
    }
    fn allow_video_recording(&self) -> &bool {
        &self.allow_video_recording
    }
    fn allow_video_recording_mut(&mut self) -> &mut bool {
        &mut self.allow_video_recording
    }
    fn debris_cluster_enabled(&self) -> &bool {
        &self.debris_cluster_enabled
    }
    fn debris_cluster_enabled_mut(&mut self) -> &mut bool {
        &mut self.debris_cluster_enabled
    }
    fn vegetation_enabled(&self) -> &bool {
        &self.vegetation_enabled
    }
    fn vegetation_enabled_mut(&mut self) -> &mut bool {
        &mut self.vegetation_enabled
    }
    fn force_enabled(&self) -> &bool {
        &self.force_enabled
    }
    fn force_enabled_mut(&mut self) -> &mut bool {
        &mut self.force_enabled
    }
    fn world_render_enabled(&self) -> &bool {
        &self.world_render_enabled
    }
    fn world_render_enabled_mut(&mut self) -> &mut bool {
        &mut self.world_render_enabled
    }
    fn terrain_enabled(&self) -> &bool {
        &self.terrain_enabled
    }
    fn terrain_enabled_mut(&mut self) -> &mut bool {
        &mut self.terrain_enabled
    }
    fn water_physics_enabled(&self) -> &bool {
        &self.water_physics_enabled
    }
    fn water_physics_enabled_mut(&mut self) -> &mut bool {
        &mut self.water_physics_enabled
    }
    fn overgrowth_enabled(&self) -> &bool {
        &self.overgrowth_enabled
    }
    fn overgrowth_enabled_mut(&mut self) -> &mut bool {
        &mut self.overgrowth_enabled
    }
    fn effects_enabled(&self) -> &bool {
        &self.effects_enabled
    }
    fn effects_enabled_mut(&mut self) -> &mut bool {
        &mut self.effects_enabled
    }
    fn auto_increment_pad_index(&self) -> &bool {
        &self.auto_increment_pad_index
    }
    fn auto_increment_pad_index_mut(&mut self) -> &mut bool {
        &mut self.auto_increment_pad_index
    }
    fn lip_sync_enabled(&self) -> &bool {
        &self.lip_sync_enabled
    }
    fn lip_sync_enabled_mut(&mut self) -> &mut bool {
        &mut self.lip_sync_enabled
    }
    fn pause_game_on_start_up(&self) -> &bool {
        &self.pause_game_on_start_up
    }
    fn pause_game_on_start_up_mut(&mut self) -> &mut bool {
        &mut self.pause_game_on_start_up
    }
    fn skip_fast_level_load(&self) -> &bool {
        &self.skip_fast_level_load
    }
    fn skip_fast_level_load_mut(&mut self) -> &mut bool {
        &mut self.skip_fast_level_load
    }
    fn screenshot_to_file(&self) -> &bool {
        &self.screenshot_to_file
    }
    fn screenshot_to_file_mut(&mut self) -> &mut bool {
        &mut self.screenshot_to_file
    }
    fn screenshot_filename(&self) -> &String {
        &self.screenshot_filename
    }
    fn screenshot_filename_mut(&mut self) -> &mut String {
        &mut self.screenshot_filename
    }
    fn screenshot_suffix(&self) -> &String {
        &self.screenshot_suffix
    }
    fn screenshot_suffix_mut(&mut self) -> &mut String {
        &mut self.screenshot_suffix
    }
    fn load_menu(&self) -> &bool {
        &self.load_menu
    }
    fn load_menu_mut(&mut self) -> &mut bool {
        &mut self.load_menu
    }
    fn debug_menu_on_l_thumb(&self) -> &bool {
        &self.debug_menu_on_l_thumb
    }
    fn debug_menu_on_l_thumb_mut(&mut self) -> &mut bool {
        &mut self.debug_menu_on_l_thumb
    }
    fn screenshot_comparisons_enable(&self) -> &bool {
        &self.screenshot_comparisons_enable
    }
    fn screenshot_comparisons_enable_mut(&mut self) -> &mut bool {
        &mut self.screenshot_comparisons_enable
    }
    fn render_tags(&self) -> &bool {
        &self.render_tags
    }
    fn render_tags_mut(&mut self) -> &mut bool {
        &mut self.render_tags
    }
    fn team(&self) -> &u32 {
        &self.team
    }
    fn team_mut(&mut self) -> &mut u32 {
        &mut self.team
    }
    fn spawn_point_index(&self) -> &i32 {
        &self.spawn_point_index
    }
    fn spawn_point_index_mut(&mut self) -> &mut i32 {
        &mut self.spawn_point_index
    }
    fn server_ip(&self) -> &String {
        &self.server_ip
    }
    fn server_ip_mut(&mut self) -> &mut String {
        &mut self.server_ip
    }
    fn secondary_server_ip(&self) -> &String {
        &self.secondary_server_ip
    }
    fn secondary_server_ip_mut(&mut self) -> &mut String {
        &mut self.secondary_server_ip
    }
    fn scheme0_flip_y(&self) -> &bool {
        &self.scheme0_flip_y
    }
    fn scheme0_flip_y_mut(&mut self) -> &mut bool {
        &mut self.scheme0_flip_y
    }
    fn scheme1_flip_y(&self) -> &bool {
        &self.scheme1_flip_y
    }
    fn scheme1_flip_y_mut(&mut self) -> &mut bool {
        &mut self.scheme1_flip_y
    }
    fn scheme2_flip_y(&self) -> &bool {
        &self.scheme2_flip_y
    }
    fn scheme2_flip_y_mut(&mut self) -> &mut bool {
        &mut self.scheme2_flip_y
    }
    fn aim_scale(&self) -> &f32 {
        &self.aim_scale
    }
    fn aim_scale_mut(&mut self) -> &mut f32 {
        &mut self.aim_scale
    }
    fn havok_visual_debugger(&self) -> &bool {
        &self.havok_visual_debugger
    }
    fn havok_visual_debugger_mut(&mut self) -> &mut bool {
        &mut self.havok_visual_debugger
    }
    fn havok_capture_to_file(&self) -> &bool {
        &self.havok_capture_to_file
    }
    fn havok_capture_to_file_mut(&mut self) -> &mut bool {
        &mut self.havok_capture_to_file
    }
    fn show_build_id(&self) -> &bool {
        &self.show_build_id
    }
    fn show_build_id_mut(&mut self) -> &mut bool {
        &mut self.show_build_id
    }
    fn extract_persistence_information(&self) -> &bool {
        &self.extract_persistence_information
    }
    fn extract_persistence_information_mut(&mut self) -> &mut bool {
        &mut self.extract_persistence_information
    }
    fn enable_rest_tool(&self) -> &bool {
        &self.enable_rest_tool
    }
    fn enable_rest_tool_mut(&mut self) -> &mut bool {
        &mut self.enable_rest_tool
    }
    fn local_vehicle_simulation_enabled(&self) -> &bool {
        &self.local_vehicle_simulation_enabled
    }
    fn local_vehicle_simulation_enabled_mut(&mut self) -> &mut bool {
        &mut self.local_vehicle_simulation_enabled
    }
    fn auto_unspawn_dynamic_objects(&self) -> &bool {
        &self.auto_unspawn_dynamic_objects
    }
    fn auto_unspawn_dynamic_objects_mut(&mut self) -> &mut bool {
        &mut self.auto_unspawn_dynamic_objects
    }
    fn incoming_frequency(&self) -> &f32 {
        &self.incoming_frequency
    }
    fn incoming_frequency_mut(&mut self) -> &mut f32 {
        &mut self.incoming_frequency
    }
    fn outgoing_frequency(&self) -> &f32 {
        &self.outgoing_frequency
    }
    fn outgoing_frequency_mut(&mut self) -> &mut f32 {
        &mut self.outgoing_frequency
    }
    fn incoming_rate(&self) -> &u32 {
        &self.incoming_rate
    }
    fn incoming_rate_mut(&mut self) -> &mut u32 {
        &mut self.incoming_rate
    }
    fn outgoing_rate(&self) -> &u32 {
        &self.outgoing_rate
    }
    fn outgoing_rate_mut(&mut self) -> &mut u32 {
        &mut self.outgoing_rate
    }
    fn loading_timeout(&self) -> &f32 {
        &self.loading_timeout
    }
    fn loading_timeout_mut(&mut self) -> &mut f32 {
        &mut self.loading_timeout
    }
    fn loaded_timeout(&self) -> &f32 {
        &self.loaded_timeout
    }
    fn loaded_timeout_mut(&mut self) -> &mut f32 {
        &mut self.loaded_timeout
    }
    fn ingame_timeout(&self) -> &f32 {
        &self.ingame_timeout
    }
    fn ingame_timeout_mut(&mut self) -> &mut f32 {
        &mut self.ingame_timeout
    }
    fn quit_game_on_server_disconnect(&self) -> &bool {
        &self.quit_game_on_server_disconnect
    }
    fn quit_game_on_server_disconnect_mut(&mut self) -> &mut bool {
        &mut self.quit_game_on_server_disconnect
    }
    fn lua_option_set_enable(&self) -> &bool {
        &self.lua_option_set_enable
    }
    fn lua_option_set_enable_mut(&mut self) -> &mut bool {
        &mut self.lua_option_set_enable
    }
    fn cpu_quality(&self) -> &f32 {
        &self.cpu_quality
    }
    fn cpu_quality_mut(&mut self) -> &mut f32 {
        &mut self.cpu_quality
    }
    fn instance_path(&self) -> &String {
        &self.instance_path
    }
    fn instance_path_mut(&mut self) -> &mut String {
        &mut self.instance_path
    }
    fn audio_system_guid(&self) -> &glacier_util::guid::Guid {
        &self.audio_system_guid
    }
    fn audio_system_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.audio_system_guid
    }
    fn frame_history_time_warn_scale(&self) -> &f32 {
        &self.frame_history_time_warn_scale
    }
    fn frame_history_time_warn_scale_mut(&mut self) -> &mut f32 {
        &mut self.frame_history_time_warn_scale
    }
    fn fast_exit(&self) -> &bool {
        &self.fast_exit
    }
    fn fast_exit_mut(&mut self) -> &mut bool {
        &mut self.fast_exit
    }
}

impl super::core::SystemSettingsTrait for ClientSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for ClientSettings {
}

pub static CLIENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSettings",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsSpectator",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, is_spectator),
            },
            FieldInfoData {
                name: "AllowVideoRecording",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, allow_video_recording),
            },
            FieldInfoData {
                name: "DebrisClusterEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, debris_cluster_enabled),
            },
            FieldInfoData {
                name: "VegetationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, vegetation_enabled),
            },
            FieldInfoData {
                name: "ForceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, force_enabled),
            },
            FieldInfoData {
                name: "WorldRenderEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, world_render_enabled),
            },
            FieldInfoData {
                name: "TerrainEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, terrain_enabled),
            },
            FieldInfoData {
                name: "WaterPhysicsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, water_physics_enabled),
            },
            FieldInfoData {
                name: "OvergrowthEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, overgrowth_enabled),
            },
            FieldInfoData {
                name: "EffectsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, effects_enabled),
            },
            FieldInfoData {
                name: "AutoIncrementPadIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, auto_increment_pad_index),
            },
            FieldInfoData {
                name: "LipSyncEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, lip_sync_enabled),
            },
            FieldInfoData {
                name: "PauseGameOnStartUp",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, pause_game_on_start_up),
            },
            FieldInfoData {
                name: "SkipFastLevelLoad",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, skip_fast_level_load),
            },
            FieldInfoData {
                name: "ScreenshotToFile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, screenshot_to_file),
            },
            FieldInfoData {
                name: "ScreenshotFilename",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClientSettings, screenshot_filename),
            },
            FieldInfoData {
                name: "ScreenshotSuffix",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClientSettings, screenshot_suffix),
            },
            FieldInfoData {
                name: "LoadMenu",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, load_menu),
            },
            FieldInfoData {
                name: "DebugMenuOnLThumb",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, debug_menu_on_l_thumb),
            },
            FieldInfoData {
                name: "ScreenshotComparisonsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, screenshot_comparisons_enable),
            },
            FieldInfoData {
                name: "RenderTags",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, render_tags),
            },
            FieldInfoData {
                name: "Team",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClientSettings, team),
            },
            FieldInfoData {
                name: "SpawnPointIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ClientSettings, spawn_point_index),
            },
            FieldInfoData {
                name: "ServerIp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClientSettings, server_ip),
            },
            FieldInfoData {
                name: "SecondaryServerIp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClientSettings, secondary_server_ip),
            },
            FieldInfoData {
                name: "Scheme0FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, scheme0_flip_y),
            },
            FieldInfoData {
                name: "Scheme1FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, scheme1_flip_y),
            },
            FieldInfoData {
                name: "Scheme2FlipY",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, scheme2_flip_y),
            },
            FieldInfoData {
                name: "AimScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, aim_scale),
            },
            FieldInfoData {
                name: "HavokVisualDebugger",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, havok_visual_debugger),
            },
            FieldInfoData {
                name: "HavokCaptureToFile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, havok_capture_to_file),
            },
            FieldInfoData {
                name: "ShowBuildId",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, show_build_id),
            },
            FieldInfoData {
                name: "ExtractPersistenceInformation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, extract_persistence_information),
            },
            FieldInfoData {
                name: "EnableRestTool",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, enable_rest_tool),
            },
            FieldInfoData {
                name: "LocalVehicleSimulationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, local_vehicle_simulation_enabled),
            },
            FieldInfoData {
                name: "AutoUnspawnDynamicObjects",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, auto_unspawn_dynamic_objects),
            },
            FieldInfoData {
                name: "IncomingFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, incoming_frequency),
            },
            FieldInfoData {
                name: "OutgoingFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, outgoing_frequency),
            },
            FieldInfoData {
                name: "IncomingRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClientSettings, incoming_rate),
            },
            FieldInfoData {
                name: "OutgoingRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClientSettings, outgoing_rate),
            },
            FieldInfoData {
                name: "LoadingTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, loading_timeout),
            },
            FieldInfoData {
                name: "LoadedTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, loaded_timeout),
            },
            FieldInfoData {
                name: "IngameTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, ingame_timeout),
            },
            FieldInfoData {
                name: "QuitGameOnServerDisconnect",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, quit_game_on_server_disconnect),
            },
            FieldInfoData {
                name: "LuaOptionSetEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, lua_option_set_enable),
            },
            FieldInfoData {
                name: "CpuQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, cpu_quality),
            },
            FieldInfoData {
                name: "InstancePath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ClientSettings, instance_path),
            },
            FieldInfoData {
                name: "AudioSystemGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(ClientSettings, audio_system_guid),
            },
            FieldInfoData {
                name: "FrameHistoryTimeWarnScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClientSettings, frame_history_time_warn_scale),
            },
            FieldInfoData {
                name: "FastExit",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClientSettings, fast_exit),
            },
        ],
    }),
    array_type: Some(CLIENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClientSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSETTINGS_TYPE_INFO
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


pub static CLIENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSpawnSpawnedOrUnSpawnedMessage {
}

pub trait ClientSpawnSpawnedOrUnSpawnedMessageTrait: TypeObject {
}

impl ClientSpawnSpawnedOrUnSpawnedMessageTrait for ClientSpawnSpawnedOrUnSpawnedMessage {
}

pub static CLIENTSPAWNSPAWNEDORUNSPAWNEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpawnSpawnedOrUnSpawnedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSpawnSpawnedOrUnSpawnedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSpawnSpawnedOrUnSpawnedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSPAWNSPAWNEDORUNSPAWNEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLevelFinalizedMessage {
}

pub trait ClientLevelFinalizedMessageTrait: TypeObject {
}

impl ClientLevelFinalizedMessageTrait for ClientLevelFinalizedMessage {
}

pub static CLIENTLEVELFINALIZEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelFinalizedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLevelFinalizedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelFinalizedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEVELFINALIZEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLevelSpawnDebugEntitiesMessage {
}

pub trait ClientLevelSpawnDebugEntitiesMessageTrait: TypeObject {
}

impl ClientLevelSpawnDebugEntitiesMessageTrait for ClientLevelSpawnDebugEntitiesMessage {
}

pub static CLIENTLEVELSPAWNDEBUGENTITIESMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelSpawnDebugEntitiesMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLevelSpawnDebugEntitiesMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelSpawnDebugEntitiesMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEVELSPAWNDEBUGENTITIESMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientControllableUnspawnDoneMessage {
}

pub trait ClientControllableUnspawnDoneMessageTrait: TypeObject {
}

impl ClientControllableUnspawnDoneMessageTrait for ClientControllableUnspawnDoneMessage {
}

pub static CLIENTCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableUnspawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableUnspawnDoneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableUnspawnDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLEUNSPAWNDONEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientControllableSpawnDoneMessage {
}

pub trait ClientControllableSpawnDoneMessageTrait: TypeObject {
}

impl ClientControllableSpawnDoneMessageTrait for ClientControllableSpawnDoneMessage {
}

pub static CLIENTCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableSpawnDoneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableSpawnDoneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientControllableSpawnDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLESPAWNDONEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientConnectionUnloadLevelMessage {
}

pub trait ClientConnectionUnloadLevelMessageTrait: TypeObject {
}

impl ClientConnectionUnloadLevelMessageTrait for ClientConnectionUnloadLevelMessage {
}

pub static CLIENTCONNECTIONUNLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionUnloadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnectionUnloadLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionUnloadLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONNECTIONUNLOADLEVELMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientConnectionLinkLevelMessage {
}

pub trait ClientConnectionLinkLevelMessageTrait: TypeObject {
}

impl ClientConnectionLinkLevelMessageTrait for ClientConnectionLinkLevelMessage {
}

pub static CLIENTCONNECTIONLINKLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionLinkLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnectionLinkLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionLinkLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONNECTIONLINKLEVELMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientConnectionLoadLevelMessage {
}

pub trait ClientConnectionLoadLevelMessageTrait: TypeObject {
}

impl ClientConnectionLoadLevelMessageTrait for ClientConnectionLoadLevelMessage {
}

pub static CLIENTCONNECTIONLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionLoadLevelMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnectionLoadLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionLoadLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONNECTIONLOADLEVELMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientConnectionInitializedMessage {
}

pub trait ClientConnectionInitializedMessageTrait: TypeObject {
}

impl ClientConnectionInitializedMessageTrait for ClientConnectionInitializedMessage {
}

pub static CLIENTCONNECTIONINITIALIZEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionInitializedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnectionInitializedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectionInitializedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONNECTIONINITIALIZEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlaceHolderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPlaceHolderEntityTrait: super::entity::EntityTrait {
}

impl ClientPlaceHolderEntityTrait for ClientPlaceHolderEntity {
}

impl super::entity::EntityTrait for ClientPlaceHolderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlaceHolderEntity {
}

pub static CLIENTPLACEHOLDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlaceHolderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlaceHolderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLACEHOLDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlaceHolderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLACEHOLDERENTITY_TYPE_INFO
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


pub static CLIENTPLACEHOLDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlaceHolderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPlaceHolderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPhysicsEntityWithPoseProvider {
    pub _glacier_base: ClientPhysicsEntity,
}

pub trait ClientPhysicsEntityWithPoseProviderTrait: ClientPhysicsEntityTrait {
}

impl ClientPhysicsEntityWithPoseProviderTrait for ClientPhysicsEntityWithPoseProvider {
}

impl ClientPhysicsEntityTrait for ClientPhysicsEntityWithPoseProvider {
}

impl ClientGameComponentEntityTrait for ClientPhysicsEntityWithPoseProvider {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientPhysicsEntityWithPoseProvider {
}

impl super::entity::ComponentEntityTrait for ClientPhysicsEntityWithPoseProvider {
}

impl super::entity::SpatialEntityTrait for ClientPhysicsEntityWithPoseProvider {
}

impl super::entity::EntityTrait for ClientPhysicsEntityWithPoseProvider {
}

impl super::entity::EntityBusPeerTrait for ClientPhysicsEntityWithPoseProvider {
}

pub static CLIENTPHYSICSENTITYWITHPOSEPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntityWithPoseProvider",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPhysicsEntityWithPoseProvider as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHYSICSENTITYWITHPOSEPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhysicsEntityWithPoseProvider {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPHYSICSENTITYWITHPOSEPROVIDER_TYPE_INFO
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


pub static CLIENTPHYSICSENTITYWITHPOSEPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntityWithPoseProvider-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPhysicsEntityWithPoseProvider"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPhysicsEntity {
    pub _glacier_base: ClientGameComponentEntity,
}

pub trait ClientPhysicsEntityTrait: ClientGameComponentEntityTrait {
}

impl ClientPhysicsEntityTrait for ClientPhysicsEntity {
}

impl ClientGameComponentEntityTrait for ClientPhysicsEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientPhysicsEntity {
}

impl super::entity::ComponentEntityTrait for ClientPhysicsEntity {
}

impl super::entity::SpatialEntityTrait for ClientPhysicsEntity {
}

impl super::entity::EntityTrait for ClientPhysicsEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPhysicsEntity {
}

pub static CLIENTPHYSICSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPhysicsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPHYSICSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPhysicsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPHYSICSENTITY_TYPE_INFO
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


pub static CLIENTPHYSICSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPhysicsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPhysicsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameComponentEntity {
    pub _glacier_base: super::gameplay_sim::GameComponentEntity,
}

pub trait ClientGameComponentEntityTrait: super::gameplay_sim::GameComponentEntityTrait {
}

impl ClientGameComponentEntityTrait for ClientGameComponentEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientGameComponentEntity {
}

impl super::entity::ComponentEntityTrait for ClientGameComponentEntity {
}

impl super::entity::SpatialEntityTrait for ClientGameComponentEntity {
}

impl super::entity::EntityTrait for ClientGameComponentEntity {
}

impl super::entity::EntityBusPeerTrait for ClientGameComponentEntity {
}

pub static CLIENTGAMECOMPONENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponentEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMECOMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameComponentEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMECOMPONENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameComponentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMECOMPONENTENTITY_TYPE_INFO
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


pub static CLIENTGAMECOMPONENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientGameComponentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameComponent {
    pub _glacier_base: super::gameplay_sim::GameComponent,
}

pub trait ClientGameComponentTrait: super::gameplay_sim::GameComponentTrait {
}

impl ClientGameComponentTrait for ClientGameComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientGameComponent {
}

impl super::entity::ComponentTrait for ClientGameComponent {
}

impl super::entity::EntityBusPeerTrait for ClientGameComponent {
}

pub static CLIENTGAMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMECOMPONENT_TYPE_INFO
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


pub static CLIENTGAMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientGameComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientControllableEntityCreatedEntityInfo {
}

pub trait ClientControllableEntityCreatedEntityInfoTrait: TypeObject {
}

impl ClientControllableEntityCreatedEntityInfoTrait for ClientControllableEntityCreatedEntityInfo {
}

pub static CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntityCreatedEntityInfo",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableEntityCreatedEntityInfo as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientControllableEntityCreatedEntityInfo {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_TYPE_INFO
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


pub static CLIENTCONTROLLABLEENTITYCREATEDENTITYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntityCreatedEntityInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientControllableEntityCreatedEntityInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientControllableEntity {
    pub _glacier_base: ClientPhysicsEntity,
}

pub trait ClientControllableEntityTrait: ClientPhysicsEntityTrait {
}

impl ClientControllableEntityTrait for ClientControllableEntity {
}

impl ClientPhysicsEntityTrait for ClientControllableEntity {
}

impl ClientGameComponentEntityTrait for ClientControllableEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientControllableEntity {
}

impl super::entity::ComponentEntityTrait for ClientControllableEntity {
}

impl super::entity::SpatialEntityTrait for ClientControllableEntity {
}

impl super::entity::EntityTrait for ClientControllableEntity {
}

impl super::entity::EntityBusPeerTrait for ClientControllableEntity {
}

pub static CLIENTCONTROLLABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONTROLLABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientControllableEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLEENTITY_TYPE_INFO
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


pub static CLIENTCONTROLLABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientControllableEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientEntryComponent {
    pub _glacier_base: ClientGameComponent,
}

pub trait ClientEntryComponentTrait: ClientGameComponentTrait {
}

impl ClientEntryComponentTrait for ClientEntryComponent {
}

impl ClientGameComponentTrait for ClientEntryComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientEntryComponent {
}

impl super::entity::ComponentTrait for ClientEntryComponent {
}

impl super::entity::EntityBusPeerTrait for ClientEntryComponent {
}

pub static CLIENTENTRYCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEntryComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTRYCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntryComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENTRYCOMPONENT_TYPE_INFO
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


pub static CLIENTENTRYCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntryComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientEntryComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientControllableHealthComponent {
    pub _glacier_base: ClientGameHealthComponent,
}

pub trait ClientControllableHealthComponentTrait: ClientGameHealthComponentTrait {
}

impl ClientControllableHealthComponentTrait for ClientControllableHealthComponent {
}

impl ClientGameHealthComponentTrait for ClientControllableHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientControllableHealthComponent {
}

impl super::entity::ComponentTrait for ClientControllableHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientControllableHealthComponent {
}

pub static CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientControllableHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientControllableHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTROLLABLEHEALTHCOMPONENT_TYPE_INFO
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


pub static CLIENTCONTROLLABLEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientControllableHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientControllableHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerExtent {
}

pub trait ClientPlayerExtentTrait: TypeObject {
}

impl ClientPlayerExtentTrait for ClientPlayerExtent {
}

pub static CLIENTPLAYEREXTENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerExtent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerExtent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYEREXTENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerExtent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYEREXTENT_TYPE_INFO
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


pub static CLIENTPLAYEREXTENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerExtent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPlayerExtent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnlineManager {
}

pub trait OnlineManagerTrait: TypeObject {
}

impl OnlineManagerTrait for OnlineManager {
}

pub static ONLINEMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineManager",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineManager as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONLINEMANAGER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnlineManager {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEMANAGER_TYPE_INFO
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


pub static ONLINEMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("OnlineManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientConnection {
    pub _glacier_base: super::network::EngineConnectionPeer,
}

pub trait ClientConnectionTrait: super::network::EngineConnectionPeerTrait {
}

impl ClientConnectionTrait for ClientConnection {
}

impl super::network::EngineConnectionPeerTrait for ClientConnection {
}

impl super::network::EngineConnectionTrait for ClientConnection {
}

pub static CLIENTCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnection",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::network::ENGINECONNECTIONPEER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnection as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientConnection {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONNECTION_TYPE_INFO
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


pub static CLIENTCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSpawnEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait ClientSpawnEntityTrait: super::entity::SpatialEntityTrait {
}

impl ClientSpawnEntityTrait for ClientSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ClientSpawnEntity {
}

impl super::entity::EntityTrait for ClientSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSpawnEntity {
}

pub static CLIENTSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSpawnEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSPAWNENTITY_TYPE_INFO
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


pub static CLIENTSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerEvent {
    pub _glacier_base: super::gameplay_sim::PlayerEventBase,
}

pub trait ClientPlayerEventTrait: super::gameplay_sim::PlayerEventBaseTrait {
}

impl ClientPlayerEventTrait for ClientPlayerEvent {
}

impl super::gameplay_sim::PlayerEventBaseTrait for ClientPlayerEvent {
}

impl super::entity::EntityEventTrait for ClientPlayerEvent {
}

pub static CLIENTPLAYEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::PLAYEREVENTBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayerEvent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYEREVENT_TYPE_INFO
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


pub static CLIENTPLAYEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPlayerEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDestructionComponent {
    pub _glacier_base: super::destruction::DestructionComponent,
}

pub trait ClientDestructionComponentTrait: super::destruction::DestructionComponentTrait {
}

impl ClientDestructionComponentTrait for ClientDestructionComponent {
}

impl super::destruction::DestructionComponentTrait for ClientDestructionComponent {
}

impl super::entity::ComponentTrait for ClientDestructionComponent {
}

impl super::entity::EntityBusPeerTrait for ClientDestructionComponent {
}

pub static CLIENTDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::destruction::DESTRUCTIONCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDestructionComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDestructionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDESTRUCTIONCOMPONENT_TYPE_INFO
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


pub static CLIENTDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientDestructionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayerUpdateCameraComponentMessage {
}

pub trait ClientPlayerUpdateCameraComponentMessageTrait: TypeObject {
}

impl ClientPlayerUpdateCameraComponentMessageTrait for ClientPlayerUpdateCameraComponentMessage {
}

pub static CLIENTPLAYERUPDATECAMERACOMPONENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerUpdateCameraComponentMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerUpdateCameraComponentMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerUpdateCameraComponentMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERUPDATECAMERACOMPONENTMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerSwitchTeamMessage {
}

pub trait ClientPlayerSwitchTeamMessageTrait: TypeObject {
}

impl ClientPlayerSwitchTeamMessageTrait for ClientPlayerSwitchTeamMessage {
}

pub static CLIENTPLAYERSWITCHTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerSwitchTeamMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerSwitchTeamMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerSwitchTeamMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERSWITCHTEAMMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerRequestCameraChangeMessage {
}

pub trait ClientPlayerRequestCameraChangeMessageTrait: TypeObject {
}

impl ClientPlayerRequestCameraChangeMessageTrait for ClientPlayerRequestCameraChangeMessage {
}

pub static CLIENTPLAYERREQUESTCAMERACHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerRequestCameraChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerRequestCameraChangeMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerRequestCameraChangeMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERREQUESTCAMERACHANGEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerLocalSetMessage {
}

pub trait ClientPlayerLocalSetMessageTrait: TypeObject {
}

impl ClientPlayerLocalSetMessageTrait for ClientPlayerLocalSetMessage {
}

pub static CLIENTPLAYERLOCALSETMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerLocalSetMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerLocalSetMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerLocalSetMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERLOCALSETMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerExitEntryMessage {
}

pub trait ClientPlayerExitEntryMessageTrait: TypeObject {
}

impl ClientPlayerExitEntryMessageTrait for ClientPlayerExitEntryMessage {
}

pub static CLIENTPLAYEREXITENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerExitEntryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerExitEntryMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerExitEntryMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYEREXITENTRYMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerEnterEntryMessage {
}

pub trait ClientPlayerEnterEntryMessageTrait: TypeObject {
}

impl ClientPlayerEnterEntryMessageTrait for ClientPlayerEnterEntryMessage {
}

pub static CLIENTPLAYERENTERENTRYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerEnterEntryMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerEnterEntryMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerEnterEntryMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERENTERENTRYMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerDeletedMessage {
}

pub trait ClientPlayerDeletedMessageTrait: TypeObject {
}

impl ClientPlayerDeletedMessageTrait for ClientPlayerDeletedMessage {
}

pub static CLIENTPLAYERDELETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerDeletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerDeletedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerDeletedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERDELETEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerConnectMessage {
}

pub trait ClientPlayerConnectMessageTrait: TypeObject {
}

impl ClientPlayerConnectMessageTrait for ClientPlayerConnectMessage {
}

pub static CLIENTPLAYERCONNECTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerConnectMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerConnectMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerConnectMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERCONNECTMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPlayerChangedPlayerViewMessage {
}

pub trait ClientPlayerChangedPlayerViewMessageTrait: TypeObject {
}

impl ClientPlayerChangedPlayerViewMessageTrait for ClientPlayerChangedPlayerViewMessage {
}

pub static CLIENTPLAYERCHANGEDPLAYERVIEWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayerChangedPlayerViewMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayerChangedPlayerViewMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientPlayerChangedPlayerViewMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYERCHANGEDPLAYERVIEWMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientSetServerPasswordMessage {
}

pub trait ClientSetServerPasswordMessageTrait: TypeObject {
}

impl ClientSetServerPasswordMessageTrait for ClientSetServerPasswordMessage {
}

pub static CLIENTSETSERVERPASSWORDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSetServerPasswordMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSetServerPasswordMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientSetServerPasswordMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSETSERVERPASSWORDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientWantFullscreenMessage {
}

pub trait ClientWantFullscreenMessageTrait: TypeObject {
}

impl ClientWantFullscreenMessageTrait for ClientWantFullscreenMessage {
}

pub static CLIENTWANTFULLSCREENMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWantFullscreenMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWantFullscreenMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientWantFullscreenMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWANTFULLSCREENMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLeftRemoteServerMessage {
}

pub trait ClientLeftRemoteServerMessageTrait: TypeObject {
}

impl ClientLeftRemoteServerMessageTrait for ClientLeftRemoteServerMessage {
}

pub static CLIENTLEFTREMOTESERVERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLeftRemoteServerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLeftRemoteServerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLeftRemoteServerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEFTREMOTESERVERMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientDisconnectedMessage {
}

pub trait ClientDisconnectedMessageTrait: TypeObject {
}

impl ClientDisconnectedMessageTrait for ClientDisconnectedMessage {
}

pub static CLIENTDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDisconnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDisconnectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientDisconnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDISCONNECTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientConnectedMessage {
}

pub trait ClientConnectedMessageTrait: TypeObject {
}

impl ClientConnectedMessageTrait for ClientConnectedMessage {
}

pub static CLIENTCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientConnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONNECTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientAbortCutsceneMessage {
}

pub trait ClientAbortCutsceneMessageTrait: TypeObject {
}

impl ClientAbortCutsceneMessageTrait for ClientAbortCutsceneMessage {
}

pub static CLIENTABORTCUTSCENEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAbortCutsceneMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAbortCutsceneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientAbortCutsceneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTABORTCUTSCENEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLevelLoadedMessage {
}

pub trait ClientLevelLoadedMessageTrait: TypeObject {
}

impl ClientLevelLoadedMessageTrait for ClientLevelLoadedMessage {
}

pub static CLIENTLEVELLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelLoadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLevelLoadedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEVELLOADEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLevelLoadProgressMessage {
}

pub trait ClientLevelLoadProgressMessageTrait: TypeObject {
}

impl ClientLevelLoadProgressMessageTrait for ClientLevelLoadProgressMessage {
}

pub static CLIENTLEVELLOADPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelLoadProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLevelLoadProgressMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelLoadProgressMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEVELLOADPROGRESSMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLevelDescriptionLoadedMessage {
}

pub trait ClientLevelDescriptionLoadedMessageTrait: TypeObject {
}

impl ClientLevelDescriptionLoadedMessageTrait for ClientLevelDescriptionLoadedMessage {
}

pub static CLIENTLEVELDESCRIPTIONLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelDescriptionLoadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLevelDescriptionLoadedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelDescriptionLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEVELDESCRIPTIONLOADEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLevelUnloadedMessage {
}

pub trait ClientLevelUnloadedMessageTrait: TypeObject {
}

impl ClientLevelUnloadedMessageTrait for ClientLevelUnloadedMessage {
}

pub static CLIENTLEVELUNLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelUnloadedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLevelUnloadedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelUnloadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEVELUNLOADEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLevelUnloadStartedMessage {
}

pub trait ClientLevelUnloadStartedMessageTrait: TypeObject {
}

impl ClientLevelUnloadStartedMessageTrait for ClientLevelUnloadStartedMessage {
}

pub static CLIENTLEVELUNLOADSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLevelUnloadStartedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLevelUnloadStartedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLevelUnloadStartedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEVELUNLOADSTARTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientQuittingFinishedMessage {
}

pub trait ClientQuittingFinishedMessageTrait: TypeObject {
}

impl ClientQuittingFinishedMessageTrait for ClientQuittingFinishedMessage {
}

pub static CLIENTQUITTINGFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuittingFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientQuittingFinishedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientQuittingFinishedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTQUITTINGFINISHEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientQuittingStartedMessage {
}

pub trait ClientQuittingStartedMessageTrait: TypeObject {
}

impl ClientQuittingStartedMessageTrait for ClientQuittingStartedMessage {
}

pub static CLIENTQUITTINGSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuittingStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientQuittingStartedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientQuittingStartedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTQUITTINGSTARTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientStartupFinishedMessage {
}

pub trait ClientStartupFinishedMessageTrait: TypeObject {
}

impl ClientStartupFinishedMessageTrait for ClientStartupFinishedMessage {
}

pub static CLIENTSTARTUPFINISHEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartupFinishedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStartupFinishedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStartupFinishedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTARTUPFINISHEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLoadLevelMessage {
}

pub trait ClientLoadLevelMessageTrait: TypeObject {
}

impl ClientLoadLevelMessageTrait for ClientLoadLevelMessage {
}

pub static CLIENTLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLoadLevelMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLoadLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLoadLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOADLEVELMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientLoadLevelRequestedMessage {
}

pub trait ClientLoadLevelRequestedMessageTrait: TypeObject {
}

impl ClientLoadLevelRequestedMessageTrait for ClientLoadLevelRequestedMessage {
}

pub static CLIENTLOADLEVELREQUESTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLoadLevelRequestedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLoadLevelRequestedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientLoadLevelRequestedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLOADLEVELREQUESTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientEnteredIngameMessage {
}

pub trait ClientEnteredIngameMessageTrait: TypeObject {
}

impl ClientEnteredIngameMessageTrait for ClientEnteredIngameMessage {
}

pub static CLIENTENTEREDINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEnteredIngameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEnteredIngameMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientEnteredIngameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENTEREDINGAMEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientEnterHudIngameMessage {
}

pub trait ClientEnterHudIngameMessageTrait: TypeObject {
}

impl ClientEnterHudIngameMessageTrait for ClientEnterHudIngameMessage {
}

pub static CLIENTENTERHUDINGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEnterHudIngameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEnterHudIngameMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientEnterHudIngameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENTERHUDINGAMEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientExitGameMessage {
}

pub trait ClientExitGameMessageTrait: TypeObject {
}

impl ClientExitGameMessageTrait for ClientExitGameMessage {
}

pub static CLIENTEXITGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExitGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientExitGameMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientExitGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEXITGAMEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientHostMigrationMessage {
}

pub trait ClientHostMigrationMessageTrait: TypeObject {
}

impl ClientHostMigrationMessageTrait for ClientHostMigrationMessage {
}

pub static CLIENTHOSTMIGRATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHostMigrationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientHostMigrationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientHostMigrationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTHOSTMIGRATIONMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientExitToMenuMessage {
}

pub trait ClientExitToMenuMessageTrait: TypeObject {
}

impl ClientExitToMenuMessageTrait for ClientExitToMenuMessage {
}

pub static CLIENTEXITTOMENUMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientExitToMenuMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientExitToMenuMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientExitToMenuMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEXITTOMENUMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientReturnToMenuMessage {
}

pub trait ClientReturnToMenuMessageTrait: TypeObject {
}

impl ClientReturnToMenuMessageTrait for ClientReturnToMenuMessage {
}

pub static CLIENTRETURNTOMENUMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientReturnToMenuMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientReturnToMenuMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientReturnToMenuMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTRETURNTOMENUMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientStartMultiplayerMessage {
}

pub trait ClientStartMultiplayerMessageTrait: TypeObject {
}

impl ClientStartMultiplayerMessageTrait for ClientStartMultiplayerMessage {
}

pub static CLIENTSTARTMULTIPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartMultiplayerMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStartMultiplayerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStartMultiplayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTARTMULTIPLAYERMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientContinueSingleplayerMessage {
}

pub trait ClientContinueSingleplayerMessageTrait: TypeObject {
}

impl ClientContinueSingleplayerMessageTrait for ClientContinueSingleplayerMessage {
}

pub static CLIENTCONTINUESINGLEPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientContinueSingleplayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientContinueSingleplayerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientContinueSingleplayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONTINUESINGLEPLAYERMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientStartedMessage {
}

pub trait ClientStartedMessageTrait: TypeObject {
}

impl ClientStartedMessageTrait for ClientStartedMessage {
}

pub static CLIENTSTARTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStartedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientStartedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTARTEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientJoinServerJobMessage {
}

pub trait ClientJoinServerJobMessageTrait: TypeObject {
}

impl ClientJoinServerJobMessageTrait for ClientJoinServerJobMessage {
}

pub static CLIENTJOINSERVERJOBMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJoinServerJobMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientJoinServerJobMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientJoinServerJobMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTJOINSERVERJOBMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientPeerNetworkRemovedMessageBase {
}

pub trait ClientPeerNetworkRemovedMessageBaseTrait: TypeObject {
}

impl ClientPeerNetworkRemovedMessageBaseTrait for ClientPeerNetworkRemovedMessageBase {
}

pub static CLIENTPEERNETWORKREMOVEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPeerNetworkRemovedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPeerNetworkRemovedMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientPeerNetworkRemovedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPEERNETWORKREMOVEDMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientJoinMultiplayerMessageBase {
}

pub trait ClientJoinMultiplayerMessageBaseTrait: TypeObject {
}

impl ClientJoinMultiplayerMessageBaseTrait for ClientJoinMultiplayerMessageBase {
}

pub static CLIENTJOINMULTIPLAYERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientJoinMultiplayerMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientJoinMultiplayerMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientJoinMultiplayerMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTJOINMULTIPLAYERMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientRestartSingleplayerMessageBase {
}

pub trait ClientRestartSingleplayerMessageBaseTrait: TypeObject {
}

impl ClientRestartSingleplayerMessageBaseTrait for ClientRestartSingleplayerMessageBase {
}

pub static CLIENTRESTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRestartSingleplayerMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientRestartSingleplayerMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientRestartSingleplayerMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTRESTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ClientStartSingleplayerMessageBase {
}

pub trait ClientStartSingleplayerMessageBaseTrait: TypeObject {
}

impl ClientStartSingleplayerMessageBaseTrait for ClientStartSingleplayerMessageBase {
}

pub static CLIENTSTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStartSingleplayerMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameplayClientServer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStartSingleplayerMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ClientStartSingleplayerMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTARTSINGLEPLAYERMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct ServerDestructionComponent {
    pub _glacier_base: super::destruction::DestructionComponent,
}

pub trait ServerDestructionComponentTrait: super::destruction::DestructionComponentTrait {
}

impl ServerDestructionComponentTrait for ServerDestructionComponent {
}

impl super::destruction::DestructionComponentTrait for ServerDestructionComponent {
}

impl super::entity::ComponentTrait for ServerDestructionComponent {
}

impl super::entity::EntityBusPeerTrait for ServerDestructionComponent {
}

pub static SERVERDESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::destruction::DESTRUCTIONCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDestructionComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDestructionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDESTRUCTIONCOMPONENT_TYPE_INFO
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


pub static SERVERDESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ServerDestructionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerGameComponent {
    pub _glacier_base: super::gameplay_sim::GameComponent,
}

pub trait ServerGameComponentTrait: super::gameplay_sim::GameComponentTrait {
}

impl ServerGameComponentTrait for ServerGameComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerGameComponent {
}

impl super::entity::ComponentTrait for ServerGameComponent {
}

impl super::entity::EntityBusPeerTrait for ServerGameComponent {
}

pub static SERVERGAMECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMECOMPONENT_TYPE_INFO
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


pub static SERVERGAMECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ServerGameComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ShaderParameterComponent {
    pub _glacier_base: ClientGameComponent,
}

pub trait ShaderParameterComponentTrait: ClientGameComponentTrait {
}

impl ShaderParameterComponentTrait for ShaderParameterComponent {
}

impl ClientGameComponentTrait for ShaderParameterComponent {
}

impl super::gameplay_sim::GameComponentTrait for ShaderParameterComponent {
}

impl super::entity::ComponentTrait for ShaderParameterComponent {
}

impl super::entity::EntityBusPeerTrait for ShaderParameterComponent {
}

pub static SHADERPARAMETERCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShaderParameterComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMETERCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderParameterComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SHADERPARAMETERCOMPONENT_TYPE_INFO
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


pub static SHADERPARAMETERCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParameterComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ShaderParameterComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPartComponent {
    pub _glacier_base: ClientGameComponent,
}

pub trait ClientPartComponentTrait: ClientGameComponentTrait {
}

impl ClientPartComponentTrait for ClientPartComponent {
}

impl ClientGameComponentTrait for ClientPartComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientPartComponent {
}

impl super::entity::ComponentTrait for ClientPartComponent {
}

impl super::entity::EntityBusPeerTrait for ClientPartComponent {
}

pub static CLIENTPARTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPartComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPARTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPartComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPARTCOMPONENT_TYPE_INFO
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


pub static CLIENTPARTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPartComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPart {
    pub _glacier_base: super::entity::Part,
}

pub trait ClientPartTrait: super::entity::PartTrait {
}

impl ClientPartTrait for ClientPart {
}

impl super::entity::PartTrait for ClientPart {
}

impl super::entity::ComponentTrait for ClientPart {
}

impl super::entity::EntityBusPeerTrait for ClientPart {
}

pub static CLIENTPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPart",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::PART_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPart as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPart {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPART_TYPE_INFO
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


pub static CLIENTPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientPart"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameHealthComponent {
    pub _glacier_base: super::gameplay_sim::HealthComponent,
}

pub trait ClientGameHealthComponentTrait: super::gameplay_sim::HealthComponentTrait {
}

impl ClientGameHealthComponentTrait for ClientGameHealthComponent {
}

impl super::gameplay_sim::HealthComponentTrait for ClientGameHealthComponent {
}

impl super::entity::ComponentTrait for ClientGameHealthComponent {
}

impl super::entity::EntityBusPeerTrait for ClientGameHealthComponent {
}

pub static CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameHealthComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::HEALTHCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameHealthComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameHealthComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEHEALTHCOMPONENT_TYPE_INFO
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


pub static CLIENTGAMEHEALTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameHealthComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientGameHealthComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBoneComponent {
    pub _glacier_base: ClientGameComponent,
}

pub trait ClientBoneComponentTrait: ClientGameComponentTrait {
}

impl ClientBoneComponentTrait for ClientBoneComponent {
}

impl ClientGameComponentTrait for ClientBoneComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientBoneComponent {
}

impl super::entity::ComponentTrait for ClientBoneComponent {
}

impl super::entity::EntityBusPeerTrait for ClientBoneComponent {
}

pub static CLIENTBONECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBoneComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBONECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBoneComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBONECOMPONENT_TYPE_INFO
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


pub static CLIENTBONECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBoneComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientBoneComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSubView {
    pub _glacier_base: super::gameplay_sim::SubView,
}

pub trait ClientSubViewTrait: super::gameplay_sim::SubViewTrait {
}

impl ClientSubViewTrait for ClientSubView {
}

impl super::gameplay_sim::SubViewTrait for ClientSubView {
}

pub static CLIENTSUBVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubView",
    flags: MemberInfoFlags::new(101),
    module: "GameplayClientServer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::SUBVIEW_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSubView as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSUBVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSubView {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSUBVIEW_TYPE_INFO
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


pub static CLIENTSUBVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSubView-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayClientServer",
    data: TypeInfoData::Array("ClientSubView"),
    array_type: None,
    alignment: 8,
};


