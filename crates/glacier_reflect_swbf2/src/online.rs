use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_online_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERGAMEBACKEND_TYPE_INFO);
    registry.register_type(SERVERGAMEBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCESTRATEGY_TYPE_INFO);
    registry.register_type(PRESENCESTRATEGY_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEBACKEND_TYPE_INFO);
    registry.register_type(PRESENCEBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(BLOCKLISTEVENT_TYPE_INFO);
    registry.register_type(BLOCKLISTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ONMODECHANGEDEVENT_TYPE_INFO);
    registry.register_type(ONMODECHANGEDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ONCONNECTFAILEDEVENT_TYPE_INFO);
    registry.register_type(ONCONNECTFAILEDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ONDISCONNECTEDEVENT_TYPE_INFO);
    registry.register_type(ONDISCONNECTEDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ONCONNECTEDEVENT_TYPE_INFO);
    registry.register_type(ONCONNECTEDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ONCONNECTINGEVENT_TYPE_INFO);
    registry.register_type(ONCONNECTINGEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ONLINESTATUSEVENT_TYPE_INFO);
    registry.register_type(ONLINESTATUSEVENT_ARRAY_TYPE_INFO);
    registry.register_type(FIRSTPARTYNETWORKSTATUSEVENT_TYPE_INFO);
    registry.register_type(FIRSTPARTYNETWORKSTATUSEVENT_ARRAY_TYPE_INFO);
    registry.register_type(FRIENDUPDATEDEVENT_TYPE_INFO);
    registry.register_type(FRIENDUPDATEDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(MULTIPLAYERPRIVILEGEEVENT_TYPE_INFO);
    registry.register_type(MULTIPLAYERPRIVILEGEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(FIRSTPARTYUSEREVENT_TYPE_INFO);
    registry.register_type(FIRSTPARTYUSEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(BACKENDSTATECHANGEEVENT_TYPE_INFO);
    registry.register_type(BACKENDSTATECHANGEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(CANCELHTTPREQUESTEVENT_TYPE_INFO);
    registry.register_type(CANCELHTTPREQUESTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(DIRTYSOCKPRESENCEBACKEND_TYPE_INFO);
    registry.register_type(DIRTYSOCKPRESENCEBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(GAMESTATESTRATEGY_TYPE_INFO);
    registry.register_type(GAMESTATESTRATEGY_ARRAY_TYPE_INFO);
    registry.register_type(BLAZEPRESENCEBACKEND_TYPE_INFO);
    registry.register_type(BLAZEPRESENCEBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLOBSERVICE_TYPE_INFO);
    registry.register_type(CLIENTBLOBSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTAUTHENTICATIONSERVICE_TYPE_INFO);
    registry.register_type(CLIENTAUTHENTICATIONSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEFILTERPROFANITYREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEFILTERPROFANITYREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETUSERPROFILESREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETUSERPROFILESREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETUSERIDREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETUSERIDREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEVIEWINVITEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEVIEWINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCESENDINVITEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCESENDINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEHTTPPOSTREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEHTTPPOSTREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEHTTPGETREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEHTTPGETREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETBLOCKLISTREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETBLOCKLISTREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEREMOVEFRIENDREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEREMOVEFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEACCEPTFRIENDREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEACCEPTFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETAUTHCODEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETAUTHCODEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(ORIGINPRESENCEBACKEND_TYPE_INFO);
    registry.register_type(ORIGINPRESENCEBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(BLAZESTATENOTIFICATIONEVENT_TYPE_INFO);
    registry.register_type(BLAZESTATENOTIFICATIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(BLAZEGAMESTRATEGY_TYPE_INFO);
    registry.register_type(BLAZEGAMESTRATEGY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTUSERPROFILESERVICE_TYPE_INFO);
    registry.register_type(CLIENTUSERPROFILESERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTUSERIDSERVICE_TYPE_INFO);
    registry.register_type(CLIENTUSERIDSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPROFANITYFILTERSERVICE_TYPE_INFO);
    registry.register_type(CLIENTPROFANITYFILTERSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPRIVILEGESERVICE_TYPE_INFO);
    registry.register_type(CLIENTPRIVILEGESERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTINVITESERVICE_TYPE_INFO);
    registry.register_type(CLIENTINVITESERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFRIENDSSERVICE_TYPE_INFO);
    registry.register_type(CLIENTFRIENDSSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCONNECTIONSERVICE_TYPE_INFO);
    registry.register_type(CLIENTCONNECTIONSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCESERVICE_TYPE_INFO);
    registry.register_type(PRESENCESERVICE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEEVENT_TYPE_INFO);
    registry.register_type(PRESENCEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEINTERNALCURRENTUSERCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(ONLINEINTERNALFRIENDSMESSAGEBASE_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct ServerGameBackend {
    pub _glacier_base: PresenceBackend,
}

pub trait ServerGameBackendTrait: PresenceBackendTrait {
}

impl ServerGameBackendTrait for ServerGameBackend {
}

impl PresenceBackendTrait for ServerGameBackend {
}

pub static SERVERGAMEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameBackend {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERGAMEBACKEND_TYPE_INFO
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


pub static SERVERGAMEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ServerGameBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceStrategy {
}

pub trait PresenceStrategyTrait: TypeObject {
}

impl PresenceStrategyTrait for PresenceStrategy {
}

pub static PRESENCESTRATEGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStrategy",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStrategy as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESTRATEGY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceStrategy {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESTRATEGY_TYPE_INFO
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


pub static PRESENCESTRATEGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStrategy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceStrategy"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceBackend {
}

pub trait PresenceBackendTrait: TypeObject {
}

impl PresenceBackendTrait for PresenceBackend {
}

pub static PRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceBackend {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBACKEND_TYPE_INFO
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


pub static PRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlockListEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait BlockListEventTrait: PresenceEventTrait {
}

impl BlockListEventTrait for BlockListEvent {
}

impl PresenceEventTrait for BlockListEvent {
}

pub static BLOCKLISTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockListEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlockListEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLOCKLISTEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlockListEvent {
    fn type_info(&self) -> &'static TypeInfo {
        BLOCKLISTEVENT_TYPE_INFO
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


pub static BLOCKLISTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockListEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlockListEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnModeChangedEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait OnModeChangedEventTrait: PresenceEventTrait {
}

impl OnModeChangedEventTrait for OnModeChangedEvent {
}

impl PresenceEventTrait for OnModeChangedEvent {
}

pub static ONMODECHANGEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnModeChangedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnModeChangedEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONMODECHANGEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnModeChangedEvent {
    fn type_info(&self) -> &'static TypeInfo {
        ONMODECHANGEDEVENT_TYPE_INFO
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


pub static ONMODECHANGEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnModeChangedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnModeChangedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnConnectFailedEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait OnConnectFailedEventTrait: PresenceEventTrait {
}

impl OnConnectFailedEventTrait for OnConnectFailedEvent {
}

impl PresenceEventTrait for OnConnectFailedEvent {
}

pub static ONCONNECTFAILEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectFailedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnConnectFailedEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONCONNECTFAILEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnConnectFailedEvent {
    fn type_info(&self) -> &'static TypeInfo {
        ONCONNECTFAILEDEVENT_TYPE_INFO
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


pub static ONCONNECTFAILEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectFailedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectFailedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnDisconnectedEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait OnDisconnectedEventTrait: PresenceEventTrait {
}

impl OnDisconnectedEventTrait for OnDisconnectedEvent {
}

impl PresenceEventTrait for OnDisconnectedEvent {
}

pub static ONDISCONNECTEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnDisconnectedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnDisconnectedEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONDISCONNECTEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnDisconnectedEvent {
    fn type_info(&self) -> &'static TypeInfo {
        ONDISCONNECTEDEVENT_TYPE_INFO
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


pub static ONDISCONNECTEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnDisconnectedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnDisconnectedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnConnectedEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait OnConnectedEventTrait: PresenceEventTrait {
}

impl OnConnectedEventTrait for OnConnectedEvent {
}

impl PresenceEventTrait for OnConnectedEvent {
}

pub static ONCONNECTEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnConnectedEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONCONNECTEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnConnectedEvent {
    fn type_info(&self) -> &'static TypeInfo {
        ONCONNECTEDEVENT_TYPE_INFO
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


pub static ONCONNECTEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnConnectingEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait OnConnectingEventTrait: PresenceEventTrait {
}

impl OnConnectingEventTrait for OnConnectingEvent {
}

impl PresenceEventTrait for OnConnectingEvent {
}

pub static ONCONNECTINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectingEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnConnectingEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONCONNECTINGEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnConnectingEvent {
    fn type_info(&self) -> &'static TypeInfo {
        ONCONNECTINGEVENT_TYPE_INFO
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


pub static ONCONNECTINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectingEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectingEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnlineStatusEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait OnlineStatusEventTrait: PresenceEventTrait {
}

impl OnlineStatusEventTrait for OnlineStatusEvent {
}

impl PresenceEventTrait for OnlineStatusEvent {
}

pub static ONLINESTATUSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineStatusEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineStatusEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONLINESTATUSEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnlineStatusEvent {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINESTATUSEVENT_TYPE_INFO
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


pub static ONLINESTATUSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineStatusEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnlineStatusEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FirstPartyNetworkStatusEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait FirstPartyNetworkStatusEventTrait: PresenceEventTrait {
}

impl FirstPartyNetworkStatusEventTrait for FirstPartyNetworkStatusEvent {
}

impl PresenceEventTrait for FirstPartyNetworkStatusEvent {
}

pub static FIRSTPARTYNETWORKSTATUSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyNetworkStatusEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FirstPartyNetworkStatusEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FIRSTPARTYNETWORKSTATUSEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FirstPartyNetworkStatusEvent {
    fn type_info(&self) -> &'static TypeInfo {
        FIRSTPARTYNETWORKSTATUSEVENT_TYPE_INFO
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


pub static FIRSTPARTYNETWORKSTATUSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyNetworkStatusEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FirstPartyNetworkStatusEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FriendUpdatedEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait FriendUpdatedEventTrait: PresenceEventTrait {
}

impl FriendUpdatedEventTrait for FriendUpdatedEvent {
}

impl PresenceEventTrait for FriendUpdatedEvent {
}

pub static FRIENDUPDATEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FriendUpdatedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FriendUpdatedEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FRIENDUPDATEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FriendUpdatedEvent {
    fn type_info(&self) -> &'static TypeInfo {
        FRIENDUPDATEDEVENT_TYPE_INFO
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


pub static FRIENDUPDATEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FriendUpdatedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FriendUpdatedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MultiplayerPrivilegeEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait MultiplayerPrivilegeEventTrait: PresenceEventTrait {
}

impl MultiplayerPrivilegeEventTrait for MultiplayerPrivilegeEvent {
}

impl PresenceEventTrait for MultiplayerPrivilegeEvent {
}

pub static MULTIPLAYERPRIVILEGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerPrivilegeEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiplayerPrivilegeEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MULTIPLAYERPRIVILEGEEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiplayerPrivilegeEvent {
    fn type_info(&self) -> &'static TypeInfo {
        MULTIPLAYERPRIVILEGEEVENT_TYPE_INFO
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


pub static MULTIPLAYERPRIVILEGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerPrivilegeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("MultiplayerPrivilegeEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FirstPartyUserEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait FirstPartyUserEventTrait: PresenceEventTrait {
}

impl FirstPartyUserEventTrait for FirstPartyUserEvent {
}

impl PresenceEventTrait for FirstPartyUserEvent {
}

pub static FIRSTPARTYUSEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyUserEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FirstPartyUserEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FIRSTPARTYUSEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FirstPartyUserEvent {
    fn type_info(&self) -> &'static TypeInfo {
        FIRSTPARTYUSEREVENT_TYPE_INFO
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


pub static FIRSTPARTYUSEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyUserEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FirstPartyUserEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BackendStateChangeEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait BackendStateChangeEventTrait: PresenceEventTrait {
}

impl BackendStateChangeEventTrait for BackendStateChangeEvent {
}

impl PresenceEventTrait for BackendStateChangeEvent {
}

pub static BACKENDSTATECHANGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BackendStateChangeEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BackendStateChangeEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BACKENDSTATECHANGEEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BackendStateChangeEvent {
    fn type_info(&self) -> &'static TypeInfo {
        BACKENDSTATECHANGEEVENT_TYPE_INFO
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


pub static BACKENDSTATECHANGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BackendStateChangeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BackendStateChangeEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CancelHttpRequestEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait CancelHttpRequestEventTrait: PresenceEventTrait {
}

impl CancelHttpRequestEventTrait for CancelHttpRequestEvent {
}

impl PresenceEventTrait for CancelHttpRequestEvent {
}

pub static CANCELHTTPREQUESTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CancelHttpRequestEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CancelHttpRequestEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CANCELHTTPREQUESTEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CancelHttpRequestEvent {
    fn type_info(&self) -> &'static TypeInfo {
        CANCELHTTPREQUESTEVENT_TYPE_INFO
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


pub static CANCELHTTPREQUESTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CancelHttpRequestEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("CancelHttpRequestEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DirtySockPresenceBackend {
    pub _glacier_base: PresenceBackend,
}

pub trait DirtySockPresenceBackendTrait: PresenceBackendTrait {
}

impl DirtySockPresenceBackendTrait for DirtySockPresenceBackend {
}

impl PresenceBackendTrait for DirtySockPresenceBackend {
}

pub static DIRTYSOCKPRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DirtySockPresenceBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DIRTYSOCKPRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DirtySockPresenceBackend {
    fn type_info(&self) -> &'static TypeInfo {
        DIRTYSOCKPRESENCEBACKEND_TYPE_INFO
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


pub static DIRTYSOCKPRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("DirtySockPresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GameStateStrategy {
    pub _glacier_base: PresenceStrategy,
}

pub trait GameStateStrategyTrait: PresenceStrategyTrait {
}

impl GameStateStrategyTrait for GameStateStrategy {
}

impl PresenceStrategyTrait for GameStateStrategy {
}

pub static GAMESTATESTRATEGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateStrategy",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESTRATEGY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameStateStrategy as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMESTATESTRATEGY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameStateStrategy {
    fn type_info(&self) -> &'static TypeInfo {
        GAMESTATESTRATEGY_TYPE_INFO
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


pub static GAMESTATESTRATEGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateStrategy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("GameStateStrategy"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlazePresenceBackend {
    pub _glacier_base: PresenceBackend,
}

pub trait BlazePresenceBackendTrait: PresenceBackendTrait {
}

impl BlazePresenceBackendTrait for BlazePresenceBackend {
}

impl PresenceBackendTrait for BlazePresenceBackend {
}

pub static BLAZEPRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazePresenceBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLAZEPRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlazePresenceBackend {
    fn type_info(&self) -> &'static TypeInfo {
        BLAZEPRESENCEBACKEND_TYPE_INFO
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


pub static BLAZEPRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazePresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBlobService {
    pub _glacier_base: PresenceService,
}

pub trait ClientBlobServiceTrait: PresenceServiceTrait {
}

impl ClientBlobServiceTrait for ClientBlobService {
}

impl PresenceServiceTrait for ClientBlobService {
}

pub static CLIENTBLOBSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlobService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlobService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLOBSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlobService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLOBSERVICE_TYPE_INFO
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


pub static CLIENTBLOBSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlobService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientBlobService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAuthenticationService {
    pub _glacier_base: PresenceService,
}

pub trait ClientAuthenticationServiceTrait: PresenceServiceTrait {
}

impl ClientAuthenticationServiceTrait for ClientAuthenticationService {
}

impl PresenceServiceTrait for ClientAuthenticationService {
}

pub static CLIENTAUTHENTICATIONSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAuthenticationService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAuthenticationService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAUTHENTICATIONSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAuthenticationService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTAUTHENTICATIONSERVICE_TYPE_INFO
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


pub static CLIENTAUTHENTICATIONSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAuthenticationService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientAuthenticationService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceFilterProfanityRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceFilterProfanityRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceFilterProfanityRequestParametersTrait for PresenceFilterProfanityRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceFilterProfanityRequestParameters {
}

pub static PRESENCEFILTERPROFANITYREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFilterProfanityRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceFilterProfanityRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEFILTERPROFANITYREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceFilterProfanityRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEFILTERPROFANITYREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEFILTERPROFANITYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFilterProfanityRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceFilterProfanityRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceAcquireMultiplayerPrivilegeRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceAcquireMultiplayerPrivilegeRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceAcquireMultiplayerPrivilegeRequestParametersTrait for PresenceAcquireMultiplayerPrivilegeRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceAcquireMultiplayerPrivilegeRequestParameters {
}

pub static PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcquireMultiplayerPrivilegeRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAcquireMultiplayerPrivilegeRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceAcquireMultiplayerPrivilegeRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcquireMultiplayerPrivilegeRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceAcquireMultiplayerPrivilegeRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceCheckPrivilegesRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceCheckPrivilegesRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceCheckPrivilegesRequestParametersTrait for PresenceCheckPrivilegesRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceCheckPrivilegesRequestParameters {
}

pub static PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCheckPrivilegesRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceCheckPrivilegesRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceCheckPrivilegesRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCheckPrivilegesRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceCheckPrivilegesRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGetUserProfilesRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceGetUserProfilesRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceGetUserProfilesRequestParametersTrait for PresenceGetUserProfilesRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceGetUserProfilesRequestParameters {
}

pub static PRESENCEGETUSERPROFILESREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserProfilesRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetUserProfilesRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETUSERPROFILESREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetUserProfilesRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETUSERPROFILESREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETUSERPROFILESREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserProfilesRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetUserProfilesRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGetNativeDataByInviteTokenRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceGetNativeDataByInviteTokenRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceGetNativeDataByInviteTokenRequestParametersTrait for PresenceGetNativeDataByInviteTokenRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceGetNativeDataByInviteTokenRequestParameters {
}

pub static PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNativeDataByInviteTokenRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetNativeDataByInviteTokenRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetNativeDataByInviteTokenRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNativeDataByInviteTokenRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetNativeDataByInviteTokenRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGetUserIdRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceGetUserIdRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceGetUserIdRequestParametersTrait for PresenceGetUserIdRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceGetUserIdRequestParameters {
}

pub static PRESENCEGETUSERIDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserIdRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetUserIdRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETUSERIDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetUserIdRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETUSERIDREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETUSERIDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserIdRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetUserIdRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceViewInviteRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceViewInviteRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceViewInviteRequestParametersTrait for PresenceViewInviteRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceViewInviteRequestParameters {
}

pub static PRESENCEVIEWINVITEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceViewInviteRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceViewInviteRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEVIEWINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceViewInviteRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEVIEWINVITEREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEVIEWINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceViewInviteRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceViewInviteRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceSendInviteRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceSendInviteRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceSendInviteRequestParametersTrait for PresenceSendInviteRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceSendInviteRequestParameters {
}

pub static PRESENCESENDINVITEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendInviteRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceSendInviteRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESENDINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceSendInviteRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESENDINVITEREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCESENDINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendInviteRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceSendInviteRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceHttpPostRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceHttpPostRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceHttpPostRequestParametersTrait for PresenceHttpPostRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceHttpPostRequestParameters {
}

pub static PRESENCEHTTPPOSTREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpPostRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHttpPostRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEHTTPPOSTREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceHttpPostRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEHTTPPOSTREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEHTTPPOSTREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpPostRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceHttpPostRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceHttpGetRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceHttpGetRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceHttpGetRequestParametersTrait for PresenceHttpGetRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceHttpGetRequestParameters {
}

pub static PRESENCEHTTPGETREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpGetRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHttpGetRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEHTTPGETREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceHttpGetRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEHTTPGETREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEHTTPGETREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpGetRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceHttpGetRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGetBlockListRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceGetBlockListRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceGetBlockListRequestParametersTrait for PresenceGetBlockListRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceGetBlockListRequestParameters {
}

pub static PRESENCEGETBLOCKLISTREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetBlockListRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetBlockListRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETBLOCKLISTREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetBlockListRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETBLOCKLISTREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETBLOCKLISTREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetBlockListRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetBlockListRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceRemoveFriendRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceRemoveFriendRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceRemoveFriendRequestParametersTrait for PresenceRemoveFriendRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceRemoveFriendRequestParameters {
}

pub static PRESENCEREMOVEFRIENDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRemoveFriendRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceRemoveFriendRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEREMOVEFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceRemoveFriendRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEREMOVEFRIENDREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEREMOVEFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRemoveFriendRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceRemoveFriendRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceAcceptFriendRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceAcceptFriendRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceAcceptFriendRequestParametersTrait for PresenceAcceptFriendRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceAcceptFriendRequestParameters {
}

pub static PRESENCEACCEPTFRIENDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptFriendRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAcceptFriendRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACCEPTFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceAcceptFriendRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACCEPTFRIENDREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEACCEPTFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptFriendRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceAcceptFriendRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceDownloadBlobRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceDownloadBlobRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceDownloadBlobRequestParametersTrait for PresenceDownloadBlobRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceDownloadBlobRequestParameters {
}

pub static PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadBlobRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDownloadBlobRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDownloadBlobRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadBlobRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceDownloadBlobRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGetAuthCodeRequestParameters {
    pub _glacier_base: PresenceRequestParameters,
}

pub trait PresenceGetAuthCodeRequestParametersTrait: PresenceRequestParametersTrait {
}

impl PresenceGetAuthCodeRequestParametersTrait for PresenceGetAuthCodeRequestParameters {
}

impl PresenceRequestParametersTrait for PresenceGetAuthCodeRequestParameters {
}

pub static PRESENCEGETAUTHCODEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetAuthCodeRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetAuthCodeRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETAUTHCODEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetAuthCodeRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETAUTHCODEREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETAUTHCODEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetAuthCodeRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetAuthCodeRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OriginPresenceBackend {
    pub _glacier_base: PresenceBackend,
}

pub trait OriginPresenceBackendTrait: PresenceBackendTrait {
}

impl OriginPresenceBackendTrait for OriginPresenceBackend {
}

impl PresenceBackendTrait for OriginPresenceBackend {
}

pub static ORIGINPRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginPresenceBackend as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ORIGINPRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OriginPresenceBackend {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINPRESENCEBACKEND_TYPE_INFO
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


pub static ORIGINPRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OriginPresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlazeStateNotificationEvent {
    pub _glacier_base: PresenceEvent,
}

pub trait BlazeStateNotificationEventTrait: PresenceEventTrait {
}

impl BlazeStateNotificationEventTrait for BlazeStateNotificationEvent {
}

impl PresenceEventTrait for BlazeStateNotificationEvent {
}

pub static BLAZESTATENOTIFICATIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeStateNotificationEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazeStateNotificationEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLAZESTATENOTIFICATIONEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlazeStateNotificationEvent {
    fn type_info(&self) -> &'static TypeInfo {
        BLAZESTATENOTIFICATIONEVENT_TYPE_INFO
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


pub static BLAZESTATENOTIFICATIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeStateNotificationEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazeStateNotificationEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlazeGameStrategy {
    pub _glacier_base: PresenceStrategy,
}

pub trait BlazeGameStrategyTrait: PresenceStrategyTrait {
}

impl BlazeGameStrategyTrait for BlazeGameStrategy {
}

impl PresenceStrategyTrait for BlazeGameStrategy {
}

pub static BLAZEGAMESTRATEGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeGameStrategy",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESTRATEGY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazeGameStrategy as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLAZEGAMESTRATEGY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlazeGameStrategy {
    fn type_info(&self) -> &'static TypeInfo {
        BLAZEGAMESTRATEGY_TYPE_INFO
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


pub static BLAZEGAMESTRATEGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeGameStrategy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazeGameStrategy"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientUserProfileService {
    pub _glacier_base: PresenceService,
}

pub trait ClientUserProfileServiceTrait: PresenceServiceTrait {
}

impl ClientUserProfileServiceTrait for ClientUserProfileService {
}

impl PresenceServiceTrait for ClientUserProfileService {
}

pub static CLIENTUSERPROFILESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserProfileService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUserProfileService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUSERPROFILESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUserProfileService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTUSERPROFILESERVICE_TYPE_INFO
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


pub static CLIENTUSERPROFILESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserProfileService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientUserProfileService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientUserIdService {
    pub _glacier_base: PresenceService,
}

pub trait ClientUserIdServiceTrait: PresenceServiceTrait {
}

impl ClientUserIdServiceTrait for ClientUserIdService {
}

impl PresenceServiceTrait for ClientUserIdService {
}

pub static CLIENTUSERIDSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserIdService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUserIdService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUSERIDSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUserIdService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTUSERIDSERVICE_TYPE_INFO
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


pub static CLIENTUSERIDSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserIdService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientUserIdService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientProfanityFilterService {
    pub _glacier_base: PresenceService,
}

pub trait ClientProfanityFilterServiceTrait: PresenceServiceTrait {
}

impl ClientProfanityFilterServiceTrait for ClientProfanityFilterService {
}

impl PresenceServiceTrait for ClientProfanityFilterService {
}

pub static CLIENTPROFANITYFILTERSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProfanityFilterService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProfanityFilterService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROFANITYFILTERSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProfanityFilterService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPROFANITYFILTERSERVICE_TYPE_INFO
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


pub static CLIENTPROFANITYFILTERSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProfanityFilterService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientProfanityFilterService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPrivilegeService {
    pub _glacier_base: PresenceService,
}

pub trait ClientPrivilegeServiceTrait: PresenceServiceTrait {
}

impl ClientPrivilegeServiceTrait for ClientPrivilegeService {
}

impl PresenceServiceTrait for ClientPrivilegeService {
}

pub static CLIENTPRIVILEGESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPrivilegeService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPrivilegeService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPRIVILEGESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPrivilegeService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPRIVILEGESERVICE_TYPE_INFO
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


pub static CLIENTPRIVILEGESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPrivilegeService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientPrivilegeService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientInviteService {
    pub _glacier_base: PresenceService,
}

pub trait ClientInviteServiceTrait: PresenceServiceTrait {
}

impl ClientInviteServiceTrait for ClientInviteService {
}

impl PresenceServiceTrait for ClientInviteService {
}

pub static CLIENTINVITESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientInviteService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientInviteService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTINVITESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientInviteService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTINVITESERVICE_TYPE_INFO
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


pub static CLIENTINVITESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientInviteService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientInviteService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFriendsService {
    pub _glacier_base: PresenceService,
}

pub trait ClientFriendsServiceTrait: PresenceServiceTrait {
}

impl ClientFriendsServiceTrait for ClientFriendsService {
}

impl PresenceServiceTrait for ClientFriendsService {
}

pub static CLIENTFRIENDSSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFriendsService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFriendsService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFRIENDSSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFriendsService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFRIENDSSERVICE_TYPE_INFO
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


pub static CLIENTFRIENDSSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFriendsService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientFriendsService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientConnectionService {
    pub _glacier_base: PresenceService,
}

pub trait ClientConnectionServiceTrait: PresenceServiceTrait {
}

impl ClientConnectionServiceTrait for ClientConnectionService {
}

impl PresenceServiceTrait for ClientConnectionService {
}

pub static CLIENTCONNECTIONSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnectionService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONNECTIONSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientConnectionService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCONNECTIONSERVICE_TYPE_INFO
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


pub static CLIENTCONNECTIONSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientConnectionService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceService {
}

pub trait PresenceServiceTrait: TypeObject {
}

impl PresenceServiceTrait for PresenceService {
}

pub static PRESENCESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceService {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESERVICE_TYPE_INFO
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


pub static PRESENCESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceRequestParameters {
}

pub trait PresenceRequestParametersTrait: TypeObject {
}

impl PresenceRequestParametersTrait for PresenceRequestParameters {
}

pub static PRESENCEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceEvent {
}

pub trait PresenceEventTrait: TypeObject {
}

impl PresenceEventTrait for PresenceEvent {
}

pub static PRESENCEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceEvent {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEEVENT_TYPE_INFO
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


pub static PRESENCEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OnlineInternalCurrentUserChangedMessage {
}

pub trait OnlineInternalCurrentUserChangedMessageTrait: TypeObject {
}

impl OnlineInternalCurrentUserChangedMessageTrait for OnlineInternalCurrentUserChangedMessage {
}

pub static ONLINEINTERNALCURRENTUSERCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineInternalCurrentUserChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Online",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineInternalCurrentUserChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OnlineInternalCurrentUserChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEINTERNALCURRENTUSERCHANGEDMESSAGE_TYPE_INFO
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
pub struct OnlineInternalFriendsMessageBase {
}

pub trait OnlineInternalFriendsMessageBaseTrait: TypeObject {
}

impl OnlineInternalFriendsMessageBaseTrait for OnlineInternalFriendsMessageBase {
}

pub static ONLINEINTERNALFRIENDSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineInternalFriendsMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Online",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineInternalFriendsMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OnlineInternalFriendsMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEINTERNALFRIENDSMESSAGEBASE_TYPE_INFO
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

