use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Debug)]
pub struct StaticMorphEntityData {
    pub targets: super::morph_base::MorphTargets,
    pub model_pose: super::core::SparseTransformArray,
}

pub const STATICMORPHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Targets",
                flags: MemberInfoFlags::new(0),
                field_type: MORPHTARGETS_TYPE_INFO,
                rust_offset: offset_of!(StaticMorphEntityData, targets),
            },
            FieldInfoData {
                name: "ModelPose",
                flags: MemberInfoFlags::new(0),
                field_type: SPARSETRANSFORMARRAY_TYPE_INFO,
                rust_offset: offset_of!(StaticMorphEntityData, model_pose),
            },
        ],
    }),
    array_type: Some(STATICMORPHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StaticMorphEntityData {
    fn type_info() -> &'static TypeInfo {
        STATICMORPHENTITYDATA_TYPE_INFO
    }
}


pub const STATICMORPHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticMorphSaveGameStorageEntityData {
}

pub const STATICMORPHSAVEGAMESTORAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphSaveGameStorageEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATICMORPHSTORAGEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATICMORPHSAVEGAMESTORAGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticMorphSaveGameStorageEntityData {
    fn type_info() -> &'static TypeInfo {
        STATICMORPHSAVEGAMESTORAGEENTITYDATA_TYPE_INFO
    }
}


pub const STATICMORPHSAVEGAMESTORAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphSaveGameStorageEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphSaveGameStorageEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticMorphStorageEntityData {
}

pub const STATICMORPHSTORAGEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphStorageEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATICMORPHSTORAGEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticMorphStorageEntityData {
    fn type_info() -> &'static TypeInfo {
        STATICMORPHSTORAGEENTITYDATA_TYPE_INFO
    }
}


pub const STATICMORPHSTORAGEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphStorageEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphStorageEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticMorphBlendEntityData {
    pub presets: Vec<super::morph_base::MorphTargetsInterfaceInfo>,
    pub blends: Vec<super::morph_base::MorphTargetsInterfaceInfo>,
}

pub const STATICMORPHBLENDENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphBlendEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Presets",
                flags: MemberInfoFlags::new(144),
                field_type: MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StaticMorphBlendEntityData, presets),
            },
            FieldInfoData {
                name: "Blends",
                flags: MemberInfoFlags::new(144),
                field_type: MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StaticMorphBlendEntityData, blends),
            },
        ],
    }),
    array_type: Some(STATICMORPHBLENDENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticMorphBlendEntityData {
    fn type_info() -> &'static TypeInfo {
        STATICMORPHBLENDENTITYDATA_TYPE_INFO
    }
}


pub const STATICMORPHBLENDENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphBlendEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphBlendEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StoredStaticMorphComponentData {
    pub targets_id: super::core::Guid,
    pub presets: Vec<StoredStaticMorphPreset>,
}

pub const STOREDSTATICMORPHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TargetsId",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(StoredStaticMorphComponentData, targets_id),
            },
            FieldInfoData {
                name: "Presets",
                flags: MemberInfoFlags::new(144),
                field_type: STOREDSTATICMORPHPRESET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StoredStaticMorphComponentData, presets),
            },
        ],
    }),
    array_type: Some(STOREDSTATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StoredStaticMorphComponentData {
    fn type_info() -> &'static TypeInfo {
        STOREDSTATICMORPHCOMPONENTDATA_TYPE_INFO
    }
}


pub const STOREDSTATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StoredStaticMorphPreset {
    pub preset_id: super::core::Guid,
    pub preset_mesh: super::render_base::MeshBaseAsset,
}

pub const STOREDSTATICMORPHPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphPreset",
    flags: MemberInfoFlags::new(73),
    module: "MorphSim",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PresetId",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(StoredStaticMorphPreset, preset_id),
            },
            FieldInfoData {
                name: "PresetMesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(StoredStaticMorphPreset, preset_mesh),
            },
        ],
    }),
    array_type: Some(STOREDSTATICMORPHPRESET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StoredStaticMorphPreset {
    fn type_info() -> &'static TypeInfo {
        STOREDSTATICMORPHPRESET_TYPE_INFO
    }
}


pub const STOREDSTATICMORPHPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphPreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphPreset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StoredStaticMorphBaseComponentData {
}

pub const STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphBaseComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STOREDSTATICMORPHBASECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StoredStaticMorphBaseComponentData {
    fn type_info() -> &'static TypeInfo {
        STOREDSTATICMORPHBASECOMPONENTDATA_TYPE_INFO
    }
}


pub const STOREDSTATICMORPHBASECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StoredStaticMorphBaseComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StoredStaticMorphBaseComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StaticMorphGeneratorComponentData {
    pub targets: super::morph_base::MorphTargets,
}

pub const STATICMORPHGENERATORCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphGeneratorComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Targets",
                flags: MemberInfoFlags::new(0),
                field_type: MORPHTARGETS_TYPE_INFO,
                rust_offset: offset_of!(StaticMorphGeneratorComponentData, targets),
            },
        ],
    }),
    array_type: Some(STATICMORPHGENERATORCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StaticMorphGeneratorComponentData {
    fn type_info() -> &'static TypeInfo {
        STATICMORPHGENERATORCOMPONENTDATA_TYPE_INFO
    }
}


pub const STATICMORPHGENERATORCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphGeneratorComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphGeneratorComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StaticMorphComponentData {
    pub morph: super::morph_base::MorphStatic,
}

pub const STATICMORPHCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphComponentData",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Morph",
                flags: MemberInfoFlags::new(0),
                field_type: MORPHSTATIC_TYPE_INFO,
                rust_offset: offset_of!(StaticMorphComponentData, morph),
            },
        ],
    }),
    array_type: Some(STATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StaticMorphComponentData {
    fn type_info() -> &'static TypeInfo {
        STATICMORPHCOMPONENTDATA_TYPE_INFO
    }
}


pub const STATICMORPHCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticMorphComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("StaticMorphComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerStaticMorphSaveGameStorageEntity {
}

pub const SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticMorphSaveGameStorageEntity",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerStaticMorphSaveGameStorageEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO
    }
}


pub const SERVERSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerStaticMorphSaveGameStorageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ServerStaticMorphSaveGameStorageEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStoredStaticMorphComponent {
}

pub const CLIENTSTOREDSTATICMORPHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphComponent",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTOREDSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStoredStaticMorphComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTOREDSTATICMORPHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTOREDSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStoredStaticMorphComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStoredStaticMorphBaseComponent {
}

pub const CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphBaseComponent",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTOREDSTATICMORPHBASECOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStoredStaticMorphBaseComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTOREDSTATICMORPHBASECOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTOREDSTATICMORPHBASECOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStoredStaticMorphBaseComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStoredStaticMorphBaseComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticMorphStorageEntity {
}

pub const CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphStorageEntity",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHSTORAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphStorageEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO
    }
}


pub const CLIENTSTATICMORPHSTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphStorageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphStorageEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticMorphSaveGameStorageEntity {
}

pub const CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphSaveGameStorageEntity",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTSTATICMORPHSTORAGEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphSaveGameStorageEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_TYPE_INFO
    }
}


pub const CLIENTSTATICMORPHSAVEGAMESTORAGEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphSaveGameStorageEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphSaveGameStorageEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticMorphGeneratorComponent {
}

pub const CLIENTSTATICMORPHGENERATORCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphGeneratorComponent",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHGENERATORCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphGeneratorComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMORPHGENERATORCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTATICMORPHGENERATORCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphGeneratorComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphGeneratorComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticMorphComponent {
}

pub const CLIENTSTATICMORPHCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphComponent",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMORPHCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTSTATICMORPHCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStaticMorphBlendEntity {
}

pub const CLIENTSTATICMORPHBLENDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphBlendEntity",
    flags: MemberInfoFlags::new(101),
    module: "MorphSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATICMORPHBLENDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStaticMorphBlendEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATICMORPHBLENDENTITY_TYPE_INFO
    }
}


pub const CLIENTSTATICMORPHBLENDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStaticMorphBlendEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphSim",
    data: TypeInfoData::Array("ClientStaticMorphBlendEntity-Array"),
    array_type: None,
    alignment: 8,
};


