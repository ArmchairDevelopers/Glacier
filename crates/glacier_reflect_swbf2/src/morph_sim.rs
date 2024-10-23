use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_morph_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(STATICMORPHENTITYDATA_TYPE_INFO);
    registry.register_type(STATICMORPHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATICMORPHSAVEGAMESTORAGEENTITYDATA_TYPE_INFO);
    registry.register_type(STATICMORPHSAVEGAMESTORAGEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATICMORPHSTORAGEENTITYDATA_TYPE_INFO);
    registry.register_type(STATICMORPHSTORAGEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATICMORPHBLENDENTITYDATA_TYPE_INFO);
    registry.register_type(STATICMORPHBLENDENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STOREDSTATICMORPHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(STOREDSTATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(STOREDSTATICMORPHPRESET_TYPE_INFO);
    registry.register_type(STOREDSTATICMORPHPRESET_ARRAY_TYPE_INFO);
    registry.register_type(STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO);
    registry.register_type(STOREDSTATICMORPHBASECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATICMORPHGENERATORCOMPONENTDATA_TYPE_INFO);
    registry.register_type(STATICMORPHGENERATORCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATICMORPHCOMPONENTDATA_TYPE_INFO);
    registry.register_type(STATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO);
    registry.register_type(SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTOREDSTATICMORPHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTOREDSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTOREDSTATICMORPHBASECOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHSTORAGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHGENERATORCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHGENERATORCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHBLENDENTITY_TYPE_INFO);
    registry.register_type(CLIENTSTATICMORPHBLENDENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StaticMorphEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub targets: Option<LockedTypeObject /* super::morph_base::MorphTargets */>,
    pub model_pose: super::core::SparseTransformArray,
}

pub trait StaticMorphEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn targets(&self) -> &Option<LockedTypeObject /* super::morph_base::MorphTargets */>;
    fn targets_mut(&mut self) -> &mut Option<LockedTypeObject /* super::morph_base::MorphTargets */>;
    fn model_pose(&self) -> &super::core::SparseTransformArray;
    fn model_pose_mut(&mut self) -> &mut super::core::SparseTransformArray;
}

impl StaticMorphEntityDataTrait for StaticMorphEntityData {
    fn targets(&self) -> &Option<LockedTypeObject /* super::morph_base::MorphTargets */> {
        &self.targets
    }
    fn targets_mut(&mut self) -> &mut Option<LockedTypeObject /* super::morph_base::MorphTargets */> {
        &mut self.targets
    }
    fn model_pose(&self) -> &super::core::SparseTransformArray {
        &self.model_pose
    }
    fn model_pose_mut(&mut self) -> &mut super::core::SparseTransformArray {
        &mut self.model_pose
    }
}

impl super::entity::SpatialEntityDataTrait for StaticMorphEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for StaticMorphEntityData {
}

impl super::entity::GameObjectDataTrait for StaticMorphEntityData {
}

impl super::core::DataBusPeerTrait for StaticMorphEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphEntityData {
}

impl super::core::DataContainerTrait for StaticMorphEntityData {
}

pub static STATICMORPHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphEntityData",
    name_hash: 1257304574,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(StaticMorphEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphEntityData as Default>::default())),
            create_boxed: || Box::new(<StaticMorphEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Targets",
                name_hash: 3016537383,
                flags: MemberInfoFlags::new(0),
                field_type: "MorphTargets",
                rust_offset: offset_of!(StaticMorphEntityData, targets),
            },
            FieldInfoData {
                name: "ModelPose",
                name_hash: 4243999587,
                flags: MemberInfoFlags::new(0),
                field_type: "SparseTransformArray",
                rust_offset: offset_of!(StaticMorphEntityData, model_pose),
            },
        ],
    }),
    array_type: Some(STATICMORPHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StaticMorphEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        STATICMORPHENTITYDATA_TYPE_INFO
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


pub static STATICMORPHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphEntityData-Array",
    name_hash: 1229411018,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StaticMorphSaveGameStorageEntityData {
    pub _glacier_base: StaticMorphStorageEntityData,
}

pub trait StaticMorphSaveGameStorageEntityDataTrait: StaticMorphStorageEntityDataTrait {
}

impl StaticMorphSaveGameStorageEntityDataTrait for StaticMorphSaveGameStorageEntityData {
}

impl StaticMorphStorageEntityDataTrait for StaticMorphSaveGameStorageEntityData {
}

impl super::entity::EntityDataTrait for StaticMorphSaveGameStorageEntityData {
}

impl super::entity::GameObjectDataTrait for StaticMorphSaveGameStorageEntityData {
}

impl super::core::DataBusPeerTrait for StaticMorphSaveGameStorageEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphSaveGameStorageEntityData {
}

impl super::core::DataContainerTrait for StaticMorphSaveGameStorageEntityData {
}

pub static STATICMORPHSAVEGAMESTORAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphSaveGameStorageEntityData",
    name_hash: 1242618280,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATICMORPHSTORAGEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(StaticMorphSaveGameStorageEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphSaveGameStorageEntityData as Default>::default())),
            create_boxed: || Box::new(<StaticMorphSaveGameStorageEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATICMORPHSAVEGAMESTORAGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticMorphSaveGameStorageEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        STATICMORPHSAVEGAMESTORAGEENTITYDATA_TYPE_INFO
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


pub static STATICMORPHSAVEGAMESTORAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphSaveGameStorageEntityData-Array",
    name_hash: 3932904220,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphSaveGameStorageEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StaticMorphStorageEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait StaticMorphStorageEntityDataTrait: super::entity::EntityDataTrait {
}

impl StaticMorphStorageEntityDataTrait for StaticMorphStorageEntityData {
}

impl super::entity::EntityDataTrait for StaticMorphStorageEntityData {
}

impl super::entity::GameObjectDataTrait for StaticMorphStorageEntityData {
}

impl super::core::DataBusPeerTrait for StaticMorphStorageEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphStorageEntityData {
}

impl super::core::DataContainerTrait for StaticMorphStorageEntityData {
}

pub static STATICMORPHSTORAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphStorageEntityData",
    name_hash: 308659943,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(StaticMorphStorageEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphStorageEntityData as Default>::default())),
            create_boxed: || Box::new(<StaticMorphStorageEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATICMORPHSTORAGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticMorphStorageEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        STATICMORPHSTORAGEENTITYDATA_TYPE_INFO
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


pub static STATICMORPHSTORAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphStorageEntityData-Array",
    name_hash: 3777054931,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphStorageEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StaticMorphBlendEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub presets: Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */>,
    pub blends: Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */>,
}

