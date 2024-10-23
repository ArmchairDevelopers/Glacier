use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectReferenceObjectData {
    pub _glacier_base: super::entity::SpatialReferenceObjectData,
    pub auto_start: bool,
    pub effect_parameters: Vec<Option<LockedTypeObject /* EffectParameter */>>,
    pub affected_by_lightprobe_visibility: bool,
}

pub trait EffectReferenceObjectDataTrait: super::entity::SpatialReferenceObjectDataTrait {
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* EffectParameter */>>;
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* EffectParameter */>>;
    fn affected_by_lightprobe_visibility(&self) -> &bool;
    fn affected_by_lightprobe_visibility_mut(&mut self) -> &mut bool;
}

impl EffectReferenceObjectDataTrait for EffectReferenceObjectData {
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* EffectParameter */>> {
        &self.effect_parameters
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* EffectParameter */>> {
        &mut self.effect_parameters
    }
    fn affected_by_lightprobe_visibility(&self) -> &bool {
        &self.affected_by_lightprobe_visibility
    }
    fn affected_by_lightprobe_visibility_mut(&mut self) -> &mut bool {
        &mut self.affected_by_lightprobe_visibility
    }
}

impl super::entity::SpatialReferenceObjectDataTrait for EffectReferenceObjectData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for EffectReferenceObjectData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.blueprint_transform_mut()
    }
    fn blueprint(&self) -> &Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint()
    }
    fn blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint_mut()
    }
    fn object_variation(&self) -> &Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation()
    }
    fn object_variation_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation_mut()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        self._glacier_base.stream_realm_mut()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn radiosity_type_override_mut(&mut self) -> &mut super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override_mut()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_resolution_scale_mut(&mut self) -> &mut u32 {
        self._glacier_base.lightmap_resolution_scale_mut()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn lightmap_scale_with_size_mut(&mut self) -> &mut bool {
        self._glacier_base.lightmap_scale_with_size_mut()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn rendering_overrides_mut(&mut self) -> &mut super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
    fn create_indestructible_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.create_indestructible_entity_mut()
    }
}

impl super::entity::GameObjectDataTrait for EffectReferenceObjectData {
}

impl super::core::DataBusPeerTrait for EffectReferenceObjectData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EffectReferenceObjectData {
}

impl super::core::DataContainerTrait for EffectReferenceObjectData {
}

pub static EFFECTREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectReferenceObjectData",
    name_hash: 2637079484,
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(EffectReferenceObjectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectReferenceObjectData as Default>::default())),
            create_boxed: || Box::new(<EffectReferenceObjectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AutoStart",
                name_hash: 792615882,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EffectReferenceObjectData, auto_start),
            },
            FieldInfoData {
                name: "EffectParameters",
                name_hash: 929782248,
                flags: MemberInfoFlags::new(144),
                field_type: "EffectParameter-Array",
                rust_offset: offset_of!(EffectReferenceObjectData, effect_parameters),
            },
            FieldInfoData {
                name: "AffectedByLightprobeVisibility",
                name_hash: 3285178174,
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


pub static EFFECTREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectReferenceObjectData-Array",
    name_hash: 3289062152,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectReferenceObjectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
    pub time_delta_type: super::entity::TimeDeltaType,
    pub is_simple: bool,
}

pub trait EffectBlueprintTrait: super::entity::ObjectBlueprintTrait {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType;
    fn time_delta_type_mut(&mut self) -> &mut super::entity::TimeDeltaType;
    fn is_simple(&self) -> &bool;
    fn is_simple_mut(&mut self) -> &mut bool;
}

impl EffectBlueprintTrait for EffectBlueprint {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType {
        &self.time_delta_type
    }
    fn time_delta_type_mut(&mut self) -> &mut super::entity::TimeDeltaType {
        &mut self.time_delta_type
    }
    fn is_simple(&self) -> &bool {
        &self.is_simple
    }
    fn is_simple_mut(&mut self) -> &mut bool {
        &mut self.is_simple
    }
}

