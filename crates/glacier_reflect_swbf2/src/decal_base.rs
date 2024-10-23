use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        self._glacier_base.shader_mut()
    }
    fn user_masks(&self) -> &super::core::Vec4 {
        self._glacier_base.user_masks()
    }
    fn user_masks_mut(&mut self) -> &mut super::core::Vec4 {
        self._glacier_base.user_masks_mut()
    }
    fn transform_type(&self) -> &super::world_base::RenderVolumeTransformType {
        self._glacier_base.transform_type()
    }
    fn transform_type_mut(&mut self) -> &mut super::world_base::RenderVolumeTransformType {
        self._glacier_base.transform_type_mut()
    }
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for DecalVolumeEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DecalVolumeEntityData {
}

impl super::core::DataContainerTrait for DecalVolumeEntityData {
}

pub static DECALVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntityData",
    name_hash: 978151817,
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::world_sim::RENDERVOLUMEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DecalVolumeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalVolumeEntityData as Default>::default())),
            create_boxed: || Box::new(<DecalVolumeEntityData as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DECALVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntityData-Array",
    name_hash: 2197774141,
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalVolumeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn decal_type_mut(&mut self) -> &mut super::render_base::DecalType;
    fn clip_angle(&self) -> &f32;
    fn clip_angle_mut(&mut self) -> &mut f32;
    fn sorting_priority(&self) -> &u8;
    fn sorting_priority_mut(&mut self) -> &mut u8;
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile;
    fn atlas_tile_mut(&mut self) -> &mut super::render_base::DecalAtlasTile;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
}

impl DecalEntityDataTrait for DecalEntityData {
    fn decal_type(&self) -> &super::render_base::DecalType {
        &self.decal_type
    }
    fn decal_type_mut(&mut self) -> &mut super::render_base::DecalType {
        &mut self.decal_type
    }
    fn clip_angle(&self) -> &f32 {
        &self.clip_angle
    }
    fn clip_angle_mut(&mut self) -> &mut f32 {
        &mut self.clip_angle
    }
    fn sorting_priority(&self) -> &u8 {
        &self.sorting_priority
    }
    fn sorting_priority_mut(&mut self) -> &mut u8 {
        &mut self.sorting_priority
    }
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile {
        &self.atlas_tile
    }
    fn atlas_tile_mut(&mut self) -> &mut super::render_base::DecalAtlasTile {
        &mut self.atlas_tile
    }
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        &mut self.shader
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}

impl super::entity::SpatialEntityDataTrait for DecalEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
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
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DecalEntityData {
}

impl super::core::DataContainerTrait for DecalEntityData {
}

pub static DECALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntityData",
    name_hash: 817991169,
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(DecalEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalEntityData as Default>::default())),
            create_boxed: || Box::new(<DecalEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DecalType",
                name_hash: 3246067474,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalType",
                rust_offset: offset_of!(DecalEntityData, decal_type),
            },
            FieldInfoData {
                name: "ClipAngle",
                name_hash: 1632648850,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalEntityData, clip_angle),
            },
            FieldInfoData {
                name: "SortingPriority",
                name_hash: 3523655821,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DecalEntityData, sorting_priority),
            },
            FieldInfoData {
                name: "AtlasTile",
                name_hash: 3027817338,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalAtlasTile",
                rust_offset: offset_of!(DecalEntityData, atlas_tile),
            },
            FieldInfoData {
                name: "Shader",
                name_hash: 3352909900,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(DecalEntityData, shader),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DECALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntityData-Array",
    name_hash: 3952969653,
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn size_mut(&mut self) -> &mut f32;
    fn random_size(&self) -> &f32;
    fn random_size_mut(&mut self) -> &mut f32;
    fn rotation(&self) -> &f32;
    fn rotation_mut(&mut self) -> &mut f32;
    fn random_rotation(&self) -> &f32;
    fn random_rotation_mut(&mut self) -> &mut f32;
    fn clip_angle(&self) -> &f32;
    fn clip_angle_mut(&mut self) -> &mut f32;
    fn proximity_radius_factor(&self) -> &f32;
    fn proximity_radius_factor_mut(&mut self) -> &mut f32;
    fn normal_offset(&self) -> &f32;
    fn normal_offset_mut(&mut self) -> &mut f32;
    fn sorting_priority(&self) -> &u8;
    fn sorting_priority_mut(&mut self) -> &mut u8;
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile;
    fn atlas_tile_mut(&mut self) -> &mut super::render_base::DecalAtlasTile;
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct;
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct;
    fn decal_type(&self) -> &super::render_base::DecalType;
    fn decal_type_mut(&mut self) -> &mut super::render_base::DecalType;
    fn project_multiple(&self) -> &bool;
    fn project_multiple_mut(&mut self) -> &mut bool;
    fn mesh_u_v_index(&self) -> &i32;
    fn mesh_u_v_index_mut(&mut self) -> &mut i32;
    fn ignore_setup_job_distance_culling(&self) -> &bool;
    fn ignore_setup_job_distance_culling_mut(&mut self) -> &mut bool;
}

