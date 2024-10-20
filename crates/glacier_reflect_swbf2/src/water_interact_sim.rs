use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_water_interact_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(WATERHEIGHTENTITYDATA_TYPE_INFO);
    registry.register_type(WATERHEIGHTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATERINTERACTWAVEENTITYDATA_TYPE_INFO);
    registry.register_type(WATERINTERACTWAVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATERINTERACTTURBULENCEDISTURBENTITYDATA_TYPE_INFO);
    registry.register_type(WATERINTERACTTURBULENCEDISTURBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATERINTERACTPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WATERINTERACTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATERSURFACEENTITYDATA_TYPE_INFO);
    registry.register_type(WATERSURFACEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATEROCEANSIMULATIONENTITYDATA_TYPE_INFO);
    registry.register_type(WATEROCEANSIMULATIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WATEREFFECTSETUP_TYPE_INFO);
    registry.register_type(WATEREFFECTSETUP_ARRAY_TYPE_INFO);
    registry.register_type(WATERAMBIENTFOAMEFFECTSPAWNER_TYPE_INFO);
    registry.register_type(WATERAMBIENTFOAMEFFECTSPAWNER_ARRAY_TYPE_INFO);
    registry.register_type(WATERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO);
    registry.register_type(WATERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWATEROCEANSIMULATIONENTITY_TYPE_INFO);
    registry.register_type(SERVERWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATEROCEANSIMULATIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WATEROCEANSIMULATIONENTITY_TYPE_INFO);
    registry.register_type(WATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATERINTERACTWAVEENTITY_TYPE_INFO);
    registry.register_type(CLIENTWATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WATERINTERACTWAVEENTITY_TYPE_INFO);
    registry.register_type(WATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WATERHEIGHTENTITY_TYPE_INFO);
    registry.register_type(WATERHEIGHTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWATERSURFACEENTITY_TYPE_INFO);
    registry.register_type(SERVERWATERSURFACEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATERSURFACEENTITY_TYPE_INFO);
    registry.register_type(CLIENTWATERSURFACEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WATERINTERACTTURBULENCEDISTURBENTITY_TYPE_INFO);
    registry.register_type(WATERINTERACTTURBULENCEDISTURBENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct WaterHeightEntityData {
    pub realm: super::core::Realm,
    pub auto_start: bool,
}

pub const WATERHEIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(WaterHeightEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterHeightEntityData, auto_start),
            },
        ],
    }),
    array_type: Some(WATERHEIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterHeightEntityData {
    fn type_info() -> &'static TypeInfo {
        WATERHEIGHTENTITYDATA_TYPE_INFO
    }
}


pub const WATERHEIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterHeightEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterInteractWaveEntityData {
    pub radius: f32,
    pub amplitude: f32,
}

pub const WATERINTERACTWAVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractWaveEntityData, radius),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractWaveEntityData, amplitude),
            },
        ],
    }),
    array_type: Some(WATERINTERACTWAVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterInteractWaveEntityData {
    fn type_info() -> &'static TypeInfo {
        WATERINTERACTWAVEENTITYDATA_TYPE_INFO
    }
}


pub const WATERINTERACTWAVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractWaveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterInteractTurbulenceDisturbEntityData {
    pub disturb_freq: f32,
    pub area_size_x: f32,
    pub area_size_z: f32,
    pub disturb_size: f32,
    pub disturb_vel: f32,
}

pub const WATERINTERACTTURBULENCEDISTURBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DisturbFreq",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, disturb_freq),
            },
            FieldInfoData {
                name: "AreaSizeX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, area_size_x),
            },
            FieldInfoData {
                name: "AreaSizeZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, area_size_z),
            },
            FieldInfoData {
                name: "DisturbSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, disturb_size),
            },
            FieldInfoData {
                name: "DisturbVel",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, disturb_vel),
            },
        ],
    }),
    array_type: Some(WATERINTERACTTURBULENCEDISTURBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterInteractTurbulenceDisturbEntityData {
    fn type_info() -> &'static TypeInfo {
        WATERINTERACTTURBULENCEDISTURBENTITYDATA_TYPE_INFO
    }
}


