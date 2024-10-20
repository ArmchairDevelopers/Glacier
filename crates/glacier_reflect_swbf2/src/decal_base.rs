use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_base_types(registry: &mut TypeRegistry) {
    registry.register_type(DECALVOLUMEENTITYDATA_TYPE_INFO);
    registry.register_type(DECALVOLUMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DECALENTITYDATA_TYPE_INFO);
    registry.register_type(DECALENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(DECALTEMPLATEASSET_TYPE_INFO);
    registry.register_type(DECALTEMPLATEASSET_ARRAY_TYPE_INFO);
    registry.register_type(DECALSETTINGS_TYPE_INFO);
    registry.register_type(DECALSETTINGS_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct DecalVolumeEntityData {
    pub _glacier_base: super::world_sim::RenderVolumeEntityData,
}

pub trait DecalVolumeEntityDataTrait: super::world_sim::RenderVolumeEntityDataTrait {
}

impl DecalVolumeEntityDataTrait for DecalVolumeEntityData {
}

impl super::world_sim::RenderVolumeEntityDataTrait for DecalVolumeEntityData {
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        self._glacier_base.shader()
    }
    fn user_masks(&self) -> &super::core::Vec4 {
        self._glacier_base.user_masks()
    }
    fn transform_type(&self) -> &super::world_base::RenderVolumeTransformType {
        self._glacier_base.transform_type()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
}

impl super::entity::SpatialEntityDataTrait for DecalVolumeEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for DecalVolumeEntityData {
}

impl super::entity::GameObjectDataTrait for DecalVolumeEntityData {
}

impl super::core::DataBusPeerTrait for DecalVolumeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DecalVolumeEntityData {
}

impl super::core::DataContainerTrait for DecalVolumeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DECALVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::world_sim::RENDERVOLUMEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalVolumeEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DECALVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DecalVolumeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DECALVOLUMEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalVolumeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub decal_type: super::render_base::DecalType,
    pub clip_angle: f32,
    pub sorting_priority: u8,
    pub atlas_tile: super::render_base::DecalAtlasTile,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub enabled: bool,
}

pub trait DecalEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn decal_type(&self) -> &super::render_base::DecalType;
    fn clip_angle(&self) -> &f32;
    fn sorting_priority(&self) -> &u8;
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn enabled(&self) -> &bool;
}

impl DecalEntityDataTrait for DecalEntityData {
    fn decal_type(&self) -> &super::render_base::DecalType {
        &self.decal_type
    }
    fn clip_angle(&self) -> &f32 {
        &self.clip_angle
    }
    fn sorting_priority(&self) -> &u8 {
        &self.sorting_priority
    }
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile {
        &self.atlas_tile
    }
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
}

impl super::entity::SpatialEntityDataTrait for DecalEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for DecalEntityData {
}

impl super::entity::GameObjectDataTrait for DecalEntityData {
}

impl super::core::DataBusPeerTrait for DecalEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DecalEntityData {
}

impl super::core::DataContainerTrait for DecalEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DECALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DecalType",
                flags: MemberInfoFlags::new(0),
                field_type: "DecalType",
                rust_offset: offset_of!(DecalEntityData, decal_type),
            },
            FieldInfoData {
                name: "ClipAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalEntityData, clip_angle),
            },
            FieldInfoData {
                name: "SortingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DecalEntityData, sorting_priority),
            },
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: "DecalAtlasTile",
                rust_offset: offset_of!(DecalEntityData, atlas_tile),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(DecalEntityData, shader),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalEntityData, enabled),
            },
        ],
    }),
    array_type: Some(DECALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DecalEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DECALENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalTemplateAsset {
    pub _glacier_base: super::render_base::DecalTemplateBaseAsset,
    pub size: f32,
    pub random_size: f32,
    pub rotation: f32,
    pub random_rotation: f32,
    pub clip_angle: f32,
    pub proximity_radius_factor: f32,
    pub normal_offset: f32,
    pub sorting_priority: u8,
    pub atlas_tile: super::render_base::DecalAtlasTile,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub decal_type: super::render_base::DecalType,
    pub project_multiple: bool,
    pub mesh_u_v_index: i32,
    pub ignore_setup_job_distance_culling: bool,
}

pub trait DecalTemplateAssetTrait: super::render_base::DecalTemplateBaseAssetTrait {
    fn size(&self) -> &f32;
    fn random_size(&self) -> &f32;
    fn rotation(&self) -> &f32;
    fn random_rotation(&self) -> &f32;
    fn clip_angle(&self) -> &f32;
    fn proximity_radius_factor(&self) -> &f32;
    fn normal_offset(&self) -> &f32;
    fn sorting_priority(&self) -> &u8;
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn decal_type(&self) -> &super::render_base::DecalType;
    fn project_multiple(&self) -> &bool;
    fn mesh_u_v_index(&self) -> &i32;
    fn ignore_setup_job_distance_culling(&self) -> &bool;
}

impl DecalTemplateAssetTrait for DecalTemplateAsset {
    fn size(&self) -> &f32 {
        &self.size
    }
    fn random_size(&self) -> &f32 {
        &self.random_size
    }
    fn rotation(&self) -> &f32 {
        &self.rotation
    }
    fn random_rotation(&self) -> &f32 {
        &self.random_rotation
    }
    fn clip_angle(&self) -> &f32 {
        &self.clip_angle
    }
    fn proximity_radius_factor(&self) -> &f32 {
        &self.proximity_radius_factor
    }
    fn normal_offset(&self) -> &f32 {
        &self.normal_offset
    }
    fn sorting_priority(&self) -> &u8 {
        &self.sorting_priority
    }
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile {
        &self.atlas_tile
    }
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn decal_type(&self) -> &super::render_base::DecalType {
        &self.decal_type
    }
    fn project_multiple(&self) -> &bool {
        &self.project_multiple
    }
    fn mesh_u_v_index(&self) -> &i32 {
        &self.mesh_u_v_index
    }
    fn ignore_setup_job_distance_culling(&self) -> &bool {
        &self.ignore_setup_job_distance_culling
    }
}

impl super::render_base::DecalTemplateBaseAssetTrait for DecalTemplateAsset {
}

impl super::core::AssetTrait for DecalTemplateAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for DecalTemplateAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DECALTEMPLATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateAsset",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render_base::DECALTEMPLATEBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalTemplateAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, size),
            },
            FieldInfoData {
                name: "RandomSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, random_size),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, rotation),
            },
            FieldInfoData {
                name: "RandomRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, random_rotation),
            },
            FieldInfoData {
                name: "ClipAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, clip_angle),
            },
            FieldInfoData {
                name: "ProximityRadiusFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, proximity_radius_factor),
            },
            FieldInfoData {
                name: "NormalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, normal_offset),
            },
            FieldInfoData {
                name: "SortingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DecalTemplateAsset, sorting_priority),
            },
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: "DecalAtlasTile",
                rust_offset: offset_of!(DecalTemplateAsset, atlas_tile),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(DecalTemplateAsset, shader),
            },
            FieldInfoData {
                name: "DecalType",
                flags: MemberInfoFlags::new(0),
                field_type: "DecalType",
                rust_offset: offset_of!(DecalTemplateAsset, decal_type),
            },
            FieldInfoData {
                name: "ProjectMultiple",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalTemplateAsset, project_multiple),
            },
            FieldInfoData {
                name: "MeshUVIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DecalTemplateAsset, mesh_u_v_index),
            },
            FieldInfoData {
                name: "IgnoreSetupJobDistanceCulling",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalTemplateAsset, ignore_setup_job_distance_culling),
            },
        ],
    }),
    array_type: Some(DECALTEMPLATEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DecalTemplateAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DECALTEMPLATEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALTEMPLATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalTemplateAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DecalSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enable: bool,
    pub draw_enable: bool,
    pub ring_buffer_size_in_verts: u32,
    pub ring_buffer_recreate_size_in_verts: u32,
    pub projected_decals_triangles_per_job: u32,
    pub decal_recreate_distance_in_meters: f32,
    pub debug_draw_entity_handles: bool,
    pub cull_enable: bool,
    pub distance_cull_enable: bool,
    pub frustum_cull_enable: bool,
    pub occlusion_cull_enable: bool,
    pub distance_cull_falloff: f32,
    pub min_occlusion_test_distance: f32,
    pub min_occlusion_screen_area: f32,
    pub debug_occlusion_cull_enable: bool,
    pub debug_batches: bool,
    pub debug_per_entity_batches: i32,
    pub wireframe_enable: bool,
    pub debug_ringbuffer: i32,
    pub displacement_bias: f32,
    pub displacement_scale: f32,
    pub max_decal_object_prims: u32,
    pub max_total_decal_object_prims: u32,
    pub decal_max_distance: f32,
}

