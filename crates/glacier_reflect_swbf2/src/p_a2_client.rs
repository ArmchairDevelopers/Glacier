use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_p_a2_client_types(registry: &mut TypeRegistry) {
    registry.register_type(PA2ATTITUDEENTITYDATA_TYPE_INFO);
    registry.register_type(PA2ATTITUDEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2ATTITUDEASSET_TYPE_INFO);
    registry.register_type(PA2ATTITUDEASSET_ARRAY_TYPE_INFO);
    registry.register_type(PA2BEHAVIORENTITYDATA_TYPE_INFO);
    registry.register_type(PA2BEHAVIORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETENTITYDATA_TYPE_INFO);
    registry.register_type(PA2TARGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETINGMODE_TYPE_INFO);
    registry.register_type(PA2TARGETINGMODE_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETTYPE_TYPE_INFO);
    registry.register_type(PA2TARGETTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TARGETFLAGS_TYPE_INFO);
    registry.register_type(TARGETFLAGS_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETSTATE_TYPE_INFO);
    registry.register_type(PA2TARGETSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPA2TARGETENTITY_TYPE_INFO);
    registry.register_type(CLIENTPA2TARGETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPA2BEHAVIORENTITY_TYPE_INFO);
    registry.register_type(CLIENTPA2BEHAVIORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPA2ATTITUDEENTITY_TYPE_INFO);
    registry.register_type(CLIENTPA2ATTITUDEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PA2AttitudeEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub attitude: Option<LockedTypeObject /* PA2AttitudeAsset */>,
}

pub trait PA2AttitudeEntityDataTrait: super::entity::EntityDataTrait {
    fn attitude(&self) -> &Option<LockedTypeObject /* PA2AttitudeAsset */>;
    fn attitude_mut(&mut self) -> &mut Option<LockedTypeObject /* PA2AttitudeAsset */>;
}

impl PA2AttitudeEntityDataTrait for PA2AttitudeEntityData {
    fn attitude(&self) -> &Option<LockedTypeObject /* PA2AttitudeAsset */> {
        &self.attitude
    }
    fn attitude_mut(&mut self) -> &mut Option<LockedTypeObject /* PA2AttitudeAsset */> {
        &mut self.attitude
    }
}

impl super::entity::EntityDataTrait for PA2AttitudeEntityData {
}

impl super::entity::GameObjectDataTrait for PA2AttitudeEntityData {
}

impl super::core::DataBusPeerTrait for PA2AttitudeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PA2AttitudeEntityData {
}

impl super::core::DataContainerTrait for PA2AttitudeEntityData {
}

pub static PA2ATTITUDEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeEntityData",
    name_hash: 3046377221,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PA2AttitudeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2AttitudeEntityData as Default>::default())),
            create_boxed: || Box::new(<PA2AttitudeEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Attitude",
                name_hash: 464264077,
                flags: MemberInfoFlags::new(0),
                field_type: "PA2AttitudeAsset",
                rust_offset: offset_of!(PA2AttitudeEntityData, attitude),
            },
        ],
    }),
    array_type: Some(PA2ATTITUDEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2AttitudeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PA2ATTITUDEENTITYDATA_TYPE_INFO
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


pub static PA2ATTITUDEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeEntityData-Array",
    name_hash: 2917559985,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2AttitudeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PA2AttitudeAsset {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub active_game_state: super::ant::AntRef,
    pub primary_target_min_duration: f32,
}

pub trait PA2AttitudeAssetTrait: super::core::DataContainerPolicyAssetTrait {
    fn active_game_state(&self) -> &super::ant::AntRef;
    fn active_game_state_mut(&mut self) -> &mut super::ant::AntRef;
    fn primary_target_min_duration(&self) -> &f32;
    fn primary_target_min_duration_mut(&mut self) -> &mut f32;
}

impl PA2AttitudeAssetTrait for PA2AttitudeAsset {
    fn active_game_state(&self) -> &super::ant::AntRef {
        &self.active_game_state
    }
    fn active_game_state_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.active_game_state
    }
    fn primary_target_min_duration(&self) -> &f32 {
        &self.primary_target_min_duration
    }
    fn primary_target_min_duration_mut(&mut self) -> &mut f32 {
        &mut self.primary_target_min_duration
    }
}

