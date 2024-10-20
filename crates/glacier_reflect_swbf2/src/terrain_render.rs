use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_terrain_render_types(registry: &mut TypeRegistry) {
    registry.register_type(MESHSCATTERINGTREE_TYPE_INFO);
    registry.register_type(MESHSCATTERINGTREE_ARRAY_TYPE_INFO);
    registry.register_type(DISPLACEMENTTEXTURETREE_TYPE_INFO);
    registry.register_type(DISPLACEMENTTEXTURETREE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALTERRAINSETTINGS_TYPE_INFO);
    registry.register_type(VISUALTERRAINSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINRENDERMODE_TYPE_INFO);
    registry.register_type(TERRAINRENDERMODE_ARRAY_TYPE_INFO);
    registry.register_type(VISUALTERRAIN_TYPE_INFO);
    registry.register_type(VISUALTERRAIN_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONS_TYPE_INFO);
    registry.register_type(TERRAINLAYERCOMBINATIONS_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINDECALS_TYPE_INFO);
    registry.register_type(TERRAINDECALS_ARRAY_TYPE_INFO);
    registry.register_type(IVISUALTERRAIN_TYPE_INFO);
    registry.register_type(IVISUALTERRAIN_ARRAY_TYPE_INFO);
    registry.register_type(TERRAINTEXTURETREE_TYPE_INFO);
    registry.register_type(TERRAINTEXTURETREE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct MeshScatteringTree {
}

pub trait MeshScatteringTreeTrait: TypeObject {
}

impl MeshScatteringTreeTrait for MeshScatteringTree {
}

pub static MESHSCATTERINGTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshScatteringTree as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHSCATTERINGTREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshScatteringTree {
    fn type_info(&self) -> &'static TypeInfo {
        MESHSCATTERINGTREE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHSCATTERINGTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("MeshScatteringTree"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DisplacementTextureTree {
}

pub trait DisplacementTextureTreeTrait: TypeObject {
}

impl DisplacementTextureTreeTrait for DisplacementTextureTree {
}

pub static DISPLACEMENTTEXTURETREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplacementTextureTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DisplacementTextureTree as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DISPLACEMENTTEXTURETREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DisplacementTextureTree {
    fn type_info(&self) -> &'static TypeInfo {
        DISPLACEMENTTEXTURETREE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DISPLACEMENTTEXTURETREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplacementTextureTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("DisplacementTextureTree"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualTerrainSettings {
    pub _glacier_base: super::terrain_base::VisualTerrainBaseSettings,
    pub mesh_scattering_quality_level: super::core::QualityLevel,
    pub render_mode: TerrainRenderMode,
    pub wireframe_enable: bool,
    pub enable: bool,
    pub edit_service_enable: bool,
    pub triangle_size_min: f32,
    pub lod_scale: f32,
    pub lod_center_extrapolation_distance_max: f32,
    pub lod_center_extrapolation_time: f32,
    pub texture_skip_mip_speed: f32,
    pub tessellation_enable: bool,
    pub tessellation_for_reflections_enable: bool,
    pub tessellation_for_shadows_enable: bool,
    pub detail_displacement_for_reflections_enable: bool,
    pub detail_displacement_for_shadows_enable: bool,
    pub tessellation_patch_shrink: f32,
    pub tessellation_patch_faces_per_side: u32,
    pub tessellated_tri_width: f32,
    pub tessellated_tri_width_scale_for_reflections: f32,
    pub tessellated_tri_width_scale_for_shadows: f32,
    pub additional_cull_frustum_enable: bool,
    pub density_map_enable: bool,
    pub vertex_buffer_heights_enable: bool,
    pub draw_vertex_y_texture_enable: bool,
    pub texture_atlas_sample_count_x_factor: u32,
    pub texture_atlas_sample_count_y_factor: u32,
    pub texture_samples_per_meter_max: f32,
    pub texture_detail_falloff_factor: f32,
    pub texture_detail_falloff_distance: f32,
    pub texture_detail_falloff_curve: f32,
    pub texture_invisible_detail_reduction_factor: f32,
    pub texture_occluded_detail_reduction_factor: f32,
    pub texture_render_job_count: u32,
    pub real_time_edit_mode_texture_render_job_count: u32,
    pub texture_vt_indirection_job_enable: bool,
    pub texture_streaming_prioritization_job_enable: bool,
    pub texture_render_jobs_launched_per_frame_count_max: u32,
    pub real_time_edit_mode_texture_render_jobs_launched_per_frame_count_max: u32,
    pub texture_tile_samples_per_side: u32,
    pub texture_tile_border_width: u32,
    pub texture_level_offset: i32,
    pub texture_clod_frame_count: u32,
    pub texture_clod_enable: bool,
    pub texture_clod_cutoff_priority: f32,
    pub texture_force_update_enable: bool,
    pub texture_streamable_texture_instance_buffer_size: u32,
    pub texture_compress_job_count: u32,
    pub texture_compress_fast_algorithm_enable: bool,
    pub texture_compression_quality: i32,
    pub texture_detail_slope_boost: f32,
    pub texture_generation_mip_bias: f32,
    pub draw_texture_debug_colors: bool,
    pub texture_draw_terrain_layers_enable: bool,
    pub texture_keep_pool_full_enable: bool,
    pub texture_layer_culling_enable: bool,
    pub gpu_texture_compression_enable: bool,
    pub texture_dirty_retry_rate: f32,
    pub texture_force_draw_pass: i32,
    pub use_v_t_max_level_as_patch_level: bool,
    pub texture_streaming_prioritization_enable: bool,
    pub detail_texture_streaming_prioritization_enable: bool,
    pub mesh_scattering_mesh_streaming_prioritization_enable: bool,
    pub texture_quads_per_tile_level: u32,
    pub prioritization_occlusion_enable: bool,
    pub draw_enable: bool,
    pub draw_patches_enable: bool,
    pub detail_overlay_enable: bool,
    pub draw_instancing_stats: bool,
    pub decal_enable: bool,
    pub force_decal_reduced_quality: bool,
    pub draw_decal2d_enable: bool,
    pub draw_decal3d_enable: bool,
    pub draw_decal_z_pass_enable: bool,
    pub draw_only_decal_z_pass_enable: bool,
    pub decal_z_pass_draw_distance: f32,
    pub decal_offset_y: f32,
    pub decal3d_far_draw_distance_scale_factor: f32,
    pub draw_patch_boxes_enable: bool,
    pub draw_bad_patches_enable: bool,
    pub draw_texture_tile_boxes_enable: bool,
    pub draw_debug_text_enable: bool,
    pub draw_debug_textures_enable: bool,
    pub draw_quadtrees_enable: bool,
    pub draw_quadtree_zoom_index: i32,
    pub draw_quadtree_stats_enable: bool,
    pub draw_quadtree_atlas_textures_enable: bool,
    pub draw_quadtree_atlas_textures_scale: f32,
    pub draw_indirection_textures_enable: bool,
    pub quickscope_pool_stats_enable: bool,
    pub draw_dynamic_mask: bool,
    pub draw_debug_tile_priority_quadtree_enable: bool,
    pub draw_water_enable: bool,
    pub patch_error_fov_enable: bool,
    pub patch_error_fov: f32,
    pub z_pass_distance: f32,
    pub debug_overlay_grid_enable: bool,
    pub debug_overlay_grid_size: f32,
    pub debug_overlay_isolines_enable: bool,
    pub debug_overlay_isoline_spacing: f32,
    pub debug_overlay_wireframe_enable: bool,
    pub debug_overlay_sketch_texture_enable: bool,
    pub debug_overlay_brush_enable: bool,
    pub force_graphics_driver_crash: bool,
    pub force_patch_rebuild_enable: bool,
    pub destroy_all: bool,
    pub slot_reuse_wait_count: u32,
    pub slot_debug_output_enable: bool,
    pub update_jobs_enable: bool,
    pub build_job_count: u32,
    pub regenerate_textures_enable: bool,
    pub dynamic_mask_enable: bool,
    pub max_non_visible_texture_update_count: u32,
    pub patch_faces_per_side: u32,
    pub tessellation_faces_per_side_min: u32,
    pub patch_slot_count: u32,
    pub patch_lod_transitions_enable: bool,
    pub patch_lod_transitions_vertex_shader_enable: bool,
    pub patch_lod_transitions_vertex_shader_disable_fixup: bool,
    pub patch_material_sorting_enable: bool,
    pub cull_sample_bounding_box_height_enable: bool,
    pub force_shadows_off: bool,
    pub terrain_cast_shadows_quality_level: super::core::QualityLevel,
    pub cast_planar_reflection_enable: bool,
    pub cast_envmap_reflection_enable: bool,
    pub cast_decal3d_planar_reflection_enable: bool,
    pub cast_decal3d_envmap_reflection_enable: bool,
    pub detail_displacement_in_shadow_view_enable: bool,
    pub global_colormap_enable: bool,
    pub occluder_enable: bool,
    pub occluder_job_enable: bool,
    pub occluder_job_count: u32,
    pub occluder_patch_faces_per_side: u32,
    pub occluder_lod_scale: f32,
    pub occluder_back_face_culling_enable: bool,
    pub occluded_enable: bool,
    pub occluded_min_distance: f32,
    pub mesh_scattering_enable: bool,
    pub mesh_scattering_reflections_enable: bool,
    pub mesh_scattering_jobs_enable: bool,
    pub mesh_scattering_cast_shadows_enable: bool,
    pub draw_mesh_scattering_enable: bool,
    pub draw_mesh_scattering_cell_boxes_enable: bool,
    pub draw_mesh_scattering_batch_boxes_enable: bool,
    pub draw_mesh_scattering_node_boxes_enable: bool,
    pub draw_mesh_scattering_culled_cell_boxes_enable: bool,
    pub draw_mesh_scattering_debug_mask_scale_textures_enable: bool,
    pub draw_mesh_scattering_stats_enable: bool,
    pub draw_mesh_scattering_quadtree_enable: bool,
    pub mesh_scattering_cell_pool_capacity: u32,
    pub mesh_scattering_tree_node_pool_capacity: u32,
    pub mesh_scattering_invisible_cell_fov_factor: f32,
    pub mesh_scattering_force_update_enable: bool,
    pub mesh_scattering_cull_record_count: u32,
    pub mesh_scattering_build_channel_count: u32,
    pub mesh_scattering_build_channels_launched_per_frame_count_max: u32,
    pub mesh_scattering_build_visible_first: bool,
    pub mesh_scattering_clod_frame_count: u32,
    pub mesh_scattering_wind_speed: f32,
    pub mesh_scattering_instances_per_cell_max: u32,
    pub mesh_scattering_density_margin_factor: f32,
    pub mesh_scattering_pregeneration_distance_ratio: f32,
    pub mesh_scattering_keep_distance_ratio: f32,
    pub mesh_scattering_merge_instance_lists: bool,
    pub mesh_scattering_virtual_texture_blurriness: i32,
    pub mesh_scattering_shadow_view_cull_size: f32,
    pub mesh_scattering_distance_scale_factor: f32,
    pub mesh_scattering_instance_cull_job_count: u32,
    pub mesh_scattering_instance_cull_list_count: u32,
    pub mesh_scattering_instance_cull_box_test_enable: bool,
    pub mesh_scattering_instance_frustum_cull_enable: bool,
    pub mesh_scattering_instance_occlusion_cull_enable: bool,
    pub mesh_scattering_instance_additional_cull_enable: bool,
    pub draw_mesh_scattering_instance_boxes_enable: bool,
    pub mesh_scattering_instance_cull_dynamic_alloc_enable: bool,
    pub mesh_scattering_wind_enable: bool,
    pub mesh_scattering_draw_motion_vectors_enable: bool,
    pub mesh_scattering_snapping_grid_multiplier_vertical: f32,
    pub mesh_scattering_snapping_grid_multiplier_horizontal: f32,
    pub detail_displacement_quality_level: super::core::QualityLevel,
    pub detail_displacement_enable: bool,
    pub draw_detail_displacement_enable: bool,
    pub draw_debug_detail_displacement_enable: bool,
    pub draw_detail_displacement_tree_level: i32,
    pub detail_displacement_max_tess_factor: f32,
    pub detail_displacement_scale: f32,
    pub detail_displacement_bias: f32,
    pub detail_displacement_fade_range: f32,
    pub height_field_tessellation_factor: f32,
    pub height_field_tessellation_fade_range: f32,
}

pub trait VisualTerrainSettingsTrait: super::terrain_base::VisualTerrainBaseSettingsTrait {
    fn mesh_scattering_quality_level(&self) -> &super::core::QualityLevel;
    fn render_mode(&self) -> &TerrainRenderMode;
    fn wireframe_enable(&self) -> &bool;
    fn enable(&self) -> &bool;
    fn edit_service_enable(&self) -> &bool;
    fn triangle_size_min(&self) -> &f32;
    fn lod_scale(&self) -> &f32;
    fn lod_center_extrapolation_distance_max(&self) -> &f32;
    fn lod_center_extrapolation_time(&self) -> &f32;
    fn texture_skip_mip_speed(&self) -> &f32;
    fn tessellation_enable(&self) -> &bool;
    fn tessellation_for_reflections_enable(&self) -> &bool;
    fn tessellation_for_shadows_enable(&self) -> &bool;
    fn detail_displacement_for_reflections_enable(&self) -> &bool;
    fn detail_displacement_for_shadows_enable(&self) -> &bool;
    fn tessellation_patch_shrink(&self) -> &f32;
    fn tessellation_patch_faces_per_side(&self) -> &u32;
    fn tessellated_tri_width(&self) -> &f32;
    fn tessellated_tri_width_scale_for_reflections(&self) -> &f32;
    fn tessellated_tri_width_scale_for_shadows(&self) -> &f32;
    fn additional_cull_frustum_enable(&self) -> &bool;
    fn density_map_enable(&self) -> &bool;
    fn vertex_buffer_heights_enable(&self) -> &bool;
    fn draw_vertex_y_texture_enable(&self) -> &bool;
    fn texture_atlas_sample_count_x_factor(&self) -> &u32;
    fn texture_atlas_sample_count_y_factor(&self) -> &u32;
    fn texture_samples_per_meter_max(&self) -> &f32;
    fn texture_detail_falloff_factor(&self) -> &f32;
    fn texture_detail_falloff_distance(&self) -> &f32;
    fn texture_detail_falloff_curve(&self) -> &f32;
    fn texture_invisible_detail_reduction_factor(&self) -> &f32;
    fn texture_occluded_detail_reduction_factor(&self) -> &f32;
    fn texture_render_job_count(&self) -> &u32;
    fn real_time_edit_mode_texture_render_job_count(&self) -> &u32;
    fn texture_vt_indirection_job_enable(&self) -> &bool;
    fn texture_streaming_prioritization_job_enable(&self) -> &bool;
    fn texture_render_jobs_launched_per_frame_count_max(&self) -> &u32;
    fn real_time_edit_mode_texture_render_jobs_launched_per_frame_count_max(&self) -> &u32;
    fn texture_tile_samples_per_side(&self) -> &u32;
    fn texture_tile_border_width(&self) -> &u32;
    fn texture_level_offset(&self) -> &i32;
    fn texture_clod_frame_count(&self) -> &u32;
    fn texture_clod_enable(&self) -> &bool;
    fn texture_clod_cutoff_priority(&self) -> &f32;
    fn texture_force_update_enable(&self) -> &bool;
    fn texture_streamable_texture_instance_buffer_size(&self) -> &u32;
    fn texture_compress_job_count(&self) -> &u32;
    fn texture_compress_fast_algorithm_enable(&self) -> &bool;
    fn texture_compression_quality(&self) -> &i32;
    fn texture_detail_slope_boost(&self) -> &f32;
    fn texture_generation_mip_bias(&self) -> &f32;
    fn draw_texture_debug_colors(&self) -> &bool;
    fn texture_draw_terrain_layers_enable(&self) -> &bool;
    fn texture_keep_pool_full_enable(&self) -> &bool;
    fn texture_layer_culling_enable(&self) -> &bool;
    fn gpu_texture_compression_enable(&self) -> &bool;
    fn texture_dirty_retry_rate(&self) -> &f32;
    fn texture_force_draw_pass(&self) -> &i32;
    fn use_v_t_max_level_as_patch_level(&self) -> &bool;
    fn texture_streaming_prioritization_enable(&self) -> &bool;
    fn detail_texture_streaming_prioritization_enable(&self) -> &bool;
    fn mesh_scattering_mesh_streaming_prioritization_enable(&self) -> &bool;
    fn texture_quads_per_tile_level(&self) -> &u32;
    fn prioritization_occlusion_enable(&self) -> &bool;
    fn draw_enable(&self) -> &bool;
    fn draw_patches_enable(&self) -> &bool;
    fn detail_overlay_enable(&self) -> &bool;
    fn draw_instancing_stats(&self) -> &bool;
    fn decal_enable(&self) -> &bool;
    fn force_decal_reduced_quality(&self) -> &bool;
    fn draw_decal2d_enable(&self) -> &bool;
    fn draw_decal3d_enable(&self) -> &bool;
    fn draw_decal_z_pass_enable(&self) -> &bool;
    fn draw_only_decal_z_pass_enable(&self) -> &bool;
    fn decal_z_pass_draw_distance(&self) -> &f32;
    fn decal_offset_y(&self) -> &f32;
    fn decal3d_far_draw_distance_scale_factor(&self) -> &f32;
    fn draw_patch_boxes_enable(&self) -> &bool;
    fn draw_bad_patches_enable(&self) -> &bool;
    fn draw_texture_tile_boxes_enable(&self) -> &bool;
    fn draw_debug_text_enable(&self) -> &bool;
    fn draw_debug_textures_enable(&self) -> &bool;
    fn draw_quadtrees_enable(&self) -> &bool;
    fn draw_quadtree_zoom_index(&self) -> &i32;
    fn draw_quadtree_stats_enable(&self) -> &bool;
    fn draw_quadtree_atlas_textures_enable(&self) -> &bool;
    fn draw_quadtree_atlas_textures_scale(&self) -> &f32;
    fn draw_indirection_textures_enable(&self) -> &bool;
    fn quickscope_pool_stats_enable(&self) -> &bool;
    fn draw_dynamic_mask(&self) -> &bool;
    fn draw_debug_tile_priority_quadtree_enable(&self) -> &bool;
    fn draw_water_enable(&self) -> &bool;
    fn patch_error_fov_enable(&self) -> &bool;
    fn patch_error_fov(&self) -> &f32;
    fn z_pass_distance(&self) -> &f32;
    fn debug_overlay_grid_enable(&self) -> &bool;
    fn debug_overlay_grid_size(&self) -> &f32;
    fn debug_overlay_isolines_enable(&self) -> &bool;
    fn debug_overlay_isoline_spacing(&self) -> &f32;
    fn debug_overlay_wireframe_enable(&self) -> &bool;
    fn debug_overlay_sketch_texture_enable(&self) -> &bool;
    fn debug_overlay_brush_enable(&self) -> &bool;
    fn force_graphics_driver_crash(&self) -> &bool;
    fn force_patch_rebuild_enable(&self) -> &bool;
    fn destroy_all(&self) -> &bool;
    fn slot_reuse_wait_count(&self) -> &u32;
    fn slot_debug_output_enable(&self) -> &bool;
    fn update_jobs_enable(&self) -> &bool;
    fn build_job_count(&self) -> &u32;
    fn regenerate_textures_enable(&self) -> &bool;
    fn dynamic_mask_enable(&self) -> &bool;
    fn max_non_visible_texture_update_count(&self) -> &u32;
    fn patch_faces_per_side(&self) -> &u32;
    fn tessellation_faces_per_side_min(&self) -> &u32;
    fn patch_slot_count(&self) -> &u32;
    fn patch_lod_transitions_enable(&self) -> &bool;
    fn patch_lod_transitions_vertex_shader_enable(&self) -> &bool;
    fn patch_lod_transitions_vertex_shader_disable_fixup(&self) -> &bool;
    fn patch_material_sorting_enable(&self) -> &bool;
    fn cull_sample_bounding_box_height_enable(&self) -> &bool;
    fn force_shadows_off(&self) -> &bool;
    fn terrain_cast_shadows_quality_level(&self) -> &super::core::QualityLevel;
    fn cast_planar_reflection_enable(&self) -> &bool;
    fn cast_envmap_reflection_enable(&self) -> &bool;
    fn cast_decal3d_planar_reflection_enable(&self) -> &bool;
    fn cast_decal3d_envmap_reflection_enable(&self) -> &bool;
    fn detail_displacement_in_shadow_view_enable(&self) -> &bool;
    fn global_colormap_enable(&self) -> &bool;
    fn occluder_enable(&self) -> &bool;
    fn occluder_job_enable(&self) -> &bool;
    fn occluder_job_count(&self) -> &u32;
    fn occluder_patch_faces_per_side(&self) -> &u32;
    fn occluder_lod_scale(&self) -> &f32;
    fn occluder_back_face_culling_enable(&self) -> &bool;
    fn occluded_enable(&self) -> &bool;
    fn occluded_min_distance(&self) -> &f32;
    fn mesh_scattering_enable(&self) -> &bool;
    fn mesh_scattering_reflections_enable(&self) -> &bool;
    fn mesh_scattering_jobs_enable(&self) -> &bool;
    fn mesh_scattering_cast_shadows_enable(&self) -> &bool;
    fn draw_mesh_scattering_enable(&self) -> &bool;
    fn draw_mesh_scattering_cell_boxes_enable(&self) -> &bool;
    fn draw_mesh_scattering_batch_boxes_enable(&self) -> &bool;
    fn draw_mesh_scattering_node_boxes_enable(&self) -> &bool;
    fn draw_mesh_scattering_culled_cell_boxes_enable(&self) -> &bool;
    fn draw_mesh_scattering_debug_mask_scale_textures_enable(&self) -> &bool;
    fn draw_mesh_scattering_stats_enable(&self) -> &bool;
    fn draw_mesh_scattering_quadtree_enable(&self) -> &bool;
    fn mesh_scattering_cell_pool_capacity(&self) -> &u32;
    fn mesh_scattering_tree_node_pool_capacity(&self) -> &u32;
    fn mesh_scattering_invisible_cell_fov_factor(&self) -> &f32;
    fn mesh_scattering_force_update_enable(&self) -> &bool;
    fn mesh_scattering_cull_record_count(&self) -> &u32;
    fn mesh_scattering_build_channel_count(&self) -> &u32;
    fn mesh_scattering_build_channels_launched_per_frame_count_max(&self) -> &u32;
    fn mesh_scattering_build_visible_first(&self) -> &bool;
    fn mesh_scattering_clod_frame_count(&self) -> &u32;
    fn mesh_scattering_wind_speed(&self) -> &f32;
    fn mesh_scattering_instances_per_cell_max(&self) -> &u32;
    fn mesh_scattering_density_margin_factor(&self) -> &f32;
    fn mesh_scattering_pregeneration_distance_ratio(&self) -> &f32;
    fn mesh_scattering_keep_distance_ratio(&self) -> &f32;
    fn mesh_scattering_merge_instance_lists(&self) -> &bool;
    fn mesh_scattering_virtual_texture_blurriness(&self) -> &i32;
    fn mesh_scattering_shadow_view_cull_size(&self) -> &f32;
    fn mesh_scattering_distance_scale_factor(&self) -> &f32;
    fn mesh_scattering_instance_cull_job_count(&self) -> &u32;
    fn mesh_scattering_instance_cull_list_count(&self) -> &u32;
    fn mesh_scattering_instance_cull_box_test_enable(&self) -> &bool;
    fn mesh_scattering_instance_frustum_cull_enable(&self) -> &bool;
    fn mesh_scattering_instance_occlusion_cull_enable(&self) -> &bool;
    fn mesh_scattering_instance_additional_cull_enable(&self) -> &bool;
    fn draw_mesh_scattering_instance_boxes_enable(&self) -> &bool;
    fn mesh_scattering_instance_cull_dynamic_alloc_enable(&self) -> &bool;
    fn mesh_scattering_wind_enable(&self) -> &bool;
    fn mesh_scattering_draw_motion_vectors_enable(&self) -> &bool;
    fn mesh_scattering_snapping_grid_multiplier_vertical(&self) -> &f32;
    fn mesh_scattering_snapping_grid_multiplier_horizontal(&self) -> &f32;
    fn detail_displacement_quality_level(&self) -> &super::core::QualityLevel;
    fn detail_displacement_enable(&self) -> &bool;
    fn draw_detail_displacement_enable(&self) -> &bool;
    fn draw_debug_detail_displacement_enable(&self) -> &bool;
    fn draw_detail_displacement_tree_level(&self) -> &i32;
    fn detail_displacement_max_tess_factor(&self) -> &f32;
    fn detail_displacement_scale(&self) -> &f32;
    fn detail_displacement_bias(&self) -> &f32;
    fn detail_displacement_fade_range(&self) -> &f32;
    fn height_field_tessellation_factor(&self) -> &f32;
    fn height_field_tessellation_fade_range(&self) -> &f32;
}

impl VisualTerrainSettingsTrait for VisualTerrainSettings {
    fn mesh_scattering_quality_level(&self) -> &super::core::QualityLevel {
        &self.mesh_scattering_quality_level
    }
    fn render_mode(&self) -> &TerrainRenderMode {
        &self.render_mode
    }
    fn wireframe_enable(&self) -> &bool {
        &self.wireframe_enable
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn edit_service_enable(&self) -> &bool {
        &self.edit_service_enable
    }
    fn triangle_size_min(&self) -> &f32 {
        &self.triangle_size_min
    }
    fn lod_scale(&self) -> &f32 {
        &self.lod_scale
    }
    fn lod_center_extrapolation_distance_max(&self) -> &f32 {
        &self.lod_center_extrapolation_distance_max
    }
    fn lod_center_extrapolation_time(&self) -> &f32 {
        &self.lod_center_extrapolation_time
    }
    fn texture_skip_mip_speed(&self) -> &f32 {
        &self.texture_skip_mip_speed
    }
    fn tessellation_enable(&self) -> &bool {
        &self.tessellation_enable
    }
    fn tessellation_for_reflections_enable(&self) -> &bool {
        &self.tessellation_for_reflections_enable
    }
    fn tessellation_for_shadows_enable(&self) -> &bool {
        &self.tessellation_for_shadows_enable
    }
    fn detail_displacement_for_reflections_enable(&self) -> &bool {
        &self.detail_displacement_for_reflections_enable
    }
    fn detail_displacement_for_shadows_enable(&self) -> &bool {
        &self.detail_displacement_for_shadows_enable
    }
    fn tessellation_patch_shrink(&self) -> &f32 {
        &self.tessellation_patch_shrink
    }
    fn tessellation_patch_faces_per_side(&self) -> &u32 {
        &self.tessellation_patch_faces_per_side
    }
    fn tessellated_tri_width(&self) -> &f32 {
        &self.tessellated_tri_width
    }
    fn tessellated_tri_width_scale_for_reflections(&self) -> &f32 {
        &self.tessellated_tri_width_scale_for_reflections
    }
    fn tessellated_tri_width_scale_for_shadows(&self) -> &f32 {
        &self.tessellated_tri_width_scale_for_shadows
    }
    fn additional_cull_frustum_enable(&self) -> &bool {
        &self.additional_cull_frustum_enable
    }
    fn density_map_enable(&self) -> &bool {
        &self.density_map_enable
    }
    fn vertex_buffer_heights_enable(&self) -> &bool {
        &self.vertex_buffer_heights_enable
    }
    fn draw_vertex_y_texture_enable(&self) -> &bool {
        &self.draw_vertex_y_texture_enable
    }
    fn texture_atlas_sample_count_x_factor(&self) -> &u32 {
        &self.texture_atlas_sample_count_x_factor
    }
    fn texture_atlas_sample_count_y_factor(&self) -> &u32 {
        &self.texture_atlas_sample_count_y_factor
    }
    fn texture_samples_per_meter_max(&self) -> &f32 {
        &self.texture_samples_per_meter_max
    }
    fn texture_detail_falloff_factor(&self) -> &f32 {
        &self.texture_detail_falloff_factor
    }
    fn texture_detail_falloff_distance(&self) -> &f32 {
        &self.texture_detail_falloff_distance
    }
    fn texture_detail_falloff_curve(&self) -> &f32 {
        &self.texture_detail_falloff_curve
    }
    fn texture_invisible_detail_reduction_factor(&self) -> &f32 {
        &self.texture_invisible_detail_reduction_factor
    }
    fn texture_occluded_detail_reduction_factor(&self) -> &f32 {
        &self.texture_occluded_detail_reduction_factor
    }
    fn texture_render_job_count(&self) -> &u32 {
        &self.texture_render_job_count
    }
    fn real_time_edit_mode_texture_render_job_count(&self) -> &u32 {
        &self.real_time_edit_mode_texture_render_job_count
    }
    fn texture_vt_indirection_job_enable(&self) -> &bool {
        &self.texture_vt_indirection_job_enable
    }
    fn texture_streaming_prioritization_job_enable(&self) -> &bool {
        &self.texture_streaming_prioritization_job_enable
    }
    fn texture_render_jobs_launched_per_frame_count_max(&self) -> &u32 {
        &self.texture_render_jobs_launched_per_frame_count_max
    }
    fn real_time_edit_mode_texture_render_jobs_launched_per_frame_count_max(&self) -> &u32 {
        &self.real_time_edit_mode_texture_render_jobs_launched_per_frame_count_max
    }
    fn texture_tile_samples_per_side(&self) -> &u32 {
        &self.texture_tile_samples_per_side
    }
    fn texture_tile_border_width(&self) -> &u32 {
        &self.texture_tile_border_width
    }
    fn texture_level_offset(&self) -> &i32 {
        &self.texture_level_offset
    }
    fn texture_clod_frame_count(&self) -> &u32 {
        &self.texture_clod_frame_count
    }
    fn texture_clod_enable(&self) -> &bool {
        &self.texture_clod_enable
    }
    fn texture_clod_cutoff_priority(&self) -> &f32 {
        &self.texture_clod_cutoff_priority
    }
    fn texture_force_update_enable(&self) -> &bool {
        &self.texture_force_update_enable
    }
    fn texture_streamable_texture_instance_buffer_size(&self) -> &u32 {
        &self.texture_streamable_texture_instance_buffer_size
    }
    fn texture_compress_job_count(&self) -> &u32 {
        &self.texture_compress_job_count
    }
    fn texture_compress_fast_algorithm_enable(&self) -> &bool {
        &self.texture_compress_fast_algorithm_enable
    }
    fn texture_compression_quality(&self) -> &i32 {
        &self.texture_compression_quality
    }
    fn texture_detail_slope_boost(&self) -> &f32 {
        &self.texture_detail_slope_boost
    }
    fn texture_generation_mip_bias(&self) -> &f32 {
        &self.texture_generation_mip_bias
    }
    fn draw_texture_debug_colors(&self) -> &bool {
        &self.draw_texture_debug_colors
    }
    fn texture_draw_terrain_layers_enable(&self) -> &bool {
        &self.texture_draw_terrain_layers_enable
    }
    fn texture_keep_pool_full_enable(&self) -> &bool {
        &self.texture_keep_pool_full_enable
    }
    fn texture_layer_culling_enable(&self) -> &bool {
        &self.texture_layer_culling_enable
    }
    fn gpu_texture_compression_enable(&self) -> &bool {
        &self.gpu_texture_compression_enable
    }
    fn texture_dirty_retry_rate(&self) -> &f32 {
        &self.texture_dirty_retry_rate
    }
    fn texture_force_draw_pass(&self) -> &i32 {
        &self.texture_force_draw_pass
    }
    fn use_v_t_max_level_as_patch_level(&self) -> &bool {
        &self.use_v_t_max_level_as_patch_level
    }
    fn texture_streaming_prioritization_enable(&self) -> &bool {
        &self.texture_streaming_prioritization_enable
    }
    fn detail_texture_streaming_prioritization_enable(&self) -> &bool {
        &self.detail_texture_streaming_prioritization_enable
    }
    fn mesh_scattering_mesh_streaming_prioritization_enable(&self) -> &bool {
        &self.mesh_scattering_mesh_streaming_prioritization_enable
    }
    fn texture_quads_per_tile_level(&self) -> &u32 {
        &self.texture_quads_per_tile_level
    }
    fn prioritization_occlusion_enable(&self) -> &bool {
        &self.prioritization_occlusion_enable
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn draw_patches_enable(&self) -> &bool {
        &self.draw_patches_enable
    }
    fn detail_overlay_enable(&self) -> &bool {
        &self.detail_overlay_enable
    }
    fn draw_instancing_stats(&self) -> &bool {
        &self.draw_instancing_stats
    }
    fn decal_enable(&self) -> &bool {
        &self.decal_enable
    }
    fn force_decal_reduced_quality(&self) -> &bool {
        &self.force_decal_reduced_quality
    }
    fn draw_decal2d_enable(&self) -> &bool {
        &self.draw_decal2d_enable
    }
    fn draw_decal3d_enable(&self) -> &bool {
        &self.draw_decal3d_enable
    }
    fn draw_decal_z_pass_enable(&self) -> &bool {
        &self.draw_decal_z_pass_enable
    }
    fn draw_only_decal_z_pass_enable(&self) -> &bool {
        &self.draw_only_decal_z_pass_enable
    }
    fn decal_z_pass_draw_distance(&self) -> &f32 {
        &self.decal_z_pass_draw_distance
    }
    fn decal_offset_y(&self) -> &f32 {
        &self.decal_offset_y
    }
    fn decal3d_far_draw_distance_scale_factor(&self) -> &f32 {
        &self.decal3d_far_draw_distance_scale_factor
    }
    fn draw_patch_boxes_enable(&self) -> &bool {
        &self.draw_patch_boxes_enable
    }
    fn draw_bad_patches_enable(&self) -> &bool {
        &self.draw_bad_patches_enable
    }
    fn draw_texture_tile_boxes_enable(&self) -> &bool {
        &self.draw_texture_tile_boxes_enable
    }
    fn draw_debug_text_enable(&self) -> &bool {
        &self.draw_debug_text_enable
    }
    fn draw_debug_textures_enable(&self) -> &bool {
        &self.draw_debug_textures_enable
    }
    fn draw_quadtrees_enable(&self) -> &bool {
        &self.draw_quadtrees_enable
    }
    fn draw_quadtree_zoom_index(&self) -> &i32 {
        &self.draw_quadtree_zoom_index
    }
    fn draw_quadtree_stats_enable(&self) -> &bool {
        &self.draw_quadtree_stats_enable
    }
    fn draw_quadtree_atlas_textures_enable(&self) -> &bool {
        &self.draw_quadtree_atlas_textures_enable
    }
    fn draw_quadtree_atlas_textures_scale(&self) -> &f32 {
        &self.draw_quadtree_atlas_textures_scale
    }
    fn draw_indirection_textures_enable(&self) -> &bool {
        &self.draw_indirection_textures_enable
    }
    fn quickscope_pool_stats_enable(&self) -> &bool {
        &self.quickscope_pool_stats_enable
    }
    fn draw_dynamic_mask(&self) -> &bool {
        &self.draw_dynamic_mask
    }
    fn draw_debug_tile_priority_quadtree_enable(&self) -> &bool {
        &self.draw_debug_tile_priority_quadtree_enable
    }
    fn draw_water_enable(&self) -> &bool {
        &self.draw_water_enable
    }
    fn patch_error_fov_enable(&self) -> &bool {
        &self.patch_error_fov_enable
    }
    fn patch_error_fov(&self) -> &f32 {
        &self.patch_error_fov
    }
    fn z_pass_distance(&self) -> &f32 {
        &self.z_pass_distance
    }
    fn debug_overlay_grid_enable(&self) -> &bool {
        &self.debug_overlay_grid_enable
    }
    fn debug_overlay_grid_size(&self) -> &f32 {
        &self.debug_overlay_grid_size
    }
    fn debug_overlay_isolines_enable(&self) -> &bool {
        &self.debug_overlay_isolines_enable
    }
    fn debug_overlay_isoline_spacing(&self) -> &f32 {
        &self.debug_overlay_isoline_spacing
    }
    fn debug_overlay_wireframe_enable(&self) -> &bool {
        &self.debug_overlay_wireframe_enable
    }
    fn debug_overlay_sketch_texture_enable(&self) -> &bool {
        &self.debug_overlay_sketch_texture_enable
    }
    fn debug_overlay_brush_enable(&self) -> &bool {
        &self.debug_overlay_brush_enable
    }
    fn force_graphics_driver_crash(&self) -> &bool {
        &self.force_graphics_driver_crash
    }
    fn force_patch_rebuild_enable(&self) -> &bool {
        &self.force_patch_rebuild_enable
    }
    fn destroy_all(&self) -> &bool {
        &self.destroy_all
    }
    fn slot_reuse_wait_count(&self) -> &u32 {
        &self.slot_reuse_wait_count
    }
    fn slot_debug_output_enable(&self) -> &bool {
        &self.slot_debug_output_enable
    }
    fn update_jobs_enable(&self) -> &bool {
        &self.update_jobs_enable
    }
    fn build_job_count(&self) -> &u32 {
        &self.build_job_count
    }
    fn regenerate_textures_enable(&self) -> &bool {
        &self.regenerate_textures_enable
    }
    fn dynamic_mask_enable(&self) -> &bool {
        &self.dynamic_mask_enable
    }
    fn max_non_visible_texture_update_count(&self) -> &u32 {
        &self.max_non_visible_texture_update_count
    }
    fn patch_faces_per_side(&self) -> &u32 {
        &self.patch_faces_per_side
    }
    fn tessellation_faces_per_side_min(&self) -> &u32 {
        &self.tessellation_faces_per_side_min
    }
    fn patch_slot_count(&self) -> &u32 {
        &self.patch_slot_count
    }
    fn patch_lod_transitions_enable(&self) -> &bool {
        &self.patch_lod_transitions_enable
    }
    fn patch_lod_transitions_vertex_shader_enable(&self) -> &bool {
        &self.patch_lod_transitions_vertex_shader_enable
    }
    fn patch_lod_transitions_vertex_shader_disable_fixup(&self) -> &bool {
        &self.patch_lod_transitions_vertex_shader_disable_fixup
    }
    fn patch_material_sorting_enable(&self) -> &bool {
        &self.patch_material_sorting_enable
    }
    fn cull_sample_bounding_box_height_enable(&self) -> &bool {
        &self.cull_sample_bounding_box_height_enable
    }
    fn force_shadows_off(&self) -> &bool {
        &self.force_shadows_off
    }
    fn terrain_cast_shadows_quality_level(&self) -> &super::core::QualityLevel {
        &self.terrain_cast_shadows_quality_level
    }
    fn cast_planar_reflection_enable(&self) -> &bool {
        &self.cast_planar_reflection_enable
    }
    fn cast_envmap_reflection_enable(&self) -> &bool {
        &self.cast_envmap_reflection_enable
    }
    fn cast_decal3d_planar_reflection_enable(&self) -> &bool {
        &self.cast_decal3d_planar_reflection_enable
    }
    fn cast_decal3d_envmap_reflection_enable(&self) -> &bool {
        &self.cast_decal3d_envmap_reflection_enable
    }
    fn detail_displacement_in_shadow_view_enable(&self) -> &bool {
        &self.detail_displacement_in_shadow_view_enable
    }
    fn global_colormap_enable(&self) -> &bool {
        &self.global_colormap_enable
    }
    fn occluder_enable(&self) -> &bool {
        &self.occluder_enable
    }
    fn occluder_job_enable(&self) -> &bool {
        &self.occluder_job_enable
    }
    fn occluder_job_count(&self) -> &u32 {
        &self.occluder_job_count
    }
    fn occluder_patch_faces_per_side(&self) -> &u32 {
        &self.occluder_patch_faces_per_side
    }
    fn occluder_lod_scale(&self) -> &f32 {
        &self.occluder_lod_scale
    }
    fn occluder_back_face_culling_enable(&self) -> &bool {
        &self.occluder_back_face_culling_enable
    }
    fn occluded_enable(&self) -> &bool {
        &self.occluded_enable
    }
    fn occluded_min_distance(&self) -> &f32 {
        &self.occluded_min_distance
    }
    fn mesh_scattering_enable(&self) -> &bool {
        &self.mesh_scattering_enable
    }
    fn mesh_scattering_reflections_enable(&self) -> &bool {
        &self.mesh_scattering_reflections_enable
    }
    fn mesh_scattering_jobs_enable(&self) -> &bool {
        &self.mesh_scattering_jobs_enable
    }
    fn mesh_scattering_cast_shadows_enable(&self) -> &bool {
        &self.mesh_scattering_cast_shadows_enable
    }
    fn draw_mesh_scattering_enable(&self) -> &bool {
        &self.draw_mesh_scattering_enable
    }
    fn draw_mesh_scattering_cell_boxes_enable(&self) -> &bool {
        &self.draw_mesh_scattering_cell_boxes_enable
    }
    fn draw_mesh_scattering_batch_boxes_enable(&self) -> &bool {
        &self.draw_mesh_scattering_batch_boxes_enable
    }
    fn draw_mesh_scattering_node_boxes_enable(&self) -> &bool {
        &self.draw_mesh_scattering_node_boxes_enable
    }
    fn draw_mesh_scattering_culled_cell_boxes_enable(&self) -> &bool {
        &self.draw_mesh_scattering_culled_cell_boxes_enable
    }
    fn draw_mesh_scattering_debug_mask_scale_textures_enable(&self) -> &bool {
        &self.draw_mesh_scattering_debug_mask_scale_textures_enable
    }
    fn draw_mesh_scattering_stats_enable(&self) -> &bool {
        &self.draw_mesh_scattering_stats_enable
    }
    fn draw_mesh_scattering_quadtree_enable(&self) -> &bool {
        &self.draw_mesh_scattering_quadtree_enable
    }
    fn mesh_scattering_cell_pool_capacity(&self) -> &u32 {
        &self.mesh_scattering_cell_pool_capacity
    }
    fn mesh_scattering_tree_node_pool_capacity(&self) -> &u32 {
        &self.mesh_scattering_tree_node_pool_capacity
    }
    fn mesh_scattering_invisible_cell_fov_factor(&self) -> &f32 {
        &self.mesh_scattering_invisible_cell_fov_factor
    }
    fn mesh_scattering_force_update_enable(&self) -> &bool {
        &self.mesh_scattering_force_update_enable
    }
    fn mesh_scattering_cull_record_count(&self) -> &u32 {
        &self.mesh_scattering_cull_record_count
    }
    fn mesh_scattering_build_channel_count(&self) -> &u32 {
        &self.mesh_scattering_build_channel_count
    }
    fn mesh_scattering_build_channels_launched_per_frame_count_max(&self) -> &u32 {
        &self.mesh_scattering_build_channels_launched_per_frame_count_max
    }
    fn mesh_scattering_build_visible_first(&self) -> &bool {
        &self.mesh_scattering_build_visible_first
    }
    fn mesh_scattering_clod_frame_count(&self) -> &u32 {
        &self.mesh_scattering_clod_frame_count
    }
    fn mesh_scattering_wind_speed(&self) -> &f32 {
        &self.mesh_scattering_wind_speed
    }
    fn mesh_scattering_instances_per_cell_max(&self) -> &u32 {
        &self.mesh_scattering_instances_per_cell_max
    }
    fn mesh_scattering_density_margin_factor(&self) -> &f32 {
        &self.mesh_scattering_density_margin_factor
    }
    fn mesh_scattering_pregeneration_distance_ratio(&self) -> &f32 {
        &self.mesh_scattering_pregeneration_distance_ratio
    }
    fn mesh_scattering_keep_distance_ratio(&self) -> &f32 {
        &self.mesh_scattering_keep_distance_ratio
    }
    fn mesh_scattering_merge_instance_lists(&self) -> &bool {
        &self.mesh_scattering_merge_instance_lists
    }
    fn mesh_scattering_virtual_texture_blurriness(&self) -> &i32 {
        &self.mesh_scattering_virtual_texture_blurriness
    }
    fn mesh_scattering_shadow_view_cull_size(&self) -> &f32 {
        &self.mesh_scattering_shadow_view_cull_size
    }
    fn mesh_scattering_distance_scale_factor(&self) -> &f32 {
        &self.mesh_scattering_distance_scale_factor
    }
    fn mesh_scattering_instance_cull_job_count(&self) -> &u32 {
        &self.mesh_scattering_instance_cull_job_count
    }
    fn mesh_scattering_instance_cull_list_count(&self) -> &u32 {
        &self.mesh_scattering_instance_cull_list_count
    }
    fn mesh_scattering_instance_cull_box_test_enable(&self) -> &bool {
        &self.mesh_scattering_instance_cull_box_test_enable
    }
    fn mesh_scattering_instance_frustum_cull_enable(&self) -> &bool {
        &self.mesh_scattering_instance_frustum_cull_enable
    }
    fn mesh_scattering_instance_occlusion_cull_enable(&self) -> &bool {
        &self.mesh_scattering_instance_occlusion_cull_enable
    }
    fn mesh_scattering_instance_additional_cull_enable(&self) -> &bool {
        &self.mesh_scattering_instance_additional_cull_enable
    }
    fn draw_mesh_scattering_instance_boxes_enable(&self) -> &bool {
        &self.draw_mesh_scattering_instance_boxes_enable
    }
    fn mesh_scattering_instance_cull_dynamic_alloc_enable(&self) -> &bool {
        &self.mesh_scattering_instance_cull_dynamic_alloc_enable
    }
    fn mesh_scattering_wind_enable(&self) -> &bool {
        &self.mesh_scattering_wind_enable
    }
    fn mesh_scattering_draw_motion_vectors_enable(&self) -> &bool {
        &self.mesh_scattering_draw_motion_vectors_enable
    }
    fn mesh_scattering_snapping_grid_multiplier_vertical(&self) -> &f32 {
        &self.mesh_scattering_snapping_grid_multiplier_vertical
    }
    fn mesh_scattering_snapping_grid_multiplier_horizontal(&self) -> &f32 {
        &self.mesh_scattering_snapping_grid_multiplier_horizontal
    }
    fn detail_displacement_quality_level(&self) -> &super::core::QualityLevel {
        &self.detail_displacement_quality_level
    }
    fn detail_displacement_enable(&self) -> &bool {
        &self.detail_displacement_enable
    }
    fn draw_detail_displacement_enable(&self) -> &bool {
        &self.draw_detail_displacement_enable
    }
    fn draw_debug_detail_displacement_enable(&self) -> &bool {
        &self.draw_debug_detail_displacement_enable
    }
    fn draw_detail_displacement_tree_level(&self) -> &i32 {
        &self.draw_detail_displacement_tree_level
    }
    fn detail_displacement_max_tess_factor(&self) -> &f32 {
        &self.detail_displacement_max_tess_factor
    }
    fn detail_displacement_scale(&self) -> &f32 {
        &self.detail_displacement_scale
    }
    fn detail_displacement_bias(&self) -> &f32 {
        &self.detail_displacement_bias
    }
    fn detail_displacement_fade_range(&self) -> &f32 {
        &self.detail_displacement_fade_range
    }
    fn height_field_tessellation_factor(&self) -> &f32 {
        &self.height_field_tessellation_factor
    }
    fn height_field_tessellation_fade_range(&self) -> &f32 {
        &self.height_field_tessellation_fade_range
    }
}

impl super::terrain_base::VisualTerrainBaseSettingsTrait for VisualTerrainSettings {
}

impl super::core::DataContainerTrait for VisualTerrainSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VISUALTERRAINSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::terrain_base::VISUALTERRAINBASESETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrainSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeshScatteringQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_quality_level),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: "TerrainRenderMode",
                rust_offset: offset_of!(VisualTerrainSettings, render_mode),
            },
            FieldInfoData {
                name: "WireframeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, wireframe_enable),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, enable),
            },
            FieldInfoData {
                name: "EditServiceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, edit_service_enable),
            },
            FieldInfoData {
                name: "TriangleSizeMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, triangle_size_min),
            },
            FieldInfoData {
                name: "LodScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, lod_scale),
            },
            FieldInfoData {
                name: "LodCenterExtrapolationDistanceMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, lod_center_extrapolation_distance_max),
            },
            FieldInfoData {
                name: "LodCenterExtrapolationTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, lod_center_extrapolation_time),
            },
            FieldInfoData {
                name: "TextureSkipMipSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_skip_mip_speed),
            },
            FieldInfoData {
                name: "TessellationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_enable),
            },
            FieldInfoData {
                name: "TessellationForReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_for_reflections_enable),
            },
            FieldInfoData {
                name: "TessellationForShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_for_shadows_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementForReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_for_reflections_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementForShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_for_shadows_enable),
            },
            FieldInfoData {
                name: "TessellationPatchShrink",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_patch_shrink),
            },
            FieldInfoData {
                name: "TessellationPatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_patch_faces_per_side),
            },
            FieldInfoData {
                name: "TessellatedTriWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, tessellated_tri_width),
            },
            FieldInfoData {
                name: "TessellatedTriWidthScaleForReflections",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, tessellated_tri_width_scale_for_reflections),
            },
            FieldInfoData {
                name: "TessellatedTriWidthScaleForShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, tessellated_tri_width_scale_for_shadows),
            },
            FieldInfoData {
                name: "AdditionalCullFrustumEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, additional_cull_frustum_enable),
            },
            FieldInfoData {
                name: "DensityMapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, density_map_enable),
            },
            FieldInfoData {
                name: "VertexBufferHeightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, vertex_buffer_heights_enable),
            },
            FieldInfoData {
                name: "DrawVertexYTextureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_vertex_y_texture_enable),
            },
            FieldInfoData {
                name: "TextureAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "TextureAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "TextureSamplesPerMeterMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_samples_per_meter_max),
            },
            FieldInfoData {
                name: "TextureDetailFalloffFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_falloff_factor),
            },
            FieldInfoData {
                name: "TextureDetailFalloffDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_falloff_distance),
            },
            FieldInfoData {
                name: "TextureDetailFalloffCurve",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_falloff_curve),
            },
            FieldInfoData {
                name: "TextureInvisibleDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_invisible_detail_reduction_factor),
            },
            FieldInfoData {
                name: "TextureOccludedDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_occluded_detail_reduction_factor),
            },
            FieldInfoData {
                name: "TextureRenderJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_render_job_count),
            },
            FieldInfoData {
                name: "RealTimeEditModeTextureRenderJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, real_time_edit_mode_texture_render_job_count),
            },
            FieldInfoData {
                name: "TextureVtIndirectionJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_vt_indirection_job_enable),
            },
            FieldInfoData {
                name: "TextureStreamingPrioritizationJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_streaming_prioritization_job_enable),
            },
            FieldInfoData {
                name: "TextureRenderJobsLaunchedPerFrameCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_render_jobs_launched_per_frame_count_max),
            },
            FieldInfoData {
                name: "RealTimeEditModeTextureRenderJobsLaunchedPerFrameCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, real_time_edit_mode_texture_render_jobs_launched_per_frame_count_max),
            },
            FieldInfoData {
                name: "TextureTileSamplesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_tile_samples_per_side),
            },
            FieldInfoData {
                name: "TextureTileBorderWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_tile_border_width),
            },
            FieldInfoData {
                name: "TextureLevelOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_level_offset),
            },
            FieldInfoData {
                name: "TextureClodFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_clod_frame_count),
            },
            FieldInfoData {
                name: "TextureClodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_clod_enable),
            },
            FieldInfoData {
                name: "TextureClodCutoffPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_clod_cutoff_priority),
            },
            FieldInfoData {
                name: "TextureForceUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_force_update_enable),
            },
            FieldInfoData {
                name: "TextureStreamableTextureInstanceBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_streamable_texture_instance_buffer_size),
            },
            FieldInfoData {
                name: "TextureCompressJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_compress_job_count),
            },
            FieldInfoData {
                name: "TextureCompressFastAlgorithmEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_compress_fast_algorithm_enable),
            },
            FieldInfoData {
                name: "TextureCompressionQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_compression_quality),
            },
            FieldInfoData {
                name: "TextureDetailSlopeBoost",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_slope_boost),
            },
            FieldInfoData {
                name: "TextureGenerationMipBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_generation_mip_bias),
            },
            FieldInfoData {
                name: "DrawTextureDebugColors",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_texture_debug_colors),
            },
            FieldInfoData {
                name: "TextureDrawTerrainLayersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_draw_terrain_layers_enable),
            },
            FieldInfoData {
                name: "TextureKeepPoolFullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_keep_pool_full_enable),
            },
            FieldInfoData {
                name: "TextureLayerCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_layer_culling_enable),
            },
            FieldInfoData {
                name: "GpuTextureCompressionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, gpu_texture_compression_enable),
            },
            FieldInfoData {
                name: "TextureDirtyRetryRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_dirty_retry_rate),
            },
            FieldInfoData {
                name: "TextureForceDrawPass",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_force_draw_pass),
            },
            FieldInfoData {
                name: "UseVTMaxLevelAsPatchLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, use_v_t_max_level_as_patch_level),
            },
            FieldInfoData {
                name: "TextureStreamingPrioritizationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, texture_streaming_prioritization_enable),
            },
            FieldInfoData {
                name: "DetailTextureStreamingPrioritizationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, detail_texture_streaming_prioritization_enable),
            },
            FieldInfoData {
                name: "MeshScatteringMeshStreamingPrioritizationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_mesh_streaming_prioritization_enable),
            },
            FieldInfoData {
                name: "TextureQuadsPerTileLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, texture_quads_per_tile_level),
            },
            FieldInfoData {
                name: "PrioritizationOcclusionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, prioritization_occlusion_enable),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_enable),
            },
            FieldInfoData {
                name: "DrawPatchesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_patches_enable),
            },
            FieldInfoData {
                name: "DetailOverlayEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, detail_overlay_enable),
            },
            FieldInfoData {
                name: "DrawInstancingStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_instancing_stats),
            },
            FieldInfoData {
                name: "DecalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, decal_enable),
            },
            FieldInfoData {
                name: "ForceDecalReducedQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, force_decal_reduced_quality),
            },
            FieldInfoData {
                name: "DrawDecal2dEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_decal2d_enable),
            },
            FieldInfoData {
                name: "DrawDecal3dEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_decal3d_enable),
            },
            FieldInfoData {
                name: "DrawDecalZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_decal_z_pass_enable),
            },
            FieldInfoData {
                name: "DrawOnlyDecalZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_only_decal_z_pass_enable),
            },
            FieldInfoData {
                name: "DecalZPassDrawDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, decal_z_pass_draw_distance),
            },
            FieldInfoData {
                name: "DecalOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, decal_offset_y),
            },
            FieldInfoData {
                name: "Decal3dFarDrawDistanceScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, decal3d_far_draw_distance_scale_factor),
            },
            FieldInfoData {
                name: "DrawPatchBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_patch_boxes_enable),
            },
            FieldInfoData {
                name: "DrawBadPatchesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_bad_patches_enable),
            },
            FieldInfoData {
                name: "DrawTextureTileBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_texture_tile_boxes_enable),
            },
            FieldInfoData {
                name: "DrawDebugTextEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_text_enable),
            },
            FieldInfoData {
                name: "DrawDebugTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_textures_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtrees_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreeZoomIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_zoom_index),
            },
            FieldInfoData {
                name: "DrawQuadtreeStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_stats_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreeAtlasTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_atlas_textures_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreeAtlasTexturesScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_atlas_textures_scale),
            },
            FieldInfoData {
                name: "DrawIndirectionTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_indirection_textures_enable),
            },
            FieldInfoData {
                name: "QuickscopePoolStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, quickscope_pool_stats_enable),
            },
            FieldInfoData {
                name: "DrawDynamicMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_dynamic_mask),
            },
            FieldInfoData {
                name: "DrawDebugTilePriorityQuadtreeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_tile_priority_quadtree_enable),
            },
            FieldInfoData {
                name: "DrawWaterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_water_enable),
            },
            FieldInfoData {
                name: "PatchErrorFovEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, patch_error_fov_enable),
            },
            FieldInfoData {
                name: "PatchErrorFov",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, patch_error_fov),
            },
            FieldInfoData {
                name: "ZPassDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, z_pass_distance),
            },
            FieldInfoData {
                name: "DebugOverlayGridEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_grid_enable),
            },
            FieldInfoData {
                name: "DebugOverlayGridSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_grid_size),
            },
            FieldInfoData {
                name: "DebugOverlayIsolinesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_isolines_enable),
            },
            FieldInfoData {
                name: "DebugOverlayIsolineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_isoline_spacing),
            },
            FieldInfoData {
                name: "DebugOverlayWireframeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_wireframe_enable),
            },
            FieldInfoData {
                name: "DebugOverlaySketchTextureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_sketch_texture_enable),
            },
            FieldInfoData {
                name: "DebugOverlayBrushEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_brush_enable),
            },
            FieldInfoData {
                name: "ForceGraphicsDriverCrash",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, force_graphics_driver_crash),
            },
            FieldInfoData {
                name: "ForcePatchRebuildEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, force_patch_rebuild_enable),
            },
            FieldInfoData {
                name: "DestroyAll",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, destroy_all),
            },
            FieldInfoData {
                name: "SlotReuseWaitCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, slot_reuse_wait_count),
            },
            FieldInfoData {
                name: "SlotDebugOutputEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, slot_debug_output_enable),
            },
            FieldInfoData {
                name: "UpdateJobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, update_jobs_enable),
            },
            FieldInfoData {
                name: "BuildJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, build_job_count),
            },
            FieldInfoData {
                name: "RegenerateTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, regenerate_textures_enable),
            },
            FieldInfoData {
                name: "DynamicMaskEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, dynamic_mask_enable),
            },
            FieldInfoData {
                name: "MaxNonVisibleTextureUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, max_non_visible_texture_update_count),
            },
            FieldInfoData {
                name: "PatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, patch_faces_per_side),
            },
            FieldInfoData {
                name: "TessellationFacesPerSideMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_faces_per_side_min),
            },
            FieldInfoData {
                name: "PatchSlotCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, patch_slot_count),
            },
            FieldInfoData {
                name: "PatchLodTransitionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, patch_lod_transitions_enable),
            },
            FieldInfoData {
                name: "PatchLodTransitionsVertexShaderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, patch_lod_transitions_vertex_shader_enable),
            },
            FieldInfoData {
                name: "PatchLodTransitionsVertexShaderDisableFixup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, patch_lod_transitions_vertex_shader_disable_fixup),
            },
            FieldInfoData {
                name: "PatchMaterialSortingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, patch_material_sorting_enable),
            },
            FieldInfoData {
                name: "CullSampleBoundingBoxHeightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, cull_sample_bounding_box_height_enable),
            },
            FieldInfoData {
                name: "ForceShadowsOff",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, force_shadows_off),
            },
            FieldInfoData {
                name: "TerrainCastShadowsQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(VisualTerrainSettings, terrain_cast_shadows_quality_level),
            },
            FieldInfoData {
                name: "CastPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, cast_planar_reflection_enable),
            },
            FieldInfoData {
                name: "CastEnvmapReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, cast_envmap_reflection_enable),
            },
            FieldInfoData {
                name: "CastDecal3dPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, cast_decal3d_planar_reflection_enable),
            },
            FieldInfoData {
                name: "CastDecal3dEnvmapReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, cast_decal3d_envmap_reflection_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementInShadowViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_in_shadow_view_enable),
            },
            FieldInfoData {
                name: "GlobalColormapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, global_colormap_enable),
            },
            FieldInfoData {
                name: "OccluderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, occluder_enable),
            },
            FieldInfoData {
                name: "OccluderJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, occluder_job_enable),
            },
            FieldInfoData {
                name: "OccluderJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, occluder_job_count),
            },
            FieldInfoData {
                name: "OccluderPatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, occluder_patch_faces_per_side),
            },
            FieldInfoData {
                name: "OccluderLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, occluder_lod_scale),
            },
            FieldInfoData {
                name: "OccluderBackFaceCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, occluder_back_face_culling_enable),
            },
            FieldInfoData {
                name: "OccludedEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, occluded_enable),
            },
            FieldInfoData {
                name: "OccludedMinDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, occluded_min_distance),
            },
            FieldInfoData {
                name: "MeshScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_enable),
            },
            FieldInfoData {
                name: "MeshScatteringReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_reflections_enable),
            },
            FieldInfoData {
                name: "MeshScatteringJobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_jobs_enable),
            },
            FieldInfoData {
                name: "MeshScatteringCastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_cast_shadows_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringCellBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_cell_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringBatchBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_batch_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringNodeBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_node_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringCulledCellBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_culled_cell_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringDebugMaskScaleTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_debug_mask_scale_textures_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_stats_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringQuadtreeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_quadtree_enable),
            },
            FieldInfoData {
                name: "MeshScatteringCellPoolCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_cell_pool_capacity),
            },
            FieldInfoData {
                name: "MeshScatteringTreeNodePoolCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_tree_node_pool_capacity),
            },
            FieldInfoData {
                name: "MeshScatteringInvisibleCellFovFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_invisible_cell_fov_factor),
            },
            FieldInfoData {
                name: "MeshScatteringForceUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_force_update_enable),
            },
            FieldInfoData {
                name: "MeshScatteringCullRecordCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_cull_record_count),
            },
            FieldInfoData {
                name: "MeshScatteringBuildChannelCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_build_channel_count),
            },
            FieldInfoData {
                name: "MeshScatteringBuildChannelsLaunchedPerFrameCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_build_channels_launched_per_frame_count_max),
            },
            FieldInfoData {
                name: "MeshScatteringBuildVisibleFirst",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_build_visible_first),
            },
            FieldInfoData {
                name: "MeshScatteringClodFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_clod_frame_count),
            },
            FieldInfoData {
                name: "MeshScatteringWindSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_wind_speed),
            },
            FieldInfoData {
                name: "MeshScatteringInstancesPerCellMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instances_per_cell_max),
            },
            FieldInfoData {
                name: "MeshScatteringDensityMarginFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_density_margin_factor),
            },
            FieldInfoData {
                name: "MeshScatteringPregenerationDistanceRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_pregeneration_distance_ratio),
            },
            FieldInfoData {
                name: "MeshScatteringKeepDistanceRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_keep_distance_ratio),
            },
            FieldInfoData {
                name: "MeshScatteringMergeInstanceLists",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_merge_instance_lists),
            },
            FieldInfoData {
                name: "MeshScatteringVirtualTextureBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_virtual_texture_blurriness),
            },
            FieldInfoData {
                name: "MeshScatteringShadowViewCullSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_shadow_view_cull_size),
            },
            FieldInfoData {
                name: "MeshScatteringDistanceScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_distance_scale_factor),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_job_count),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullListCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_list_count),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullBoxTestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_box_test_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceFrustumCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_frustum_cull_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceAdditionalCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_additional_cull_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringInstanceBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_instance_boxes_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullDynamicAllocEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_dynamic_alloc_enable),
            },
            FieldInfoData {
                name: "MeshScatteringWindEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_wind_enable),
            },
            FieldInfoData {
                name: "MeshScatteringDrawMotionVectorsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_draw_motion_vectors_enable),
            },
            FieldInfoData {
                name: "MeshScatteringSnappingGridMultiplierVertical",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_snapping_grid_multiplier_vertical),
            },
            FieldInfoData {
                name: "MeshScatteringSnappingGridMultiplierHorizontal",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_snapping_grid_multiplier_horizontal),
            },
            FieldInfoData {
                name: "DetailDisplacementQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_quality_level),
            },
            FieldInfoData {
                name: "DetailDisplacementEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_enable),
            },
            FieldInfoData {
                name: "DrawDetailDisplacementEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_detail_displacement_enable),
            },
            FieldInfoData {
                name: "DrawDebugDetailDisplacementEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_detail_displacement_enable),
            },
            FieldInfoData {
                name: "DrawDetailDisplacementTreeLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VisualTerrainSettings, draw_detail_displacement_tree_level),
            },
            FieldInfoData {
                name: "DetailDisplacementMaxTessFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_max_tess_factor),
            },
            FieldInfoData {
                name: "DetailDisplacementScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_scale),
            },
            FieldInfoData {
                name: "DetailDisplacementBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_bias),
            },
            FieldInfoData {
                name: "DetailDisplacementFadeRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_fade_range),
            },
            FieldInfoData {
                name: "HeightFieldTessellationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, height_field_tessellation_factor),
            },
            FieldInfoData {
                name: "HeightFieldTessellationFadeRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VisualTerrainSettings, height_field_tessellation_fade_range),
            },
        ],
    }),
    array_type: Some(VISUALTERRAINSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualTerrainSettings {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALTERRAINSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALTERRAINSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("VisualTerrainSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TerrainRenderMode {
    #[default]
    TerrainRenderMode_Default = 0,
    TerrainRenderMode_DrawPassCount2d = 1,
    TerrainRenderMode_LayerCount2d = 2,
    TerrainRenderMode_LayerCount3d = 3,
    TerrainRenderMode_MaskedLayerCount2d = 4,
    TerrainRenderMode_MaskedLayerCount3d = 5,
    TerrainRenderMode_DensityMap = 6,
}

pub static TERRAINRENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainRender",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINRENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainRenderMode {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINRENDERMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINRENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainRenderMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisualTerrain {
    pub _glacier_base: IVisualTerrain,
}

pub trait VisualTerrainTrait: IVisualTerrainTrait {
}

impl VisualTerrainTrait for VisualTerrain {
}

impl IVisualTerrainTrait for VisualTerrain {
}

impl super::terrain_base::ITerrainTrait for VisualTerrain {
}

pub static VISUALTERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IVISUALTERRAIN_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisualTerrain as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VISUALTERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VisualTerrain {
    fn type_info(&self) -> &'static TypeInfo {
        VISUALTERRAIN_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISUALTERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("VisualTerrain"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainLayerCombinations {
}

pub trait TerrainLayerCombinationsTrait: TypeObject {
}

impl TerrainLayerCombinationsTrait for TerrainLayerCombinations {
}

pub static TERRAINLAYERCOMBINATIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinations",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainLayerCombinations as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainLayerCombinations {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINLAYERCOMBINATIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinations-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainLayerCombinations"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainDecals {
}

pub trait TerrainDecalsTrait: TypeObject {
}

impl TerrainDecalsTrait for TerrainDecals {
}

pub static TERRAINDECALS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDecals",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainDecals as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINDECALS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainDecals {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINDECALS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINDECALS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDecals-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainDecals"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IVisualTerrain {
    pub _glacier_base: super::terrain_base::ITerrain,
}

pub trait IVisualTerrainTrait: super::terrain_base::ITerrainTrait {
}

impl IVisualTerrainTrait for IVisualTerrain {
}

impl super::terrain_base::ITerrainTrait for IVisualTerrain {
}

pub static IVISUALTERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVisualTerrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::terrain_base::ITERRAIN_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IVisualTerrain as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IVISUALTERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IVisualTerrain {
    fn type_info(&self) -> &'static TypeInfo {
        IVISUALTERRAIN_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IVISUALTERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVisualTerrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("IVisualTerrain"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TerrainTextureTree {
}

pub trait TerrainTextureTreeTrait: TypeObject {
}

impl TerrainTextureTreeTrait for TerrainTextureTree {
}

pub static TERRAINTEXTURETREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainTextureTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TerrainTextureTree as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TERRAINTEXTURETREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainTextureTree {
    fn type_info(&self) -> &'static TypeInfo {
        TERRAINTEXTURETREE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TERRAINTEXTURETREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainTextureTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainTextureTree"),
    array_type: None,
    alignment: 8,
};