pub trait DecalSettingsTrait: super::core::SystemSettingsTrait {
    fn enable(&self) -> &bool;
    fn draw_enable(&self) -> &bool;
    fn ring_buffer_size_in_verts(&self) -> &u32;
    fn ring_buffer_recreate_size_in_verts(&self) -> &u32;
    fn projected_decals_triangles_per_job(&self) -> &u32;
    fn decal_recreate_distance_in_meters(&self) -> &f32;
    fn debug_draw_entity_handles(&self) -> &bool;
    fn cull_enable(&self) -> &bool;
    fn distance_cull_enable(&self) -> &bool;
    fn frustum_cull_enable(&self) -> &bool;
    fn occlusion_cull_enable(&self) -> &bool;
    fn distance_cull_falloff(&self) -> &f32;
    fn min_occlusion_test_distance(&self) -> &f32;
    fn min_occlusion_screen_area(&self) -> &f32;
    fn debug_occlusion_cull_enable(&self) -> &bool;
    fn debug_batches(&self) -> &bool;
    fn debug_per_entity_batches(&self) -> &i32;
    fn wireframe_enable(&self) -> &bool;
    fn debug_ringbuffer(&self) -> &i32;
    fn displacement_bias(&self) -> &f32;
    fn displacement_scale(&self) -> &f32;
    fn max_decal_object_prims(&self) -> &u32;
    fn max_total_decal_object_prims(&self) -> &u32;
    fn decal_max_distance(&self) -> &f32;
}