pub const WATERINTERACTTURBULENCEDISTURBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractTurbulenceDisturbEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterInteractPhysicsComponentData {
}

pub const WATERINTERACTPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERINTERACTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterInteractPhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        WATERINTERACTPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const WATERINTERACTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractPhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterSurfaceEntityData {
    pub additional_water_depth: f32,
    pub effect_setup: WaterEffectSetup,
    pub material_pair: super::entity::MaterialDecl,
    pub query_box_half_extent: super::core::Vec3,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub shader_low_detail: super::render_base::SurfaceShaderInstanceDataStruct,
    pub low_detail_distance: super::core::QualityScalableFloat,
    pub projector_elevation: f32,
    pub terrain_virtual_texture_access_enable: bool,
    pub clip_info: super::water_interact_base::WaterEntityClipInfo,
    pub interactive_foam_enable: super::core::QualityScalableBool,
    pub interactive_foam_splat_texture: super::render_base::TextureBaseAsset,
    pub interactive_foam_half_life: f32,
    pub interactive_foam_target_scale: f32,
    pub interactive_foam_splat_interval: f32,
    pub interactive_waves_enable: super::core::QualityScalableBool,
    pub interactive_wave_disturbance_scale: f32,
    pub culling_aabbs: Vec<super::core::AxisAlignedBox>,
    pub visible: bool,
    pub tile_offset: super::core::Vec3,
    pub wave_amplitude_scale: f32,
    pub shore_enable: bool,
    pub shore_depth: f32,
}

pub const WATERSURFACEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AdditionalWaterDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, additional_water_depth),
            },
            FieldInfoData {
                name: "EffectSetup",
                flags: MemberInfoFlags::new(0),
                field_type: WATEREFFECTSETUP_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, effect_setup),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, material_pair),
            },
            FieldInfoData {
                name: "QueryBoxHalfExtent",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, query_box_half_extent),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, shader),
            },
            FieldInfoData {
                name: "ShaderLowDetail",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, shader_low_detail),
            },
            FieldInfoData {
                name: "LowDetailDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, low_detail_distance),
            },
            FieldInfoData {
                name: "ProjectorElevation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, projector_elevation),
            },
            FieldInfoData {
                name: "TerrainVirtualTextureAccessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, terrain_virtual_texture_access_enable),
            },
            FieldInfoData {
                name: "ClipInfo",
                flags: MemberInfoFlags::new(0),
                field_type: WATERENTITYCLIPINFO_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, clip_info),
            },
            FieldInfoData {
                name: "InteractiveFoamEnable",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_enable),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_splat_texture),
            },
            FieldInfoData {
                name: "InteractiveFoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_half_life),
            },
            FieldInfoData {
                name: "InteractiveFoamTargetScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_target_scale),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatInterval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_splat_interval),
            },
            FieldInfoData {
                name: "InteractiveWavesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_waves_enable),
            },
            FieldInfoData {
                name: "InteractiveWaveDisturbanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_wave_disturbance_scale),
            },
            FieldInfoData {
                name: "CullingAabbs",
                flags: MemberInfoFlags::new(144),
                field_type: AXISALIGNEDBOX_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, culling_aabbs),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, visible),
            },
            FieldInfoData {
                name: "TileOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, tile_offset),
            },
            FieldInfoData {
                name: "WaveAmplitudeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, wave_amplitude_scale),
            },
            FieldInfoData {
                name: "ShoreEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, shore_enable),
            },
            FieldInfoData {
                name: "ShoreDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterSurfaceEntityData, shore_depth),
            },
        ],
    }),
    array_type: Some(WATERSURFACEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSurfaceEntityData {
    fn type_info() -> &'static TypeInfo {
        WATERSURFACEENTITYDATA_TYPE_INFO
    }
}


pub const WATERSURFACEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterSurfaceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterOceanSimulationEntityData {
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
    pub enable: bool,
    pub enable_foam: bool,
    pub choppiness: f32,
}

