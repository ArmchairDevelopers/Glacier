use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct WaterHeightEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub realm: super::core::Realm,
    pub auto_start: bool,
}

pub trait WaterHeightEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn auto_start(&self) -> &bool;
}

impl WaterHeightEntityDataTrait for WaterHeightEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
}

impl super::entity::SpatialEntityDataTrait for WaterHeightEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for WaterHeightEntityData {
}

impl super::entity::GameObjectDataTrait for WaterHeightEntityData {
}

impl super::core::DataBusPeerTrait for WaterHeightEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WaterHeightEntityData {
}

impl super::core::DataContainerTrait for WaterHeightEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERHEIGHTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterHeightEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(WaterHeightEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterHeightEntityData, auto_start),
            },
        ],
    }),
    array_type: Some(WATERHEIGHTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterHeightEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERHEIGHTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERHEIGHTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterHeightEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterInteractWaveEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub radius: f32,
    pub amplitude: f32,
}

pub trait WaterInteractWaveEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn radius(&self) -> &f32;
    fn amplitude(&self) -> &f32;
}

impl WaterInteractWaveEntityDataTrait for WaterInteractWaveEntityData {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn amplitude(&self) -> &f32 {
        &self.amplitude
    }
}

impl super::entity::SpatialEntityDataTrait for WaterInteractWaveEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for WaterInteractWaveEntityData {
}

impl super::entity::GameObjectDataTrait for WaterInteractWaveEntityData {
}

impl super::core::DataBusPeerTrait for WaterInteractWaveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WaterInteractWaveEntityData {
}

impl super::core::DataContainerTrait for WaterInteractWaveEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERINTERACTWAVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterInteractWaveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractWaveEntityData, radius),
            },
            FieldInfoData {
                name: "Amplitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractWaveEntityData, amplitude),
            },
        ],
    }),
    array_type: Some(WATERINTERACTWAVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterInteractWaveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERINTERACTWAVEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERINTERACTWAVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractWaveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterInteractTurbulenceDisturbEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub disturb_freq: f32,
    pub area_size_x: f32,
    pub area_size_z: f32,
    pub disturb_size: f32,
    pub disturb_vel: f32,
}

pub trait WaterInteractTurbulenceDisturbEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn disturb_freq(&self) -> &f32;
    fn area_size_x(&self) -> &f32;
    fn area_size_z(&self) -> &f32;
    fn disturb_size(&self) -> &f32;
    fn disturb_vel(&self) -> &f32;
}

impl WaterInteractTurbulenceDisturbEntityDataTrait for WaterInteractTurbulenceDisturbEntityData {
    fn disturb_freq(&self) -> &f32 {
        &self.disturb_freq
    }
    fn area_size_x(&self) -> &f32 {
        &self.area_size_x
    }
    fn area_size_z(&self) -> &f32 {
        &self.area_size_z
    }
    fn disturb_size(&self) -> &f32 {
        &self.disturb_size
    }
    fn disturb_vel(&self) -> &f32 {
        &self.disturb_vel
    }
}

impl super::entity::SpatialEntityDataTrait for WaterInteractTurbulenceDisturbEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for WaterInteractTurbulenceDisturbEntityData {
}

impl super::entity::GameObjectDataTrait for WaterInteractTurbulenceDisturbEntityData {
}

impl super::core::DataBusPeerTrait for WaterInteractTurbulenceDisturbEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WaterInteractTurbulenceDisturbEntityData {
}

impl super::core::DataContainerTrait for WaterInteractTurbulenceDisturbEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERINTERACTTURBULENCEDISTURBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterInteractTurbulenceDisturbEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DisturbFreq",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, disturb_freq),
            },
            FieldInfoData {
                name: "AreaSizeX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, area_size_x),
            },
            FieldInfoData {
                name: "AreaSizeZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, area_size_z),
            },
            FieldInfoData {
                name: "DisturbSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, disturb_size),
            },
            FieldInfoData {
                name: "DisturbVel",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterInteractTurbulenceDisturbEntityData, disturb_vel),
            },
        ],
    }),
    array_type: Some(WATERINTERACTTURBULENCEDISTURBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterInteractTurbulenceDisturbEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERINTERACTTURBULENCEDISTURBENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERINTERACTTURBULENCEDISTURBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractTurbulenceDisturbEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterInteractPhysicsComponentData {
    pub _glacier_base: super::gameplay_sim::GamePhysicsComponentData,
}

pub trait WaterInteractPhysicsComponentDataTrait: super::gameplay_sim::GamePhysicsComponentDataTrait {
}

impl WaterInteractPhysicsComponentDataTrait for WaterInteractPhysicsComponentData {
}

impl super::gameplay_sim::GamePhysicsComponentDataTrait for WaterInteractPhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn effect_parameters(&self) -> &Vec<Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>> {
        self._glacier_base.effect_parameters()
    }
}

