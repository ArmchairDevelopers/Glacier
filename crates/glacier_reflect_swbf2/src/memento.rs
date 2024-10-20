use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_memento_types(registry: &mut TypeRegistry) {
    registry.register_type(MEMENTOSETTINGS_TYPE_INFO);
    registry.register_type(MEMENTOSETTINGS_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct MementoSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub gzip_compression: bool,
}

pub trait MementoSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn gzip_compression(&self) -> &bool;
}

impl MementoSettingsTrait for MementoSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn gzip_compression(&self) -> &bool {
        &self.gzip_compression
    }
}

impl super::core::SystemSettingsTrait for MementoSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for MementoSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MEMENTOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MementoSettings",
    flags: MemberInfoFlags::new(101),
    module: "Memento",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MementoSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MementoSettings, enabled),
            },
            FieldInfoData {
                name: "GzipCompression",
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
}


pub static MEMENTOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MementoSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Memento",
    data: TypeInfoData::Array("MementoSettings"),
    array_type: None,
    alignment: 8,
};


