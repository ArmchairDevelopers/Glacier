use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ServerGameManagerAddQueuedPlayerMessage {
}

pub trait ServerGameManagerAddQueuedPlayerMessageTrait: TypeObject {
}

impl ServerGameManagerAddQueuedPlayerMessageTrait for ServerGameManagerAddQueuedPlayerMessage {
}

pub static SERVERGAMEMANAGERADDQUEUEDPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerAddQueuedPlayerMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerAddQueuedPlayerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerAddQueuedPlayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERADDQUEUEDPLAYERMESSAGE_TYPE_INFO
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
pub struct ServerGameManagerGameDestructingMessage {
}

pub trait ServerGameManagerGameDestructingMessageTrait: TypeObject {
}

impl ServerGameManagerGameDestructingMessageTrait for ServerGameManagerGameDestructingMessage {
}

pub static SERVERGAMEMANAGERGAMEDESTRUCTINGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameDestructingMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerGameDestructingMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerGameDestructingMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMEDESTRUCTINGMESSAGE_TYPE_INFO
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
pub struct ServerGameManagerPlayerRemovedMessageBase {
}

pub trait ServerGameManagerPlayerRemovedMessageBaseTrait: TypeObject {
}

impl ServerGameManagerPlayerRemovedMessageBaseTrait for ServerGameManagerPlayerRemovedMessageBase {
}

pub static SERVERGAMEMANAGERPLAYERREMOVEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerPlayerRemovedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerPlayerRemovedMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerPlayerRemovedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERPLAYERREMOVEDMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagerPlayerJoinCompleteMessage {
}

pub trait ServerGameManagerPlayerJoinCompleteMessageTrait: TypeObject {
}

impl ServerGameManagerPlayerJoinCompleteMessageTrait for ServerGameManagerPlayerJoinCompleteMessage {
}

pub static SERVERGAMEMANAGERPLAYERJOINCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerPlayerJoinCompleteMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerPlayerJoinCompleteMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerPlayerJoinCompleteMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERPLAYERJOINCOMPLETEMESSAGE_TYPE_INFO
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
pub struct ServerGameManagerPlayerJoiningMessageBase {
}

pub trait ServerGameManagerPlayerJoiningMessageBaseTrait: TypeObject {
}

impl ServerGameManagerPlayerJoiningMessageBaseTrait for ServerGameManagerPlayerJoiningMessageBase {
}

pub static SERVERGAMEMANAGERPLAYERJOININGMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerPlayerJoiningMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerPlayerJoiningMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerPlayerJoiningMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERPLAYERJOININGMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagerUpdateCapacityMessageBase {
}

pub trait ServerGameManagerUpdateCapacityMessageBaseTrait: TypeObject {
}

impl ServerGameManagerUpdateCapacityMessageBaseTrait for ServerGameManagerUpdateCapacityMessageBase {
}

pub static SERVERGAMEMANAGERUPDATECAPACITYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerUpdateCapacityMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerUpdateCapacityMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerUpdateCapacityMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERUPDATECAPACITYMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagerGameParametersChangedMessageBase {
}

pub trait ServerGameManagerGameParametersChangedMessageBaseTrait: TypeObject {
}

impl ServerGameManagerGameParametersChangedMessageBaseTrait for ServerGameManagerGameParametersChangedMessageBase {
}

pub static SERVERGAMEMANAGERGAMEPARAMETERSCHANGEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameParametersChangedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerGameParametersChangedMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerGameParametersChangedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMEPARAMETERSCHANGEDMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagerChangeGameParametersMessageBase {
}

pub trait ServerGameManagerChangeGameParametersMessageBaseTrait: TypeObject {
}

impl ServerGameManagerChangeGameParametersMessageBaseTrait for ServerGameManagerChangeGameParametersMessageBase {
}

pub static SERVERGAMEMANAGERCHANGEGAMEPARAMETERSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerChangeGameParametersMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerChangeGameParametersMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerChangeGameParametersMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERCHANGEGAMEPARAMETERSMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagerReconfigurableGameCreatedMessage {
}

pub trait ServerGameManagerReconfigurableGameCreatedMessageTrait: TypeObject {
}

impl ServerGameManagerReconfigurableGameCreatedMessageTrait for ServerGameManagerReconfigurableGameCreatedMessage {
}

