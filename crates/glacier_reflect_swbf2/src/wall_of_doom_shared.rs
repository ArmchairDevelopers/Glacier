use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Debug)]
pub struct WallOfDoomMeshEntityData {
    pub show_curvature: bool,
    pub vertex_perturbation_texture: super::render::TextureAsset,
    pub perturbation_max_scale: f32,
    pub perturbation_max_scale_height: f32,
    pub perturbation_min_height: f32,
}

pub const WALLOFDOOMMESHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomMeshEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoomShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATICMODELENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShowCurvature",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomMeshEntityData, show_curvature),
            },
            FieldInfoData {
                name: "VertexPerturbationTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomMeshEntityData, vertex_perturbation_texture),
            },
            FieldInfoData {
                name: "PerturbationMaxScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomMeshEntityData, perturbation_max_scale),
            },
            FieldInfoData {
                name: "PerturbationMaxScaleHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomMeshEntityData, perturbation_max_scale_height),
            },
            FieldInfoData {
                name: "PerturbationMinHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomMeshEntityData, perturbation_min_height),
            },
        ],
    }),
    array_type: Some(WALLOFDOOMMESHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WallOfDoomMeshEntityData {
    fn type_info() -> &'static TypeInfo {
        WALLOFDOOMMESHENTITYDATA_TYPE_INFO
    }
}


pub const WALLOFDOOMMESHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomMeshEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoomShared",
    data: TypeInfoData::Array("WallOfDoomMeshEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WallOfDoomEntityData {
    pub wall_of_doom_blueprint: super::entity::ObjectBlueprint,
    pub wall_of_doom_blend_blueprint: super::entity::ObjectBlueprint,
    pub mesh_segment_length: f32,
    pub mesh_segment_height: f32,
    pub mesh_blending_segment_length: f32,
    pub radius: f32,
    pub center: super::core::Vec3,
    pub min_max_pairs: Vec<f32>,
    pub meta_data: WallOfDoomHeightmapMetaData,
    pub height_map: super::render::TextureAsset,
    pub effect_parameters: Vec<super::effect_base::EffectParameter>,
    pub wall_visible: bool,
    pub v_f_x_visible: bool,
}

pub const WALLOFDOOMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WallOfDoomShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WallOfDoomBlueprint",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, wall_of_doom_blueprint),
            },
            FieldInfoData {
                name: "WallOfDoomBlendBlueprint",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, wall_of_doom_blend_blueprint),
            },
            FieldInfoData {
                name: "MeshSegmentLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, mesh_segment_length),
            },
            FieldInfoData {
                name: "MeshSegmentHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, mesh_segment_height),
            },
            FieldInfoData {
                name: "MeshBlendingSegmentLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, mesh_blending_segment_length),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, radius),
            },
            FieldInfoData {
                name: "Center",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, center),
            },
            FieldInfoData {
                name: "MinMaxPairs",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, min_max_pairs),
            },
            FieldInfoData {
                name: "MetaData",
                flags: MemberInfoFlags::new(0),
                field_type: WALLOFDOOMHEIGHTMAPMETADATA_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, meta_data),
            },
            FieldInfoData {
                name: "HeightMap",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, height_map),
            },
            FieldInfoData {
                name: "EffectParameters",
                flags: MemberInfoFlags::new(144),
                field_type: EFFECTPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, effect_parameters),
            },
            FieldInfoData {
                name: "WallVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, wall_visible),
            },
            FieldInfoData {
                name: "VFXVisible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomEntityData, v_f_x_visible),
            },
        ],
    }),
    array_type: Some(WALLOFDOOMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WallOfDoomEntityData {
    fn type_info() -> &'static TypeInfo {
        WALLOFDOOMENTITYDATA_TYPE_INFO
    }
}


pub const WALLOFDOOMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoomShared",
    data: TypeInfoData::Array("WallOfDoomEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const WALLOFDOOMHEIGHTMAPMETADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomHeightmapMetaData",
    flags: MemberInfoFlags::new(36937),
    module: "WallOfDoomShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "HeightMapWidth",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, height_map_width),
            },
            FieldInfoData {
                name: "HeightMapHeight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, height_map_height),
            },
            FieldInfoData {
                name: "WorldExtentsMinX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_min_x),
            },
            FieldInfoData {
                name: "WorldExtentsMinY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_min_y),
            },
            FieldInfoData {
                name: "WorldExtentsMinZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_min_z),
            },
            FieldInfoData {
                name: "WorldExtentsMaxX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_max_x),
            },
            FieldInfoData {
                name: "WorldExtentsMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_max_y),
            },
            FieldInfoData {
                name: "WorldExtentsMaxZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, world_extents_max_z),
            },
            FieldInfoData {
                name: "MinMaxTextureRatio",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, min_max_texture_ratio),
            },
            FieldInfoData {
                name: "MinMaxDataWidth",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, min_max_data_width),
            },
            FieldInfoData {
                name: "MinMaxDataHeight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WallOfDoomHeightmapMetaData, min_max_data_height),
            },
        ],
    }),
    array_type: Some(WALLOFDOOMHEIGHTMAPMETADATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WallOfDoomHeightmapMetaData {
    fn type_info() -> &'static TypeInfo {
        WALLOFDOOMHEIGHTMAPMETADATA_TYPE_INFO
    }
}


pub const WALLOFDOOMHEIGHTMAPMETADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WallOfDoomHeightmapMetaData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WallOfDoomShared",
    data: TypeInfoData::Array("WallOfDoomHeightmapMetaData-Array"),
    array_type: None,
    alignment: 8,
};


