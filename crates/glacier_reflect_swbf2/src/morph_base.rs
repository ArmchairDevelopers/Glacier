use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_morph_base_types(registry: &mut TypeRegistry) {
    registry.register_type(MORPHSTATIC_TYPE_INFO);
    registry.register_type(MORPHSTATIC_ARRAY_TYPE_INFO);
    registry.register_type(MORPHTARGETS_TYPE_INFO);
    registry.register_type(MORPHTARGETS_ARRAY_TYPE_INFO);
    registry.register_type(MORPHTARGETSINTERFACEINFO_TYPE_INFO);
    registry.register_type(MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO);
    registry.register_type(MORPHDEBUGRENDEROPTION_TYPE_INFO);
    registry.register_type(MORPHDEBUGRENDEROPTION_ARRAY_TYPE_INFO);
    registry.register_type(MORPHDEBUGRENDERFLAG_TYPE_INFO);
    registry.register_type(MORPHDEBUGRENDERFLAG_ARRAY_TYPE_INFO);
    registry.register_type(MORPHTARGETSRESOURCE_TYPE_INFO);
    registry.register_type(MORPHTARGETSRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(MORPHRESOURCE_TYPE_INFO);
    registry.register_type(MORPHRESOURCE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MorphStatic {
    pub preset_mesh: super::render_base::MeshBaseAsset,
    pub resource: super::core::ResourceRef,
}

pub const MORPHSTATIC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphStatic",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PresetMesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MorphStatic, preset_mesh),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(MorphStatic, resource),
            },
        ],
    }),
    array_type: Some(MORPHSTATIC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MorphStatic {
    fn type_info() -> &'static TypeInfo {
        MORPHSTATIC_TYPE_INFO
    }
}


pub const MORPHSTATIC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphStatic-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphStatic-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MorphTargets {
    pub disable_additive_bone_offsets: bool,
    pub editor_vertical_offset: f32,
    pub preset_meshes: Vec<super::render_base::MeshBaseAsset>,
    pub resource: super::core::ResourceRef,
}

pub const MORPHTARGETS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargets",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DisableAdditiveBoneOffsets",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MorphTargets, disable_additive_bone_offsets),
            },
            FieldInfoData {
                name: "EditorVerticalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MorphTargets, editor_vertical_offset),
            },
            FieldInfoData {
                name: "PresetMeshes",
                flags: MemberInfoFlags::new(144),
                field_type: MESHBASEASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MorphTargets, preset_meshes),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(MorphTargets, resource),
            },
        ],
    }),
    array_type: Some(MORPHTARGETS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MorphTargets {
    fn type_info() -> &'static TypeInfo {
        MORPHTARGETS_TYPE_INFO
    }
}


pub const MORPHTARGETS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargets-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargets-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MorphTargetsInterfaceInfo {
    pub name_hash: u32,
    pub id: super::core::Guid,
}

pub const MORPHTARGETSINTERFACEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsInterfaceInfo",
    flags: MemberInfoFlags::new(32841),
    module: "MorphBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MorphTargetsInterfaceInfo, name_hash),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MorphTargetsInterfaceInfo, id),
            },
        ],
    }),
    array_type: Some(MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MorphTargetsInterfaceInfo {
    fn type_info() -> &'static TypeInfo {
        MORPHTARGETSINTERFACEINFO_TYPE_INFO
    }
}


pub const MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsInterfaceInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargetsInterfaceInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MorphDebugRenderOption {
    #[default]
    MorphDebugRenderOption_BoneWidth = 0,
}

pub const MORPHDEBUGRENDEROPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderOption",
    flags: MemberInfoFlags::new(49429),
    module: "MorphBase",
    data: TypeInfoData::Enum,
    array_type: Some(MORPHDEBUGRENDEROPTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MorphDebugRenderOption {
    fn type_info() -> &'static TypeInfo {
        MORPHDEBUGRENDEROPTION_TYPE_INFO
    }
}


pub const MORPHDEBUGRENDEROPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderOption-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphDebugRenderOption-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MorphDebugRenderFlag {
    #[default]
    MorphDebugRenderFlag_Bones = 0,
    MorphDebugRenderFlag_BoneNames = 1,
    MorphDebugRenderFlag_BoneHierarchy = 2,
    MorphDebugRenderFlag_BoneCoordinates = 3,
}

pub const MORPHDEBUGRENDERFLAG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderFlag",
    flags: MemberInfoFlags::new(49429),
    module: "MorphBase",
    data: TypeInfoData::Enum,
    array_type: Some(MORPHDEBUGRENDERFLAG_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MorphDebugRenderFlag {
    fn type_info() -> &'static TypeInfo {
        MORPHDEBUGRENDERFLAG_TYPE_INFO
    }
}


pub const MORPHDEBUGRENDERFLAG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderFlag-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphDebugRenderFlag-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MorphTargetsResource {
}

pub const MORPHTARGETSRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsResource",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(MORPHTARGETSRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MorphTargetsResource {
    fn type_info() -> &'static TypeInfo {
        MORPHTARGETSRESOURCE_TYPE_INFO
    }
}


pub const MORPHTARGETSRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargetsResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MorphResource {
}

pub const MORPHRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphResource",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(MORPHRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MorphResource {
    fn type_info() -> &'static TypeInfo {
        MORPHRESOURCE_TYPE_INFO
    }
}


pub const MORPHRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphResource-Array"),
    array_type: None,
    alignment: 8,
};


