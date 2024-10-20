use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_snowglobe_types(registry: &mut TypeRegistry) {
    registry.register_type(SNOWGLOBETRACKERENTITYDATA_TYPE_INFO);
    registry.register_type(SNOWGLOBETRACKERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SNOWGLOBECOMPONENTDATA_TYPE_INFO);
    registry.register_type(SNOWGLOBECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICSETTINGSSPLITTERDATA_TYPE_INFO);
    registry.register_type(CINEMATICSETTINGSSPLITTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKSNOWGLOBESTATUSCHANGEMESSAGE_TYPE_INFO);
    registry.register_type(CINEMATICSETTINGS_TYPE_INFO);
    registry.register_type(CINEMATICSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SNOWGLOBEENTITYDATA_TYPE_INFO);
    registry.register_type(SNOWGLOBEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SNOWGLOBEENTITYSTATUS_TYPE_INFO);
    registry.register_type(SNOWGLOBEENTITYSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(MULTIPLAYERSTARTTYPE_TYPE_INFO);
    registry.register_type(MULTIPLAYERSTARTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(USERINPUTTYPE_TYPE_INFO);
    registry.register_type(USERINPUTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CLEANUPTYPE_TYPE_INFO);
    registry.register_type(CLEANUPTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PRIMETYPE_TYPE_INFO);
    registry.register_type(PRIMETYPE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSNOWGLOBEENTITY_TYPE_INFO);
    registry.register_type(SERVERSNOWGLOBEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SNOWGLOBETRACKERENTITY_TYPE_INFO);
    registry.register_type(SNOWGLOBETRACKERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SNOWGLOBECOMPONENT_TYPE_INFO);
    registry.register_type(SNOWGLOBECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICSETTINGSSPLITTERENTITY_TYPE_INFO);
    registry.register_type(CINEMATICSETTINGSSPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSNOWGLOBEENTITY_TYPE_INFO);
    registry.register_type(CLIENTSNOWGLOBEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SnowglobeTrackerEntityData {
    pub realm: super::core::Realm,
    pub name_filter: String,
}

pub const SNOWGLOBETRACKERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeTrackerEntityData, realm),
            },
            FieldInfoData {
                name: "NameFilter",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeTrackerEntityData, name_filter),
            },
        ],
    }),
    array_type: Some(SNOWGLOBETRACKERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnowglobeTrackerEntityData {
    fn type_info() -> &'static TypeInfo {
        SNOWGLOBETRACKERENTITYDATA_TYPE_INFO
    }
}


pub const SNOWGLOBETRACKERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeTrackerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SnowglobeComponentData {
}

pub const SNOWGLOBECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SNOWGLOBECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SnowglobeComponentData {
    fn type_info() -> &'static TypeInfo {
        SNOWGLOBECOMPONENTDATA_TYPE_INFO
    }
}


pub const SNOWGLOBECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicSettingsSplitterData {
    pub realm: super::core::Realm,
    pub settings: CinematicSettings,
}

pub const CINEMATICSETTINGSSPLITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettingsSplitterData, realm),
            },
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(0),
                field_type: CINEMATICSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettingsSplitterData, settings),
            },
        ],
    }),
    array_type: Some(CINEMATICSETTINGSSPLITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicSettingsSplitterData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICSETTINGSSPLITTERDATA_TYPE_INFO
    }
}


pub const CINEMATICSETTINGSSPLITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettingsSplitterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSnowglobeStatusChangeMessage {
}

pub const NETWORKSNOWGLOBESTATUSCHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSnowglobeStatusChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snowglobe",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSnowglobeStatusChangeMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSNOWGLOBESTATUSCHANGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicSettings {
    pub continue_i_g_c_settings_on_finish: bool,
    pub user_control: UserInputType,
    pub allow_restart: bool,
    pub hide_h_u_d: bool,
    pub disable_motion_blur: bool,
    pub enable_cinematic_camera: bool,
    pub disable_visual_environment_effects: bool,
}

pub const CINEMATICSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ContinueIGCSettingsOnFinish",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettings, continue_i_g_c_settings_on_finish),
            },
            FieldInfoData {
                name: "UserControl",
                flags: MemberInfoFlags::new(0),
                field_type: USERINPUTTYPE_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettings, user_control),
            },
            FieldInfoData {
                name: "AllowRestart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettings, allow_restart),
            },
            FieldInfoData {
                name: "HideHUD",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettings, hide_h_u_d),
            },
            FieldInfoData {
                name: "DisableMotionBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettings, disable_motion_blur),
            },
            FieldInfoData {
                name: "EnableCinematicCamera",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettings, enable_cinematic_camera),
            },
            FieldInfoData {
                name: "DisableVisualEnvironmentEffects",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicSettings, disable_visual_environment_effects),
            },
        ],
    }),
    array_type: Some(CINEMATICSETTINGS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CinematicSettings {
    fn type_info() -> &'static TypeInfo {
        CINEMATICSETTINGS_TYPE_INFO
    }
}


pub const CINEMATICSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SnowglobeEntityData {
    pub prime_behavior: PrimeType,
    pub clean_up_behavior: CleanUpType,
    pub multi_player_behavior: MultiplayerStartType,
    pub external_time: f32,
    pub cinematic_name: String,
    pub cinematic_settings: CinematicSettings,
}