impl DecalTemplateAssetTrait for DecalTemplateAsset {
    fn size(&self) -> &f32 {
        &self.size
    }
    fn size_mut(&mut self) -> &mut f32 {
        &mut self.size
    }
    fn random_size(&self) -> &f32 {
        &self.random_size
    }
    fn random_size_mut(&mut self) -> &mut f32 {
        &mut self.random_size
    }
    fn rotation(&self) -> &f32 {
        &self.rotation
    }
    fn rotation_mut(&mut self) -> &mut f32 {
        &mut self.rotation
    }
    fn random_rotation(&self) -> &f32 {
        &self.random_rotation
    }
    fn random_rotation_mut(&mut self) -> &mut f32 {
        &mut self.random_rotation
    }
    fn clip_angle(&self) -> &f32 {
        &self.clip_angle
    }
    fn clip_angle_mut(&mut self) -> &mut f32 {
        &mut self.clip_angle
    }
    fn proximity_radius_factor(&self) -> &f32 {
        &self.proximity_radius_factor
    }
    fn proximity_radius_factor_mut(&mut self) -> &mut f32 {
        &mut self.proximity_radius_factor
    }
    fn normal_offset(&self) -> &f32 {
        &self.normal_offset
    }
    fn normal_offset_mut(&mut self) -> &mut f32 {
        &mut self.normal_offset
    }
    fn sorting_priority(&self) -> &u8 {
        &self.sorting_priority
    }
    fn sorting_priority_mut(&mut self) -> &mut u8 {
        &mut self.sorting_priority
    }
    fn atlas_tile(&self) -> &super::render_base::DecalAtlasTile {
        &self.atlas_tile
    }
    fn atlas_tile_mut(&mut self) -> &mut super::render_base::DecalAtlasTile {
        &mut self.atlas_tile
    }
    fn shader(&self) -> &super::render_base::SurfaceShaderInstanceDataStruct {
        &self.shader
    }
    fn shader_mut(&mut self) -> &mut super::render_base::SurfaceShaderInstanceDataStruct {
        &mut self.shader
    }
    fn decal_type(&self) -> &super::render_base::DecalType {
        &self.decal_type
    }
    fn decal_type_mut(&mut self) -> &mut super::render_base::DecalType {
        &mut self.decal_type
    }
    fn project_multiple(&self) -> &bool {
        &self.project_multiple
    }
    fn project_multiple_mut(&mut self) -> &mut bool {
        &mut self.project_multiple
    }
    fn mesh_u_v_index(&self) -> &i32 {
        &self.mesh_u_v_index
    }
    fn mesh_u_v_index_mut(&mut self) -> &mut i32 {
        &mut self.mesh_u_v_index
    }
    fn ignore_setup_job_distance_culling(&self) -> &bool {
        &self.ignore_setup_job_distance_culling
    }
    fn ignore_setup_job_distance_culling_mut(&mut self) -> &mut bool {
        &mut self.ignore_setup_job_distance_culling
    }
}

impl super::render_base::DecalTemplateBaseAssetTrait for DecalTemplateAsset {
}

impl super::core::AssetTrait for DecalTemplateAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DecalTemplateAsset {
}

