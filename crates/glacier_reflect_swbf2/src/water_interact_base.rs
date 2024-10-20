use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct WaterWaveHandle {
}

pub trait WaterWaveHandleTrait: TypeObject {
}

impl WaterWaveHandleTrait for WaterWaveHandle {
}

pub static WATERWAVEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterWaveHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERWAVEHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterWaveHandle {
    fn type_info(&self) -> &'static TypeInfo {
        WATERWAVEHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERWAVEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterSurfaceHandle {
}

pub trait WaterSurfaceHandleTrait: TypeObject {
}

impl WaterSurfaceHandleTrait for WaterSurfaceHandle {
}

pub static WATERSURFACEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSurfaceHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERSURFACEHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterSurfaceHandle {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSURFACEHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSURFACEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterGlobalHandle {
}

pub trait WaterGlobalHandleTrait: TypeObject {
}

impl WaterGlobalHandleTrait for WaterGlobalHandle {
}

pub static WATERGLOBALHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterGlobalHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERGLOBALHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterGlobalHandle {
    fn type_info(&self) -> &'static TypeInfo {
        WATERGLOBALHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERGLOBALHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterGlobalHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterSimulationHandle {
}

pub trait WaterSimulationHandleTrait: TypeObject {
}

impl WaterSimulationHandleTrait for WaterSimulationHandle {
}

pub static WATERSIMULATIONHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationHandle",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSimulationHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERSIMULATIONHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterSimulationHandle {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSIMULATIONHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSIMULATIONHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSimulationHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterWaveDynamicState {
    pub radius: f32,
    pub amplitude: f32,
    pub field_flag_changed0: u8,
}

pub trait WaterWaveDynamicStateTrait: TypeObject {
    fn radius(&self) -> &f32;
    fn amplitude(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl WaterWaveDynamicStateTrait for WaterWaveDynamicState {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn amplitude(&self) -> &f32 {
        &self.amplitude
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static WATERWAVEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterWaveDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterWaveDynamicState, radius),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterWaveDynamicState, amplitude),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(WaterWaveDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERWAVEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterWaveDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERWAVEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERWAVEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterWaveStaticState {
    pub transform_space: super::state_stream::TransformSpaceHandle,
    pub field_flag_changed0: u8,
}

pub trait WaterWaveStaticStateTrait: TypeObject {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn field_flag_changed0(&self) -> &u8;
}

impl WaterWaveStaticStateTrait for WaterWaveStaticState {
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static WATERWAVESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterWaveStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(WaterWaveStaticState, transform_space),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(WaterWaveStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERWAVESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterWaveStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERWAVESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERWAVESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterWaveCreateState {
    pub transform: super::core::LinearTransform,
}

pub trait WaterWaveCreateStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
}

impl WaterWaveCreateStateTrait for WaterWaveCreateState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
}

pub static WATERWAVECREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveCreateState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterWaveCreateState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(WaterWaveCreateState, transform),
            },
        ],
    }),
    array_type: Some(WATERWAVECREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterWaveCreateState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERWAVECREATESTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERWAVECREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterWaveCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterWaveCreateState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterSurfaceDynamicState {
    pub disturbs: Vec<WaterDisturbParams>,
    pub visible: bool,
    pub tile_offset: super::core::Vec3,
    pub wave_amplitude_scale: f32,
    pub shore_enable: bool,
    pub shore_depth: f32,
    pub field_flag_changed0: u8,
}

pub trait WaterSurfaceDynamicStateTrait: TypeObject {
    fn disturbs(&self) -> &Vec<WaterDisturbParams>;
    fn visible(&self) -> &bool;
    fn tile_offset(&self) -> &super::core::Vec3;
    fn wave_amplitude_scale(&self) -> &f32;
    fn shore_enable(&self) -> &bool;
    fn shore_depth(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl WaterSurfaceDynamicStateTrait for WaterSurfaceDynamicState {
    fn disturbs(&self) -> &Vec<WaterDisturbParams> {
        &self.disturbs
    }
    fn visible(&self) -> &bool {
        &self.visible
    }
    fn tile_offset(&self) -> &super::core::Vec3 {
        &self.tile_offset
    }
    fn wave_amplitude_scale(&self) -> &f32 {
        &self.wave_amplitude_scale
    }
    fn shore_enable(&self) -> &bool {
        &self.shore_enable
    }
    fn shore_depth(&self) -> &f32 {
        &self.shore_depth
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static WATERSURFACEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSurfaceDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Disturbs",
                flags: MemberInfoFlags::new(144),
                field_type: "WaterDisturbParams-Array",
                rust_offset: offset_of!(WaterSurfaceDynamicState, disturbs),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSurfaceDynamicState, visible),
            },
            FieldInfoData {
                name: "TileOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WaterSurfaceDynamicState, tile_offset),
            },
            FieldInfoData {
                name: "WaveAmplitudeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceDynamicState, wave_amplitude_scale),
            },
            FieldInfoData {
                name: "ShoreEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSurfaceDynamicState, shore_enable),
            },
            FieldInfoData {
                name: "ShoreDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceDynamicState, shore_depth),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(WaterSurfaceDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSURFACEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSurfaceDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSURFACEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSURFACEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    pub interactive_foam_splat_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub interactive_foam_half_life: f32,
    pub interactive_foam_target_scale: f32,
    pub interactive_foam_splat_interval: f32,
    pub interactive_waves_enable: super::core::QualityScalableBool,
    pub interactive_wave_disturbance_scale: f32,
    pub culling_aabbs: Vec<super::core::AxisAlignedBox>,
    pub field_flag_changed0: u32,
}

pub trait WaterSurfaceStaticStateTrait: TypeObject {
    fn coarse_simulation(&self) -> &WaterSimulationHandle;
    fn detail_simulation(&self) -> &WaterSimulationHandle;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn effects(&self) -> &Vec<WaterAmbientFoamEffect>;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn shader_low_detail(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn low_detail_distance(&self) -> &super::core::QualityScalableFloat;
    fn projector_elevation(&self) -> &f32;
    fn terrain_virtual_texture_access_enable(&self) -> &bool;
    fn clip_info(&self) -> &WaterEntityClipInfo;
    fn interactive_foam_enable(&self) -> &super::core::QualityScalableBool;
    fn interactive_foam_splat_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn interactive_foam_half_life(&self) -> &f32;
    fn interactive_foam_target_scale(&self) -> &f32;
    fn interactive_foam_splat_interval(&self) -> &f32;
    fn interactive_waves_enable(&self) -> &super::core::QualityScalableBool;
    fn interactive_wave_disturbance_scale(&self) -> &f32;
    fn culling_aabbs(&self) -> &Vec<super::core::AxisAlignedBox>;
    fn field_flag_changed0(&self) -> &u32;
}

impl WaterSurfaceStaticStateTrait for WaterSurfaceStaticState {
    fn coarse_simulation(&self) -> &WaterSimulationHandle {
        &self.coarse_simulation
    }
    fn detail_simulation(&self) -> &WaterSimulationHandle {
        &self.detail_simulation
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn effects(&self) -> &Vec<WaterAmbientFoamEffect> {
        &self.effects
    }
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn shader_low_detail(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader_low_detail
    }
    fn low_detail_distance(&self) -> &super::core::QualityScalableFloat {
        &self.low_detail_distance
    }
    fn projector_elevation(&self) -> &f32 {
        &self.projector_elevation
    }
    fn terrain_virtual_texture_access_enable(&self) -> &bool {
        &self.terrain_virtual_texture_access_enable
    }
    fn clip_info(&self) -> &WaterEntityClipInfo {
        &self.clip_info
    }
    fn interactive_foam_enable(&self) -> &super::core::QualityScalableBool {
        &self.interactive_foam_enable
    }
    fn interactive_foam_splat_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.interactive_foam_splat_texture
    }
    fn interactive_foam_half_life(&self) -> &f32 {
        &self.interactive_foam_half_life
    }
    fn interactive_foam_target_scale(&self) -> &f32 {
        &self.interactive_foam_target_scale
    }
    fn interactive_foam_splat_interval(&self) -> &f32 {
        &self.interactive_foam_splat_interval
    }
    fn interactive_waves_enable(&self) -> &super::core::QualityScalableBool {
        &self.interactive_waves_enable
    }
    fn interactive_wave_disturbance_scale(&self) -> &f32 {
        &self.interactive_wave_disturbance_scale
    }
    fn culling_aabbs(&self) -> &Vec<super::core::AxisAlignedBox> {
        &self.culling_aabbs
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static WATERSURFACESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceStaticState",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSurfaceStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CoarseSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: "WaterSimulationHandle",
                rust_offset: offset_of!(WaterSurfaceStaticState, coarse_simulation),
            },
            FieldInfoData {
                name: "DetailSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: "WaterSimulationHandle",
                rust_offset: offset_of!(WaterSurfaceStaticState, detail_simulation),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(WaterSurfaceStaticState, transform_space),
            },
            FieldInfoData {
                name: "Effects",
                flags: MemberInfoFlags::new(144),
                field_type: "WaterAmbientFoamEffect-Array",
                rust_offset: offset_of!(WaterSurfaceStaticState, effects),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(WaterSurfaceStaticState, shader),
            },
            FieldInfoData {
                name: "ShaderLowDetail",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(WaterSurfaceStaticState, shader_low_detail),
            },
            FieldInfoData {
                name: "LowDetailDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(WaterSurfaceStaticState, low_detail_distance),
            },
            FieldInfoData {
                name: "ProjectorElevation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceStaticState, projector_elevation),
            },
            FieldInfoData {
                name: "TerrainVirtualTextureAccessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSurfaceStaticState, terrain_virtual_texture_access_enable),
            },
            FieldInfoData {
                name: "ClipInfo",
                flags: MemberInfoFlags::new(0),
                field_type: "WaterEntityClipInfo",
                rust_offset: offset_of!(WaterSurfaceStaticState, clip_info),
            },
            FieldInfoData {
                name: "InteractiveFoamEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_enable),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_splat_texture),
            },
            FieldInfoData {
                name: "InteractiveFoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_half_life),
            },
            FieldInfoData {
                name: "InteractiveFoamTargetScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_target_scale),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatInterval",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_foam_splat_interval),
            },
            FieldInfoData {
                name: "InteractiveWavesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_waves_enable),
            },
            FieldInfoData {
                name: "InteractiveWaveDisturbanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceStaticState, interactive_wave_disturbance_scale),
            },
            FieldInfoData {
                name: "CullingAabbs",
                flags: MemberInfoFlags::new(144),
                field_type: "AxisAlignedBox-Array",
                rust_offset: offset_of!(WaterSurfaceStaticState, culling_aabbs),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterSurfaceStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSURFACESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterSurfaceStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSURFACESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSURFACESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterSimulationDynamicState {
    pub enable: bool,
    pub enable_foam: bool,
    pub choppiness: f32,
    pub field_flag_changed0: u8,
}

pub trait WaterSimulationDynamicStateTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn enable_foam(&self) -> &bool;
    fn choppiness(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u8;
}

impl WaterSimulationDynamicStateTrait for WaterSimulationDynamicState {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_foam(&self) -> &bool {
        &self.enable_foam
    }
    fn choppiness(&self) -> &f32 {
        &self.choppiness
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static WATERSIMULATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSimulationDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSimulationDynamicState, enable),
            },
            FieldInfoData {
                name: "EnableFoam",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSimulationDynamicState, enable_foam),
            },
            FieldInfoData {
                name: "Choppiness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationDynamicState, choppiness),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(WaterSimulationDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterSimulationDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSIMULATIONDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSimulationDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait WaterSimulationStaticStateTrait: TypeObject {
    fn resolution(&self) -> &super::core::PlatformScalableInt;
    fn tile_dimension(&self) -> &f32;
    fn physics_simulation_enabled(&self) -> &bool;
    fn force_simple_plane_collision(&self) -> &bool;
    fn wave_amplitude(&self) -> &f32;
    fn wind_speed(&self) -> &f32;
    fn wind_angle(&self) -> &f32;
    fn wind_distribution(&self) -> &super::core::SplineCurve;
    fn min_wavelength(&self) -> &f32;
    fn large_wave_reduction(&self) -> &f32;
    fn foam_half_life(&self) -> &f32;
    fn foam_threshold(&self) -> &f32;
    fn foam_max_value(&self) -> &f32;
    fn ocean_visual_cpu_simulation_enable(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u16;
}

impl WaterSimulationStaticStateTrait for WaterSimulationStaticState {
    fn resolution(&self) -> &super::core::PlatformScalableInt {
        &self.resolution
    }
    fn tile_dimension(&self) -> &f32 {
        &self.tile_dimension
    }
    fn physics_simulation_enabled(&self) -> &bool {
        &self.physics_simulation_enabled
    }
    fn force_simple_plane_collision(&self) -> &bool {
        &self.force_simple_plane_collision
    }
    fn wave_amplitude(&self) -> &f32 {
        &self.wave_amplitude
    }
    fn wind_speed(&self) -> &f32 {
        &self.wind_speed
    }
    fn wind_angle(&self) -> &f32 {
        &self.wind_angle
    }
    fn wind_distribution(&self) -> &super::core::SplineCurve {
        &self.wind_distribution
    }
    fn min_wavelength(&self) -> &f32 {
        &self.min_wavelength
    }
    fn large_wave_reduction(&self) -> &f32 {
        &self.large_wave_reduction
    }
    fn foam_half_life(&self) -> &f32 {
        &self.foam_half_life
    }
    fn foam_threshold(&self) -> &f32 {
        &self.foam_threshold
    }
    fn foam_max_value(&self) -> &f32 {
        &self.foam_max_value
    }
    fn ocean_visual_cpu_simulation_enable(&self) -> &bool {
        &self.ocean_visual_cpu_simulation_enable
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static WATERSIMULATIONSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSimulationStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Resolution",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterSimulationStaticState, resolution),
            },
            FieldInfoData {
                name: "TileDimension",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, tile_dimension),
            },
            FieldInfoData {
                name: "PhysicsSimulationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSimulationStaticState, physics_simulation_enabled),
            },
            FieldInfoData {
                name: "ForceSimplePlaneCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSimulationStaticState, force_simple_plane_collision),
            },
            FieldInfoData {
                name: "WaveAmplitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, wave_amplitude),
            },
            FieldInfoData {
                name: "WindSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, wind_speed),
            },
            FieldInfoData {
                name: "WindAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, wind_angle),
            },
            FieldInfoData {
                name: "WindDistribution",
                flags: MemberInfoFlags::new(0),
                field_type: "SplineCurve",
                rust_offset: offset_of!(WaterSimulationStaticState, wind_distribution),
            },
            FieldInfoData {
                name: "MinWavelength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, min_wavelength),
            },
            FieldInfoData {
                name: "LargeWaveReduction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, large_wave_reduction),
            },
            FieldInfoData {
                name: "FoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, foam_half_life),
            },
            FieldInfoData {
                name: "FoamThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, foam_threshold),
            },
            FieldInfoData {
                name: "FoamMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSimulationStaticState, foam_max_value),
            },
            FieldInfoData {
                name: "OceanVisualCpuSimulationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSimulationStaticState, ocean_visual_cpu_simulation_enable),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(WaterSimulationStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERSIMULATIONSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSimulationStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSIMULATIONSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSIMULATIONSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSimulationStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSimulationStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait WaterGlobalDynamicStateTrait: TypeObject {
    fn ticks(&self) -> &u32;
    fn delta_ticks(&self) -> &u32;
    fn tick_frequency(&self) -> &u32;
    fn current_time(&self) -> &f32;
    fn water_height_sample_debugger_sample_position(&self) -> &super::core::Vec3;
    fn water_height_sample_debugger_sample_position_height(&self) -> &f32;
    fn water_height_sample_debugger_enabled(&self) -> &bool;
    fn water_height_sample_debugger_lock_position_enabled(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u8;
}

impl WaterGlobalDynamicStateTrait for WaterGlobalDynamicState {
    fn ticks(&self) -> &u32 {
        &self.ticks
    }
    fn delta_ticks(&self) -> &u32 {
        &self.delta_ticks
    }
    fn tick_frequency(&self) -> &u32 {
        &self.tick_frequency
    }
    fn current_time(&self) -> &f32 {
        &self.current_time
    }
    fn water_height_sample_debugger_sample_position(&self) -> &super::core::Vec3 {
        &self.water_height_sample_debugger_sample_position
    }
    fn water_height_sample_debugger_sample_position_height(&self) -> &f32 {
        &self.water_height_sample_debugger_sample_position_height
    }
    fn water_height_sample_debugger_enabled(&self) -> &bool {
        &self.water_height_sample_debugger_enabled
    }
    fn water_height_sample_debugger_lock_position_enabled(&self) -> &bool {
        &self.water_height_sample_debugger_lock_position_enabled
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static WATERGLOBALDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterGlobalDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Ticks",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterGlobalDynamicState, ticks),
            },
            FieldInfoData {
                name: "DeltaTicks",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterGlobalDynamicState, delta_ticks),
            },
            FieldInfoData {
                name: "TickFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterGlobalDynamicState, tick_frequency),
            },
            FieldInfoData {
                name: "CurrentTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterGlobalDynamicState, current_time),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerSamplePosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_sample_position),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerSamplePositionHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_sample_position_height),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_enabled),
            },
            FieldInfoData {
                name: "WaterHeightSampleDebuggerLockPositionEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterGlobalDynamicState, water_height_sample_debugger_lock_position_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(WaterGlobalDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(WATERGLOBALDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterGlobalDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERGLOBALDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERGLOBALDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterGlobalDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterGlobalStaticState {
}

pub trait WaterGlobalStaticStateTrait: TypeObject {
}

impl WaterGlobalStaticStateTrait for WaterGlobalStaticState {
}

pub static WATERGLOBALSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterGlobalStaticState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERGLOBALSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaterGlobalStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERGLOBALSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERGLOBALSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterGlobalStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterGlobalStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterAmbientFoamEffect {
    pub emitters: Vec<Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>>>,
    pub threshold: f32,
    pub randomness: f32,
    pub cool_down_time: f32,
    pub near_distance: f32,
    pub far_distance: f32,
    pub vertical_velocity_scale: f32,
    pub horizontal_velocity_scale: f32,
}

pub trait WaterAmbientFoamEffectTrait: TypeObject {
    fn emitters(&self) -> &Vec<Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>>>;
    fn threshold(&self) -> &f32;
    fn randomness(&self) -> &f32;
    fn cool_down_time(&self) -> &f32;
    fn near_distance(&self) -> &f32;
    fn far_distance(&self) -> &f32;
    fn vertical_velocity_scale(&self) -> &f32;
    fn horizontal_velocity_scale(&self) -> &f32;
}

impl WaterAmbientFoamEffectTrait for WaterAmbientFoamEffect {
    fn emitters(&self) -> &Vec<Option<Arc<Mutex<dyn super::emitter_base::EmitterBaseAssetTrait>>>> {
        &self.emitters
    }
    fn threshold(&self) -> &f32 {
        &self.threshold
    }
    fn randomness(&self) -> &f32 {
        &self.randomness
    }
    fn cool_down_time(&self) -> &f32 {
        &self.cool_down_time
    }
    fn near_distance(&self) -> &f32 {
        &self.near_distance
    }
    fn far_distance(&self) -> &f32 {
        &self.far_distance
    }
    fn vertical_velocity_scale(&self) -> &f32 {
        &self.vertical_velocity_scale
    }
    fn horizontal_velocity_scale(&self) -> &f32 {
        &self.horizontal_velocity_scale
    }
}

pub static WATERAMBIENTFOAMEFFECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffect",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterAmbientFoamEffect as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Emitters",
                flags: MemberInfoFlags::new(144),
                field_type: "EmitterBaseAsset-Array",
                rust_offset: offset_of!(WaterAmbientFoamEffect, emitters),
            },
            FieldInfoData {
                name: "Threshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffect, threshold),
            },
            FieldInfoData {
                name: "Randomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffect, randomness),
            },
            FieldInfoData {
                name: "CoolDownTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffect, cool_down_time),
            },
            FieldInfoData {
                name: "NearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffect, near_distance),
            },
            FieldInfoData {
                name: "FarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffect, far_distance),
            },
            FieldInfoData {
                name: "VerticalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffect, vertical_velocity_scale),
            },
            FieldInfoData {
                name: "HorizontalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffect, horizontal_velocity_scale),
            },
        ],
    }),
    array_type: Some(WATERAMBIENTFOAMEFFECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterAmbientFoamEffect {
    fn type_info(&self) -> &'static TypeInfo {
        WATERAMBIENTFOAMEFFECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERAMBIENTFOAMEFFECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffect-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterAmbientFoamEffect"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterSurfaceCreateState {
    pub transform: super::core::LinearTransform,
}

pub trait WaterSurfaceCreateStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
}

impl WaterSurfaceCreateStateTrait for WaterSurfaceCreateState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
}

pub static WATERSURFACECREATESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceCreateState",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSurfaceCreateState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(WaterSurfaceCreateState, transform),
            },
        ],
    }),
    array_type: Some(WATERSURFACECREATESTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSurfaceCreateState {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSURFACECREATESTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSURFACECREATESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceCreateState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterSurfaceCreateState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterDisturbParams {
    pub transform: super::core::LinearTransform,
    pub impulse: super::core::Vec3,
}

pub trait WaterDisturbParamsTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn impulse(&self) -> &super::core::Vec3;
}

impl WaterDisturbParamsTrait for WaterDisturbParams {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn impulse(&self) -> &super::core::Vec3 {
        &self.impulse
    }
}

pub static WATERDISTURBPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDisturbParams",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterDisturbParams as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(WaterDisturbParams, transform),
            },
            FieldInfoData {
                name: "Impulse",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WaterDisturbParams, impulse),
            },
        ],
    }),
    array_type: Some(WATERDISTURBPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterDisturbParams {
    fn type_info(&self) -> &'static TypeInfo {
        WATERDISTURBPARAMS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERDISTURBPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDisturbParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterDisturbParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterEntityClipInfo {
    pub enable: bool,
    pub clip_face_north: bool,
    pub clip_face_south: bool,
    pub clip_face_east: bool,
    pub clip_face_west: bool,
}

pub trait WaterEntityClipInfoTrait: TypeObject {
    fn enable(&self) -> &bool;
    fn clip_face_north(&self) -> &bool;
    fn clip_face_south(&self) -> &bool;
    fn clip_face_east(&self) -> &bool;
    fn clip_face_west(&self) -> &bool;
}

impl WaterEntityClipInfoTrait for WaterEntityClipInfo {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn clip_face_north(&self) -> &bool {
        &self.clip_face_north
    }
    fn clip_face_south(&self) -> &bool {
        &self.clip_face_south
    }
    fn clip_face_east(&self) -> &bool {
        &self.clip_face_east
    }
    fn clip_face_west(&self) -> &bool {
        &self.clip_face_west
    }
}

pub static WATERENTITYCLIPINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityClipInfo",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterEntityClipInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterEntityClipInfo, enable),
            },
            FieldInfoData {
                name: "ClipFaceNorth",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_north),
            },
            FieldInfoData {
                name: "ClipFaceSouth",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_south),
            },
            FieldInfoData {
                name: "ClipFaceEast",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_east),
            },
            FieldInfoData {
                name: "ClipFaceWest",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterEntityClipInfo, clip_face_west),
            },
        ],
    }),
    array_type: Some(WATERENTITYCLIPINFO_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaterEntityClipInfo {
    fn type_info(&self) -> &'static TypeInfo {
        WATERENTITYCLIPINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERENTITYCLIPINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEntityClipInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterEntityClipInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait WaterInteractLevelSettingsTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn r#override(&self) -> &bool;
    fn max_simulation_count(&self) -> &super::core::PlatformScalableInt;
    fn max_visible_water_surface_count(&self) -> &super::core::PlatformScalableInt;
    fn render_grid_width(&self) -> &super::core::PlatformScalableInt;
    fn render_grid_height(&self) -> &super::core::PlatformScalableInt;
    fn min_ambient_simulation_resolution(&self) -> &super::core::PlatformScalableInt;
    fn max_ambient_simulation_resolution(&self) -> &super::core::PlatformScalableInt;
}

impl WaterInteractLevelSettingsTrait for WaterInteractLevelSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn r#override(&self) -> &bool {
        &self.r#override
    }
    fn max_simulation_count(&self) -> &super::core::PlatformScalableInt {
        &self.max_simulation_count
    }
    fn max_visible_water_surface_count(&self) -> &super::core::PlatformScalableInt {
        &self.max_visible_water_surface_count
    }
    fn render_grid_width(&self) -> &super::core::PlatformScalableInt {
        &self.render_grid_width
    }
    fn render_grid_height(&self) -> &super::core::PlatformScalableInt {
        &self.render_grid_height
    }
    fn min_ambient_simulation_resolution(&self) -> &super::core::PlatformScalableInt {
        &self.min_ambient_simulation_resolution
    }
    fn max_ambient_simulation_resolution(&self) -> &super::core::PlatformScalableInt {
        &self.max_ambient_simulation_resolution
    }
}

