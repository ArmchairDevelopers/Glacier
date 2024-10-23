use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct MorphStatic {
    pub _glacier_base: super::core::Asset,
    pub preset_mesh: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait MorphStaticTrait: super::core::AssetTrait {
    fn preset_mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn preset_mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl MorphStaticTrait for MorphStatic {
    fn preset_mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.preset_mesh
    }
    fn preset_mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.preset_mesh
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.resource
    }
}

impl super::core::AssetTrait for MorphStatic {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MorphStatic {
}

pub static MORPHSTATIC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphStatic",
    name_hash: 115899861,
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(MorphStatic, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphStatic as Default>::default())),
            create_boxed: || Box::new(<MorphStatic as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PresetMesh",
                name_hash: 2521412627,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(MorphStatic, preset_mesh),
            },
            FieldInfoData {
                name: "Resource",
                name_hash: 74513935,
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


pub static MORPHSTATIC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphStatic-Array",
    name_hash: 2293743073,
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphStatic"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MorphTargets {
    pub _glacier_base: super::core::Asset,
    pub disable_additive_bone_offsets: bool,
    pub editor_vertical_offset: f32,
    pub preset_meshes: Vec<Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>>,
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait MorphTargetsTrait: super::core::AssetTrait {
    fn disable_additive_bone_offsets(&self) -> &bool;
    fn disable_additive_bone_offsets_mut(&mut self) -> &mut bool;
    fn editor_vertical_offset(&self) -> &f32;
    fn editor_vertical_offset_mut(&mut self) -> &mut f32;
    fn preset_meshes(&self) -> &Vec<Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>>;
    fn preset_meshes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>>;
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl MorphTargetsTrait for MorphTargets {
    fn disable_additive_bone_offsets(&self) -> &bool {
        &self.disable_additive_bone_offsets
    }
    fn disable_additive_bone_offsets_mut(&mut self) -> &mut bool {
        &mut self.disable_additive_bone_offsets
    }
    fn editor_vertical_offset(&self) -> &f32 {
        &self.editor_vertical_offset
    }
    fn editor_vertical_offset_mut(&mut self) -> &mut f32 {
        &mut self.editor_vertical_offset
    }
    fn preset_meshes(&self) -> &Vec<Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>> {
        &self.preset_meshes
    }
    fn preset_meshes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>> {
        &mut self.preset_meshes
    }
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.resource
    }
}

impl super::core::AssetTrait for MorphTargets {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MorphTargets {
}

pub static MORPHTARGETS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargets",
    name_hash: 369362223,
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(MorphTargets, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphTargets as Default>::default())),
            create_boxed: || Box::new(<MorphTargets as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DisableAdditiveBoneOffsets",
                name_hash: 838956175,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MorphTargets, disable_additive_bone_offsets),
            },
            FieldInfoData {
                name: "EditorVerticalOffset",
                name_hash: 3208306011,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MorphTargets, editor_vertical_offset),
            },
            FieldInfoData {
                name: "PresetMeshes",
                name_hash: 1334245541,
                flags: MemberInfoFlags::new(144),
                field_type: "MeshBaseAsset-Array",
                rust_offset: offset_of!(MorphTargets, preset_meshes),
            },
            FieldInfoData {
                name: "Resource",
                name_hash: 74513935,
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


pub static MORPHTARGETS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargets-Array",
    name_hash: 3057685147,
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargets"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MorphTargetsInterfaceInfo {
    pub name_hash: u32,
    pub id: glacier_util::guid::Guid,
}

pub trait MorphTargetsInterfaceInfoTrait: TypeObject {
    fn name_hash(&self) -> &u32;
    fn name_hash_mut(&mut self) -> &mut u32;
    fn id(&self) -> &glacier_util::guid::Guid;
    fn id_mut(&mut self) -> &mut glacier_util::guid::Guid;
}

impl MorphTargetsInterfaceInfoTrait for MorphTargetsInterfaceInfo {
    fn name_hash(&self) -> &u32 {
        &self.name_hash
    }
    fn name_hash_mut(&mut self) -> &mut u32 {
        &mut self.name_hash
    }
    fn id(&self) -> &glacier_util::guid::Guid {
        &self.id
    }
    fn id_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.id
    }
}

pub static MORPHTARGETSINTERFACEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsInterfaceInfo",
    name_hash: 2430919428,
    flags: MemberInfoFlags::new(32841),
    module: "MorphBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphTargetsInterfaceInfo as Default>::default())),
            create_boxed: || Box::new(<MorphTargetsInterfaceInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "NameHash",
                name_hash: 994057744,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MorphTargetsInterfaceInfo, name_hash),
            },
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
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


pub static MORPHTARGETSINTERFACEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsInterfaceInfo-Array",
    name_hash: 362903344,
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
    name_hash: 75235349,
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


pub static MORPHDEBUGRENDEROPTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderOption-Array",
    name_hash: 2193092513,
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
    name_hash: 3931689850,
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


pub static MORPHDEBUGRENDERFLAG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphDebugRenderFlag-Array",
    name_hash: 3845301838,
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphDebugRenderFlag"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MorphTargetsResource {
}

pub trait MorphTargetsResourceTrait: TypeObject {
}

impl MorphTargetsResourceTrait for MorphTargetsResource {
}

pub static MORPHTARGETSRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsResource",
    name_hash: 123200997,
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphTargetsResource as Default>::default())),
            create_boxed: || Box::new(<MorphTargetsResource as Default>::default()),
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


pub static MORPHTARGETSRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphTargetsResource-Array",
    name_hash: 4148326097,
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphTargetsResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MorphResource {
}

pub trait MorphResourceTrait: TypeObject {
}

impl MorphResourceTrait for MorphResource {
}

pub static MORPHRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphResource",
    name_hash: 2893072135,
    flags: MemberInfoFlags::new(101),
    module: "MorphBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MorphResource as Default>::default())),
            create_boxed: || Box::new(<MorphResource as Default>::default()),
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


pub static MORPHRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MorphResource-Array",
    name_hash: 2710284211,
    flags: MemberInfoFlags::new(145),
    module: "MorphBase",
    data: TypeInfoData::Array("MorphResource"),
    array_type: None,
    alignment: 8,
};