impl super::entity::ObjectBlueprintTrait for EffectBlueprint {
    fn object(&self) -> &Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object()
    }
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object_mut()
    }
}

impl super::entity::BlueprintTrait for EffectBlueprint {
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

impl super::entity::EntityBusDataTrait for EffectBlueprint {
    fn event_connections(&self) -> &Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for EffectBlueprint {
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

impl super::core::AssetTrait for EffectBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EffectBlueprint {
}

pub static EFFECTBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectBlueprint",
    name_hash: 3756200925,
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        super_class_offset: offset_of!(EffectBlueprint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectBlueprint as Default>::default())),
            create_boxed: || Box::new(<EffectBlueprint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                name_hash: 1100516816,
                flags: MemberInfoFlags::new(0),
                field_type: "TimeDeltaType",
                rust_offset: offset_of!(EffectBlueprint, time_delta_type),
            },
            FieldInfoData {
                name: "IsSimple",
                name_hash: 447223665,
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


pub static EFFECTBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectBlueprint-Array",
    name_hash: 2538221545,
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
    name_hash: 3358446298,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EMITTERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterParameter-Array",
    name_hash: 2434672238,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MeshEmitterMaskBaseAsset {
}

pub static MESHEMITTERMASKBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskBaseAsset",
    name_hash: 167449393,
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(MeshEmitterMaskBaseAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterMaskBaseAsset as Default>::default())),
            create_boxed: || Box::new(<MeshEmitterMaskBaseAsset as Default>::default()),
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


pub static MESHEMITTERMASKBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterMaskBaseAsset-Array",
    name_hash: 140862853,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("MeshEmitterMaskBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MeshEmitterBaseAsset {
}

pub static MESHEMITTERBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterBaseAsset",
    name_hash: 3401147781,
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(MeshEmitterBaseAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshEmitterBaseAsset as Default>::default())),
            create_boxed: || Box::new(<MeshEmitterBaseAsset as Default>::default()),
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


pub static MESHEMITTERBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshEmitterBaseAsset-Array",
    name_hash: 2855841073,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("MeshEmitterBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectTransformSpaceParam {
    pub index: u32,
    pub transform_space: super::state_stream::TransformSpaceHandle,
}

pub trait EffectTransformSpaceParamTrait: TypeObject {
    fn index(&self) -> &u32;
    fn index_mut(&mut self) -> &mut u32;
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle;
    fn transform_space_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle;
}

impl EffectTransformSpaceParamTrait for EffectTransformSpaceParam {
    fn index(&self) -> &u32 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
    fn transform_space(&self) -> &super::state_stream::TransformSpaceHandle {
        &self.transform_space
    }
    fn transform_space_mut(&mut self) -> &mut super::state_stream::TransformSpaceHandle {
        &mut self.transform_space
    }
}

pub static EFFECTTRANSFORMSPACEPARAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectTransformSpaceParam",
    name_hash: 3805789525,
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectTransformSpaceParam as Default>::default())),
            create_boxed: || Box::new(<EffectTransformSpaceParam as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EffectTransformSpaceParam, index),
            },
            FieldInfoData {
                name: "TransformSpace",
                name_hash: 3602558253,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EFFECTTRANSFORMSPACEPARAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectTransformSpaceParam-Array",
    name_hash: 1845009761,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectTransformSpaceParam"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectParams {
}

pub trait EffectParamsTrait: TypeObject {
}

impl EffectParamsTrait for EffectParams {
}

