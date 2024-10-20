use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct OriginSettings {
    pub _glacier_base: super::core::SystemSettings,
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

pub trait OriginSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn required_for_online(&self) -> &bool;
    fn require_latest_for_online_features(&self) -> &bool;
    fn content_id(&self) -> &String;
    fn title(&self) -> &String;
    fn multiplayer_id(&self) -> &String;
    fn language(&self) -> &String;
    fn log(&self) -> &bool;
    fn allow_production_environment(&self) -> &bool;
    fn achievements_secret(&self) -> &String;
    fn achievements_timeout(&self) -> &i32;
    fn entitlements_timeout(&self) -> &i32;
    fn installer_game(&self) -> &String;
    fn installer_studio(&self) -> &String;
    fn disable_in_live_edit_mode(&self) -> &bool;
    fn auto_restart_origin_s_d_k(&self) -> &bool;
}

impl OriginSettingsTrait for OriginSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn required_for_online(&self) -> &bool {
        &self.required_for_online
    }
    fn require_latest_for_online_features(&self) -> &bool {
        &self.require_latest_for_online_features
    }
    fn content_id(&self) -> &String {
        &self.content_id
    }
    fn title(&self) -> &String {
        &self.title
    }
    fn multiplayer_id(&self) -> &String {
        &self.multiplayer_id
    }
    fn language(&self) -> &String {
        &self.language
    }
    fn log(&self) -> &bool {
        &self.log
    }
    fn allow_production_environment(&self) -> &bool {
        &self.allow_production_environment
    }
    fn achievements_secret(&self) -> &String {
        &self.achievements_secret
    }
    fn achievements_timeout(&self) -> &i32 {
        &self.achievements_timeout
    }
    fn entitlements_timeout(&self) -> &i32 {
        &self.entitlements_timeout
    }
    fn installer_game(&self) -> &String {
        &self.installer_game
    }
    fn installer_studio(&self) -> &String {
        &self.installer_studio
    }
    fn disable_in_live_edit_mode(&self) -> &bool {
        &self.disable_in_live_edit_mode
    }
    fn auto_restart_origin_s_d_k(&self) -> &bool {
        &self.auto_restart_origin_s_d_k
    }
}

impl super::core::SystemSettingsTrait for OriginSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for OriginSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ORIGINSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginSettings",
    flags: MemberInfoFlags::new(101),
    module: "OriginShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OriginSettings, enabled),
            },
            FieldInfoData {
                name: "RequiredForOnline",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OriginSettings, required_for_online),
            },
            FieldInfoData {
                name: "RequireLatestForOnlineFeatures",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OriginSettings, require_latest_for_online_features),
            },
            FieldInfoData {
                name: "ContentId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OriginSettings, content_id),
            },
            FieldInfoData {
                name: "Title",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OriginSettings, title),
            },
            FieldInfoData {
                name: "MultiplayerId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OriginSettings, multiplayer_id),
            },
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OriginSettings, language),
            },
            FieldInfoData {
                name: "Log",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OriginSettings, log),
            },
            FieldInfoData {
                name: "AllowProductionEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OriginSettings, allow_production_environment),
            },
            FieldInfoData {
                name: "AchievementsSecret",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OriginSettings, achievements_secret),
            },
            FieldInfoData {
                name: "AchievementsTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OriginSettings, achievements_timeout),
            },
            FieldInfoData {
                name: "EntitlementsTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OriginSettings, entitlements_timeout),
            },
            FieldInfoData {
                name: "InstallerGame",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OriginSettings, installer_game),
            },
            FieldInfoData {
                name: "InstallerStudio",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OriginSettings, installer_studio),
            },
            FieldInfoData {
                name: "DisableInLiveEditMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OriginSettings, disable_in_live_edit_mode),
            },
            FieldInfoData {
                name: "AutoRestartOriginSDK",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OriginSettings, auto_restart_origin_s_d_k),
            },
        ],
    }),
    array_type: Some(ORIGINSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OriginSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ORIGINSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "OriginShared",
    data: TypeInfoData::Array("OriginSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OriginCoreNotAvailableMessage {
}

pub trait OriginCoreNotAvailableMessageTrait: TypeObject {
}

impl OriginCoreNotAvailableMessageTrait for OriginCoreNotAvailableMessage {
}

pub static ORIGINCORENOTAVAILABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginCoreNotAvailableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginCoreNotAvailableMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginCoreNotAvailableMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINCORENOTAVAILABLEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OriginNotLoadedMessage {
}

pub trait OriginNotLoadedMessageTrait: TypeObject {
}

impl OriginNotLoadedMessageTrait for OriginNotLoadedMessage {
}

pub static ORIGINNOTLOADEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginNotLoadedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginNotLoadedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginNotLoadedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINNOTLOADEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OriginOnlineMessage {
}

pub trait OriginOnlineMessageTrait: TypeObject {
}

impl OriginOnlineMessageTrait for OriginOnlineMessage {
}

pub static ORIGINONLINEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginOnlineMessage",
    flags: MemberInfoFlags::new(73),
    module: "OriginShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginOnlineMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginOnlineMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINONLINEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OriginResponseMessageBase {
}

pub trait OriginResponseMessageBaseTrait: TypeObject {
}

impl OriginResponseMessageBaseTrait for OriginResponseMessageBase {
}

pub static ORIGINRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginResponseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginResponseMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OriginResponseMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINRESPONSEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OriginRequestMessageBase {
}

pub trait OriginRequestMessageBaseTrait: TypeObject {
}

impl OriginRequestMessageBaseTrait for OriginRequestMessageBase {
}

pub static ORIGINREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginRequestMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OriginRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINREQUESTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OriginJoinableMessageBase {
}

pub trait OriginJoinableMessageBaseTrait: TypeObject {
}

impl OriginJoinableMessageBaseTrait for OriginJoinableMessageBase {
}

pub static ORIGINJOINABLEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginJoinableMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginJoinableMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for OriginJoinableMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINJOINABLEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct OriginErrorMessage {
}

pub trait OriginErrorMessageTrait: TypeObject {
}

impl OriginErrorMessageTrait for OriginErrorMessage {
}

pub static ORIGINERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginErrorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OriginShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginErrorMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for OriginErrorMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINERRORMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

