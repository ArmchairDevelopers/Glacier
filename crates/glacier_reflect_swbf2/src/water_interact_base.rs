use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_water_interact_base_types(registry: &mut TypeRegistry) {
    registry.register_type(WATERWAVEHANDLE_TYPE_INFO);
    registry.register_type(WATERWAVEHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(WATERSURFACEHANDLE_TYPE_INFO);
    registry.register_type(WATERSURFACEHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(WATERGLOBALHANDLE_TYPE_INFO);
    registry.register_type(WATERGLOBALHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(WATERSIMULATIONHANDLE_TYPE_INFO);
    registry.register_type(WATERSIMULATIONHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(WATERWAVEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(WATERWAVEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERWAVESTATICSTATE_TYPE_INFO);
    registry.register_type(WATERWAVESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERWAVECREATESTATE_TYPE_INFO);
    registry.register_type(WATERWAVECREATESTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERSURFACEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(WATERSURFACEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERSURFACESTATICSTATE_TYPE_INFO);
    registry.register_type(WATERSURFACESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERSIMULATIONDYNAMICSTATE_TYPE_INFO);
    registry.register_type(WATERSIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERSIMULATIONSTATICSTATE_TYPE_INFO);
    registry.register_type(WATERSIMULATIONSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERGLOBALDYNAMICSTATE_TYPE_INFO);
    registry.register_type(WATERGLOBALDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERGLOBALSTATICSTATE_TYPE_INFO);
    registry.register_type(WATERGLOBALSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERAMBIENTFOAMEFFECT_TYPE_INFO);
    registry.register_type(WATERAMBIENTFOAMEFFECT_ARRAY_TYPE_INFO);
    registry.register_type(WATERSURFACECREATESTATE_TYPE_INFO);
    registry.register_type(WATERSURFACECREATESTATE_ARRAY_TYPE_INFO);
    registry.register_type(WATERDISTURBPARAMS_TYPE_INFO);
    registry.register_type(WATERDISTURBPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(WATERENTITYCLIPINFO_TYPE_INFO);
    registry.register_type(WATERENTITYCLIPINFO_ARRAY_TYPE_INFO);
    registry.register_type(WATERINTERACTLEVELSETTINGS_TYPE_INFO);
    registry.register_type(WATERINTERACTLEVELSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WATERINTERACTSETTINGS_TYPE_INFO);
    registry.register_type(WATERINTERACTSETTINGS_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterWaveHandle {
}

pub const WATERWAVEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(WATERWAVEHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterWaveHandle {
    fn type_info() -> &'static TypeInfo {
        WATERWAVEHANDLE_TYPE_INFO
    }
}


pub const WATERWAVEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterSurfaceHandle {
}

pub const WATERSURFACEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(WATERSURFACEHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterSurfaceHandle {
    fn type_info() -> &'static TypeInfo {
        WATERSURFACEHANDLE_TYPE_INFO
    }
}


pub const WATERSURFACEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterGlobalHandle {
}

pub const WATERGLOBALHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(WATERGLOBALHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterGlobalHandle {
    fn type_info() -> &'static TypeInfo {
        WATERGLOBALHANDLE_TYPE_INFO
    }
}


pub const WATERGLOBALHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterGlobalHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterSimulationHandle {
}

pub const WATERSIMULATIONHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(WATERSIMULATIONHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterSimulationHandle {
    fn type_info() -> &'static TypeInfo {
        WATERSIMULATIONHANDLE_TYPE_INFO
    }
}


pub const WATERSIMULATIONHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSimulationHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterWaveDynamicState {
    pub radius: f32,
    pub amplitude: f32,
    pub field_flag_changed0: u8,
}

pub const WATERWAVEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterWaveDynamicState, radius),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterWaveDynamicState, amplitude),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WaterWaveDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERWAVEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterWaveDynamicState {
    fn type_info() -> &'static TypeInfo {
        WATERWAVEDYNAMICSTATE_TYPE_INFO
    }
}


pub const WATERWAVEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterWaveStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub const WATERWAVESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(WaterWaveStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WaterWaveStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERWAVESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterWaveStaticState {
    fn type_info() -> &'static TypeInfo {
        WATERWAVESTATICSTATE_TYPE_INFO
    }
}


pub const WATERWAVESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterWaveCreateState {
    pub transform: super::core::LinearTransform,
}

pub const WATERWAVECREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveCreateState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(WaterWaveCreateState, transform),
            },
        ],
    }),
    array_type: Some(WATERWAVECREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterWaveCreateState {
    fn type_info() -> &'static TypeInfo {
        WATERWAVECREATESTATE_TYPE_INFO
    }
}


pub const WATERWAVECREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveCreateState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterSurfaceDynamicState {
    pub disturbs: Vec<WaterDisturbParams>,
    pub visible: bool,
    pub tile_offset: super::core::Vec3,
    pub wave_amplitude_scale: f32,
    pub shore_enable: bool,
    pub shore_depth: f32,
    pub field_flag_changed0: u8,
}

pub const WATERSURFACEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Disturbs",
                flags: MemberInfoFlags::new(144),
                field_type: WATERDISTURBPARAMS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceDynamicState, disturbs),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceDynamicState, visible),
            },
            FieldInfoData {
                name: "TileOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceDynamicState, tile_offset),
            },
            FieldInfoData {
                name: "WaveAmplitudeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceDynamicState, wave_amplitude_scale),
            },
            FieldInfoData {
                name: "ShoreEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceDynamicState, shore_enable),
            },
            FieldInfoData {
                name: "ShoreDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceDynamicState, shore_depth),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSURFACEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSurfaceDynamicState {
    fn type_info() -> &'static TypeInfo {
        WATERSURFACEDYNAMICSTATE_TYPE_INFO
    }
}