pub static SERVERGAMEMANAGERRECONFIGURABLEGAMECREATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerReconfigurableGameCreatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerReconfigurableGameCreatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerReconfigurableGameCreatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERRECONFIGURABLEGAMECREATEDMESSAGE_TYPE_INFO
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
pub struct ServerGameManagerCreatingReconfigurableGameMessage {
}

pub trait ServerGameManagerCreatingReconfigurableGameMessageTrait: TypeObject {
}

impl ServerGameManagerCreatingReconfigurableGameMessageTrait for ServerGameManagerCreatingReconfigurableGameMessage {
}

pub static SERVERGAMEMANAGERCREATINGRECONFIGURABLEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerCreatingReconfigurableGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerCreatingReconfigurableGameMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerGameManagerCreatingReconfigurableGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERCREATINGRECONFIGURABLEGAMEMESSAGE_TYPE_INFO
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
pub struct ServerGameManagerGameResetMessageBase {
}

pub trait ServerGameManagerGameResetMessageBaseTrait: TypeObject {
}

impl ServerGameManagerGameResetMessageBaseTrait for ServerGameManagerGameResetMessageBase {
}

pub static SERVERGAMEMANAGERGAMERESETMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameResetMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerGameResetMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerGameResetMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMERESETMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagerGameCreatedMessageBase {
}

pub trait ServerGameManagerGameCreatedMessageBaseTrait: TypeObject {
}

impl ServerGameManagerGameCreatedMessageBaseTrait for ServerGameManagerGameCreatedMessageBase {
}

pub static SERVERGAMEMANAGERGAMECREATEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerGameCreatedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerGameCreatedMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerGameCreatedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERGAMECREATEDMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagerCreatingGameMessageBase {
}

pub trait ServerGameManagerCreatingGameMessageBaseTrait: TypeObject {
}

impl ServerGameManagerCreatingGameMessageBaseTrait for ServerGameManagerCreatingGameMessageBase {
}

pub static SERVERGAMEMANAGERCREATINGGAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagerCreatingGameMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagerCreatingGameMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerGameManagerCreatingGameMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGERCREATINGGAMEMESSAGEBASE_TYPE_INFO
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
pub struct ServerGameManagementBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub create_parameters: Option<Arc<Mutex<dyn GameParametersDataTrait>>>,
}

pub trait ServerGameManagementBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn create_parameters(&self) -> &Option<Arc<Mutex<dyn GameParametersDataTrait>>>;
    fn create_parameters_mut(&mut self) -> &mut Option<Arc<Mutex<dyn GameParametersDataTrait>>>;
}

impl ServerGameManagementBackendDataTrait for ServerGameManagementBackendData {
    fn create_parameters(&self) -> &Option<Arc<Mutex<dyn GameParametersDataTrait>>> {
        &self.create_parameters
    }
    fn create_parameters_mut(&mut self) -> &mut Option<Arc<Mutex<dyn GameParametersDataTrait>>> {
        &mut self.create_parameters
    }
}

impl super::online_shared::PresenceBackendDataTrait for ServerGameManagementBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for ServerGameManagementBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ServerGameManagementBackendData {
}

pub static SERVERGAMEMANAGEMENTBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagementBackendData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameManagementBackendData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CreateParameters",
                flags: MemberInfoFlags::new(0),
                field_type: "GameParametersData",
                rust_offset: offset_of!(ServerGameManagementBackendData, create_parameters),
            },
        ],
    }),
    array_type: Some(SERVERGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ServerGameManagementBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEMANAGEMENTBACKENDDATA_TYPE_INFO
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


pub static SERVERGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameManagementBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("ServerGameManagementBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GameParametersData {
    pub _glacier_base: super::core::Asset,
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
    pub role_configuration: Vec<Option<Arc<Mutex<dyn GameRoleInformationTrait>>>>,
    pub role: String,
    pub teams: i32,
}

pub trait GameParametersDataTrait: super::core::AssetTrait {
    fn server_name(&self) -> &String;
    fn server_name_mut(&mut self) -> &mut String;
    fn game_type(&self) -> &super::game_shared::PersistenceGameType;
    fn game_type_mut(&mut self) -> &mut super::game_shared::PersistenceGameType;
    fn open_to_invites(&self) -> &bool;
    fn open_to_invites_mut(&mut self) -> &mut bool;
    fn open_to_matchmaking(&self) -> &bool;
    fn open_to_matchmaking_mut(&mut self) -> &mut bool;
    fn open_to_join_by_player(&self) -> &bool;
    fn open_to_join_by_player_mut(&mut self) -> &mut bool;
    fn open_to_browsing(&self) -> &bool;
    fn open_to_browsing_mut(&mut self) -> &mut bool;
    fn ranked(&self) -> &bool;
    fn ranked_mut(&mut self) -> &mut bool;
    fn host_migratable(&self) -> &bool;
    fn host_migratable_mut(&mut self) -> &mut bool;
    fn join_in_progress_supported(&self) -> &bool;
    fn join_in_progress_supported_mut(&mut self) -> &mut bool;
    fn allow_any_reputation(&self) -> &bool;
    fn allow_any_reputation_mut(&mut self) -> &mut bool;
    fn dynamic_reputation_requirement(&self) -> &bool;
    fn dynamic_reputation_requirement_mut(&mut self) -> &mut bool;
    fn friends_bypass_closed_to_join_by_player(&self) -> &bool;
    fn friends_bypass_closed_to_join_by_player_mut(&mut self) -> &mut bool;
    fn queue_capacity(&self) -> &u32;
    fn queue_capacity_mut(&mut self) -> &mut u32;
    fn r#mod(&self) -> &u32;
    fn r#mod_mut(&mut self) -> &mut u32;
    fn max_spectators(&self) -> &u32;
    fn max_spectators_mut(&mut self) -> &mut u32;
    fn max_allowed_capacity(&self) -> &u32;
    fn max_allowed_capacity_mut(&mut self) -> &mut u32;
    fn default_max_players(&self) -> &u32;
    fn default_max_players_mut(&mut self) -> &mut u32;
    fn game_topology(&self) -> &GameNetworkTopology;
    fn game_topology_mut(&mut self) -> &mut GameNetworkTopology;
    fn peer_mode(&self) -> &GamePeer2PeerMode;
    fn peer_mode_mut(&mut self) -> &mut GamePeer2PeerMode;
    fn voip_topology(&self) -> &GameNetworkTopology;
    fn voip_topology_mut(&mut self) -> &mut GameNetworkTopology;
    fn attributes(&self) -> &Vec<GameAttributeData>;
    fn attributes_mut(&mut self) -> &mut Vec<GameAttributeData>;
    fn role_configuration(&self) -> &Vec<Option<Arc<Mutex<dyn GameRoleInformationTrait>>>>;
    fn role_configuration_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn GameRoleInformationTrait>>>>;
    fn role(&self) -> &String;
    fn role_mut(&mut self) -> &mut String;
    fn teams(&self) -> &i32;
    fn teams_mut(&mut self) -> &mut i32;
}

impl GameParametersDataTrait for GameParametersData {
    fn server_name(&self) -> &String {
        &self.server_name
    }
    fn server_name_mut(&mut self) -> &mut String {
        &mut self.server_name
    }
    fn game_type(&self) -> &super::game_shared::PersistenceGameType {
        &self.game_type
    }
    fn game_type_mut(&mut self) -> &mut super::game_shared::PersistenceGameType {
        &mut self.game_type
    }
    fn open_to_invites(&self) -> &bool {
        &self.open_to_invites
    }
    fn open_to_invites_mut(&mut self) -> &mut bool {
        &mut self.open_to_invites
    }
    fn open_to_matchmaking(&self) -> &bool {
        &self.open_to_matchmaking
    }
    fn open_to_matchmaking_mut(&mut self) -> &mut bool {
        &mut self.open_to_matchmaking
    }
    fn open_to_join_by_player(&self) -> &bool {
        &self.open_to_join_by_player
    }
    fn open_to_join_by_player_mut(&mut self) -> &mut bool {
        &mut self.open_to_join_by_player
    }
    fn open_to_browsing(&self) -> &bool {
        &self.open_to_browsing
    }
    fn open_to_browsing_mut(&mut self) -> &mut bool {
        &mut self.open_to_browsing
    }
    fn ranked(&self) -> &bool {
        &self.ranked
    }
    fn ranked_mut(&mut self) -> &mut bool {
        &mut self.ranked
    }
    fn host_migratable(&self) -> &bool {
        &self.host_migratable
    }
    fn host_migratable_mut(&mut self) -> &mut bool {
        &mut self.host_migratable
    }
    fn join_in_progress_supported(&self) -> &bool {
        &self.join_in_progress_supported
    }
    fn join_in_progress_supported_mut(&mut self) -> &mut bool {
        &mut self.join_in_progress_supported
    }
    fn allow_any_reputation(&self) -> &bool {
        &self.allow_any_reputation
    }
    fn allow_any_reputation_mut(&mut self) -> &mut bool {
        &mut self.allow_any_reputation
    }
    fn dynamic_reputation_requirement(&self) -> &bool {
        &self.dynamic_reputation_requirement
    }
    fn dynamic_reputation_requirement_mut(&mut self) -> &mut bool {
        &mut self.dynamic_reputation_requirement
    }
    fn friends_bypass_closed_to_join_by_player(&self) -> &bool {
        &self.friends_bypass_closed_to_join_by_player
    }
    fn friends_bypass_closed_to_join_by_player_mut(&mut self) -> &mut bool {
        &mut self.friends_bypass_closed_to_join_by_player
    }
    fn queue_capacity(&self) -> &u32 {
        &self.queue_capacity
    }
    fn queue_capacity_mut(&mut self) -> &mut u32 {
        &mut self.queue_capacity
    }
    fn r#mod(&self) -> &u32 {
        &self.r#mod
    }
    fn r#mod_mut(&mut self) -> &mut u32 {
        &mut self.r#mod
    }
    fn max_spectators(&self) -> &u32 {
        &self.max_spectators
    }
    fn max_spectators_mut(&mut self) -> &mut u32 {
        &mut self.max_spectators
    }
    fn max_allowed_capacity(&self) -> &u32 {
        &self.max_allowed_capacity
    }
    fn max_allowed_capacity_mut(&mut self) -> &mut u32 {
        &mut self.max_allowed_capacity
    }
    fn default_max_players(&self) -> &u32 {
        &self.default_max_players
    }
    fn default_max_players_mut(&mut self) -> &mut u32 {
        &mut self.default_max_players
    }
    fn game_topology(&self) -> &GameNetworkTopology {
        &self.game_topology
    }
    fn game_topology_mut(&mut self) -> &mut GameNetworkTopology {
        &mut self.game_topology
    }
    fn peer_mode(&self) -> &GamePeer2PeerMode {
        &self.peer_mode
    }
    fn peer_mode_mut(&mut self) -> &mut GamePeer2PeerMode {
        &mut self.peer_mode
    }
    fn voip_topology(&self) -> &GameNetworkTopology {
        &self.voip_topology
    }
    fn voip_topology_mut(&mut self) -> &mut GameNetworkTopology {
        &mut self.voip_topology
    }
    fn attributes(&self) -> &Vec<GameAttributeData> {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Vec<GameAttributeData> {
        &mut self.attributes
    }
    fn role_configuration(&self) -> &Vec<Option<Arc<Mutex<dyn GameRoleInformationTrait>>>> {
        &self.role_configuration
    }
    fn role_configuration_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn GameRoleInformationTrait>>>> {
        &mut self.role_configuration
    }
    fn role(&self) -> &String {
        &self.role
    }
    fn role_mut(&mut self) -> &mut String {
        &mut self.role
    }
    fn teams(&self) -> &i32 {
        &self.teams
    }
    fn teams_mut(&mut self) -> &mut i32 {
        &mut self.teams
    }
}

