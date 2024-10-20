use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct SnowglobeTrackerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub name_filter: String,
}

pub trait SnowglobeTrackerEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn name_filter(&self) -> &String;
}

impl SnowglobeTrackerEntityDataTrait for SnowglobeTrackerEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn name_filter(&self) -> &String {
        &self.name_filter
    }
}

impl super::entity::EntityDataTrait for SnowglobeTrackerEntityData {
}

impl super::entity::GameObjectDataTrait for SnowglobeTrackerEntityData {
}

impl super::core::DataBusPeerTrait for SnowglobeTrackerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for SnowglobeTrackerEntityData {
}

impl super::core::DataContainerTrait for SnowglobeTrackerEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SNOWGLOBETRACKERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeTrackerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(SnowglobeTrackerEntityData, realm),
            },
            FieldInfoData {
                name: "NameFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SnowglobeTrackerEntityData, name_filter),
            },
        ],
    }),
    array_type: Some(SNOWGLOBETRACKERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnowglobeTrackerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SNOWGLOBETRACKERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SNOWGLOBETRACKERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeTrackerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SnowglobeComponentData {
    pub _glacier_base: super::entity::ComponentData,
}

pub trait SnowglobeComponentDataTrait: super::entity::ComponentDataTrait {
}

impl SnowglobeComponentDataTrait for SnowglobeComponentData {
}

impl super::entity::ComponentDataTrait for SnowglobeComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for SnowglobeComponentData {
}

impl super::core::DataBusPeerTrait for SnowglobeComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for SnowglobeComponentData {
}

impl super::core::DataContainerTrait for SnowglobeComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SNOWGLOBECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeComponentData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SNOWGLOBECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SnowglobeComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        SNOWGLOBECOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SNOWGLOBECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicSettingsSplitterData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub settings: Option<Arc<Mutex<dyn CinematicSettingsTrait>>>,
}

pub trait CinematicSettingsSplitterDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn settings(&self) -> &Option<Arc<Mutex<dyn CinematicSettingsTrait>>>;
}

impl CinematicSettingsSplitterDataTrait for CinematicSettingsSplitterData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn settings(&self) -> &Option<Arc<Mutex<dyn CinematicSettingsTrait>>> {
        &self.settings
    }
}

impl super::entity::EntityDataTrait for CinematicSettingsSplitterData {
}

impl super::entity::GameObjectDataTrait for CinematicSettingsSplitterData {
}

impl super::core::DataBusPeerTrait for CinematicSettingsSplitterData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for CinematicSettingsSplitterData {
}

impl super::core::DataContainerTrait for CinematicSettingsSplitterData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICSETTINGSSPLITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicSettingsSplitterData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CinematicSettingsSplitterData, realm),
            },
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicSettings",
                rust_offset: offset_of!(CinematicSettingsSplitterData, settings),
            },
        ],
    }),
    array_type: Some(CINEMATICSETTINGSSPLITTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicSettingsSplitterData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICSETTINGSSPLITTERDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CINEMATICSETTINGSSPLITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettingsSplitterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NetworkSnowglobeStatusChangeMessage {
}

pub trait NetworkSnowglobeStatusChangeMessageTrait: TypeObject {
}

impl NetworkSnowglobeStatusChangeMessageTrait for NetworkSnowglobeStatusChangeMessage {
}

pub static NETWORKSNOWGLOBESTATUSCHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSnowglobeStatusChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snowglobe",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSnowglobeStatusChangeMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSnowglobeStatusChangeMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSNOWGLOBESTATUSCHANGEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CinematicSettings {
    pub _glacier_base: super::core::DataContainer,
    pub continue_i_g_c_settings_on_finish: bool,
    pub user_control: UserInputType,
    pub allow_restart: bool,
    pub hide_h_u_d: bool,
    pub disable_motion_blur: bool,
    pub enable_cinematic_camera: bool,
    pub disable_visual_environment_effects: bool,
}

pub trait CinematicSettingsTrait: super::core::DataContainerTrait {
    fn continue_i_g_c_settings_on_finish(&self) -> &bool;
    fn user_control(&self) -> &UserInputType;
    fn allow_restart(&self) -> &bool;
    fn hide_h_u_d(&self) -> &bool;
    fn disable_motion_blur(&self) -> &bool;
    fn enable_cinematic_camera(&self) -> &bool;
    fn disable_visual_environment_effects(&self) -> &bool;
}