pub const WATERSURFACEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterSurfaceStaticState {
    pub coarse_simulation: WaterSimulationHandle,
    pub detail_simulation: WaterSimulationHandle,
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub effects: Vec<WaterAmbientFoamEffect>,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub shader_low_detail: super::render_base::SurfaceShaderInstanceDataStruct,
    pub low_detail_distance: super::core::QualityScalableFloat,
    pub projector_elevation: f32,
    pub terrain_virtual_texture_access_enable: bool,
    pub clip_info: WaterEntityClipInfo,
    pub interactive_foam_enable: super::core::QualityScalableBool,
    pub interactive_foam_splat_texture: super::render_base::TextureBaseAsset,
    pub interactive_foam_half_life: f32,
    pub interactive_foam_target_scale: f32,
    pub interactive_foam_splat_interval: f32,
    pub interactive_waves_enable: super::core::QualityScalableBool,
    pub interactive_wave_disturbance_scale: f32,
    pub culling_aabbs: Vec<super::core::AxisAlignedBox>,
    pub field_flag_changed0: u32,
}

pub const WATERSURFACESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CoarseSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: WATERSIMULATIONHANDLE_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, coarse_simulation),
            },
            FieldInfoData {
                name: "DetailSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: WATERSIMULATIONHANDLE_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, detail_simulation),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSFORMSPACEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, transform_space),
            },
            FieldInfoData {
                name: "Effects",
                flags: MemberInfoFlags::new(144),
                field_type: WATERAMBIENTFOAMEFFECT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, effects),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, shader),
            },
            FieldInfoData {
                name: "ShaderLowDetail",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, shader_low_detail),
            },
            FieldInfoData {
                name: "LowDetailDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, low_detail_distance),
            },
            FieldInfoData {
                name: "ProjectorElevation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, projector_elevation),
            },
            FieldInfoData {
                name: "TerrainVirtualTextureAccessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, terrain_virtual_texture_access_enable),
            },
            FieldInfoData {
                name: "ClipInfo",
                flags: MemberInfoFlags::new(0),
                field_type: WATERENTITYCLIPINFO_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, clip_info),
            },
            FieldInfoData {
                name: "InteractiveFoamEnable",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_enable),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_splat_texture),
            },
            FieldInfoData {
                name: "InteractiveFoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_half_life),
            },
            FieldInfoData {
                name: "InteractiveFoamTargetScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_target_scale),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatInterval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_splat_interval),
            },
            FieldInfoData {
                name: "InteractiveWavesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_waves_enable),
            },
            FieldInfoData {
                name: "InteractiveWaveDisturbanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_wave_disturbance_scale),
            },
            FieldInfoData {
                name: "CullingAabbs",
                flags: MemberInfoFlags::new(144),
                field_type: AXISALIGNEDBOX_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, culling_aabbs),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSURFACESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterSurfaceStaticState {
    fn type_info() -> &'static TypeInfo {
        WATERSURFACESTATICSTATE_TYPE_INFO
    }
}


