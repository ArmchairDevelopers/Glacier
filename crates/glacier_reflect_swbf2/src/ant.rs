use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct AssetBank {
}

pub trait AssetBankTrait: TypeObject {
}

impl AssetBankTrait for AssetBank {
}

pub static ASSETBANK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetBank",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AssetBank as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ASSETBANK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AssetBank {
    fn type_info(&self) -> &'static TypeInfo {
        ASSETBANK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ASSETBANK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetBank-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AssetBank"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Animatable {
}

pub trait AnimatableTrait: TypeObject {
}

impl AnimatableTrait for Animatable {
}

pub static ANIMATABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Animatable",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Animatable as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANIMATABLE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Animatable {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATABLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANIMATABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Animatable-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("Animatable"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SceneOpMatrix {
}

pub trait SceneOpMatrixTrait: TypeObject {
}

impl SceneOpMatrixTrait for SceneOpMatrix {
}

pub static SCENEOPMATRIX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SceneOpMatrix",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SceneOpMatrix as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCENEOPMATRIX_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SceneOpMatrix {
    fn type_info(&self) -> &'static TypeInfo {
        SCENEOPMATRIX_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SCENEOPMATRIX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SceneOpMatrix-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("SceneOpMatrix"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static WAYPOINTVAULTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointVaultType",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(WAYPOINTVAULTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaypointVaultType {
    fn type_info(&self) -> &'static TypeInfo {
        WAYPOINTVAULTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WAYPOINTVAULTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointVaultType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("WaypointVaultType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntSettings {
    pub _glacier_base: super::core::SystemSettings,
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

pub trait AntSettingsTrait: super::core::SystemSettingsTrait {
    fn use_p_a(&self) -> &bool;
    fn use_h_i_k(&self) -> &bool;
    fn allow_variable_tick_length(&self) -> &bool;
    fn use_weapon_fov(&self) -> &bool;
    fn force_pose_update(&self) -> &bool;
    fn force_lod_distance(&self) -> &f32;
    fn use_camera_fov(&self) -> &bool;
    fn enable_p_a(&self) -> &bool;
    fn client_emulates_server(&self) -> &bool;
    fn update_enable(&self) -> &bool;
    fn enable_package_cache(&self) -> &bool;
    fn enable_debug_log_file(&self) -> &bool;
    fn enable_pose_jobs(&self) -> &bool;
    fn disable_a_i_lod_feature(&self) -> &bool;
    fn disable_model_animation_culling(&self) -> &bool;
    fn enable_jobs(&self) -> &bool;
    fn max_animatables_per_pose_job(&self) -> &i32;
    fn run_as_high_priority(&self) -> &bool;
    fn work_job_time_slice_ms(&self) -> &f32;
    fn update_lodding_enable(&self) -> &bool;
    fn check_giant_soldiers(&self) -> &f32;
    fn lean_signal_scale(&self) -> &f32;
    fn lean_signal_clamp(&self) -> &f32;
    fn detailed_collision_speed_limit(&self) -> &f32;
    fn enable_frostbite_ant_physics_world(&self) -> &bool;
    fn auto_cull_pixel_size(&self) -> &i32;
}

impl AntSettingsTrait for AntSettings {
    fn use_p_a(&self) -> &bool {
        &self.use_p_a
    }
    fn use_h_i_k(&self) -> &bool {
        &self.use_h_i_k
    }
    fn allow_variable_tick_length(&self) -> &bool {
        &self.allow_variable_tick_length
    }
    fn use_weapon_fov(&self) -> &bool {
        &self.use_weapon_fov
    }
    fn force_pose_update(&self) -> &bool {
        &self.force_pose_update
    }
    fn force_lod_distance(&self) -> &f32 {
        &self.force_lod_distance
    }
    fn use_camera_fov(&self) -> &bool {
        &self.use_camera_fov
    }
    fn enable_p_a(&self) -> &bool {
        &self.enable_p_a
    }
    fn client_emulates_server(&self) -> &bool {
        &self.client_emulates_server
    }
    fn update_enable(&self) -> &bool {
        &self.update_enable
    }
    fn enable_package_cache(&self) -> &bool {
        &self.enable_package_cache
    }
    fn enable_debug_log_file(&self) -> &bool {
        &self.enable_debug_log_file
    }
    fn enable_pose_jobs(&self) -> &bool {
        &self.enable_pose_jobs
    }
    fn disable_a_i_lod_feature(&self) -> &bool {
        &self.disable_a_i_lod_feature
    }
    fn disable_model_animation_culling(&self) -> &bool {
        &self.disable_model_animation_culling
    }
    fn enable_jobs(&self) -> &bool {
        &self.enable_jobs
    }
    fn max_animatables_per_pose_job(&self) -> &i32 {
        &self.max_animatables_per_pose_job
    }
    fn run_as_high_priority(&self) -> &bool {
        &self.run_as_high_priority
    }
    fn work_job_time_slice_ms(&self) -> &f32 {
        &self.work_job_time_slice_ms
    }
    fn update_lodding_enable(&self) -> &bool {
        &self.update_lodding_enable
    }
    fn check_giant_soldiers(&self) -> &f32 {
        &self.check_giant_soldiers
    }
    fn lean_signal_scale(&self) -> &f32 {
        &self.lean_signal_scale
    }
    fn lean_signal_clamp(&self) -> &f32 {
        &self.lean_signal_clamp
    }
    fn detailed_collision_speed_limit(&self) -> &f32 {
        &self.detailed_collision_speed_limit
    }
    fn enable_frostbite_ant_physics_world(&self) -> &bool {
        &self.enable_frostbite_ant_physics_world
    }
    fn auto_cull_pixel_size(&self) -> &i32 {
        &self.auto_cull_pixel_size
    }
}

impl super::core::SystemSettingsTrait for AntSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for AntSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntSettings",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UsePA",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, use_p_a),
            },
            FieldInfoData {
                name: "UseHIK",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, use_h_i_k),
            },
            FieldInfoData {
                name: "AllowVariableTickLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, allow_variable_tick_length),
            },
            FieldInfoData {
                name: "UseWeaponFov",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, use_weapon_fov),
            },
            FieldInfoData {
                name: "ForcePoseUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, force_pose_update),
            },
            FieldInfoData {
                name: "ForceLodDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntSettings, force_lod_distance),
            },
            FieldInfoData {
                name: "UseCameraFov",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, use_camera_fov),
            },
            FieldInfoData {
                name: "EnablePA",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, enable_p_a),
            },
            FieldInfoData {
                name: "ClientEmulatesServer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, client_emulates_server),
            },
            FieldInfoData {
                name: "UpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, update_enable),
            },
            FieldInfoData {
                name: "EnablePackageCache",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, enable_package_cache),
            },
            FieldInfoData {
                name: "EnableDebugLogFile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, enable_debug_log_file),
            },
            FieldInfoData {
                name: "EnablePoseJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, enable_pose_jobs),
            },
            FieldInfoData {
                name: "DisableAILodFeature",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, disable_a_i_lod_feature),
            },
            FieldInfoData {
                name: "DisableModelAnimationCulling",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, disable_model_animation_culling),
            },
            FieldInfoData {
                name: "EnableJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, enable_jobs),
            },
            FieldInfoData {
                name: "MaxAnimatablesPerPoseJob",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AntSettings, max_animatables_per_pose_job),
            },
            FieldInfoData {
                name: "RunAsHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, run_as_high_priority),
            },
            FieldInfoData {
                name: "WorkJobTimeSliceMs",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntSettings, work_job_time_slice_ms),
            },
            FieldInfoData {
                name: "UpdateLoddingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, update_lodding_enable),
            },
            FieldInfoData {
                name: "CheckGiantSoldiers",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntSettings, check_giant_soldiers),
            },
            FieldInfoData {
                name: "LeanSignalScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntSettings, lean_signal_scale),
            },
            FieldInfoData {
                name: "LeanSignalClamp",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntSettings, lean_signal_clamp),
            },
            FieldInfoData {
                name: "DetailedCollisionSpeedLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AntSettings, detailed_collision_speed_limit),
            },
            FieldInfoData {
                name: "EnableFrostbiteAntPhysicsWorld",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntSettings, enable_frostbite_ant_physics_world),
            },
            FieldInfoData {
                name: "AutoCullPixelSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AntSettings, auto_cull_pixel_size),
            },
        ],
    }),
    array_type: Some(ANTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ANTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntRef {
    pub asset_guid: glacier_util::guid::Guid,
    pub project_id: i32,
}

pub trait AntRefTrait: TypeObject {
    fn asset_guid(&self) -> &glacier_util::guid::Guid;
    fn project_id(&self) -> &i32;
}

impl AntRefTrait for AntRef {
    fn asset_guid(&self) -> &glacier_util::guid::Guid {
        &self.asset_guid
    }
    fn project_id(&self) -> &i32 {
        &self.project_id
    }
}

pub static ANTREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRef",
    flags: MemberInfoFlags::new(32841),
    module: "Ant",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntRef as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AssetGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(AntRef, asset_guid),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AntRef, project_id),
            },
        ],
    }),
    array_type: Some(ANTREF_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AntRef {
    fn type_info(&self) -> &'static TypeInfo {
        ANTREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AntRefType {
    #[default]
    AntRefType_None = 0,
    AntRefType_Chunk = 1,
    AntRefType_3PChunk = 2,
    AntRefType_Lock = 3,
    AntRefType_ExcludeFromBundling = 4,
}

pub static ANTREFTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefType",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(ANTREFTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntRefType {
    fn type_info(&self) -> &'static TypeInfo {
        ANTREFTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTREFTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntRefType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static GAMESTATEVALUETYPEENUM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateValueTypeEnum",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(GAMESTATEVALUETYPEENUM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GameStateValueTypeEnum {
    fn type_info(&self) -> &'static TypeInfo {
        GAMESTATEVALUETYPEENUM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GAMESTATEVALUETYPEENUM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameStateValueTypeEnum-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("GameStateValueTypeEnum"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntAnimationSetAsset {
    pub _glacier_base: super::core::Asset,
    pub skeleton_asset: Option<Arc<Mutex<dyn super::entity::SkeletonAssetTrait>>>,
    pub actor_asset_guid: glacier_util::guid::Guid,
    pub clip_asset_guids: Vec<glacier_util::guid::Guid>,
    pub looping_clip_asset_guids: Vec<glacier_util::guid::Guid>,
    pub scene_op_matrix_asset_guid: glacier_util::guid::Guid,
    pub use_traj2_ref: bool,
    pub allow_animation_culling: bool,
    pub asset_bank_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait AntAnimationSetAssetTrait: super::core::AssetTrait {
    fn skeleton_asset(&self) -> &Option<Arc<Mutex<dyn super::entity::SkeletonAssetTrait>>>;
    fn actor_asset_guid(&self) -> &glacier_util::guid::Guid;
    fn clip_asset_guids(&self) -> &Vec<glacier_util::guid::Guid>;
    fn looping_clip_asset_guids(&self) -> &Vec<glacier_util::guid::Guid>;
    fn scene_op_matrix_asset_guid(&self) -> &glacier_util::guid::Guid;
    fn use_traj2_ref(&self) -> &bool;
    fn allow_animation_culling(&self) -> &bool;
    fn asset_bank_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl AntAnimationSetAssetTrait for AntAnimationSetAsset {
    fn skeleton_asset(&self) -> &Option<Arc<Mutex<dyn super::entity::SkeletonAssetTrait>>> {
        &self.skeleton_asset
    }
    fn actor_asset_guid(&self) -> &glacier_util::guid::Guid {
        &self.actor_asset_guid
    }
    fn clip_asset_guids(&self) -> &Vec<glacier_util::guid::Guid> {
        &self.clip_asset_guids
    }
    fn looping_clip_asset_guids(&self) -> &Vec<glacier_util::guid::Guid> {
        &self.looping_clip_asset_guids
    }
    fn scene_op_matrix_asset_guid(&self) -> &glacier_util::guid::Guid {
        &self.scene_op_matrix_asset_guid
    }
    fn use_traj2_ref(&self) -> &bool {
        &self.use_traj2_ref
    }
    fn allow_animation_culling(&self) -> &bool {
        &self.allow_animation_culling
    }
    fn asset_bank_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.asset_bank_resource
    }
}

impl super::core::AssetTrait for AntAnimationSetAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for AntAnimationSetAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANTANIMATIONSETASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationSetAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntAnimationSetAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SkeletonAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "SkeletonAsset",
                rust_offset: offset_of!(AntAnimationSetAsset, skeleton_asset),
            },
            FieldInfoData {
                name: "ActorAssetGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(AntAnimationSetAsset, actor_asset_guid),
            },
            FieldInfoData {
                name: "ClipAssetGuids",
                flags: MemberInfoFlags::new(144),
                field_type: "Guid-Array",
                rust_offset: offset_of!(AntAnimationSetAsset, clip_asset_guids),
            },
            FieldInfoData {
                name: "LoopingClipAssetGuids",
                flags: MemberInfoFlags::new(144),
                field_type: "Guid-Array",
                rust_offset: offset_of!(AntAnimationSetAsset, looping_clip_asset_guids),
            },
            FieldInfoData {
                name: "SceneOpMatrixAssetGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(AntAnimationSetAsset, scene_op_matrix_asset_guid),
            },
            FieldInfoData {
                name: "UseTraj2Ref",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimationSetAsset, use_traj2_ref),
            },
            FieldInfoData {
                name: "AllowAnimationCulling",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AntAnimationSetAsset, allow_animation_culling),
            },
            FieldInfoData {
                name: "AssetBankResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(AntAnimationSetAsset, asset_bank_resource),
            },
        ],
    }),
    array_type: Some(ANTANIMATIONSETASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimationSetAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATIONSETASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTANIMATIONSETASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimationSetAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntAnimationSetAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntAnimatableData {
    pub actor: AntRef,
    pub scene_op_matrix: AntRef,
    pub collision_world: AntRef,
    pub right_hand_effector_disable_override: AntRef,
    pub left_hand_effector_disable_override: AntRef,
    pub master_skeleton_asset: Option<Arc<Mutex<dyn super::entity::MasterSkeletonAssetTrait>>>,
}

pub trait AntAnimatableDataTrait: TypeObject {
    fn actor(&self) -> &AntRef;
    fn scene_op_matrix(&self) -> &AntRef;
    fn collision_world(&self) -> &AntRef;
    fn right_hand_effector_disable_override(&self) -> &AntRef;
    fn left_hand_effector_disable_override(&self) -> &AntRef;
    fn master_skeleton_asset(&self) -> &Option<Arc<Mutex<dyn super::entity::MasterSkeletonAssetTrait>>>;
}

impl AntAnimatableDataTrait for AntAnimatableData {
    fn actor(&self) -> &AntRef {
        &self.actor
    }
    fn scene_op_matrix(&self) -> &AntRef {
        &self.scene_op_matrix
    }
    fn collision_world(&self) -> &AntRef {
        &self.collision_world
    }
    fn right_hand_effector_disable_override(&self) -> &AntRef {
        &self.right_hand_effector_disable_override
    }
    fn left_hand_effector_disable_override(&self) -> &AntRef {
        &self.left_hand_effector_disable_override
    }
    fn master_skeleton_asset(&self) -> &Option<Arc<Mutex<dyn super::entity::MasterSkeletonAssetTrait>>> {
        &self.master_skeleton_asset
    }
}

pub static ANTANIMATABLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableData",
    flags: MemberInfoFlags::new(73),
    module: "Ant",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntAnimatableData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Actor",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntAnimatableData, actor),
            },
            FieldInfoData {
                name: "SceneOpMatrix",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntAnimatableData, scene_op_matrix),
            },
            FieldInfoData {
                name: "CollisionWorld",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntAnimatableData, collision_world),
            },
            FieldInfoData {
                name: "RightHandEffectorDisableOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntAnimatableData, right_hand_effector_disable_override),
            },
            FieldInfoData {
                name: "LeftHandEffectorDisableOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntAnimatableData, left_hand_effector_disable_override),
            },
            FieldInfoData {
                name: "MasterSkeletonAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "MasterSkeletonAsset",
                rust_offset: offset_of!(AntAnimatableData, master_skeleton_asset),
            },
        ],
    }),
    array_type: Some(ANTANIMATABLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntAnimatableData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTANIMATABLEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTANIMATABLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntAnimatableData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntAnimatableData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntProjectAsset {
    pub _glacier_base: super::core::Asset,
    pub ant_native_project_name: String,
    pub scene_op: AntRef,
    pub project_id: i32,
}

pub trait AntProjectAssetTrait: super::core::AssetTrait {
    fn ant_native_project_name(&self) -> &String;
    fn scene_op(&self) -> &AntRef;
    fn project_id(&self) -> &i32;
}

impl AntProjectAssetTrait for AntProjectAsset {
    fn ant_native_project_name(&self) -> &String {
        &self.ant_native_project_name
    }
    fn scene_op(&self) -> &AntRef {
        &self.scene_op
    }
    fn project_id(&self) -> &i32 {
        &self.project_id
    }
}

impl super::core::AssetTrait for AntProjectAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for AntProjectAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANTPROJECTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntProjectAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntProjectAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AntNativeProjectName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AntProjectAsset, ant_native_project_name),
            },
            FieldInfoData {
                name: "SceneOp",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntProjectAsset, scene_op),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AntProjectAsset, project_id),
            },
        ],
    }),
    array_type: Some(ANTPROJECTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntProjectAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ANTPROJECTASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTPROJECTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntProjectAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntProjectAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntStateHierarchyNode {
    pub state_asset: Option<Arc<Mutex<dyn AntStateAssetTrait>>>,
    pub child_count: u16,
    pub first_child_index: u16,
}

pub trait AntStateHierarchyNodeTrait: TypeObject {
    fn state_asset(&self) -> &Option<Arc<Mutex<dyn AntStateAssetTrait>>>;
    fn child_count(&self) -> &u16;
    fn first_child_index(&self) -> &u16;
}

impl AntStateHierarchyNodeTrait for AntStateHierarchyNode {
    fn state_asset(&self) -> &Option<Arc<Mutex<dyn AntStateAssetTrait>>> {
        &self.state_asset
    }
    fn child_count(&self) -> &u16 {
        &self.child_count
    }
    fn first_child_index(&self) -> &u16 {
        &self.first_child_index
    }
}

pub static ANTSTATEHIERARCHYNODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateHierarchyNode",
    flags: MemberInfoFlags::new(73),
    module: "Ant",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntStateHierarchyNode as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StateAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "AntStateAsset",
                rust_offset: offset_of!(AntStateHierarchyNode, state_asset),
            },
            FieldInfoData {
                name: "ChildCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(AntStateHierarchyNode, child_count),
            },
            FieldInfoData {
                name: "FirstChildIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(AntStateHierarchyNode, first_child_index),
            },
        ],
    }),
    array_type: Some(ANTSTATEHIERARCHYNODE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntStateHierarchyNode {
    fn type_info(&self) -> &'static TypeInfo {
        ANTSTATEHIERARCHYNODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTSTATEHIERARCHYNODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateHierarchyNode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntStateHierarchyNode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntCbaAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait AntCbaAssetTrait: super::core::AssetTrait {
}

impl AntCbaAssetTrait for AntCbaAsset {
}

impl super::core::AssetTrait for AntCbaAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for AntCbaAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANTCBAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntCbaAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntCbaAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTCBAASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntCbaAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCBAASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTCBAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntCbaAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntCbaAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntStateAsset {
    pub _glacier_base: super::core::Asset,
    pub streaming_guid: glacier_util::guid::Guid,
    pub chunk_size: i32,
}

pub trait AntStateAssetTrait: super::core::AssetTrait {
    fn streaming_guid(&self) -> &glacier_util::guid::Guid;
    fn chunk_size(&self) -> &i32;
}

impl AntStateAssetTrait for AntStateAsset {
    fn streaming_guid(&self) -> &glacier_util::guid::Guid {
        &self.streaming_guid
    }
    fn chunk_size(&self) -> &i32 {
        &self.chunk_size
    }
}

impl super::core::AssetTrait for AntStateAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for AntStateAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANTSTATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateAsset",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntStateAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StreamingGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(AntStateAsset, streaming_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AntStateAsset, chunk_size),
            },
        ],
    }),
    array_type: Some(ANTSTATEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntStateAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ANTSTATEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTSTATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntStateAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntStateAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnumGuidIndexPair {
    pub enum_guid: glacier_util::guid::Guid,
    pub index: i32,
}

pub trait EnumGuidIndexPairTrait: TypeObject {
    fn enum_guid(&self) -> &glacier_util::guid::Guid;
    fn index(&self) -> &i32;
}

impl EnumGuidIndexPairTrait for EnumGuidIndexPair {
    fn enum_guid(&self) -> &glacier_util::guid::Guid {
        &self.enum_guid
    }
    fn index(&self) -> &i32 {
        &self.index
    }
}

pub static ENUMGUIDINDEXPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumGuidIndexPair",
    flags: MemberInfoFlags::new(36937),
    module: "Ant",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnumGuidIndexPair as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EnumGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(EnumGuidIndexPair, enum_guid),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnumGuidIndexPair, index),
            },
        ],
    }),
    array_type: Some(ENUMGUIDINDEXPAIR_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EnumGuidIndexPair {
    fn type_info(&self) -> &'static TypeInfo {
        ENUMGUIDINDEXPAIR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENUMGUIDINDEXPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumGuidIndexPair-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("EnumGuidIndexPair"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntScenario {
}

pub trait AntScenarioTrait: TypeObject {
}

impl AntScenarioTrait for AntScenario {
}

pub static ANTSCENARIO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntScenario",
    flags: MemberInfoFlags::new(36937),
    module: "Ant",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntScenario as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTSCENARIO_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntScenario {
    fn type_info(&self) -> &'static TypeInfo {
        ANTSCENARIO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTSCENARIO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntScenario-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntScenario"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntRefInfo {
    pub referencing_partition: glacier_util::guid::Guid,
    pub ant_ref: AntRef,
}

pub trait AntRefInfoTrait: TypeObject {
    fn referencing_partition(&self) -> &glacier_util::guid::Guid;
    fn ant_ref(&self) -> &AntRef;
}

impl AntRefInfoTrait for AntRefInfo {
    fn referencing_partition(&self) -> &glacier_util::guid::Guid {
        &self.referencing_partition
    }
    fn ant_ref(&self) -> &AntRef {
        &self.ant_ref
    }
}

pub static ANTREFINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefInfo",
    flags: MemberInfoFlags::new(32841),
    module: "Ant",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntRefInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReferencingPartition",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(AntRefInfo, referencing_partition),
            },
            FieldInfoData {
                name: "AntRef",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AntRefInfo, ant_ref),
            },
        ],
    }),
    array_type: Some(ANTREFINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AntRefInfo {
    fn type_info(&self) -> &'static TypeInfo {
        ANTREFINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTREFINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntRefInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntRefInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AntPackageHelper {
    pub _glacier_base: super::core::DataContainer,
}

pub trait AntPackageHelperTrait: super::core::DataContainerTrait {
}

impl AntPackageHelperTrait for AntPackageHelper {
}

impl super::core::DataContainerTrait for AntPackageHelper {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ANTPACKAGEHELPER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackageHelper",
    flags: MemberInfoFlags::new(101),
    module: "Ant",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AntPackageHelper as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTPACKAGEHELPER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AntPackageHelper {
    fn type_info(&self) -> &'static TypeInfo {
        ANTPACKAGEHELPER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTPACKAGEHELPER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackageHelper-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntPackageHelper"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static ANTPACKAGINGTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackagingType",
    flags: MemberInfoFlags::new(49429),
    module: "Ant",
    data: TypeInfoData::Enum,
    array_type: Some(ANTPACKAGINGTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntPackagingType {
    fn type_info(&self) -> &'static TypeInfo {
        ANTPACKAGINGTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTPACKAGINGTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntPackagingType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Ant",
    data: TypeInfoData::Array("AntPackagingType"),
    array_type: None,
    alignment: 8,
};


