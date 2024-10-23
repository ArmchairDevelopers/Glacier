use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SnowglobeTrackerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub name_filter: String,
}

pub trait SnowglobeTrackerEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn name_filter(&self) -> &String;
    fn name_filter_mut(&mut self) -> &mut String;
}

impl SnowglobeTrackerEntityDataTrait for SnowglobeTrackerEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn name_filter(&self) -> &String {
        &self.name_filter
    }
    fn name_filter_mut(&mut self) -> &mut String {
        &mut self.name_filter
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SnowglobeTrackerEntityData {
}

impl super::core::DataContainerTrait for SnowglobeTrackerEntityData {
}

pub static SNOWGLOBETRACKERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntityData",
    name_hash: 2816070736,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SnowglobeTrackerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeTrackerEntityData as Default>::default())),
            create_boxed: || Box::new(<SnowglobeTrackerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(SnowglobeTrackerEntityData, realm),
            },
            FieldInfoData {
                name: "NameFilter",
                name_hash: 122941570,
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


pub static SNOWGLOBETRACKERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntityData-Array",
    name_hash: 476934884,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeTrackerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for SnowglobeComponentData {
}

impl super::core::DataBusPeerTrait for SnowglobeComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SnowglobeComponentData {
}

impl super::core::DataContainerTrait for SnowglobeComponentData {
}

pub static SNOWGLOBECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponentData",
    name_hash: 2716687548,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(SnowglobeComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeComponentData as Default>::default())),
            create_boxed: || Box::new(<SnowglobeComponentData as Default>::default()),
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


pub static SNOWGLOBECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponentData-Array",
    name_hash: 3039859720,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicSettingsSplitterData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub settings: Option<LockedTypeObject /* CinematicSettings */>,
}

pub trait CinematicSettingsSplitterDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn settings(&self) -> &Option<LockedTypeObject /* CinematicSettings */>;
    fn settings_mut(&mut self) -> &mut Option<LockedTypeObject /* CinematicSettings */>;
}

impl CinematicSettingsSplitterDataTrait for CinematicSettingsSplitterData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn settings(&self) -> &Option<LockedTypeObject /* CinematicSettings */> {
        &self.settings
    }
    fn settings_mut(&mut self) -> &mut Option<LockedTypeObject /* CinematicSettings */> {
        &mut self.settings
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicSettingsSplitterData {
}

impl super::core::DataContainerTrait for CinematicSettingsSplitterData {
}

pub static CINEMATICSETTINGSSPLITTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterData",
    name_hash: 942491378,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicSettingsSplitterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicSettingsSplitterData as Default>::default())),
            create_boxed: || Box::new(<CinematicSettingsSplitterData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CinematicSettingsSplitterData, realm),
            },
            FieldInfoData {
                name: "Settings",
                name_hash: 649772672,
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


pub static CINEMATICSETTINGSSPLITTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterData-Array",
    name_hash: 3857735110,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettingsSplitterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkSnowglobeStatusChangeMessage {
}

pub trait NetworkSnowglobeStatusChangeMessageTrait: TypeObject {
}

impl NetworkSnowglobeStatusChangeMessageTrait for NetworkSnowglobeStatusChangeMessage {
}

pub static NETWORKSNOWGLOBESTATUSCHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSnowglobeStatusChangeMessage",
    name_hash: 395434596,
    flags: MemberInfoFlags::new(36937),
    module: "Snowglobe",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSnowglobeStatusChangeMessage as Default>::default())),
            create_boxed: || Box::new(<NetworkSnowglobeStatusChangeMessage as Default>::default()),
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
    fn continue_i_g_c_settings_on_finish_mut(&mut self) -> &mut bool;
    fn user_control(&self) -> &UserInputType;
    fn user_control_mut(&mut self) -> &mut UserInputType;
    fn allow_restart(&self) -> &bool;
    fn allow_restart_mut(&mut self) -> &mut bool;
    fn hide_h_u_d(&self) -> &bool;
    fn hide_h_u_d_mut(&mut self) -> &mut bool;
    fn disable_motion_blur(&self) -> &bool;
    fn disable_motion_blur_mut(&mut self) -> &mut bool;
    fn enable_cinematic_camera(&self) -> &bool;
    fn enable_cinematic_camera_mut(&mut self) -> &mut bool;
    fn disable_visual_environment_effects(&self) -> &bool;
    fn disable_visual_environment_effects_mut(&mut self) -> &mut bool;
}