pub static WATERINTERACTLEVELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractLevelSettings",
    flags: MemberInfoFlags::new(36937),
    module: "WaterInteractBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterInteractLevelSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractLevelSettings, enabled),
            },
            FieldInfoData {
                name: "Override",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractLevelSettings, r#override),
            },
            FieldInfoData {
                name: "MaxSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterInteractLevelSettings, max_simulation_count),
            },
            FieldInfoData {
                name: "MaxVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterInteractLevelSettings, max_visible_water_surface_count),
            },
            FieldInfoData {
                name: "RenderGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterInteractLevelSettings, render_grid_width),
            },
            FieldInfoData {
                name: "RenderGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterInteractLevelSettings, render_grid_height),
            },
            FieldInfoData {
                name: "MinAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterInteractLevelSettings, min_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "MaxAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterInteractLevelSettings, max_ambient_simulation_resolution),
            },
        ],
    }),
    array_type: Some(WATERINTERACTLEVELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WaterInteractLevelSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WATERINTERACTLEVELSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERINTERACTLEVELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractLevelSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterInteractLevelSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterInteractSettings {
    pub _glacier_base: super::core::SystemSettings,
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

pub trait WaterInteractSettingsTrait: super::core::SystemSettingsTrait {
    fn enable(&self) -> &bool;
    fn draw_enable(&self) -> &bool;
    fn enable_jobs(&self) -> &bool;
    fn simulation_job_count(&self) -> &u32;
    fn water_quality_level(&self) -> &super::core::QualityLevel;
    fn max_simulation_count(&self) -> &u32;
    fn max_live_editing_simulation_count(&self) -> &u32;
    fn enable_simulation(&self) -> &bool;
    fn enable_disturbs(&self) -> &bool;
    fn interactive_grid_count(&self) -> &u32;
    fn interactive_min_grid_size(&self) -> &u32;
    fn interact_inject_noise_strength(&self) -> &f32;
    fn interact_max_slope(&self) -> &f32;
    fn interact_update_frequency(&self) -> &f32;
    fn min_ambient_simulation_resolution(&self) -> &u32;
    fn max_ambient_simulation_resolution(&self) -> &u32;
    fn render_grid_width(&self) -> &u32;
    fn render_grid_height(&self) -> &u32;
    fn render_fixed_aim_distance(&self) -> &f32;
    fn render_projector_far_plane(&self) -> &f32;
    fn max_visible_water_surface_count(&self) -> &u32;
    fn max_live_editing_visible_water_surface_count(&self) -> &u32;
    fn pc_grid_resolution_multiplier(&self) -> &super::core::QualityScalableFloat;
    fn render_occlusion_cull_enable(&self) -> &bool;
    fn render_occlusion_cull_job_count(&self) -> &u32;
    fn render_occlusion_grid_width(&self) -> &u32;
    fn render_occlusion_grid_height(&self) -> &u32;
    fn render_generate_displacement_mipmaps(&self) -> &bool;
    fn render_generate_gradient_mipmaps(&self) -> &bool;
    fn render_debug_enable(&self) -> &bool;
    fn render_debug_freeze_view_enable(&self) -> &bool;
    fn render_debug_simulation_enable(&self) -> &bool;
    fn render_debug_textures_enable(&self) -> &bool;
    fn draw_update_enable(&self) -> &bool;
    fn virtual_heightfield_atlas_size(&self) -> &i32;
    fn virtual_heightfield_indirection_size(&self) -> &i32;
    fn virtual_heightfield_quantization_range(&self) -> &f32;
}

impl WaterInteractSettingsTrait for WaterInteractSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn simulation_job_count(&self) -> &u32 {
        &self.simulation_job_count
    }
    fn water_quality_level(&self) -> &super::core::QualityLevel {
        &self.water_quality_level
    }
    fn max_simulation_count(&self) -> &u32 {
        &self.max_simulation_count
    }
    fn max_live_editing_simulation_count(&self) -> &u32 {
        &self.max_live_editing_simulation_count
    }
    fn enable_simulation(&self) -> &bool {
        &self.enable_simulation
    }
    fn enable_disturbs(&self) -> &bool {
        &self.enable_disturbs
    }
    fn interactive_grid_count(&self) -> &u32 {
        &self.interactive_grid_count
    }
    fn interactive_min_grid_size(&self) -> &u32 {
        &self.interactive_min_grid_size
    }
    fn interact_inject_noise_strength(&self) -> &f32 {
        &self.interact_inject_noise_strength
    }
    fn interact_max_slope(&self) -> &f32 {
        &self.interact_max_slope
    }
    fn interact_update_frequency(&self) -> &f32 {
        &self.interact_update_frequency
    }
    fn min_ambient_simulation_resolution(&self) -> &u32 {
        &self.min_ambient_simulation_resolution
    }
    fn max_ambient_simulation_resolution(&self) -> &u32 {
        &self.max_ambient_simulation_resolution
    }
    fn render_grid_width(&self) -> &u32 {
        &self.render_grid_width
    }
    fn render_grid_height(&self) -> &u32 {
        &self.render_grid_height
    }
    fn render_fixed_aim_distance(&self) -> &f32 {
        &self.render_fixed_aim_distance
    }
    fn render_projector_far_plane(&self) -> &f32 {
        &self.render_projector_far_plane
    }
    fn max_visible_water_surface_count(&self) -> &u32 {
        &self.max_visible_water_surface_count
    }
    fn max_live_editing_visible_water_surface_count(&self) -> &u32 {
        &self.max_live_editing_visible_water_surface_count
    }
    fn pc_grid_resolution_multiplier(&self) -> &super::core::QualityScalableFloat {
        &self.pc_grid_resolution_multiplier
    }
    fn render_occlusion_cull_enable(&self) -> &bool {
        &self.render_occlusion_cull_enable
    }
    fn render_occlusion_cull_job_count(&self) -> &u32 {
        &self.render_occlusion_cull_job_count
    }
    fn render_occlusion_grid_width(&self) -> &u32 {
        &self.render_occlusion_grid_width
    }
    fn render_occlusion_grid_height(&self) -> &u32 {
        &self.render_occlusion_grid_height
    }
    fn render_generate_displacement_mipmaps(&self) -> &bool {
        &self.render_generate_displacement_mipmaps
    }
    fn render_generate_gradient_mipmaps(&self) -> &bool {
        &self.render_generate_gradient_mipmaps
    }
    fn render_debug_enable(&self) -> &bool {
        &self.render_debug_enable
    }
    fn render_debug_freeze_view_enable(&self) -> &bool {
        &self.render_debug_freeze_view_enable
    }
    fn render_debug_simulation_enable(&self) -> &bool {
        &self.render_debug_simulation_enable
    }
    fn render_debug_textures_enable(&self) -> &bool {
        &self.render_debug_textures_enable
    }
    fn draw_update_enable(&self) -> &bool {
        &self.draw_update_enable
    }
    fn virtual_heightfield_atlas_size(&self) -> &i32 {
        &self.virtual_heightfield_atlas_size
    }
    fn virtual_heightfield_indirection_size(&self) -> &i32 {
        &self.virtual_heightfield_indirection_size
    }
    fn virtual_heightfield_quantization_range(&self) -> &f32 {
        &self.virtual_heightfield_quantization_range
    }
}

impl super::core::SystemSettingsTrait for WaterInteractSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for WaterInteractSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERINTERACTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractSettings",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterInteractSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, enable),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, draw_enable),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, enable_jobs),
            },
            FieldInfoData {
                name: "SimulationJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, simulation_job_count),
            },
            FieldInfoData {
                name: "WaterQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WaterInteractSettings, water_quality_level),
            },
            FieldInfoData {
                name: "MaxSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, max_simulation_count),
            },
            FieldInfoData {
                name: "MaxLiveEditingSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, max_live_editing_simulation_count),
            },
            FieldInfoData {
                name: "EnableSimulation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, enable_simulation),
            },
            FieldInfoData {
                name: "EnableDisturbs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, enable_disturbs),
            },
            FieldInfoData {
                name: "InteractiveGridCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, interactive_grid_count),
            },
            FieldInfoData {
                name: "InteractiveMinGridSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, interactive_min_grid_size),
            },
            FieldInfoData {
                name: "InteractInjectNoiseStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractSettings, interact_inject_noise_strength),
            },
            FieldInfoData {
                name: "InteractMaxSlope",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractSettings, interact_max_slope),
            },
            FieldInfoData {
                name: "InteractUpdateFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractSettings, interact_update_frequency),
            },
            FieldInfoData {
                name: "MinAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, min_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "MaxAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, max_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "RenderGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, render_grid_width),
            },
            FieldInfoData {
                name: "RenderGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, render_grid_height),
            },
            FieldInfoData {
                name: "RenderFixedAimDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractSettings, render_fixed_aim_distance),
            },
            FieldInfoData {
                name: "RenderProjectorFarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractSettings, render_projector_far_plane),
            },
            FieldInfoData {
                name: "MaxVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, max_visible_water_surface_count),
            },
            FieldInfoData {
                name: "MaxLiveEditingVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, max_live_editing_visible_water_surface_count),
            },
            FieldInfoData {
                name: "PcGridResolutionMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(WaterInteractSettings, pc_grid_resolution_multiplier),
            },
            FieldInfoData {
                name: "RenderOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "RenderOcclusionCullJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_cull_job_count),
            },
            FieldInfoData {
                name: "RenderOcclusionGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_grid_width),
            },
            FieldInfoData {
                name: "RenderOcclusionGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaterInteractSettings, render_occlusion_grid_height),
            },
            FieldInfoData {
                name: "RenderGenerateDisplacementMipmaps",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, render_generate_displacement_mipmaps),
            },
            FieldInfoData {
                name: "RenderGenerateGradientMipmaps",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, render_generate_gradient_mipmaps),
            },
            FieldInfoData {
                name: "RenderDebugEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, render_debug_enable),
            },
            FieldInfoData {
                name: "RenderDebugFreezeViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, render_debug_freeze_view_enable),
            },
            FieldInfoData {
                name: "RenderDebugSimulationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, render_debug_simulation_enable),
            },
            FieldInfoData {
                name: "RenderDebugTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, render_debug_textures_enable),
            },
            FieldInfoData {
                name: "DrawUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterInteractSettings, draw_update_enable),
            },
            FieldInfoData {
                name: "VirtualHeightfieldAtlasSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WaterInteractSettings, virtual_heightfield_atlas_size),
            },
            FieldInfoData {
                name: "VirtualHeightfieldIndirectionSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WaterInteractSettings, virtual_heightfield_indirection_size),
            },
            FieldInfoData {
                name: "VirtualHeightfieldQuantizationRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractSettings, virtual_heightfield_quantization_range),
            },
        ],
    }),
    array_type: Some(WATERINTERACTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterInteractSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WATERINTERACTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERINTERACTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractBase",
    data: TypeInfoData::Array("WaterInteractSettings"),
    array_type: None,
    alignment: 8,
};


