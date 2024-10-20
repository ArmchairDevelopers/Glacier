use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_effect_base_types(registry: &mut TypeRegistry) {
    registry.register_type(EFFECTREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(EFFECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTBLUEPRINT_TYPE_INFO);
    registry.register_type(EFFECTBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERPARAMETER_TYPE_INFO);
    registry.register_type(EMITTERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKBASEASSET_TYPE_INFO);
    registry.register_type(MESHEMITTERMASKBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHEMITTERBASEASSET_TYPE_INFO);
    registry.register_type(MESHEMITTERBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTTRANSFORMSPACEPARAM_TYPE_INFO);
    registry.register_type(EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMS_TYPE_INFO);
    registry.register_type(EFFECTPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERLIST_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERLIST_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETER_TYPE_INFO);
    registry.register_type(EFFECTPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERSCOPETYPE_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERSCOPETYPE_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERTYPE_TYPE_INFO);
    registry.register_type(EFFECTPARAMETERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHPARAMTYPE_TYPE_INFO);
    registry.register_type(EMITTERGRAPHPARAMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDTEXTUREINPUT_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDINPUT_TYPE_INFO);
    registry.register_type(EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO);
    registry.register_type(EMITTERGRAPHOVERRIDES_TYPE_INFO);
    registry.register_type(EMITTERGRAPHOVERRIDES_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTHANDLE_TYPE_INFO);
    registry.register_type(EFFECTHANDLE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct EffectReferenceObjectData {
    pub _glacier_base: super::entity::SpatialReferenceObjectData,
    pub auto_start: bool,
    pub effect_parameters: Vec<Option<Arc<Mutex<dyn EffectParameterTrait>>>>,
    pub affected_by_lightprobe_visibility: bool,
}

pub trait EffectReferenceObjectDataTrait: super::entity::SpatialReferenceObjectDataTrait {
    fn auto_start(&self) -> &bool;
    fn effect_parameters(&self) -> &Vec<Option<Arc<Mutex<dyn EffectParameterTrait>>>>;
    fn affected_by_lightprobe_visibility(&self) -> &bool;
}

impl EffectReferenceObjectDataTrait for EffectReferenceObjectData {
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn effect_parameters(&self) -> &Vec<Option<Arc<Mutex<dyn EffectParameterTrait>>>> {
        &self.effect_parameters
    }
    fn affected_by_lightprobe_visibility(&self) -> &bool {
        &self.affected_by_lightprobe_visibility
    }
}

impl super::entity::SpatialReferenceObjectDataTrait for EffectReferenceObjectData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
}

impl super::entity::ReferenceObjectDataTrait for EffectReferenceObjectData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        self._glacier_base.blueprint()
    }
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>> {
        self._glacier_base.object_variation()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
}

impl super::entity::GameObjectDataTrait for EffectReferenceObjectData {
}

impl super::core::DataBusPeerTrait for EffectReferenceObjectData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for EffectReferenceObjectData {
}

impl super::core::DataContainerTrait for EffectReferenceObjectData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EFFECTREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectReferenceObjectData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectReferenceObjectData, auto_start),
            },
            FieldInfoData {
                name: "EffectParameters",
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(EffectReferenceObjectData, effect_parameters),
            },
            FieldInfoData {
                name: "AffectedByLightprobeVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectReferenceObjectData, affected_by_lightprobe_visibility),
            },
        ],
    }),
    array_type: Some(EFFECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EffectReferenceObjectData {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTREFERENCEOBJECTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectReferenceObjectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
    pub time_delta_type: super::entity::TimeDeltaType,
    pub is_simple: bool,
}

pub trait EffectBlueprintTrait: super::entity::ObjectBlueprintTrait {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType;
    fn is_simple(&self) -> &bool;
}

impl EffectBlueprintTrait for EffectBlueprint {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType {
        &self.time_delta_type
    }
    fn is_simple(&self) -> &bool {
        &self.is_simple
    }
}

impl super::entity::ObjectBlueprintTrait for EffectBlueprint {
    fn object(&self) -> &Option<Arc<Mutex<dyn super::entity::EntityDataTrait>>> {
        self._glacier_base.object()
    }
}

impl super::entity::BlueprintTrait for EffectBlueprint {
    fn objects(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.objects()
    }
    fn schematics(&self) -> &Option<Arc<Mutex<dyn super::schematics::SchematicsBaseAssetTrait>>> {
        self._glacier_base.schematics()
    }
}

impl super::entity::EntityBusDataTrait for EffectBlueprint {
    fn event_connections(&self) -> &Vec<super::entity::EventConnection> {
        self._glacier_base.event_connections()
    }
}

impl super::core::DataBusDataTrait for EffectBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn property_connections(&self) -> &Vec<super::core::PropertyConnection> {
        self._glacier_base.property_connections()
    }
    fn link_connections(&self) -> &Vec<super::core::LinkConnection> {
        self._glacier_base.link_connections()
    }
    fn interface(&self) -> &Option<Arc<Mutex<dyn super::core::DynamicDataContainerTrait>>> {
        self._glacier_base.interface()
    }
}

