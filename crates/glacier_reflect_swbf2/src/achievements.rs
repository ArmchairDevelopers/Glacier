use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_achievements_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCEACHIEVEMENTSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEACHIEVEMENTSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEACHIEVEMENTREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEACHIEVEMENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(XB1ACHIEVEMENTSBACKENDDATA_TYPE_INFO);
    registry.register_type(XB1ACHIEVEMENTSBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PS4TROPHYBACKENDDATA_TYPE_INFO);
    registry.register_type(PS4TROPHYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTACHIEVEMENTSERVICE_TYPE_INFO);
    registry.register_type(CLIENTACHIEVEMENTSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAchievementServiceData {
}

pub const PRESENCEACHIEVEMENTSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACHIEVEMENTSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceAchievementServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACHIEVEMENTSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEACHIEVEMENTSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceAchievementServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAchievementRequestMessageBase {
}

pub const PRESENCEACHIEVEMENTREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Achievements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAchievementRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACHIEVEMENTREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAchievementMessageBase {
}

pub const PRESENCEACHIEVEMENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Achievements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAchievementMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACHIEVEMENTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Xb1AchievementsBackendData {
}

pub const XB1ACHIEVEMENTSBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Xb1AchievementsBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(XB1ACHIEVEMENTSBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Xb1AchievementsBackendData {
    fn type_info() -> &'static TypeInfo {
        XB1ACHIEVEMENTSBACKENDDATA_TYPE_INFO
    }
}


pub const XB1ACHIEVEMENTSBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Xb1AchievementsBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("Xb1AchievementsBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ps4TrophyBackendData {
    pub enable_trophy_support: bool,
}

pub const PS4TROPHYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4TrophyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EnableTrophySupport",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4TrophyBackendData, enable_trophy_support),
            },
        ],
    }),
    array_type: Some(PS4TROPHYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4TrophyBackendData {
    fn type_info() -> &'static TypeInfo {
        PS4TROPHYBACKENDDATA_TYPE_INFO
    }
}


pub const PS4TROPHYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4TrophyBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("Ps4TrophyBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientAchievementService {
}

pub const CLIENTACHIEVEMENTSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAchievementService",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACHIEVEMENTSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAchievementService {
    fn type_info() -> &'static TypeInfo {
        CLIENTACHIEVEMENTSERVICE_TYPE_INFO
    }
}


pub const CLIENTACHIEVEMENTSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAchievementService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("ClientAchievementService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUpdateAchievementsRequestParameters {
}

pub const PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUpdateAchievementsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceUpdateAchievementsRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUpdateAchievementsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceUpdateAchievementsRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUnlockAchievementsRequestParameters {
}

pub const PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUnlockAchievementsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceUnlockAchievementsRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUnlockAchievementsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceUnlockAchievementsRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