pub const SNOWGLOBEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PrimeBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: PRIMETYPE_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeEntityData, prime_behavior),
            },
            FieldInfoData {
                name: "CleanUpBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: CLEANUPTYPE_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeEntityData, clean_up_behavior),
            },
            FieldInfoData {
                name: "MultiPlayerBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: MULTIPLAYERSTARTTYPE_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeEntityData, multi_player_behavior),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeEntityData, external_time),
            },
            FieldInfoData {
                name: "CinematicName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeEntityData, cinematic_name),
            },
            FieldInfoData {
                name: "CinematicSettings",
                flags: MemberInfoFlags::new(0),
                field_type: CINEMATICSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(SnowglobeEntityData, cinematic_settings),
            },
        ],
    }),
    array_type: Some(SNOWGLOBEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnowglobeEntityData {
    fn type_info() -> &'static TypeInfo {
        SNOWGLOBEENTITYDATA_TYPE_INFO
    }
}


pub const SNOWGLOBEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SnowglobeEntityStatus {
    #[default]
    SnowglobeEntityStatus_Created = 0,
    SnowglobeEntityStatus_Priming = 1,
    SnowglobeEntityStatus_ReadyToPlay = 2,
    SnowglobeEntityStatus_PendingSystemApprovalToPlay = 3,
    SnowglobeEntityStatus_Playing = 4,
    SnowglobeEntityStatus_Stopped = 5,
    SnowglobeEntityStatus_Finished = 6,
    SnowglobeEntityStatus_CleaningUp = 7,
    SnowglobeEntityStatus_ReadyToBeUnloaded = 8,
}

pub const SNOWGLOBEENTITYSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(SNOWGLOBEENTITYSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SnowglobeEntityStatus {
    fn type_info() -> &'static TypeInfo {
        SNOWGLOBEENTITYSTATUS_TYPE_INFO
    }
}


pub const SNOWGLOBEENTITYSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeEntityStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MultiplayerStartType {
    #[default]
    MultiplayerStartType_WaitOnAllClients = 0,
    MultiplayerStartType_PlayWhenTriggered = 1,
}

pub const MULTIPLAYERSTARTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerStartType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(MULTIPLAYERSTARTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MultiplayerStartType {
    fn type_info() -> &'static TypeInfo {
        MULTIPLAYERSTARTTYPE_TYPE_INFO
    }
}


pub const MULTIPLAYERSTARTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerStartType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("MultiplayerStartType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UserInputType {
    #[default]
    UserInputType_NoUserControl = 0,
    UserInputType_CameraOnly = 1,
    UserInputType_FullUserControl = 2,
}

pub const USERINPUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserInputType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(USERINPUTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UserInputType {
    fn type_info() -> &'static TypeInfo {
        USERINPUTTYPE_TYPE_INFO
    }
}


pub const USERINPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserInputType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("UserInputType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CleanUpType {
    #[default]
    CleanUpType_Automatic = 0,
    CleanUpType_Manual = 1,
}

pub const CLEANUPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CleanUpType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(CLEANUPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CleanUpType {
    fn type_info() -> &'static TypeInfo {
        CLEANUPTYPE_TYPE_INFO
    }
}


pub const CLEANUPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CleanUpType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CleanUpType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PrimeType {
    #[default]
    PrimeType_OnLoad = 0,
    PrimeType_OnStartRequest = 1,
    PrimeType_Manual = 2,
}

pub const PRIMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimeType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(PRIMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PrimeType {
    fn type_info() -> &'static TypeInfo {
        PRIMETYPE_TYPE_INFO
    }
}


pub const PRIMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("PrimeType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSnowglobeEntity {
}

pub const SERVERSNOWGLOBEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSnowglobeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSNOWGLOBEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSnowglobeEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSNOWGLOBEENTITY_TYPE_INFO
    }
}


pub const SERVERSNOWGLOBEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSnowglobeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("ServerSnowglobeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SnowglobeTrackerEntity {
}

pub const SNOWGLOBETRACKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SNOWGLOBETRACKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SnowglobeTrackerEntity {
    fn type_info() -> &'static TypeInfo {
        SNOWGLOBETRACKERENTITY_TYPE_INFO
    }
}


pub const SNOWGLOBETRACKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeTrackerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SnowglobeComponent {
}

pub const SNOWGLOBECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponent",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SNOWGLOBECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SnowglobeComponent {
    fn type_info() -> &'static TypeInfo {
        SNOWGLOBECOMPONENT_TYPE_INFO
    }
}


pub const SNOWGLOBECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicSettingsSplitterEntity {
}

pub const CINEMATICSETTINGSSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICSETTINGSSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CinematicSettingsSplitterEntity {
    fn type_info() -> &'static TypeInfo {
        CINEMATICSETTINGSSPLITTERENTITY_TYPE_INFO
    }
}


pub const CINEMATICSETTINGSSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettingsSplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSnowglobeEntity {
}

pub const CLIENTSNOWGLOBEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSnowglobeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSNOWGLOBEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSnowglobeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSNOWGLOBEENTITY_TYPE_INFO
    }
}


pub const CLIENTSNOWGLOBEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSnowglobeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("ClientSnowglobeEntity-Array"),
    array_type: None,
    alignment: 8,
};