impl super::core::DataContainerPolicyAssetTrait for PA2AttitudeAsset {
}

impl super::core::AssetTrait for PA2AttitudeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PA2AttitudeAsset {
}

pub static PA2ATTITUDEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeAsset",
    name_hash: 321590846,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        super_class_offset: offset_of!(PA2AttitudeAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2AttitudeAsset as Default>::default())),
            create_boxed: || Box::new(<PA2AttitudeAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ActiveGameState",
                name_hash: 718650192,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(PA2AttitudeAsset, active_game_state),
            },
            FieldInfoData {
                name: "PrimaryTargetMinDuration",
                name_hash: 1956430828,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PA2AttitudeAsset, primary_target_min_duration),
            },
        ],
    }),
    array_type: Some(PA2ATTITUDEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2AttitudeAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PA2ATTITUDEASSET_TYPE_INFO
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


pub static PA2ATTITUDEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeAsset-Array",
    name_hash: 1298917002,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2AttitudeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PA2BehaviorEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub targeting_bone: super::entity::GameplayBones,
    pub primary_targeting_mode: PA2TargetingMode,
    pub glance_targeting_mode: PA2TargetingMode,
    pub override_primary_target_enabled: bool,
    pub override_primary_target_position: super::core::Vec3,
}

pub trait PA2BehaviorEntityDataTrait: super::entity::EntityDataTrait {
    fn targeting_bone(&self) -> &super::entity::GameplayBones;
    fn targeting_bone_mut(&mut self) -> &mut super::entity::GameplayBones;
    fn primary_targeting_mode(&self) -> &PA2TargetingMode;
    fn primary_targeting_mode_mut(&mut self) -> &mut PA2TargetingMode;
    fn glance_targeting_mode(&self) -> &PA2TargetingMode;
    fn glance_targeting_mode_mut(&mut self) -> &mut PA2TargetingMode;
    fn override_primary_target_enabled(&self) -> &bool;
    fn override_primary_target_enabled_mut(&mut self) -> &mut bool;
    fn override_primary_target_position(&self) -> &super::core::Vec3;
    fn override_primary_target_position_mut(&mut self) -> &mut super::core::Vec3;
}

impl PA2BehaviorEntityDataTrait for PA2BehaviorEntityData {
    fn targeting_bone(&self) -> &super::entity::GameplayBones {
        &self.targeting_bone
    }
    fn targeting_bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.targeting_bone
    }
    fn primary_targeting_mode(&self) -> &PA2TargetingMode {
        &self.primary_targeting_mode
    }
    fn primary_targeting_mode_mut(&mut self) -> &mut PA2TargetingMode {
        &mut self.primary_targeting_mode
    }
    fn glance_targeting_mode(&self) -> &PA2TargetingMode {
        &self.glance_targeting_mode
    }
    fn glance_targeting_mode_mut(&mut self) -> &mut PA2TargetingMode {
        &mut self.glance_targeting_mode
    }
    fn override_primary_target_enabled(&self) -> &bool {
        &self.override_primary_target_enabled
    }
    fn override_primary_target_enabled_mut(&mut self) -> &mut bool {
        &mut self.override_primary_target_enabled
    }
    fn override_primary_target_position(&self) -> &super::core::Vec3 {
        &self.override_primary_target_position
    }
    fn override_primary_target_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.override_primary_target_position
    }
}

impl super::entity::EntityDataTrait for PA2BehaviorEntityData {
}

impl super::entity::GameObjectDataTrait for PA2BehaviorEntityData {
}

impl super::core::DataBusPeerTrait for PA2BehaviorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PA2BehaviorEntityData {
}

impl super::core::DataContainerTrait for PA2BehaviorEntityData {
}