impl super::core::AssetTrait for EffectBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for EffectBlueprint {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EFFECTBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectBlueprint as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: "TimeDeltaType",
                rust_offset: offset_of!(EffectBlueprint, time_delta_type),
            },
            FieldInfoData {
                name: "IsSimple",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectBlueprint, is_simple),
            },
        ],
    }),
    array_type: Some(EFFECTBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectBlueprint {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTBLUEPRINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectBlueprint"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterParameter {
    #[default]
    EmitterParameterNone = 0,
    EmitterParameter1 = 1,
    EmitterParameter2 = 2,
    EmitterParameter3 = 3,
    EmitterParameterVec = 5,
    EmitterParameterVecAverage = 7,
    EmitterParameterDistance = 6,
    EmitterParameterCount = 8,
    EmitterParameter4 = 4,
}

pub static EMITTERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParameter",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterParameter {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERPARAMETER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshEmitterMaskBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait MeshEmitterMaskBaseAssetTrait: super::core::AssetTrait {
}

impl MeshEmitterMaskBaseAssetTrait for MeshEmitterMaskBaseAsset {
}

impl super::core::AssetTrait for MeshEmitterMaskBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for MeshEmitterMaskBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MESHEMITTERMASKBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterMaskBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERMASKBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterMaskBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHEMITTERMASKBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHEMITTERMASKBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("MeshEmitterMaskBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshEmitterBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait MeshEmitterBaseAssetTrait: super::core::AssetTrait {
}

impl MeshEmitterBaseAssetTrait for MeshEmitterBaseAsset {
}

impl super::core::AssetTrait for MeshEmitterBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for MeshEmitterBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MESHEMITTERBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHEMITTERBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshEmitterBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHEMITTERBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHEMITTERBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("MeshEmitterBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectTransformSpaceParam {
    pub index: u32,
    pub transform_space: super::state_stream::TransformSpaceHandle,
}

pub trait EffectTransformSpaceParamTrait: TypeObject {
    fn index(&self) -> &u32;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
}

impl EffectTransformSpaceParamTrait for EffectTransformSpaceParam {
    fn index(&self) -> &u32 {
        &self.index
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
}

pub static EFFECTTRANSFORMSPACEPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectTransformSpaceParam",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectTransformSpaceParam as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EffectTransformSpaceParam, index),
            },
            FieldInfoData {
                name: "TransformSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "TransformSpaceHandle",
                rust_offset: offset_of!(EffectTransformSpaceParam, transform_space),
            },
        ],
    }),
    array_type: Some(EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EffectTransformSpaceParam {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTTRANSFORMSPACEPARAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectTransformSpaceParam-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectTransformSpaceParam"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectParams {
}

pub trait EffectParamsTrait: TypeObject {
}

impl EffectParamsTrait for EffectParams {
}

pub static EFFECTPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParams",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectParams as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EFFECTPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EffectParams {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTPARAMS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectParameterList {
    pub _glacier_base: super::core::Asset,
    pub parameters: Vec<Option<Arc<Mutex<dyn EffectParameterTrait>>>>,
}

