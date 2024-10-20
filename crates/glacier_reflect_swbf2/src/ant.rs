use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_ant_types(registry: &mut TypeRegistry) {
    registry.register_type(ASSETBANK_TYPE_INFO);
    registry.register_type(ASSETBANK_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATABLE_TYPE_INFO);
    registry.register_type(ANIMATABLE_ARRAY_TYPE_INFO);
    registry.register_type(SCENEOPMATRIX_TYPE_INFO);
    registry.register_type(SCENEOPMATRIX_ARRAY_TYPE_INFO);
    registry.register_type(WAYPOINTVAULTTYPE_TYPE_INFO);
    registry.register_type(WAYPOINTVAULTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(ANTSETTINGS_TYPE_INFO);
    registry.register_type(ANTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ANTREF_TYPE_INFO);
    registry.register_type(ANTREF_ARRAY_TYPE_INFO);
    registry.register_type(ANTREFTYPE_TYPE_INFO);
    registry.register_type(ANTREFTYPE_ARRAY_TYPE_INFO);
    registry.register_type(GAMESTATEVALUETYPEENUM_TYPE_INFO);
    registry.register_type(GAMESTATEVALUETYPEENUM_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATIONSETASSET_TYPE_INFO);
    registry.register_type(ANTANIMATIONSETASSET_ARRAY_TYPE_INFO);
    registry.register_type(ANTANIMATABLEDATA_TYPE_INFO);
    registry.register_type(ANTANIMATABLEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTPROJECTASSET_TYPE_INFO);
    registry.register_type(ANTPROJECTASSET_ARRAY_TYPE_INFO);
    registry.register_type(ANTSTATEHIERARCHYNODE_TYPE_INFO);
    registry.register_type(ANTSTATEHIERARCHYNODE_ARRAY_TYPE_INFO);
    registry.register_type(ANTCBAASSET_TYPE_INFO);
    registry.register_type(ANTCBAASSET_ARRAY_TYPE_INFO);
    registry.register_type(ANTSTATEASSET_TYPE_INFO);
    registry.register_type(ANTSTATEASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENUMGUIDINDEXPAIR_TYPE_INFO);
    registry.register_type(ENUMGUIDINDEXPAIR_ARRAY_TYPE_INFO);
    registry.register_type(ANTSCENARIO_TYPE_INFO);
    registry.register_type(ANTSCENARIO_ARRAY_TYPE_INFO);
    registry.register_type(ANTREFINFO_TYPE_INFO);
    registry.register_type(ANTREFINFO_ARRAY_TYPE_INFO);
    registry.register_type(ANTPACKAGEHELPER_TYPE_INFO);
    registry.register_type(ANTPACKAGEHELPER_ARRAY_TYPE_INFO);
    registry.register_type(ANTPACKAGINGTYPE_TYPE_INFO);
    registry.register_type(ANTPACKAGINGTYPE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssetBank {
}

pub const ASSETBANK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetBank",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ASSETBANK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AssetBank {
    fn type_info() -> &'static TypeInfo {
        ASSETBANK_TYPE_INFO
    }
}


pub const ASSETBANK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetBank-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AssetBank-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Animatable {
}

pub const ANIMATABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Animatable",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ANIMATABLE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Animatable {
    fn type_info() -> &'static TypeInfo {
        ANIMATABLE_TYPE_INFO
    }
}


pub const ANIMATABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Animatable-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("Animatable-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SceneOpMatrix {
}

pub const SCENEOPMATRIX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SceneOpMatrix",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SCENEOPMATRIX_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SceneOpMatrix {
    fn type_info() -> &'static TypeInfo {
        SCENEOPMATRIX_TYPE_INFO
    }
}


pub const SCENEOPMATRIX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SceneOpMatrix-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("SceneOpMatrix-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WaypointVaultType {
    #[default]
    WaypointVaultType_VaultOverHigh = 0,
    WaypointVaultType_VaultOntoLow = 1,
    WaypointVaultType_VaultDownLow = 2,
    WaypointVaultType_VaultDownHigh = 3,
    WaypointVaultType_ClimbUpHigh = 4,
    WaypointVaultType_ClimbOverHigh = 5,
    WaypointVaultType_JumpAcross = 6,
    WaypointVaultType_Count = 7,
}

pub const WAYPOINTVAULTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointVaultType",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(WAYPOINTVAULTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaypointVaultType {
    fn type_info() -> &'static TypeInfo {
        WAYPOINTVAULTTYPE_TYPE_INFO
    }
}