pub static PA2BEHAVIORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BehaviorEntityData",
    name_hash: 325302017,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PA2BehaviorEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2BehaviorEntityData as Default>::default())),
            create_boxed: || Box::new(<PA2BehaviorEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TargetingBone",
                name_hash: 1176503890,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(PA2BehaviorEntityData, targeting_bone),
            },
            FieldInfoData {
                name: "PrimaryTargetingMode",
                name_hash: 1223818619,
                flags: MemberInfoFlags::new(0),
                field_type: "PA2TargetingMode",
                rust_offset: offset_of!(PA2BehaviorEntityData, primary_targeting_mode),
            },
            FieldInfoData {
                name: "GlanceTargetingMode",
                name_hash: 1558585589,
                flags: MemberInfoFlags::new(0),
                field_type: "PA2TargetingMode",
                rust_offset: offset_of!(PA2BehaviorEntityData, glance_targeting_mode),
            },
            FieldInfoData {
                name: "OverridePrimaryTargetEnabled",
                name_hash: 2860883913,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PA2BehaviorEntityData, override_primary_target_enabled),
            },
            FieldInfoData {
                name: "OverridePrimaryTargetPosition",
                name_hash: 4239737365,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PA2BehaviorEntityData, override_primary_target_position),
            },
        ],
    }),
    array_type: Some(PA2BEHAVIORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PA2BehaviorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PA2BEHAVIORENTITYDATA_TYPE_INFO
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


pub static PA2BEHAVIORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BehaviorEntityData-Array",
    name_hash: 3548016821,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2BehaviorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PA2TargetEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub transform: super::core::LinearTransform,
    pub radius: f32,
    pub enabled: bool,
    pub targeting_mode: PA2TargetingMode,
    pub target_type: PA2TargetType,
}

pub trait PA2TargetEntityDataTrait: super::entity::EntityDataTrait {
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn targeting_mode(&self) -> &PA2TargetingMode;
    fn targeting_mode_mut(&mut self) -> &mut PA2TargetingMode;
    fn target_type(&self) -> &PA2TargetType;
    fn target_type_mut(&mut self) -> &mut PA2TargetType;
}

impl PA2TargetEntityDataTrait for PA2TargetEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn targeting_mode(&self) -> &PA2TargetingMode {
        &self.targeting_mode
    }
    fn targeting_mode_mut(&mut self) -> &mut PA2TargetingMode {
        &mut self.targeting_mode
    }
    fn target_type(&self) -> &PA2TargetType {
        &self.target_type
    }
    fn target_type_mut(&mut self) -> &mut PA2TargetType {
        &mut self.target_type
    }
}

impl super::entity::EntityDataTrait for PA2TargetEntityData {
}

impl super::entity::GameObjectDataTrait for PA2TargetEntityData {
}

impl super::core::DataBusPeerTrait for PA2TargetEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PA2TargetEntityData {
}

impl super::core::DataContainerTrait for PA2TargetEntityData {
}

