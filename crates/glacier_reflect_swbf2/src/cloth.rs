use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_cloth_types(registry: &mut TypeRegistry) {
    registry.register_type(CLOTHASSET_TYPE_INFO);
    registry.register_type(CLOTHASSET_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(CLOTHSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CLOTHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHASSETINSTANCE_TYPE_INFO);
    registry.register_type(CLOTHASSETINSTANCE_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHCOLLISIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CLOTHCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHOBJECTBLUEPRINT_TYPE_INFO);
    registry.register_type(CLOTHOBJECTBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHENTITYDATA_TYPE_INFO);
    registry.register_type(CLOTHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_TYPE_INFO);
    registry.register_type(CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHINSTANCEOBSERVERENTITYDATA_TYPE_INFO);
    registry.register_type(CLOTHINSTANCEOBSERVERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHDEBUGRENDERERSETTINGS_TYPE_INFO);
    registry.register_type(CLOTHDEBUGRENDERERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHCOLLISIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLOTHCOLLISIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHENTITY_TYPE_INFO);
    registry.register_type(CLOTHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHMANAGER_TYPE_INFO);
    registry.register_type(CLOTHMANAGER_ARRAY_TYPE_INFO);
    registry.register_type(EACLOTHMEMORYINITIALIZER_TYPE_INFO);
    registry.register_type(EACLOTHMEMORYINITIALIZER_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHCOMPONENT_TYPE_INFO);
    registry.register_type(CLOTHCOMPONENT_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothAsset {
    pub runtime_rig_bone_count: u32,
}

pub const CLOTHASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAsset",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RuntimeRigBoneCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClothAsset, runtime_rig_bone_count),
            },
        ],
    }),
    array_type: Some(CLOTHASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothAsset {
    fn type_info() -> &'static TypeInfo {
        CLOTHASSET_TYPE_INFO
    }
}


pub const CLOTHASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClothSystemSettings {
    pub client_cloth_world_thread_count: u32,
    pub cloth_system_quality_level: super::core::QualityLevel,
    pub enable_render_dt_rustler: bool,
    pub render_dt_rustler_short_frame_probability: f32,
    pub render_dt_rustler_short_frame_min_dt: f32,
    pub render_dt_rustler_short_frame_max_dt: f32,
    pub render_dt_rustler_long_frame_probability: f32,
    pub render_dt_rustler_long_frame_min_dt: f32,
    pub render_dt_rustler_long_frame_max_dt: f32,
    pub enable_render_dt_smoother: bool,
    pub render_dt_smoother_window_size: u32,
    pub render_dt_smoother_min_dt: f32,
    pub render_dt_smoother_max_dt: f32,
    pub sim_prevents_previous_dt0: bool,
}

pub const CLOTHSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ClientClothWorldThreadCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, client_cloth_world_thread_count),
            },
            FieldInfoData {
                name: "ClothSystemQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, cloth_system_quality_level),
            },
            FieldInfoData {
                name: "EnableRenderDtRustler",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, enable_render_dt_rustler),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_probability),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameMinDt",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_min_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameMaxDt",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_max_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_probability),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameMinDt",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_min_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameMaxDt",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_max_dt),
            },
            FieldInfoData {
                name: "EnableRenderDtSmoother",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, enable_render_dt_smoother),
            },
            FieldInfoData {
                name: "RenderDtSmootherWindowSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_window_size),
            },
            FieldInfoData {
                name: "RenderDtSmootherMinDt",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_min_dt),
            },
            FieldInfoData {
                name: "RenderDtSmootherMaxDt",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_max_dt),
            },
            FieldInfoData {
                name: "SimPreventsPreviousDt0",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothSystemSettings, sim_prevents_previous_dt0),
            },
        ],
    }),
    array_type: Some(CLOTHSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothSystemSettings {
    fn type_info() -> &'static TypeInfo {
        CLOTHSYSTEMSETTINGS_TYPE_INFO
    }
}