pub const WATERSURFACESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterSimulationDynamicState {
    pub enable: bool,
    pub enable_foam: bool,
    pub choppiness: f32,
    pub field_flag_changed0: u8,
}

pub const WATERSIMULATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationDynamicState, enable),
            },
            FieldInfoData {
                name: "EnableFoam",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationDynamicState, enable_foam),
            },
            FieldInfoData {
                name: "Choppiness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationDynamicState, choppiness),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterSimulationDynamicState {
    fn type_info() -> &'static TypeInfo {
        WATERSIMULATIONDYNAMICSTATE_TYPE_INFO
    }
}


pub const WATERSIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSimulationDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterSimulationStaticState {
    pub resolution: super::core::PlatformScalableInt,
    pub tile_dimension: f32,
    pub physics_simulation_enabled: bool,
    pub force_simple_plane_collision: bool,
    pub wave_amplitude: f32,
    pub wind_speed: f32,
    pub wind_angle: f32,
    pub wind_distribution: super::core::SplineCurve,
    pub min_wavelength: f32,
    pub large_wave_reduction: f32,
    pub foam_half_life: f32,
    pub foam_threshold: f32,
    pub foam_max_value: f32,
    pub ocean_visual_cpu_simulation_enable: bool,
    pub field_flag_changed0: u16,
}

pub const WATERSIMULATIONSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Resolution",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, resolution),
            },
            FieldInfoData {
                name: "TileDimension",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, tile_dimension),
            },
            FieldInfoData {
                name: "PhysicsSimulationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, physics_simulation_enabled),
            },
            FieldInfoData {
                name: "ForceSimplePlaneCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, force_simple_plane_collision),
            },
            FieldInfoData {
                name: "WaveAmplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, wave_amplitude),
            },
            FieldInfoData {
                name: "WindSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, wind_speed),
            },
            FieldInfoData {
                name: "WindAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, wind_angle),
            },
            FieldInfoData {
                name: "WindDistribution",
                flags: MemberInfoFlags::new(0),
                field_type: SPLINECURVE_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, wind_distribution),
            },
            FieldInfoData {
                name: "MinWavelength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, min_wavelength),
            },
            FieldInfoData {
                name: "LargeWaveReduction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, large_wave_reduction),
            },
            FieldInfoData {
                name: "FoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, foam_half_life),
            },
            FieldInfoData {
                name: "FoamThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, foam_threshold),
            },
            FieldInfoData {
                name: "FoamMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, foam_max_value),
            },
            FieldInfoData {
                name: "OceanVisualCpuSimulationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, ocean_visual_cpu_simulation_enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(WaterSimulationStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSIMULATIONSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSimulationStaticState {
    fn type_info() -> &'static TypeInfo {
        WATERSIMULATIONSTATICSTATE_TYPE_INFO
    }
}


