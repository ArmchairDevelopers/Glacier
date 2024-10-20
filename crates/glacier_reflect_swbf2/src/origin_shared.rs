use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_origin_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(ORIGINSETTINGS_TYPE_INFO);
    registry.register_type(ORIGINSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ORIGINCORENOTAVAILABLEMESSAGE_TYPE_INFO);
    registry.register_type(ORIGINNOTLOADEDMESSAGE_TYPE_INFO);
    registry.register_type(ORIGINONLINEMESSAGE_TYPE_INFO);
    registry.register_type(ORIGINRESPONSEMESSAGEBASE_TYPE_INFO);
    registry.register_type(ORIGINREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(ORIGINJOINABLEMESSAGEBASE_TYPE_INFO);
    registry.register_type(ORIGINERRORMESSAGE_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginSettings {
    pub enabled: bool,
    pub required_for_online: bool,
    pub require_latest_for_online_features: bool,
    pub content_id: String,
    pub title: String,
    pub multiplayer_id: String,
    pub language: String,
    pub log: bool,
    pub allow_production_environment: bool,
    pub achievements_secret: String,
    pub achievements_timeout: i32,
    pub entitlements_timeout: i32,
    pub installer_game: String,
    pub installer_studio: String,
    pub disable_in_live_edit_mode: bool,
    pub auto_restart_origin_s_d_k: bool,
}

pub const ORIGINSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginSettings",
    flags: MemberInfoFlags::new(101),
    module: "OriginShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, enabled),
            },
            FieldInfoData {
                name: "RequiredForOnline",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, required_for_online),
            },
            FieldInfoData {
                name: "RequireLatestForOnlineFeatures",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, require_latest_for_online_features),
            },
            FieldInfoData {
                name: "ContentId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, content_id),
            },
            FieldInfoData {
                name: "Title",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, title),
            },
            FieldInfoData {
                name: "MultiplayerId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, multiplayer_id),
            },
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, language),
            },
            FieldInfoData {
                name: "Log",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, log),
            },
            FieldInfoData {
                name: "AllowProductionEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, allow_production_environment),
            },
            FieldInfoData {
                name: "AchievementsSecret",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, achievements_secret),
            },
            FieldInfoData {
                name: "AchievementsTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, achievements_timeout),
            },
            FieldInfoData {
                name: "EntitlementsTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, entitlements_timeout),
            },
            FieldInfoData {
                name: "InstallerGame",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, installer_game),
            },
            FieldInfoData {
                name: "InstallerStudio",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, installer_studio),
            },
            FieldInfoData {
                name: "DisableInLiveEditMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, disable_in_live_edit_mode),
            },
            FieldInfoData {
                name: "AutoRestartOriginSDK",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OriginSettings, auto_restart_origin_s_d_k),
            },
        ],
    }),
    array_type: Some(ORIGINSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OriginSettings {
    fn type_info() -> &'static TypeInfo {
        ORIGINSETTINGS_TYPE_INFO
    }
}


pub const ORIGINSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "OriginShared",
    data: TypeInfoData::Array("OriginSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginCoreNotAvailableMessage {
}

pub const ORIGINCORENOTAVAILABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginCoreNotAvailableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginCoreNotAvailableMessage {
    fn type_info() -> &'static TypeInfo {
        ORIGINCORENOTAVAILABLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginNotLoadedMessage {
}

pub const ORIGINNOTLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginNotLoadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginNotLoadedMessage {
    fn type_info() -> &'static TypeInfo {
        ORIGINNOTLOADEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginOnlineMessage {
}

pub const ORIGINONLINEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginOnlineMessage",
    flags: MemberInfoFlags::new(73),
    module: "OriginShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginOnlineMessage {
    fn type_info() -> &'static TypeInfo {
        ORIGINONLINEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginResponseMessageBase {
}

pub const ORIGINRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginResponseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OriginResponseMessageBase {
    fn type_info() -> &'static TypeInfo {
        ORIGINRESPONSEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginRequestMessageBase {
}

pub const ORIGINREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OriginRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        ORIGINREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginJoinableMessageBase {
}

pub const ORIGINJOINABLEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginJoinableMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OriginJoinableMessageBase {
    fn type_info() -> &'static TypeInfo {
        ORIGINJOINABLEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginErrorMessage {
}

pub const ORIGINERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginErrorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginErrorMessage {
    fn type_info() -> &'static TypeInfo {
        ORIGINERRORMESSAGE_TYPE_INFO
    }
}

