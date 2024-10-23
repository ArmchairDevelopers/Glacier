use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothAsset {
    pub _glacier_base: super::cloth_base::ClothBaseAsset,
    pub runtime_rig_bone_count: u32,
}

pub trait ClothAssetTrait: super::cloth_base::ClothBaseAssetTrait {
    fn runtime_rig_bone_count(&self) -> &u32;
    fn runtime_rig_bone_count_mut(&mut self) -> &mut u32;
}

impl ClothAssetTrait for ClothAsset {
    fn runtime_rig_bone_count(&self) -> &u32 {
        &self.runtime_rig_bone_count
    }
    fn runtime_rig_bone_count_mut(&mut self) -> &mut u32 {
        &mut self.runtime_rig_bone_count
    }
}

impl super::cloth_base::ClothBaseAssetTrait for ClothAsset {
    fn cloth_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        self._glacier_base.cloth_asset_resource()
    }
    fn cloth_asset_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        self._glacier_base.cloth_asset_resource_mut()
    }
}

impl super::core::AssetTrait for ClothAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ClothAsset {
}

pub static CLOTHASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAsset",
    name_hash: 164268617,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::cloth_base::CLOTHBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(ClothAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothAsset as Default>::default())),
            create_boxed: || Box::new(<ClothAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RuntimeRigBoneCount",
                name_hash: 1592992064,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothAsset, runtime_rig_bone_count),
            },
        ],
    }),
    array_type: Some(CLOTHASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothAsset {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHASSET_TYPE_INFO
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


pub static CLOTHASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAsset-Array",
    name_hash: 918425725,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothSystemSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait ClothSystemSettingsTrait: super::core::DataContainerTrait {
    fn client_cloth_world_thread_count(&self) -> &u32;
    fn client_cloth_world_thread_count_mut(&mut self) -> &mut u32;
    fn cloth_system_quality_level(&self) -> &super::core::QualityLevel;
    fn cloth_system_quality_level_mut(&mut self) -> &mut super::core::QualityLevel;
    fn enable_render_dt_rustler(&self) -> &bool;
    fn enable_render_dt_rustler_mut(&mut self) -> &mut bool;
    fn render_dt_rustler_short_frame_probability(&self) -> &f32;
    fn render_dt_rustler_short_frame_probability_mut(&mut self) -> &mut f32;
    fn render_dt_rustler_short_frame_min_dt(&self) -> &f32;
    fn render_dt_rustler_short_frame_min_dt_mut(&mut self) -> &mut f32;
    fn render_dt_rustler_short_frame_max_dt(&self) -> &f32;
    fn render_dt_rustler_short_frame_max_dt_mut(&mut self) -> &mut f32;
    fn render_dt_rustler_long_frame_probability(&self) -> &f32;
    fn render_dt_rustler_long_frame_probability_mut(&mut self) -> &mut f32;
    fn render_dt_rustler_long_frame_min_dt(&self) -> &f32;
    fn render_dt_rustler_long_frame_min_dt_mut(&mut self) -> &mut f32;
    fn render_dt_rustler_long_frame_max_dt(&self) -> &f32;
    fn render_dt_rustler_long_frame_max_dt_mut(&mut self) -> &mut f32;
    fn enable_render_dt_smoother(&self) -> &bool;
    fn enable_render_dt_smoother_mut(&mut self) -> &mut bool;
    fn render_dt_smoother_window_size(&self) -> &u32;
    fn render_dt_smoother_window_size_mut(&mut self) -> &mut u32;
    fn render_dt_smoother_min_dt(&self) -> &f32;
    fn render_dt_smoother_min_dt_mut(&mut self) -> &mut f32;
    fn render_dt_smoother_max_dt(&self) -> &f32;
    fn render_dt_smoother_max_dt_mut(&mut self) -> &mut f32;
    fn sim_prevents_previous_dt0(&self) -> &bool;
    fn sim_prevents_previous_dt0_mut(&mut self) -> &mut bool;
}

impl ClothSystemSettingsTrait for ClothSystemSettings {
    fn client_cloth_world_thread_count(&self) -> &u32 {
        &self.client_cloth_world_thread_count
    }
    fn client_cloth_world_thread_count_mut(&mut self) -> &mut u32 {
        &mut self.client_cloth_world_thread_count
    }
    fn cloth_system_quality_level(&self) -> &super::core::QualityLevel {
        &self.cloth_system_quality_level
    }
    fn cloth_system_quality_level_mut(&mut self) -> &mut super::core::QualityLevel {
        &mut self.cloth_system_quality_level
    }
    fn enable_render_dt_rustler(&self) -> &bool {
        &self.enable_render_dt_rustler
    }
    fn enable_render_dt_rustler_mut(&mut self) -> &mut bool {
        &mut self.enable_render_dt_rustler
    }
    fn render_dt_rustler_short_frame_probability(&self) -> &f32 {
        &self.render_dt_rustler_short_frame_probability
    }
    fn render_dt_rustler_short_frame_probability_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_rustler_short_frame_probability
    }
    fn render_dt_rustler_short_frame_min_dt(&self) -> &f32 {
        &self.render_dt_rustler_short_frame_min_dt
    }
    fn render_dt_rustler_short_frame_min_dt_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_rustler_short_frame_min_dt
    }
    fn render_dt_rustler_short_frame_max_dt(&self) -> &f32 {
        &self.render_dt_rustler_short_frame_max_dt
    }
    fn render_dt_rustler_short_frame_max_dt_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_rustler_short_frame_max_dt
    }
    fn render_dt_rustler_long_frame_probability(&self) -> &f32 {
        &self.render_dt_rustler_long_frame_probability
    }
    fn render_dt_rustler_long_frame_probability_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_rustler_long_frame_probability
    }
    fn render_dt_rustler_long_frame_min_dt(&self) -> &f32 {
        &self.render_dt_rustler_long_frame_min_dt
    }
    fn render_dt_rustler_long_frame_min_dt_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_rustler_long_frame_min_dt
    }
    fn render_dt_rustler_long_frame_max_dt(&self) -> &f32 {
        &self.render_dt_rustler_long_frame_max_dt
    }
    fn render_dt_rustler_long_frame_max_dt_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_rustler_long_frame_max_dt
    }
    fn enable_render_dt_smoother(&self) -> &bool {
        &self.enable_render_dt_smoother
    }
    fn enable_render_dt_smoother_mut(&mut self) -> &mut bool {
        &mut self.enable_render_dt_smoother
    }
    fn render_dt_smoother_window_size(&self) -> &u32 {
        &self.render_dt_smoother_window_size
    }
    fn render_dt_smoother_window_size_mut(&mut self) -> &mut u32 {
        &mut self.render_dt_smoother_window_size
    }
    fn render_dt_smoother_min_dt(&self) -> &f32 {
        &self.render_dt_smoother_min_dt
    }
    fn render_dt_smoother_min_dt_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_smoother_min_dt
    }
    fn render_dt_smoother_max_dt(&self) -> &f32 {
        &self.render_dt_smoother_max_dt
    }
    fn render_dt_smoother_max_dt_mut(&mut self) -> &mut f32 {
        &mut self.render_dt_smoother_max_dt
    }
    fn sim_prevents_previous_dt0(&self) -> &bool {
        &self.sim_prevents_previous_dt0
    }
    fn sim_prevents_previous_dt0_mut(&mut self) -> &mut bool {
        &mut self.sim_prevents_previous_dt0
    }
}

