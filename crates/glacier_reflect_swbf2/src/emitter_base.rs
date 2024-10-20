use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_emitter_base_types(registry: &mut TypeRegistry) {
    registry.register_type(EMITTERHANDLE_TYPE_INFO);
    registry.register_type(EMITTERHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCREATESTATE_TYPE_INFO);
    registry.register_type(EMITTERCREATESTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERDYNAMICSTATE_TYPE_INFO);
    registry.register_type(EMITTERDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERCONTROLPOINT_TYPE_INFO);
    registry.register_type(EMITTERCONTROLPOINT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSTATE_TYPE_INFO);
    registry.register_type(EMITTERSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERSTATICSTATE_TYPE_INFO);
    registry.register_type(EMITTERSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMESBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERTAG_TYPE_INFO);
    registry.register_type(EMITTERTAG_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHBASEASSET_TYPE_INFO);
    registry.register_type(EMITTERGRAPHBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERBASEASSET_TYPE_INFO);
    registry.register_type(EMITTERBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYIDLOOKUP_TYPE_INFO);
    registry.register_type(PROPERTYIDLOOKUP_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXPOSABLETYPE_TYPE_INFO);
    registry.register_type(EMITTEREXPOSABLETYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMERESULTDATA_TYPE_INFO);
    registry.register_type(EMITTEREXCLUSIONVOLUMERESULTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEOFFSETMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEMETHOD_TYPE_INFO);
    registry.register_type(LIGHTPROBESAMPLEMETHOD_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterHandle {
}

pub const EMITTERHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterHandle",
    flags: MemberInfoFlags::new(73),
    module: "EmitterBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(EMITTERHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EmitterHandle {
    fn type_info() -> &'static TypeInfo {
        EMITTERHANDLE_TYPE_INFO
    }
}


pub const EMITTERHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterCreateState {
    pub transform: super::core::LinearTransform,
}

pub const EMITTERCREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCreateState",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(EmitterCreateState, transform),
            },
        ],
    }),
    array_type: Some(EMITTERCREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterCreateState {
    fn type_info() -> &'static TypeInfo {
        EMITTERCREATESTATE_TYPE_INFO
    }
}


pub const EMITTERCREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterCreateState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterDynamicState {
    pub params: super::effect_base::EffectParams,
    pub effect_transform_space_params: Vec<super::effect_base::EffectTransformSpaceParam>,
    pub state: EmitterState,
    pub source_pt: super::core::Vec3,
    pub target_pt: super::core::Vec3,
    pub other_pt: super::core::Vec3,
    pub effect_position: super::core::Vec3,
    pub light_probe_sample_offset: super::core::Vec3,
    pub ctrl_points: Vec<EmitterControlPoint>,
    pub active_ctrl_point_count: u32,
    pub emitter_exposed_inputs: Vec<super::effect_base::EmitterExposedInput>,
    pub emitter_graph_overrides: super::effect_base::EmitterGraphOverrides,
    pub included_cull_id: super::render_base::CullIdHandle,
    pub excluded_cull_id: super::render_base::CullIdHandle,
    pub field_flag_changed0: u16,
}

pub const EMITTERDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "EmitterBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMS_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, params),
            },
            FieldInfoData {
                name: "EffectTransformSpaceParams",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, effect_transform_space_params),
            },
            FieldInfoData {
                name: "State",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERSTATE_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, state),
            },
            FieldInfoData {
                name: "SourcePt",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, source_pt),
            },
            FieldInfoData {
                name: "TargetPt",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, target_pt),
            },
            FieldInfoData {
                name: "OtherPt",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, other_pt),
            },
            FieldInfoData {
                name: "EffectPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, effect_position),
            },
            FieldInfoData {
                name: "LightProbeSampleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, light_probe_sample_offset),
            },
            FieldInfoData {
                name: "CtrlPoints",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTERCONTROLPOINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, ctrl_points),
            },
            FieldInfoData {
                name: "ActiveCtrlPointCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, active_ctrl_point_count),
            },
            FieldInfoData {
                name: "EmitterExposedInputs",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, emitter_exposed_inputs),
            },
            FieldInfoData {
                name: "EmitterGraphOverrides",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERGRAPHOVERRIDES_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, emitter_graph_overrides),
            },
            FieldInfoData {
                name: "IncludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, included_cull_id),
            },
            FieldInfoData {
                name: "ExcludedCullId",
                flags: MemberInfoFlags::new(0),
                field_type: CULLIDHANDLE_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, excluded_cull_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(EmitterDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterDynamicState {
    fn type_info() -> &'static TypeInfo {
        EMITTERDYNAMICSTATE_TYPE_INFO
    }
}


pub const EMITTERDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterControlPoint {
    pub position: super::core::Vec3,
    pub index: u32,
}

pub const EMITTERCONTROLPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterControlPoint",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterControlPoint, position),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterControlPoint, index),
            },
        ],
    }),
    array_type: Some(EMITTERCONTROLPOINT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterControlPoint {
    fn type_info() -> &'static TypeInfo {
        EMITTERCONTROLPOINT_TYPE_INFO
    }
}