pub const CLOTHSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothSystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClothComponentData {
    pub instances: Vec<ClothAssetInstance>,
    pub object_variations: super::entity::ObjectVariationCollection,
}

pub const CLOTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Instances",
                flags: MemberInfoFlags::new(144),
                field_type: CLOTHASSETINSTANCE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ClothComponentData, instances),
            },
            FieldInfoData {
                name: "ObjectVariations",
                flags: MemberInfoFlags::new(0),
                field_type: OBJECTVARIATIONCOLLECTION_TYPE_INFO,
                rust_offset: offset_of!(ClothComponentData, object_variations),
            },
        ],
    }),
    array_type: Some(CLOTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothComponentData {
    fn type_info() -> &'static TypeInfo {
        CLOTHCOMPONENTDATA_TYPE_INFO
    }
}


pub const CLOTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothAssetInstance {
    pub cloth: ClothObjectBlueprint,
}

pub const CLOTHASSETINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAssetInstance",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Cloth",
                flags: MemberInfoFlags::new(0),
                field_type: CLOTHOBJECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(ClothAssetInstance, cloth),
            },
        ],
    }),
    array_type: Some(CLOTHASSETINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothAssetInstance {
    fn type_info() -> &'static TypeInfo {
        CLOTHASSETINSTANCE_TYPE_INFO
    }
}


pub const CLOTHASSETINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAssetInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothAssetInstance-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClothCollisionComponentData {
    pub geometries: Vec<super::cloth_base::ClothCollisionGeometry>,
    pub source_part_range: Vec<u32>,
}

pub const CLOTHCOLLISIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Geometries",
                flags: MemberInfoFlags::new(144),
                field_type: CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ClothCollisionComponentData, geometries),
            },
            FieldInfoData {
                name: "SourcePartRange",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ClothCollisionComponentData, source_part_range),
            },
        ],
    }),
    array_type: Some(CLOTHCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothCollisionComponentData {
    fn type_info() -> &'static TypeInfo {
        CLOTHCOLLISIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const CLOTHCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothCollisionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothObjectBlueprint {
}

pub const CLOTHOBJECTBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTBLUEPRINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLOTHOBJECTBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothObjectBlueprint {
    fn type_info() -> &'static TypeInfo {
        CLOTHOBJECTBLUEPRINT_TYPE_INFO
    }
}


pub const CLOTHOBJECTBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothObjectBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClothEntityData {
    pub mesh: super::render_base::MeshBaseAsset,
    pub mesh_name_hash: u32,
    pub cloth: ClothAsset,
    pub character_lighting_enable: bool,
    pub enabled: bool,
    pub wind_scale: f32,
    pub wrap_on_g_p_u: bool,
    pub solve_on_render_side: bool,
    pub use_for_bounding_box_calculations: bool,
    pub cloth_wrapping: super::cloth_base::ClothWrappingAsset,
    pub ground_plane_transform: super::core::LinearTransform,
    pub enable_ground_plane_collision: bool,
    pub local_ground_plane_transform: bool,
    pub activation_radius: super::core::QualityScalableFloat,
    pub smooth_blend_out_distance: super::core::QualityScalableFloat,
    pub collision_layer: super::physics::RigidBodyCollisionLayer,
}

pub const CLOTHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, mesh),
            },
            FieldInfoData {
                name: "MeshNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, mesh_name_hash),
            },
            FieldInfoData {
                name: "Cloth",
                flags: MemberInfoFlags::new(0),
                field_type: CLOTHASSET_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, cloth),
            },
            FieldInfoData {
                name: "CharacterLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, character_lighting_enable),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, enabled),
            },
            FieldInfoData {
                name: "WindScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, wind_scale),
            },
            FieldInfoData {
                name: "WrapOnGPU",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, wrap_on_g_p_u),
            },
            FieldInfoData {
                name: "SolveOnRenderSide",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, solve_on_render_side),
            },
            FieldInfoData {
                name: "UseForBoundingBoxCalculations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, use_for_bounding_box_calculations),
            },
            FieldInfoData {
                name: "ClothWrapping",
                flags: MemberInfoFlags::new(0),
                field_type: CLOTHWRAPPINGASSET_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, cloth_wrapping),
            },
            FieldInfoData {
                name: "GroundPlaneTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, ground_plane_transform),
            },
            FieldInfoData {
                name: "EnableGroundPlaneCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, enable_ground_plane_collision),
            },
            FieldInfoData {
                name: "LocalGroundPlaneTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, local_ground_plane_transform),
            },
            FieldInfoData {
                name: "ActivationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, activation_radius),
            },
            FieldInfoData {
                name: "SmoothBlendOutDistance",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEFLOAT_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, smooth_blend_out_distance),
            },
            FieldInfoData {
                name: "CollisionLayer",
                flags: MemberInfoFlags::new(0),
                field_type: RIGIDBODYCOLLISIONLAYER_TYPE_INFO,
                rust_offset: offset_of!(ClothEntityData, collision_layer),
            },
        ],
    }),
    array_type: Some(CLOTHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothEntityData {
    fn type_info() -> &'static TypeInfo {
        CLOTHENTITYDATA_TYPE_INFO
    }
}


