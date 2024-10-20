use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerGameBackend {
}

pub const SERVERGAMEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERGAMEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerGameBackend {
    fn type_info() -> &'static TypeInfo {
        SERVERGAMEBACKEND_TYPE_INFO
    }
}


pub const SERVERGAMEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerGameBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ServerGameBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceStrategy {
}

pub const PRESENCESTRATEGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStrategy",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESTRATEGY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceStrategy {
    fn type_info() -> &'static TypeInfo {
        PRESENCESTRATEGY_TYPE_INFO
    }
}


pub const PRESENCESTRATEGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStrategy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceStrategy-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBackend {
}

pub const PRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceBackend {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBACKEND_TYPE_INFO
    }
}


pub const PRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlockListEvent {
}

pub const BLOCKLISTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockListEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLOCKLISTEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlockListEvent {
    fn type_info() -> &'static TypeInfo {
        BLOCKLISTEVENT_TYPE_INFO
    }
}


pub const BLOCKLISTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockListEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlockListEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnModeChangedEvent {
}

pub const ONMODECHANGEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnModeChangedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ONMODECHANGEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnModeChangedEvent {
    fn type_info() -> &'static TypeInfo {
        ONMODECHANGEDEVENT_TYPE_INFO
    }
}


pub const ONMODECHANGEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnModeChangedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnModeChangedEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnConnectFailedEvent {
}

pub const ONCONNECTFAILEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectFailedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ONCONNECTFAILEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnConnectFailedEvent {
    fn type_info() -> &'static TypeInfo {
        ONCONNECTFAILEDEVENT_TYPE_INFO
    }
}


pub const ONCONNECTFAILEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectFailedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectFailedEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnDisconnectedEvent {
}

pub const ONDISCONNECTEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnDisconnectedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ONDISCONNECTEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnDisconnectedEvent {
    fn type_info() -> &'static TypeInfo {
        ONDISCONNECTEDEVENT_TYPE_INFO
    }
}


pub const ONDISCONNECTEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnDisconnectedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnDisconnectedEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnConnectedEvent {
}

pub const ONCONNECTEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ONCONNECTEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnConnectedEvent {
    fn type_info() -> &'static TypeInfo {
        ONCONNECTEDEVENT_TYPE_INFO
    }
}


pub const ONCONNECTEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectedEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnConnectingEvent {
}

pub const ONCONNECTINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectingEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ONCONNECTINGEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnConnectingEvent {
    fn type_info() -> &'static TypeInfo {
        ONCONNECTINGEVENT_TYPE_INFO
    }
}


pub const ONCONNECTINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnConnectingEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnConnectingEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineStatusEvent {
}

pub const ONLINESTATUSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineStatusEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ONLINESTATUSEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OnlineStatusEvent {
    fn type_info() -> &'static TypeInfo {
        ONLINESTATUSEVENT_TYPE_INFO
    }
}


pub const ONLINESTATUSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineStatusEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OnlineStatusEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FirstPartyNetworkStatusEvent {
}

pub const FIRSTPARTYNETWORKSTATUSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyNetworkStatusEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FIRSTPARTYNETWORKSTATUSEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FirstPartyNetworkStatusEvent {
    fn type_info() -> &'static TypeInfo {
        FIRSTPARTYNETWORKSTATUSEVENT_TYPE_INFO
    }
}


pub const FIRSTPARTYNETWORKSTATUSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyNetworkStatusEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FirstPartyNetworkStatusEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FriendUpdatedEvent {
}

pub const FRIENDUPDATEDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FriendUpdatedEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FRIENDUPDATEDEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FriendUpdatedEvent {
    fn type_info() -> &'static TypeInfo {
        FRIENDUPDATEDEVENT_TYPE_INFO
    }
}


pub const FRIENDUPDATEDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FriendUpdatedEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FriendUpdatedEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultiplayerPrivilegeEvent {
}

pub const MULTIPLAYERPRIVILEGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerPrivilegeEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MULTIPLAYERPRIVILEGEEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MultiplayerPrivilegeEvent {
    fn type_info() -> &'static TypeInfo {
        MULTIPLAYERPRIVILEGEEVENT_TYPE_INFO
    }
}


pub const MULTIPLAYERPRIVILEGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerPrivilegeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("MultiplayerPrivilegeEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FirstPartyUserEvent {
}

pub const FIRSTPARTYUSEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyUserEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FIRSTPARTYUSEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FirstPartyUserEvent {
    fn type_info() -> &'static TypeInfo {
        FIRSTPARTYUSEREVENT_TYPE_INFO
    }
}


pub const FIRSTPARTYUSEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FirstPartyUserEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("FirstPartyUserEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BackendStateChangeEvent {
}

pub const BACKENDSTATECHANGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BackendStateChangeEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BACKENDSTATECHANGEEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BackendStateChangeEvent {
    fn type_info() -> &'static TypeInfo {
        BACKENDSTATECHANGEEVENT_TYPE_INFO
    }
}


pub const BACKENDSTATECHANGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BackendStateChangeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BackendStateChangeEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CancelHttpRequestEvent {
}

pub const CANCELHTTPREQUESTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CancelHttpRequestEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CANCELHTTPREQUESTEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CancelHttpRequestEvent {
    fn type_info() -> &'static TypeInfo {
        CANCELHTTPREQUESTEVENT_TYPE_INFO
    }
}


pub const CANCELHTTPREQUESTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CancelHttpRequestEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("CancelHttpRequestEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DirtySockPresenceBackend {
}

pub const DIRTYSOCKPRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DIRTYSOCKPRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DirtySockPresenceBackend {
    fn type_info() -> &'static TypeInfo {
        DIRTYSOCKPRESENCEBACKEND_TYPE_INFO
    }
}


pub const DIRTYSOCKPRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("DirtySockPresenceBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameStateStrategy {
}

pub const GAMESTATESTRATEGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateStrategy",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESTRATEGY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMESTATESTRATEGY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameStateStrategy {
    fn type_info() -> &'static TypeInfo {
        GAMESTATESTRATEGY_TYPE_INFO
    }
}


pub const GAMESTATESTRATEGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateStrategy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("GameStateStrategy-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlazePresenceBackend {
}

pub const BLAZEPRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLAZEPRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlazePresenceBackend {
    fn type_info() -> &'static TypeInfo {
        BLAZEPRESENCEBACKEND_TYPE_INFO
    }
}


pub const BLAZEPRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazePresenceBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlobService {
}

pub const CLIENTBLOBSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlobService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLOBSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlobService {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLOBSERVICE_TYPE_INFO
    }
}


pub const CLIENTBLOBSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlobService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientBlobService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAuthenticationService {
}

pub const CLIENTAUTHENTICATIONSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAuthenticationService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTAUTHENTICATIONSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAuthenticationService {
    fn type_info() -> &'static TypeInfo {
        CLIENTAUTHENTICATIONSERVICE_TYPE_INFO
    }
}


pub const CLIENTAUTHENTICATIONSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAuthenticationService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientAuthenticationService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceFilterProfanityRequestParameters {
}

pub const PRESENCEFILTERPROFANITYREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFilterProfanityRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEFILTERPROFANITYREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceFilterProfanityRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEFILTERPROFANITYREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEFILTERPROFANITYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFilterProfanityRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceFilterProfanityRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAcquireMultiplayerPrivilegeRequestParameters {
}

pub const PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcquireMultiplayerPrivilegeRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceAcquireMultiplayerPrivilegeRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEACQUIREMULTIPLAYERPRIVILEGEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcquireMultiplayerPrivilegeRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceAcquireMultiplayerPrivilegeRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceCheckPrivilegesRequestParameters {
}

pub const PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCheckPrivilegesRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceCheckPrivilegesRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCECHECKPRIVILEGESREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCheckPrivilegesRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceCheckPrivilegesRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetUserProfilesRequestParameters {
}

pub const PRESENCEGETUSERPROFILESREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserProfilesRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETUSERPROFILESREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetUserProfilesRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETUSERPROFILESREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETUSERPROFILESREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserProfilesRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetUserProfilesRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetNativeDataByInviteTokenRequestParameters {
}

pub const PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNativeDataByInviteTokenRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetNativeDataByInviteTokenRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETNATIVEDATABYINVITETOKENREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNativeDataByInviteTokenRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetNativeDataByInviteTokenRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetUserIdRequestParameters {
}

pub const PRESENCEGETUSERIDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserIdRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETUSERIDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetUserIdRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETUSERIDREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETUSERIDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetUserIdRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetUserIdRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceViewInviteRequestParameters {
}

pub const PRESENCEVIEWINVITEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceViewInviteRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEVIEWINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceViewInviteRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEVIEWINVITEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEVIEWINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceViewInviteRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceViewInviteRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceSendInviteRequestParameters {
}

pub const PRESENCESENDINVITEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendInviteRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESENDINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceSendInviteRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCESENDINVITEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCESENDINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendInviteRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceSendInviteRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceHttpPostRequestParameters {
}

pub const PRESENCEHTTPPOSTREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpPostRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEHTTPPOSTREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceHttpPostRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEHTTPPOSTREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEHTTPPOSTREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpPostRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceHttpPostRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceHttpGetRequestParameters {
}

pub const PRESENCEHTTPGETREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpGetRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEHTTPGETREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceHttpGetRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEHTTPGETREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEHTTPGETREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceHttpGetRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceHttpGetRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetBlockListRequestParameters {
}

pub const PRESENCEGETBLOCKLISTREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetBlockListRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETBLOCKLISTREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetBlockListRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETBLOCKLISTREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETBLOCKLISTREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetBlockListRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetBlockListRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceRemoveFriendRequestParameters {
}

pub const PRESENCEREMOVEFRIENDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRemoveFriendRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEREMOVEFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceRemoveFriendRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEREMOVEFRIENDREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEREMOVEFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRemoveFriendRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceRemoveFriendRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAcceptFriendRequestParameters {
}

pub const PRESENCEACCEPTFRIENDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptFriendRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACCEPTFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceAcceptFriendRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACCEPTFRIENDREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEACCEPTFRIENDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptFriendRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceAcceptFriendRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceDownloadBlobRequestParameters {
}