pub static EFFECTPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParams",
    name_hash: 2299616430,
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectParams as Default>::default())),
            create_boxed: || Box::new(<EffectParams as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EFFECTPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParams-Array",
    name_hash: 838485274,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectParameterList {
    pub _glacier_base: super::core::Asset,
    pub parameters: Vec<Option<LockedTypeObject /* EffectParameter */>>,
}

pub trait EffectParameterListTrait: super::core::AssetTrait {
    fn parameters(&self) -> &Vec<Option<LockedTypeObject /* EffectParameter */>>;
    fn parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* EffectParameter */>>;
}

impl EffectParameterListTrait for EffectParameterList {
    fn parameters(&self) -> &Vec<Option<LockedTypeObject /* EffectParameter */>> {
        &self.parameters
    }
    fn parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* EffectParameter */>> {
        &mut self.parameters
    }
}

impl super::core::AssetTrait for EffectParameterList {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EffectParameterList {
}

pub static EFFECTPARAMETERLIST_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterList",
    name_hash: 3033485049,
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(EffectParameterList, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectParameterList as Default>::default())),
            create_boxed: || Box::new(<EffectParameterList as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Parameters",
                name_hash: 3325515039,
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


pub static EFFECTPARAMETERLIST_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterList-Array",
    name_hash: 901189069,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectParameterList"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectParameter {
    pub _glacier_base: super::core::DataContainer,
    pub name: String,
    pub param_type: EffectParameterType,
    pub param_scope: EffectParameterScopeType,
}

pub trait EffectParameterTrait: super::core::DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn param_type(&self) -> &EffectParameterType;
    fn param_type_mut(&mut self) -> &mut EffectParameterType;
    fn param_scope(&self) -> &EffectParameterScopeType;
    fn param_scope_mut(&mut self) -> &mut EffectParameterScopeType;
}

impl EffectParameterTrait for EffectParameter {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn param_type(&self) -> &EffectParameterType {
        &self.param_type
    }
    fn param_type_mut(&mut self) -> &mut EffectParameterType {
        &mut self.param_type
    }
    fn param_scope(&self) -> &EffectParameterScopeType {
        &self.param_scope
    }
    fn param_scope_mut(&mut self) -> &mut EffectParameterScopeType {
        &mut self.param_scope
    }
}

impl super::core::DataContainerTrait for EffectParameter {
}

pub static EFFECTPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameter",
    name_hash: 1850282555,
    flags: MemberInfoFlags::new(101),
    module: "EffectBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(EffectParameter, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectParameter as Default>::default())),
            create_boxed: || Box::new(<EffectParameter as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EffectParameter, name),
            },
            FieldInfoData {
                name: "ParamType",
                name_hash: 2965736018,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameterType",
                rust_offset: offset_of!(EffectParameter, param_type),
            },
            FieldInfoData {
                name: "ParamScope",
                name_hash: 3377213504,
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


pub static EFFECTPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameter-Array",
    name_hash: 3517872527,
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
    name_hash: 42479369,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EFFECTPARAMETERSCOPETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterScopeType-Array",
    name_hash: 479741117,
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
    name_hash: 3033197507,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EFFECTPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectParameterType-Array",
    name_hash: 3339611895,
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
    name_hash: 1660145800,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EMITTERGRAPHPARAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphParamType-Array",
    name_hash: 46936764,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterGraphParamType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EmitterExposedTextureInput {
    pub shader_parameter_handle: u32,
    pub texture: Option<LockedTypeObject /* super::core::Asset */>,
}

pub trait EmitterExposedTextureInputTrait: TypeObject {
    fn shader_parameter_handle(&self) -> &u32;
    fn shader_parameter_handle_mut(&mut self) -> &mut u32;
    fn texture(&self) -> &Option<LockedTypeObject /* super::core::Asset */>;
    fn texture_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::Asset */>;
}

impl EmitterExposedTextureInputTrait for EmitterExposedTextureInput {
    fn shader_parameter_handle(&self) -> &u32 {
        &self.shader_parameter_handle
    }
    fn shader_parameter_handle_mut(&mut self) -> &mut u32 {
        &mut self.shader_parameter_handle
    }
    fn texture(&self) -> &Option<LockedTypeObject /* super::core::Asset */> {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::Asset */> {
        &mut self.texture
    }
}

pub static EMITTEREXPOSEDTEXTUREINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedTextureInput",
    name_hash: 3290871530,
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExposedTextureInput as Default>::default())),
            create_boxed: || Box::new(<EmitterExposedTextureInput as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderParameterHandle",
                name_hash: 4141069103,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EmitterExposedTextureInput, shader_parameter_handle),
            },
            FieldInfoData {
                name: "Texture",
                name_hash: 3185041626,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EMITTEREXPOSEDTEXTUREINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedTextureInput-Array",
    name_hash: 3056230878,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterExposedTextureInput"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EmitterExposedInput {
    pub property_id: i32,
    pub value: super::core::Vec4,
}