pub trait EffectParameterListTrait: super::core::AssetTrait {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn EffectParameterTrait>>>>;
}

impl EffectParameterListTrait for EffectParameterList {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn EffectParameterTrait>>>> {
        &self.parameters
    }
}

impl super::core::AssetTrait for EffectParameterList {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for EffectParameterList {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EFFECTPARAMETERLIST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterList",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectParameterList as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Parameters",
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(EffectParameterList, parameters),
            },
        ],
    }),
    array_type: Some(EFFECTPARAMETERLIST_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectParameterList {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTPARAMETERLIST_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTPARAMETERLIST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterList-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameterList"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectParameter {
    pub _glacier_base: super::core::DataContainer,
    pub name: String,
    pub param_type: EffectParameterType,
    pub param_scope: EffectParameterScopeType,
}

pub trait EffectParameterTrait: super::core::DataContainerTrait {
    fn name(&self) -> &String;
    fn param_type(&self) -> &EffectParameterType;
    fn param_scope(&self) -> &EffectParameterScopeType;
}

impl EffectParameterTrait for EffectParameter {
    fn name(&self) -> &String {
        &self.name
    }
    fn param_type(&self) -> &EffectParameterType {
        &self.param_type
    }
    fn param_scope(&self) -> &EffectParameterScopeType {
        &self.param_scope
    }
}

impl super::core::DataContainerTrait for EffectParameter {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EFFECTPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameter",
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EffectParameter, name),
            },
            FieldInfoData {
                name: "ParamType",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameterType",
                rust_offset: offset_of!(EffectParameter, param_type),
            },
            FieldInfoData {
                name: "ParamScope",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameterScopeType",
                rust_offset: offset_of!(EffectParameter, param_scope),
            },
        ],
    }),
    array_type: Some(EFFECTPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectParameter {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTPARAMETER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EffectParameterScopeType {
    #[default]
    EffectParameterScopeType_Local = 0,
    EffectParameterScopeType_Gobal = 1,
}

pub static EFFECTPARAMETERSCOPETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterScopeType",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EFFECTPARAMETERSCOPETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EffectParameterScopeType {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTPARAMETERSCOPETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTPARAMETERSCOPETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterScopeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameterScopeType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EffectParameterType {
    #[default]
    EffectParameterType_Float = 0,
    EffectParameterType_Vec3 = 1,
    EffectParameterType_Bool = 2,
    EffectParameterType_Int = 3,
    EffectParameterType_MeshEmitter = 4,
    EffectParameterType_MeshEmitterMask = 5,
    EffectParameterType_SpaceAsPosition = 6,
}

pub static EFFECTPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EFFECTPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EffectParameterType {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTPARAMETERTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameterType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EmitterGraphParamType {
    #[default]
    EmitterGraphParamType_Constant = 0,
    EmitterGraphParamType_Schematics = 1,
}

pub static EMITTERGRAPHPARAMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParamType",
    flags: MemberInfoFlags::new(49429),
    module: "EffectBase",
    data: TypeInfoData::Enum,
    array_type: Some(EMITTERGRAPHPARAMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmitterGraphParamType {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHPARAMTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTERGRAPHPARAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParamType-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterGraphParamType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExposedTextureInput {
    pub shader_parameter_handle: u32,
    pub texture: Option<Arc<Mutex<dyn super::core::AssetTrait>>>,
}

pub trait EmitterExposedTextureInputTrait: TypeObject {
    fn shader_parameter_handle(&self) -> &u32;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
}

impl EmitterExposedTextureInputTrait for EmitterExposedTextureInput {
    fn shader_parameter_handle(&self) -> &u32 {
        &self.shader_parameter_handle
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &self.texture
    }
}

pub static EMITTEREXPOSEDTEXTUREINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedTextureInput",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExposedTextureInput as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderParameterHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterExposedTextureInput, shader_parameter_handle),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(EmitterExposedTextureInput, texture),
            },
        ],
    }),
    array_type: Some(EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EmitterExposedTextureInput {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXPOSEDTEXTUREINPUT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedTextureInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterExposedTextureInput"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterExposedInput {
    pub property_id: i32,
    pub value: super::core::Vec4,
}

pub trait EmitterExposedInputTrait: TypeObject {
    fn property_id(&self) -> &i32;
    fn value(&self) -> &super::core::Vec4;
}

impl EmitterExposedInputTrait for EmitterExposedInput {
    fn property_id(&self) -> &i32 {
        &self.property_id
    }
    fn value(&self) -> &super::core::Vec4 {
        &self.value
    }
}

pub static EMITTEREXPOSEDINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedInput",
    flags: MemberInfoFlags::new(32841),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExposedInput as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PropertyId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterExposedInput, property_id),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(EmitterExposedInput, value),
            },
        ],
    }),
    array_type: Some(EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterExposedInput {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTEREXPOSEDINPUT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterExposedInput"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EmitterGraphOverrides {
    pub spawn_rate: super::core::QualityScalableFloat,
    pub particle_max_count: super::core::QualityScalableInt,
    pub particle_life_span: super::core::QualityScalableFloat,
    pub emitter_life_span: super::core::QualityScalableFloat,
    pub bounding_box_min: super::core::Vec3,
    pub bounding_box_max: super::core::Vec3,
    pub emitter_min_spawn_distance: super::core::QualityScalableFloat,
    pub emitter_max_spawn_distance: super::core::QualityScalableFloat,
    pub spawn_outside_view_radius: f32,
    pub is_spawn_rate_override_set: bool,
    pub is_particle_max_count_override_set: bool,
    pub is_particle_life_span_override_set: bool,
    pub is_emitter_life_span_override_set: bool,
    pub is_bounding_box_min_set: bool,
    pub is_bounding_box_max_set: bool,
    pub is_emitter_min_spawn_distance_override_set: bool,
    pub is_emitter_max_spawn_distance_override_set: bool,
    pub is_spawn_outside_view_radius_override_set: bool,
}

pub trait EmitterGraphOverridesTrait: TypeObject {
    fn spawn_rate(&self) -> &super::core::QualityScalableFloat;
    fn particle_max_count(&self) -> &super::core::QualityScalableInt;
    fn particle_life_span(&self) -> &super::core::QualityScalableFloat;
    fn emitter_life_span(&self) -> &super::core::QualityScalableFloat;
    fn bounding_box_min(&self) -> &super::core::Vec3;
    fn bounding_box_max(&self) -> &super::core::Vec3;
    fn emitter_min_spawn_distance(&self) -> &super::core::QualityScalableFloat;
    fn emitter_max_spawn_distance(&self) -> &super::core::QualityScalableFloat;
    fn spawn_outside_view_radius(&self) -> &f32;
    fn is_spawn_rate_override_set(&self) -> &bool;
    fn is_particle_max_count_override_set(&self) -> &bool;
    fn is_particle_life_span_override_set(&self) -> &bool;
    fn is_emitter_life_span_override_set(&self) -> &bool;
    fn is_bounding_box_min_set(&self) -> &bool;
    fn is_bounding_box_max_set(&self) -> &bool;
    fn is_emitter_min_spawn_distance_override_set(&self) -> &bool;
    fn is_emitter_max_spawn_distance_override_set(&self) -> &bool;
    fn is_spawn_outside_view_radius_override_set(&self) -> &bool;
}

impl EmitterGraphOverridesTrait for EmitterGraphOverrides {
    fn spawn_rate(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_rate
    }
    fn particle_max_count(&self) -> &super::core::QualityScalableInt {
        &self.particle_max_count
    }
    fn particle_life_span(&self) -> &super::core::QualityScalableFloat {
        &self.particle_life_span
    }
    fn emitter_life_span(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_life_span
    }
    fn bounding_box_min(&self) -> &super::core::Vec3 {
        &self.bounding_box_min
    }
    fn bounding_box_max(&self) -> &super::core::Vec3 {
        &self.bounding_box_max
    }
    fn emitter_min_spawn_distance(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_min_spawn_distance
    }
    fn emitter_max_spawn_distance(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_max_spawn_distance
    }
    fn spawn_outside_view_radius(&self) -> &f32 {
        &self.spawn_outside_view_radius
    }
    fn is_spawn_rate_override_set(&self) -> &bool {
        &self.is_spawn_rate_override_set
    }
    fn is_particle_max_count_override_set(&self) -> &bool {
        &self.is_particle_max_count_override_set
    }
    fn is_particle_life_span_override_set(&self) -> &bool {
        &self.is_particle_life_span_override_set
    }
    fn is_emitter_life_span_override_set(&self) -> &bool {
        &self.is_emitter_life_span_override_set
    }
    fn is_bounding_box_min_set(&self) -> &bool {
        &self.is_bounding_box_min_set
    }
    fn is_bounding_box_max_set(&self) -> &bool {
        &self.is_bounding_box_max_set
    }
    fn is_emitter_min_spawn_distance_override_set(&self) -> &bool {
        &self.is_emitter_min_spawn_distance_override_set
    }
    fn is_emitter_max_spawn_distance_override_set(&self) -> &bool {
        &self.is_emitter_max_spawn_distance_override_set
    }
    fn is_spawn_outside_view_radius_override_set(&self) -> &bool {
        &self.is_spawn_outside_view_radius_override_set
    }
}

pub static EMITTERGRAPHOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphOverrides",
    flags: MemberInfoFlags::new(32841),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraphOverrides as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, spawn_rate),
            },
            FieldInfoData {
                name: "ParticleMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(EmitterGraphOverrides, particle_max_count),
            },
            FieldInfoData {
                name: "ParticleLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, particle_life_span),
            },
            FieldInfoData {
                name: "EmitterLifeSpan",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_life_span),
            },
            FieldInfoData {
                name: "BoundingBoxMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterGraphOverrides, bounding_box_min),
            },
            FieldInfoData {
                name: "BoundingBoxMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterGraphOverrides, bounding_box_max),
            },
            FieldInfoData {
                name: "EmitterMinSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_min_spawn_distance),
            },
            FieldInfoData {
                name: "EmitterMaxSpawnDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_max_spawn_distance),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterGraphOverrides, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "IsSpawnRateOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_spawn_rate_override_set),
            },
            FieldInfoData {
                name: "IsParticleMaxCountOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_particle_max_count_override_set),
            },
            FieldInfoData {
                name: "IsParticleLifeSpanOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_particle_life_span_override_set),
            },
            FieldInfoData {
                name: "IsEmitterLifeSpanOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_life_span_override_set),
            },
            FieldInfoData {
                name: "IsBoundingBoxMinSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_bounding_box_min_set),
            },
            FieldInfoData {
                name: "IsBoundingBoxMaxSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_bounding_box_max_set),
            },
            FieldInfoData {
                name: "IsEmitterMinSpawnDistanceOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_min_spawn_distance_override_set),
            },
            FieldInfoData {
                name: "IsEmitterMaxSpawnDistanceOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_max_spawn_distance_override_set),
            },
            FieldInfoData {
                name: "IsSpawnOutsideViewRadiusOverrideSet",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_spawn_outside_view_radius_override_set),
            },
        ],
    }),
    array_type: Some(EMITTERGRAPHOVERRIDES_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EmitterGraphOverrides {
    fn type_info(&self) -> &'static TypeInfo {
        EMITTERGRAPHOVERRIDES_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EMITTERGRAPHOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphOverrides-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterGraphOverrides"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectHandle {
}

pub trait EffectHandleTrait: TypeObject {
}

impl EffectHandleTrait for EffectHandle {
}

pub static EFFECTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectHandle",
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EFFECTHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectHandle {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectHandle"),
    array_type: None,
    alignment: 8,
};


