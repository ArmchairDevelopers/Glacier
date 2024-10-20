use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_effect_base_types(registry: &mut TypeRegistry) {
    registry.register_type(EFFECTREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(EFFECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTBLUEPRINT_TYPE_INFO);
    registry.register_type(EFFECTBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMETER_TYPE_INFO);
    registry.register_type(EMITTERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKBASEASSET_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERBASEASSET_TYPE_INFO);
    registry.register_type(MESHEMITTERBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTTRANSFORMSPACEPARAM_TYPE_INFO);
    registry.register_type(EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMS_TYPE_INFO);
    registry.register_type(EFFECTPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERLIST_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERLIST_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETER_TYPE_INFO);
    registry.register_type(EFFECTPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERSCOPETYPE_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERSCOPETYPE_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERTYPE_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHPARAMTYPE_TYPE_INFO);
    registry.register_type(EMITTERGRAPHPARAMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDTEXTUREINPUT_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDINPUT_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHOVERRIDES_TYPE_INFO);
    registry.register_type(EMITTERGRAPHOVERRIDES_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTHANDLE_TYPE_INFO);
    registry.register_type(EFFECTHANDLE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct EffectReferenceObjectData {
    pub auto_start: bool,
    pub effect_parameters: Vec<EffectParameter>,
    pub affected_by_lightprobe_visibility: bool,
}

pub const EFFECTREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectReferenceObjectData, auto_start),
            },
            FieldInfoData {
                name: "EffectParameters",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EffectReferenceObjectData, effect_parameters),
            },
            FieldInfoData {
                name: "AffectedByLightprobeVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectReferenceObjectData, affected_by_lightprobe_visibility),
            },
        ],
    }),
    array_type: Some(EFFECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EffectReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        EFFECTREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const EFFECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectBlueprint {
    pub time_delta_type: super::entity::TimeDeltaType,
    pub is_simple: bool,
}

pub const EFFECTBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTBLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(EffectBlueprint, time_delta_type),
            },
            FieldInfoData {
                name: "IsSimple",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectBlueprint, is_simple),
            },
        ],
    }),
    array_type: Some(EFFECTBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectBlueprint {
    fn type_info() -> &'static TypeInfo {
        EFFECTBLUEPRINT_TYPE_INFO
    }
}


pub const EFFECTBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterParameter {
    #[default]
    EmitterParameterNone = 0,
    EmitterParameter1 = 1,
    EmitterParameter2 = 2,
    EmitterParameter3 = 3,
    EmitterParameterVec = 5,
    EmitterParameterVecAverage = 7,
    EmitterParameterDistance = 6,
    EmitterParameterCount = 8,
    EmitterParameter4 = 4,
}

pub const EMITTERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParameter",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterParameter {
    fn type_info() -> &'static TypeInfo {
        EMITTERPARAMETER_TYPE_INFO
    }
}


pub const EMITTERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshEmitterMaskBaseAsset {
}

pub const MESHEMITTERMASKBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERMASKBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterMaskBaseAsset {
    fn type_info() -> &'static TypeInfo {
        MESHEMITTERMASKBASEASSET_TYPE_INFO
    }
}


pub const MESHEMITTERMASKBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("MeshEmitterMaskBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshEmitterBaseAsset {
}

pub const MESHEMITTERBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterBaseAsset {
    fn type_info() -> &'static TypeInfo {
        MESHEMITTERBASEASSET_TYPE_INFO
    }
}


pub const MESHEMITTERBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("MeshEmitterBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectTransformSpaceParam {
    pub index: u32,
    pub transform_space: super::state_stream::TransformSpaceHandle,
}

pub const EFFECTTRANSFORMSPACEPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectTransformSpaceParam",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EffectTransformSpaceParam, index),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(EffectTransformSpaceParam, transform_space),
            },
        ],
    }),
    array_type: Some(EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EffectTransformSpaceParam {
    fn type_info() -> &'static TypeInfo {
        EFFECTTRANSFORMSPACEPARAM_TYPE_INFO
    }
}


pub const EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectTransformSpaceParam-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectTransformSpaceParam-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectParams {
}

pub const EFFECTPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParams",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(EFFECTPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EffectParams {
    fn type_info() -> &'static TypeInfo {
        EFFECTPARAMS_TYPE_INFO
    }
}


pub const EFFECTPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectParameterList {
    pub parameters: Vec<EffectParameter>,
}

pub const EFFECTPARAMETERLIST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterList",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Parameters",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EffectParameterList, parameters),
            },
        ],
    }),
    array_type: Some(EFFECTPARAMETERLIST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectParameterList {
    fn type_info() -> &'static TypeInfo {
        EFFECTPARAMETERLIST_TYPE_INFO
    }
}


pub const EFFECTPARAMETERLIST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterList-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameterList-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectParameter {
    pub name: String,
    pub param_type: EffectParameterType,
    pub param_scope: EffectParameterScopeType,
}

pub const EFFECTPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameter",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EffectParameter, name),
            },
            FieldInfoData {
                name: "ParamType",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(EffectParameter, param_type),
            },
            FieldInfoData {
                name: "ParamScope",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETERSCOPETYPE_TYPE_INFO,
                rust_offset: offset_of!(EffectParameter, param_scope),
            },
        ],
    }),
    array_type: Some(EFFECTPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectParameter {
    fn type_info() -> &'static TypeInfo {
        EFFECTPARAMETER_TYPE_INFO
    }
}


pub const EFFECTPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EffectParameterScopeType {
    #[default]
    EffectParameterScopeType_Local = 0,
    EffectParameterScopeType_Gobal = 1,
}

pub const EFFECTPARAMETERSCOPETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterScopeType",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EFFECTPARAMETERSCOPETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EffectParameterScopeType {
    fn type_info() -> &'static TypeInfo {
        EFFECTPARAMETERSCOPETYPE_TYPE_INFO
    }
}


pub const EFFECTPARAMETERSCOPETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterScopeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameterScopeType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EffectParameterType {
    #[default]
    EffectParameterType_Float = 0,
    EffectParameterType_Vec3 = 1,
    EffectParameterType_Bool = 2,
    EffectParameterType_Int = 3,
    EffectParameterType_MeshEmitter = 4,
    EffectParameterType_MeshEmitterMask = 5,
    EffectParameterType_SpaceAsPosition = 6,
}

pub const EFFECTPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EFFECTPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EffectParameterType {
    fn type_info() -> &'static TypeInfo {
        EFFECTPARAMETERTYPE_TYPE_INFO
    }
}


pub const EFFECTPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterGraphParamType {
    #[default]
    EmitterGraphParamType_Constant = 0,
    EmitterGraphParamType_Schematics = 1,
}

pub const EMITTERGRAPHPARAMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParamType",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHPARAMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphParamType {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHPARAMTYPE_TYPE_INFO
    }
}


pub const EMITTERGRAPHPARAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParamType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterGraphParamType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterExposedTextureInput {
    pub shader_parameter_handle: u32,
    pub texture: super::core::Asset,
}

pub const EMITTEREXPOSEDTEXTUREINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedTextureInput",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ShaderParameterHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterExposedTextureInput, shader_parameter_handle),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: ASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterExposedTextureInput, texture),
            },
        ],
    }),
    array_type: Some(EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterExposedTextureInput {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXPOSEDTEXTUREINPUT_TYPE_INFO
    }
}


pub const EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedTextureInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterExposedTextureInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterExposedInput {
    pub property_id: i32,
    pub value: super::core::Vec4,
}

pub const EMITTEREXPOSEDINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedInput",
    flags: MemberInfoFlags::new(32841),
    module: "EffectBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PropertyId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterExposedInput, property_id),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(EmitterExposedInput, value),
            },
        ],
    }),
    array_type: Some(EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExposedInput {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXPOSEDINPUT_TYPE_INFO
    }
}


pub const EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterExposedInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterGraphOverrides {
    pub spawn_rate: super::core::QualityScalableFloat,
    pub particle_max_count: super::core::QualityScalableInt,
    pub particle_life_span: super::core::QualityScalableFloat,
    pub emitter_life_span: super::core::QualityScalableFloat,
    pub bounding_box_min: super::core::Vec3,
    pub bounding_box_max: super::core::Vec3,
    pub emitter_min_spawn_distance: super::core::QualityScalableFloat,
    pub emitter_max_spawn_distance: super::core::QualityScalableFloat,
    pub spawn_outside_view_radius: f32,
    pub is_spawn_rate_override_set: bool,
    pub is_particle_max_count_override_set: bool,
    pub is_particle_life_span_override_set: bool,
    pub is_emitter_life_span_override_set: bool,
    pub is_bounding_box_min_set: bool,
    pub is_bounding_box_max_set: bool,
    pub is_emitter_min_spawn_distance_override_set: bool,
    pub is_emitter_max_spawn_distance_override_set: bool,
    pub is_spawn_outside_view_radius_override_set: bool,
}

pub const EMITTERGRAPHOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphOverrides",
    flags: MemberInfoFlags::new(32841),
    module: "EffectBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, spawn_rate),
            },
            FieldInfoData {
                name: "ParticleMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, particle_max_count),
            },
            FieldInfoData {
                name: "ParticleLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, particle_life_span),
            },
            FieldInfoData {
                name: "EmitterLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_life_span),
            },
            FieldInfoData {
                name: "BoundingBoxMin",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, bounding_box_min),
            },
            FieldInfoData {
                name: "BoundingBoxMax",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, bounding_box_max),
            },
            FieldInfoData {
                name: "EmitterMinSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_min_spawn_distance),
            },
            FieldInfoData {
                name: "EmitterMaxSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_max_spawn_distance),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "IsSpawnRateOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_spawn_rate_override_set),
            },
            FieldInfoData {
                name: "IsParticleMaxCountOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_particle_max_count_override_set),
            },
            FieldInfoData {
                name: "IsParticleLifeSpanOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_particle_life_span_override_set),
            },
            FieldInfoData {
                name: "IsEmitterLifeSpanOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_life_span_override_set),
            },
            FieldInfoData {
                name: "IsBoundingBoxMinSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_bounding_box_min_set),
            },
            FieldInfoData {
                name: "IsBoundingBoxMaxSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_bounding_box_max_set),
            },
            FieldInfoData {
                name: "IsEmitterMinSpawnDistanceOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_min_spawn_distance_override_set),
            },
            FieldInfoData {
                name: "IsEmitterMaxSpawnDistanceOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_max_spawn_distance_override_set),
            },
            FieldInfoData {
                name: "IsSpawnOutsideViewRadiusOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphOverrides, is_spawn_outside_view_radius_override_set),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHOVERRIDES_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterGraphOverrides {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHOVERRIDES_TYPE_INFO
    }
}


pub const EMITTERGRAPHOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphOverrides-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterGraphOverrides-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectHandle {
}

pub const EFFECTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectHandle",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(EFFECTHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectHandle {
    fn type_info() -> &'static TypeInfo {
        EFFECTHANDLE_TYPE_INFO
    }
}


pub const EFFECTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectHandle-Array"),
    array_type: None,
    alignment: 8,
};