pub const WATERSIMULATIONSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSimulationStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterGlobalDynamicState {
    pub ticks: u32,
    pub delta_ticks: u32,
    pub tick_frequency: u32,
    pub current_time: f32,
    pub water_height_sample_debugger_sample_position: super::core::Vec3,
    pub water_height_sample_debugger_sample_position_height: f32,
    pub water_height_sample_debugger_enabled: bool,
    pub water_height_sample_debugger_lock_position_enabled: bool,
    pub field_flag_changed0: u8,
}

pub const WATERGLOBALDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Ticks",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, ticks),
            },
            FieldInfoData {
                name: "DeltaTicks",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, delta_ticks),
            },
            FieldInfoData {
                name: "TickFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, tick_frequency),
            },
            FieldInfoData {
                name: "CurrentTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, current_time),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerSamplePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_sample_position),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerSamplePositionHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_sample_position_height),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_enabled),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerLockPositionEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_lock_position_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(WaterGlobalDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERGLOBALDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterGlobalDynamicState {
    fn type_info() -> &'static TypeInfo {
        WATERGLOBALDYNAMICSTATE_TYPE_INFO
    }
}


pub const WATERGLOBALDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterGlobalDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterGlobalStaticState {
}

pub const WATERGLOBALSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(WATERGLOBALSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaterGlobalStaticState {
    fn type_info() -> &'static TypeInfo {
        WATERGLOBALSTATICSTATE_TYPE_INFO
    }
}


pub const WATERGLOBALSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterGlobalStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterAmbientFoamEffect {
    pub emitters: Vec<super::emitter_base::EmitterBaseAsset>,
    pub threshold: f32,
    pub randomness: f32,
    pub cool_down_time: f32,
    pub near_distance: f32,
    pub far_distance: f32,
    pub vertical_velocity_scale: f32,
    pub horizontal_velocity_scale: f32,
}

pub const WATERAMBIENTFOAMEFFECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffect",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Emitters",
                flags: MemberInfoFlags::new(144),
                field_type: EMITTERBASEASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, emitters),
            },
            FieldInfoData {
                name: "Threshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, threshold),
            },
            FieldInfoData {
                name: "Randomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, randomness),
            },
            FieldInfoData {
                name: "CoolDownTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, cool_down_time),
            },
            FieldInfoData {
                name: "NearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, near_distance),
            },
            FieldInfoData {
                name: "FarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, far_distance),
            },
            FieldInfoData {
                name: "VerticalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, vertical_velocity_scale),
            },
            FieldInfoData {
                name: "HorizontalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffect, horizontal_velocity_scale),
            },
        ],
    }),
    array_type: Some(WATERAMBIENTFOAMEFFECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterAmbientFoamEffect {
    fn type_info() -> &'static TypeInfo {
        WATERAMBIENTFOAMEFFECT_TYPE_INFO
    }
}


pub const WATERAMBIENTFOAMEFFECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffect-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterAmbientFoamEffect-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterSurfaceCreateState {
    pub transform: super::core::LinearTransform,
}

pub const WATERSURFACECREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceCreateState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceCreateState, transform),
            },
        ],
    }),
    array_type: Some(WATERSURFACECREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSurfaceCreateState {
    fn type_info() -> &'static TypeInfo {
        WATERSURFACECREATESTATE_TYPE_INFO
    }
}


pub const WATERSURFACECREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceCreateState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterDisturbParams {
    pub transform: super::core::LinearTransform,
    pub impulse: super::core::Vec3,
}

pub const WATERDISTURBPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDisturbParams",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(WaterDisturbParams, transform),
            },
            FieldInfoData {
                name: "Impulse",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WaterDisturbParams, impulse),
            },
        ],
    }),
    array_type: Some(WATERDISTURBPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterDisturbParams {
    fn type_info() -> &'static TypeInfo {
        WATERDISTURBPARAMS_TYPE_INFO
    }
}


