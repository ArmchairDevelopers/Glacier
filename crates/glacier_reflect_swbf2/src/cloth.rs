use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::cloth_base::CLOTHBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RuntimeRigBoneCount",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothSystemSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ClientClothWorldThreadCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothSystemSettings, client_cloth_world_thread_count),
            },
            FieldInfoData {
                name: "ClothSystemQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(ClothSystemSettings, cloth_system_quality_level),
            },
            FieldInfoData {
                name: "EnableRenderDtRustler",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothSystemSettings, enable_render_dt_rustler),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_probability),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameMinDt",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_min_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerShortFrameMaxDt",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_short_frame_max_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_probability),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameMinDt",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_min_dt),
            },
            FieldInfoData {
                name: "RenderDtRustlerLongFrameMaxDt",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_rustler_long_frame_max_dt),
            },
            FieldInfoData {
                name: "EnableRenderDtSmoother",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothSystemSettings, enable_render_dt_smoother),
            },
            FieldInfoData {
                name: "RenderDtSmootherWindowSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_window_size),
            },
            FieldInfoData {
                name: "RenderDtSmootherMinDt",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_min_dt),
            },
            FieldInfoData {
                name: "RenderDtSmootherMaxDt",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothSystemSettings, render_dt_smoother_max_dt),
            },
            FieldInfoData {
                name: "SimPreventsPreviousDt0",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub instances: Vec<Option<Arc<Mutex<dyn ClothAssetInstanceTrait>>>>,
    pub object_variations: Option<Arc<Mutex<dyn super::entity::ObjectVariationCollectionTrait>>>,
}