pub trait EmitterExposedInputTrait: TypeObject {
    fn property_id(&self) -> &i32;
    fn property_id_mut(&mut self) -> &mut i32;
    fn value(&self) -> &super::core::Vec4;
    fn value_mut(&mut self) -> &mut super::core::Vec4;
}

impl EmitterExposedInputTrait for EmitterExposedInput {
    fn property_id(&self) -> &i32 {
        &self.property_id
    }
    fn property_id_mut(&mut self) -> &mut i32 {
        &mut self.property_id
    }
    fn value(&self) -> &super::core::Vec4 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.value
    }
}

pub static EMITTEREXPOSEDINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedInput",
    name_hash: 2819021461,
    flags: MemberInfoFlags::new(32841),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterExposedInput as Default>::default())),
            create_boxed: || Box::new(<EmitterExposedInput as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PropertyId",
                name_hash: 1506318447,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EmitterExposedInput, property_id),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EMITTEREXPOSEDINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterExposedInput-Array",
    name_hash: 3130829345,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterExposedInput"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn particle_max_count(&self) -> &super::core::QualityScalableInt;
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt;
    fn particle_life_span(&self) -> &super::core::QualityScalableFloat;
    fn particle_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn emitter_life_span(&self) -> &super::core::QualityScalableFloat;
    fn emitter_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn bounding_box_min(&self) -> &super::core::Vec3;
    fn bounding_box_min_mut(&mut self) -> &mut super::core::Vec3;
    fn bounding_box_max(&self) -> &super::core::Vec3;
    fn bounding_box_max_mut(&mut self) -> &mut super::core::Vec3;
    fn emitter_min_spawn_distance(&self) -> &super::core::QualityScalableFloat;
    fn emitter_min_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn emitter_max_spawn_distance(&self) -> &super::core::QualityScalableFloat;
    fn emitter_max_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat;
    fn spawn_outside_view_radius(&self) -> &f32;
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32;
    fn is_spawn_rate_override_set(&self) -> &bool;
    fn is_spawn_rate_override_set_mut(&mut self) -> &mut bool;
    fn is_particle_max_count_override_set(&self) -> &bool;
    fn is_particle_max_count_override_set_mut(&mut self) -> &mut bool;
    fn is_particle_life_span_override_set(&self) -> &bool;
    fn is_particle_life_span_override_set_mut(&mut self) -> &mut bool;
    fn is_emitter_life_span_override_set(&self) -> &bool;
    fn is_emitter_life_span_override_set_mut(&mut self) -> &mut bool;
    fn is_bounding_box_min_set(&self) -> &bool;
    fn is_bounding_box_min_set_mut(&mut self) -> &mut bool;
    fn is_bounding_box_max_set(&self) -> &bool;
    fn is_bounding_box_max_set_mut(&mut self) -> &mut bool;
    fn is_emitter_min_spawn_distance_override_set(&self) -> &bool;
    fn is_emitter_min_spawn_distance_override_set_mut(&mut self) -> &mut bool;
    fn is_emitter_max_spawn_distance_override_set(&self) -> &bool;
    fn is_emitter_max_spawn_distance_override_set_mut(&mut self) -> &mut bool;
    fn is_spawn_outside_view_radius_override_set(&self) -> &bool;
    fn is_spawn_outside_view_radius_override_set_mut(&mut self) -> &mut bool;
}

