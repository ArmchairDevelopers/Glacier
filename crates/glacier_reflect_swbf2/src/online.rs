use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2304481226,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(ServerGameBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerGameBackend as Default>::default())),
            create_boxed: || Box::new(<ServerGameBackend as Default>::default()),
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
    name_hash: 2167989118,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ServerGameBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceStrategy {
}

pub trait PresenceStrategyTrait: TypeObject {
}

impl PresenceStrategyTrait for PresenceStrategy {
}

pub static PRESENCESTRATEGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStrategy",
    name_hash: 1596046375,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStrategy as Default>::default())),
            create_boxed: || Box::new(<PresenceStrategy as Default>::default()),
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
    name_hash: 1074539923,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceStrategy"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBackend {
}

pub trait PresenceBackendTrait: TypeObject {
}

impl PresenceBackendTrait for PresenceBackend {
}

pub static PRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackend",
    name_hash: 3123678936,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBackend as Default>::default())),
            create_boxed: || Box::new(<PresenceBackend as Default>::default()),
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
    name_hash: 1782868332,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2996515330,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(BlockListEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlockListEvent as Default>::default())),
            create_boxed: || Box::new(<BlockListEvent as Default>::default()),
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
    name_hash: 3640889654,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlockListEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2904438249,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(OnModeChangedEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnModeChangedEvent as Default>::default())),
            create_boxed: || Box::new(<OnModeChangedEvent as Default>::default()),
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
    name_hash: 2915331293,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnModeChangedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3412785109,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(OnConnectFailedEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnConnectFailedEvent as Default>::default())),
            create_boxed: || Box::new(<OnConnectFailedEvent as Default>::default()),
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
    name_hash: 1739438049,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectFailedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2991796873,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(OnDisconnectedEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnDisconnectedEvent as Default>::default())),
            create_boxed: || Box::new(<OnDisconnectedEvent as Default>::default()),
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
    name_hash: 2759321661,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnDisconnectedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1618635415,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(OnConnectedEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnConnectedEvent as Default>::default())),
            create_boxed: || Box::new(<OnConnectedEvent as Default>::default()),
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
    name_hash: 431272739,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 35437174,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(OnConnectingEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnConnectingEvent as Default>::default())),
            create_boxed: || Box::new(<OnConnectingEvent as Default>::default()),
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
    name_hash: 784867138,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectingEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 387774674,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(OnlineStatusEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineStatusEvent as Default>::default())),
            create_boxed: || Box::new(<OnlineStatusEvent as Default>::default()),
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
    name_hash: 4191318118,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnlineStatusEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 248677687,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(FirstPartyNetworkStatusEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FirstPartyNetworkStatusEvent as Default>::default())),
            create_boxed: || Box::new(<FirstPartyNetworkStatusEvent as Default>::default()),
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
    name_hash: 1023573635,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FirstPartyNetworkStatusEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 408025006,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(FriendUpdatedEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FriendUpdatedEvent as Default>::default())),
            create_boxed: || Box::new(<FriendUpdatedEvent as Default>::default()),
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
    name_hash: 3020103706,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FriendUpdatedEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4198124556,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(MultiplayerPrivilegeEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MultiplayerPrivilegeEvent as Default>::default())),
            create_boxed: || Box::new(<MultiplayerPrivilegeEvent as Default>::default()),
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
    name_hash: 1344339000,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("MultiplayerPrivilegeEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3067362700,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(FirstPartyUserEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FirstPartyUserEvent as Default>::default())),
            create_boxed: || Box::new(<FirstPartyUserEvent as Default>::default()),
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
    name_hash: 1808403896,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FirstPartyUserEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3314405340,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(BackendStateChangeEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BackendStateChangeEvent as Default>::default())),
            create_boxed: || Box::new(<BackendStateChangeEvent as Default>::default()),
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
    name_hash: 2932807784,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BackendStateChangeEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3024062022,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(CancelHttpRequestEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CancelHttpRequestEvent as Default>::default())),
            create_boxed: || Box::new(<CancelHttpRequestEvent as Default>::default()),
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
    name_hash: 179700722,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("CancelHttpRequestEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 319587710,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(DirtySockPresenceBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DirtySockPresenceBackend as Default>::default())),
            create_boxed: || Box::new(<DirtySockPresenceBackend as Default>::default()),
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
    name_hash: 2812511818,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("DirtySockPresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2225053927,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESTRATEGY_TYPE_INFO),
        super_class_offset: offset_of!(GameStateStrategy, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameStateStrategy as Default>::default())),
            create_boxed: || Box::new(<GameStateStrategy as Default>::default()),
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
    name_hash: 1909891795,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("GameStateStrategy"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1516997928,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(BlazePresenceBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazePresenceBackend as Default>::default())),
            create_boxed: || Box::new(<BlazePresenceBackend as Default>::default()),
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
    name_hash: 3420368540,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazePresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2242533250,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientBlobService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlobService as Default>::default())),
            create_boxed: || Box::new(<ClientBlobService as Default>::default()),
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
    name_hash: 590927542,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientBlobService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2435740545,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientAuthenticationService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAuthenticationService as Default>::default())),
            create_boxed: || Box::new(<ClientAuthenticationService as Default>::default()),
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
    name_hash: 1115721013,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientAuthenticationService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3728121783,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceFilterProfanityRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceFilterProfanityRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceFilterProfanityRequestParameters as Default>::default()),
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
    name_hash: 4004542723,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceFilterProfanityRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3357245642,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceAcquireMultiplayerPrivilegeRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAcquireMultiplayerPrivilegeRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceAcquireMultiplayerPrivilegeRequestParameters as Default>::default()),
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
    name_hash: 1417907326,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceAcquireMultiplayerPrivilegeRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2662738685,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceCheckPrivilegesRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceCheckPrivilegesRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceCheckPrivilegesRequestParameters as Default>::default()),
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
    name_hash: 2710738889,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceCheckPrivilegesRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4088890824,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetUserProfilesRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetUserProfilesRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetUserProfilesRequestParameters as Default>::default()),
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
    name_hash: 236378236,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetUserProfilesRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 789212505,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetNativeDataByInviteTokenRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetNativeDataByInviteTokenRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetNativeDataByInviteTokenRequestParameters as Default>::default()),
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
    name_hash: 3266247533,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetNativeDataByInviteTokenRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3759175549,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetUserIdRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetUserIdRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetUserIdRequestParameters as Default>::default()),
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
    name_hash: 3314828361,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetUserIdRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3754360307,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceViewInviteRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceViewInviteRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceViewInviteRequestParameters as Default>::default()),
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
    name_hash: 3835877831,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceViewInviteRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2166999554,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceSendInviteRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceSendInviteRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceSendInviteRequestParameters as Default>::default()),
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
    name_hash: 1847863606,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceSendInviteRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2911157975,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceHttpPostRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHttpPostRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceHttpPostRequestParameters as Default>::default()),
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
    name_hash: 3614242787,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceHttpPostRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1240086521,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceHttpGetRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceHttpGetRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceHttpGetRequestParameters as Default>::default()),
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
    name_hash: 2006759117,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceHttpGetRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 858253450,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetBlockListRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetBlockListRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetBlockListRequestParameters as Default>::default()),
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
    name_hash: 962495422,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetBlockListRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 90710243,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceRemoveFriendRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceRemoveFriendRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceRemoveFriendRequestParameters as Default>::default()),
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
    name_hash: 1707046103,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceRemoveFriendRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 86997893,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceAcceptFriendRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAcceptFriendRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceAcceptFriendRequestParameters as Default>::default()),
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
    name_hash: 3865527089,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceAcceptFriendRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 533185056,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceDownloadBlobRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDownloadBlobRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceDownloadBlobRequestParameters as Default>::default()),
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
    name_hash: 2991540116,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceDownloadBlobRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2954278244,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetAuthCodeRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetAuthCodeRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetAuthCodeRequestParameters as Default>::default()),
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
    name_hash: 2748537168,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetAuthCodeRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3264877388,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(OriginPresenceBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginPresenceBackend as Default>::default())),
            create_boxed: || Box::new(<OriginPresenceBackend as Default>::default()),
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
    name_hash: 1884107768,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OriginPresenceBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 32317507,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(BlazeStateNotificationEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazeStateNotificationEvent as Default>::default())),
            create_boxed: || Box::new(<BlazeStateNotificationEvent as Default>::default()),
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
    name_hash: 925238135,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazeStateNotificationEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1563503072,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESTRATEGY_TYPE_INFO),
        super_class_offset: offset_of!(BlazeGameStrategy, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazeGameStrategy as Default>::default())),
            create_boxed: || Box::new(<BlazeGameStrategy as Default>::default()),
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
    name_hash: 4224942036,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazeGameStrategy"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1562790843,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientUserProfileService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUserProfileService as Default>::default())),
            create_boxed: || Box::new(<ClientUserProfileService as Default>::default()),
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
    name_hash: 3874542351,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientUserProfileService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1629660573,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientUserIdService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUserIdService as Default>::default())),
            create_boxed: || Box::new(<ClientUserIdService as Default>::default()),
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
    name_hash: 1285820713,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientUserIdService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3039698209,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientProfanityFilterService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientProfanityFilterService as Default>::default())),
            create_boxed: || Box::new(<ClientProfanityFilterService as Default>::default()),
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
    name_hash: 77932437,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientProfanityFilterService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1670872574,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientPrivilegeService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPrivilegeService as Default>::default())),
            create_boxed: || Box::new(<ClientPrivilegeService as Default>::default()),
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
    name_hash: 2678513354,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientPrivilegeService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3532734216,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientInviteService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientInviteService as Default>::default())),
            create_boxed: || Box::new(<ClientInviteService as Default>::default()),
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
    name_hash: 4150427964,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientInviteService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 256913824,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientFriendsService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFriendsService as Default>::default())),
            create_boxed: || Box::new(<ClientFriendsService as Default>::default()),
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
    name_hash: 625825556,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientFriendsService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3473547447,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientConnectionService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientConnectionService as Default>::default())),
            create_boxed: || Box::new(<ClientConnectionService as Default>::default()),
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
    name_hash: 174111235,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientConnectionService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceService {
}

