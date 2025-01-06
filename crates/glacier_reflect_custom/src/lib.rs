#![allow(dead_code)]
#![allow(unused_imports)]

use glacier_reflect_swbf2::{core::{Asset, DataContainer, DataContainerTrait, ASSET_TYPE_INFO}, register_mod_types};
use glacier_util::hash::hash_quick_str_const;
use std::{any::Any, mem::offset_of, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        BoxedTypeObject, ClassInfoData, FieldInfoData, LockedTypeObject, TypeFunctions, TypeInfo,
        TypeInfoData, TypeObject, ValueTypeInfoData,
    },
    type_registry::TypeRegistry,
};

#[derive(Debug, Default)]
#[repr(C)]
pub struct ResourceBundleAsset {
    pub _glacier_base: Asset,
    pub assets: Vec<Option<LockedTypeObject /* Asset */>>,
}

impl DataContainerTrait for ResourceBundleAsset {}

pub static RESOURCEBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceBundleAsset",
    name_hash: hash_quick_str_const("ResourceBundleAsset"),
    flags: MemberInfoFlags::new(101),
    module: "Glacier",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ResourceBundleAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ResourceBundleAsset as Default>::default())),
            create_boxed: || Box::new(<ResourceBundleAsset as Default>::default()),
        },
        fields: &[FieldInfoData {
            name: "Assets",
            name_hash: hash_quick_str_const("Assets"),
            flags: MemberInfoFlags::new(0),
            field_type: "Asset-Array",
            rust_offset: offset_of!(ResourceBundleAsset, assets),
        }],
    }),
    array_type: Some(RESOURCEBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ResourceBundleAsset {
    fn type_info(&self) -> &'static TypeInfo {
        RESOURCEBUNDLEASSET_TYPE_INFO
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
    fn data_container_core_mut(
        &mut self,
    ) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}

pub static RESOURCEBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceBundleAsset-Array",
    name_hash: 1986217298,
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshAsset"),
    array_type: None,
    alignment: 8,
};

pub fn register_types(registry: &mut TypeRegistry) {
    register_mod_types(registry);

    registry.register_type(RESOURCEBUNDLEASSET_TYPE_INFO);
    registry.register_type(RESOURCEBUNDLEASSET_ARRAY_TYPE_INFO);
}