impl super::core::AssetTrait for GameParametersData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GameParametersData {
}

pub static GAMEPARAMETERSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParametersData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameParametersData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ServerName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameParametersData, server_name),
            },
            FieldInfoData {
                name: "GameType",
                flags: MemberInfoFlags::new(0),
                field_type: "PersistenceGameType",
                rust_offset: offset_of!(GameParametersData, game_type),
            },
            FieldInfoData {
                name: "OpenToInvites",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, open_to_invites),
            },
            FieldInfoData {
                name: "OpenToMatchmaking",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, open_to_matchmaking),
            },
            FieldInfoData {
                name: "OpenToJoinByPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, open_to_join_by_player),
            },
            FieldInfoData {
                name: "OpenToBrowsing",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, open_to_browsing),
            },
            FieldInfoData {
                name: "Ranked",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, ranked),
            },
            FieldInfoData {
                name: "HostMigratable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, host_migratable),
            },
            FieldInfoData {
                name: "JoinInProgressSupported",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, join_in_progress_supported),
            },
            FieldInfoData {
                name: "AllowAnyReputation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, allow_any_reputation),
            },
            FieldInfoData {
                name: "DynamicReputationRequirement",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, dynamic_reputation_requirement),
            },
            FieldInfoData {
                name: "FriendsBypassClosedToJoinByPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GameParametersData, friends_bypass_closed_to_join_by_player),
            },
            FieldInfoData {
                name: "QueueCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameParametersData, queue_capacity),
            },
            FieldInfoData {
                name: "Mod",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameParametersData, r#mod),
            },
            FieldInfoData {
                name: "MaxSpectators",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameParametersData, max_spectators),
            },
            FieldInfoData {
                name: "MaxAllowedCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameParametersData, max_allowed_capacity),
            },
            FieldInfoData {
                name: "DefaultMaxPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameParametersData, default_max_players),
            },
            FieldInfoData {
                name: "GameTopology",
                flags: MemberInfoFlags::new(0),
                field_type: "GameNetworkTopology",
                rust_offset: offset_of!(GameParametersData, game_topology),
            },
            FieldInfoData {
                name: "PeerMode",
                flags: MemberInfoFlags::new(0),
                field_type: "GamePeer2PeerMode",
                rust_offset: offset_of!(GameParametersData, peer_mode),
            },
            FieldInfoData {
                name: "VoipTopology",
                flags: MemberInfoFlags::new(0),
                field_type: "GameNetworkTopology",
                rust_offset: offset_of!(GameParametersData, voip_topology),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: "GameAttributeData-Array",
                rust_offset: offset_of!(GameParametersData, attributes),
            },
            FieldInfoData {
                name: "RoleConfiguration",
                flags: MemberInfoFlags::new(144),
                field_type: "GameRoleInformation-Array",
                rust_offset: offset_of!(GameParametersData, role_configuration),
            },
            FieldInfoData {
                name: "Role",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameParametersData, role),
            },
            FieldInfoData {
                name: "Teams",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GameParametersData, teams),
            },
        ],
    }),
    array_type: Some(GAMEPARAMETERSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameParametersData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPARAMETERSDATA_TYPE_INFO
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


pub static GAMEPARAMETERSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParametersData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameParametersData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GameRoleInformation {
    pub _glacier_base: super::core::DataContainer,
    pub role_name: String,
    pub capacity: u32,
}

pub trait GameRoleInformationTrait: super::core::DataContainerTrait {
    fn role_name(&self) -> &String;
    fn role_name_mut(&mut self) -> &mut String;
    fn capacity(&self) -> &u32;
    fn capacity_mut(&mut self) -> &mut u32;
}

impl GameRoleInformationTrait for GameRoleInformation {
    fn role_name(&self) -> &String {
        &self.role_name
    }
    fn role_name_mut(&mut self) -> &mut String {
        &mut self.role_name
    }
    fn capacity(&self) -> &u32 {
        &self.capacity
    }
    fn capacity_mut(&mut self) -> &mut u32 {
        &mut self.capacity
    }
}

impl super::core::DataContainerTrait for GameRoleInformation {
}

pub static GAMEROLEINFORMATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameRoleInformation",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameRoleInformation as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RoleName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameRoleInformation, role_name),
            },
            FieldInfoData {
                name: "Capacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(GameRoleInformation, capacity),
            },
        ],
    }),
    array_type: Some(GAMEROLEINFORMATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameRoleInformation {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEROLEINFORMATION_TYPE_INFO
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


pub static GAMEROLEINFORMATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameRoleInformation-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameRoleInformation"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GameAttributeData {
    pub attribute: String,
    pub value: String,
}

pub trait GameAttributeDataTrait: TypeObject {
    fn attribute(&self) -> &String;
    fn attribute_mut(&mut self) -> &mut String;
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
}

impl GameAttributeDataTrait for GameAttributeData {
    fn attribute(&self) -> &String {
        &self.attribute
    }
    fn attribute_mut(&mut self) -> &mut String {
        &mut self.attribute
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

pub static GAMEATTRIBUTEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAttributeData",
    flags: MemberInfoFlags::new(73),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameAttributeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Attribute",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameAttributeData, attribute),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(GameAttributeData, value),
            },
        ],
    }),
    array_type: Some(GAMEATTRIBUTEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameAttributeData {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEATTRIBUTEDATA_TYPE_INFO
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


pub static GAMEATTRIBUTEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameAttributeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameAttributeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GamePeer2PeerMode {
    #[default]
    GamePeer2PeerMode_FullMesh = 0,
    GamePeer2PeerMode_PartialMesh = 1,
}

pub static GAMEPEER2PEERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePeer2PeerMode",
    flags: MemberInfoFlags::new(49429),
    module: "GameManagement",
    data: TypeInfoData::Enum,
    array_type: Some(GAMEPEER2PEERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GamePeer2PeerMode {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPEER2PEERMODE_TYPE_INFO
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


pub static GAMEPEER2PEERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePeer2PeerMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GamePeer2PeerMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GameNetworkTopology {
    #[default]
    GameNetworkTopology_Disabled = 0,
    GameNetworkTopology_Peer2Peer = 1,
    GameNetworkTopology_PeerHosted = 2,
    GameNetworkTopology_DedicatedServer = 3,
}

pub static GAMENETWORKTOPOLOGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameNetworkTopology",
    flags: MemberInfoFlags::new(49429),
    module: "GameManagement",
    data: TypeInfoData::Enum,
    array_type: Some(GAMENETWORKTOPOLOGY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GameNetworkTopology {
    fn type_info(&self) -> &'static TypeInfo {
        GAMENETWORKTOPOLOGY_TYPE_INFO
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


pub static GAMENETWORKTOPOLOGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameNetworkTopology-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameNetworkTopology"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGameManagementServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
    pub gamegroup_max_participants: i32,
    pub join_game_session_after_gamegroup: bool,
    pub default_game_role_name: String,
}

pub trait PresenceGameManagementServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
    fn gamegroup_max_participants(&self) -> &i32;
    fn gamegroup_max_participants_mut(&mut self) -> &mut i32;
    fn join_game_session_after_gamegroup(&self) -> &bool;
    fn join_game_session_after_gamegroup_mut(&mut self) -> &mut bool;
    fn default_game_role_name(&self) -> &String;
    fn default_game_role_name_mut(&mut self) -> &mut String;
}

impl PresenceGameManagementServiceDataTrait for PresenceGameManagementServiceData {
    fn gamegroup_max_participants(&self) -> &i32 {
        &self.gamegroup_max_participants
    }
    fn gamegroup_max_participants_mut(&mut self) -> &mut i32 {
        &mut self.gamegroup_max_participants
    }
    fn join_game_session_after_gamegroup(&self) -> &bool {
        &self.join_game_session_after_gamegroup
    }
    fn join_game_session_after_gamegroup_mut(&mut self) -> &mut bool {
        &mut self.join_game_session_after_gamegroup
    }
    fn default_game_role_name(&self) -> &String {
        &self.default_game_role_name
    }
    fn default_game_role_name_mut(&mut self) -> &mut String {
        &mut self.default_game_role_name
    }
}

impl super::online_shared::PresenceServiceDataTrait for PresenceGameManagementServiceData {
}

impl super::core::AssetTrait for PresenceGameManagementServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceGameManagementServiceData {
}

pub static PRESENCEGAMEMANAGEMENTSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementServiceData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGameManagementServiceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GamegroupMaxParticipants",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PresenceGameManagementServiceData, gamegroup_max_participants),
            },
            FieldInfoData {
                name: "JoinGameSessionAfterGamegroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PresenceGameManagementServiceData, join_game_session_after_gamegroup),
            },
            FieldInfoData {
                name: "DefaultGameRoleName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PresenceGameManagementServiceData, default_game_role_name),
            },
        ],
    }),
    array_type: Some(PRESENCEGAMEMANAGEMENTSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceGameManagementServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGAMEMANAGEMENTSERVICEDATA_TYPE_INFO
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


pub static PRESENCEGAMEMANAGEMENTSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("PresenceGameManagementServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGameManagementBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub use_game_info_strategy: bool,
}

pub trait PresenceGameManagementBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn use_game_info_strategy(&self) -> &bool;
    fn use_game_info_strategy_mut(&mut self) -> &mut bool;
}

