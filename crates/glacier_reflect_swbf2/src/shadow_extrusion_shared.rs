use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_shadow_extrusion_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(SHADOWEXTRUSIONLEVELSETTINGS_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLEVELSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLEVELDATAENTITYDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLEVELDATAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONDATAENTITYDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONDATAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONASSET_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTINSTANCE_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTINSTANCE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct ShadowExtrusionLevelSettings {
    pub dynamic_light_direction: bool,
    pub allow_extrusion_back_face_hit: bool,
    pub allow_full_occluded_back_face_hit: bool,
    pub additional_extrusion_length: f32,
}

pub const SHADOWEXTRUSIONLEVELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelSettings",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDDATACOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DynamicLightDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, dynamic_light_direction),
            },
            FieldInfoData {
                name: "AllowExtrusionBackFaceHit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, allow_extrusion_back_face_hit),
            },
            FieldInfoData {
                name: "AllowFullOccludedBackFaceHit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, allow_full_occluded_back_face_hit),
            },
            FieldInfoData {
                name: "AdditionalExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, additional_extrusion_length),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLEVELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionLevelSettings {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONLEVELSETTINGS_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONLEVELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionLevelSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShadowExtrusionLightDirectionEntityData {
    pub light_direction: super::core::Vec3,
}

pub const SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LightDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionLightDirectionEntityData, light_direction),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShadowExtrusionLightDirectionEntityData {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionLightDirectionEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShadowExtrusionLevelDataEntityData {
    pub extrusion_directions: Vec<super::core::Vec3>,
}

pub const SHADOWEXTRUSIONLEVELDATAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExtrusionDirections",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionLevelDataEntityData, extrusion_directions),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLEVELDATAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionLevelDataEntityData {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONLEVELDATAENTITYDATA_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONLEVELDATAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionLevelDataEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShadowExtrusionDataEntityData {
    pub extrusion_data: ShadowExtrusionAsset,
    pub realm: super::core::Realm,
}

pub const SHADOWEXTRUSIONDATAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExtrusionData",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWEXTRUSIONASSET_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionDataEntityData, extrusion_data),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionDataEntityData, realm),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONDATAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionDataEntityData {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONDATAENTITYDATA_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONDATAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionDataEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShadowExtrusionAsset {
    pub extrusion_data: ShadowExtrusionObjectData,
}

pub const SHADOWEXTRUSIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionAsset",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExtrusionData",
                flags: MemberInfoFlags::new(0),
                field_type: SHADOWEXTRUSIONOBJECTDATA_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionAsset, extrusion_data),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionAsset {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONASSET_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShadowExtrusionObjectData {
    pub object_extrusions: Vec<ShadowExtrusionObjectInstance>,
}

pub const SHADOWEXTRUSIONOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectData",
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ObjectExtrusions",
                flags: MemberInfoFlags::new(144),
                field_type: SHADOWEXTRUSIONOBJECTINSTANCE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionObjectData, object_extrusions),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionObjectData {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONOBJECTDATA_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShadowExtrusionObjectInstance {
    pub guid: super::core::Guid,
    pub extrusion_lengths: Vec<f32>,
}

pub const SHADOWEXTRUSIONOBJECTINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectInstance",
    flags: MemberInfoFlags::new(73),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Guid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionObjectInstance, guid),
            },
            FieldInfoData {
                name: "ExtrusionLengths",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShadowExtrusionObjectInstance, extrusion_lengths),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONOBJECTINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionObjectInstance {
    fn type_info() -> &'static TypeInfo {
        SHADOWEXTRUSIONOBJECTINSTANCE_TYPE_INFO
    }
}


pub const SHADOWEXTRUSIONOBJECTINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionObjectInstance-Array"),
    array_type: None,
    alignment: 8,
};