impl super::physics::PhysicsComponentDataTrait for WaterInteractPhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<Arc<Mutex<dyn super::physics::PhysicsBodyDataTrait>>>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_constraints(&self) -> &Vec<Option<Arc<Mutex<dyn super::physics::PhysicsConstraintDataTrait>>>> {
        self._glacier_base.physics_constraints()
    }
    fn parts(&self) -> &Vec<super::physics::PhysicsPartData> {
        self._glacier_base.parts()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn internal_collision_disabling(&self) -> &super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
}

impl super::entity::ComponentDataTrait for WaterInteractPhysicsComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for WaterInteractPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for WaterInteractPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WaterInteractPhysicsComponentData {
}

impl super::core::DataContainerTrait for WaterInteractPhysicsComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERINTERACTPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterInteractPhysicsComponentData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERINTERACTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterInteractPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERINTERACTPHYSICSCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERINTERACTPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterSurfaceEntityData {
    pub _glacier_base: super::physics::GamePhysicsEntityData,
    pub additional_water_depth: f32,
    pub effect_setup: Option<Arc<Mutex<dyn WaterEffectSetupTrait>>>,
    pub material_pair: super::entity::MaterialDecl,
    pub query_box_half_extent: super::core::Vec3,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub shader_low_detail: super::render_base::SurfaceShaderInstanceDataStruct,
    pub low_detail_distance: super::core::QualityScalableFloat,
    pub projector_elevation: f32,
    pub terrain_virtual_texture_access_enable: bool,
    pub clip_info: super::water_interact_base::WaterEntityClipInfo,
    pub interactive_foam_enable: super::core::QualityScalableBool,
    pub interactive_foam_splat_texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
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

pub trait WaterSurfaceEntityDataTrait: super::physics::GamePhysicsEntityDataTrait {
    fn additional_water_depth(&self) -> &f32;
    fn effect_setup(&self) -> &Option<Arc<Mutex<dyn WaterEffectSetupTrait>>>;
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn query_box_half_extent(&self) -> &super::core::Vec3;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn shader_low_detail(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn low_detail_distance(&self) -> &super::core::QualityScalableFloat;
    fn projector_elevation(&self) -> &f32;
    fn terrain_virtual_texture_access_enable(&self) -> &bool;
    fn clip_info(&self) -> &super::water_interact_base::WaterEntityClipInfo;
    fn interactive_foam_enable(&self) -> &super::core::QualityScalableBool;
    fn interactive_foam_splat_texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn interactive_foam_half_life(&self) -> &f32;
    fn interactive_foam_target_scale(&self) -> &f32;
    fn interactive_foam_splat_interval(&self) -> &f32;
    fn interactive_waves_enable(&self) -> &super::core::QualityScalableBool;
    fn interactive_wave_disturbance_scale(&self) -> &f32;
    fn culling_aabbs(&self) -> &Vec<super::core::AxisAlignedBox>;
    fn visible(&self) -> &bool;
    fn tile_offset(&self) -> &super::core::Vec3;
    fn wave_amplitude_scale(&self) -> &f32;
    fn shore_enable(&self) -> &bool;
    fn shore_depth(&self) -> &f32;
}

impl WaterSurfaceEntityDataTrait for WaterSurfaceEntityData {
    fn additional_water_depth(&self) -> &f32 {
        &self.additional_water_depth
    }
    fn effect_setup(&self) -> &Option<Arc<Mutex<dyn WaterEffectSetupTrait>>> {
        &self.effect_setup
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn query_box_half_extent(&self) -> &super::core::Vec3 {
        &self.query_box_half_extent
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
    fn clip_info(&self) -> &super::water_interact_base::WaterEntityClipInfo {
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
}

impl super::physics::GamePhysicsEntityDataTrait for WaterSurfaceEntityData {
}

impl super::entity::GameComponentEntityDataTrait for WaterSurfaceEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::entity::ComponentEntityDataTrait for WaterSurfaceEntityData {
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn part_bounding_boxes(&self) -> &Vec<super::core::AxisAlignedBox> {
        self._glacier_base.part_bounding_boxes()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
}

impl super::entity::SpatialEntityDataTrait for WaterSurfaceEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for WaterSurfaceEntityData {
}

impl super::entity::GameObjectDataTrait for WaterSurfaceEntityData {
}

impl super::core::DataBusPeerTrait for WaterSurfaceEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WaterSurfaceEntityData {
}

impl super::core::DataContainerTrait for WaterSurfaceEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERSURFACEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GAMEPHYSICSENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterSurfaceEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AdditionalWaterDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, additional_water_depth),
            },
            FieldInfoData {
                name: "EffectSetup",
                flags: MemberInfoFlags::new(0),
                field_type: "WaterEffectSetup",
                rust_offset: offset_of!(WaterSurfaceEntityData, effect_setup),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(WaterSurfaceEntityData, material_pair),
            },
            FieldInfoData {
                name: "QueryBoxHalfExtent",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WaterSurfaceEntityData, query_box_half_extent),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(WaterSurfaceEntityData, shader),
            },
            FieldInfoData {
                name: "ShaderLowDetail",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(WaterSurfaceEntityData, shader_low_detail),
            },
            FieldInfoData {
                name: "LowDetailDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(WaterSurfaceEntityData, low_detail_distance),
            },
            FieldInfoData {
                name: "ProjectorElevation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, projector_elevation),
            },
            FieldInfoData {
                name: "TerrainVirtualTextureAccessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSurfaceEntityData, terrain_virtual_texture_access_enable),
            },
            FieldInfoData {
                name: "ClipInfo",
                flags: MemberInfoFlags::new(0),
                field_type: "WaterEntityClipInfo",
                rust_offset: offset_of!(WaterSurfaceEntityData, clip_info),
            },
            FieldInfoData {
                name: "InteractiveFoamEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_enable),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_splat_texture),
            },
            FieldInfoData {
                name: "InteractiveFoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_half_life),
            },
            FieldInfoData {
                name: "InteractiveFoamTargetScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_target_scale),
            },
            FieldInfoData {
                name: "InteractiveFoamSplatInterval",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_foam_splat_interval),
            },
            FieldInfoData {
                name: "InteractiveWavesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableBool",
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_waves_enable),
            },
            FieldInfoData {
                name: "InteractiveWaveDisturbanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, interactive_wave_disturbance_scale),
            },
            FieldInfoData {
                name: "CullingAabbs",
                flags: MemberInfoFlags::new(144),
                field_type: "AxisAlignedBox-Array",
                rust_offset: offset_of!(WaterSurfaceEntityData, culling_aabbs),
            },
            FieldInfoData {
                name: "Visible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSurfaceEntityData, visible),
            },
            FieldInfoData {
                name: "TileOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WaterSurfaceEntityData, tile_offset),
            },
            FieldInfoData {
                name: "WaveAmplitudeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, wave_amplitude_scale),
            },
            FieldInfoData {
                name: "ShoreEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterSurfaceEntityData, shore_enable),
            },
            FieldInfoData {
                name: "ShoreDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterSurfaceEntityData, shore_depth),
            },
        ],
    }),
    array_type: Some(WATERSURFACEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterSurfaceEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WATERSURFACEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERSURFACEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterSurfaceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterSurfaceEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterOceanSimulationEntityData {
    pub _glacier_base: super::entity::EntityData,
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

pub trait WaterOceanSimulationEntityDataTrait: super::entity::EntityDataTrait {
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
    fn enable(&self) -> &bool;
    fn enable_foam(&self) -> &bool;
    fn choppiness(&self) -> &f32;
}

impl WaterOceanSimulationEntityDataTrait for WaterOceanSimulationEntityData {
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
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_foam(&self) -> &bool {
        &self.enable_foam
    }
    fn choppiness(&self) -> &f32 {
        &self.choppiness
    }
}

impl super::entity::EntityDataTrait for WaterOceanSimulationEntityData {
}

impl super::entity::GameObjectDataTrait for WaterOceanSimulationEntityData {
}

impl super::core::DataBusPeerTrait for WaterOceanSimulationEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WaterOceanSimulationEntityData {
}

impl super::core::DataContainerTrait for WaterOceanSimulationEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATEROCEANSIMULATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterOceanSimulationEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Resolution",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, resolution),
            },
            FieldInfoData {
                name: "TileDimension",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, tile_dimension),
            },
            FieldInfoData {
                name: "PhysicsSimulationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, physics_simulation_enabled),
            },
            FieldInfoData {
                name: "ForceSimplePlaneCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, force_simple_plane_collision),
            },
            FieldInfoData {
                name: "WaveAmplitude",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wave_amplitude),
            },
            FieldInfoData {
                name: "WindSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wind_speed),
            },
            FieldInfoData {
                name: "WindAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wind_angle),
            },
            FieldInfoData {
                name: "WindDistribution",
                flags: MemberInfoFlags::new(0),
                field_type: "SplineCurve",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, wind_distribution),
            },
            FieldInfoData {
                name: "MinWavelength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, min_wavelength),
            },
            FieldInfoData {
                name: "LargeWaveReduction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, large_wave_reduction),
            },
            FieldInfoData {
                name: "FoamHalfLife",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, foam_half_life),
            },
            FieldInfoData {
                name: "FoamThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, foam_threshold),
            },
            FieldInfoData {
                name: "FoamMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, foam_max_value),
            },
            FieldInfoData {
                name: "OceanVisualCpuSimulationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, ocean_visual_cpu_simulation_enable),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, enable),
            },
            FieldInfoData {
                name: "EnableFoam",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, enable_foam),
            },
            FieldInfoData {
                name: "Choppiness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterOceanSimulationEntityData, choppiness),
            },
        ],
    }),
    array_type: Some(WATEROCEANSIMULATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WaterOceanSimulationEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WATEROCEANSIMULATIONENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATEROCEANSIMULATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterOceanSimulationEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterEffectSetup {
    pub _glacier_base: super::core::Asset,
    pub ambient_effects: Vec<WaterAmbientFoamEffectSpawner>,
}

pub trait WaterEffectSetupTrait: super::core::AssetTrait {
    fn ambient_effects(&self) -> &Vec<WaterAmbientFoamEffectSpawner>;
}

impl WaterEffectSetupTrait for WaterEffectSetup {
    fn ambient_effects(&self) -> &Vec<WaterAmbientFoamEffectSpawner> {
        &self.ambient_effects
    }
}

impl super::core::AssetTrait for WaterEffectSetup {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for WaterEffectSetup {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATEREFFECTSETUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEffectSetup",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterEffectSetup as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AmbientEffects",
                flags: MemberInfoFlags::new(144),
                field_type: "WaterAmbientFoamEffectSpawner-Array",
                rust_offset: offset_of!(WaterEffectSetup, ambient_effects),
            },
        ],
    }),
    array_type: Some(WATEREFFECTSETUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterEffectSetup {
    fn type_info(&self) -> &'static TypeInfo {
        WATEREFFECTSETUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATEREFFECTSETUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterEffectSetup-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterEffectSetup"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterAmbientFoamEffectSpawner {
    pub effect: Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>,
    pub threshold: f32,
    pub randomness: f32,
    pub cool_down_time: f32,
    pub near_distance: f32,
    pub far_distance: f32,
    pub vertical_velocity_scale: f32,
    pub horizontal_velocity_scale: f32,
}

pub trait WaterAmbientFoamEffectSpawnerTrait: TypeObject {
    fn effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>>;
    fn threshold(&self) -> &f32;
    fn randomness(&self) -> &f32;
    fn cool_down_time(&self) -> &f32;
    fn near_distance(&self) -> &f32;
    fn far_distance(&self) -> &f32;
    fn vertical_velocity_scale(&self) -> &f32;
    fn horizontal_velocity_scale(&self) -> &f32;
}

impl WaterAmbientFoamEffectSpawnerTrait for WaterAmbientFoamEffectSpawner {
    fn effect(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectBlueprintTrait>>> {
        &self.effect
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

pub static WATERAMBIENTFOAMEFFECTSPAWNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffectSpawner",
    flags: MemberInfoFlags::new(73),
    module: "WaterInteractSim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterAmbientFoamEffectSpawner as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, effect),
            },
            FieldInfoData {
                name: "Threshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, threshold),
            },
            FieldInfoData {
                name: "Randomness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, randomness),
            },
            FieldInfoData {
                name: "CoolDownTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, cool_down_time),
            },
            FieldInfoData {
                name: "NearDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, near_distance),
            },
            FieldInfoData {
                name: "FarDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, far_distance),
            },
            FieldInfoData {
                name: "VerticalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, vertical_velocity_scale),
            },
            FieldInfoData {
                name: "HorizontalVelocityScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WaterAmbientFoamEffectSpawner, horizontal_velocity_scale),
            },
        ],
    }),
    array_type: Some(WATERAMBIENTFOAMEFFECTSPAWNER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterAmbientFoamEffectSpawner {
    fn type_info(&self) -> &'static TypeInfo {
        WATERAMBIENTFOAMEFFECTSPAWNER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERAMBIENTFOAMEFFECTSPAWNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAmbientFoamEffectSpawner-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterAmbientFoamEffectSpawner"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterLevelDescriptionComponent {
    pub _glacier_base: super::gameplay_sim::LevelDescriptionComponent,
    pub enabled: bool,
    pub r#override: bool,
    pub max_simulation_count: super::core::PlatformScalableInt,
    pub max_visible_water_surface_count: super::core::PlatformScalableInt,
    pub render_grid_width: super::core::PlatformScalableInt,
    pub render_grid_height: super::core::PlatformScalableInt,
    pub min_ambient_simulation_resolution: super::core::PlatformScalableInt,
    pub max_ambient_simulation_resolution: super::core::PlatformScalableInt,
}

pub trait WaterLevelDescriptionComponentTrait: super::gameplay_sim::LevelDescriptionComponentTrait {
    fn enabled(&self) -> &bool;
    fn r#override(&self) -> &bool;
    fn max_simulation_count(&self) -> &super::core::PlatformScalableInt;
    fn max_visible_water_surface_count(&self) -> &super::core::PlatformScalableInt;
    fn render_grid_width(&self) -> &super::core::PlatformScalableInt;
    fn render_grid_height(&self) -> &super::core::PlatformScalableInt;
    fn min_ambient_simulation_resolution(&self) -> &super::core::PlatformScalableInt;
    fn max_ambient_simulation_resolution(&self) -> &super::core::PlatformScalableInt;
}

impl WaterLevelDescriptionComponentTrait for WaterLevelDescriptionComponent {
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

impl super::gameplay_sim::LevelDescriptionComponentTrait for WaterLevelDescriptionComponent {
}

impl super::core::DataContainerTrait for WaterLevelDescriptionComponent {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WATERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterLevelDescriptionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterLevelDescriptionComponent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, enabled),
            },
            FieldInfoData {
                name: "Override",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, r#override),
            },
            FieldInfoData {
                name: "MaxSimulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, max_simulation_count),
            },
            FieldInfoData {
                name: "MaxVisibleWaterSurfaceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, max_visible_water_surface_count),
            },
            FieldInfoData {
                name: "RenderGridWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, render_grid_width),
            },
            FieldInfoData {
                name: "RenderGridHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, render_grid_height),
            },
            FieldInfoData {
                name: "MinAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, min_ambient_simulation_resolution),
            },
            FieldInfoData {
                name: "MaxAmbientSimulationResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableInt",
                rust_offset: offset_of!(WaterLevelDescriptionComponent, max_ambient_simulation_resolution),
            },
        ],
    }),
    array_type: Some(WATERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterLevelDescriptionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        WATERLEVELDESCRIPTIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERLEVELDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterLevelDescriptionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterLevelDescriptionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWaterOceanSimulationEntity {
    pub _glacier_base: WaterOceanSimulationEntity,
}

pub trait ServerWaterOceanSimulationEntityTrait: WaterOceanSimulationEntityTrait {
}

impl ServerWaterOceanSimulationEntityTrait for ServerWaterOceanSimulationEntity {
}

impl WaterOceanSimulationEntityTrait for ServerWaterOceanSimulationEntity {
}

impl super::entity::EntityTrait for ServerWaterOceanSimulationEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWaterOceanSimulationEntity {
}

pub static SERVERWATEROCEANSIMULATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterOceanSimulationEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATEROCEANSIMULATIONENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaterOceanSimulationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterOceanSimulationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWATEROCEANSIMULATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterOceanSimulationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ServerWaterOceanSimulationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterOceanSimulationEntity {
    pub _glacier_base: WaterOceanSimulationEntity,
}

pub trait ClientWaterOceanSimulationEntityTrait: WaterOceanSimulationEntityTrait {
}

impl ClientWaterOceanSimulationEntityTrait for ClientWaterOceanSimulationEntity {
}

impl WaterOceanSimulationEntityTrait for ClientWaterOceanSimulationEntity {
}

impl super::entity::EntityTrait for ClientWaterOceanSimulationEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWaterOceanSimulationEntity {
}

pub static CLIENTWATEROCEANSIMULATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterOceanSimulationEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATEROCEANSIMULATIONENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterOceanSimulationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterOceanSimulationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATEROCEANSIMULATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterOceanSimulationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterOceanSimulationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterOceanSimulationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait WaterOceanSimulationEntityTrait: super::entity::EntityTrait {
}

impl WaterOceanSimulationEntityTrait for WaterOceanSimulationEntity {
}

impl super::entity::EntityTrait for WaterOceanSimulationEntity {
}

impl super::entity::EntityBusPeerTrait for WaterOceanSimulationEntity {
}

pub static WATEROCEANSIMULATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterOceanSimulationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterOceanSimulationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WATEROCEANSIMULATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATEROCEANSIMULATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterOceanSimulationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterOceanSimulationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterInteractWaveEntity {
    pub _glacier_base: WaterInteractWaveEntity,
}

pub trait ClientWaterInteractWaveEntityTrait: WaterInteractWaveEntityTrait {
}

impl ClientWaterInteractWaveEntityTrait for ClientWaterInteractWaveEntity {
}

impl WaterInteractWaveEntityTrait for ClientWaterInteractWaveEntity {
}

impl super::entity::EntityTrait for ClientWaterInteractWaveEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWaterInteractWaveEntity {
}

pub static CLIENTWATERINTERACTWAVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractWaveEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WATERINTERACTWAVEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterInteractWaveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterInteractWaveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATERINTERACTWAVEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractWaveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterInteractWaveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterInteractWaveEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait WaterInteractWaveEntityTrait: super::entity::EntityTrait {
}

impl WaterInteractWaveEntityTrait for WaterInteractWaveEntity {
}

impl super::entity::EntityTrait for WaterInteractWaveEntity {
}

impl super::entity::EntityBusPeerTrait for WaterInteractWaveEntity {
}

pub static WATERINTERACTWAVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterInteractWaveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterInteractWaveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WATERINTERACTWAVEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERINTERACTWAVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractWaveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractWaveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterHeightEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait WaterHeightEntityTrait: super::entity::SpatialEntityTrait {
}

impl WaterHeightEntityTrait for WaterHeightEntity {
}

impl super::entity::SpatialEntityTrait for WaterHeightEntity {
}

impl super::entity::EntityTrait for WaterHeightEntity {
}

impl super::entity::EntityBusPeerTrait for WaterHeightEntity {
}

pub static WATERHEIGHTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterHeightEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERHEIGHTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterHeightEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WATERHEIGHTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERHEIGHTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterHeightEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterHeightEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWaterSurfaceEntity {
    pub _glacier_base: super::game_server::ServerPhysicsEntity,
}

pub trait ServerWaterSurfaceEntityTrait: super::game_server::ServerPhysicsEntityTrait {
}

impl ServerWaterSurfaceEntityTrait for ServerWaterSurfaceEntity {
}

impl super::game_server::ServerPhysicsEntityTrait for ServerWaterSurfaceEntity {
}

impl super::game_server::ServerGameComponentEntityTrait for ServerWaterSurfaceEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ServerWaterSurfaceEntity {
}

impl super::entity::ComponentEntityTrait for ServerWaterSurfaceEntity {
}

impl super::entity::SpatialEntityTrait for ServerWaterSurfaceEntity {
}

impl super::entity::EntityTrait for ServerWaterSurfaceEntity {
}

impl super::entity::EntityBusPeerTrait for ServerWaterSurfaceEntity {
}

pub static SERVERWATERSURFACEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterSurfaceEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_server::SERVERPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaterSurfaceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERSURFACEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterSurfaceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWATERSURFACEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWATERSURFACEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterSurfaceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ServerWaterSurfaceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerWaterInteractPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ServerWaterInteractPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ServerWaterInteractPhysicsComponentTrait for ServerWaterInteractPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ServerWaterInteractPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerWaterInteractPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWaterInteractPhysicsComponent {
}

pub static SERVERWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterInteractPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWaterInteractPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWaterInteractPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWaterInteractPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ServerWaterInteractPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterSurfaceEntity {
    pub _glacier_base: super::gameplay_client_server::ClientPhysicsEntity,
}

pub trait ClientWaterSurfaceEntityTrait: super::gameplay_client_server::ClientPhysicsEntityTrait {
}

impl ClientWaterSurfaceEntityTrait for ClientWaterSurfaceEntity {
}

impl super::gameplay_client_server::ClientPhysicsEntityTrait for ClientWaterSurfaceEntity {
}

impl super::gameplay_client_server::ClientGameComponentEntityTrait for ClientWaterSurfaceEntity {
}

impl super::gameplay_sim::GameComponentEntityTrait for ClientWaterSurfaceEntity {
}

impl super::entity::ComponentEntityTrait for ClientWaterSurfaceEntity {
}

impl super::entity::SpatialEntityTrait for ClientWaterSurfaceEntity {
}

impl super::entity::EntityTrait for ClientWaterSurfaceEntity {
}

impl super::entity::EntityBusPeerTrait for ClientWaterSurfaceEntity {
}

pub static CLIENTWATERSURFACEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterSurfaceEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTPHYSICSENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterSurfaceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERSURFACEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterSurfaceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATERSURFACEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATERSURFACEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterSurfaceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterSurfaceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWaterInteractPhysicsComponent {
    pub _glacier_base: super::physics::PhysicsComponent,
}

pub trait ClientWaterInteractPhysicsComponentTrait: super::physics::PhysicsComponentTrait {
}

impl ClientWaterInteractPhysicsComponentTrait for ClientWaterInteractPhysicsComponent {
}

impl super::physics::PhysicsComponentTrait for ClientWaterInteractPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientWaterInteractPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWaterInteractPhysicsComponent {
}

pub static CLIENTWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWaterInteractPhysicsComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWaterInteractPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWATERINTERACTPHYSICSCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTWATERINTERACTPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWaterInteractPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("ClientWaterInteractPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterInteractTurbulenceDisturbEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait WaterInteractTurbulenceDisturbEntityTrait: super::entity::SpatialEntityTrait {
}

impl WaterInteractTurbulenceDisturbEntityTrait for WaterInteractTurbulenceDisturbEntity {
}

impl super::entity::SpatialEntityTrait for WaterInteractTurbulenceDisturbEntity {
}

impl super::entity::EntityTrait for WaterInteractTurbulenceDisturbEntity {
}

impl super::entity::EntityBusPeerTrait for WaterInteractTurbulenceDisturbEntity {
}

pub static WATERINTERACTTURBULENCEDISTURBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntity",
    flags: MemberInfoFlags::new(101),
    module: "WaterInteractSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterInteractTurbulenceDisturbEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERINTERACTTURBULENCEDISTURBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterInteractTurbulenceDisturbEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WATERINTERACTTURBULENCEDISTURBENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERINTERACTTURBULENCEDISTURBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterInteractTurbulenceDisturbEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "WaterInteractSim",
    data: TypeInfoData::Array("WaterInteractTurbulenceDisturbEntity"),
    array_type: None,
    alignment: 8,
};


