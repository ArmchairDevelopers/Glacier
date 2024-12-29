#![allow(dead_code)]
#![allow(unused_imports)]

use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

#[derive(Debug, Default)]
#[repr(C)]
pub struct ContentFlag {
    pub _glacier_base: DataContainer,
}

pub trait ContentFlagTrait: DataContainerTrait {
}

impl ContentFlagTrait for ContentFlag {
}

impl super::core::DataContainerTrait for MeshAsset {
}

pub static MESHASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshAsset",
    name_hash: 15738982,
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render_base::MESHBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(MeshAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshAsset as Default>::default())),
            create_boxed: || Box::new(<MeshAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LodGroup",
                name_hash: 2046326013,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshLodGroup",
                rust_offset: offset_of!(MeshAsset, lod_group),
            }
        ],
    }),
    array_type: Some(MESHASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MeshAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MESHASSET_TYPE_INFO
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


pub static MESHASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshAsset-Array",
    name_hash: 1986217298,
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshAsset"),
    array_type: None,
    alignment: 8,
};

pub fn register_mod_types(registry: &mut TypeRegistry) {
    a_i_tools::register_a_i_tools_types(registry);
}