impl super::core::DataContainerTrait for ClothSystemSettings {
}

pub static CLOTHSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothSystemSettings",
    name_hash: 2697085433,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ClothSystemSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothSystemSettings as Default>::default())),
            create_boxed: || Box::new(<ClothSystemSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClientClothWorldThreadCount",
                name_hash: 160073135,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothSystemSettings, client_cloth_world_thread_count),
            },
            FieldInfoData {
                name: "ClothSystemQualityLevel",
                name_hash: 688049767,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(ClothSystemSettings, cloth_system_quality_level),
            },
            FieldInfoData {
                name: "EnableRenderDtRustler",
                name_hash: 3407008645,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothSystemSettings, enable_render_dt_rustler),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameProbability",
                name_hash: 3320372422,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_probability),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameMinDt",
                name_hash: 510871825,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_min_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameMaxDt",
                name_hash: 510621199,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_max_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameProbability",
                name_hash: 2966633662,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_probability),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameMinDt",
                name_hash: 4206522281,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_min_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameMaxDt",
                name_hash: 4206785463,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_max_dt),
            },
            FieldInfoData {
                name: "EnableRenderDtSmoother",
                name_hash: 2089249419,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothSystemSettings, enable_render_dt_smoother),
            },
            FieldInfoData {
                name: "RenderDtSmootherWindowSize",
                name_hash: 420013955,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_window_size),
            },
            FieldInfoData {
                name: "RenderDtSmootherMinDt",
                name_hash: 1581346032,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_min_dt),
            },
            FieldInfoData {
                name: "RenderDtSmootherMaxDt",
                name_hash: 1581103854,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_max_dt),
            },
            FieldInfoData {
                name: "SimPreventsPreviousDt0",
                name_hash: 1744225982,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothSystemSettings, sim_prevents_previous_dt0),
            },
        ],
    }),
    array_type: Some(CLOTHSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothSystemSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHSYSTEMSETTINGS_TYPE_INFO
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


pub static CLOTHSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothSystemSettings-Array",
    name_hash: 1456750797,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub instances: Vec<Option<LockedTypeObject /* ClothAssetInstance */>>,
    pub object_variations: Option<LockedTypeObject /* super::entity::ObjectVariationCollection */>,
}

pub trait ClothComponentDataTrait: super::entity::ComponentDataTrait {
    fn instances(&self) -> &Vec<Option<LockedTypeObject /* ClothAssetInstance */>>;
    fn instances_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ClothAssetInstance */>>;
    fn object_variations(&self) -> &Option<LockedTypeObject /* super::entity::ObjectVariationCollection */>;
    fn object_variations_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectVariationCollection */>;
}

impl ClothComponentDataTrait for ClothComponentData {
    fn instances(&self) -> &Vec<Option<LockedTypeObject /* ClothAssetInstance */>> {
        &self.instances
    }
    fn instances_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ClothAssetInstance */>> {
        &mut self.instances
    }
    fn object_variations(&self) -> &Option<LockedTypeObject /* super::entity::ObjectVariationCollection */> {
        &self.object_variations
    }
    fn object_variations_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectVariationCollection */> {
        &mut self.object_variations
    }
}

impl super::entity::ComponentDataTrait for ClothComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for ClothComponentData {
}

impl super::core::DataBusPeerTrait for ClothComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ClothComponentData {
}

impl super::core::DataContainerTrait for ClothComponentData {
}

