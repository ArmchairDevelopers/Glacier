use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_memento_types(registry: &mut TypeRegistry) {
    registry.register_type(MEMENTOSETTINGS_TYPE_INFO);
    registry.register_type(MEMENTOSETTINGS_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct MementoSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub gzip_compression: bool,
}

pub trait MementoSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn gzip_compression(&self) -> &bool;
    fn gzip_compression_mut(&mut self) -> &mut bool;
}

impl MementoSettingsTrait for MementoSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn gzip_compression(&self) -> &bool {
        &self.gzip_compression
    }
    fn gzip_compression_mut(&mut self) -> &mut bool {
        &mut self.gzip_compression
    }
}

impl super::core::SystemSettingsTrait for MementoSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for MementoSettings {
}

pub static MEMENTOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MementoSettings",
    name_hash: 2003918453,
    flags: MemberInfoFlags::new(101),
    module: "Memento",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(MementoSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MementoSettings as Default>::default())),
            create_boxed: || Box::new(<MementoSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MementoSettings, enabled),
            },
            FieldInfoData {
                name: "GzipCompression",
                name_hash: 3094795343,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MementoSettings, gzip_compression),
            },
        ],
    }),
    array_type: Some(MEMENTOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MementoSettings {
    fn type_info(&self) -> &'static TypeInfo {
        MEMENTOSETTINGS_TYPE_INFO
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


pub static MEMENTOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MementoSettings-Array",
    name_hash: 4014411585,
    flags: MemberInfoFlags::new(145),
    module: "Memento",
    data: TypeInfoData::Array("MementoSettings"),
    array_type: None,
    alignment: 8,
};


