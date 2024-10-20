use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_effect_types(registry: &mut TypeRegistry) {
    registry.register_type(VISUALENVIRONMENTEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(VISUALENVIRONMENTEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(SOUNDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDSTATE_TYPE_INFO);
    registry.register_type(SOUNDSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SOUNDSTATICSTATE_TYPE_INFO);
    registry.register_type(SOUNDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(LOCATIONEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(LOCATIONEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCATIONTYPE_TYPE_INFO);
    registry.register_type(LOCATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(LIGHTEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSYSTEMCOMPONENT_TYPE_INFO);
    registry.register_type(EMITTERSYSTEMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMEDATA_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERENTITYDATA_TYPE_INFO);
    registry.register_type(EMITTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHENTITYDATA_TYPE_INFO);
    registry.register_type(EMITTERGRAPHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(EMITTERCHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SPAWNPROBABILITYRANDOMTYPE_TYPE_INFO);
    registry.register_type(SPAWNPROBABILITYRANDOMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMCOMPONENT_TYPE_INFO);
    registry.register_type(EFFECTSYSTEMCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTEFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(BLUEPRINTEFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTASSET_TYPE_INFO);
    registry.register_type(EFFECTASSET_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTENTITYDATA_TYPE_INFO);
    registry.register_type(EFFECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTENTITY_TYPE_INFO);
    registry.register_type(EFFECTENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct VisualEnvironmentEffectEntityData {
    pub visual_environment: super::world_sim::VisualEnvironmentBlueprint,
    pub lifetime: f32,
    pub lifetime_curve: super::core::Vec4,
    pub sample_on_start_only: bool,
    pub cull_angle_curve: super::core::Vec4,
    pub cull_distance: super::core::QualityScalableFloat,
    pub cull_distance_curve: super::core::Vec4,
    pub hide_occluded: bool,
}

pub const VISUALENVIRONMENTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHILDEFFECTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VisualEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: VISUALENVIRONMENTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, visual_environment),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, lifetime),
            },
            FieldInfoData {
                name: "LifetimeCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, lifetime_curve),
            },
            FieldInfoData {
                name: "SampleOnStartOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, sample_on_start_only),
            },
            FieldInfoData {
                name: "CullAngleCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, cull_angle_curve),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, cull_distance),
            },
            FieldInfoData {
                name: "CullDistanceCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, cull_distance_curve),
            },
            FieldInfoData {
                name: "HideOccluded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualEnvironmentEffectEntityData, hide_occluded),
            },
        ],
    }),
    array_type: Some(VISUALENVIRONMENTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VisualEnvironmentEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        VISUALENVIRONMENTEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const VISUALENVIRONMENTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualEnvironmentEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("VisualEnvironmentEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SoundDynamicState {
    pub transform: super::core::LinearTransform,
    pub state: SoundState,
    pub params: super::effect_base::EffectParams,
    pub field_flag_changed0: u8,
}

pub const SOUNDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Effect",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SoundDynamicState, transform),
            },
            FieldInfoData {
                name: "State",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDSTATE_TYPE_INFO,
                rust_offset: offset_of!(SoundDynamicState, state),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMS_TYPE_INFO,
                rust_offset: offset_of!(SoundDynamicState, params),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SoundDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOUNDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SoundDynamicState {
    fn type_info() -> &'static TypeInfo {
        SOUNDDYNAMICSTATE_TYPE_INFO
    }
}


pub const SOUNDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SoundDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SoundState {
    #[default]
    SoundState_Waiting = 0,
    SoundState_Playing = 1,
    SoundState_Stopping = 2,
    SoundState_Dead = 3,
}

pub const SOUNDSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundState",
    flags: MemberInfoFlags::new(49429),
    module: "Effect",
    data: TypeInfoData::Enum,
    array_type: Some(SOUNDSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SoundState {
    fn type_info() -> &'static TypeInfo {
        SOUNDSTATE_TYPE_INFO
    }
}


pub const SOUNDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SoundState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SoundStaticState {
    pub asset: super::audio::SoundAsset,
    pub is_local_player: bool,
    pub is_first_person: bool,
    pub group_guid: super::core::Guid,
    pub max_instance_count_in_group: u32,
    pub kill_on_max_count: bool,
    pub field_flag_changed0: u8,
}

pub const SOUNDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundStaticState",
    flags: MemberInfoFlags::new(73),
    module: "Effect",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(SoundStaticState, asset),
            },
            FieldInfoData {
                name: "IsLocalPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoundStaticState, is_local_player),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoundStaticState, is_first_person),
            },
            FieldInfoData {
                name: "GroupGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(SoundStaticState, group_guid),
            },
            FieldInfoData {
                name: "MaxInstanceCountInGroup",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SoundStaticState, max_instance_count_in_group),
            },
            FieldInfoData {
                name: "KillOnMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SoundStaticState, kill_on_max_count),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(SoundStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SOUNDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SoundStaticState {
    fn type_info() -> &'static TypeInfo {
        SOUNDSTATICSTATE_TYPE_INFO
    }
}