pub static CLOTHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponentData",
    name_hash: 766137030,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ClothComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothComponentData as Default>::default())),
            create_boxed: || Box::new(<ClothComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Instances",
                name_hash: 3890087583,
                flags: MemberInfoFlags::new(144),
                field_type: "ClothAssetInstance-Array",
                rust_offset: offset_of!(ClothComponentData, instances),
            },
            FieldInfoData {
                name: "ObjectVariations",
                name_hash: 3638989970,
                flags: MemberInfoFlags::new(0),
                field_type: "ObjectVariationCollection",
                rust_offset: offset_of!(ClothComponentData, object_variations),
            },
        ],
    }),
    array_type: Some(CLOTHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHCOMPONENTDATA_TYPE_INFO
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


pub static CLOTHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponentData-Array",
    name_hash: 1361588338,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothAssetInstance {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub cloth: Option<LockedTypeObject /* ClothObjectBlueprint */>,
}

pub trait ClothAssetInstanceTrait: super::core::DataContainerPolicyAssetTrait {
    fn cloth(&self) -> &Option<LockedTypeObject /* ClothObjectBlueprint */>;
    fn cloth_mut(&mut self) -> &mut Option<LockedTypeObject /* ClothObjectBlueprint */>;
}

impl ClothAssetInstanceTrait for ClothAssetInstance {
    fn cloth(&self) -> &Option<LockedTypeObject /* ClothObjectBlueprint */> {
        &self.cloth
    }
    fn cloth_mut(&mut self) -> &mut Option<LockedTypeObject /* ClothObjectBlueprint */> {
        &mut self.cloth
    }
}

impl super::core::DataContainerPolicyAssetTrait for ClothAssetInstance {
}

impl super::core::AssetTrait for ClothAssetInstance {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ClothAssetInstance {
}

pub static CLOTHASSETINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAssetInstance",
    name_hash: 4276113568,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        super_class_offset: offset_of!(ClothAssetInstance, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothAssetInstance as Default>::default())),
            create_boxed: || Box::new(<ClothAssetInstance as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Cloth",
                name_hash: 212633593,
                flags: MemberInfoFlags::new(0),
                field_type: "ClothObjectBlueprint",
                rust_offset: offset_of!(ClothAssetInstance, cloth),
            },
        ],
    }),
    array_type: Some(CLOTHASSETINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothAssetInstance {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHASSETINSTANCE_TYPE_INFO
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


pub static CLOTHASSETINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothAssetInstance-Array",
    name_hash: 3459823124,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothAssetInstance"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothCollisionComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub geometries: Vec<Option<LockedTypeObject /* super::cloth_base::ClothCollisionGeometry */>>,
    pub source_part_range: Vec<u32>,
}

pub trait ClothCollisionComponentDataTrait: super::entity::ComponentDataTrait {
    fn geometries(&self) -> &Vec<Option<LockedTypeObject /* super::cloth_base::ClothCollisionGeometry */>>;
    fn geometries_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::cloth_base::ClothCollisionGeometry */>>;
    fn source_part_range(&self) -> &Vec<u32>;
    fn source_part_range_mut(&mut self) -> &mut Vec<u32>;
}

impl ClothCollisionComponentDataTrait for ClothCollisionComponentData {
    fn geometries(&self) -> &Vec<Option<LockedTypeObject /* super::cloth_base::ClothCollisionGeometry */>> {
        &self.geometries
    }
    fn geometries_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::cloth_base::ClothCollisionGeometry */>> {
        &mut self.geometries
    }
    fn source_part_range(&self) -> &Vec<u32> {
        &self.source_part_range
    }
    fn source_part_range_mut(&mut self) -> &mut Vec<u32> {
        &mut self.source_part_range
    }
}

impl super::entity::ComponentDataTrait for ClothCollisionComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for ClothCollisionComponentData {
}

impl super::core::DataBusPeerTrait for ClothCollisionComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ClothCollisionComponentData {
}

impl super::core::DataContainerTrait for ClothCollisionComponentData {
}

pub static CLOTHCOLLISIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponentData",
    name_hash: 217975000,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ClothCollisionComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothCollisionComponentData as Default>::default())),
            create_boxed: || Box::new(<ClothCollisionComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Geometries",
                name_hash: 2646101561,
                flags: MemberInfoFlags::new(144),
                field_type: "ClothCollisionGeometry-Array",
                rust_offset: offset_of!(ClothCollisionComponentData, geometries),
            },
            FieldInfoData {
                name: "SourcePartRange",
                name_hash: 2980309712,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(ClothCollisionComponentData, source_part_range),
            },
        ],
    }),
    array_type: Some(CLOTHCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothCollisionComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHCOLLISIONCOMPONENTDATA_TYPE_INFO
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


pub static CLOTHCOLLISIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponentData-Array",
    name_hash: 1619420012,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothCollisionComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothObjectBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
}

pub trait ClothObjectBlueprintTrait: super::entity::ObjectBlueprintTrait {
}

impl ClothObjectBlueprintTrait for ClothObjectBlueprint {
}

impl super::entity::ObjectBlueprintTrait for ClothObjectBlueprint {
    fn object(&self) -> &Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object()
    }
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object_mut()
    }
}

impl super::entity::BlueprintTrait for ClothObjectBlueprint {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for ClothObjectBlueprint {
    fn event_connections(&self) -> &Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for ClothObjectBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.flags_mut()
    }
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface_mut()
    }
}

impl super::core::AssetTrait for ClothObjectBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ClothObjectBlueprint {
}