pub const WATEROCEANSIMULATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Resolution",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, resolution),
            },
            FieldInfoData {
                name: "TileDimension",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, tile_dimension),
            },
            FieldInfoData {
                name: "PhysicsSimulationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, physics_simulation_enabled),
            },
            FieldInfoData {
                name: "ForceSimplePlaneCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, force_simple_plane_collision),
            },
            FieldInfoData {
                name: "WaveAmplitude",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wave_amplitude),
            },
            FieldInfoData {
                name: "WindSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wind_speed),
            },
            FieldInfoData {
                name: "WindAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wind_angle),
            },
            FieldInfoData {
                name: "WindDistribution",
                flags: MemberInfoFlags::new(0),
                field_type: SPLINECURVE_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wind_distribution),
            },
            FieldInfoData {
                name: "MinWavelength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, min_wavelength),
            },
            FieldInfoData {
                name: "LargeWaveReduction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, large_wave_reduction),
            },
            FieldInfoData {
                name: "FoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, foam_half_life),
            },
            FieldInfoData {
                name: "FoamThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, foam_threshold),
            },
            FieldInfoData {
                name: "FoamMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, foam_max_value),
            },
            FieldInfoData {
                name: "OceanVisualCpuSimulationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, ocean_visual_cpu_simulation_enable),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, enable),
            },
            FieldInfoData {
                name: "EnableFoam",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, enable_foam),
            },
            FieldInfoData {
                name: "Choppiness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterOceanSimulationEntityData, choppiness),
            },
        ],
    }),
    array_type: Some(WATEROCEANSIMULATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterOceanSimulationEntityData {
    fn type_info() -> &'static TypeInfo {
        WATEROCEANSIMULATIONENTITYDATA_TYPE_INFO
    }
}


pub const WATEROCEANSIMULATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterOceanSimulationEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterEffectSetup {
    pub ambient_effects: Vec<WaterAmbientFoamEffectSpawner>,
}

pub const WATEREFFECTSETUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEffectSetup",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AmbientEffects",
                flags: MemberInfoFlags::new(144),
                field_type: WATERAMBIENTFOAMEFFECTSPAWNER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaterEffectSetup, ambient_effects),
            },
        ],
    }),
    array_type: Some(WATEREFFECTSETUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterEffectSetup {
    fn type_info() -> &'static TypeInfo {
        WATEREFFECTSETUP_TYPE_INFO
    }
}


pub const WATEREFFECTSETUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEffectSetup-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterEffectSetup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaterAmbientFoamEffectSpawner {
    pub effect: super::effect_base::EffectBlueprint,
    pub threshold: f32,
    pub randomness: f32,
    pub cool_down_time: f32,
    pub near_distance: f32,
    pub far_distance: f32,
    pub vertical_velocity_scale: f32,
    pub horizontal_velocity_scale: f32,
}

pub const WATERAMBIENTFOAMEFFECTSPAWNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffectSpawner",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractSim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, effect),
            },
            FieldInfoData {
                name: "Threshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, threshold),
            },
            FieldInfoData {
                name: "Randomness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, randomness),
            },
            FieldInfoData {
                name: "CoolDownTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, cool_down_time),
            },
            FieldInfoData {
                name: "NearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, near_distance),
            },
            FieldInfoData {
                name: "FarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, far_distance),
            },
            FieldInfoData {
                name: "VerticalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, vertical_velocity_scale),
            },
            FieldInfoData {
                name: "HorizontalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, horizontal_velocity_scale),
            },
        ],
    }),
    array_type: Some(WATERAMBIENTFOAMEFFECTSPAWNER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterAmbientFoamEffectSpawner {
    fn type_info() -> &'static TypeInfo {
        WATERAMBIENTFOAMEFFECTSPAWNER_TYPE_INFO
    }
}


pub const WATERAMBIENTFOAMEFFECTSPAWNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffectSpawner-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterAmbientFoamEffectSpawner-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterLevelDescriptionComponent {
    pub enabled: bool,
    pub r#override: bool,
    pub max_simulation_count: super::core::PlatformScalableInt,
    pub max_visible_water_surface_count: super::core::PlatformScalableInt,
    pub render_grid_width: super::core::PlatformScalableInt,
    pub render_grid_height: super::core::PlatformScalableInt,
    pub min_ambient_simulation_resolution: super::core::PlatformScalableInt,
    pub max_ambient_simulation_resolution: super::core::PlatformScalableInt,
}

