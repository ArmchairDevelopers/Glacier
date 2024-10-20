use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShadowExtrusionLevelDataEntity {
}

pub const SHADOWEXTRUSIONLEVELDATAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntity",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusion",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLEVELDATAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowExtrusionLevelDataEntity {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONLEVELDATAENTITY_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONLEVELDATAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusion",
    data: TypeInfoData::Array("ShadowExtrusionLevelDataEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShadowExtrusionLightDirectionEntity {
}

pub const SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusion",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowExtrusionLightDirectionEntity {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONLIGHTDIRECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusion",
    data: TypeInfoData::Array("ShadowExtrusionLightDirectionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShadowExtrusionDataEntity {
}

pub const SHADOWEXTRUSIONDATAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntity",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusion",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONDATAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShadowExtrusionDataEntity {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONDATAENTITY_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONDATAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusion",
    data: TypeInfoData::Array("ShadowExtrusionDataEntity-Array"),
    array_type: None,
    alignment: 8,
};


