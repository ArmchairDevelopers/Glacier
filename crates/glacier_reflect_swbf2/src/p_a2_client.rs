use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct PA2AttitudeEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub attitude: Option<Arc<Mutex<dyn PA2AttitudeAssetTrait>>>,
}

pub trait PA2AttitudeEntityDataTrait: super::entity::EntityDataTrait {
    fn attitude(&self) -> &Option<Arc<Mutex<dyn PA2AttitudeAssetTrait>>>;
}

impl PA2AttitudeEntityDataTrait for PA2AttitudeEntityData {
    fn attitude(&self) -> &Option<Arc<Mutex<dyn PA2AttitudeAssetTrait>>> {
        &self.attitude
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
}

impl super::core::GameDataContainerTrait for PA2AttitudeEntityData {
}

impl super::core::DataContainerTrait for PA2AttitudeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2ATTITUDEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2AttitudeEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Attitude",
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
}


pub static PA2ATTITUDEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2AttitudeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2AttitudeAsset {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub active_game_state: super::ant::AntRef,
    pub primary_target_min_duration: f32,
}

pub trait PA2AttitudeAssetTrait: super::core::DataContainerPolicyAssetTrait {
    fn active_game_state(&self) -> &super::ant::AntRef;
    fn primary_target_min_duration(&self) -> &f32;
}

impl PA2AttitudeAssetTrait for PA2AttitudeAsset {
    fn active_game_state(&self) -> &super::ant::AntRef {
        &self.active_game_state
    }
    fn primary_target_min_duration(&self) -> &f32 {
        &self.primary_target_min_duration
    }
}

impl super::core::DataContainerPolicyAssetTrait for PA2AttitudeAsset {
}

impl super::core::AssetTrait for PA2AttitudeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PA2AttitudeAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2ATTITUDEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeAsset",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2AttitudeAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ActiveGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(PA2AttitudeAsset, active_game_state),
            },
            FieldInfoData {
                name: "PrimaryTargetMinDuration",
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
}


pub static PA2ATTITUDEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2AttitudeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn primary_targeting_mode(&self) -> &PA2TargetingMode;
    fn glance_targeting_mode(&self) -> &PA2TargetingMode;
    fn override_primary_target_enabled(&self) -> &bool;
    fn override_primary_target_position(&self) -> &super::core::Vec3;
}

impl PA2BehaviorEntityDataTrait for PA2BehaviorEntityData {
    fn targeting_bone(&self) -> &super::entity::GameplayBones {
        &self.targeting_bone
    }
    fn primary_targeting_mode(&self) -> &PA2TargetingMode {
        &self.primary_targeting_mode
    }
    fn glance_targeting_mode(&self) -> &PA2TargetingMode {
        &self.glance_targeting_mode
    }
    fn override_primary_target_enabled(&self) -> &bool {
        &self.override_primary_target_enabled
    }
    fn override_primary_target_position(&self) -> &super::core::Vec3 {
        &self.override_primary_target_position
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
}

impl super::core::GameDataContainerTrait for PA2BehaviorEntityData {
}

impl super::core::DataContainerTrait for PA2BehaviorEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2BEHAVIORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BehaviorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2BehaviorEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TargetingBone",
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(PA2BehaviorEntityData, targeting_bone),
            },
            FieldInfoData {
                name: "PrimaryTargetingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "PA2TargetingMode",
                rust_offset: offset_of!(PA2BehaviorEntityData, primary_targeting_mode),
            },
            FieldInfoData {
                name: "GlanceTargetingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "PA2TargetingMode",
                rust_offset: offset_of!(PA2BehaviorEntityData, glance_targeting_mode),
            },
            FieldInfoData {
                name: "OverridePrimaryTargetEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PA2BehaviorEntityData, override_primary_target_enabled),
            },
            FieldInfoData {
                name: "OverridePrimaryTargetPosition",
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
}


pub static PA2BEHAVIORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BehaviorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2BehaviorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn radius(&self) -> &f32;
    fn enabled(&self) -> &bool;
    fn targeting_mode(&self) -> &PA2TargetingMode;
    fn target_type(&self) -> &PA2TargetType;
}

impl PA2TargetEntityDataTrait for PA2TargetEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn targeting_mode(&self) -> &PA2TargetingMode {
        &self.targeting_mode
    }
    fn target_type(&self) -> &PA2TargetType {
        &self.target_type
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
}

impl super::core::GameDataContainerTrait for PA2TargetEntityData {
}

impl super::core::DataContainerTrait for PA2TargetEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PA2TARGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2TargetEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(PA2TargetEntityData, transform),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PA2TargetEntityData, radius),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PA2TargetEntityData, enabled),
            },
            FieldInfoData {
                name: "TargetingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "PA2TargetingMode",
                rust_offset: offset_of!(PA2TargetEntityData, targeting_mode),
            },
            FieldInfoData {
                name: "TargetType",
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
}


pub static PA2TARGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetEntityData-Array",
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
}


pub static PA2TARGETINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetingMode-Array",
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
}


pub static PA2TARGETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetType-Array",
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
}


pub static TARGETFLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetFlags-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("TargetFlags"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PA2TargetState {
}

pub trait PA2TargetStateTrait: TypeObject {
}

impl PA2TargetStateTrait for PA2TargetState {
}

pub static PA2TARGETSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetState",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PA2TargetState as Default>::default())),
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
}


pub static PA2TARGETSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetState-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPA2TargetEntity as Default>::default())),
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
}


pub static CLIENTPA2TARGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2TargetEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPA2BehaviorEntity as Default>::default())),
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
}


pub static CLIENTPA2BEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2BehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2BehaviorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPA2AttitudeEntity as Default>::default())),
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
}


pub static CLIENTPA2ATTITUDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2AttitudeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2AttitudeEntity"),
    array_type: None,
    alignment: 8,
};