pub static DECALTEMPLATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateAsset",
    name_hash: 1238023850,
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render_base::DECALTEMPLATEBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(DecalTemplateAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalTemplateAsset as Default>::default())),
            create_boxed: || Box::new(<DecalTemplateAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Size",
                name_hash: 2089429248,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, size),
            },
            FieldInfoData {
                name: "RandomSize",
                name_hash: 3548218523,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, random_size),
            },
            FieldInfoData {
                name: "Rotation",
                name_hash: 48673745,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, rotation),
            },
            FieldInfoData {
                name: "RandomRotation",
                name_hash: 4221673930,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, random_rotation),
            },
            FieldInfoData {
                name: "ClipAngle",
                name_hash: 1632648850,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, clip_angle),
            },
            FieldInfoData {
                name: "ProximityRadiusFactor",
                name_hash: 4036214629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, proximity_radius_factor),
            },
            FieldInfoData {
                name: "NormalOffset",
                name_hash: 393006331,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalTemplateAsset, normal_offset),
            },
            FieldInfoData {
                name: "SortingPriority",
                name_hash: 3523655821,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DecalTemplateAsset, sorting_priority),
            },
            FieldInfoData {
                name: "AtlasTile",
                name_hash: 3027817338,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalAtlasTile",
                rust_offset: offset_of!(DecalTemplateAsset, atlas_tile),
            },
            FieldInfoData {
                name: "Shader",
                name_hash: 3352909900,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceShaderInstanceDataStruct",
                rust_offset: offset_of!(DecalTemplateAsset, shader),
            },
            FieldInfoData {
                name: "DecalType",
                name_hash: 3246067474,
                flags: MemberInfoFlags::new(0),
                field_type: "DecalType",
                rust_offset: offset_of!(DecalTemplateAsset, decal_type),
            },
            FieldInfoData {
                name: "ProjectMultiple",
                name_hash: 4168918560,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalTemplateAsset, project_multiple),
            },
            FieldInfoData {
                name: "MeshUVIndex",
                name_hash: 114817035,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DecalTemplateAsset, mesh_u_v_index),
            },
            FieldInfoData {
                name: "IgnoreSetupJobDistanceCulling",
                name_hash: 551741352,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DECALTEMPLATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateAsset-Array",
    name_hash: 2205514014,
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalTemplateAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn enable_mut(&mut self) -> &mut bool;
    fn draw_enable(&self) -> &bool;
    fn draw_enable_mut(&mut self) -> &mut bool;
    fn ring_buffer_size_in_verts(&self) -> &u32;
    fn ring_buffer_size_in_verts_mut(&mut self) -> &mut u32;
    fn ring_buffer_recreate_size_in_verts(&self) -> &u32;
    fn ring_buffer_recreate_size_in_verts_mut(&mut self) -> &mut u32;
    fn projected_decals_triangles_per_job(&self) -> &u32;
    fn projected_decals_triangles_per_job_mut(&mut self) -> &mut u32;
    fn decal_recreate_distance_in_meters(&self) -> &f32;
    fn decal_recreate_distance_in_meters_mut(&mut self) -> &mut f32;
    fn debug_draw_entity_handles(&self) -> &bool;
    fn debug_draw_entity_handles_mut(&mut self) -> &mut bool;
    fn cull_enable(&self) -> &bool;
    fn cull_enable_mut(&mut self) -> &mut bool;
    fn distance_cull_enable(&self) -> &bool;
    fn distance_cull_enable_mut(&mut self) -> &mut bool;
    fn frustum_cull_enable(&self) -> &bool;
    fn frustum_cull_enable_mut(&mut self) -> &mut bool;
    fn occlusion_cull_enable(&self) -> &bool;
    fn occlusion_cull_enable_mut(&mut self) -> &mut bool;
    fn distance_cull_falloff(&self) -> &f32;
    fn distance_cull_falloff_mut(&mut self) -> &mut f32;
    fn min_occlusion_test_distance(&self) -> &f32;
    fn min_occlusion_test_distance_mut(&mut self) -> &mut f32;
    fn min_occlusion_screen_area(&self) -> &f32;
    fn min_occlusion_screen_area_mut(&mut self) -> &mut f32;
    fn debug_occlusion_cull_enable(&self) -> &bool;
    fn debug_occlusion_cull_enable_mut(&mut self) -> &mut bool;
    fn debug_batches(&self) -> &bool;
    fn debug_batches_mut(&mut self) -> &mut bool;
    fn debug_per_entity_batches(&self) -> &i32;
    fn debug_per_entity_batches_mut(&mut self) -> &mut i32;
    fn wireframe_enable(&self) -> &bool;
    fn wireframe_enable_mut(&mut self) -> &mut bool;
    fn debug_ringbuffer(&self) -> &i32;
    fn debug_ringbuffer_mut(&mut self) -> &mut i32;
    fn displacement_bias(&self) -> &f32;
    fn displacement_bias_mut(&mut self) -> &mut f32;
    fn displacement_scale(&self) -> &f32;
    fn displacement_scale_mut(&mut self) -> &mut f32;
    fn max_decal_object_prims(&self) -> &u32;
    fn max_decal_object_prims_mut(&mut self) -> &mut u32;
    fn max_total_decal_object_prims(&self) -> &u32;
    fn max_total_decal_object_prims_mut(&mut self) -> &mut u32;
    fn decal_max_distance(&self) -> &f32;
    fn decal_max_distance_mut(&mut self) -> &mut f32;
}

