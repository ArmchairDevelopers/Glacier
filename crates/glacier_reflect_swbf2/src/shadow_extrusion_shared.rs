use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_shadow_extrusion_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(SHADOWEXTRUSIONLEVELSETTINGS_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLEVELSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLEVELDATAENTITYDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONLEVELDATAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONDATAENTITYDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONDATAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONASSET_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTDATA_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTINSTANCE_TYPE_INFO);
    registry.register_type(SHADOWEXTRUSIONOBJECTINSTANCE_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionLevelSettings {
    pub _glacier_base: super::entity::SubWorldDataComponent,
    pub dynamic_light_direction: bool,
    pub allow_extrusion_back_face_hit: bool,
    pub allow_full_occluded_back_face_hit: bool,
    pub additional_extrusion_length: f32,
}

pub trait ShadowExtrusionLevelSettingsTrait: super::entity::SubWorldDataComponentTrait {
    fn dynamic_light_direction(&self) -> &bool;
    fn dynamic_light_direction_mut(&mut self) -> &mut bool;
    fn allow_extrusion_back_face_hit(&self) -> &bool;
    fn allow_extrusion_back_face_hit_mut(&mut self) -> &mut bool;
    fn allow_full_occluded_back_face_hit(&self) -> &bool;
    fn allow_full_occluded_back_face_hit_mut(&mut self) -> &mut bool;
    fn additional_extrusion_length(&self) -> &f32;
    fn additional_extrusion_length_mut(&mut self) -> &mut f32;
}

impl ShadowExtrusionLevelSettingsTrait for ShadowExtrusionLevelSettings {
    fn dynamic_light_direction(&self) -> &bool {
        &self.dynamic_light_direction
    }
    fn dynamic_light_direction_mut(&mut self) -> &mut bool {
        &mut self.dynamic_light_direction
    }
    fn allow_extrusion_back_face_hit(&self) -> &bool {
        &self.allow_extrusion_back_face_hit
    }
    fn allow_extrusion_back_face_hit_mut(&mut self) -> &mut bool {
        &mut self.allow_extrusion_back_face_hit
    }
    fn allow_full_occluded_back_face_hit(&self) -> &bool {
        &self.allow_full_occluded_back_face_hit
    }
    fn allow_full_occluded_back_face_hit_mut(&mut self) -> &mut bool {
        &mut self.allow_full_occluded_back_face_hit
    }
    fn additional_extrusion_length(&self) -> &f32 {
        &self.additional_extrusion_length
    }
    fn additional_extrusion_length_mut(&mut self) -> &mut f32 {
        &mut self.additional_extrusion_length
    }
}

impl super::entity::SubWorldDataComponentTrait for ShadowExtrusionLevelSettings {
}

impl super::core::DataContainerTrait for ShadowExtrusionLevelSettings {
}

pub static SHADOWEXTRUSIONLEVELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelSettings",
    name_hash: 4044125797,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBWORLDDATACOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionLevelSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionLevelSettings as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionLevelSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DynamicLightDirection",
                name_hash: 450807051,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, dynamic_light_direction),
            },
            FieldInfoData {
                name: "AllowExtrusionBackFaceHit",
                name_hash: 1619593110,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, allow_extrusion_back_face_hit),
            },
            FieldInfoData {
                name: "AllowFullOccludedBackFaceHit",
                name_hash: 4052047139,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, allow_full_occluded_back_face_hit),
            },
            FieldInfoData {
                name: "AdditionalExtrusionLength",
                name_hash: 402302901,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShadowExtrusionLevelSettings, additional_extrusion_length),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLEVELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionLevelSettings {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONLEVELSETTINGS_TYPE_INFO
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


pub static SHADOWEXTRUSIONLEVELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelSettings-Array",
    name_hash: 2537643857,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionLevelSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionLightDirectionEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub light_direction: super::core::Vec3,
}

pub trait ShadowExtrusionLightDirectionEntityDataTrait: super::entity::EntityDataTrait {
    fn light_direction(&self) -> &super::core::Vec3;
    fn light_direction_mut(&mut self) -> &mut super::core::Vec3;
}

impl ShadowExtrusionLightDirectionEntityDataTrait for ShadowExtrusionLightDirectionEntityData {
    fn light_direction(&self) -> &super::core::Vec3 {
        &self.light_direction
    }
    fn light_direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.light_direction
    }
}

impl super::entity::EntityDataTrait for ShadowExtrusionLightDirectionEntityData {
}

impl super::entity::GameObjectDataTrait for ShadowExtrusionLightDirectionEntityData {
}

impl super::core::DataBusPeerTrait for ShadowExtrusionLightDirectionEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ShadowExtrusionLightDirectionEntityData {
}