pub const SOUNDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SoundStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SoundStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocationEffectEntityData {
    pub location: LocationType,
    pub ctrl_point_index: u32,
}

pub const LOCATIONEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHILDEFFECTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Location",
                flags: MemberInfoFlags::new(0),
                field_type: LOCATIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(LocationEffectEntityData, location),
            },
            FieldInfoData {
                name: "CtrlPointIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LocationEffectEntityData, ctrl_point_index),
            },
        ],
    }),
    array_type: Some(LOCATIONEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LocationEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCATIONEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const LOCATIONEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("LocationEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocationType {
    #[default]
    LtSource = 0,
    LtTarget = 1,
    LtOther = 2,
    LtCtrlPoint = 3,
}

pub const LOCATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationType",
    flags: MemberInfoFlags::new(49429),
    module: "Effect",
    data: TypeInfoData::Enum,
    array_type: Some(LOCATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocationType {
    fn type_info() -> &'static TypeInfo {
        LOCATIONTYPE_TYPE_INFO
    }
}


pub const LOCATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("LocationType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LightEffectEntityData {
    pub light: super::world_sim::LocalLightEntityData,
    pub lifetime: f32,
    pub looping: bool,
    pub random_intensity_freq: f32,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub local_player_only: bool,
    pub intensity_multiplier: f32,
    pub tube_width: f32,
    pub random_intensity_min: f32,
    pub random_intensity_max: f32,
    pub intensity_curve: super::core::Vec4,
    pub intensity_min: f32,
    pub intensity_max: f32,
}

pub const LIGHTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHILDEFFECTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Light",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALLIGHTENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, light),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, lifetime),
            },
            FieldInfoData {
                name: "Looping",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, looping),
            },
            FieldInfoData {
                name: "RandomIntensityFreq",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, random_intensity_freq),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, spawn_probability),
            },
            FieldInfoData {
                name: "LocalPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, local_player_only),
            },
            FieldInfoData {
                name: "IntensityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, intensity_multiplier),
            },
            FieldInfoData {
                name: "TubeWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, tube_width),
            },
            FieldInfoData {
                name: "RandomIntensityMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, random_intensity_min),
            },
            FieldInfoData {
                name: "RandomIntensityMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, random_intensity_max),
            },
            FieldInfoData {
                name: "IntensityCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, intensity_curve),
            },
            FieldInfoData {
                name: "IntensityMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, intensity_min),
            },
            FieldInfoData {
                name: "IntensityMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LightEffectEntityData, intensity_max),
            },
        ],
    }),
    array_type: Some(LIGHTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LightEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        LIGHTEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const LIGHTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("LightEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterSystemComponent {
    pub exclusion_volumes: super::emitter_base::EmitterExclusionVolumesBaseAsset,
}

pub const EMITTERSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDDATACOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExclusionVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterSystemComponent, exclusion_volumes),
            },
        ],
    }),
    array_type: Some(EMITTERSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterSystemComponent {
    fn type_info() -> &'static TypeInfo {
        EMITTERSYSTEMCOMPONENT_TYPE_INFO
    }
}


pub const EMITTERSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterSystemComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterExclusionVolumeData {
    pub id: u32,
}

pub const EMITTEREXCLUSIONVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBBDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeData, id),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolumeData {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMEDATA_TYPE_INFO
    }
}


pub const EMITTEREXCLUSIONVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterExclusionVolumeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterEntityData {
    pub emitter: super::emitter_base::EmitterBaseAsset,
    pub property_id_lookup_table: Vec<super::emitter_base::PropertyIdLookup>,
}

pub const EMITTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Emitter",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterEntityData, emitter),
            },
            FieldInfoData {
                name: "PropertyIdLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: PROPERTYIDLOOKUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterEntityData, property_id_lookup_table),
            },
        ],
    }),
    array_type: Some(EMITTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterEntityData {
    fn type_info() -> &'static TypeInfo {
        EMITTERENTITYDATA_TYPE_INFO
    }
}