pub const WATERDISTURBPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDisturbParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterDisturbParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterEntityClipInfo {
    pub enable: bool,
    pub clip_face_north: bool,
    pub clip_face_south: bool,
    pub clip_face_east: bool,
    pub clip_face_west: bool,
}

pub const WATERENTITYCLIPINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityClipInfo",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterEntityClipInfo, enable),
            },
            FieldInfoData {
                name: "ClipFaceNorth",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_north),
            },
            FieldInfoData {
                name: "ClipFaceSouth",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_south),
            },
            FieldInfoData {
                name: "ClipFaceEast",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_east),
            },
            FieldInfoData {
                name: "ClipFaceWest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_west),
            },
        ],
    }),
    array_type: Some(WATERENTITYCLIPINFO_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaterEntityClipInfo {
    fn type_info() -> &'static TypeInfo {
        WATERENTITYCLIPINFO_TYPE_INFO
    }
}


pub const WATERENTITYCLIPINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityClipInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterEntityClipInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterInteractLevelSettings {
    pub enabled: bool,
    pub r#override: bool,
    pub max_simulation_count: super::core::PlatformScalableInt,
    pub max_visible_water_surface_count: super::core::PlatformScalableInt,
    pub render_grid_width: super::core::PlatformScalableInt,
    pub render_grid_height: super::core::PlatformScalableInt,
    pub min_ambient_simulation_resolution: super::core::PlatformScalableInt,
    pub max_ambient_simulation_resolution: super::core::PlatformScalableInt,
}

pub const WATERINTERACTLEVELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractLevelSettings",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, enabled),
            },
            FieldInfoData {
                name: "Override",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, r#override),
            },
            FieldInfoData {
                name: "MaxSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, max_simulation_count),
            },
            FieldInfoData {
                name: "MaxVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, max_visible_water_surface_count),
            },
            FieldInfoData {
                name: "RenderGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, render_grid_width),
            },
            FieldInfoData {
                name: "RenderGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, render_grid_height),
            },
            FieldInfoData {
                name: "MinAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, min_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "MaxAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractLevelSettings, max_ambient_simulation_resolution),
            },
        ],
    }),
    array_type: Some(WATERINTERACTLEVELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterInteractLevelSettings {
    fn type_info() -> &'static TypeInfo {
        WATERINTERACTLEVELSETTINGS_TYPE_INFO
    }
}


pub const WATERINTERACTLEVELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractLevelSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterInteractLevelSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterInteractSettings {
    pub enable: bool,
    pub draw_enable: bool,
    pub enable_jobs: bool,
    pub simulation_job_count: u32,
    pub water_quality_level: super::core::QualityLevel,
    pub max_simulation_count: u32,
    pub max_live_editing_simulation_count: u32,
    pub enable_simulation: bool,
    pub enable_disturbs: bool,
    pub interactive_grid_count: u32,
    pub interactive_min_grid_size: u32,
    pub interact_inject_noise_strength: f32,
    pub interact_max_slope: f32,
    pub interact_update_frequency: f32,
    pub min_ambient_simulation_resolution: u32,
    pub max_ambient_simulation_resolution: u32,
    pub render_grid_width: u32,
    pub render_grid_height: u32,
    pub render_fixed_aim_distance: f32,
    pub render_projector_far_plane: f32,
    pub max_visible_water_surface_count: u32,
    pub max_live_editing_visible_water_surface_count: u32,
    pub pc_grid_resolution_multiplier: super::core::QualityScalableFloat,
    pub render_occlusion_cull_enable: bool,
    pub render_occlusion_cull_job_count: u32,
    pub render_occlusion_grid_width: u32,
    pub render_occlusion_grid_height: u32,
    pub render_generate_displacement_mipmaps: bool,
    pub render_generate_gradient_mipmaps: bool,
    pub render_debug_enable: bool,
    pub render_debug_freeze_view_enable: bool,
    pub render_debug_simulation_enable: bool,
    pub render_debug_textures_enable: bool,
    pub draw_update_enable: bool,
    pub virtual_heightfield_atlas_size: i32,
    pub virtual_heightfield_indirection_size: i32,
    pub virtual_heightfield_quantization_range: f32,
}

