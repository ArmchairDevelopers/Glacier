use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshScatteringTree {
}

pub const MESHSCATTERINGTREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(MESHSCATTERINGTREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshScatteringTree {
    fn type_info() -> &'static TypeInfo {
        MESHSCATTERINGTREE_TYPE_INFO
    }
}


pub const MESHSCATTERINGTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshScatteringTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("MeshScatteringTree-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DisplacementTextureTree {
}

pub const DISPLACEMENTTEXTURETREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplacementTextureTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(DISPLACEMENTTEXTURETREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DisplacementTextureTree {
    fn type_info() -> &'static TypeInfo {
        DISPLACEMENTTEXTURETREE_TYPE_INFO
    }
}


pub const DISPLACEMENTTEXTURETREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplacementTextureTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("DisplacementTextureTree-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VisualTerrainSettings {
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

pub const VISUALTERRAINSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainSettings",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VISUALTERRAINBASESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MeshScatteringQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_quality_level),
            },
            FieldInfoData {
                name: "RenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: TERRAINRENDERMODE_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, render_mode),
            },
            FieldInfoData {
                name: "WireframeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, wireframe_enable),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, enable),
            },
            FieldInfoData {
                name: "EditServiceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, edit_service_enable),
            },
            FieldInfoData {
                name: "TriangleSizeMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, triangle_size_min),
            },
            FieldInfoData {
                name: "LodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, lod_scale),
            },
            FieldInfoData {
                name: "LodCenterExtrapolationDistanceMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, lod_center_extrapolation_distance_max),
            },
            FieldInfoData {
                name: "LodCenterExtrapolationTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, lod_center_extrapolation_time),
            },
            FieldInfoData {
                name: "TextureSkipMipSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_skip_mip_speed),
            },
            FieldInfoData {
                name: "TessellationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_enable),
            },
            FieldInfoData {
                name: "TessellationForReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_for_reflections_enable),
            },
            FieldInfoData {
                name: "TessellationForShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_for_shadows_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementForReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_for_reflections_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementForShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_for_shadows_enable),
            },
            FieldInfoData {
                name: "TessellationPatchShrink",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_patch_shrink),
            },
            FieldInfoData {
                name: "TessellationPatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_patch_faces_per_side),
            },
            FieldInfoData {
                name: "TessellatedTriWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellated_tri_width),
            },
            FieldInfoData {
                name: "TessellatedTriWidthScaleForReflections",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellated_tri_width_scale_for_reflections),
            },
            FieldInfoData {
                name: "TessellatedTriWidthScaleForShadows",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellated_tri_width_scale_for_shadows),
            },
            FieldInfoData {
                name: "AdditionalCullFrustumEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, additional_cull_frustum_enable),
            },
            FieldInfoData {
                name: "DensityMapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, density_map_enable),
            },
            FieldInfoData {
                name: "VertexBufferHeightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, vertex_buffer_heights_enable),
            },
            FieldInfoData {
                name: "DrawVertexYTextureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_vertex_y_texture_enable),
            },
            FieldInfoData {
                name: "TextureAtlasSampleCountXFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_atlas_sample_count_x_factor),
            },
            FieldInfoData {
                name: "TextureAtlasSampleCountYFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_atlas_sample_count_y_factor),
            },
            FieldInfoData {
                name: "TextureSamplesPerMeterMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_samples_per_meter_max),
            },
            FieldInfoData {
                name: "TextureDetailFalloffFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_falloff_factor),
            },
            FieldInfoData {
                name: "TextureDetailFalloffDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_falloff_distance),
            },
            FieldInfoData {
                name: "TextureDetailFalloffCurve",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_falloff_curve),
            },
            FieldInfoData {
                name: "TextureInvisibleDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_invisible_detail_reduction_factor),
            },
            FieldInfoData {
                name: "TextureOccludedDetailReductionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_occluded_detail_reduction_factor),
            },
            FieldInfoData {
                name: "TextureRenderJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_render_job_count),
            },
            FieldInfoData {
                name: "RealTimeEditModeTextureRenderJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, real_time_edit_mode_texture_render_job_count),
            },
            FieldInfoData {
                name: "TextureVtIndirectionJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_vt_indirection_job_enable),
            },
            FieldInfoData {
                name: "TextureStreamingPrioritizationJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_streaming_prioritization_job_enable),
            },
            FieldInfoData {
                name: "TextureRenderJobsLaunchedPerFrameCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_render_jobs_launched_per_frame_count_max),
            },
            FieldInfoData {
                name: "RealTimeEditModeTextureRenderJobsLaunchedPerFrameCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, real_time_edit_mode_texture_render_jobs_launched_per_frame_count_max),
            },
            FieldInfoData {
                name: "TextureTileSamplesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_tile_samples_per_side),
            },
            FieldInfoData {
                name: "TextureTileBorderWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_tile_border_width),
            },
            FieldInfoData {
                name: "TextureLevelOffset",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_level_offset),
            },
            FieldInfoData {
                name: "TextureClodFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_clod_frame_count),
            },
            FieldInfoData {
                name: "TextureClodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_clod_enable),
            },
            FieldInfoData {
                name: "TextureClodCutoffPriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_clod_cutoff_priority),
            },
            FieldInfoData {
                name: "TextureForceUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_force_update_enable),
            },
            FieldInfoData {
                name: "TextureStreamableTextureInstanceBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_streamable_texture_instance_buffer_size),
            },
            FieldInfoData {
                name: "TextureCompressJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_compress_job_count),
            },
            FieldInfoData {
                name: "TextureCompressFastAlgorithmEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_compress_fast_algorithm_enable),
            },
            FieldInfoData {
                name: "TextureCompressionQuality",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_compression_quality),
            },
            FieldInfoData {
                name: "TextureDetailSlopeBoost",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_detail_slope_boost),
            },
            FieldInfoData {
                name: "TextureGenerationMipBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_generation_mip_bias),
            },
            FieldInfoData {
                name: "DrawTextureDebugColors",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_texture_debug_colors),
            },
            FieldInfoData {
                name: "TextureDrawTerrainLayersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_draw_terrain_layers_enable),
            },
            FieldInfoData {
                name: "TextureKeepPoolFullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_keep_pool_full_enable),
            },
            FieldInfoData {
                name: "TextureLayerCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_layer_culling_enable),
            },
            FieldInfoData {
                name: "GpuTextureCompressionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, gpu_texture_compression_enable),
            },
            FieldInfoData {
                name: "TextureDirtyRetryRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_dirty_retry_rate),
            },
            FieldInfoData {
                name: "TextureForceDrawPass",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_force_draw_pass),
            },
            FieldInfoData {
                name: "UseVTMaxLevelAsPatchLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, use_v_t_max_level_as_patch_level),
            },
            FieldInfoData {
                name: "TextureStreamingPrioritizationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_streaming_prioritization_enable),
            },
            FieldInfoData {
                name: "DetailTextureStreamingPrioritizationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_texture_streaming_prioritization_enable),
            },
            FieldInfoData {
                name: "MeshScatteringMeshStreamingPrioritizationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_mesh_streaming_prioritization_enable),
            },
            FieldInfoData {
                name: "TextureQuadsPerTileLevel",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, texture_quads_per_tile_level),
            },
            FieldInfoData {
                name: "PrioritizationOcclusionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, prioritization_occlusion_enable),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_enable),
            },
            FieldInfoData {
                name: "DrawPatchesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_patches_enable),
            },
            FieldInfoData {
                name: "DetailOverlayEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_overlay_enable),
            },
            FieldInfoData {
                name: "DrawInstancingStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_instancing_stats),
            },
            FieldInfoData {
                name: "DecalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, decal_enable),
            },
            FieldInfoData {
                name: "ForceDecalReducedQuality",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, force_decal_reduced_quality),
            },
            FieldInfoData {
                name: "DrawDecal2dEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_decal2d_enable),
            },
            FieldInfoData {
                name: "DrawDecal3dEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_decal3d_enable),
            },
            FieldInfoData {
                name: "DrawDecalZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_decal_z_pass_enable),
            },
            FieldInfoData {
                name: "DrawOnlyDecalZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_only_decal_z_pass_enable),
            },
            FieldInfoData {
                name: "DecalZPassDrawDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, decal_z_pass_draw_distance),
            },
            FieldInfoData {
                name: "DecalOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, decal_offset_y),
            },
            FieldInfoData {
                name: "Decal3dFarDrawDistanceScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, decal3d_far_draw_distance_scale_factor),
            },
            FieldInfoData {
                name: "DrawPatchBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_patch_boxes_enable),
            },
            FieldInfoData {
                name: "DrawBadPatchesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_bad_patches_enable),
            },
            FieldInfoData {
                name: "DrawTextureTileBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_texture_tile_boxes_enable),
            },
            FieldInfoData {
                name: "DrawDebugTextEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_text_enable),
            },
            FieldInfoData {
                name: "DrawDebugTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_textures_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtrees_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreeZoomIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_zoom_index),
            },
            FieldInfoData {
                name: "DrawQuadtreeStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_stats_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreeAtlasTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_atlas_textures_enable),
            },
            FieldInfoData {
                name: "DrawQuadtreeAtlasTexturesScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_quadtree_atlas_textures_scale),
            },
            FieldInfoData {
                name: "DrawIndirectionTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_indirection_textures_enable),
            },
            FieldInfoData {
                name: "QuickscopePoolStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, quickscope_pool_stats_enable),
            },
            FieldInfoData {
                name: "DrawDynamicMask",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_dynamic_mask),
            },
            FieldInfoData {
                name: "DrawDebugTilePriorityQuadtreeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_tile_priority_quadtree_enable),
            },
            FieldInfoData {
                name: "DrawWaterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_water_enable),
            },
            FieldInfoData {
                name: "PatchErrorFovEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_error_fov_enable),
            },
            FieldInfoData {
                name: "PatchErrorFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_error_fov),
            },
            FieldInfoData {
                name: "ZPassDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, z_pass_distance),
            },
            FieldInfoData {
                name: "DebugOverlayGridEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_grid_enable),
            },
            FieldInfoData {
                name: "DebugOverlayGridSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_grid_size),
            },
            FieldInfoData {
                name: "DebugOverlayIsolinesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_isolines_enable),
            },
            FieldInfoData {
                name: "DebugOverlayIsolineSpacing",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_isoline_spacing),
            },
            FieldInfoData {
                name: "DebugOverlayWireframeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_wireframe_enable),
            },
            FieldInfoData {
                name: "DebugOverlaySketchTextureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_sketch_texture_enable),
            },
            FieldInfoData {
                name: "DebugOverlayBrushEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, debug_overlay_brush_enable),
            },
            FieldInfoData {
                name: "ForceGraphicsDriverCrash",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, force_graphics_driver_crash),
            },
            FieldInfoData {
                name: "ForcePatchRebuildEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, force_patch_rebuild_enable),
            },
            FieldInfoData {
                name: "DestroyAll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, destroy_all),
            },
            FieldInfoData {
                name: "SlotReuseWaitCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, slot_reuse_wait_count),
            },
            FieldInfoData {
                name: "SlotDebugOutputEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, slot_debug_output_enable),
            },
            FieldInfoData {
                name: "UpdateJobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, update_jobs_enable),
            },
            FieldInfoData {
                name: "BuildJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, build_job_count),
            },
            FieldInfoData {
                name: "RegenerateTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, regenerate_textures_enable),
            },
            FieldInfoData {
                name: "DynamicMaskEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, dynamic_mask_enable),
            },
            FieldInfoData {
                name: "MaxNonVisibleTextureUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, max_non_visible_texture_update_count),
            },
            FieldInfoData {
                name: "PatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_faces_per_side),
            },
            FieldInfoData {
                name: "TessellationFacesPerSideMin",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, tessellation_faces_per_side_min),
            },
            FieldInfoData {
                name: "PatchSlotCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_slot_count),
            },
            FieldInfoData {
                name: "PatchLodTransitionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_lod_transitions_enable),
            },
            FieldInfoData {
                name: "PatchLodTransitionsVertexShaderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_lod_transitions_vertex_shader_enable),
            },
            FieldInfoData {
                name: "PatchLodTransitionsVertexShaderDisableFixup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_lod_transitions_vertex_shader_disable_fixup),
            },
            FieldInfoData {
                name: "PatchMaterialSortingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, patch_material_sorting_enable),
            },
            FieldInfoData {
                name: "CullSampleBoundingBoxHeightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, cull_sample_bounding_box_height_enable),
            },
            FieldInfoData {
                name: "ForceShadowsOff",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, force_shadows_off),
            },
            FieldInfoData {
                name: "TerrainCastShadowsQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, terrain_cast_shadows_quality_level),
            },
            FieldInfoData {
                name: "CastPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, cast_planar_reflection_enable),
            },
            FieldInfoData {
                name: "CastEnvmapReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, cast_envmap_reflection_enable),
            },
            FieldInfoData {
                name: "CastDecal3dPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, cast_decal3d_planar_reflection_enable),
            },
            FieldInfoData {
                name: "CastDecal3dEnvmapReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, cast_decal3d_envmap_reflection_enable),
            },
            FieldInfoData {
                name: "DetailDisplacementInShadowViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_in_shadow_view_enable),
            },
            FieldInfoData {
                name: "GlobalColormapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, global_colormap_enable),
            },
            FieldInfoData {
                name: "OccluderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluder_enable),
            },
            FieldInfoData {
                name: "OccluderJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluder_job_enable),
            },
            FieldInfoData {
                name: "OccluderJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluder_job_count),
            },
            FieldInfoData {
                name: "OccluderPatchFacesPerSide",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluder_patch_faces_per_side),
            },
            FieldInfoData {
                name: "OccluderLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluder_lod_scale),
            },
            FieldInfoData {
                name: "OccluderBackFaceCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluder_back_face_culling_enable),
            },
            FieldInfoData {
                name: "OccludedEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluded_enable),
            },
            FieldInfoData {
                name: "OccludedMinDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, occluded_min_distance),
            },
            FieldInfoData {
                name: "MeshScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_enable),
            },
            FieldInfoData {
                name: "MeshScatteringReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_reflections_enable),
            },
            FieldInfoData {
                name: "MeshScatteringJobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_jobs_enable),
            },
            FieldInfoData {
                name: "MeshScatteringCastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_cast_shadows_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringCellBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_cell_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringBatchBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_batch_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringNodeBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_node_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringCulledCellBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_culled_cell_boxes_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringDebugMaskScaleTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_debug_mask_scale_textures_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_stats_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringQuadtreeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_quadtree_enable),
            },
            FieldInfoData {
                name: "MeshScatteringCellPoolCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_cell_pool_capacity),
            },
            FieldInfoData {
                name: "MeshScatteringTreeNodePoolCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_tree_node_pool_capacity),
            },
            FieldInfoData {
                name: "MeshScatteringInvisibleCellFovFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_invisible_cell_fov_factor),
            },
            FieldInfoData {
                name: "MeshScatteringForceUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_force_update_enable),
            },
            FieldInfoData {
                name: "MeshScatteringCullRecordCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_cull_record_count),
            },
            FieldInfoData {
                name: "MeshScatteringBuildChannelCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_build_channel_count),
            },
            FieldInfoData {
                name: "MeshScatteringBuildChannelsLaunchedPerFrameCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_build_channels_launched_per_frame_count_max),
            },
            FieldInfoData {
                name: "MeshScatteringBuildVisibleFirst",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_build_visible_first),
            },
            FieldInfoData {
                name: "MeshScatteringClodFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_clod_frame_count),
            },
            FieldInfoData {
                name: "MeshScatteringWindSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_wind_speed),
            },
            FieldInfoData {
                name: "MeshScatteringInstancesPerCellMax",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instances_per_cell_max),
            },
            FieldInfoData {
                name: "MeshScatteringDensityMarginFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_density_margin_factor),
            },
            FieldInfoData {
                name: "MeshScatteringPregenerationDistanceRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_pregeneration_distance_ratio),
            },
            FieldInfoData {
                name: "MeshScatteringKeepDistanceRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_keep_distance_ratio),
            },
            FieldInfoData {
                name: "MeshScatteringMergeInstanceLists",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_merge_instance_lists),
            },
            FieldInfoData {
                name: "MeshScatteringVirtualTextureBlurriness",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_virtual_texture_blurriness),
            },
            FieldInfoData {
                name: "MeshScatteringShadowViewCullSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_shadow_view_cull_size),
            },
            FieldInfoData {
                name: "MeshScatteringDistanceScaleFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_distance_scale_factor),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_job_count),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullListCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_list_count),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullBoxTestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_box_test_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceFrustumCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_frustum_cull_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceAdditionalCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_additional_cull_enable),
            },
            FieldInfoData {
                name: "DrawMeshScatteringInstanceBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_mesh_scattering_instance_boxes_enable),
            },
            FieldInfoData {
                name: "MeshScatteringInstanceCullDynamicAllocEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_instance_cull_dynamic_alloc_enable),
            },
            FieldInfoData {
                name: "MeshScatteringWindEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_wind_enable),
            },
            FieldInfoData {
                name: "MeshScatteringDrawMotionVectorsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_draw_motion_vectors_enable),
            },
            FieldInfoData {
                name: "MeshScatteringSnappingGridMultiplierVertical",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_snapping_grid_multiplier_vertical),
            },
            FieldInfoData {
                name: "MeshScatteringSnappingGridMultiplierHorizontal",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, mesh_scattering_snapping_grid_multiplier_horizontal),
            },
            FieldInfoData {
                name: "DetailDisplacementQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_quality_level),
            },
            FieldInfoData {
                name: "DetailDisplacementEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_enable),
            },
            FieldInfoData {
                name: "DrawDetailDisplacementEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_detail_displacement_enable),
            },
            FieldInfoData {
                name: "DrawDebugDetailDisplacementEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_debug_detail_displacement_enable),
            },
            FieldInfoData {
                name: "DrawDetailDisplacementTreeLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, draw_detail_displacement_tree_level),
            },
            FieldInfoData {
                name: "DetailDisplacementMaxTessFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_max_tess_factor),
            },
            FieldInfoData {
                name: "DetailDisplacementScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_scale),
            },
            FieldInfoData {
                name: "DetailDisplacementBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_bias),
            },
            FieldInfoData {
                name: "DetailDisplacementFadeRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, detail_displacement_fade_range),
            },
            FieldInfoData {
                name: "HeightFieldTessellationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, height_field_tessellation_factor),
            },
            FieldInfoData {
                name: "HeightFieldTessellationFadeRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VisualTerrainSettings, height_field_tessellation_fade_range),
            },
        ],
    }),
    array_type: Some(VISUALTERRAINSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VisualTerrainSettings {
    fn type_info() -> &'static TypeInfo {
        VISUALTERRAINSETTINGS_TYPE_INFO
    }
}