impl CinematicSettingsTrait for CinematicSettings {
    fn continue_i_g_c_settings_on_finish(&self) -> &bool {
        &self.continue_i_g_c_settings_on_finish
    }
    fn user_control(&self) -> &UserInputType {
        &self.user_control
    }
    fn allow_restart(&self) -> &bool {
        &self.allow_restart
    }
    fn hide_h_u_d(&self) -> &bool {
        &self.hide_h_u_d
    }
    fn disable_motion_blur(&self) -> &bool {
        &self.disable_motion_blur
    }
    fn enable_cinematic_camera(&self) -> &bool {
        &self.enable_cinematic_camera
    }
    fn disable_visual_environment_effects(&self) -> &bool {
        &self.disable_visual_environment_effects
    }
}

impl super::core::DataContainerTrait for CinematicSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ContinueIGCSettingsOnFinish",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, continue_i_g_c_settings_on_finish),
            },
            FieldInfoData {
                name: "UserControl",
                flags: MemberInfoFlags::new(0),
                field_type: "UserInputType",
                rust_offset: offset_of!(CinematicSettings, user_control),
            },
            FieldInfoData {
                name: "AllowRestart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, allow_restart),
            },
            FieldInfoData {
                name: "HideHUD",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, hide_h_u_d),
            },
            FieldInfoData {
                name: "DisableMotionBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, disable_motion_blur),
            },
            FieldInfoData {
                name: "EnableCinematicCamera",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, enable_cinematic_camera),
            },
            FieldInfoData {
                name: "DisableVisualEnvironmentEffects",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, disable_visual_environment_effects),
            },
        ],
    }),
    array_type: Some(CINEMATICSETTINGS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CinematicSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CINEMATICSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SnowglobeEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub prime_behavior: PrimeType,
    pub clean_up_behavior: CleanUpType,
    pub multi_player_behavior: MultiplayerStartType,
    pub external_time: f32,
    pub cinematic_name: String,
    pub cinematic_settings: Option<Arc<Mutex<dyn CinematicSettingsTrait>>>,
}

pub trait SnowglobeEntityDataTrait: super::entity::EntityDataTrait {
    fn prime_behavior(&self) -> &PrimeType;
    fn clean_up_behavior(&self) -> &CleanUpType;
    fn multi_player_behavior(&self) -> &MultiplayerStartType;
    fn external_time(&self) -> &f32;
    fn cinematic_name(&self) -> &String;
    fn cinematic_settings(&self) -> &Option<Arc<Mutex<dyn CinematicSettingsTrait>>>;
}

impl SnowglobeEntityDataTrait for SnowglobeEntityData {
    fn prime_behavior(&self) -> &PrimeType {
        &self.prime_behavior
    }
    fn clean_up_behavior(&self) -> &CleanUpType {
        &self.clean_up_behavior
    }
    fn multi_player_behavior(&self) -> &MultiplayerStartType {
        &self.multi_player_behavior
    }
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn cinematic_name(&self) -> &String {
        &self.cinematic_name
    }
    fn cinematic_settings(&self) -> &Option<Arc<Mutex<dyn CinematicSettingsTrait>>> {
        &self.cinematic_settings
    }
}

impl super::entity::EntityDataTrait for SnowglobeEntityData {
}

impl super::entity::GameObjectDataTrait for SnowglobeEntityData {
}

impl super::core::DataBusPeerTrait for SnowglobeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for SnowglobeEntityData {
}

