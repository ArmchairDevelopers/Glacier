use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_party_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCEPARTYSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEPARTYSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEDURANGOPARTYBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEDURANGOPARTYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPS4PARTYBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEPS4PARTYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEORIGINPARTYBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEORIGINPARTYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(DURANGOCURRENTACTIVITY_TYPE_INFO);
    registry.register_type(DURANGOCURRENTACTIVITY_ARRAY_TYPE_INFO);
    registry.register_type(ORIGINPARTYTYPE_TYPE_INFO);
    registry.register_type(ORIGINPARTYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPARTYREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPARTYMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCELEAVEPARTYREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCELEAVEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCESENDPARTYINVITESREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCESENDPARTYINVITESREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCECREATEPARTYREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCECREATEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPARTYSERVICE_TYPE_INFO);
    registry.register_type(CLIENTPARTYSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(PARTYEVENT_TYPE_INFO);
    registry.register_type(PARTYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ORIGINPARTYBACKEND_TYPE_INFO);
    registry.register_type(ORIGINPARTYBACKEND_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct PresencePartyServiceData {
    pub user_profile_refresh_timeout: f32,
    pub user_profiles_fetch_enabled: bool,
}

pub const PRESENCEPARTYSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UserProfileRefreshTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PresencePartyServiceData, user_profile_refresh_timeout),
            },
            FieldInfoData {
                name: "UserProfilesFetchEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PresencePartyServiceData, user_profiles_fetch_enabled),
            },
        ],
    }),
    array_type: Some(PRESENCEPARTYSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePartyServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPARTYSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEPARTYSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresencePartyServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceDurangoPartyBackendData {
    pub party_session_template: String,
    pub party_invite_context: String,
    pub auto_create_party: bool,
    pub current_activity: DurangoCurrentActivity,
}

pub const PRESENCEDURANGOPARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDurangoPartyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PartySessionTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, party_session_template),
            },
            FieldInfoData {
                name: "PartyInviteContext",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, party_invite_context),
            },
            FieldInfoData {
                name: "AutoCreateParty",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, auto_create_party),
            },
            FieldInfoData {
                name: "CurrentActivity",
                flags: MemberInfoFlags::new(0),
                field_type: DURANGOCURRENTACTIVITY_TYPE_INFO,
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, current_activity),
            },
        ],
    }),
    array_type: Some(PRESENCEDURANGOPARTYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceDurangoPartyBackendData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEDURANGOPARTYBACKENDDATA_TYPE_INFO
    }
}


pub const PRESENCEDURANGOPARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDurangoPartyBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceDurangoPartyBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePs4PartyBackendData {
}

pub const PRESENCEPS4PARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePs4PartyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPS4PARTYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePs4PartyBackendData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPS4PARTYBACKENDDATA_TYPE_INFO
    }
}


pub const PRESENCEPS4PARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePs4PartyBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresencePs4PartyBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceOriginPartyBackendData {
    pub party_type: OriginPartyType,
}

pub const PRESENCEORIGINPARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginPartyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PartyType",
                flags: MemberInfoFlags::new(0),
                field_type: ORIGINPARTYTYPE_TYPE_INFO,
                rust_offset: offset_of!(PresenceOriginPartyBackendData, party_type),
            },
        ],
    }),
    array_type: Some(PRESENCEORIGINPARTYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceOriginPartyBackendData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEORIGINPARTYBACKENDDATA_TYPE_INFO
    }
}


pub const PRESENCEORIGINPARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginPartyBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceOriginPartyBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DurangoCurrentActivity {
    #[default]
    DurangoCurrentActivity_Game = 0,
    DurangoCurrentActivity_Party = 1,
    DurangoCurrentActivity_Count = 2,
}

pub const DURANGOCURRENTACTIVITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoCurrentActivity",
    flags: MemberInfoFlags::new(49429),
    module: "Party",
    data: TypeInfoData::Enum,
    array_type: Some(DURANGOCURRENTACTIVITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DurangoCurrentActivity {
    fn type_info() -> &'static TypeInfo {
        DURANGOCURRENTACTIVITY_TYPE_INFO
    }
}


pub const DURANGOCURRENTACTIVITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoCurrentActivity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("DurangoCurrentActivity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OriginPartyType {
    #[default]
    OriginPartyType_Public = 0,
    OriginPartyType_Private = 1,
    OriginPartyType_Count = 2,
}

pub const ORIGINPARTYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyType",
    flags: MemberInfoFlags::new(49429),
    module: "Party",
    data: TypeInfoData::Enum,
    array_type: Some(ORIGINPARTYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OriginPartyType {
    fn type_info() -> &'static TypeInfo {
        ORIGINPARTYTYPE_TYPE_INFO
    }
}


pub const ORIGINPARTYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("OriginPartyType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePartyRequestMessageBase {
}

pub const PRESENCEPARTYREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Party",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePartyRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPARTYREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePartyMessageBase {
}

pub const PRESENCEPARTYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Party",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePartyMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPARTYMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceLeavePartyRequestParameters {
}

pub const PRESENCELEAVEPARTYREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeavePartyRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCELEAVEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceLeavePartyRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCELEAVEPARTYREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCELEAVEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeavePartyRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceLeavePartyRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAcceptPartyInviteRequestParameters {
}

pub const PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptPartyInviteRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceAcceptPartyInviteRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptPartyInviteRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceAcceptPartyInviteRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceSendPartyInvitesRequestParameters {
}

pub const PRESENCESENDPARTYINVITESREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendPartyInvitesRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESENDPARTYINVITESREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceSendPartyInvitesRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCESENDPARTYINVITESREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCESENDPARTYINVITESREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendPartyInvitesRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceSendPartyInvitesRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceCreatePartyRequestParameters {
}

pub const PRESENCECREATEPARTYREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreatePartyRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECREATEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceCreatePartyRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCECREATEPARTYREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCECREATEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreatePartyRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceCreatePartyRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPartyService {
}

pub const CLIENTPARTYSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartyService",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPARTYSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPartyService {
    fn type_info() -> &'static TypeInfo {
        CLIENTPARTYSERVICE_TYPE_INFO
    }
}


pub const CLIENTPARTYSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartyService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("ClientPartyService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartyEvent {
}

pub const PARTYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartyEvent",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PARTYEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PartyEvent {
    fn type_info() -> &'static TypeInfo {
        PARTYEVENT_TYPE_INFO
    }
}


pub const PARTYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartyEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PartyEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginPartyBackend {
}

pub const ORIGINPARTYBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyBackend",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ORIGINPARTYBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OriginPartyBackend {
    fn type_info() -> &'static TypeInfo {
        ORIGINPARTYBACKEND_TYPE_INFO
    }
}


pub const ORIGINPARTYBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("OriginPartyBackend-Array"),
    array_type: None,
    alignment: 8,
};