pub static PA2TARGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetEntityData",
    name_hash: 4071689500,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PA2TargetEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2TargetEntityData as Default>::default())),
            create_boxed: || Box::new(<PA2TargetEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PA2TargetEntityData, transform),
            },
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PA2TargetEntityData, radius),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PA2TargetEntityData, enabled),
            },
            FieldInfoData {
                name: "TargetingMode",
                name_hash: 1176670519,
                flags: MemberInfoFlags::new(0),
                field_type: "PA2TargetingMode",
                rust_offset: offset_of!(PA2TargetEntityData, targeting_mode),
            },
            FieldInfoData {
                name: "TargetType",
                name_hash: 328114796,
                flags: MemberInfoFlags::new(0),
                field_type: "PA2TargetType",
                rust_offset: offset_of!(PA2TargetEntityData, target_type),
            },
        ],
    }),
    array_type: Some(PA2TARGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PA2TargetEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PA2TARGETENTITYDATA_TYPE_INFO
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


pub static PA2TARGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetEntityData-Array",
    name_hash: 3229041960,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PA2TargetingMode {
    #[default]
    PA2TargetingMode_General = 0,
    PA2TargetingMode_ConnectedOnly = 1,
    PA2TargetingMode_UnConnectedOnly = 2,
    PA2TargetingMode_Disabled = 3,
}

pub static PA2TARGETINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetingMode",
    name_hash: 1163196724,
    flags: MemberInfoFlags::new(49429),
    module: "PA2Client",
    data: TypeInfoData::Enum,
    array_type: Some(PA2TARGETINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PA2TargetingMode {
    fn type_info(&self) -> &'static TypeInfo {
        PA2TARGETINGMODE_TYPE_INFO
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


pub static PA2TARGETINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetingMode-Array",
    name_hash: 3246142592,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetingMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PA2TargetType {
    #[default]
    PA2TargetType_General = 0,
    PA2TargetType_Primary = 1,
    PA2TargetType_Glance = 2,
}

pub static PA2TARGETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetType",
    name_hash: 592050447,
    flags: MemberInfoFlags::new(49429),
    module: "PA2Client",
    data: TypeInfoData::Enum,
    array_type: Some(PA2TARGETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PA2TargetType {
    fn type_info(&self) -> &'static TypeInfo {
        PA2TARGETTYPE_TYPE_INFO
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


pub static PA2TARGETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetType-Array",
    name_hash: 964801979,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TargetFlags {
    #[default]
    TargetFlags_PrimaryCandidate = 0,
    TargetFlags_GlanceCandidate = 1,
}

pub static TARGETFLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetFlags",
    name_hash: 2253935979,
    flags: MemberInfoFlags::new(49429),
    module: "PA2Client",
    data: TypeInfoData::Enum,
    array_type: Some(TARGETFLAGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TargetFlags {
    fn type_info(&self) -> &'static TypeInfo {
        TARGETFLAGS_TYPE_INFO
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


pub static TARGETFLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetFlags-Array",
    name_hash: 1856267615,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("TargetFlags"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PA2TargetState {
}

pub trait PA2TargetStateTrait: TypeObject {
}

impl PA2TargetStateTrait for PA2TargetState {
}

pub static PA2TARGETSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetState",
    name_hash: 2365438464,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2TargetState as Default>::default())),
            create_boxed: || Box::new(<PA2TargetState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PA2TARGETSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PA2TargetState {
    fn type_info(&self) -> &'static TypeInfo {
        PA2TARGETSTATE_TYPE_INFO
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


pub static PA2TARGETSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetState-Array",
    name_hash: 3414155316,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientPA2TargetEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPA2TargetEntityTrait: super::entity::EntityTrait {
}

impl ClientPA2TargetEntityTrait for ClientPA2TargetEntity {
}

impl super::entity::EntityTrait for ClientPA2TargetEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPA2TargetEntity {
}

pub static CLIENTPA2TARGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetEntity",
    name_hash: 352344309,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientPA2TargetEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPA2TargetEntity as Default>::default())),
            create_boxed: || Box::new(<ClientPA2TargetEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2TARGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2TargetEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPA2TARGETENTITY_TYPE_INFO
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


pub static CLIENTPA2TARGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetEntity-Array",
    name_hash: 3687311809,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2TargetEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientPA2BehaviorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPA2BehaviorEntityTrait: super::entity::EntityTrait {
}

impl ClientPA2BehaviorEntityTrait for ClientPA2BehaviorEntity {
}

impl super::entity::EntityTrait for ClientPA2BehaviorEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPA2BehaviorEntity {
}

pub static CLIENTPA2BEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2BehaviorEntity",
    name_hash: 117527336,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientPA2BehaviorEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPA2BehaviorEntity as Default>::default())),
            create_boxed: || Box::new(<ClientPA2BehaviorEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2BEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2BehaviorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPA2BEHAVIORENTITY_TYPE_INFO
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


pub static CLIENTPA2BEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2BehaviorEntity-Array",
    name_hash: 1597043868,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2BehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientPA2AttitudeEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPA2AttitudeEntityTrait: super::entity::EntityTrait {
}

impl ClientPA2AttitudeEntityTrait for ClientPA2AttitudeEntity {
}

impl super::entity::EntityTrait for ClientPA2AttitudeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPA2AttitudeEntity {
}

pub static CLIENTPA2ATTITUDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2AttitudeEntity",
    name_hash: 3360499628,
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientPA2AttitudeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPA2AttitudeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientPA2AttitudeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2ATTITUDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2AttitudeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPA2ATTITUDEENTITY_TYPE_INFO
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


pub static CLIENTPA2ATTITUDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2AttitudeEntity-Array",
    name_hash: 2238306072,
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2AttitudeEntity"),
    array_type: None,
    alignment: 8,
};