pub trait StaticMorphBlendEntityDataTrait: super::entity::EntityDataTrait {
    fn presets(&self) -> &Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */>;
    fn presets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */>;
    fn blends(&self) -> &Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */>;
    fn blends_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */>;
}

impl StaticMorphBlendEntityDataTrait for StaticMorphBlendEntityData {
    fn presets(&self) -> &Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */> {
        &self.presets
    }
    fn presets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */> {
        &mut self.presets
    }
    fn blends(&self) -> &Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */> {
        &self.blends
    }
    fn blends_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::morph_base::MorphTargetsInterfaceInfo */> {
        &mut self.blends
    }
}

impl super::entity::EntityDataTrait for StaticMorphBlendEntityData {
}

impl super::entity::GameObjectDataTrait for StaticMorphBlendEntityData {
}

impl super::core::DataBusPeerTrait for StaticMorphBlendEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphBlendEntityData {
}

impl super::core::DataContainerTrait for StaticMorphBlendEntityData {
}

pub static STATICMORPHBLENDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphBlendEntityData",
    name_hash: 559656799,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(StaticMorphBlendEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphBlendEntityData as Default>::default())),
            create_boxed: || Box::new(<StaticMorphBlendEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Presets",
                name_hash: 3463460435,
                flags: MemberInfoFlags::new(144),
                field_type: "MorphTargetsInterfaceInfo-Array",
                rust_offset: offset_of!(StaticMorphBlendEntityData, presets),
            },
            FieldInfoData {
                name: "Blends",
                name_hash: 2685362903,
                flags: MemberInfoFlags::new(144),
                field_type: "MorphTargetsInterfaceInfo-Array",
                rust_offset: offset_of!(StaticMorphBlendEntityData, blends),
            },
        ],
    }),
    array_type: Some(STATICMORPHBLENDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticMorphBlendEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        STATICMORPHBLENDENTITYDATA_TYPE_INFO
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


pub static STATICMORPHBLENDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphBlendEntityData-Array",
    name_hash: 2891293803,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphBlendEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StoredStaticMorphComponentData {
    pub _glacier_base: StoredStaticMorphBaseComponentData,
    pub targets_id: glacier_util::guid::Guid,
    pub presets: Vec<BoxedTypeObject /* StoredStaticMorphPreset */>,
}

pub trait StoredStaticMorphComponentDataTrait: StoredStaticMorphBaseComponentDataTrait {
    fn targets_id(&self) -> &glacier_util::guid::Guid;
    fn targets_id_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn presets(&self) -> &Vec<BoxedTypeObject /* StoredStaticMorphPreset */>;
    fn presets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* StoredStaticMorphPreset */>;
}

impl StoredStaticMorphComponentDataTrait for StoredStaticMorphComponentData {
    fn targets_id(&self) -> &glacier_util::guid::Guid {
        &self.targets_id
    }
    fn targets_id_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.targets_id
    }
    fn presets(&self) -> &Vec<BoxedTypeObject /* StoredStaticMorphPreset */> {
        &self.presets
    }
    fn presets_mut(&mut self) -> &mut Vec<BoxedTypeObject /* StoredStaticMorphPreset */> {
        &mut self.presets
    }
}

