use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_memento_types(registry: &mut TypeRegistry) {
    registry.register_type(MEMENTOSETTINGS_TYPE_INFO);
    registry.register_type(MEMENTOSETTINGS_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MementoSettings {
    pub enabled: bool,
    pub gzip_compression: bool,
}

pub const MEMENTOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MementoSettings",
    flags: MemberInfoFlags::new(101),
    module: "Memento",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MementoSettings, enabled),
            },
            FieldInfoData {
                name: "GzipCompression",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MementoSettings, gzip_compression),
            },
        ],
    }),
    array_type: Some(MEMENTOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MementoSettings {
    fn type_info() -> &'static TypeInfo {
        MEMENTOSETTINGS_TYPE_INFO
    }
}


pub const MEMENTOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MementoSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Memento",
    data: TypeInfoData::Array("MementoSettings-Array"),
    array_type: None,
    alignment: 8,
};