pub trait ClothComponentDataTrait: super::entity::ComponentDataTrait {
    fn instances(&self) -> &Vec<Option<Arc<Mutex<dyn ClothAssetInstanceTrait>>>>;
    fn instances_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ClothAssetInstanceTrait>>>>;
    fn object_variations(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationCollectionTrait>>>;
    fn object_variations_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::ObjectVariationCollectionTrait>>>;
}

impl ClothComponentDataTrait for ClothComponentData {
    fn instances(&self) -> &Vec<Option<Arc<Mutex<dyn ClothAssetInstanceTrait>>>> {
        &self.instances
    }
    fn instances_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ClothAssetInstanceTrait>>>> {
        &mut self.instances
    }
    fn object_variations(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationCollectionTrait>>> {
        &self.object_variations
    }
    fn object_variations_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::ObjectVariationCollectionTrait>>> {
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
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Instances",
                flags: MemberInfoFlags::new(144),
                field_type: "ClothAssetInstance-Array",
                rust_offset: offset_of!(ClothComponentData, instances),
            },
            FieldInfoData {
                name: "ObjectVariations",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothAssetInstance {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub cloth: Option<Arc<Mutex<dyn ClothObjectBlueprintTrait>>>,
}

pub trait ClothAssetInstanceTrait: super::core::DataContainerPolicyAssetTrait {
    fn cloth(&self) -> &Option<Arc<Mutex<dyn ClothObjectBlueprintTrait>>>;
    fn cloth_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ClothObjectBlueprintTrait>>>;
}

impl ClothAssetInstanceTrait for ClothAssetInstance {
    fn cloth(&self) -> &Option<Arc<Mutex<dyn ClothObjectBlueprintTrait>>> {
        &self.cloth
    }
    fn cloth_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ClothObjectBlueprintTrait>>> {
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothAssetInstance as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Cloth",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothAssetInstance"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothCollisionComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub geometries: Vec<Option<Arc<Mutex<dyn super::cloth_base::ClothCollisionGeometryTrait>>>>,
    pub source_part_range: Vec<u32>,
}

pub trait ClothCollisionComponentDataTrait: super::entity::ComponentDataTrait {
    fn geometries(&self) -> &Vec<Option<Arc<Mutex<dyn super::cloth_base::ClothCollisionGeometryTrait>>>>;
    fn geometries_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::cloth_base::ClothCollisionGeometryTrait>>>>;
    fn source_part_range(&self) -> &Vec<u32>;
    fn source_part_range_mut(&mut self) -> &mut Vec<u32>;
}

impl ClothCollisionComponentDataTrait for ClothCollisionComponentData {
    fn geometries(&self) -> &Vec<Option<Arc<Mutex<dyn super::cloth_base::ClothCollisionGeometryTrait>>>> {
        &self.geometries
    }
    fn geometries_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::cloth_base::ClothCollisionGeometryTrait>>>> {
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
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothCollisionComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Geometries",
                flags: MemberInfoFlags::new(144),
                field_type: "ClothCollisionGeometry-Array",
                rust_offset: offset_of!(ClothCollisionComponentData, geometries),
            },
            FieldInfoData {
                name: "SourcePartRange",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothCollisionComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothObjectBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
}

pub trait ClothObjectBlueprintTrait: super::entity::ObjectBlueprintTrait {
}

impl ClothObjectBlueprintTrait for ClothObjectBlueprint {
}

impl super::entity::ObjectBlueprintTrait for ClothObjectBlueprint {
    fn object(&self) -> &Option<Arc<Mutex<dyn super::entity::EntityDataTrait>>> {
        self._glacier_base.object()
    }
    fn object_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::EntityDataTrait>>> {
        self._glacier_base.object_mut()
    }
}

impl super::entity::BlueprintTrait for ClothObjectBlueprint {
    fn objects(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<Arc<Mutex<dyn super::schematics::SchematicsBaseAssetTrait>>> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::schematics::SchematicsBaseAssetTrait>>> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for ClothObjectBlueprint {
    fn event_connections(&self) -> &Vec<super::entity::EventConnection> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<super::entity::EventConnection> {
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
    fn property_connections(&self) -> &Vec<super::core::PropertyConnection> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<super::core::PropertyConnection> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<super::core::LinkConnection> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<super::core::LinkConnection> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<Arc<Mutex<dyn super::core::DynamicDataContainerTrait>>> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::DynamicDataContainerTrait>>> {
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothObjectBlueprint as Default>::default())),
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothObjectBlueprint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothEntityData {
    pub _glacier_base: super::entity::ComponentEntityData,
    pub mesh: Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>,
    pub mesh_name_hash: u32,
    pub cloth: Option<Arc<Mutex<dyn ClothAssetTrait>>>,
    pub character_lighting_enable: bool,
    pub enabled: bool,
    pub wind_scale: f32,
    pub wrap_on_g_p_u: bool,
    pub solve_on_render_side: bool,
    pub use_for_bounding_box_calculations: bool,
    pub cloth_wrapping: Option<Arc<Mutex<dyn super::cloth_base::ClothWrappingAssetTrait>>>,
    pub ground_plane_transform: super::core::LinearTransform,
    pub enable_ground_plane_collision: bool,
    pub local_ground_plane_transform: bool,
    pub activation_radius: super::core::QualityScalableFloat,
    pub smooth_blend_out_distance: super::core::QualityScalableFloat,
    pub collision_layer: super::physics::RigidBodyCollisionLayer,
}

pub trait ClothEntityDataTrait: super::entity::ComponentEntityDataTrait {
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
    fn mesh_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
    fn mesh_name_hash(&self) -> &u32;
    fn mesh_name_hash_mut(&mut self) -> &mut u32;
    fn cloth(&self) -> &Option<Arc<Mutex<dyn ClothAssetTrait>>>;
    fn cloth_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ClothAssetTrait>>>;
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
    fn cloth_wrapping(&self) -> &Option<Arc<Mutex<dyn super::cloth_base::ClothWrappingAssetTrait>>>;
    fn cloth_wrapping_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::cloth_base::ClothWrappingAssetTrait>>>;
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
    fn mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &mut self.mesh
    }
    fn mesh_name_hash(&self) -> &u32 {
        &self.mesh_name_hash
    }
    fn mesh_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.mesh_name_hash
    }
    fn cloth(&self) -> &Option<Arc<Mutex<dyn ClothAssetTrait>>> {
        &self.cloth
    }
    fn cloth_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ClothAssetTrait>>> {
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
    fn cloth_wrapping(&self) -> &Option<Arc<Mutex<dyn super::cloth_base::ClothWrappingAssetTrait>>> {
        &self.cloth_wrapping
    }
    fn cloth_wrapping_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::cloth_base::ClothWrappingAssetTrait>>> {
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
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn part_bounding_boxes(&self) -> &Vec<super::core::AxisAlignedBox> {
        self._glacier_base.part_bounding_boxes()
    }
    fn part_bounding_boxes_mut(&mut self) -> &mut Vec<super::core::AxisAlignedBox> {
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(ClothEntityData, mesh),
            },
            FieldInfoData {
                name: "MeshNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothEntityData, mesh_name_hash),
            },
            FieldInfoData {
                name: "Cloth",
                flags: MemberInfoFlags::new(0),
                field_type: "ClothAsset",
                rust_offset: offset_of!(ClothEntityData, cloth),
            },
            FieldInfoData {
                name: "CharacterLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, character_lighting_enable),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, enabled),
            },
            FieldInfoData {
                name: "WindScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ClothEntityData, wind_scale),
            },
            FieldInfoData {
                name: "WrapOnGPU",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, wrap_on_g_p_u),
            },
            FieldInfoData {
                name: "SolveOnRenderSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, solve_on_render_side),
            },
            FieldInfoData {
                name: "UseForBoundingBoxCalculations",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, use_for_bounding_box_calculations),
            },
            FieldInfoData {
                name: "ClothWrapping",
                flags: MemberInfoFlags::new(0),
                field_type: "ClothWrappingAsset",
                rust_offset: offset_of!(ClothEntityData, cloth_wrapping),
            },
            FieldInfoData {
                name: "GroundPlaneTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ClothEntityData, ground_plane_transform),
            },
            FieldInfoData {
                name: "EnableGroundPlaneCollision",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, enable_ground_plane_collision),
            },
            FieldInfoData {
                name: "LocalGroundPlaneTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothEntityData, local_ground_plane_transform),
            },
            FieldInfoData {
                name: "ActivationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(ClothEntityData, activation_radius),
            },
            FieldInfoData {
                name: "SmoothBlendOutDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(ClothEntityData, smooth_blend_out_distance),
            },
            FieldInfoData {
                name: "CollisionLayer",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothObjectVariationExampleEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub variations: Vec<Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>>>,
}

pub trait ClothObjectVariationExampleEntityDataTrait: super::entity::EntityDataTrait {
    fn variations(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>>>;
    fn variations_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>>>;
}

impl ClothObjectVariationExampleEntityDataTrait for ClothObjectVariationExampleEntityData {
    fn variations(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>>> {
        &self.variations
    }
    fn variations_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>>> {
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothObjectVariationExampleEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Variations",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothObjectVariationExampleEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothInstanceObserverEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ClothIndexInComponent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ClothInstanceObserverEntityData, cloth_index_in_component),
            },
            FieldInfoData {
                name: "ForceProcessingModeCallback",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothInstanceObserverEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothDebugRendererSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SimMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sim_mesh_line_color),
            },
            FieldInfoData {
                name: "SimMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sim_mesh_face_color),
            },
            FieldInfoData {
                name: "AnchorMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_mesh_line_color),
            },
            FieldInfoData {
                name: "AnchorMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_mesh_face_color),
            },
            FieldInfoData {
                name: "AnchorDistanceMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_distance_mesh_line_color),
            },
            FieldInfoData {
                name: "AnchorDistanceMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, anchor_distance_mesh_face_color),
            },
            FieldInfoData {
                name: "PenetrationMeshLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, penetration_mesh_line_color),
            },
            FieldInfoData {
                name: "PenetrationMeshFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, penetration_mesh_face_color),
            },
            FieldInfoData {
                name: "SphereColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sphere_collider_line_color),
            },
            FieldInfoData {
                name: "SphereColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, sphere_collider_face_color),
            },
            FieldInfoData {
                name: "CapsuleColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, capsule_collider_line_color),
            },
            FieldInfoData {
                name: "CapsuleColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, capsule_collider_face_color),
            },
            FieldInfoData {
                name: "TaperedCapsuleColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, tapered_capsule_collider_line_color),
            },
            FieldInfoData {
                name: "TaperedCapsuleColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, tapered_capsule_collider_face_color),
            },
            FieldInfoData {
                name: "BoxColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, box_collider_line_color),
            },
            FieldInfoData {
                name: "BoxColliderFaceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, box_collider_face_color),
            },
            FieldInfoData {
                name: "GroundPlaneLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, ground_plane_line_color),
            },
            FieldInfoData {
                name: "DisabledGroundPlaneLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, disabled_ground_plane_line_color),
            },
            FieldInfoData {
                name: "ClothPointColliderLineColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ClothDebugRendererSettings, cloth_point_collider_line_color),
            },
            FieldInfoData {
                name: "ActivationRadiusColor",
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothDebugRendererSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothCollisionComponent as Default>::default())),
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothCollisionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothEntity as Default>::default())),
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::IGLOOSUBSYSTEM_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothManager as Default>::default())),
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::IGLOOMODULEINITIALIZER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EAClothMemoryInitializer as Default>::default())),
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("EAClothMemoryInitializer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Cloth",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothComponent as Default>::default())),
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
    flags: MemberInfoFlags::new(145),
    module: "Cloth",
    data: TypeInfoData::Array("ClothComponent"),
    array_type: None,
    alignment: 8,
};