impl StoredStaticMorphBaseComponentDataTrait for StoredStaticMorphComponentData {
}

impl super::entity::GameComponentDataTrait for StoredStaticMorphComponentData {
}

impl super::entity::ComponentDataTrait for StoredStaticMorphComponentData {
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

impl super::entity::GameObjectDataTrait for StoredStaticMorphComponentData {
}

impl super::core::DataBusPeerTrait for StoredStaticMorphComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StoredStaticMorphComponentData {
}

impl super::core::DataContainerTrait for StoredStaticMorphComponentData {
}

pub static STOREDSTATICMORPHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphComponentData",
    name_hash: 180759793,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(StoredStaticMorphComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StoredStaticMorphComponentData as Default>::default())),
            create_boxed: || Box::new(<StoredStaticMorphComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TargetsId",
                name_hash: 3654198378,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(StoredStaticMorphComponentData, targets_id),
            },
            FieldInfoData {
                name: "Presets",
                name_hash: 3463460435,
                flags: MemberInfoFlags::new(144),
                field_type: "StoredStaticMorphPreset-Array",
                rust_offset: offset_of!(StoredStaticMorphComponentData, presets),
            },
        ],
    }),
    array_type: Some(STOREDSTATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StoredStaticMorphComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        STOREDSTATICMORPHCOMPONENTDATA_TYPE_INFO
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


pub static STOREDSTATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphComponentData-Array",
    name_hash: 1784483781,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StoredStaticMorphPreset {
    pub preset_id: glacier_util::guid::Guid,
    pub preset_mesh: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
}

pub trait StoredStaticMorphPresetTrait: TypeObject {
    fn preset_id(&self) -> &glacier_util::guid::Guid;
    fn preset_id_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn preset_mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn preset_mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
}

impl StoredStaticMorphPresetTrait for StoredStaticMorphPreset {
    fn preset_id(&self) -> &glacier_util::guid::Guid {
        &self.preset_id
    }
    fn preset_id_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.preset_id
    }
    fn preset_mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.preset_mesh
    }
    fn preset_mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.preset_mesh
    }
}

pub static STOREDSTATICMORPHPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphPreset",
    name_hash: 372096203,
    flags: MemberInfoFlags::new(73),
    module: "MorphSim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StoredStaticMorphPreset as Default>::default())),
            create_boxed: || Box::new(<StoredStaticMorphPreset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PresetId",
                name_hash: 2625045485,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(StoredStaticMorphPreset, preset_id),
            },
            FieldInfoData {
                name: "PresetMesh",
                name_hash: 2521412627,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(StoredStaticMorphPreset, preset_mesh),
            },
        ],
    }),
    array_type: Some(STOREDSTATICMORPHPRESET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StoredStaticMorphPreset {
    fn type_info(&self) -> &'static TypeInfo {
        STOREDSTATICMORPHPRESET_TYPE_INFO
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


pub static STOREDSTATICMORPHPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphPreset-Array",
    name_hash: 3087621119,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphPreset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StoredStaticMorphBaseComponentData {
    pub _glacier_base: super::entity::GameComponentData,
}

pub trait StoredStaticMorphBaseComponentDataTrait: super::entity::GameComponentDataTrait {
}

impl StoredStaticMorphBaseComponentDataTrait for StoredStaticMorphBaseComponentData {
}

impl super::entity::GameComponentDataTrait for StoredStaticMorphBaseComponentData {
}

impl super::entity::ComponentDataTrait for StoredStaticMorphBaseComponentData {
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

impl super::entity::GameObjectDataTrait for StoredStaticMorphBaseComponentData {
}

impl super::core::DataBusPeerTrait for StoredStaticMorphBaseComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StoredStaticMorphBaseComponentData {
}

impl super::core::DataContainerTrait for StoredStaticMorphBaseComponentData {
}

pub static STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphBaseComponentData",
    name_hash: 2263396036,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(StoredStaticMorphBaseComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StoredStaticMorphBaseComponentData as Default>::default())),
            create_boxed: || Box::new(<StoredStaticMorphBaseComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(STOREDSTATICMORPHBASECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StoredStaticMorphBaseComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO
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


pub static STOREDSTATICMORPHBASECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphBaseComponentData-Array",
    name_hash: 1954747248,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphBaseComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StaticMorphGeneratorComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub targets: Option<LockedTypeObject /* super::morph_base::MorphTargets */>,
}

pub trait StaticMorphGeneratorComponentDataTrait: super::entity::GameComponentDataTrait {
    fn targets(&self) -> &Option<LockedTypeObject /* super::morph_base::MorphTargets */>;
    fn targets_mut(&mut self) -> &mut Option<LockedTypeObject /* super::morph_base::MorphTargets */>;
}

impl StaticMorphGeneratorComponentDataTrait for StaticMorphGeneratorComponentData {
    fn targets(&self) -> &Option<LockedTypeObject /* super::morph_base::MorphTargets */> {
        &self.targets
    }
    fn targets_mut(&mut self) -> &mut Option<LockedTypeObject /* super::morph_base::MorphTargets */> {
        &mut self.targets
    }
}

impl super::entity::GameComponentDataTrait for StaticMorphGeneratorComponentData {
}

impl super::entity::ComponentDataTrait for StaticMorphGeneratorComponentData {
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

impl super::entity::GameObjectDataTrait for StaticMorphGeneratorComponentData {
}

impl super::core::DataBusPeerTrait for StaticMorphGeneratorComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphGeneratorComponentData {
}

impl super::core::DataContainerTrait for StaticMorphGeneratorComponentData {
}

pub static STATICMORPHGENERATORCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphGeneratorComponentData",
    name_hash: 508803769,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(StaticMorphGeneratorComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphGeneratorComponentData as Default>::default())),
            create_boxed: || Box::new(<StaticMorphGeneratorComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Targets",
                name_hash: 3016537383,
                flags: MemberInfoFlags::new(0),
                field_type: "MorphTargets",
                rust_offset: offset_of!(StaticMorphGeneratorComponentData, targets),
            },
        ],
    }),
    array_type: Some(STATICMORPHGENERATORCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StaticMorphGeneratorComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        STATICMORPHGENERATORCOMPONENTDATA_TYPE_INFO
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


pub static STATICMORPHGENERATORCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphGeneratorComponentData-Array",
    name_hash: 4227664141,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphGeneratorComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StaticMorphComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub morph: Option<LockedTypeObject /* super::morph_base::MorphStatic */>,
}

pub trait StaticMorphComponentDataTrait: super::entity::GameComponentDataTrait {
    fn morph(&self) -> &Option<LockedTypeObject /* super::morph_base::MorphStatic */>;
    fn morph_mut(&mut self) -> &mut Option<LockedTypeObject /* super::morph_base::MorphStatic */>;
}

impl StaticMorphComponentDataTrait for StaticMorphComponentData {
    fn morph(&self) -> &Option<LockedTypeObject /* super::morph_base::MorphStatic */> {
        &self.morph
    }
    fn morph_mut(&mut self) -> &mut Option<LockedTypeObject /* super::morph_base::MorphStatic */> {
        &mut self.morph
    }
}

