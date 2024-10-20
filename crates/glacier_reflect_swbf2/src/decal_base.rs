use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Debug)]
pub struct DecalVolumeEntityData {
}

pub const DECALVOLUMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RENDERVOLUMEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DECALVOLUMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DecalVolumeEntityData {
    fn type_info() -> &'static TypeInfo {
        DECALVOLUMEENTITYDATA_TYPE_INFO
    }
}


pub const DECALVOLUMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalVolumeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DecalEntityData {
    pub decal_type: super::render_base::DecalType,
    pub clip_angle: f32,
    pub sorting_priority: u8,
    pub atlas_tile: super::render_base::DecalAtlasTile,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub enabled: bool,
}

pub const DECALENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntityData",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DecalType",
                flags: MemberInfoFlags::new(0),
                field_type: DECALTYPE_TYPE_INFO,
                rust_offset: offset_of!(DecalEntityData, decal_type),
            },
            FieldInfoData {
                name: "ClipAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalEntityData, clip_angle),
            },
            FieldInfoData {
                name: "SortingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DecalEntityData, sorting_priority),
            },
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: DECALATLASTILE_TYPE_INFO,
                rust_offset: offset_of!(DecalEntityData, atlas_tile),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(DecalEntityData, shader),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalEntityData, enabled),
            },
        ],
    }),
    array_type: Some(DECALENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DecalEntityData {
    fn type_info() -> &'static TypeInfo {
        DECALENTITYDATA_TYPE_INFO
    }
}


pub const DECALENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DecalTemplateAsset {
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

pub const DECALTEMPLATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateAsset",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DECALTEMPLATEBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Size",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, size),
            },
            FieldInfoData {
                name: "RandomSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, random_size),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, rotation),
            },
            FieldInfoData {
                name: "RandomRotation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, random_rotation),
            },
            FieldInfoData {
                name: "ClipAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, clip_angle),
            },
            FieldInfoData {
                name: "ProximityRadiusFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, proximity_radius_factor),
            },
            FieldInfoData {
                name: "NormalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, normal_offset),
            },
            FieldInfoData {
                name: "SortingPriority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, sorting_priority),
            },
            FieldInfoData {
                name: "AtlasTile",
                flags: MemberInfoFlags::new(0),
                field_type: DECALATLASTILE_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, atlas_tile),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, shader),
            },
            FieldInfoData {
                name: "DecalType",
                flags: MemberInfoFlags::new(0),
                field_type: DECALTYPE_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, decal_type),
            },
            FieldInfoData {
                name: "ProjectMultiple",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, project_multiple),
            },
            FieldInfoData {
                name: "MeshUVIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, mesh_u_v_index),
            },
            FieldInfoData {
                name: "IgnoreSetupJobDistanceCulling",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalTemplateAsset, ignore_setup_job_distance_culling),
            },
        ],
    }),
    array_type: Some(DECALTEMPLATEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DecalTemplateAsset {
    fn type_info() -> &'static TypeInfo {
        DECALTEMPLATEASSET_TYPE_INFO
    }
}


pub const DECALTEMPLATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalTemplateAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalTemplateAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DecalSettings {
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

pub const DECALSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalSettings",
    flags: MemberInfoFlags::new(101),
    module: "DecalBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, enable),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, draw_enable),
            },
            FieldInfoData {
                name: "RingBufferSizeInVerts",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, ring_buffer_size_in_verts),
            },
            FieldInfoData {
                name: "RingBufferRecreateSizeInVerts",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, ring_buffer_recreate_size_in_verts),
            },
            FieldInfoData {
                name: "ProjectedDecalsTrianglesPerJob",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, projected_decals_triangles_per_job),
            },
            FieldInfoData {
                name: "DecalRecreateDistanceInMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, decal_recreate_distance_in_meters),
            },
            FieldInfoData {
                name: "DebugDrawEntityHandles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, debug_draw_entity_handles),
            },
            FieldInfoData {
                name: "CullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, cull_enable),
            },
            FieldInfoData {
                name: "DistanceCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, distance_cull_enable),
            },
            FieldInfoData {
                name: "FrustumCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, frustum_cull_enable),
            },
            FieldInfoData {
                name: "OcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, occlusion_cull_enable),
            },
            FieldInfoData {
                name: "DistanceCullFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, distance_cull_falloff),
            },
            FieldInfoData {
                name: "MinOcclusionTestDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, min_occlusion_test_distance),
            },
            FieldInfoData {
                name: "MinOcclusionScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, min_occlusion_screen_area),
            },
            FieldInfoData {
                name: "DebugOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, debug_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "DebugBatches",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, debug_batches),
            },
            FieldInfoData {
                name: "DebugPerEntityBatches",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, debug_per_entity_batches),
            },
            FieldInfoData {
                name: "WireframeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, wireframe_enable),
            },
            FieldInfoData {
                name: "DebugRingbuffer",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, debug_ringbuffer),
            },
            FieldInfoData {
                name: "DisplacementBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, displacement_bias),
            },
            FieldInfoData {
                name: "DisplacementScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, displacement_scale),
            },
            FieldInfoData {
                name: "MaxDecalObjectPrims",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, max_decal_object_prims),
            },
            FieldInfoData {
                name: "MaxTotalDecalObjectPrims",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, max_total_decal_object_prims),
            },
            FieldInfoData {
                name: "DecalMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DecalSettings, decal_max_distance),
            },
        ],
    }),
    array_type: Some(DECALSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DecalSettings {
    fn type_info() -> &'static TypeInfo {
        DECALSETTINGS_TYPE_INFO
    }
}


pub const DECALSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "DecalBase",
    data: TypeInfoData::Array("DecalSettings-Array"),
    array_type: None,
    alignment: 8,
};


