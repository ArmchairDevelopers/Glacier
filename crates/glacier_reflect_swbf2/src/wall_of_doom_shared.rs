use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_wall_of_doom_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(WALLOFDOOMMESHENTITYDATA_TYPE_INFO);
    registry.register_type(WALLOFDOOMMESHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WALLOFDOOMENTITYDATA_TYPE_INFO);
    registry.register_type(WALLOFDOOMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(WALLOFDOOMHEIGHTMAPMETADATA_TYPE_INFO);
    registry.register_type(WALLOFDOOMHEIGHTMAPMETADATA_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct WallOfDoomMeshEntityData {
    pub _glacier_base: super::game_shared::StaticModelEntityData,
    pub show_curvature: bool,
    pub vertex_perturbation_texture: Option<LockedTypeObject /* super::render::TextureAsset */>,
    pub perturbation_max_scale: f32,
    pub perturbation_max_scale_height: f32,
    pub perturbation_min_height: f32,
}

pub trait WallOfDoomMeshEntityDataTrait: super::game_shared::StaticModelEntityDataTrait {
    fn show_curvature(&self) -> &bool;
    fn show_curvature_mut(&mut self) -> &mut bool;
    fn vertex_perturbation_texture(&self) -> &Option<LockedTypeObject /* super::render::TextureAsset */>;
    fn vertex_perturbation_texture_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render::TextureAsset */>;
    fn perturbation_max_scale(&self) -> &f32;
    fn perturbation_max_scale_mut(&mut self) -> &mut f32;
    fn perturbation_max_scale_height(&self) -> &f32;
    fn perturbation_max_scale_height_mut(&mut self) -> &mut f32;
    fn perturbation_min_height(&self) -> &f32;
    fn perturbation_min_height_mut(&mut self) -> &mut f32;
}

impl WallOfDoomMeshEntityDataTrait for WallOfDoomMeshEntityData {
    fn show_curvature(&self) -> &bool {
        &self.show_curvature
    }
    fn show_curvature_mut(&mut self) -> &mut bool {
        &mut self.show_curvature
    }
    fn vertex_perturbation_texture(&self) -> &Option<LockedTypeObject /* super::render::TextureAsset */> {
        &self.vertex_perturbation_texture
    }
    fn vertex_perturbation_texture_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render::TextureAsset */> {
        &mut self.vertex_perturbation_texture
    }
    fn perturbation_max_scale(&self) -> &f32 {
        &self.perturbation_max_scale
    }
    fn perturbation_max_scale_mut(&mut self) -> &mut f32 {
        &mut self.perturbation_max_scale
    }
    fn perturbation_max_scale_height(&self) -> &f32 {
        &self.perturbation_max_scale_height
    }
    fn perturbation_max_scale_height_mut(&mut self) -> &mut f32 {
        &mut self.perturbation_max_scale_height
    }
    fn perturbation_min_height(&self) -> &f32 {
        &self.perturbation_min_height
    }
    fn perturbation_min_height_mut(&mut self) -> &mut f32 {
        &mut self.perturbation_min_height
    }
}

impl super::game_shared::StaticModelEntityDataTrait for WallOfDoomMeshEntityData {
    fn part_links(&self) -> &Vec<Option<LockedTypeObject /* super::gameplay_sim::PartLinkData */>> {
        self._glacier_base.part_links()
    }
    fn part_links_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::gameplay_sim::PartLinkData */>> {
        self._glacier_base.part_links_mut()
    }
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh()
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh_mut()
    }
    fn base_pose_transforms(&self) -> &super::core::SparseTransformArray {
        self._glacier_base.base_pose_transforms()
    }
    fn base_pose_transforms_mut(&mut self) -> &mut super::core::SparseTransformArray {
        self._glacier_base.base_pose_transforms_mut()
    }
    fn exclude_from_nearby_object_destruction(&self) -> &bool {
        self._glacier_base.exclude_from_nearby_object_destruction()
    }
    fn exclude_from_nearby_object_destruction_mut(&mut self) -> &mut bool {
        self._glacier_base.exclude_from_nearby_object_destruction_mut()
    }
    fn physics_part_infos(&self) -> &Vec<BoxedTypeObject /* super::game_shared::PhysicsPartInfo */> {
        self._glacier_base.physics_part_infos()
    }
    fn physics_part_infos_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::game_shared::PhysicsPartInfo */> {
        self._glacier_base.physics_part_infos_mut()
    }
    fn network_info(&self) -> &super::game_shared::StaticModelNetworkInfo {
        self._glacier_base.network_info()
    }
    fn network_info_mut(&mut self) -> &mut super::game_shared::StaticModelNetworkInfo {
        self._glacier_base.network_info_mut()
    }
    fn animate_physics(&self) -> &bool {
        self._glacier_base.animate_physics()
    }
    fn animate_physics_mut(&mut self) -> &mut bool {
        self._glacier_base.animate_physics_mut()
    }
    fn terrain_shader_nodes_enable(&self) -> &bool {
        self._glacier_base.terrain_shader_nodes_enable()
    }
    fn terrain_shader_nodes_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.terrain_shader_nodes_enable_mut()
    }
    fn explosion_packs_attachable(&self) -> &bool {
        self._glacier_base.explosion_packs_attachable()
    }
    fn explosion_packs_attachable_mut(&mut self) -> &mut bool {
        self._glacier_base.explosion_packs_attachable_mut()
    }
    fn light_map_weight(&self) -> &f32 {
        self._glacier_base.light_map_weight()
    }
    fn light_map_weight_mut(&mut self) -> &mut f32 {
        self._glacier_base.light_map_weight_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn overlay_enabled(&self) -> &bool {
        self._glacier_base.overlay_enabled()
    }
    fn overlay_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.overlay_enabled_mut()
    }
}