pub const CLOTHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothObjectVariationExampleEntityData {
    pub variations: Vec<super::entity::ObjectVariation>,
}

pub const CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectVariationExampleEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Variations",
                flags: MemberInfoFlags::new(144),
                field_type: OBJECTVARIATION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ClothObjectVariationExampleEntityData, variations),
            },
        ],
    }),
    array_type: Some(CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothObjectVariationExampleEntityData {
    fn type_info() -> &'static TypeInfo {
        CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_TYPE_INFO
    }
}


pub const CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectVariationExampleEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothObjectVariationExampleEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothInstanceObserverEntityData {
    pub cloth_index_in_component: u32,
    pub force_processing_mode_callback: bool,
}

pub const CLOTHINSTANCEOBSERVERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothInstanceObserverEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ClothIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ClothInstanceObserverEntityData, cloth_index_in_component),
            },
            FieldInfoData {
                name: "ForceProcessingModeCallback",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothInstanceObserverEntityData, force_processing_mode_callback),
            },
        ],
    }),
    array_type: Some(CLOTHINSTANCEOBSERVERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothInstanceObserverEntityData {
    fn type_info() -> &'static TypeInfo {
        CLOTHINSTANCEOBSERVERENTITYDATA_TYPE_INFO
    }
}


pub const CLOTHINSTANCEOBSERVERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothInstanceObserverEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothInstanceObserverEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClothDebugRendererSettings {
    pub sim_mesh_line_color: super::core::Vec4,
    pub sim_mesh_face_color: super::core::Vec4,
    pub anchor_mesh_line_color: super::core::Vec4,
    pub anchor_mesh_face_color: super::core::Vec4,
    pub anchor_distance_mesh_line_color: super::core::Vec4,
    pub anchor_distance_mesh_face_color: super::core::Vec4,
    pub penetration_mesh_line_color: super::core::Vec4,
    pub penetration_mesh_face_color: super::core::Vec4,
    pub sphere_collider_line_color: super::core::Vec4,
    pub sphere_collider_face_color: super::core::Vec4,
    pub capsule_collider_line_color: super::core::Vec4,
    pub capsule_collider_face_color: super::core::Vec4,
    pub tapered_capsule_collider_line_color: super::core::Vec4,
    pub tapered_capsule_collider_face_color: super::core::Vec4,
    pub box_collider_line_color: super::core::Vec4,
    pub box_collider_face_color: super::core::Vec4,
    pub ground_plane_line_color: super::core::Vec4,
    pub disabled_ground_plane_line_color: super::core::Vec4,
    pub cloth_point_collider_line_color: super::core::Vec4,
    pub activation_radius_color: super::core::Vec4,
}

pub const CLOTHDEBUGRENDERERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothDebugRendererSettings",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SimMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, sim_mesh_line_color),
            },
            FieldInfoData {
                name: "SimMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, sim_mesh_face_color),
            },
            FieldInfoData {
                name: "AnchorMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_mesh_line_color),
            },
            FieldInfoData {
                name: "AnchorMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_mesh_face_color),
            },
            FieldInfoData {
                name: "AnchorDistanceMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_distance_mesh_line_color),
            },
            FieldInfoData {
                name: "AnchorDistanceMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_distance_mesh_face_color),
            },
            FieldInfoData {
                name: "PenetrationMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, penetration_mesh_line_color),
            },
            FieldInfoData {
                name: "PenetrationMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, penetration_mesh_face_color),
            },
            FieldInfoData {
                name: "SphereColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, sphere_collider_line_color),
            },
            FieldInfoData {
                name: "SphereColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, sphere_collider_face_color),
            },
            FieldInfoData {
                name: "CapsuleColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, capsule_collider_line_color),
            },
            FieldInfoData {
                name: "CapsuleColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, capsule_collider_face_color),
            },
            FieldInfoData {
                name: "TaperedCapsuleColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, tapered_capsule_collider_line_color),
            },
            FieldInfoData {
                name: "TaperedCapsuleColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, tapered_capsule_collider_face_color),
            },
            FieldInfoData {
                name: "BoxColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, box_collider_line_color),
            },
            FieldInfoData {
                name: "BoxColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, box_collider_face_color),
            },
            FieldInfoData {
                name: "GroundPlaneLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, ground_plane_line_color),
            },
            FieldInfoData {
                name: "DisabledGroundPlaneLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, disabled_ground_plane_line_color),
            },
            FieldInfoData {
                name: "ClothPointColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, cloth_point_collider_line_color),
            },
            FieldInfoData {
                name: "ActivationRadiusColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ClothDebugRendererSettings, activation_radius_color),
            },
        ],
    }),
    array_type: Some(CLOTHDEBUGRENDERERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothDebugRendererSettings {
    fn type_info() -> &'static TypeInfo {
        CLOTHDEBUGRENDERERSETTINGS_TYPE_INFO
    }
}


pub const CLOTHDEBUGRENDERERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothDebugRendererSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothDebugRendererSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothCollisionComponent {
}

pub const CLOTHCOLLISIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLOTHCOLLISIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClothCollisionComponent {
    fn type_info() -> &'static TypeInfo {
        CLOTHCOLLISIONCOMPONENT_TYPE_INFO
    }
}


pub const CLOTHCOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothCollisionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothEntity {
}

pub const CLOTHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntity",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLOTHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClothEntity {
    fn type_info() -> &'static TypeInfo {
        CLOTHENTITY_TYPE_INFO
    }
}


pub const CLOTHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothManager {
}

pub const CLOTHMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothManager",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOSUBSYSTEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLOTHMANAGER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothManager {
    fn type_info() -> &'static TypeInfo {
        CLOTHMANAGER_TYPE_INFO
    }
}


pub const CLOTHMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothManager-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EAClothMemoryInitializer {
}

pub const EACLOTHMEMORYINITIALIZER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothMemoryInitializer",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOMODULEINITIALIZER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EACLOTHMEMORYINITIALIZER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EAClothMemoryInitializer {
    fn type_info() -> &'static TypeInfo {
        EACLOTHMEMORYINITIALIZER_TYPE_INFO
    }
}


pub const EACLOTHMEMORYINITIALIZER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothMemoryInitializer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("EAClothMemoryInitializer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothComponent {
}

pub const CLOTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponent",
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLOTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClothComponent {
    fn type_info() -> &'static TypeInfo {
        CLOTHCOMPONENT_TYPE_INFO
    }
}


pub const CLOTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothComponent-Array"),
    array_type: None,
    alignment: 8,
};