impl DecalSettingsTrait for DecalSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn draw_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_enable
    }
    fn ring_buffer_size_in_verts(&self) -> &u32 {
        &self.ring_buffer_size_in_verts
    }
    fn ring_buffer_size_in_verts_mut(&mut self) -> &mut u32 {
        &mut self.ring_buffer_size_in_verts
    }
    fn ring_buffer_recreate_size_in_verts(&self) -> &u32 {
        &self.ring_buffer_recreate_size_in_verts
    }
    fn ring_buffer_recreate_size_in_verts_mut(&mut self) -> &mut u32 {
        &mut self.ring_buffer_recreate_size_in_verts
    }
    fn projected_decals_triangles_per_job(&self) -> &u32 {
        &self.projected_decals_triangles_per_job
    }
    fn projected_decals_triangles_per_job_mut(&mut self) -> &mut u32 {
        &mut self.projected_decals_triangles_per_job
    }
    fn decal_recreate_distance_in_meters(&self) -> &f32 {
        &self.decal_recreate_distance_in_meters
    }
    fn decal_recreate_distance_in_meters_mut(&mut self) -> &mut f32 {
        &mut self.decal_recreate_distance_in_meters
    }
    fn debug_draw_entity_handles(&self) -> &bool {
        &self.debug_draw_entity_handles
    }
    fn debug_draw_entity_handles_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_entity_handles
    }
    fn cull_enable(&self) -> &bool {
        &self.cull_enable
    }
    fn cull_enable_mut(&mut self) -> &mut bool {
        &mut self.cull_enable
    }
    fn distance_cull_enable(&self) -> &bool {
        &self.distance_cull_enable
    }
    fn distance_cull_enable_mut(&mut self) -> &mut bool {
        &mut self.distance_cull_enable
    }
    fn frustum_cull_enable(&self) -> &bool {
        &self.frustum_cull_enable
    }
    fn frustum_cull_enable_mut(&mut self) -> &mut bool {
        &mut self.frustum_cull_enable
    }
    fn occlusion_cull_enable(&self) -> &bool {
        &self.occlusion_cull_enable
    }
    fn occlusion_cull_enable_mut(&mut self) -> &mut bool {
        &mut self.occlusion_cull_enable
    }
    fn distance_cull_falloff(&self) -> &f32 {
        &self.distance_cull_falloff
    }
    fn distance_cull_falloff_mut(&mut self) -> &mut f32 {
        &mut self.distance_cull_falloff
    }
    fn min_occlusion_test_distance(&self) -> &f32 {
        &self.min_occlusion_test_distance
    }
    fn min_occlusion_test_distance_mut(&mut self) -> &mut f32 {
        &mut self.min_occlusion_test_distance
    }
    fn min_occlusion_screen_area(&self) -> &f32 {
        &self.min_occlusion_screen_area
    }
    fn min_occlusion_screen_area_mut(&mut self) -> &mut f32 {
        &mut self.min_occlusion_screen_area
    }
    fn debug_occlusion_cull_enable(&self) -> &bool {
        &self.debug_occlusion_cull_enable
    }
    fn debug_occlusion_cull_enable_mut(&mut self) -> &mut bool {
        &mut self.debug_occlusion_cull_enable
    }
    fn debug_batches(&self) -> &bool {
        &self.debug_batches
    }
    fn debug_batches_mut(&mut self) -> &mut bool {
        &mut self.debug_batches
    }
    fn debug_per_entity_batches(&self) -> &i32 {
        &self.debug_per_entity_batches
    }
    fn debug_per_entity_batches_mut(&mut self) -> &mut i32 {
        &mut self.debug_per_entity_batches
    }
    fn wireframe_enable(&self) -> &bool {
        &self.wireframe_enable
    }
    fn wireframe_enable_mut(&mut self) -> &mut bool {
        &mut self.wireframe_enable
    }
    fn debug_ringbuffer(&self) -> &i32 {
        &self.debug_ringbuffer
    }
    fn debug_ringbuffer_mut(&mut self) -> &mut i32 {
        &mut self.debug_ringbuffer
    }
    fn displacement_bias(&self) -> &f32 {
        &self.displacement_bias
    }
    fn displacement_bias_mut(&mut self) -> &mut f32 {
        &mut self.displacement_bias
    }
    fn displacement_scale(&self) -> &f32 {
        &self.displacement_scale
    }
    fn displacement_scale_mut(&mut self) -> &mut f32 {
        &mut self.displacement_scale
    }
    fn max_decal_object_prims(&self) -> &u32 {
        &self.max_decal_object_prims
    }
    fn max_decal_object_prims_mut(&mut self) -> &mut u32 {
        &mut self.max_decal_object_prims
    }
    fn max_total_decal_object_prims(&self) -> &u32 {
        &self.max_total_decal_object_prims
    }
    fn max_total_decal_object_prims_mut(&mut self) -> &mut u32 {
        &mut self.max_total_decal_object_prims
    }
    fn decal_max_distance(&self) -> &f32 {
        &self.decal_max_distance
    }
    fn decal_max_distance_mut(&mut self) -> &mut f32 {
        &mut self.decal_max_distance
    }
}