pub const WATERINTERACTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractSettings",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, enable),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, draw_enable),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, enable_jobs),
            },
            FieldInfoData {
                name: "SimulationJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, simulation_job_count),
            },
            FieldInfoData {
                name: "WaterQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, water_quality_level),
            },
            FieldInfoData {
                name: "MaxSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, max_simulation_count),
            },
            FieldInfoData {
                name: "MaxLiveEditingSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, max_live_editing_simulation_count),
            },
            FieldInfoData {
                name: "EnableSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, enable_simulation),
            },
            FieldInfoData {
                name: "EnableDisturbs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, enable_disturbs),
            },
            FieldInfoData {
                name: "InteractiveGridCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, interactive_grid_count),
            },
            FieldInfoData {
                name: "InteractiveMinGridSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, interactive_min_grid_size),
            },
            FieldInfoData {
                name: "InteractInjectNoiseStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, interact_inject_noise_strength),
            },
            FieldInfoData {
                name: "InteractMaxSlope",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, interact_max_slope),
            },
            FieldInfoData {
                name: "InteractUpdateFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, interact_update_frequency),
            },
            FieldInfoData {
                name: "MinAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, min_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "MaxAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, max_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "RenderGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_grid_width),
            },
            FieldInfoData {
                name: "RenderGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_grid_height),
            },
            FieldInfoData {
                name: "RenderFixedAimDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_fixed_aim_distance),
            },
            FieldInfoData {
                name: "RenderProjectorFarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_projector_far_plane),
            },
            FieldInfoData {
                name: "MaxVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, max_visible_water_surface_count),
            },
            FieldInfoData {
                name: "MaxLiveEditingVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, max_live_editing_visible_water_surface_count),
            },
            FieldInfoData {
                name: "PcGridResolutionMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, pc_grid_resolution_multiplier),
            },
            FieldInfoData {
                name: "RenderOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "RenderOcclusionCullJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_cull_job_count),
            },
            FieldInfoData {
                name: "RenderOcclusionGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_grid_width),
            },
            FieldInfoData {
                name: "RenderOcclusionGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_grid_height),
            },
            FieldInfoData {
                name: "RenderGenerateDisplacementMipmaps",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_generate_displacement_mipmaps),
            },
            FieldInfoData {
                name: "RenderGenerateGradientMipmaps",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_generate_gradient_mipmaps),
            },
            FieldInfoData {
                name: "RenderDebugEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_debug_enable),
            },
            FieldInfoData {
                name: "RenderDebugFreezeViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_debug_freeze_view_enable),
            },
            FieldInfoData {
                name: "RenderDebugSimulationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_debug_simulation_enable),
            },
            FieldInfoData {
                name: "RenderDebugTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, render_debug_textures_enable),
            },
            FieldInfoData {
                name: "DrawUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, draw_update_enable),
            },
            FieldInfoData {
                name: "VirtualHeightfieldAtlasSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, virtual_heightfield_atlas_size),
            },
            FieldInfoData {
                name: "VirtualHeightfieldIndirectionSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, virtual_heightfield_indirection_size),
            },
            FieldInfoData {
                name: "VirtualHeightfieldQuantizationRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractSettings, virtual_heightfield_quantization_range),
            },
        ],
    }),
    array_type: Some(WATERINTERACTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterInteractSettings {
    fn type_info() -> &'static TypeInfo {
        WATERINTERACTSETTINGS_TYPE_INFO
    }
}


pub const WATERINTERACTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterInteractSettings-Array"),
    array_type: None,
    alignment: 8,
};