pub const EMITTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterGraphEntityData {
    pub emitter_graph: super::emitter_base::EmitterGraphBaseAsset,
    pub emitter_graph_overrides: super::effect_base::EmitterGraphOverrides,
    pub emitter_graph_params: Vec<super::effect_base::EmitterExposedInput>,
    pub emitter_graph_vertex_shader_textures: Vec<super::effect_base::EmitterExposedTextureInput>,
    pub property_id_lookup_table: Vec<super::emitter_base::PropertyIdLookup>,
}

pub const EMITTERGRAPHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EmitterGraph",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph),
            },
            FieldInfoData {
                name: "EmitterGraphOverrides",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHOVERRIDES_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph_overrides),
            },
            FieldInfoData {
                name: "EmitterGraphParams",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph_params),
            },
            FieldInfoData {
                name: "EmitterGraphVertexShaderTextures",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphEntityData, emitter_graph_vertex_shader_textures),
            },
            FieldInfoData {
                name: "PropertyIdLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: PROPERTYIDLOOKUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterGraphEntityData, property_id_lookup_table),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterGraphEntityData {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHENTITYDATA_TYPE_INFO
    }
}


pub const EMITTERGRAPHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterGraphEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterChildEffectEntityData {
    pub local_player_only: bool,
    pub kill_when_distance_culled: bool,
    pub spawn_outside_view_radius: f32,
    pub spawn_probability_random_type: SpawnProbabilityRandomType,
    pub spawn_probability_range_min: super::core::QualityScalableFloat,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub draw_order_slot: u8,
    pub max_nearby_instance_count: u32,
    pub nearby_radius: f32,
    pub light_probe_sample_method: super::emitter_base::LightProbeSampleMethod,
    pub light_probe_sample_offset_method: super::emitter_base::LightProbeSampleOffsetMethod,
    pub light_probe_sample_offset: super::core::Vec3,
    pub auto_start: bool,
}

pub const EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterChildEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHILDEFFECTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocalPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, local_player_only),
            },
            FieldInfoData {
                name: "KillWhenDistanceCulled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, kill_when_distance_culled),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "SpawnProbabilityRandomType",
                flags: MemberInfoFlags::new(0),
                field_type: SPAWNPROBABILITYRANDOMTYPE_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_probability_random_type),
            },
            FieldInfoData {
                name: "SpawnProbabilityRangeMin",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_probability_range_min),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, spawn_probability),
            },
            FieldInfoData {
                name: "DrawOrderSlot",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, draw_order_slot),
            },
            FieldInfoData {
                name: "MaxNearbyInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, max_nearby_instance_count),
            },
            FieldInfoData {
                name: "NearbyRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, nearby_radius),
            },
            FieldInfoData {
                name: "LightProbeSampleMethod",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTPROBESAMPLEMETHOD_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, light_probe_sample_method),
            },
            FieldInfoData {
                name: "LightProbeSampleOffsetMethod",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, light_probe_sample_offset_method),
            },
            FieldInfoData {
                name: "LightProbeSampleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, light_probe_sample_offset),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterChildEffectEntityData, auto_start),
            },
        ],
    }),
    array_type: Some(EMITTERCHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterChildEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        EMITTERCHILDEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const EMITTERCHILDEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterChildEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EmitterChildEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SpawnProbabilityRandomType {
    #[default]
    SpawnProbabilityRandomType_PerEmitter = 0,
    SpawnProbabilityRandomType_PerEffect = 1,
}

pub const SPAWNPROBABILITYRANDOMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnProbabilityRandomType",
    flags: MemberInfoFlags::new(49429),
    module: "Effect",
    data: TypeInfoData::Enum,
    array_type: Some(SPAWNPROBABILITYRANDOMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SpawnProbabilityRandomType {
    fn type_info() -> &'static TypeInfo {
        SPAWNPROBABILITYRANDOMTYPE_TYPE_INFO
    }
}


pub const SPAWNPROBABILITYRANDOMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpawnProbabilityRandomType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("SpawnProbabilityRandomType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EffectSystemSettings {
    pub effect_quality_level: super::core::QualityLevel,
    pub enter_level_disable_effect_time: f32,
    pub emitter_root_view_duplication_enable: bool,
}

pub const EFFECTSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EffectQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(EffectSystemSettings, effect_quality_level),
            },
            FieldInfoData {
                name: "EnterLevelDisableEffectTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EffectSystemSettings, enter_level_disable_effect_time),
            },
            FieldInfoData {
                name: "EmitterRootViewDuplicationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectSystemSettings, emitter_root_view_duplication_enable),
            },
        ],
    }),
    array_type: Some(EFFECTSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectSystemSettings {
    fn type_info() -> &'static TypeInfo {
        EFFECTSYSTEMSETTINGS_TYPE_INFO
    }
}