impl super::entity::GameComponentDataTrait for StaticMorphComponentData {
}

impl super::entity::ComponentDataTrait for StaticMorphComponentData {
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

impl super::entity::GameObjectDataTrait for StaticMorphComponentData {
}

impl super::core::DataBusPeerTrait for StaticMorphComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphComponentData {
}

impl super::core::DataContainerTrait for StaticMorphComponentData {
}

pub static STATICMORPHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphComponentData",
    name_hash: 1464191850,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(StaticMorphComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphComponentData as Default>::default())),
            create_boxed: || Box::new(<StaticMorphComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Morph",
                name_hash: 210034189,
                flags: MemberInfoFlags::new(0),
                field_type: "MorphStatic",
                rust_offset: offset_of!(StaticMorphComponentData, morph),
            },
        ],
    }),
    array_type: Some(STATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StaticMorphComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        STATICMORPHCOMPONENTDATA_TYPE_INFO
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


pub static STATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphComponentData-Array",
    name_hash: 3999716958,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerStaticMorphSaveGameStorageEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerStaticMorphSaveGameStorageEntityTrait: super::entity::EntityTrait {
}

impl ServerStaticMorphSaveGameStorageEntityTrait for ServerStaticMorphSaveGameStorageEntity {
}

impl super::entity::EntityTrait for ServerStaticMorphSaveGameStorageEntity {
}

impl super::entity::EntityBusPeerTrait for ServerStaticMorphSaveGameStorageEntity {
}

pub static SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticMorphSaveGameStorageEntity",
    name_hash: 3936986237,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerStaticMorphSaveGameStorageEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticMorphSaveGameStorageEntity as Default>::default())),
            create_boxed: || Box::new(<ServerStaticMorphSaveGameStorageEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticMorphSaveGameStorageEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO
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


pub static SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticMorphSaveGameStorageEntity-Array",
    name_hash: 2610639177,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ServerStaticMorphSaveGameStorageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientStoredStaticMorphComponent {
    pub _glacier_base: ClientStoredStaticMorphBaseComponent,
}

pub trait ClientStoredStaticMorphComponentTrait: ClientStoredStaticMorphBaseComponentTrait {
}

impl ClientStoredStaticMorphComponentTrait for ClientStoredStaticMorphComponent {
}

impl ClientStoredStaticMorphBaseComponentTrait for ClientStoredStaticMorphComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientStoredStaticMorphComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientStoredStaticMorphComponent {
}

impl super::entity::ComponentTrait for ClientStoredStaticMorphComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStoredStaticMorphComponent {
}

pub static CLIENTSTOREDSTATICMORPHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphComponent",
    name_hash: 1277303096,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientStoredStaticMorphComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStoredStaticMorphComponent as Default>::default())),
            create_boxed: || Box::new(<ClientStoredStaticMorphComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTOREDSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStoredStaticMorphComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTOREDSTATICMORPHCOMPONENT_TYPE_INFO
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


pub static CLIENTSTOREDSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphComponent-Array",
    name_hash: 1457586828,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStoredStaticMorphComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientStoredStaticMorphBaseComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientStoredStaticMorphBaseComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientStoredStaticMorphBaseComponentTrait for ClientStoredStaticMorphBaseComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientStoredStaticMorphBaseComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientStoredStaticMorphBaseComponent {
}

impl super::entity::ComponentTrait for ClientStoredStaticMorphBaseComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStoredStaticMorphBaseComponent {
}

pub static CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphBaseComponent",
    name_hash: 3308012365,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientStoredStaticMorphBaseComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStoredStaticMorphBaseComponent as Default>::default())),
            create_boxed: || Box::new(<ClientStoredStaticMorphBaseComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTOREDSTATICMORPHBASECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStoredStaticMorphBaseComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO
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


pub static CLIENTSTOREDSTATICMORPHBASECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphBaseComponent-Array",
    name_hash: 772159353,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStoredStaticMorphBaseComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientStaticMorphStorageEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientStaticMorphStorageEntityTrait: super::entity::EntityTrait {
}

impl ClientStaticMorphStorageEntityTrait for ClientStaticMorphStorageEntity {
}

impl super::entity::EntityTrait for ClientStaticMorphStorageEntity {
}

impl super::entity::EntityBusPeerTrait for ClientStaticMorphStorageEntity {
}

pub static CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphStorageEntity",
    name_hash: 3318932782,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientStaticMorphStorageEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphStorageEntity as Default>::default())),
            create_boxed: || Box::new(<ClientStaticMorphStorageEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHSTORAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphStorageEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO
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


pub static CLIENTSTATICMORPHSTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphStorageEntity-Array",
    name_hash: 2833264026,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphStorageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientStaticMorphSaveGameStorageEntity {
    pub _glacier_base: ClientStaticMorphStorageEntity,
}

pub trait ClientStaticMorphSaveGameStorageEntityTrait: ClientStaticMorphStorageEntityTrait {
}

impl ClientStaticMorphSaveGameStorageEntityTrait for ClientStaticMorphSaveGameStorageEntity {
}

impl ClientStaticMorphStorageEntityTrait for ClientStaticMorphSaveGameStorageEntity {
}

impl super::entity::EntityTrait for ClientStaticMorphSaveGameStorageEntity {
}

impl super::entity::EntityBusPeerTrait for ClientStaticMorphSaveGameStorageEntity {
}

pub static CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphSaveGameStorageEntity",
    name_hash: 1106209313,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientStaticMorphSaveGameStorageEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphSaveGameStorageEntity as Default>::default())),
            create_boxed: || Box::new(<ClientStaticMorphSaveGameStorageEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphSaveGameStorageEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO
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


pub static CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphSaveGameStorageEntity-Array",
    name_hash: 2725000341,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphSaveGameStorageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientStaticMorphGeneratorComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientStaticMorphGeneratorComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientStaticMorphGeneratorComponentTrait for ClientStaticMorphGeneratorComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientStaticMorphGeneratorComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientStaticMorphGeneratorComponent {
}

impl super::entity::ComponentTrait for ClientStaticMorphGeneratorComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStaticMorphGeneratorComponent {
}

pub static CLIENTSTATICMORPHGENERATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphGeneratorComponent",
    name_hash: 3151020496,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientStaticMorphGeneratorComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphGeneratorComponent as Default>::default())),
            create_boxed: || Box::new(<ClientStaticMorphGeneratorComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHGENERATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphGeneratorComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMORPHGENERATORCOMPONENT_TYPE_INFO
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


pub static CLIENTSTATICMORPHGENERATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphGeneratorComponent-Array",
    name_hash: 4164976740,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphGeneratorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientStaticMorphComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientStaticMorphComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientStaticMorphComponentTrait for ClientStaticMorphComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientStaticMorphComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientStaticMorphComponent {
}

impl super::entity::ComponentTrait for ClientStaticMorphComponent {
}

impl super::entity::EntityBusPeerTrait for ClientStaticMorphComponent {
}

pub static CLIENTSTATICMORPHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphComponent",
    name_hash: 3339726371,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientStaticMorphComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphComponent as Default>::default())),
            create_boxed: || Box::new(<ClientStaticMorphComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMORPHCOMPONENT_TYPE_INFO
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


pub static CLIENTSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphComponent-Array",
    name_hash: 2957537687,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientStaticMorphBlendEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientStaticMorphBlendEntityTrait: super::entity::EntityTrait {
}

impl ClientStaticMorphBlendEntityTrait for ClientStaticMorphBlendEntity {
}

impl super::entity::EntityTrait for ClientStaticMorphBlendEntity {
}

impl super::entity::EntityBusPeerTrait for ClientStaticMorphBlendEntity {
}

pub static CLIENTSTATICMORPHBLENDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphBlendEntity",
    name_hash: 2112276758,
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientStaticMorphBlendEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphBlendEntity as Default>::default())),
            create_boxed: || Box::new(<ClientStaticMorphBlendEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHBLENDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphBlendEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATICMORPHBLENDENTITY_TYPE_INFO
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


pub static CLIENTSTATICMORPHBLENDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphBlendEntity-Array",
    name_hash: 993451554,
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphBlendEntity"),
    array_type: None,
    alignment: 8,
};