pub const WAYPOINTVAULTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointVaultType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("WaypointVaultType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntSettings {
    pub use_p_a: bool,
    pub use_h_i_k: bool,
    pub allow_variable_tick_length: bool,
    pub use_weapon_fov: bool,
    pub force_pose_update: bool,
    pub force_lod_distance: f32,
    pub use_camera_fov: bool,
    pub enable_p_a: bool,
    pub client_emulates_server: bool,
    pub update_enable: bool,
    pub enable_package_cache: bool,
    pub enable_debug_log_file: bool,
    pub enable_pose_jobs: bool,
    pub disable_a_i_lod_feature: bool,
    pub disable_model_animation_culling: bool,
    pub enable_jobs: bool,
    pub max_animatables_per_pose_job: i32,
    pub run_as_high_priority: bool,
    pub work_job_time_slice_ms: f32,
    pub update_lodding_enable: bool,
    pub check_giant_soldiers: f32,
    pub lean_signal_scale: f32,
    pub lean_signal_clamp: f32,
    pub detailed_collision_speed_limit: f32,
    pub enable_frostbite_ant_physics_world: bool,
    pub auto_cull_pixel_size: i32,
}

pub const ANTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntSettings",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UsePA",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, use_p_a),
            },
            FieldInfoData {
                name: "UseHIK",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, use_h_i_k),
            },
            FieldInfoData {
                name: "AllowVariableTickLength",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, allow_variable_tick_length),
            },
            FieldInfoData {
                name: "UseWeaponFov",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, use_weapon_fov),
            },
            FieldInfoData {
                name: "ForcePoseUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, force_pose_update),
            },
            FieldInfoData {
                name: "ForceLodDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, force_lod_distance),
            },
            FieldInfoData {
                name: "UseCameraFov",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, use_camera_fov),
            },
            FieldInfoData {
                name: "EnablePA",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, enable_p_a),
            },
            FieldInfoData {
                name: "ClientEmulatesServer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, client_emulates_server),
            },
            FieldInfoData {
                name: "UpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, update_enable),
            },
            FieldInfoData {
                name: "EnablePackageCache",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, enable_package_cache),
            },
            FieldInfoData {
                name: "EnableDebugLogFile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, enable_debug_log_file),
            },
            FieldInfoData {
                name: "EnablePoseJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, enable_pose_jobs),
            },
            FieldInfoData {
                name: "DisableAILodFeature",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, disable_a_i_lod_feature),
            },
            FieldInfoData {
                name: "DisableModelAnimationCulling",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, disable_model_animation_culling),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, enable_jobs),
            },
            FieldInfoData {
                name: "MaxAnimatablesPerPoseJob",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, max_animatables_per_pose_job),
            },
            FieldInfoData {
                name: "RunAsHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, run_as_high_priority),
            },
            FieldInfoData {
                name: "WorkJobTimeSliceMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, work_job_time_slice_ms),
            },
            FieldInfoData {
                name: "UpdateLoddingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, update_lodding_enable),
            },
            FieldInfoData {
                name: "CheckGiantSoldiers",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, check_giant_soldiers),
            },
            FieldInfoData {
                name: "LeanSignalScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, lean_signal_scale),
            },
            FieldInfoData {
                name: "LeanSignalClamp",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, lean_signal_clamp),
            },
            FieldInfoData {
                name: "DetailedCollisionSpeedLimit",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, detailed_collision_speed_limit),
            },
            FieldInfoData {
                name: "EnableFrostbiteAntPhysicsWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, enable_frostbite_ant_physics_world),
            },
            FieldInfoData {
                name: "AutoCullPixelSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AntSettings, auto_cull_pixel_size),
            },
        ],
    }),
    array_type: Some(ANTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntSettings {
    fn type_info() -> &'static TypeInfo {
        ANTSETTINGS_TYPE_INFO
    }
}


pub const ANTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntRef {
    pub asset_guid: super::core::Guid,
    pub project_id: i32,
}

pub const ANTREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRef",
    flags: MemberInfoFlags::new(32841),
    module: "Ant",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AssetGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(AntRef, asset_guid),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AntRef, project_id),
            },
        ],
    }),
    array_type: Some(ANTREF_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AntRef {
    fn type_info() -> &'static TypeInfo {
        ANTREF_TYPE_INFO
    }
}


pub const ANTREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntRef-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AntRefType {
    #[default]
    AntRefType_None = 0,
    AntRefType_Chunk = 1,
    AntRefType_3PChunk = 2,
    AntRefType_Lock = 3,
    AntRefType_ExcludeFromBundling = 4,
}

pub const ANTREFTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefType",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(ANTREFTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntRefType {
    fn type_info() -> &'static TypeInfo {
        ANTREFTYPE_TYPE_INFO
    }
}