impl CinematicSettingsTrait for CinematicSettings {
    fn continue_i_g_c_settings_on_finish(&self) -> &bool {
        &self.continue_i_g_c_settings_on_finish
    }
    fn continue_i_g_c_settings_on_finish_mut(&mut self) -> &mut bool {
        &mut self.continue_i_g_c_settings_on_finish
    }
    fn user_control(&self) -> &UserInputType {
        &self.user_control
    }
    fn user_control_mut(&mut self) -> &mut UserInputType {
        &mut self.user_control
    }
    fn allow_restart(&self) -> &bool {
        &self.allow_restart
    }
    fn allow_restart_mut(&mut self) -> &mut bool {
        &mut self.allow_restart
    }
    fn hide_h_u_d(&self) -> &bool {
        &self.hide_h_u_d
    }
    fn hide_h_u_d_mut(&mut self) -> &mut bool {
        &mut self.hide_h_u_d
    }
    fn disable_motion_blur(&self) -> &bool {
        &self.disable_motion_blur
    }
    fn disable_motion_blur_mut(&mut self) -> &mut bool {
        &mut self.disable_motion_blur
    }
    fn enable_cinematic_camera(&self) -> &bool {
        &self.enable_cinematic_camera
    }
    fn enable_cinematic_camera_mut(&mut self) -> &mut bool {
        &mut self.enable_cinematic_camera
    }
    fn disable_visual_environment_effects(&self) -> &bool {
        &self.disable_visual_environment_effects
    }
    fn disable_visual_environment_effects_mut(&mut self) -> &mut bool {
        &mut self.disable_visual_environment_effects
    }
}

impl super::core::DataContainerTrait for CinematicSettings {
}

pub static CINEMATICSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettings",
    name_hash: 684665523,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CinematicSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicSettings as Default>::default())),
            create_boxed: || Box::new(<CinematicSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ContinueIGCSettingsOnFinish",
                name_hash: 2549985662,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, continue_i_g_c_settings_on_finish),
            },
            FieldInfoData {
                name: "UserControl",
                name_hash: 917789299,
                flags: MemberInfoFlags::new(0),
                field_type: "UserInputType",
                rust_offset: offset_of!(CinematicSettings, user_control),
            },
            FieldInfoData {
                name: "AllowRestart",
                name_hash: 1676675563,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, allow_restart),
            },
            FieldInfoData {
                name: "HideHUD",
                name_hash: 1849196028,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, hide_h_u_d),
            },
            FieldInfoData {
                name: "DisableMotionBlur",
                name_hash: 336091974,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, disable_motion_blur),
            },
            FieldInfoData {
                name: "EnableCinematicCamera",
                name_hash: 3473119118,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicSettings, enable_cinematic_camera),
            },
            FieldInfoData {
                name: "DisableVisualEnvironmentEffects",
                name_hash: 4258705332,
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


pub static CINEMATICSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettings-Array",
    name_hash: 2298844679,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SnowglobeEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub prime_behavior: PrimeType,
    pub clean_up_behavior: CleanUpType,
    pub multi_player_behavior: MultiplayerStartType,
    pub external_time: f32,
    pub cinematic_name: String,
    pub cinematic_settings: Option<LockedTypeObject /* CinematicSettings */>,
}

pub trait SnowglobeEntityDataTrait: super::entity::EntityDataTrait {
    fn prime_behavior(&self) -> &PrimeType;
    fn prime_behavior_mut(&mut self) -> &mut PrimeType;
    fn clean_up_behavior(&self) -> &CleanUpType;
    fn clean_up_behavior_mut(&mut self) -> &mut CleanUpType;
    fn multi_player_behavior(&self) -> &MultiplayerStartType;
    fn multi_player_behavior_mut(&mut self) -> &mut MultiplayerStartType;
    fn external_time(&self) -> &f32;
    fn external_time_mut(&mut self) -> &mut f32;
    fn cinematic_name(&self) -> &String;
    fn cinematic_name_mut(&mut self) -> &mut String;
    fn cinematic_settings(&self) -> &Option<LockedTypeObject /* CinematicSettings */>;
    fn cinematic_settings_mut(&mut self) -> &mut Option<LockedTypeObject /* CinematicSettings */>;
}