pub trait PresenceServiceTrait: TypeObject {
}

impl PresenceServiceTrait for PresenceService {
}

pub static PRESENCESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceService",
    name_hash: 2360007873,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceService as Default>::default())),
            create_boxed: || Box::new(<PresenceService as Default>::default()),
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
    name_hash: 3283281653,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceRequestParameters {
}

pub trait PresenceRequestParametersTrait: TypeObject {
}

impl PresenceRequestParametersTrait for PresenceRequestParameters {
}

pub static PRESENCEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRequestParameters",
    name_hash: 67586487,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceRequestParameters as Default>::default()),
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
    name_hash: 1557701379,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceEvent {
}

pub trait PresenceEventTrait: TypeObject {
}

impl PresenceEventTrait for PresenceEvent {
}

pub static PRESENCEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEvent",
    name_hash: 1801165456,
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceEvent as Default>::default())),
            create_boxed: || Box::new(<PresenceEvent as Default>::default()),
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
    name_hash: 2104944804,
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineInternalCurrentUserChangedMessage {
}

pub trait OnlineInternalCurrentUserChangedMessageTrait: TypeObject {
}

impl OnlineInternalCurrentUserChangedMessageTrait for OnlineInternalCurrentUserChangedMessage {
}

pub static ONLINEINTERNALCURRENTUSERCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineInternalCurrentUserChangedMessage",
    name_hash: 1233159772,
    flags: MemberInfoFlags::new(36937),
    module: "Online",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineInternalCurrentUserChangedMessage as Default>::default())),
            create_boxed: || Box::new(<OnlineInternalCurrentUserChangedMessage as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineInternalFriendsMessageBase {
}

pub trait OnlineInternalFriendsMessageBaseTrait: TypeObject {
}

impl OnlineInternalFriendsMessageBaseTrait for OnlineInternalFriendsMessageBase {
}

pub static ONLINEINTERNALFRIENDSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineInternalFriendsMessageBase",
    name_hash: 766822034,
    flags: MemberInfoFlags::new(36937),
    module: "Online",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineInternalFriendsMessageBase as Default>::default())),
            create_boxed: || Box::new(<OnlineInternalFriendsMessageBase as Default>::default()),
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

