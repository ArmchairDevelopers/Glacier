use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct DecalDynamicState {
    pub is_detached: bool,
    pub field_flag_changed0: u8,
}

pub trait DecalDynamicStateTrait: TypeObject {
    fn is_detached(&self) -> &bool;
    fn is_detached_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl DecalDynamicStateTrait for DecalDynamicState {
    fn is_detached(&self) -> &bool {
        &self.is_detached
    }
    fn is_detached_mut(&mut self) -> &mut bool {
        &mut self.is_detached
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static DECALDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalDynamicState",
    name_hash: 2131202792,
    flags: MemberInfoFlags::new(36937),
    module: "Decal",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalDynamicState as Default>::default())),
            create_boxed: || Box::new(<DecalDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsDetached",
                name_hash: 1899853473,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalDynamicState, is_detached),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static DECALDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalDynamicState-Array",
    name_hash: 2341733596,
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DecalStaticState {
    pub transform: super::core::LinearTransform,
    pub asset: Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>,
    pub life_time: DecalLifeTime,
    pub apply_on_terrain: bool,
    pub exclusive_model: super::world_base::ModelHandle,
    pub exclusive_model_part: u32,
    pub ignored_models: Vec<BoxedTypeObject /* super::world_base::ModelHandle */>,
    pub field_flag_changed0: u8,
}

pub trait DecalStaticStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn asset(&self) -> &Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>;
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */>;
    fn life_time(&self) -> &DecalLifeTime;
    fn life_time_mut(&mut self) -> &mut DecalLifeTime;
    fn apply_on_terrain(&self) -> &bool;
    fn apply_on_terrain_mut(&mut self) -> &mut bool;
    fn exclusive_model(&self) -> &super::world_base::ModelHandle;
    fn exclusive_model_mut(&mut self) -> &mut super::world_base::ModelHandle;
    fn exclusive_model_part(&self) -> &u32;
    fn exclusive_model_part_mut(&mut self) -> &mut u32;
    fn ignored_models(&self) -> &Vec<BoxedTypeObject /* super::world_base::ModelHandle */>;
    fn ignored_models_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::world_base::ModelHandle */>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl DecalStaticStateTrait for DecalStaticState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn asset(&self) -> &Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::DecalTemplateBaseAsset */> {
        &mut self.asset
    }
    fn life_time(&self) -> &DecalLifeTime {
        &self.life_time
    }
    fn life_time_mut(&mut self) -> &mut DecalLifeTime {
        &mut self.life_time
    }
    fn apply_on_terrain(&self) -> &bool {
        &self.apply_on_terrain
    }
    fn apply_on_terrain_mut(&mut self) -> &mut bool {
        &mut self.apply_on_terrain
    }
    fn exclusive_model(&self) -> &super::world_base::ModelHandle {
        &self.exclusive_model
    }
    fn exclusive_model_mut(&mut self) -> &mut super::world_base::ModelHandle {
        &mut self.exclusive_model
    }
    fn exclusive_model_part(&self) -> &u32 {
        &self.exclusive_model_part
    }
    fn exclusive_model_part_mut(&mut self) -> &mut u32 {
        &mut self.exclusive_model_part
    }
    fn ignored_models(&self) -> &Vec<BoxedTypeObject /* super::world_base::ModelHandle */> {
        &self.ignored_models
    }
    fn ignored_models_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::world_base::ModelHandle */> {
        &mut self.ignored_models
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static DECALSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalStaticState",
    name_hash: 561014437,
    flags: MemberInfoFlags::new(73),
    module: "Decal",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalStaticState as Default>::default())),
            create_boxed: || Box::new(<DecalStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(DecalStaticState, transform),
            },
            FieldInfoData {
                name: "Asset",
                name_hash: 205976053,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalTemplateBaseAsset",
                rust_offset: offset_of!(DecalStaticState, asset),
            },
            FieldInfoData {
                name: "LifeTime",
                name_hash: 2449447350,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalLifeTime",
                rust_offset: offset_of!(DecalStaticState, life_time),
            },
            FieldInfoData {
                name: "ApplyOnTerrain",
                name_hash: 4105242151,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalStaticState, apply_on_terrain),
            },
            FieldInfoData {
                name: "ExclusiveModel",
                name_hash: 562941092,
                flags: MemberInfoFlags::new(0),
                field_type: "ModelHandle",
                rust_offset: offset_of!(DecalStaticState, exclusive_model),
            },
            FieldInfoData {
                name: "ExclusiveModelPart",
                name_hash: 2538982227,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalStaticState, exclusive_model_part),
            },
            FieldInfoData {
                name: "IgnoredModels",
                name_hash: 906617925,
                flags: MemberInfoFlags::new(144),
                field_type: "ModelHandle-Array",
                rust_offset: offset_of!(DecalStaticState, ignored_models),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static DECALSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalStaticState-Array",
    name_hash: 337133329,
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
    name_hash: 1727821337,
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


pub static DECALLIFETIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalLifeTime-Array",
    name_hash: 1048089517,
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalLifeTime"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1010638457,
    flags: MemberInfoFlags::new(101),
    module: "Decal",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::world_sim::RENDERVOLUMEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DecalVolumeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalVolumeEntity as Default>::default())),
            create_boxed: || Box::new(<DecalVolumeEntity as Default>::default()),
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


pub static DECALVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntity-Array",
    name_hash: 2938273101,
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalVolumeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4162963185,
    flags: MemberInfoFlags::new(101),
    module: "Decal",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(DecalEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalEntity as Default>::default())),
            create_boxed: || Box::new(<DecalEntity as Default>::default()),
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


pub static DECALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntity-Array",
    name_hash: 948185541,
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalEntity"),
    array_type: None,
    alignment: 8,
};