impl EmitterGraphOverridesTrait for EmitterGraphOverrides {
    fn spawn_rate(&self) -> &super::core::QualityScalableFloat {
        &self.spawn_rate
    }
    fn spawn_rate_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.spawn_rate
    }
    fn particle_max_count(&self) -> &super::core::QualityScalableInt {
        &self.particle_max_count
    }
    fn particle_max_count_mut(&mut self) -> &mut super::core::QualityScalableInt {
        &mut self.particle_max_count
    }
    fn particle_life_span(&self) -> &super::core::QualityScalableFloat {
        &self.particle_life_span
    }
    fn particle_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.particle_life_span
    }
    fn emitter_life_span(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_life_span
    }
    fn emitter_life_span_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.emitter_life_span
    }
    fn bounding_box_min(&self) -> &super::core::Vec3 {
        &self.bounding_box_min
    }
    fn bounding_box_min_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bounding_box_min
    }
    fn bounding_box_max(&self) -> &super::core::Vec3 {
        &self.bounding_box_max
    }
    fn bounding_box_max_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.bounding_box_max
    }
    fn emitter_min_spawn_distance(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_min_spawn_distance
    }
    fn emitter_min_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.emitter_min_spawn_distance
    }
    fn emitter_max_spawn_distance(&self) -> &super::core::QualityScalableFloat {
        &self.emitter_max_spawn_distance
    }
    fn emitter_max_spawn_distance_mut(&mut self) -> &mut super::core::QualityScalableFloat {
        &mut self.emitter_max_spawn_distance
    }
    fn spawn_outside_view_radius(&self) -> &f32 {
        &self.spawn_outside_view_radius
    }
    fn spawn_outside_view_radius_mut(&mut self) -> &mut f32 {
        &mut self.spawn_outside_view_radius
    }
    fn is_spawn_rate_override_set(&self) -> &bool {
        &self.is_spawn_rate_override_set
    }
    fn is_spawn_rate_override_set_mut(&mut self) -> &mut bool {
        &mut self.is_spawn_rate_override_set
    }
    fn is_particle_max_count_override_set(&self) -> &bool {
        &self.is_particle_max_count_override_set
    }
    fn is_particle_max_count_override_set_mut(&mut self) -> &mut bool {
        &mut self.is_particle_max_count_override_set
    }
    fn is_particle_life_span_override_set(&self) -> &bool {
        &self.is_particle_life_span_override_set
    }
    fn is_particle_life_span_override_set_mut(&mut self) -> &mut bool {
        &mut self.is_particle_life_span_override_set
    }
    fn is_emitter_life_span_override_set(&self) -> &bool {
        &self.is_emitter_life_span_override_set
    }
    fn is_emitter_life_span_override_set_mut(&mut self) -> &mut bool {
        &mut self.is_emitter_life_span_override_set
    }
    fn is_bounding_box_min_set(&self) -> &bool {
        &self.is_bounding_box_min_set
    }
    fn is_bounding_box_min_set_mut(&mut self) -> &mut bool {
        &mut self.is_bounding_box_min_set
    }
    fn is_bounding_box_max_set(&self) -> &bool {
        &self.is_bounding_box_max_set
    }
    fn is_bounding_box_max_set_mut(&mut self) -> &mut bool {
        &mut self.is_bounding_box_max_set
    }
    fn is_emitter_min_spawn_distance_override_set(&self) -> &bool {
        &self.is_emitter_min_spawn_distance_override_set
    }
    fn is_emitter_min_spawn_distance_override_set_mut(&mut self) -> &mut bool {
        &mut self.is_emitter_min_spawn_distance_override_set
    }
    fn is_emitter_max_spawn_distance_override_set(&self) -> &bool {
        &self.is_emitter_max_spawn_distance_override_set
    }
    fn is_emitter_max_spawn_distance_override_set_mut(&mut self) -> &mut bool {
        &mut self.is_emitter_max_spawn_distance_override_set
    }
    fn is_spawn_outside_view_radius_override_set(&self) -> &bool {
        &self.is_spawn_outside_view_radius_override_set
    }
    fn is_spawn_outside_view_radius_override_set_mut(&mut self) -> &mut bool {
        &mut self.is_spawn_outside_view_radius_override_set
    }
}

