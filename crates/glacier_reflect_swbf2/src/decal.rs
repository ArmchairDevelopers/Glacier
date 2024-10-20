use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_types(registry: &mut TypeRegistry) {
    registry.register_type(DECALDYNAMICSTATE_TYPE_INFO);
    registry.register_type(DECALDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DECALSTATICSTATE_TYPE_INFO);
    registry.register_type(DECALSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DECALLIFETIME_TYPE_INFO);
    registry.register_type(DECALLIFETIME_ARRAY_TYPE_INFO);
    registry.register_type(DECALVOLUMEENTITY_TYPE_INFO);
    registry.register_type(DECALVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DECALENTITY_TYPE_INFO);
    registry.register_type(DECALENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct DecalDynamicState {
    pub is_detached: bool,
    pub field_flag_changed0: u8,
}

pub trait DecalDynamicStateTrait: TypeObject {
    fn is_detached(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl DecalDynamicStateTrait for DecalDynamicState {
    fn is_detached(&self) -> &bool {
        &self.is_detached
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DECALDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "Decal",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsDetached",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalDynamicState, is_detached),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DecalDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DECALDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DecalDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        DECALDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalStaticState {
    pub transform: super::core::LinearTransform,
    pub asset: Option<Arc<Mutex<dyn super::render_base::DecalTemplateBaseAssetTrait>>>,
    pub life_time: DecalLifeTime,
    pub apply_on_terrain: bool,
    pub exclusive_model: super::world_base::ModelHandle,
    pub exclusive_model_part: u32,
    pub ignored_models: Vec<super::world_base::ModelHandle>,
    pub field_flag_changed0: u8,
}

pub trait DecalStaticStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::render_base::DecalTemplateBaseAssetTrait>>>;
    fn life_time(&self) -> &DecalLifeTime;
    fn apply_on_terrain(&self) -> &bool;
    fn exclusive_model(&self) -> &super::world_base::ModelHandle;
    fn exclusive_model_part(&self) -> &u32;
    fn ignored_models(&self) -> &Vec<super::world_base::ModelHandle>;
    fn field_flag_changed0(&self) -> &u8;
}

impl DecalStaticStateTrait for DecalStaticState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::render_base::DecalTemplateBaseAssetTrait>>> {
        &self.asset
    }
    fn life_time(&self) -> &DecalLifeTime {
        &self.life_time
    }
    fn apply_on_terrain(&self) -> &bool {
        &self.apply_on_terrain
    }
    fn exclusive_model(&self) -> &super::world_base::ModelHandle {
        &self.exclusive_model
    }
    fn exclusive_model_part(&self) -> &u32 {
        &self.exclusive_model_part
    }
    fn ignored_models(&self) -> &Vec<super::world_base::ModelHandle> {
        &self.ignored_models
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DECALSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalStaticState",
    flags: MemberInfoFlags::new(73),
    module: "Decal",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(DecalStaticState, transform),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "DecalTemplateBaseAsset",
                rust_offset: offset_of!(DecalStaticState, asset),
            },
            FieldInfoData {
                name: "LifeTime",
                flags: MemberInfoFlags::new(0),
                field_type: "DecalLifeTime",
                rust_offset: offset_of!(DecalStaticState, life_time),
            },
            FieldInfoData {
                name: "ApplyOnTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalStaticState, apply_on_terrain),
            },
            FieldInfoData {
                name: "ExclusiveModel",
                flags: MemberInfoFlags::new(0),
                field_type: "ModelHandle",
                rust_offset: offset_of!(DecalStaticState, exclusive_model),
            },
            FieldInfoData {
                name: "ExclusiveModelPart",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalStaticState, exclusive_model_part),
            },
            FieldInfoData {
                name: "IgnoredModels",
                flags: MemberInfoFlags::new(144),
                field_type: "ModelHandle-Array",
                rust_offset: offset_of!(DecalStaticState, ignored_models),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DecalStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DECALSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DecalStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        DECALSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DecalLifeTime {
    #[default]
    DecalLifeTime_RingBuffer = 0,
    DecalLifeTime_RingBufferRecreate = 1,
}

pub static DECALLIFETIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalLifeTime",
    flags: MemberInfoFlags::new(49429),
    module: "Decal",
    data: TypeInfoData::Enum,
    array_type: Some(DECALLIFETIME_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DecalLifeTime {
    fn type_info(&self) -> &'static TypeInfo {
        DECALLIFETIME_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALLIFETIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalLifeTime-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalLifeTime"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalVolumeEntity {
    pub _glacier_base: super::world_sim::RenderVolumeEntity,
}

pub trait DecalVolumeEntityTrait: super::world_sim::RenderVolumeEntityTrait {
}

impl DecalVolumeEntityTrait for DecalVolumeEntity {
}

impl super::world_sim::RenderVolumeEntityTrait for DecalVolumeEntity {
}

impl super::entity::EntityTrait for DecalVolumeEntity {
}

impl super::entity::EntityBusPeerTrait for DecalVolumeEntity {
}

pub static DECALVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Decal",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::world_sim::RENDERVOLUMEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalVolumeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DECALVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DecalVolumeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DECALVOLUMEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalVolumeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DecalEntityTrait: super::entity::EntityTrait {
}

impl DecalEntityTrait for DecalEntity {
}

impl super::entity::EntityTrait for DecalEntity {
}

impl super::entity::EntityBusPeerTrait for DecalEntity {
}

pub static DECALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntity",
    flags: MemberInfoFlags::new(101),
    module: "Decal",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DECALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DecalEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DECALENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalEntity"),
    array_type: None,
    alignment: 8,
};