pub const ANTREFTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntRefType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GameStateValueTypeEnum {
    #[default]
    GameStateValueTypeEnum_Bool = 0,
    GameStateValueTypeEnum_Integer = 1,
    GameStateValueTypeEnum_Float = 2,
    GameStateValueTypeEnum_Vector2 = 3,
    GameStateValueTypeEnum_Vector3 = 4,
    GameStateValueTypeEnum_Vector4 = 5,
    GameStateValueTypeEnum_Quaternion = 6,
    GameStateValueTypeEnum_Matrix = 7,
    GameStateValueTypeEnum_FaceposeLibrary = 8,
    GameStateValueTypeEnum_RandomValue = 9,
    GameStateValueTypeEnum_Count = 10,
}

pub const GAMESTATEVALUETYPEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateValueTypeEnum",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(GAMESTATEVALUETYPEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GameStateValueTypeEnum {
    fn type_info() -> &'static TypeInfo {
        GAMESTATEVALUETYPEENUM_TYPE_INFO
    }
}


pub const GAMESTATEVALUETYPEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateValueTypeEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("GameStateValueTypeEnum-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntAnimationSetAsset {
    pub skeleton_asset: super::entity::SkeletonAsset,
    pub actor_asset_guid: super::core::Guid,
    pub clip_asset_guids: Vec<super::core::Guid>,
    pub looping_clip_asset_guids: Vec<super::core::Guid>,
    pub scene_op_matrix_asset_guid: super::core::Guid,
    pub use_traj2_ref: bool,
    pub allow_animation_culling: bool,
    pub asset_bank_resource: super::core::ResourceRef,
}

pub const ANTANIMATIONSETASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationSetAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SkeletonAsset",
                flags: MemberInfoFlags::new(0),
                field_type: SKELETONASSET_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, skeleton_asset),
            },
            FieldInfoData {
                name: "ActorAssetGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, actor_asset_guid),
            },
            FieldInfoData {
                name: "ClipAssetGuids",
                flags: MemberInfoFlags::new(144),
                field_type: GUID_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, clip_asset_guids),
            },
            FieldInfoData {
                name: "LoopingClipAssetGuids",
                flags: MemberInfoFlags::new(144),
                field_type: GUID_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, looping_clip_asset_guids),
            },
            FieldInfoData {
                name: "SceneOpMatrixAssetGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, scene_op_matrix_asset_guid),
            },
            FieldInfoData {
                name: "UseTraj2Ref",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, use_traj2_ref),
            },
            FieldInfoData {
                name: "AllowAnimationCulling",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, allow_animation_culling),
            },
            FieldInfoData {
                name: "AssetBankResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(AntAnimationSetAsset, asset_bank_resource),
            },
        ],
    }),
    array_type: Some(ANTANIMATIONSETASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimationSetAsset {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATIONSETASSET_TYPE_INFO
    }
}


pub const ANTANIMATIONSETASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationSetAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntAnimationSetAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AntAnimatableData {
    pub actor: AntRef,
    pub scene_op_matrix: AntRef,
    pub collision_world: AntRef,
    pub right_hand_effector_disable_override: AntRef,
    pub left_hand_effector_disable_override: AntRef,
    pub master_skeleton_asset: super::entity::MasterSkeletonAsset,
}

pub const ANTANIMATABLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableData",
    flags: MemberInfoFlags::new(73),
    module: "Ant",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Actor",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableData, actor),
            },
            FieldInfoData {
                name: "SceneOpMatrix",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableData, scene_op_matrix),
            },
            FieldInfoData {
                name: "CollisionWorld",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableData, collision_world),
            },
            FieldInfoData {
                name: "RightHandEffectorDisableOverride",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableData, right_hand_effector_disable_override),
            },
            FieldInfoData {
                name: "LeftHandEffectorDisableOverride",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableData, left_hand_effector_disable_override),
            },
            FieldInfoData {
                name: "MasterSkeletonAsset",
                flags: MemberInfoFlags::new(0),
                field_type: MASTERSKELETONASSET_TYPE_INFO,
                rust_offset: offset_of!(AntAnimatableData, master_skeleton_asset),
            },
        ],
    }),
    array_type: Some(ANTANIMATABLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimatableData {
    fn type_info() -> &'static TypeInfo {
        ANTANIMATABLEDATA_TYPE_INFO
    }
}


pub const ANTANIMATABLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntAnimatableData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntProjectAsset {
    pub ant_native_project_name: String,
    pub scene_op: AntRef,
    pub project_id: i32,
}

pub const ANTPROJECTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntProjectAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AntNativeProjectName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AntProjectAsset, ant_native_project_name),
            },
            FieldInfoData {
                name: "SceneOp",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntProjectAsset, scene_op),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AntProjectAsset, project_id),
            },
        ],
    }),
    array_type: Some(ANTPROJECTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntProjectAsset {
    fn type_info() -> &'static TypeInfo {
        ANTPROJECTASSET_TYPE_INFO
    }
}