pub const EMITTERCONTROLPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterControlPoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterControlPoint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterState {
    #[default]
    EmitterState_Waiting = 0,
    EmitterState_Playing = 1,
    EmitterState_Stopping = 2,
    EmitterState_Dead = 3,
}

pub const EMITTERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterState",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterState {
    fn type_info() -> &'static TypeInfo {
        EMITTERSTATE_TYPE_INFO
    }
}


pub const EMITTERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub emitter_base_asset: EmitterBaseAsset,
    pub is_emitter_graph: bool,
    pub emitter_graph_base_asset: super::core::Asset,
    pub spawn_probability: super::core::QualityScalableFloat,
    pub simulation_cull_distance: super::core::QualityScalableFloat,
    pub kill_when_distance_culled: bool,
    pub kill_by_water: bool,
    pub spawn_outside_view_radius: f32,
    pub nearby_radius: f32,
    pub max_nearby_instance_count: u32,
    pub override_draw_order: bool,
    pub draw_order_slot: u8,
    pub is_first_person: bool,
    pub cast_shadows: bool,
    pub cast_reflection: bool,
    pub use_lightprobe_visibility: bool,
    pub inherited_velocity_and_start_delta_time: super::core::Vec,
    pub inherited_velocity_enabled: bool,
    pub light_probe_sample_method: LightProbeSampleMethod,
    pub light_probe_sample_offset_method: LightProbeSampleOffsetMethod,
    pub group_guid: super::core::Guid,
    pub max_instance_count_in_group: u32,
    pub kill_on_max_count: bool,
    pub property_id_lookup_table: Vec<PropertyIdLookup>,
    pub emitter_exposed_texture_inputs: Vec<super::effect_base::EmitterExposedTextureInput>,
    pub effect_time_delta_type: u32,
    pub internal_duplication_render_view_id: u16,
    pub field_flag_changed0: u32,
}

pub const EMITTERSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStaticState",
    flags: MemberInfoFlags::new(73),
    module: "EmitterBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, transform_space),
            },
            FieldInfoData {
                name: "EmitterBaseAsset",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, emitter_base_asset),
            },
            FieldInfoData {
                name: "IsEmitterGraph",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, is_emitter_graph),
            },
            FieldInfoData {
                name: "EmitterGraphBaseAsset",
                flags: MemberInfoFlags::new(0),
                field_type: ASSET_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, emitter_graph_base_asset),
            },
            FieldInfoData {
                name: "SpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, spawn_probability),
            },
            FieldInfoData {
                name: "SimulationCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, simulation_cull_distance),
            },
            FieldInfoData {
                name: "KillWhenDistanceCulled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, kill_when_distance_culled),
            },
            FieldInfoData {
                name: "KillByWater",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, kill_by_water),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "NearbyRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, nearby_radius),
            },
            FieldInfoData {
                name: "MaxNearbyInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, max_nearby_instance_count),
            },
            FieldInfoData {
                name: "OverrideDrawOrder",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, override_draw_order),
            },
            FieldInfoData {
                name: "DrawOrderSlot",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, draw_order_slot),
            },
            FieldInfoData {
                name: "IsFirstPerson",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, is_first_person),
            },
            FieldInfoData {
                name: "CastShadows",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, cast_shadows),
            },
            FieldInfoData {
                name: "CastReflection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, cast_reflection),
            },
            FieldInfoData {
                name: "UseLightprobeVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, use_lightprobe_visibility),
            },
            FieldInfoData {
                name: "InheritedVelocityAndStartDeltaTime",
                flags: MemberInfoFlags::new(0),
                field_type: VEC_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, inherited_velocity_and_start_delta_time),
            },
            FieldInfoData {
                name: "InheritedVelocityEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, inherited_velocity_enabled),
            },
            FieldInfoData {
                name: "LightProbeSampleMethod",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTPROBESAMPLEMETHOD_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, light_probe_sample_method),
            },
            FieldInfoData {
                name: "LightProbeSampleOffsetMethod",
                flags: MemberInfoFlags::new(0),
                field_type: LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, light_probe_sample_offset_method),
            },
            FieldInfoData {
                name: "GroupGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, group_guid),
            },
            FieldInfoData {
                name: "MaxInstanceCountInGroup",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, max_instance_count_in_group),
            },
            FieldInfoData {
                name: "KillOnMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, kill_on_max_count),
            },
            FieldInfoData {
                name: "PropertyIdLookupTable",
                flags: MemberInfoFlags::new(144),
                field_type: PROPERTYIDLOOKUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, property_id_lookup_table),
            },
            FieldInfoData {
                name: "EmitterExposedTextureInputs",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, emitter_exposed_texture_inputs),
            },
            FieldInfoData {
                name: "EffectTimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, effect_time_delta_type),
            },
            FieldInfoData {
                name: "InternalDuplicationRenderViewId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, internal_duplication_render_view_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(EMITTERSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterStaticState {
    fn type_info() -> &'static TypeInfo {
        EMITTERSTATICSTATE_TYPE_INFO
    }
}


