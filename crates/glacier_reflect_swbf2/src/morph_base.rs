use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct MorphStatic {
    pub _glacier_base: super::core::Asset,
    pub preset_mesh: Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait MorphStaticTrait: super::core::AssetTrait {
    fn preset_mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl MorphStaticTrait for MorphStatic {
    fn preset_mesh(&self) -> &Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>> {
        &self.preset_mesh
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
}

impl super::core::AssetTrait for MorphStatic {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for MorphStatic {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MORPHSTATIC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphStatic",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphStatic as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PresetMesh",
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(MorphStatic, preset_mesh),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(MorphStatic, resource),
            },
        ],
    }),
    array_type: Some(MORPHSTATIC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MorphStatic {
    fn type_info(&self) -> &'static TypeInfo {
        MORPHSTATIC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MORPHSTATIC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphStatic-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphStatic"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MorphTargets {
    pub _glacier_base: super::core::Asset,
    pub disable_additive_bone_offsets: bool,
    pub editor_vertical_offset: f32,
    pub preset_meshes: Vec<Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>>,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait MorphTargetsTrait: super::core::AssetTrait {
    fn disable_additive_bone_offsets(&self) -> &bool;
    fn editor_vertical_offset(&self) -> &f32;
    fn preset_meshes(&self) -> &Vec<Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>>;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl MorphTargetsTrait for MorphTargets {
    fn disable_additive_bone_offsets(&self) -> &bool {
        &self.disable_additive_bone_offsets
    }
    fn editor_vertical_offset(&self) -> &f32 {
        &self.editor_vertical_offset
    }
    fn preset_meshes(&self) -> &Vec<Option<Arc<Mutex<dyn super::render_base::MeshBaseAssetTrait>>>> {
        &self.preset_meshes
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
}

impl super::core::AssetTrait for MorphTargets {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for MorphTargets {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MORPHTARGETS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargets",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphTargets as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DisableAdditiveBoneOffsets",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MorphTargets, disable_additive_bone_offsets),
            },
            FieldInfoData {
                name: "EditorVerticalOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MorphTargets, editor_vertical_offset),
            },
            FieldInfoData {
                name: "PresetMeshes",
                flags: MemberInfoFlags::new(144),
                field_type: "MeshBaseAsset-Array",
                rust_offset: offset_of!(MorphTargets, preset_meshes),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(MorphTargets, resource),
            },
        ],
    }),
    array_type: Some(MORPHTARGETS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MorphTargets {
    fn type_info(&self) -> &'static TypeInfo {
        MORPHTARGETS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MORPHTARGETS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargets-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargets"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MorphTargetsInterfaceInfo {
    pub name_hash: u32,
    pub id: glacier_util::guid::Guid,
}

pub trait MorphTargetsInterfaceInfoTrait: TypeObject {
    fn name_hash(&self) -> &u32;
    fn id(&self) -> &glacier_util::guid::Guid;
}

impl MorphTargetsInterfaceInfoTrait for MorphTargetsInterfaceInfo {
    fn name_hash(&self) -> &u32 {
        &self.name_hash
    }
    fn id(&self) -> &glacier_util::guid::Guid {
        &self.id
    }
}

pub static MORPHTARGETSINTERFACEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsInterfaceInfo",
    flags: MemberInfoFlags::new(32841),
    module: "MorphBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphTargetsInterfaceInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "NameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MorphTargetsInterfaceInfo, name_hash),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MorphTargetsInterfaceInfo, id),
            },
        ],
    }),
    array_type: Some(MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MorphTargetsInterfaceInfo {
    fn type_info(&self) -> &'static TypeInfo {
        MORPHTARGETSINTERFACEINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsInterfaceInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargetsInterfaceInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MorphDebugRenderOption {
    #[default]
    MorphDebugRenderOption_BoneWidth = 0,
}

pub static MORPHDEBUGRENDEROPTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderOption",
    flags: MemberInfoFlags::new(49429),
    module: "MorphBase",
    data: TypeInfoData::Enum,
    array_type: Some(MORPHDEBUGRENDEROPTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MorphDebugRenderOption {
    fn type_info(&self) -> &'static TypeInfo {
        MORPHDEBUGRENDEROPTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MORPHDEBUGRENDEROPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderOption-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphDebugRenderOption"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MorphDebugRenderFlag {
    #[default]
    MorphDebugRenderFlag_Bones = 0,
    MorphDebugRenderFlag_BoneNames = 1,
    MorphDebugRenderFlag_BoneHierarchy = 2,
    MorphDebugRenderFlag_BoneCoordinates = 3,
}

pub static MORPHDEBUGRENDERFLAG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderFlag",
    flags: MemberInfoFlags::new(49429),
    module: "MorphBase",
    data: TypeInfoData::Enum,
    array_type: Some(MORPHDEBUGRENDERFLAG_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MorphDebugRenderFlag {
    fn type_info(&self) -> &'static TypeInfo {
        MORPHDEBUGRENDERFLAG_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MORPHDEBUGRENDERFLAG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderFlag-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphDebugRenderFlag"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MorphTargetsResource {
}

pub trait MorphTargetsResourceTrait: TypeObject {
}

impl MorphTargetsResourceTrait for MorphTargetsResource {
}

pub static MORPHTARGETSRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsResource",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphTargetsResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MORPHTARGETSRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MorphTargetsResource {
    fn type_info(&self) -> &'static TypeInfo {
        MORPHTARGETSRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MORPHTARGETSRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargetsResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MorphResource {
}

pub trait MorphResourceTrait: TypeObject {
}

impl MorphResourceTrait for MorphResource {
}

pub static MORPHRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphResource",
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MORPHRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MorphResource {
    fn type_info(&self) -> &'static TypeInfo {
        MORPHRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MORPHRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphResource"),
    array_type: None,
    alignment: 8,
};