pub const ANTPROJECTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntProjectAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntProjectAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntStateHierarchyNode {
    pub state_asset: AntStateAsset,
    pub child_count: u16,
    pub first_child_index: u16,
}

pub const ANTSTATEHIERARCHYNODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateHierarchyNode",
    flags: MemberInfoFlags::new(73),
    module: "Ant",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StateAsset",
                flags: MemberInfoFlags::new(0),
                field_type: ANTSTATEASSET_TYPE_INFO,
                rust_offset: offset_of!(AntStateHierarchyNode, state_asset),
            },
            FieldInfoData {
                name: "ChildCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(AntStateHierarchyNode, child_count),
            },
            FieldInfoData {
                name: "FirstChildIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(AntStateHierarchyNode, first_child_index),
            },
        ],
    }),
    array_type: Some(ANTSTATEHIERARCHYNODE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntStateHierarchyNode {
    fn type_info() -> &'static TypeInfo {
        ANTSTATEHIERARCHYNODE_TYPE_INFO
    }
}


pub const ANTSTATEHIERARCHYNODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateHierarchyNode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntStateHierarchyNode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntCbaAsset {
}

pub const ANTCBAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntCbaAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTCBAASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntCbaAsset {
    fn type_info() -> &'static TypeInfo {
        ANTCBAASSET_TYPE_INFO
    }
}


pub const ANTCBAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntCbaAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntCbaAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntStateAsset {
    pub streaming_guid: super::core::Guid,
    pub chunk_size: i32,
}

pub const ANTSTATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StreamingGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(AntStateAsset, streaming_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AntStateAsset, chunk_size),
            },
        ],
    }),
    array_type: Some(ANTSTATEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntStateAsset {
    fn type_info() -> &'static TypeInfo {
        ANTSTATEASSET_TYPE_INFO
    }
}


pub const ANTSTATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntStateAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumGuidIndexPair {
    pub enum_guid: super::core::Guid,
    pub index: i32,
}

pub const ENUMGUIDINDEXPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumGuidIndexPair",
    flags: MemberInfoFlags::new(36937),
    module: "Ant",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EnumGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(EnumGuidIndexPair, enum_guid),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EnumGuidIndexPair, index),
            },
        ],
    }),
    array_type: Some(ENUMGUIDINDEXPAIR_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EnumGuidIndexPair {
    fn type_info() -> &'static TypeInfo {
        ENUMGUIDINDEXPAIR_TYPE_INFO
    }
}


pub const ENUMGUIDINDEXPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumGuidIndexPair-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("EnumGuidIndexPair-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntScenario {
}

pub const ANTSCENARIO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntScenario",
    flags: MemberInfoFlags::new(36937),
    module: "Ant",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(ANTSCENARIO_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntScenario {
    fn type_info() -> &'static TypeInfo {
        ANTSCENARIO_TYPE_INFO
    }
}


pub const ANTSCENARIO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntScenario-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntScenario-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntRefInfo {
    pub referencing_partition: super::core::Guid,
    pub ant_ref: AntRef,
}

pub const ANTREFINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefInfo",
    flags: MemberInfoFlags::new(32841),
    module: "Ant",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ReferencingPartition",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(AntRefInfo, referencing_partition),
            },
            FieldInfoData {
                name: "AntRef",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AntRefInfo, ant_ref),
            },
        ],
    }),
    array_type: Some(ANTREFINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AntRefInfo {
    fn type_info() -> &'static TypeInfo {
        ANTREFINFO_TYPE_INFO
    }
}


pub const ANTREFINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntRefInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AntPackageHelper {
}

pub const ANTPACKAGEHELPER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackageHelper",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTPACKAGEHELPER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntPackageHelper {
    fn type_info() -> &'static TypeInfo {
        ANTPACKAGEHELPER_TYPE_INFO
    }
}


pub const ANTPACKAGEHELPER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackageHelper-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntPackageHelper-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AntPackagingType {
    #[default]
    AntPackagingType_Chunk = 0,
    AntPackagingType_Bundle = 1,
    AntPackagingType_AnimationSet = 2,
    AntPackagingType_Static = 3,
    AntPackagingType_SharedLevel = 4,
    AntPackagingType_SharedBundle = 5,
    AntPackagingType_SharedGame = 6,
    AntPackagingType_LiveEdit = 7,
    AntPackagingType_ForceResolve = 8,
}

pub const ANTPACKAGINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackagingType",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(ANTPACKAGINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntPackagingType {
    fn type_info() -> &'static TypeInfo {
        ANTPACKAGINGTYPE_TYPE_INFO
    }
}


pub const ANTPACKAGINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackagingType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntPackagingType-Array"),
    array_type: None,
    alignment: 8,
};