impl SnowglobeEntityDataTrait for SnowglobeEntityData {
    fn prime_behavior(&self) -> &PrimeType {
        &self.prime_behavior
    }
    fn prime_behavior_mut(&mut self) -> &mut PrimeType {
        &mut self.prime_behavior
    }
    fn clean_up_behavior(&self) -> &CleanUpType {
        &self.clean_up_behavior
    }
    fn clean_up_behavior_mut(&mut self) -> &mut CleanUpType {
        &mut self.clean_up_behavior
    }
    fn multi_player_behavior(&self) -> &MultiplayerStartType {
        &self.multi_player_behavior
    }
    fn multi_player_behavior_mut(&mut self) -> &mut MultiplayerStartType {
        &mut self.multi_player_behavior
    }
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn external_time_mut(&mut self) -> &mut f32 {
        &mut self.external_time
    }
    fn cinematic_name(&self) -> &String {
        &self.cinematic_name
    }
    fn cinematic_name_mut(&mut self) -> &mut String {
        &mut self.cinematic_name
    }
    fn cinematic_settings(&self) -> &Option<LockedTypeObject /* CinematicSettings */> {
        &self.cinematic_settings
    }
    fn cinematic_settings_mut(&mut self) -> &mut Option<LockedTypeObject /* CinematicSettings */> {
        &mut self.cinematic_settings
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SnowglobeEntityData {
}

impl super::core::DataContainerTrait for SnowglobeEntityData {
}

pub static SNOWGLOBEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityData",
    name_hash: 486837864,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SnowglobeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeEntityData as Default>::default())),
            create_boxed: || Box::new(<SnowglobeEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PrimeBehavior",
                name_hash: 3892996234,
                flags: MemberInfoFlags::new(0),
                field_type: "PrimeType",
                rust_offset: offset_of!(SnowglobeEntityData, prime_behavior),
            },
            FieldInfoData {
                name: "CleanUpBehavior",
                name_hash: 1900139401,
                flags: MemberInfoFlags::new(0),
                field_type: "CleanUpType",
                rust_offset: offset_of!(SnowglobeEntityData, clean_up_behavior),
            },
            FieldInfoData {
                name: "MultiPlayerBehavior",
                name_hash: 2194322035,
                flags: MemberInfoFlags::new(0),
                field_type: "MultiplayerStartType",
                rust_offset: offset_of!(SnowglobeEntityData, multi_player_behavior),
            },
            FieldInfoData {
                name: "ExternalTime",
                name_hash: 2162678253,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SnowglobeEntityData, external_time),
            },
            FieldInfoData {
                name: "CinematicName",
                name_hash: 4053421489,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SnowglobeEntityData, cinematic_name),
            },
            FieldInfoData {
                name: "CinematicSettings",
                name_hash: 684665523,
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


pub static SNOWGLOBEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityData-Array",
    name_hash: 2369757276,
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
    name_hash: 1008610700,
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


pub static SNOWGLOBEENTITYSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeEntityStatus-Array",
    name_hash: 2630352312,
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
    name_hash: 3952809863,
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


pub static MULTIPLAYERSTARTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MultiplayerStartType-Array",
    name_hash: 3360708659,
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
    name_hash: 1321244282,
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


pub static USERINPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserInputType-Array",
    name_hash: 1461758286,
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
    name_hash: 2232228221,
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


pub static CLEANUPTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CleanUpType-Array",
    name_hash: 2449630793,
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
    name_hash: 2170429118,
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


pub static PRIMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimeType-Array",
    name_hash: 2766360330,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("PrimeType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1948325405,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSnowglobeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSnowglobeEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSnowglobeEntity as Default>::default()),
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


pub static SERVERSNOWGLOBEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSnowglobeEntity-Array",
    name_hash: 500490665,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("ServerSnowglobeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 135563616,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(SnowglobeTrackerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeTrackerEntity as Default>::default())),
            create_boxed: || Box::new(<SnowglobeTrackerEntity as Default>::default()),
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


pub static SNOWGLOBETRACKERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeTrackerEntity-Array",
    name_hash: 1587576660,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeTrackerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4049933708,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(SnowglobeComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnowglobeComponent as Default>::default())),
            create_boxed: || Box::new(<SnowglobeComponent as Default>::default()),
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


pub static SNOWGLOBECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnowglobeComponent-Array",
    name_hash: 1973216696,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("SnowglobeComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4133224025,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(CinematicSettingsSplitterEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicSettingsSplitterEntity as Default>::default())),
            create_boxed: || Box::new(<CinematicSettingsSplitterEntity as Default>::default()),
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


pub static CINEMATICSETTINGSSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicSettingsSplitterEntity-Array",
    name_hash: 3207154797,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("CinematicSettingsSplitterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 88323265,
    flags: MemberInfoFlags::new(101),
    module: "Snowglobe",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSnowglobeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSnowglobeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSnowglobeEntity as Default>::default()),
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


pub static CLIENTSNOWGLOBEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSnowglobeEntity-Array",
    name_hash: 1905114869,
    flags: MemberInfoFlags::new(145),
    module: "Snowglobe",
    data: TypeInfoData::Array("ClientSnowglobeEntity"),
    array_type: None,
    alignment: 8,
};