impl super::core::DataContainerTrait for ShadowExtrusionLightDirectionEntityData {
}

pub static SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntityData",
    name_hash: 2951269126,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionLightDirectionEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionLightDirectionEntityData as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionLightDirectionEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LightDirection",
                name_hash: 1907672094,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ShadowExtrusionLightDirectionEntityData, light_direction),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShadowExtrusionLightDirectionEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_TYPE_INFO
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


pub static SHADOWEXTRUSIONLIGHTDIRECTIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLightDirectionEntityData-Array",
    name_hash: 2342492210,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionLightDirectionEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionLevelDataEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub extrusion_directions: Vec<BoxedTypeObject /* super::core::Vec3 */>,
}

pub trait ShadowExtrusionLevelDataEntityDataTrait: super::entity::EntityDataTrait {
    fn extrusion_directions(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */>;
    fn extrusion_directions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */>;
}

impl ShadowExtrusionLevelDataEntityDataTrait for ShadowExtrusionLevelDataEntityData {
    fn extrusion_directions(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        &self.extrusion_directions
    }
    fn extrusion_directions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        &mut self.extrusion_directions
    }
}

impl super::entity::EntityDataTrait for ShadowExtrusionLevelDataEntityData {
}

impl super::entity::GameObjectDataTrait for ShadowExtrusionLevelDataEntityData {
}

impl super::core::DataBusPeerTrait for ShadowExtrusionLevelDataEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ShadowExtrusionLevelDataEntityData {
}

impl super::core::DataContainerTrait for ShadowExtrusionLevelDataEntityData {
}

pub static SHADOWEXTRUSIONLEVELDATAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntityData",
    name_hash: 3603626363,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionLevelDataEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionLevelDataEntityData as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionLevelDataEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExtrusionDirections",
                name_hash: 299643782,
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(ShadowExtrusionLevelDataEntityData, extrusion_directions),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONLEVELDATAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionLevelDataEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONLEVELDATAENTITYDATA_TYPE_INFO
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


pub static SHADOWEXTRUSIONLEVELDATAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionLevelDataEntityData-Array",
    name_hash: 3944010063,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionLevelDataEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionDataEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub extrusion_data: Option<LockedTypeObject /* ShadowExtrusionAsset */>,
    pub realm: super::core::Realm,
}

pub trait ShadowExtrusionDataEntityDataTrait: super::entity::EntityDataTrait {
    fn extrusion_data(&self) -> &Option<LockedTypeObject /* ShadowExtrusionAsset */>;
    fn extrusion_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ShadowExtrusionAsset */>;
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl ShadowExtrusionDataEntityDataTrait for ShadowExtrusionDataEntityData {
    fn extrusion_data(&self) -> &Option<LockedTypeObject /* ShadowExtrusionAsset */> {
        &self.extrusion_data
    }
    fn extrusion_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ShadowExtrusionAsset */> {
        &mut self.extrusion_data
    }
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl super::entity::EntityDataTrait for ShadowExtrusionDataEntityData {
}

impl super::entity::GameObjectDataTrait for ShadowExtrusionDataEntityData {
}

impl super::core::DataBusPeerTrait for ShadowExtrusionDataEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ShadowExtrusionDataEntityData {
}

impl super::core::DataContainerTrait for ShadowExtrusionDataEntityData {
}

pub static SHADOWEXTRUSIONDATAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntityData",
    name_hash: 1823279341,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionDataEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionDataEntityData as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionDataEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExtrusionData",
                name_hash: 1406519104,
                flags: MemberInfoFlags::new(0),
                field_type: "ShadowExtrusionAsset",
                rust_offset: offset_of!(ShadowExtrusionDataEntityData, extrusion_data),
            },
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ShadowExtrusionDataEntityData, realm),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONDATAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionDataEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONDATAENTITYDATA_TYPE_INFO
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


pub static SHADOWEXTRUSIONDATAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionDataEntityData-Array",
    name_hash: 3240439257,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionDataEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionAsset {
    pub _glacier_base: super::core::Asset,
    pub extrusion_data: Option<LockedTypeObject /* ShadowExtrusionObjectData */>,
}

pub trait ShadowExtrusionAssetTrait: super::core::AssetTrait {
    fn extrusion_data(&self) -> &Option<LockedTypeObject /* ShadowExtrusionObjectData */>;
    fn extrusion_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ShadowExtrusionObjectData */>;
}

impl ShadowExtrusionAssetTrait for ShadowExtrusionAsset {
    fn extrusion_data(&self) -> &Option<LockedTypeObject /* ShadowExtrusionObjectData */> {
        &self.extrusion_data
    }
    fn extrusion_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ShadowExtrusionObjectData */> {
        &mut self.extrusion_data
    }
}