pub const EMITTERSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterExclusionVolumesBaseAsset {
}

pub const EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMESBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterExclusionVolumesBaseAsset {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMESBASEASSET_TYPE_INFO
    }
}


pub const EMITTEREXCLUSIONVOLUMESBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumesBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterExclusionVolumesBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterTag {
    pub name: String,
}

pub const EMITTERTAG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTag",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EmitterTag, name),
            },
        ],
    }),
    array_type: Some(EMITTERTAG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterTag {
    fn type_info() -> &'static TypeInfo {
        EMITTERTAG_TYPE_INFO
    }
}


pub const EMITTERTAG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterTag-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterTag-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterGraphBaseAsset {
}

pub const EMITTERGRAPHBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EMITTERGRAPHBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterGraphBaseAsset {
    fn type_info() -> &'static TypeInfo {
        EMITTERGRAPHBASEASSET_TYPE_INFO
    }
}


pub const EMITTERGRAPHBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterGraphBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmitterBaseAsset {
}

pub const EMITTERBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EmitterBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EMITTERBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterBaseAsset {
    fn type_info() -> &'static TypeInfo {
        EMITTERBASEASSET_TYPE_INFO
    }
}


pub const EMITTERBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyIdLookup {
    pub field_property_id: i32,
    pub field_property_hash: i32,
    pub emitter_exposable_type: EmitterExposableType,
}

pub const PROPERTYIDLOOKUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyIdLookup",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FieldPropertyId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyIdLookup, field_property_id),
            },
            FieldInfoData {
                name: "FieldPropertyHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyIdLookup, field_property_hash),
            },
            FieldInfoData {
                name: "EmitterExposableType",
                flags: MemberInfoFlags::new(0),
                field_type: EMITTEREXPOSABLETYPE_TYPE_INFO,
                rust_offset: offset_of!(PropertyIdLookup, emitter_exposable_type),
            },
        ],
    }),
    array_type: Some(PROPERTYIDLOOKUP_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PropertyIdLookup {
    fn type_info() -> &'static TypeInfo {
        PROPERTYIDLOOKUP_TYPE_INFO
    }
}


pub const PROPERTYIDLOOKUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyIdLookup-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("PropertyIdLookup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EmitterExposableType {
    #[default]
    EmitterExposableType_Float = 0,
    EmitterExposableType_Vec2 = 1,
    EmitterExposableType_Vec3 = 2,
    EmitterExposableType_Vec4 = 3,
}

pub const EMITTEREXPOSABLETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposableType",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTEREXPOSABLETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterExposableType {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXPOSABLETYPE_TYPE_INFO
    }
}


pub const EMITTEREXPOSABLETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposableType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterExposableType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EmitterExclusionVolumeResultData {
    pub transform: super::core::LinearTransform,
    pub half_extent: super::core::Vec3,
    pub id: u32,
}

pub const EMITTEREXCLUSIONVOLUMERESULTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeResultData",
    flags: MemberInfoFlags::new(36937),
    module: "EmitterBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeResultData, transform),
            },
            FieldInfoData {
                name: "HalfExtent",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeResultData, half_extent),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EmitterExclusionVolumeResultData, id),
            },
        ],
    }),
    array_type: Some(EMITTEREXCLUSIONVOLUMERESULTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExclusionVolumeResultData {
    fn type_info() -> &'static TypeInfo {
        EMITTEREXCLUSIONVOLUMERESULTDATA_TYPE_INFO
    }
}


pub const EMITTEREXCLUSIONVOLUMERESULTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExclusionVolumeResultData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("EmitterExclusionVolumeResultData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LightProbeSampleOffsetMethod {
    #[default]
    LightProbeSampleOffsetMethod_World = 0,
    LightProbeSampleOffsetMethod_Local = 1,
}

pub const LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleOffsetMethod",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTPROBESAMPLEOFFSETMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightProbeSampleOffsetMethod {
    fn type_info() -> &'static TypeInfo {
        LIGHTPROBESAMPLEOFFSETMETHOD_TYPE_INFO
    }
}


pub const LIGHTPROBESAMPLEOFFSETMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleOffsetMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("LightProbeSampleOffsetMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LightProbeSampleMethod {
    #[default]
    LightProbeSampleMethod_BoundingBoxCenter = 0,
    LightProbeSampleMethod_EmitterTransform = 1,
    LightProbeSampleMethod_DefaultLightProbe = 2,
    LightProbeSampleMethod_WorldAbsolute = 3,
}

pub const LIGHTPROBESAMPLEMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleMethod",
    flags: MemberInfoFlags::new(49429),
    module: "EmitterBase",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTPROBESAMPLEMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightProbeSampleMethod {
    fn type_info() -> &'static TypeInfo {
        LIGHTPROBESAMPLEMETHOD_TYPE_INFO
    }
}


pub const LIGHTPROBESAMPLEMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightProbeSampleMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "EmitterBase",
    data: TypeInfoData::Array("LightProbeSampleMethod-Array"),
    array_type: None,
    alignment: 8,
};


