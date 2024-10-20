use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_matchmaking_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCEMATCHMAKINGSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEMATCHMAKINGSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEMATCHMAKERMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEMATCHMAKINGREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(CLIENTMATCHMAKINGSERVICE_TYPE_INFO);
    registry.register_type(CLIENTMATCHMAKINGSERVICE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceMatchmakingServiceData {
    pub dummy_bool: bool,
}

pub const PRESENCEMATCHMAKINGSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakingServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Matchmaking",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "dummyBool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PresenceMatchmakingServiceData, dummy_bool),
            },
        ],
    }),
    array_type: Some(PRESENCEMATCHMAKINGSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceMatchmakingServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEMATCHMAKINGSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEMATCHMAKINGSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakingServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Matchmaking",
    data: TypeInfoData::Array("PresenceMatchmakingServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceMatchmakerMessageBase {
}

pub const PRESENCEMATCHMAKERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakerMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Matchmaking",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceMatchmakerMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEMATCHMAKERMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceMatchmakingRequestMessageBase {
}

pub const PRESENCEMATCHMAKINGREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakingRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Matchmaking",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceMatchmakingRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEMATCHMAKINGREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMatchmakingService {
}

pub const CLIENTMATCHMAKINGSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMatchmakingService",
    flags: MemberInfoFlags::new(101),
    module: "Matchmaking",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMATCHMAKINGSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMatchmakingService {
    fn type_info() -> &'static TypeInfo {
        CLIENTMATCHMAKINGSERVICE_TYPE_INFO
    }
}


pub const CLIENTMATCHMAKINGSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMatchmakingService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Matchmaking",
    data: TypeInfoData::Array("ClientMatchmakingService-Array"),
    array_type: None,
    alignment: 8,
};