pub const WATERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterLevelDescriptionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, enabled),
            },
            FieldInfoData {
                name: "Override",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, r#override),
            },
            FieldInfoData {
                name: "MaxSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, max_simulation_count),
            },
            FieldInfoData {
                name: "MaxVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, max_visible_water_surface_count),
            },
            FieldInfoData {
                name: "RenderGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, render_grid_width),
            },
            FieldInfoData {
                name: "RenderGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, render_grid_height),
            },
            FieldInfoData {
                name: "MinAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, min_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "MaxAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEINT_TYPE_INFO,
                rust_offset: offset_of!(WaterLevelDescriptionComponent, max_ambient_simulation_resolution),
            },
        ],
    }),
    array_type: Some(WATERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterLevelDescriptionComponent {
    fn type_info() -> &'static TypeInfo {
        WATERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO
    }
}


pub const WATERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterLevelDescriptionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterLevelDescriptionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaterOceanSimulationEntity {
}

pub const SERVERWATEROCEANSIMULATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterOceanSimulationEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATEROCEANSIMULATIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterOceanSimulationEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWATEROCEANSIMULATIONENTITY_TYPE_INFO
    }
}


pub const SERVERWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterOceanSimulationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ServerWaterOceanSimulationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterOceanSimulationEntity {
}

pub const CLIENTWATEROCEANSIMULATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterOceanSimulationEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATEROCEANSIMULATIONENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterOceanSimulationEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATEROCEANSIMULATIONENTITY_TYPE_INFO
    }
}


pub const CLIENTWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterOceanSimulationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterOceanSimulationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterOceanSimulationEntity {
}

pub const WATEROCEANSIMULATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterOceanSimulationEntity {
    fn type_info() -> &'static TypeInfo {
        WATEROCEANSIMULATIONENTITY_TYPE_INFO
    }
}


pub const WATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterOceanSimulationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterInteractWaveEntity {
}

pub const CLIENTWATERINTERACTWAVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractWaveEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATERINTERACTWAVEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterInteractWaveEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATERINTERACTWAVEENTITY_TYPE_INFO
    }
}


pub const CLIENTWATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractWaveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterInteractWaveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterInteractWaveEntity {
}

pub const WATERINTERACTWAVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterInteractWaveEntity {
    fn type_info() -> &'static TypeInfo {
        WATERINTERACTWAVEENTITY_TYPE_INFO
    }
}


pub const WATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractWaveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterHeightEntity {
}

pub const WATERHEIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERHEIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterHeightEntity {
    fn type_info() -> &'static TypeInfo {
        WATERHEIGHTENTITY_TYPE_INFO
    }
}


pub const WATERHEIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterHeightEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaterSurfaceEntity {
}

pub const SERVERWATERSURFACEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterSurfaceEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERSURFACEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterSurfaceEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERWATERSURFACEENTITY_TYPE_INFO
    }
}


pub const SERVERWATERSURFACEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterSurfaceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ServerWaterSurfaceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWaterInteractPhysicsComponent {
}

pub const SERVERWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterInteractPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterInteractPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterInteractPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ServerWaterInteractPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterSurfaceEntity {
}

pub const CLIENTWATERSURFACEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterSurfaceEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTPHYSICSENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERSURFACEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterSurfaceEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATERSURFACEENTITY_TYPE_INFO
    }
}


pub const CLIENTWATERSURFACEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterSurfaceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterSurfaceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWaterInteractPhysicsComponent {
}

pub const CLIENTWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PHYSICSCOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterInteractPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterInteractPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterInteractTurbulenceDisturbEntity {
}

pub const WATERINTERACTTURBULENCEDISTURBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERINTERACTTURBULENCEDISTURBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterInteractTurbulenceDisturbEntity {
    fn type_info() -> &'static TypeInfo {
        WATERINTERACTTURBULENCEDISTURBENTITY_TYPE_INFO
    }
}


pub const WATERINTERACTTURBULENCEDISTURBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractTurbulenceDisturbEntity-Array"),
    array_type: None,
    alignment: 8,
};