pub static CLOTHOBJECTBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectBlueprint",
    name_hash: 3031485667,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        super_class_offset: offset_of!(ClothObjectBlueprint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothObjectBlueprint as Default>::default())),
            create_boxed: || Box::new(<ClothObjectBlueprint as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLOTHOBJECTBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothObjectBlueprint {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHOBJECTBLUEPRINT_TYPE_INFO
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


pub static CLOTHOBJECTBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectBlueprint-Array",
    name_hash: 2346262743,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothObjectBlueprint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothEntityData {
    pub _glacier_base: super::entity::ComponentEntityData,
    pub mesh: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
    pub mesh_name_hash: u32,
    pub cloth: Option<LockedTypeObject /* ClothAsset */>,
    pub character_lighting_enable: bool,
    pub enabled: bool,
    pub wind_scale: f32,
    pub wrap_on_g_p_u: bool,
    pub solve_on_render_side: bool,
    pub use_for_bounding_box_calculations: bool,
    pub cloth_wrapping: Option<LockedTypeObject /* super::cloth_base::ClothWrappingAsset */>,
    pub ground_plane_transform: super::core::LinearTransform,
    pub enable_ground_plane_collision: bool,
    pub local_ground_plane_transform: bool,
    pub activation_radius: super::core::QualityScalableFloat,
    pub smooth_blend_out_distance: super::core::QualityScalableFloat,
    pub collision_layer: super::physics::RigidBodyCollisionLayer,
}

pub trait ClothEntityDataTrait: super::entity::ComponentEntityDataTrait {
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn mesh_name_hash(&self) -> &u32;
    fn mesh_name_hash_mut(&mut self) -> &mut u32;
    fn cloth(&self) -> &Option<LockedTypeObject /* ClothAsset */>;
    fn cloth_mut(&mut self) -> &mut Option<LockedTypeObject /* ClothAsset */>;
    fn character_lighting_enable(&self) -> &bool;
    fn character_lighting_enable_mut(&mut self) -> &mut bool;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn wind_scale(&self) -> &f32;
    fn wind_scale_mut(&mut self) -> &mut f32;
    fn wrap_on_g_p_u(&self) -> &bool;
    fn wrap_on_g_p_u_mut(&mut self) -> &mut bool;
    fn solve_on_render_side(&self) -> &bool;
    fn solve_on_render_side_mut(&mut self) -> &mut bool;
    fn use_for_bounding_box_calculations(&self) -> &bool;
    fn use_for_bounding_box_calculations_mut(&mut self) -> &mut bool;
    fn cloth_wrapping(&self) -> &Option<LockedTypeObject /* super::cloth_base::ClothWrappingAsset */>;
    fn cloth_wrapping_mut(&mut self) -> &mut Option<LockedTypeObject /* super::cloth_base::ClothWrappingAsset */>;
    fn ground_plane_transform(&self) -> &super::core::LinearTransform;
    fn ground_plane_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn enable_ground_plane_collision(&self) -> &bool;
    fn enable_ground_plane_collision_mut(&mut self) -> &mut bool;
    fn local_ground_plane_transform(&self) -> &bool;
    fn local_ground_plane_transform_mut(&mut self) -> &mut bool;
    fn activation_radius(&self) -> &super::core::QualityScalableFloat;
    fn activation_radius_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn smooth_blend_out_distance(&self) -> &super::core::QualityScalableFloat;
    fn smooth_blend_out_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn collision_layer(&self) -> &super::physics::RigidBodyCollisionLayer;
    fn collision_layer_mut(&mut self) -> &mut super::physics::RigidBodyCollisionLayer;
}

impl ClothEntityDataTrait for ClothEntityData {
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.mesh
    }
    fn mesh_name_hash(&self) -> &u32 {
        &self.mesh_name_hash
    }
    fn mesh_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.mesh_name_hash
    }
    fn cloth(&self) -> &Option<LockedTypeObject /* ClothAsset */> {
        &self.cloth
    }
    fn cloth_mut(&mut self) -> &mut Option<LockedTypeObject /* ClothAsset */> {
        &mut self.cloth
    }
    fn character_lighting_enable(&self) -> &bool {
        &self.character_lighting_enable
    }
    fn character_lighting_enable_mut(&mut self) -> &mut bool {
        &mut self.character_lighting_enable
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn wind_scale(&self) -> &f32 {
        &self.wind_scale
    }
    fn wind_scale_mut(&mut self) -> &mut f32 {
        &mut self.wind_scale
    }
    fn wrap_on_g_p_u(&self) -> &bool {
        &self.wrap_on_g_p_u
    }
    fn wrap_on_g_p_u_mut(&mut self) -> &mut bool {
        &mut self.wrap_on_g_p_u
    }
    fn solve_on_render_side(&self) -> &bool {
        &self.solve_on_render_side
    }
    fn solve_on_render_side_mut(&mut self) -> &mut bool {
        &mut self.solve_on_render_side
    }
    fn use_for_bounding_box_calculations(&self) -> &bool {
        &self.use_for_bounding_box_calculations
    }
    fn use_for_bounding_box_calculations_mut(&mut self) -> &mut bool {
        &mut self.use_for_bounding_box_calculations
    }
    fn cloth_wrapping(&self) -> &Option<LockedTypeObject /* super::cloth_base::ClothWrappingAsset */> {
        &self.cloth_wrapping
    }
    fn cloth_wrapping_mut(&mut self) -> &mut Option<LockedTypeObject /* super::cloth_base::ClothWrappingAsset */> {
        &mut self.cloth_wrapping
    }
    fn ground_plane_transform(&self) -> &super::core::LinearTransform {
        &self.ground_plane_transform
    }
    fn ground_plane_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.ground_plane_transform
    }
    fn enable_ground_plane_collision(&self) -> &bool {
        &self.enable_ground_plane_collision
    }
    fn enable_ground_plane_collision_mut(&mut self) -> &mut bool {
        &mut self.enable_ground_plane_collision
    }
    fn local_ground_plane_transform(&self) -> &bool {
        &self.local_ground_plane_transform
    }
    fn local_ground_plane_transform_mut(&mut self) -> &mut bool {
        &mut self.local_ground_plane_transform
    }
    fn activation_radius(&self) -> &super::core::QualityScalableFloat {
        &self.activation_radius
    }
    fn activation_radius_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.activation_radius
    }
    fn smooth_blend_out_distance(&self) -> &super::core::QualityScalableFloat {
        &self.smooth_blend_out_distance
    }
    fn smooth_blend_out_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.smooth_blend_out_distance
    }
    fn collision_layer(&self) -> &super::physics::RigidBodyCollisionLayer {
        &self.collision_layer
    }
    fn collision_layer_mut(&mut self) -> &mut super::physics::RigidBodyCollisionLayer {
        &mut self.collision_layer
    }
}