pub const PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadBlobRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDownloadBlobRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEDOWNLOADBLOBREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadBlobRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceDownloadBlobRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetAuthCodeRequestParameters {
}

pub const PRESENCEGETAUTHCODEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetAuthCodeRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETAUTHCODEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetAuthCodeRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETAUTHCODEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETAUTHCODEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetAuthCodeRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceGetAuthCodeRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginPresenceBackend {
}

pub const ORIGINPRESENCEBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackend",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ORIGINPRESENCEBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OriginPresenceBackend {
    fn type_info() -> &'static TypeInfo {
        ORIGINPRESENCEBACKEND_TYPE_INFO
    }
}


pub const ORIGINPRESENCEBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("OriginPresenceBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlazeStateNotificationEvent {
}

pub const BLAZESTATENOTIFICATIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeStateNotificationEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLAZESTATENOTIFICATIONEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlazeStateNotificationEvent {
    fn type_info() -> &'static TypeInfo {
        BLAZESTATENOTIFICATIONEVENT_TYPE_INFO
    }
}


pub const BLAZESTATENOTIFICATIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeStateNotificationEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazeStateNotificationEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlazeGameStrategy {
}

pub const BLAZEGAMESTRATEGY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeGameStrategy",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESTRATEGY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLAZEGAMESTRATEGY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlazeGameStrategy {
    fn type_info() -> &'static TypeInfo {
        BLAZEGAMESTRATEGY_TYPE_INFO
    }
}


pub const BLAZEGAMESTRATEGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeGameStrategy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("BlazeGameStrategy-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientUserProfileService {
}

pub const CLIENTUSERPROFILESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserProfileService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUSERPROFILESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUserProfileService {
    fn type_info() -> &'static TypeInfo {
        CLIENTUSERPROFILESERVICE_TYPE_INFO
    }
}


pub const CLIENTUSERPROFILESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserProfileService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientUserProfileService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientUserIdService {
}

pub const CLIENTUSERIDSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserIdService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUSERIDSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUserIdService {
    fn type_info() -> &'static TypeInfo {
        CLIENTUSERIDSERVICE_TYPE_INFO
    }
}


pub const CLIENTUSERIDSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUserIdService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientUserIdService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientProfanityFilterService {
}

pub const CLIENTPROFANITYFILTERSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProfanityFilterService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPROFANITYFILTERSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientProfanityFilterService {
    fn type_info() -> &'static TypeInfo {
        CLIENTPROFANITYFILTERSERVICE_TYPE_INFO
    }
}


pub const CLIENTPROFANITYFILTERSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientProfanityFilterService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientProfanityFilterService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPrivilegeService {
}

pub const CLIENTPRIVILEGESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPrivilegeService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPRIVILEGESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPrivilegeService {
    fn type_info() -> &'static TypeInfo {
        CLIENTPRIVILEGESERVICE_TYPE_INFO
    }
}


pub const CLIENTPRIVILEGESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPrivilegeService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientPrivilegeService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientInviteService {
}

pub const CLIENTINVITESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientInviteService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTINVITESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientInviteService {
    fn type_info() -> &'static TypeInfo {
        CLIENTINVITESERVICE_TYPE_INFO
    }
}


pub const CLIENTINVITESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientInviteService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientInviteService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFriendsService {
}

pub const CLIENTFRIENDSSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFriendsService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFRIENDSSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFriendsService {
    fn type_info() -> &'static TypeInfo {
        CLIENTFRIENDSSERVICE_TYPE_INFO
    }
}


pub const CLIENTFRIENDSSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFriendsService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientFriendsService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientConnectionService {
}

pub const CLIENTCONNECTIONSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCONNECTIONSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientConnectionService {
    fn type_info() -> &'static TypeInfo {
        CLIENTCONNECTIONSERVICE_TYPE_INFO
    }
}


pub const CLIENTCONNECTIONSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientConnectionService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("ClientConnectionService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceService {
}

pub const PRESENCESERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceService",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceService {
    fn type_info() -> &'static TypeInfo {
        PRESENCESERVICE_TYPE_INFO
    }
}


pub const PRESENCESERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceRequestParameters {
}

pub const PRESENCEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceEvent {
}

pub const PRESENCEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEvent",
    flags: MemberInfoFlags::new(101),
    module: "Online",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceEvent {
    fn type_info() -> &'static TypeInfo {
        PRESENCEEVENT_TYPE_INFO
    }
}


pub const PRESENCEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Online",
    data: TypeInfoData::Array("PresenceEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineInternalCurrentUserChangedMessage {
}

pub const ONLINEINTERNALCURRENTUSERCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineInternalCurrentUserChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Online",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OnlineInternalCurrentUserChangedMessage {
    fn type_info() -> &'static TypeInfo {
        ONLINEINTERNALCURRENTUSERCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineInternalFriendsMessageBase {
}

pub const ONLINEINTERNALFRIENDSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineInternalFriendsMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Online",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OnlineInternalFriendsMessageBase {
    fn type_info() -> &'static TypeInfo {
        ONLINEINTERNALFRIENDSMESSAGEBASE_TYPE_INFO
    }
}