impl DecalSettingsTrait for DecalSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn ring_buffer_size_in_verts(&self) -> &u32 {
        &self.ring_buffer_size_in_verts
    }
    fn ring_buffer_recreate_size_in_verts(&self) -> &u32 {
        &self.ring_buffer_recreate_size_in_verts
    }
    fn projected_decals_triangles_per_job(&self) -> &u32 {
        &self.projected_decals_triangles_per_job
    }
    fn decal_recreate_distance_in_meters(&self) -> &f32 {
        &self.decal_recreate_distance_in_meters
    }
    fn debug_draw_entity_handles(&self) -> &bool {
        &self.debug_draw_entity_handles
    }
    fn cull_enable(&self) -> &bool {
        &self.cull_enable
    }
    fn distance_cull_enable(&self) -> &bool {
        &self.distance_cull_enable
    }
    fn frustum_cull_enable(&self) -> &bool {
        &self.frustum_cull_enable
    }
    fn occlusion_cull_enable(&self) -> &bool {
        &self.occlusion_cull_enable
    }
    fn distance_cull_falloff(&self) -> &f32 {
        &self.distance_cull_falloff
    }
    fn min_occlusion_test_distance(&self) -> &f32 {
        &self.min_occlusion_test_distance
    }
    fn min_occlusion_screen_area(&self) -> &f32 {
        &self.min_occlusion_screen_area
    }
    fn debug_occlusion_cull_enable(&self) -> &bool {
        &self.debug_occlusion_cull_enable
    }
    fn debug_batches(&self) -> &bool {
        &self.debug_batches
    }
    fn debug_per_entity_batches(&self) -> &i32 {
        &self.debug_per_entity_batches
    }
    fn wireframe_enable(&self) -> &bool {
        &self.wireframe_enable
    }
    fn debug_ringbuffer(&self) -> &i32 {
        &self.debug_ringbuffer
    }
    fn displacement_bias(&self) -> &f32 {
        &self.displacement_bias
    }
    fn displacement_scale(&self) -> &f32 {
        &self.displacement_scale
    }
    fn max_decal_object_prims(&self) -> &u32 {
        &self.max_decal_object_prims
    }
    fn max_total_decal_object_prims(&self) -> &u32 {
        &self.max_total_decal_object_prims
    }
    fn decal_max_distance(&self) -> &f32 {
        &self.decal_max_distance
    }
}

impl super::core::SystemSettingsTrait for DecalSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for DecalSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DECALSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalSettings",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, enable),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, draw_enable),
            },
            FieldInfoData {
                name: "RingBufferSizeInVerts",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, ring_buffer_size_in_verts),
            },
            FieldInfoData {
                name: "RingBufferRecreateSizeInVerts",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, ring_buffer_recreate_size_in_verts),
            },
            FieldInfoData {
                name: "ProjectedDecalsTrianglesPerJob",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, projected_decals_triangles_per_job),
            },
            FieldInfoData {
                name: "DecalRecreateDistanceInMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, decal_recreate_distance_in_meters),
            },
            FieldInfoData {
                name: "DebugDrawEntityHandles",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, debug_draw_entity_handles),
            },
            FieldInfoData {
                name: "CullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, cull_enable),
            },
            FieldInfoData {
                name: "DistanceCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, distance_cull_enable),
            },
            FieldInfoData {
                name: "FrustumCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, frustum_cull_enable),
            },
            FieldInfoData {
                name: "OcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, occlusion_cull_enable),
            },
            FieldInfoData {
                name: "DistanceCullFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, distance_cull_falloff),
            },
            FieldInfoData {
                name: "MinOcclusionTestDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, min_occlusion_test_distance),
            },
            FieldInfoData {
                name: "MinOcclusionScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, min_occlusion_screen_area),
            },
            FieldInfoData {
                name: "DebugOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, debug_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "DebugBatches",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, debug_batches),
            },
            FieldInfoData {
                name: "DebugPerEntityBatches",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DecalSettings, debug_per_entity_batches),
            },
            FieldInfoData {
                name: "WireframeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, wireframe_enable),
            },
            FieldInfoData {
                name: "DebugRingbuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DecalSettings, debug_ringbuffer),
            },
            FieldInfoData {
                name: "DisplacementBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, displacement_bias),
            },
            FieldInfoData {
                name: "DisplacementScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, displacement_scale),
            },
            FieldInfoData {
                name: "MaxDecalObjectPrims",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, max_decal_object_prims),
            },
            FieldInfoData {
                name: "MaxTotalDecalObjectPrims",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, max_total_decal_object_prims),
            },
            FieldInfoData {
                name: "DecalMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, decal_max_distance),
            },
        ],
    }),
    array_type: Some(DECALSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DecalSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DECALSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DECALSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalSettings"),
    array_type: None,
    alignment: 8,
};