impl super::entity::ComponentEntityDataTrait for ClothEntityData {
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

impl super::entity::SpatialEntityDataTrait for ClothEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for ClothEntityData {
}

impl super::entity::GameObjectDataTrait for ClothEntityData {
}

impl super::core::DataBusPeerTrait for ClothEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ClothEntityData {
}

impl super::core::DataContainerTrait for ClothEntityData {
}

pub static CLOTHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntityData",
    name_hash: 3259558290,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ClothEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothEntityData as Default>::default())),
            create_boxed: || Box::new(<ClothEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Mesh",
                name_hash: 2088783990,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(ClothEntityData, mesh),
            },
            FieldInfoData {
                name: "MeshNameHash",
                name_hash: 632059235,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothEntityData, mesh_name_hash),
            },
            FieldInfoData {
                name: "Cloth",
                name_hash: 212633593,
                flags: MemberInfoFlags::new(0),
                field_type: "ClothAsset",
                rust_offset: offset_of!(ClothEntityData, cloth),
            },
            FieldInfoData {
                name: "CharacterLightingEnable",
                name_hash: 4279193667,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, character_lighting_enable),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, enabled),
            },
            FieldInfoData {
                name: "WindScale",
                name_hash: 3183611401,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothEntityData, wind_scale),
            },
            FieldInfoData {
                name: "WrapOnGPU",
                name_hash: 1808879410,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, wrap_on_g_p_u),
            },
            FieldInfoData {
                name: "SolveOnRenderSide",
                name_hash: 2149893782,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, solve_on_render_side),
            },
            FieldInfoData {
                name: "UseForBoundingBoxCalculations",
                name_hash: 4205634432,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, use_for_bounding_box_calculations),
            },
            FieldInfoData {
                name: "ClothWrapping",
                name_hash: 97555101,
                flags: MemberInfoFlags::new(0),
                field_type: "ClothWrappingAsset",
                rust_offset: offset_of!(ClothEntityData, cloth_wrapping),
            },
            FieldInfoData {
                name: "GroundPlaneTransform",
                name_hash: 790189114,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ClothEntityData, ground_plane_transform),
            },
            FieldInfoData {
                name: "EnableGroundPlaneCollision",
                name_hash: 2421217577,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, enable_ground_plane_collision),
            },
            FieldInfoData {
                name: "LocalGroundPlaneTransform",
                name_hash: 3721893463,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, local_ground_plane_transform),
            },
            FieldInfoData {
                name: "ActivationRadius",
                name_hash: 1553047017,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(ClothEntityData, activation_radius),
            },
            FieldInfoData {
                name: "SmoothBlendOutDistance",
                name_hash: 2288342827,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(ClothEntityData, smooth_blend_out_distance),
            },
            FieldInfoData {
                name: "CollisionLayer",
                name_hash: 719540408,
                flags: MemberInfoFlags::new(0),
                field_type: "RigidBodyCollisionLayer",
                rust_offset: offset_of!(ClothEntityData, collision_layer),
            },
        ],
    }),
    array_type: Some(CLOTHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHENTITYDATA_TYPE_INFO
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


pub static CLOTHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntityData-Array",
    name_hash: 1687138470,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothObjectVariationExampleEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub variations: Vec<Option<LockedTypeObject /* super::entity::ObjectVariation */>>,
}

pub trait ClothObjectVariationExampleEntityDataTrait: super::entity::EntityDataTrait {
    fn variations(&self) -> &Vec<Option<LockedTypeObject /* super::entity::ObjectVariation */>>;
    fn variations_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::ObjectVariation */>>;
}

impl ClothObjectVariationExampleEntityDataTrait for ClothObjectVariationExampleEntityData {
    fn variations(&self) -> &Vec<Option<LockedTypeObject /* super::entity::ObjectVariation */>> {
        &self.variations
    }
    fn variations_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::ObjectVariation */>> {
        &mut self.variations
    }
}

impl super::entity::EntityDataTrait for ClothObjectVariationExampleEntityData {
}

impl super::entity::GameObjectDataTrait for ClothObjectVariationExampleEntityData {
}

impl super::core::DataBusPeerTrait for ClothObjectVariationExampleEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ClothObjectVariationExampleEntityData {
}

impl super::core::DataContainerTrait for ClothObjectVariationExampleEntityData {
}