impl super::core::AssetTrait for ShadowExtrusionAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ShadowExtrusionAsset {
}

pub static SHADOWEXTRUSIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionAsset",
    name_hash: 4016093478,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionAsset as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExtrusionData",
                name_hash: 1406519104,
                flags: MemberInfoFlags::new(0),
                field_type: "ShadowExtrusionObjectData",
                rust_offset: offset_of!(ShadowExtrusionAsset, extrusion_data),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionAsset {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONASSET_TYPE_INFO
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


pub static SHADOWEXTRUSIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionAsset-Array",
    name_hash: 4294813074,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionObjectData {
    pub _glacier_base: super::entity::GameObjectData,
    pub object_extrusions: Vec<BoxedTypeObject /* ShadowExtrusionObjectInstance */>,
}

pub trait ShadowExtrusionObjectDataTrait: super::entity::GameObjectDataTrait {
    fn object_extrusions(&self) -> &Vec<BoxedTypeObject /* ShadowExtrusionObjectInstance */>;
    fn object_extrusions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ShadowExtrusionObjectInstance */>;
}

impl ShadowExtrusionObjectDataTrait for ShadowExtrusionObjectData {
    fn object_extrusions(&self) -> &Vec<BoxedTypeObject /* ShadowExtrusionObjectInstance */> {
        &self.object_extrusions
    }
    fn object_extrusions_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ShadowExtrusionObjectInstance */> {
        &mut self.object_extrusions
    }
}

impl super::entity::GameObjectDataTrait for ShadowExtrusionObjectData {
}

impl super::core::DataBusPeerTrait for ShadowExtrusionObjectData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ShadowExtrusionObjectData {
}

impl super::core::DataContainerTrait for ShadowExtrusionObjectData {
}

pub static SHADOWEXTRUSIONOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectData",
    name_hash: 3711769587,
    flags: MemberInfoFlags::new(101),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ShadowExtrusionObjectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionObjectData as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionObjectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ObjectExtrusions",
                name_hash: 508044214,
                flags: MemberInfoFlags::new(144),
                field_type: "ShadowExtrusionObjectInstance-Array",
                rust_offset: offset_of!(ShadowExtrusionObjectData, object_extrusions),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionObjectData {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONOBJECTDATA_TYPE_INFO
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


pub static SHADOWEXTRUSIONOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectData-Array",
    name_hash: 4122441671,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionObjectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShadowExtrusionObjectInstance {
    pub guid: glacier_util::guid::Guid,
    pub extrusion_lengths: Vec<f32>,
}

pub trait ShadowExtrusionObjectInstanceTrait: TypeObject {
    fn guid(&self) -> &glacier_util::guid::Guid;
    fn guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn extrusion_lengths(&self) -> &Vec<f32>;
    fn extrusion_lengths_mut(&mut self) -> &mut Vec<f32>;
}

impl ShadowExtrusionObjectInstanceTrait for ShadowExtrusionObjectInstance {
    fn guid(&self) -> &glacier_util::guid::Guid {
        &self.guid
    }
    fn guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.guid
    }
    fn extrusion_lengths(&self) -> &Vec<f32> {
        &self.extrusion_lengths
    }
    fn extrusion_lengths_mut(&mut self) -> &mut Vec<f32> {
        &mut self.extrusion_lengths
    }
}

pub static SHADOWEXTRUSIONOBJECTINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectInstance",
    name_hash: 1358914666,
    flags: MemberInfoFlags::new(73),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShadowExtrusionObjectInstance as Default>::default())),
            create_boxed: || Box::new(<ShadowExtrusionObjectInstance as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Guid",
                name_hash: 2088724858,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(ShadowExtrusionObjectInstance, guid),
            },
            FieldInfoData {
                name: "ExtrusionLengths",
                name_hash: 208821119,
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(ShadowExtrusionObjectInstance, extrusion_lengths),
            },
        ],
    }),
    array_type: Some(SHADOWEXTRUSIONOBJECTINSTANCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShadowExtrusionObjectInstance {
    fn type_info(&self) -> &'static TypeInfo {
        SHADOWEXTRUSIONOBJECTINSTANCE_TYPE_INFO
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


pub static SHADOWEXTRUSIONOBJECTINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShadowExtrusionObjectInstance-Array",
    name_hash: 489631582,
    flags: MemberInfoFlags::new(145),
    module: "ShadowExtrusionShared",
    data: TypeInfoData::Array("ShadowExtrusionObjectInstance"),
    array_type: None,
    alignment: 8,
};