impl super::core::DataContainerTrait for SnowglobeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SNOWGLOBEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PrimeBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: "PrimeType",
                rust_offset: offset_of!(SnowglobeEntityData, prime_behavior),
            },
            FieldInfoData {
                name: "CleanUpBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: "CleanUpType",
                rust_offset: offset_of!(SnowglobeEntityData, clean_up_behavior),
            },
            FieldInfoData {
                name: "MultiPlayerBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: "MultiplayerStartType",
                rust_offset: offset_of!(SnowglobeEntityData, multi_player_behavior),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SnowglobeEntityData, external_time),
            },
            FieldInfoData {
                name: "CinematicName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SnowglobeEntityData, cinematic_name),
            },
            FieldInfoData {
                name: "CinematicSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicSettings",
                rust_offset: offset_of!(SnowglobeEntityData, cinematic_settings),
            },
        ],
    }),
    array_type: Some(SNOWGLOBEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnowglobeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SNOWGLOBEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SNOWGLOBEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static SNOWGLOBEENTITYSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(SNOWGLOBEENTITYSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SnowglobeEntityStatus {
    fn type_info(&self) -> &'static TypeInfo {
        SNOWGLOBEENTITYSTATUS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SNOWGLOBEENTITYSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeEntityStatus"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MultiplayerStartType {
    #[default]
    MultiplayerStartType_WaitOnAllClients = 0,
    MultiplayerStartType_PlayWhenTriggered = 1,
}

pub static MULTIPLAYERSTARTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerStartType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(MULTIPLAYERSTARTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MultiplayerStartType {
    fn type_info(&self) -> &'static TypeInfo {
        MULTIPLAYERSTARTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MULTIPLAYERSTARTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerStartType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("MultiplayerStartType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UserInputType {
    #[default]
    UserInputType_NoUserControl = 0,
    UserInputType_CameraOnly = 1,
    UserInputType_FullUserControl = 2,
}

pub static USERINPUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserInputType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(USERINPUTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UserInputType {
    fn type_info(&self) -> &'static TypeInfo {
        USERINPUTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static USERINPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserInputType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("UserInputType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CleanUpType {
    #[default]
    CleanUpType_Automatic = 0,
    CleanUpType_Manual = 1,
}

pub static CLEANUPTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CleanUpType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(CLEANUPTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CleanUpType {
    fn type_info(&self) -> &'static TypeInfo {
        CLEANUPTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLEANUPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CleanUpType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CleanUpType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PrimeType {
    #[default]
    PrimeType_OnLoad = 0,
    PrimeType_OnStartRequest = 1,
    PrimeType_Manual = 2,
}

pub static PRIMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimeType",
    flags: MemberInfoFlags::new(49429),
    module: "Snowglobe",
    data: TypeInfoData::Enum,
    array_type: Some(PRIMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PrimeType {
    fn type_info(&self) -> &'static TypeInfo {
        PRIMETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRIMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("PrimeType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerSnowglobeEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSnowglobeEntityTrait: super::entity::EntityTrait {
}

impl ServerSnowglobeEntityTrait for ServerSnowglobeEntity {
}

impl super::entity::EntityTrait for ServerSnowglobeEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSnowglobeEntity {
}

pub static SERVERSNOWGLOBEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSnowglobeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSnowglobeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSNOWGLOBEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSnowglobeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSNOWGLOBEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERSNOWGLOBEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSnowglobeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("ServerSnowglobeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SnowglobeTrackerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SnowglobeTrackerEntityTrait: super::entity::EntityTrait {
}

impl SnowglobeTrackerEntityTrait for SnowglobeTrackerEntity {
}

impl super::entity::EntityTrait for SnowglobeTrackerEntity {
}

impl super::entity::EntityBusPeerTrait for SnowglobeTrackerEntity {
}

pub static SNOWGLOBETRACKERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeTrackerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SNOWGLOBETRACKERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SnowglobeTrackerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SNOWGLOBETRACKERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SNOWGLOBETRACKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeTrackerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SnowglobeComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait SnowglobeComponentTrait: super::entity::ComponentTrait {
}

impl SnowglobeComponentTrait for SnowglobeComponent {
}

impl super::entity::ComponentTrait for SnowglobeComponent {
}

impl super::entity::EntityBusPeerTrait for SnowglobeComponent {
}

pub static SNOWGLOBECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponent",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SNOWGLOBECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SnowglobeComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SNOWGLOBECOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SNOWGLOBECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicSettingsSplitterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CinematicSettingsSplitterEntityTrait: super::entity::EntityTrait {
}

impl CinematicSettingsSplitterEntityTrait for CinematicSettingsSplitterEntity {
}

impl super::entity::EntityTrait for CinematicSettingsSplitterEntity {
}

impl super::entity::EntityBusPeerTrait for CinematicSettingsSplitterEntity {
}

pub static CINEMATICSETTINGSSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicSettingsSplitterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICSETTINGSSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CinematicSettingsSplitterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICSETTINGSSPLITTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CINEMATICSETTINGSSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettingsSplitterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientSnowglobeEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSnowglobeEntityTrait: super::entity::EntityTrait {
}

impl ClientSnowglobeEntityTrait for ClientSnowglobeEntity {
}

impl super::entity::EntityTrait for ClientSnowglobeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSnowglobeEntity {
}

pub static CLIENTSNOWGLOBEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSnowglobeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSnowglobeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSNOWGLOBEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSnowglobeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSNOWGLOBEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTSNOWGLOBEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSnowglobeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("ClientSnowglobeEntity"),
    array_type: None,
    alignment: 8,
};