pub static CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectVariationExampleEntityData",
    name_hash: 45611678,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ClothObjectVariationExampleEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothObjectVariationExampleEntityData as Default>::default())),
            create_boxed: || Box::new(<ClothObjectVariationExampleEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Variations",
                name_hash: 2728063271,
                flags: MemberInfoFlags::new(144),
                field_type: "ObjectVariation-Array",
                rust_offset: offset_of!(ClothObjectVariationExampleEntityData, variations),
            },
        ],
    }),
    array_type: Some(CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothObjectVariationExampleEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_TYPE_INFO
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


pub static CLOTHOBJECTVARIATIONEXAMPLEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothObjectVariationExampleEntityData-Array",
    name_hash: 4261841834,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothObjectVariationExampleEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothInstanceObserverEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub cloth_index_in_component: u32,
    pub force_processing_mode_callback: bool,
}

pub trait ClothInstanceObserverEntityDataTrait: super::entity::EntityDataTrait {
    fn cloth_index_in_component(&self) -> &u32;
    fn cloth_index_in_component_mut(&mut self) -> &mut u32;
    fn force_processing_mode_callback(&self) -> &bool;
    fn force_processing_mode_callback_mut(&mut self) -> &mut bool;
}

impl ClothInstanceObserverEntityDataTrait for ClothInstanceObserverEntityData {
    fn cloth_index_in_component(&self) -> &u32 {
        &self.cloth_index_in_component
    }
    fn cloth_index_in_component_mut(&mut self) -> &mut u32 {
        &mut self.cloth_index_in_component
    }
    fn force_processing_mode_callback(&self) -> &bool {
        &self.force_processing_mode_callback
    }
    fn force_processing_mode_callback_mut(&mut self) -> &mut bool {
        &mut self.force_processing_mode_callback
    }
}

impl super::entity::EntityDataTrait for ClothInstanceObserverEntityData {
}

impl super::entity::GameObjectDataTrait for ClothInstanceObserverEntityData {
}

impl super::core::DataBusPeerTrait for ClothInstanceObserverEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ClothInstanceObserverEntityData {
}

impl super::core::DataContainerTrait for ClothInstanceObserverEntityData {
}

pub static CLOTHINSTANCEOBSERVERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothInstanceObserverEntityData",
    name_hash: 1866586867,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ClothInstanceObserverEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothInstanceObserverEntityData as Default>::default())),
            create_boxed: || Box::new(<ClothInstanceObserverEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClothIndexInComponent",
                name_hash: 1977850447,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothInstanceObserverEntityData, cloth_index_in_component),
            },
            FieldInfoData {
                name: "ForceProcessingModeCallback",
                name_hash: 2222000025,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothInstanceObserverEntityData, force_processing_mode_callback),
            },
        ],
    }),
    array_type: Some(CLOTHINSTANCEOBSERVERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothInstanceObserverEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHINSTANCEOBSERVERENTITYDATA_TYPE_INFO
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


pub static CLOTHINSTANCEOBSERVERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothInstanceObserverEntityData-Array",
    name_hash: 2920249031,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothInstanceObserverEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothDebugRendererSettings {
    pub _glacier_base: super::core::DataContainer,
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

pub trait ClothDebugRendererSettingsTrait: super::core::DataContainerTrait {
    fn sim_mesh_line_color(&self) -> &super::core::Vec4;
    fn sim_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn sim_mesh_face_color(&self) -> &super::core::Vec4;
    fn sim_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn anchor_mesh_line_color(&self) -> &super::core::Vec4;
    fn anchor_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn anchor_mesh_face_color(&self) -> &super::core::Vec4;
    fn anchor_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn anchor_distance_mesh_line_color(&self) -> &super::core::Vec4;
    fn anchor_distance_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn anchor_distance_mesh_face_color(&self) -> &super::core::Vec4;
    fn anchor_distance_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn penetration_mesh_line_color(&self) -> &super::core::Vec4;
    fn penetration_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn penetration_mesh_face_color(&self) -> &super::core::Vec4;
    fn penetration_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn sphere_collider_line_color(&self) -> &super::core::Vec4;
    fn sphere_collider_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn sphere_collider_face_color(&self) -> &super::core::Vec4;
    fn sphere_collider_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn capsule_collider_line_color(&self) -> &super::core::Vec4;
    fn capsule_collider_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn capsule_collider_face_color(&self) -> &super::core::Vec4;
    fn capsule_collider_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn tapered_capsule_collider_line_color(&self) -> &super::core::Vec4;
    fn tapered_capsule_collider_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn tapered_capsule_collider_face_color(&self) -> &super::core::Vec4;
    fn tapered_capsule_collider_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn box_collider_line_color(&self) -> &super::core::Vec4;
    fn box_collider_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn box_collider_face_color(&self) -> &super::core::Vec4;
    fn box_collider_face_color_mut(&mut self) -> &mut super::core::Vec4;
    fn ground_plane_line_color(&self) -> &super::core::Vec4;
    fn ground_plane_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn disabled_ground_plane_line_color(&self) -> &super::core::Vec4;
    fn disabled_ground_plane_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn cloth_point_collider_line_color(&self) -> &super::core::Vec4;
    fn cloth_point_collider_line_color_mut(&mut self) -> &mut super::core::Vec4;
    fn activation_radius_color(&self) -> &super::core::Vec4;
    fn activation_radius_color_mut(&mut self) -> &mut super::core::Vec4;
}

impl ClothDebugRendererSettingsTrait for ClothDebugRendererSettings {
    fn sim_mesh_line_color(&self) -> &super::core::Vec4 {
        &self.sim_mesh_line_color
    }
    fn sim_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.sim_mesh_line_color
    }
    fn sim_mesh_face_color(&self) -> &super::core::Vec4 {
        &self.sim_mesh_face_color
    }
    fn sim_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.sim_mesh_face_color
    }
    fn anchor_mesh_line_color(&self) -> &super::core::Vec4 {
        &self.anchor_mesh_line_color
    }
    fn anchor_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.anchor_mesh_line_color
    }
    fn anchor_mesh_face_color(&self) -> &super::core::Vec4 {
        &self.anchor_mesh_face_color
    }
    fn anchor_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.anchor_mesh_face_color
    }
    fn anchor_distance_mesh_line_color(&self) -> &super::core::Vec4 {
        &self.anchor_distance_mesh_line_color
    }
    fn anchor_distance_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.anchor_distance_mesh_line_color
    }
    fn anchor_distance_mesh_face_color(&self) -> &super::core::Vec4 {
        &self.anchor_distance_mesh_face_color
    }
    fn anchor_distance_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.anchor_distance_mesh_face_color
    }
    fn penetration_mesh_line_color(&self) -> &super::core::Vec4 {
        &self.penetration_mesh_line_color
    }
    fn penetration_mesh_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.penetration_mesh_line_color
    }
    fn penetration_mesh_face_color(&self) -> &super::core::Vec4 {
        &self.penetration_mesh_face_color
    }
    fn penetration_mesh_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.penetration_mesh_face_color
    }
    fn sphere_collider_line_color(&self) -> &super::core::Vec4 {
        &self.sphere_collider_line_color
    }
    fn sphere_collider_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.sphere_collider_line_color
    }
    fn sphere_collider_face_color(&self) -> &super::core::Vec4 {
        &self.sphere_collider_face_color
    }
    fn sphere_collider_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.sphere_collider_face_color
    }
    fn capsule_collider_line_color(&self) -> &super::core::Vec4 {
        &self.capsule_collider_line_color
    }
    fn capsule_collider_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.capsule_collider_line_color
    }
    fn capsule_collider_face_color(&self) -> &super::core::Vec4 {
        &self.capsule_collider_face_color
    }
    fn capsule_collider_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.capsule_collider_face_color
    }
    fn tapered_capsule_collider_line_color(&self) -> &super::core::Vec4 {
        &self.tapered_capsule_collider_line_color
    }
    fn tapered_capsule_collider_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.tapered_capsule_collider_line_color
    }
    fn tapered_capsule_collider_face_color(&self) -> &super::core::Vec4 {
        &self.tapered_capsule_collider_face_color
    }
    fn tapered_capsule_collider_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.tapered_capsule_collider_face_color
    }
    fn box_collider_line_color(&self) -> &super::core::Vec4 {
        &self.box_collider_line_color
    }
    fn box_collider_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.box_collider_line_color
    }
    fn box_collider_face_color(&self) -> &super::core::Vec4 {
        &self.box_collider_face_color
    }
    fn box_collider_face_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.box_collider_face_color
    }
    fn ground_plane_line_color(&self) -> &super::core::Vec4 {
        &self.ground_plane_line_color
    }
    fn ground_plane_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.ground_plane_line_color
    }
    fn disabled_ground_plane_line_color(&self) -> &super::core::Vec4 {
        &self.disabled_ground_plane_line_color
    }
    fn disabled_ground_plane_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.disabled_ground_plane_line_color
    }
    fn cloth_point_collider_line_color(&self) -> &super::core::Vec4 {
        &self.cloth_point_collider_line_color
    }
    fn cloth_point_collider_line_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.cloth_point_collider_line_color
    }
    fn activation_radius_color(&self) -> &super::core::Vec4 {
        &self.activation_radius_color
    }
    fn activation_radius_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.activation_radius_color
    }
}