pub static EMITTERGRAPHOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphOverrides",
    name_hash: 3103421752,
    flags: MemberInfoFlags::new(32841),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmitterGraphOverrides as Default>::default())),
            create_boxed: || Box::new(<EmitterGraphOverrides as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SpawnRate",
                name_hash: 2317161148,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, spawn_rate),
            },
            FieldInfoData {
                name: "ParticleMaxCount",
                name_hash: 1668354694,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(EmitterGraphOverrides, particle_max_count),
            },
            FieldInfoData {
                name: "ParticleLifeSpan",
                name_hash: 355291419,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, particle_life_span),
            },
            FieldInfoData {
                name: "EmitterLifeSpan",
                name_hash: 2884727609,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_life_span),
            },
            FieldInfoData {
                name: "BoundingBoxMin",
                name_hash: 2339812968,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterGraphOverrides, bounding_box_min),
            },
            FieldInfoData {
                name: "BoundingBoxMax",
                name_hash: 2339813238,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EmitterGraphOverrides, bounding_box_max),
            },
            FieldInfoData {
                name: "EmitterMinSpawnDistance",
                name_hash: 4028457313,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_min_spawn_distance),
            },
            FieldInfoData {
                name: "EmitterMaxSpawnDistance",
                name_hash: 1544874047,
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableFloat",
                rust_offset: offset_of!(EmitterGraphOverrides, emitter_max_spawn_distance),
            },
            FieldInfoData {
                name: "SpawnOutsideViewRadius",
                name_hash: 3760047582,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EmitterGraphOverrides, spawn_outside_view_radius),
            },
            FieldInfoData {
                name: "IsSpawnRateOverrideSet",
                name_hash: 2368048624,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_spawn_rate_override_set),
            },
            FieldInfoData {
                name: "IsParticleMaxCountOverrideSet",
                name_hash: 1180470666,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_particle_max_count_override_set),
            },
            FieldInfoData {
                name: "IsParticleLifeSpanOverrideSet",
                name_hash: 1898410295,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_particle_life_span_override_set),
            },
            FieldInfoData {
                name: "IsEmitterLifeSpanOverrideSet",
                name_hash: 2357632917,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_life_span_override_set),
            },
            FieldInfoData {
                name: "IsBoundingBoxMinSet",
                name_hash: 3965743376,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_bounding_box_min_set),
            },
            FieldInfoData {
                name: "IsBoundingBoxMaxSet",
                name_hash: 3956227534,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_bounding_box_max_set),
            },
            FieldInfoData {
                name: "IsEmitterMinSpawnDistanceOverrideSet",
                name_hash: 411212877,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_min_spawn_distance_override_set),
            },
            FieldInfoData {
                name: "IsEmitterMaxSpawnDistanceOverrideSet",
                name_hash: 1525847507,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EmitterGraphOverrides, is_emitter_max_spawn_distance_override_set),
            },
            FieldInfoData {
                name: "IsSpawnOutsideViewRadiusOverrideSet",
                name_hash: 1561894482,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EMITTERGRAPHOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmitterGraphOverrides-Array",
    name_hash: 4086193804,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EmitterGraphOverrides"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectHandle {
}

pub trait EffectHandleTrait: TypeObject {
}

impl EffectHandleTrait for EffectHandle {
}

pub static EFFECTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectHandle",
    name_hash: 3238141784,
    flags: MemberInfoFlags::new(73),
    module: "EffectBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectHandle as Default>::default())),
            create_boxed: || Box::new(<EffectHandle as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EFFECTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectHandle-Array",
    name_hash: 98654700,
    flags: MemberInfoFlags::new(145),
    module: "EffectBase",
    data: TypeInfoData::Array("EffectHandle"),
    array_type: None,
    alignment: 8,
};