impl super::physics::GamePhysicsEntityDataTrait for WallOfDoomMeshEntityData {
}

impl super::entity::GameComponentEntityDataTrait for WallOfDoomMeshEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for WallOfDoomMeshEntityData {
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn part_bounding_boxes(&self) -> &Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes()
    }
    fn part_bounding_boxes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes_mut()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn client_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_component_count_mut()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn server_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_component_count_mut()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn client_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_transformation_count_mut()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
    fn server_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_transformation_count_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for WallOfDoomMeshEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for WallOfDoomMeshEntityData {
}

impl super::entity::GameObjectDataTrait for WallOfDoomMeshEntityData {
}

impl super::core::DataBusPeerTrait for WallOfDoomMeshEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WallOfDoomMeshEntityData {
}

impl super::core::DataContainerTrait for WallOfDoomMeshEntityData {
}

pub static WALLOFDOOMMESHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomMeshEntityData",
    name_hash: 4237316043,
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoomShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared::STATICMODELENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(WallOfDoomMeshEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WallOfDoomMeshEntityData as Default>::default())),
            create_boxed: || Box::new(<WallOfDoomMeshEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShowCurvature",
                name_hash: 4239439619,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WallOfDoomMeshEntityData, show_curvature),
            },
            FieldInfoData {
                name: "VertexPerturbationTexture",
                name_hash: 79070233,
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAsset",
                rust_offset: offset_of!(WallOfDoomMeshEntityData, vertex_perturbation_texture),
            },
            FieldInfoData {
                name: "PerturbationMaxScale",
                name_hash: 825691554,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomMeshEntityData, perturbation_max_scale),
            },
            FieldInfoData {
                name: "PerturbationMaxScaleHeight",
                name_hash: 3768414653,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomMeshEntityData, perturbation_max_scale_height),
            },
            FieldInfoData {
                name: "PerturbationMinHeight",
                name_hash: 3524828187,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomMeshEntityData, perturbation_min_height),
            },
        ],
    }),
    array_type: Some(WALLOFDOOMMESHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WallOfDoomMeshEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WALLOFDOOMMESHENTITYDATA_TYPE_INFO
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


pub static WALLOFDOOMMESHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomMeshEntityData-Array",
    name_hash: 281947903,
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoomShared",
    data: TypeInfoData::Array("WallOfDoomMeshEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WallOfDoomEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub wall_of_doom_blueprint: Option<LockedTypeObject /* super::entity::ObjectBlueprint */>,
    pub wall_of_doom_blend_blueprint: Option<LockedTypeObject /* super::entity::ObjectBlueprint */>,
    pub mesh_segment_length: f32,
    pub mesh_segment_height: f32,
    pub mesh_blending_segment_length: f32,
    pub radius: f32,
    pub center: super::core::Vec3,
    pub min_max_pairs: Vec<f32>,
    pub meta_data: WallOfDoomHeightmapMetaData,
    pub height_map: Option<LockedTypeObject /* super::render::TextureAsset */>,
    pub effect_parameters: Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>,
    pub wall_visible: bool,
    pub v_f_x_visible: bool,
}

pub trait WallOfDoomEntityDataTrait: super::entity::EntityDataTrait {
    fn wall_of_doom_blueprint(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn wall_of_doom_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn wall_of_doom_blend_blueprint(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn wall_of_doom_blend_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn mesh_segment_length(&self) -> &f32;
    fn mesh_segment_length_mut(&mut self) -> &mut f32;
    fn mesh_segment_height(&self) -> &f32;
    fn mesh_segment_height_mut(&mut self) -> &mut f32;
    fn mesh_blending_segment_length(&self) -> &f32;
    fn mesh_blending_segment_length_mut(&mut self) -> &mut f32;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn center(&self) -> &super::core::Vec3;
    fn center_mut(&mut self) -> &mut super::core::Vec3;
    fn min_max_pairs(&self) -> &Vec<f32>;
    fn min_max_pairs_mut(&mut self) -> &mut Vec<f32>;
    fn meta_data(&self) -> &WallOfDoomHeightmapMetaData;
    fn meta_data_mut(&mut self) -> &mut WallOfDoomHeightmapMetaData;
    fn height_map(&self) -> &Option<LockedTypeObject /* super::render::TextureAsset */>;
    fn height_map_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render::TextureAsset */>;
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>;
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>>;
    fn wall_visible(&self) -> &bool;
    fn wall_visible_mut(&mut self) -> &mut bool;
    fn v_f_x_visible(&self) -> &bool;
    fn v_f_x_visible_mut(&mut self) -> &mut bool;
}

impl WallOfDoomEntityDataTrait for WallOfDoomEntityData {
    fn wall_of_doom_blueprint(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &self.wall_of_doom_blueprint
    }
    fn wall_of_doom_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &mut self.wall_of_doom_blueprint
    }
    fn wall_of_doom_blend_blueprint(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &self.wall_of_doom_blend_blueprint
    }
    fn wall_of_doom_blend_blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &mut self.wall_of_doom_blend_blueprint
    }
    fn mesh_segment_length(&self) -> &f32 {
        &self.mesh_segment_length
    }
    fn mesh_segment_length_mut(&mut self) -> &mut f32 {
        &mut self.mesh_segment_length
    }
    fn mesh_segment_height(&self) -> &f32 {
        &self.mesh_segment_height
    }
    fn mesh_segment_height_mut(&mut self) -> &mut f32 {
        &mut self.mesh_segment_height
    }
    fn mesh_blending_segment_length(&self) -> &f32 {
        &self.mesh_blending_segment_length
    }
    fn mesh_blending_segment_length_mut(&mut self) -> &mut f32 {
        &mut self.mesh_blending_segment_length
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn center(&self) -> &super::core::Vec3 {
        &self.center
    }
    fn center_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.center
    }
    fn min_max_pairs(&self) -> &Vec<f32> {
        &self.min_max_pairs
    }
    fn min_max_pairs_mut(&mut self) -> &mut Vec<f32> {
        &mut self.min_max_pairs
    }
    fn meta_data(&self) -> &WallOfDoomHeightmapMetaData {
        &self.meta_data
    }
    fn meta_data_mut(&mut self) -> &mut WallOfDoomHeightmapMetaData {
        &mut self.meta_data
    }
    fn height_map(&self) -> &Option<LockedTypeObject /* super::render::TextureAsset */> {
        &self.height_map
    }
    fn height_map_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render::TextureAsset */> {
        &mut self.height_map
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        &self.effect_parameters
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        &mut self.effect_parameters
    }
    fn wall_visible(&self) -> &bool {
        &self.wall_visible
    }
    fn wall_visible_mut(&mut self) -> &mut bool {
        &mut self.wall_visible
    }
    fn v_f_x_visible(&self) -> &bool {
        &self.v_f_x_visible
    }
    fn v_f_x_visible_mut(&mut self) -> &mut bool {
        &mut self.v_f_x_visible
    }
}

impl super::entity::EntityDataTrait for WallOfDoomEntityData {
}

impl super::entity::GameObjectDataTrait for WallOfDoomEntityData {
}

impl super::core::DataBusPeerTrait for WallOfDoomEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WallOfDoomEntityData {
}

impl super::core::DataContainerTrait for WallOfDoomEntityData {
}

pub static WALLOFDOOMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomEntityData",
    name_hash: 836768568,
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoomShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(WallOfDoomEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WallOfDoomEntityData as Default>::default())),
            create_boxed: || Box::new(<WallOfDoomEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WallOfDoomBlueprint",
                name_hash: 1087273500,
                flags: MemberInfoFlags::new(0),
                field_type: "ObjectBlueprint",
                rust_offset: offset_of!(WallOfDoomEntityData, wall_of_doom_blueprint),
            },
            FieldInfoData {
                name: "WallOfDoomBlendBlueprint",
                name_hash: 1897302173,
                flags: MemberInfoFlags::new(0),
                field_type: "ObjectBlueprint",
                rust_offset: offset_of!(WallOfDoomEntityData, wall_of_doom_blend_blueprint),
            },
            FieldInfoData {
                name: "MeshSegmentLength",
                name_hash: 798323401,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomEntityData, mesh_segment_length),
            },
            FieldInfoData {
                name: "MeshSegmentHeight",
                name_hash: 945562474,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomEntityData, mesh_segment_height),
            },
            FieldInfoData {
                name: "MeshBlendingSegmentLength",
                name_hash: 2856462632,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomEntityData, mesh_blending_segment_length),
            },
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomEntityData, radius),
            },
            FieldInfoData {
                name: "Center",
                name_hash: 2711667502,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WallOfDoomEntityData, center),
            },
            FieldInfoData {
                name: "MinMaxPairs",
                name_hash: 2753167810,
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(WallOfDoomEntityData, min_max_pairs),
            },
            FieldInfoData {
                name: "MetaData",
                name_hash: 1407638056,
                flags: MemberInfoFlags::new(0),
                field_type: "WallOfDoomHeightmapMetaData",
                rust_offset: offset_of!(WallOfDoomEntityData, meta_data),
            },
            FieldInfoData {
                name: "HeightMap",
                name_hash: 362187110,
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAsset",
                rust_offset: offset_of!(WallOfDoomEntityData, height_map),
            },
            FieldInfoData {
                name: "EffectParameters",
                name_hash: 929782248,
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(WallOfDoomEntityData, effect_parameters),
            },
            FieldInfoData {
                name: "WallVisible",
                name_hash: 4222142269,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WallOfDoomEntityData, wall_visible),
            },
            FieldInfoData {
                name: "VFXVisible",
                name_hash: 2564483459,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WallOfDoomEntityData, v_f_x_visible),
            },
        ],
    }),
    array_type: Some(WALLOFDOOMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WallOfDoomEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        WALLOFDOOMENTITYDATA_TYPE_INFO
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


pub static WALLOFDOOMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomEntityData-Array",
    name_hash: 900889740,
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoomShared",
    data: TypeInfoData::Array("WallOfDoomEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WallOfDoomHeightmapMetaData {
    pub height_map_width: i32,
    pub height_map_height: i32,
    pub world_extents_min_x: f32,
    pub world_extents_min_y: f32,
    pub world_extents_min_z: f32,
    pub world_extents_max_x: f32,
    pub world_extents_max_y: f32,
    pub world_extents_max_z: f32,
    pub min_max_texture_ratio: i32,
    pub min_max_data_width: i32,
    pub min_max_data_height: i32,
}

pub trait WallOfDoomHeightmapMetaDataTrait: TypeObject {
    fn height_map_width(&self) -> &i32;
    fn height_map_width_mut(&mut self) -> &mut i32;
    fn height_map_height(&self) -> &i32;
    fn height_map_height_mut(&mut self) -> &mut i32;
    fn world_extents_min_x(&self) -> &f32;
    fn world_extents_min_x_mut(&mut self) -> &mut f32;
    fn world_extents_min_y(&self) -> &f32;
    fn world_extents_min_y_mut(&mut self) -> &mut f32;
    fn world_extents_min_z(&self) -> &f32;
    fn world_extents_min_z_mut(&mut self) -> &mut f32;
    fn world_extents_max_x(&self) -> &f32;
    fn world_extents_max_x_mut(&mut self) -> &mut f32;
    fn world_extents_max_y(&self) -> &f32;
    fn world_extents_max_y_mut(&mut self) -> &mut f32;
    fn world_extents_max_z(&self) -> &f32;
    fn world_extents_max_z_mut(&mut self) -> &mut f32;
    fn min_max_texture_ratio(&self) -> &i32;
    fn min_max_texture_ratio_mut(&mut self) -> &mut i32;
    fn min_max_data_width(&self) -> &i32;
    fn min_max_data_width_mut(&mut self) -> &mut i32;
    fn min_max_data_height(&self) -> &i32;
    fn min_max_data_height_mut(&mut self) -> &mut i32;
}

impl WallOfDoomHeightmapMetaDataTrait for WallOfDoomHeightmapMetaData {
    fn height_map_width(&self) -> &i32 {
        &self.height_map_width
    }
    fn height_map_width_mut(&mut self) -> &mut i32 {
        &mut self.height_map_width
    }
    fn height_map_height(&self) -> &i32 {
        &self.height_map_height
    }
    fn height_map_height_mut(&mut self) -> &mut i32 {
        &mut self.height_map_height
    }
    fn world_extents_min_x(&self) -> &f32 {
        &self.world_extents_min_x
    }
    fn world_extents_min_x_mut(&mut self) -> &mut f32 {
        &mut self.world_extents_min_x
    }
    fn world_extents_min_y(&self) -> &f32 {
        &self.world_extents_min_y
    }
    fn world_extents_min_y_mut(&mut self) -> &mut f32 {
        &mut self.world_extents_min_y
    }
    fn world_extents_min_z(&self) -> &f32 {
        &self.world_extents_min_z
    }
    fn world_extents_min_z_mut(&mut self) -> &mut f32 {
        &mut self.world_extents_min_z
    }
    fn world_extents_max_x(&self) -> &f32 {
        &self.world_extents_max_x
    }
    fn world_extents_max_x_mut(&mut self) -> &mut f32 {
        &mut self.world_extents_max_x
    }
    fn world_extents_max_y(&self) -> &f32 {
        &self.world_extents_max_y
    }
    fn world_extents_max_y_mut(&mut self) -> &mut f32 {
        &mut self.world_extents_max_y
    }
    fn world_extents_max_z(&self) -> &f32 {
        &self.world_extents_max_z
    }
    fn world_extents_max_z_mut(&mut self) -> &mut f32 {
        &mut self.world_extents_max_z
    }
    fn min_max_texture_ratio(&self) -> &i32 {
        &self.min_max_texture_ratio
    }
    fn min_max_texture_ratio_mut(&mut self) -> &mut i32 {
        &mut self.min_max_texture_ratio
    }
    fn min_max_data_width(&self) -> &i32 {
        &self.min_max_data_width
    }
    fn min_max_data_width_mut(&mut self) -> &mut i32 {
        &mut self.min_max_data_width
    }
    fn min_max_data_height(&self) -> &i32 {
        &self.min_max_data_height
    }
    fn min_max_data_height_mut(&mut self) -> &mut i32 {
        &mut self.min_max_data_height
    }
}

pub static WALLOFDOOMHEIGHTMAPMETADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomHeightmapMetaData",
    name_hash: 4239977661,
    flags: MemberInfoFlags::new(36937),
    module: "WallOfDoomShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WallOfDoomHeightmapMetaData as Default>::default())),
            create_boxed: || Box::new(<WallOfDoomHeightmapMetaData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HeightMapWidth",
                name_hash: 2307294208,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, height_map_width),
            },
            FieldInfoData {
                name: "HeightMapHeight",
                name_hash: 2925265273,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, height_map_height),
            },
            FieldInfoData {
                name: "WorldExtentsMinX",
                name_hash: 2295005424,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_min_x),
            },
            FieldInfoData {
                name: "WorldExtentsMinY",
                name_hash: 2295005425,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_min_y),
            },
            FieldInfoData {
                name: "WorldExtentsMinZ",
                name_hash: 2295005426,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_min_z),
            },
            FieldInfoData {
                name: "WorldExtentsMaxX",
                name_hash: 2295014190,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_max_x),
            },
            FieldInfoData {
                name: "WorldExtentsMaxY",
                name_hash: 2295014191,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_max_y),
            },
            FieldInfoData {
                name: "WorldExtentsMaxZ",
                name_hash: 2295014188,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_max_z),
            },
            FieldInfoData {
                name: "MinMaxTextureRatio",
                name_hash: 20073477,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, min_max_texture_ratio),
            },
            FieldInfoData {
                name: "MinMaxDataWidth",
                name_hash: 3981707181,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, min_max_data_width),
            },
            FieldInfoData {
                name: "MinMaxDataHeight",
                name_hash: 1661765556,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, min_max_data_height),
            },
        ],
    }),
    array_type: Some(WALLOFDOOMHEIGHTMAPMETADATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WallOfDoomHeightmapMetaData {
    fn type_info(&self) -> &'static TypeInfo {
        WALLOFDOOMHEIGHTMAPMETADATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static WALLOFDOOMHEIGHTMAPMETADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomHeightmapMetaData-Array",
    name_hash: 1396664585,
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoomShared",
    data: TypeInfoData::Array("WallOfDoomHeightmapMetaData"),
    array_type: None,
    alignment: 8,
};