impl super::core::SystemSettingsTrait for DecalSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for DecalSettings {
}

pub static DECALSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalSettings",
    name_hash: 1733412079,
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(DecalSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DecalSettings as Default>::default())),
            create_boxed: || Box::new(<DecalSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, enable),
            },
            FieldInfoData {
                name: "DrawEnable",
                name_hash: 1347356004,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, draw_enable),
            },
            FieldInfoData {
                name: "RingBufferSizeInVerts",
                name_hash: 2513445491,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, ring_buffer_size_in_verts),
            },
            FieldInfoData {
                name: "RingBufferRecreateSizeInVerts",
                name_hash: 1462915808,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, ring_buffer_recreate_size_in_verts),
            },
            FieldInfoData {
                name: "ProjectedDecalsTrianglesPerJob",
                name_hash: 2687160400,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, projected_decals_triangles_per_job),
            },
            FieldInfoData {
                name: "DecalRecreateDistanceInMeters",
                name_hash: 2976932869,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, decal_recreate_distance_in_meters),
            },
            FieldInfoData {
                name: "DebugDrawEntityHandles",
                name_hash: 1658780438,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, debug_draw_entity_handles),
            },
            FieldInfoData {
                name: "CullEnable",
                name_hash: 4089010994,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, cull_enable),
            },
            FieldInfoData {
                name: "DistanceCullEnable",
                name_hash: 130648241,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, distance_cull_enable),
            },
            FieldInfoData {
                name: "FrustumCullEnable",
                name_hash: 2176884108,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, frustum_cull_enable),
            },
            FieldInfoData {
                name: "OcclusionCullEnable",
                name_hash: 2058141375,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, occlusion_cull_enable),
            },
            FieldInfoData {
                name: "DistanceCullFalloff",
                name_hash: 122571384,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, distance_cull_falloff),
            },
            FieldInfoData {
                name: "MinOcclusionTestDistance",
                name_hash: 1538423831,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, min_occlusion_test_distance),
            },
            FieldInfoData {
                name: "MinOcclusionScreenArea",
                name_hash: 3772538969,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, min_occlusion_screen_area),
            },
            FieldInfoData {
                name: "DebugOcclusionCullEnable",
                name_hash: 2449547886,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, debug_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "DebugBatches",
                name_hash: 1035877342,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, debug_batches),
            },
            FieldInfoData {
                name: "DebugPerEntityBatches",
                name_hash: 318620898,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DecalSettings, debug_per_entity_batches),
            },
            FieldInfoData {
                name: "WireframeEnable",
                name_hash: 1610721584,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DecalSettings, wireframe_enable),
            },
            FieldInfoData {
                name: "DebugRingbuffer",
                name_hash: 683628902,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DecalSettings, debug_ringbuffer),
            },
            FieldInfoData {
                name: "DisplacementBias",
                name_hash: 3303605963,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, displacement_bias),
            },
            FieldInfoData {
                name: "DisplacementScale",
                name_hash: 1624443178,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DecalSettings, displacement_scale),
            },
            FieldInfoData {
                name: "MaxDecalObjectPrims",
                name_hash: 686281246,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, max_decal_object_prims),
            },
            FieldInfoData {
                name: "MaxTotalDecalObjectPrims",
                name_hash: 3297560700,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DecalSettings, max_total_decal_object_prims),
            },
            FieldInfoData {
                name: "DecalMaxDistance",
                name_hash: 2380052509,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DECALSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalSettings-Array",
    name_hash: 2232623835,
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalSettings"),
    array_type: None,
    alignment: 8,
};


