use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct StaticMorphEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub targets: Option<Arc<Mutex<dyn super::morph_base::MorphTargetsTrait>>>,
    pub model_pose: super::core::SparseTransformArray,
}

pub trait StaticMorphEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn targets(&self) -> &Option<Arc<Mutex<dyn super::morph_base::MorphTargetsTrait>>>;
    fn model_pose(&self) -> &super::core::SparseTransformArray;
}

impl StaticMorphEntityDataTrait for StaticMorphEntityData {
    fn targets(&self) -> &Option<Arc<Mutex<dyn super::morph_base::MorphTargetsTrait>>> {
        &self.targets
    }
    fn model_pose(&self) -> &super::core::SparseTransformArray {
        &self.model_pose
    }
}

impl super::entity::SpatialEntityDataTrait for StaticMorphEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
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
}

impl super::core::GameDataContainerTrait for StaticMorphEntityData {
}

impl super::core::DataContainerTrait for StaticMorphEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STATICMORPHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Targets",
                flags: MemberInfoFlags::new(0),
                field_type: "MorphTargets",
                rust_offset: offset_of!(StaticMorphEntityData, targets),
            },
            FieldInfoData {
                name: "ModelPose",
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
}


pub static STATICMORPHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for StaticMorphSaveGameStorageEntityData {
}

impl super::core::DataContainerTrait for StaticMorphSaveGameStorageEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STATICMORPHSAVEGAMESTORAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphSaveGameStorageEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATICMORPHSTORAGEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphSaveGameStorageEntityData as Default>::default())),
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
}


pub static STATICMORPHSAVEGAMESTORAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphSaveGameStorageEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphSaveGameStorageEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for StaticMorphStorageEntityData {
}

impl super::core::DataContainerTrait for StaticMorphStorageEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STATICMORPHSTORAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphStorageEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphStorageEntityData as Default>::default())),
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
}


pub static STATICMORPHSTORAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphStorageEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphStorageEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticMorphBlendEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub presets: Vec<super::morph_base::MorphTargetsInterfaceInfo>,
    pub blends: Vec<super::morph_base::MorphTargetsInterfaceInfo>,
}

pub trait StaticMorphBlendEntityDataTrait: super::entity::EntityDataTrait {
    fn presets(&self) -> &Vec<super::morph_base::MorphTargetsInterfaceInfo>;
    fn blends(&self) -> &Vec<super::morph_base::MorphTargetsInterfaceInfo>;
}

impl StaticMorphBlendEntityDataTrait for StaticMorphBlendEntityData {
    fn presets(&self) -> &Vec<super::morph_base::MorphTargetsInterfaceInfo> {
        &self.presets
    }
    fn blends(&self) -> &Vec<super::morph_base::MorphTargetsInterfaceInfo> {
        &self.blends
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
}

impl super::core::GameDataContainerTrait for StaticMorphBlendEntityData {
}

impl super::core::DataContainerTrait for StaticMorphBlendEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STATICMORPHBLENDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphBlendEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphBlendEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Presets",
                flags: MemberInfoFlags::new(144),
                field_type: "MorphTargetsInterfaceInfo-Array",
                rust_offset: offset_of!(StaticMorphBlendEntityData, presets),
            },
            FieldInfoData {
                name: "Blends",
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
}


pub static STATICMORPHBLENDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphBlendEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphBlendEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StoredStaticMorphComponentData {
    pub _glacier_base: StoredStaticMorphBaseComponentData,
    pub targets_id: glacier_util::guid::Guid,
    pub presets: Vec<StoredStaticMorphPreset>,
}

pub trait StoredStaticMorphComponentDataTrait: StoredStaticMorphBaseComponentDataTrait {
    fn targets_id(&self) -> &glacier_util::guid::Guid;
    fn presets(&self) -> &Vec<StoredStaticMorphPreset>;
}

impl StoredStaticMorphComponentDataTrait for StoredStaticMorphComponentData {
    fn targets_id(&self) -> &glacier_util::guid::Guid {
        &self.targets_id
    }
    fn presets(&self) -> &Vec<StoredStaticMorphPreset> {
        &self.presets
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
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for StoredStaticMorphComponentData {
}

impl super::core::DataBusPeerTrait for StoredStaticMorphComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for StoredStaticMorphComponentData {
}

impl super::core::DataContainerTrait for StoredStaticMorphComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STOREDSTATICMORPHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StoredStaticMorphComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TargetsId",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(StoredStaticMorphComponentData, targets_id),
            },
            FieldInfoData {
                name: "Presets",
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
}


pub static STOREDSTATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StoredStaticMorphPreset {
    pub preset_id: glacier_util::guid::Guid,
    pub preset_mesh: Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>,
}

pub trait StoredStaticMorphPresetTrait: TypeObject {
    fn preset_id(&self) -> &glacier_util::guid::Guid;
    fn preset_mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
}

impl StoredStaticMorphPresetTrait for StoredStaticMorphPreset {
    fn preset_id(&self) -> &glacier_util::guid::Guid {
        &self.preset_id
    }
    fn preset_mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &self.preset_mesh
    }
}

pub static STOREDSTATICMORPHPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphPreset",
    flags: MemberInfoFlags::new(73),
    module: "MorphSim",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StoredStaticMorphPreset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PresetId",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(StoredStaticMorphPreset, preset_id),
            },
            FieldInfoData {
                name: "PresetMesh",
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
}


pub static STOREDSTATICMORPHPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphPreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphPreset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for StoredStaticMorphBaseComponentData {
}

impl super::core::DataBusPeerTrait for StoredStaticMorphBaseComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for StoredStaticMorphBaseComponentData {
}

impl super::core::DataContainerTrait for StoredStaticMorphBaseComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphBaseComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StoredStaticMorphBaseComponentData as Default>::default())),
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
}


pub static STOREDSTATICMORPHBASECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphBaseComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphBaseComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticMorphGeneratorComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub targets: Option<Arc<Mutex<dyn super::morph_base::MorphTargetsTrait>>>,
}

pub trait StaticMorphGeneratorComponentDataTrait: super::entity::GameComponentDataTrait {
    fn targets(&self) -> &Option<Arc<Mutex<dyn super::morph_base::MorphTargetsTrait>>>;
}

impl StaticMorphGeneratorComponentDataTrait for StaticMorphGeneratorComponentData {
    fn targets(&self) -> &Option<Arc<Mutex<dyn super::morph_base::MorphTargetsTrait>>> {
        &self.targets
    }
}

impl super::entity::GameComponentDataTrait for StaticMorphGeneratorComponentData {
}

impl super::entity::ComponentDataTrait for StaticMorphGeneratorComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for StaticMorphGeneratorComponentData {
}

impl super::core::DataBusPeerTrait for StaticMorphGeneratorComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphGeneratorComponentData {
}

impl super::core::DataContainerTrait for StaticMorphGeneratorComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STATICMORPHGENERATORCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphGeneratorComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphGeneratorComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Targets",
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
}


pub static STATICMORPHGENERATORCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphGeneratorComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphGeneratorComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StaticMorphComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub morph: Option<Arc<Mutex<dyn super::morph_base::MorphStaticTrait>>>,
}

pub trait StaticMorphComponentDataTrait: super::entity::GameComponentDataTrait {
    fn morph(&self) -> &Option<Arc<Mutex<dyn super::morph_base::MorphStaticTrait>>>;
}

impl StaticMorphComponentDataTrait for StaticMorphComponentData {
    fn morph(&self) -> &Option<Arc<Mutex<dyn super::morph_base::MorphStaticTrait>>> {
        &self.morph
    }
}

impl super::entity::GameComponentDataTrait for StaticMorphComponentData {
}

impl super::entity::ComponentDataTrait for StaticMorphComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for StaticMorphComponentData {
}

impl super::core::DataBusPeerTrait for StaticMorphComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for StaticMorphComponentData {
}

impl super::core::DataContainerTrait for StaticMorphComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STATICMORPHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticMorphComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Morph",
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
}


pub static STATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerStaticMorphSaveGameStorageEntity as Default>::default())),
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
}


pub static SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticMorphSaveGameStorageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ServerStaticMorphSaveGameStorageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStoredStaticMorphComponent as Default>::default())),
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
}


pub static CLIENTSTOREDSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStoredStaticMorphComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStoredStaticMorphBaseComponent as Default>::default())),
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
}


pub static CLIENTSTOREDSTATICMORPHBASECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphBaseComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStoredStaticMorphBaseComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphStorageEntity as Default>::default())),
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
}


pub static CLIENTSTATICMORPHSTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphStorageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphStorageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphSaveGameStorageEntity as Default>::default())),
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
}


pub static CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphSaveGameStorageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphSaveGameStorageEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphGeneratorComponent as Default>::default())),
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
}


pub static CLIENTSTATICMORPHGENERATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphGeneratorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphGeneratorComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphComponent as Default>::default())),
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
}


pub static CLIENTSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStaticMorphBlendEntity as Default>::default())),
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
}


pub static CLIENTSTATICMORPHBLENDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphBlendEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphBlendEntity"),
    array_type: None,
    alignment: 8,
};