pub const EFFECTSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectSystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectSystemComponent {
    pub effect_parameter_list: super::effect_base::EffectParameterList,
}

pub const EFFECTSYSTEMCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemComponent",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUBWORLDDATACOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EffectParameterList",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETERLIST_TYPE_INFO,
                rust_offset: offset_of!(EffectSystemComponent, effect_parameter_list),
            },
        ],
    }),
    array_type: Some(EFFECTSYSTEMCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectSystemComponent {
    fn type_info() -> &'static TypeInfo {
        EFFECTSYSTEMCOMPONENT_TYPE_INFO
    }
}


pub const EFFECTSYSTEMCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectSystemComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectSystemComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BlueprintEffectEntityData {
    pub blueprint: super::entity::Blueprint,
    pub lifetime: f32,
    pub lifetime_curve: super::core::Vec4,
    pub dimmer: f32,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub local_player_only: bool,
}

pub const BLUEPRINTEFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintEffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CHILDEFFECTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Blueprint",
                flags: MemberInfoFlags::new(0),
                field_type: BLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(BlueprintEffectEntityData, blueprint),
            },
            FieldInfoData {
                name: "Lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintEffectEntityData, lifetime),
            },
            FieldInfoData {
                name: "LifetimeCurve",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(BlueprintEffectEntityData, lifetime_curve),
            },
            FieldInfoData {
                name: "Dimmer",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BlueprintEffectEntityData, dimmer),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(BlueprintEffectEntityData, spawn_probability),
            },
            FieldInfoData {
                name: "LocalPlayerOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlueprintEffectEntityData, local_player_only),
            },
        ],
    }),
    array_type: Some(BLUEPRINTEFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BlueprintEffectEntityData {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTEFFECTENTITYDATA_TYPE_INFO
    }
}


pub const BLUEPRINTEFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintEffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("BlueprintEffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectAsset {
}

pub const EFFECTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectAsset",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EFFECTBLUEPRINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EFFECTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectAsset {
    fn type_info() -> &'static TypeInfo {
        EFFECTASSET_TYPE_INFO
    }
}


pub const EFFECTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EffectEntityData {
    pub components: Vec<super::entity::GameObjectData>,
    pub max_active_instance_count: super::core::QualityScalableInt,
    pub cull_distance: super::core::QualityScalableFloat,
    pub kill_by_water: bool,
    pub start_delay: f32,
    pub kill_on_max_count: bool,
    pub attach_to_spawn_surface: bool,
    pub enable: super::core::QualityScalableBool,
    pub override_draw_order: bool,
    pub keep_alive: bool,
}

pub const EFFECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Components",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, components),
            },
            FieldInfoData {
                name: "MaxActiveInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, max_active_instance_count),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, cull_distance),
            },
            FieldInfoData {
                name: "KillByWater",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, kill_by_water),
            },
            FieldInfoData {
                name: "StartDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, start_delay),
            },
            FieldInfoData {
                name: "KillOnMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, kill_on_max_count),
            },
            FieldInfoData {
                name: "AttachToSpawnSurface",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, attach_to_spawn_surface),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, enable),
            },
            FieldInfoData {
                name: "OverrideDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, override_draw_order),
            },
            FieldInfoData {
                name: "KeepAlive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EffectEntityData, keep_alive),
            },
        ],
    }),
    array_type: Some(EFFECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EffectEntityData {
    fn type_info() -> &'static TypeInfo {
        EFFECTENTITYDATA_TYPE_INFO
    }
}


pub const EFFECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectEntity {
}

pub const EFFECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntity",
    flags: MemberInfoFlags::new(101),
    module: "Effect",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EFFECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EffectEntity {
    fn type_info() -> &'static TypeInfo {
        EFFECTENTITY_TYPE_INFO
    }
}


pub const EFFECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Effect",
    data: TypeInfoData::Array("EffectEntity-Array"),
    array_type: None,
    alignment: 8,
};