impl PresenceGameManagementBackendDataTrait for PresenceGameManagementBackendData {
    fn use_game_info_strategy(&self) -> &bool {
        &self.use_game_info_strategy
    }
    fn use_game_info_strategy_mut(&mut self) -> &mut bool {
        &mut self.use_game_info_strategy
    }
}

impl super::online_shared::PresenceBackendDataTrait for PresenceGameManagementBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for PresenceGameManagementBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceGameManagementBackendData {
}

pub static PRESENCEGAMEMANAGEMENTBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementBackendData",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGameManagementBackendData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UseGameInfoStrategy",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PresenceGameManagementBackendData, use_game_info_strategy),
            },
        ],
    }),
    array_type: Some(PRESENCEGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceGameManagementBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGAMEMANAGEMENTBACKENDDATA_TYPE_INFO
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


pub static PRESENCEGAMEMANAGEMENTBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameManagementBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("PresenceGameManagementBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGameQueueMessageBase {
}

pub trait PresenceGameQueueMessageBaseTrait: TypeObject {
}

impl PresenceGameQueueMessageBaseTrait for PresenceGameQueueMessageBase {
}

pub static PRESENCEGAMEQUEUEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameQueueMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGameQueueMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameQueueMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGAMEQUEUEMESSAGEBASE_TYPE_INFO
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
pub struct PresenceGameNotificationMessageBase {
}

pub trait PresenceGameNotificationMessageBaseTrait: TypeObject {
}

impl PresenceGameNotificationMessageBaseTrait for PresenceGameNotificationMessageBase {
}

pub static PRESENCEGAMENOTIFICATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameNotificationMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGameNotificationMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameNotificationMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGAMENOTIFICATIONMESSAGEBASE_TYPE_INFO
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
pub struct PresenceGameMessageBase {
}

pub trait PresenceGameMessageBaseTrait: TypeObject {
}

impl PresenceGameMessageBaseTrait for PresenceGameMessageBase {
}

pub static PRESENCEGAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGameMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGAMEMESSAGEBASE_TYPE_INFO
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
pub struct PresenceGameRequestMessageBase {
}

pub trait PresenceGameRequestMessageBaseTrait: TypeObject {
}

impl PresenceGameRequestMessageBaseTrait for PresenceGameRequestMessageBase {
}

pub static PRESENCEGAMEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGameRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameManagement",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGameRequestMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGameRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGAMEREQUESTMESSAGEBASE_TYPE_INFO
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
pub struct GameParameters {
}

pub trait GameParametersTrait: TypeObject {
}

impl GameParametersTrait for GameParameters {
}

pub static GAMEPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParameters",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMEPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameParameters {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPARAMETERS_TYPE_INFO
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


pub static GAMEPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("GameParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceCreateGameRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceCreateGameRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceCreateGameRequestParametersTrait for PresenceCreateGameRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceCreateGameRequestParameters {
}

pub static PRESENCECREATEGAMEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreateGameRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceCreateGameRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECREATEGAMEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceCreateGameRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCECREATEGAMEREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCECREATEGAMEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreateGameRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("PresenceCreateGameRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameManagementService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientGameManagementServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientGameManagementServiceTrait for ClientGameManagementService {
}

impl super::online::PresenceServiceTrait for ClientGameManagementService {
}

pub static CLIENTGAMEMANAGEMENTSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementService",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameManagementService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEMANAGEMENTSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameManagementService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEMANAGEMENTSERVICE_TYPE_INFO
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


pub static CLIENTGAMEMANAGEMENTSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementService-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("ClientGameManagementService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientGameManagementBackend {
    pub _glacier_base: super::online::PresenceBackend,
}

pub trait ClientGameManagementBackendTrait: super::online::PresenceBackendTrait {
}

impl ClientGameManagementBackendTrait for ClientGameManagementBackend {
}

impl super::online::PresenceBackendTrait for ClientGameManagementBackend {
}

pub static CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementBackend",
    flags: MemberInfoFlags::new(101),
    module: "GameManagement",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientGameManagementBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientGameManagementBackend {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTGAMEMANAGEMENTBACKEND_TYPE_INFO
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


pub static CLIENTGAMEMANAGEMENTBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientGameManagementBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameManagement",
    data: TypeInfoData::Array("ClientGameManagementBackend"),
    array_type: None,
    alignment: 8,
};