pub const VISUALTERRAINSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrainSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("VisualTerrainSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const TERRAINRENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "TerrainRender",
    data: TypeInfoData::Enum,
    array_type: Some(TERRAINRENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TerrainRenderMode {
    fn type_info() -> &'static TypeInfo {
        TERRAINRENDERMODE_TYPE_INFO
    }
}


pub const TERRAINRENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainRenderMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VisualTerrain {
}

pub const VISUALTERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IVISUALTERRAIN_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VISUALTERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VisualTerrain {
    fn type_info() -> &'static TypeInfo {
        VISUALTERRAIN_TYPE_INFO
    }
}


pub const VISUALTERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisualTerrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("VisualTerrain-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainLayerCombinations {
}

pub const TERRAINLAYERCOMBINATIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinations",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TERRAINLAYERCOMBINATIONS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainLayerCombinations {
    fn type_info() -> &'static TypeInfo {
        TERRAINLAYERCOMBINATIONS_TYPE_INFO
    }
}


pub const TERRAINLAYERCOMBINATIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainLayerCombinations-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainLayerCombinations-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainDecals {
}

pub const TERRAINDECALS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDecals",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TERRAINDECALS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainDecals {
    fn type_info() -> &'static TypeInfo {
        TERRAINDECALS_TYPE_INFO
    }
}


pub const TERRAINDECALS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainDecals-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainDecals-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IVisualTerrain {
}

pub const IVISUALTERRAIN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVisualTerrain",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITERRAIN_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IVISUALTERRAIN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IVisualTerrain {
    fn type_info() -> &'static TypeInfo {
        IVISUALTERRAIN_TYPE_INFO
    }
}


pub const IVISUALTERRAIN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVisualTerrain-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("IVisualTerrain-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TerrainTextureTree {
}

pub const TERRAINTEXTURETREE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainTextureTree",
    flags: MemberInfoFlags::new(101),
    module: "TerrainRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TERRAINTEXTURETREE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TerrainTextureTree {
    fn type_info() -> &'static TypeInfo {
        TERRAINTEXTURETREE_TYPE_INFO
    }
}


pub const TERRAINTEXTURETREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TerrainTextureTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainRender",
    data: TypeInfoData::Array("TerrainTextureTree-Array"),
    array_type: None,
    alignment: 8,
};