impl super::core::DataContainerTrait for ClothDebugRendererSettings {
}

pub static CLOTHDEBUGRENDERERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothDebugRendererSettings",
    name_hash: 121892272,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ClothDebugRendererSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothDebugRendererSettings as Default>::default())),
            create_boxed: || Box::new(<ClothDebugRendererSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SimMeshLineColor",
                name_hash: 1868103026,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sim_mesh_line_color),
            },
            FieldInfoData {
                name: "SimMeshFaceColor",
                name_hash: 2455340989,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sim_mesh_face_color),
            },
            FieldInfoData {
                name: "AnchorMeshLineColor",
                name_hash: 1839309692,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_mesh_line_color),
            },
            FieldInfoData {
                name: "AnchorMeshFaceColor",
                name_hash: 4098437299,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_mesh_face_color),
            },
            FieldInfoData {
                name: "AnchorDistanceMeshLineColor",
                name_hash: 1993753567,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_distance_mesh_line_color),
            },
            FieldInfoData {
                name: "AnchorDistanceMeshFaceColor",
                name_hash: 4006271632,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_distance_mesh_face_color),
            },
            FieldInfoData {
                name: "PenetrationMeshLineColor",
                name_hash: 4224200544,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, penetration_mesh_line_color),
            },
            FieldInfoData {
                name: "PenetrationMeshFaceColor",
                name_hash: 1908086447,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, penetration_mesh_face_color),
            },
            FieldInfoData {
                name: "SphereColliderLineColor",
                name_hash: 3495778809,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sphere_collider_line_color),
            },
            FieldInfoData {
                name: "SphereColliderFaceColor",
                name_hash: 3730354102,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sphere_collider_face_color),
            },
            FieldInfoData {
                name: "CapsuleColliderLineColor",
                name_hash: 3969656605,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, capsule_collider_line_color),
            },
            FieldInfoData {
                name: "CapsuleColliderFaceColor",
                name_hash: 201129938,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, capsule_collider_face_color),
            },
            FieldInfoData {
                name: "TaperedCapsuleColliderLineColor",
                name_hash: 1861172302,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, tapered_capsule_collider_line_color),
            },
            FieldInfoData {
                name: "TaperedCapsuleColliderFaceColor",
                name_hash: 1615006081,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, tapered_capsule_collider_face_color),
            },
            FieldInfoData {
                name: "BoxColliderLineColor",
                name_hash: 2041349557,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, box_collider_line_color),
            },
            FieldInfoData {
                name: "BoxColliderFaceColor",
                name_hash: 1162106490,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, box_collider_face_color),
            },
            FieldInfoData {
                name: "GroundPlaneLineColor",
                name_hash: 4024566533,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, ground_plane_line_color),
            },
            FieldInfoData {
                name: "DisabledGroundPlaneLineColor",
                name_hash: 1060902709,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, disabled_ground_plane_line_color),
            },
            FieldInfoData {
                name: "ClothPointColliderLineColor",
                name_hash: 3252270640,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, cloth_point_collider_line_color),
            },
            FieldInfoData {
                name: "ActivationRadiusColor",
                name_hash: 1030327380,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, activation_radius_color),
            },
        ],
    }),
    array_type: Some(CLOTHDEBUGRENDERERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothDebugRendererSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHDEBUGRENDERERSETTINGS_TYPE_INFO
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


pub static CLOTHDEBUGRENDERERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothDebugRendererSettings-Array",
    name_hash: 2612058884,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothDebugRendererSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothCollisionComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait ClothCollisionComponentTrait: super::entity::ComponentTrait {
}

impl ClothCollisionComponentTrait for ClothCollisionComponent {
}

impl super::entity::ComponentTrait for ClothCollisionComponent {
}

impl super::entity::EntityBusPeerTrait for ClothCollisionComponent {
}

pub static CLOTHCOLLISIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponent",
    name_hash: 3749951976,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClothCollisionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothCollisionComponent as Default>::default())),
            create_boxed: || Box::new(<ClothCollisionComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLOTHCOLLISIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClothCollisionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHCOLLISIONCOMPONENT_TYPE_INFO
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


pub static CLOTHCOLLISIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionComponent-Array",
    name_hash: 2018601948,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothCollisionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothEntity {
    pub _glacier_base: super::entity::ComponentEntity,
}

pub trait ClothEntityTrait: super::entity::ComponentEntityTrait {
}

impl ClothEntityTrait for ClothEntity {
}

impl super::entity::ComponentEntityTrait for ClothEntity {
}

impl super::entity::SpatialEntityTrait for ClothEntity {
}

impl super::entity::EntityTrait for ClothEntity {
}

impl super::entity::EntityBusPeerTrait for ClothEntity {
}

pub static CLOTHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntity",
    name_hash: 1278275234,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClothEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothEntity as Default>::default())),
            create_boxed: || Box::new(<ClothEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLOTHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClothEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHENTITY_TYPE_INFO
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


pub static CLOTHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothEntity-Array",
    name_hash: 3812642070,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothManager {
    pub _glacier_base: super::physics::IglooSubsystem,
}

pub trait ClothManagerTrait: super::physics::IglooSubsystemTrait {
}

impl ClothManagerTrait for ClothManager {
}

impl super::physics::IglooSubsystemTrait for ClothManager {
}

pub static CLOTHMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothManager",
    name_hash: 2318462442,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::IGLOOSUBSYSTEM_TYPE_INFO),
        super_class_offset: offset_of!(ClothManager, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothManager as Default>::default())),
            create_boxed: || Box::new(<ClothManager as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLOTHMANAGER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothManager {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHMANAGER_TYPE_INFO
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


pub static CLOTHMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothManager-Array",
    name_hash: 516020446,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EAClothMemoryInitializer {
    pub _glacier_base: super::physics::IglooModuleInitializer,
}

pub trait EAClothMemoryInitializerTrait: super::physics::IglooModuleInitializerTrait {
}

impl EAClothMemoryInitializerTrait for EAClothMemoryInitializer {
}

impl super::physics::IglooModuleInitializerTrait for EAClothMemoryInitializer {
}

pub static EACLOTHMEMORYINITIALIZER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothMemoryInitializer",
    name_hash: 3460584326,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::IGLOOMODULEINITIALIZER_TYPE_INFO),
        super_class_offset: offset_of!(EAClothMemoryInitializer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EAClothMemoryInitializer as Default>::default())),
            create_boxed: || Box::new(<EAClothMemoryInitializer as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EACLOTHMEMORYINITIALIZER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EAClothMemoryInitializer {
    fn type_info(&self) -> &'static TypeInfo {
        EACLOTHMEMORYINITIALIZER_TYPE_INFO
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


pub static EACLOTHMEMORYINITIALIZER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothMemoryInitializer-Array",
    name_hash: 798982322,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("EAClothMemoryInitializer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait ClothComponentTrait: super::entity::ComponentTrait {
}

impl ClothComponentTrait for ClothComponent {
}

impl super::entity::ComponentTrait for ClothComponent {
}

impl super::entity::EntityBusPeerTrait for ClothComponent {
}

pub static CLOTHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponent",
    name_hash: 3827618294,
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClothComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothComponent as Default>::default())),
            create_boxed: || Box::new(<ClothComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLOTHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClothComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHCOMPONENT_TYPE_INFO
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


pub static CLOTHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothComponent-Array",
    name_hash: 1868814018,
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothComponent"),
    array_type: None,
    alignment: 8,
};


