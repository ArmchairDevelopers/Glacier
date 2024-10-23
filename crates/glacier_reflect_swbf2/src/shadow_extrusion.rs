use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_shadow_extrusion_types(registry: &mut TypeRegistry) {
    registry.register_type(SHADOWEXTRUSIONLEVELDATAENTITY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLEVELDATAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONDATAENTITY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONDATAENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionLevelDataEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ShadowExtrusionLevelDataEntityTrait: super::entity::EntityTrait {
}

impl ShadowExtrusionLevelDataEntityTrait for ShadowExtrusionLevelDataEntity {
}

impl super::entity::EntityTrait for ShadowExtrusionLevelDataEntity {
}

impl super::entity::EntityBusPeerTrait for ShadowExtrusionLevelDataEntity {
}

pub static SHADOWEXTRUSIONLEVELDATAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntity",
    name_hash: 2824534155,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusion",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionLevelDataEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionLevelDataEntity as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionLevelDataEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLEVELDATAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowExtrusionLevelDataEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONLEVELDATAENTITY_TYPE_INFO
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


pub static SHADOWEXTRUSIONLEVELDATAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntity-Array",
    name_hash: 3908347711,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusion",
    data: TypeInfoData::Array("ShadowExtrusionLevelDataEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionLightDirectionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ShadowExtrusionLightDirectionEntityTrait: super::entity::EntityTrait {
}

impl ShadowExtrusionLightDirectionEntityTrait for ShadowExtrusionLightDirectionEntity {
}

impl super::entity::EntityTrait for ShadowExtrusionLightDirectionEntity {
}

impl super::entity::EntityBusPeerTrait for ShadowExtrusionLightDirectionEntity {
}

pub static SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntity",
    name_hash: 450399286,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusion",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionLightDirectionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionLightDirectionEntity as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionLightDirectionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowExtrusionLightDirectionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_TYPE_INFO
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


pub static SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntity-Array",
    name_hash: 3092938882,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusion",
    data: TypeInfoData::Array("ShadowExtrusionLightDirectionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionDataEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ShadowExtrusionDataEntityTrait: super::entity::EntityTrait {
}

impl ShadowExtrusionDataEntityTrait for ShadowExtrusionDataEntity {
}

impl super::entity::EntityTrait for ShadowExtrusionDataEntity {
}

impl super::entity::EntityBusPeerTrait for ShadowExtrusionDataEntity {
}

pub static SHADOWEXTRUSIONDATAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntity",
    name_hash: 1926976797,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusion",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionDataEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionDataEntity as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionDataEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONDATAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowExtrusionDataEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONDATAENTITY_TYPE_INFO
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


pub static SHADOWEXTRUSIONDATAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntity-Array",
    name_hash: 1426219177,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusion",
    data: TypeInfoData::Array("ShadowExtrusionDataEntity"),
    array_type: None,
    alignment: 8,
};


