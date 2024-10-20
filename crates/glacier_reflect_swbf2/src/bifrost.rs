use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_bifrost_types(registry: &mut TypeRegistry) {
    registry.register_type(BIFROSTSETTINGS_TYPE_INFO);
    registry.register_type(BIFROSTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(BIFROSTINTERNAL_TYPE_INFO);
    registry.register_type(BIFROSTINTERNAL_ARRAY_TYPE_INFO);
    registry.register_type(BIFROSTHTTPERRORMESSAGE_TYPE_INFO);
    registry.register_type(BIFROSTCONNECTIONERRORMESSAGE_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BifrostSettings {
    pub prod: BifrostInternal,
    pub dev: BifrostInternal,
    pub test: BifrostInternal,
    pub cert: BifrostInternal,
    pub title: String,
}

pub const BIFROSTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostSettings",
    flags: MemberInfoFlags::new(101),
    module: "Bifrost",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BIFROSTINTERNAL_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Prod",
                flags: MemberInfoFlags::new(0),
                field_type: BIFROSTINTERNAL_TYPE_INFO,
                rust_offset: offset_of!(BifrostSettings, prod),
            },
            FieldInfoData {
                name: "Dev",
                flags: MemberInfoFlags::new(0),
                field_type: BIFROSTINTERNAL_TYPE_INFO,
                rust_offset: offset_of!(BifrostSettings, dev),
            },
            FieldInfoData {
                name: "Test",
                flags: MemberInfoFlags::new(0),
                field_type: BIFROSTINTERNAL_TYPE_INFO,
                rust_offset: offset_of!(BifrostSettings, test),
            },
            FieldInfoData {
                name: "Cert",
                flags: MemberInfoFlags::new(0),
                field_type: BIFROSTINTERNAL_TYPE_INFO,
                rust_offset: offset_of!(BifrostSettings, cert),
            },
            FieldInfoData {
                name: "Title",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BifrostSettings, title),
            },
        ],
    }),
    array_type: Some(BIFROSTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BifrostSettings {
    fn type_info() -> &'static TypeInfo {
        BIFROSTSETTINGS_TYPE_INFO
    }
}


pub const BIFROSTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Bifrost",
    data: TypeInfoData::Array("BifrostSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BifrostInternal {
    pub host: String,
    pub root_env: String,
    pub blaze_env: String,
    pub s_s_l: bool,
}

pub const BIFROSTINTERNAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostInternal",
    flags: MemberInfoFlags::new(101),
    module: "Bifrost",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Host",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BifrostInternal, host),
            },
            FieldInfoData {
                name: "RootEnv",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BifrostInternal, root_env),
            },
            FieldInfoData {
                name: "BlazeEnv",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BifrostInternal, blaze_env),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BifrostInternal, s_s_l),
            },
        ],
    }),
    array_type: Some(BIFROSTINTERNAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BifrostInternal {
    fn type_info() -> &'static TypeInfo {
        BIFROSTINTERNAL_TYPE_INFO
    }
}


pub const BIFROSTINTERNAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostInternal-Array",
    flags: MemberInfoFlags::new(145),
    module: "Bifrost",
    data: TypeInfoData::Array("BifrostInternal-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BifrostHttpErrorMessage {
}

pub const BIFROSTHTTPERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostHttpErrorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Bifrost",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for BifrostHttpErrorMessage {
    fn type_info() -> &'static TypeInfo {
        BIFROSTHTTPERRORMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BifrostConnectionErrorMessage {
}

pub const BIFROSTCONNECTIONERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostConnectionErrorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Bifrost",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for BifrostConnectionErrorMessage {
    fn type_info() -> &'static TypeInfo {
        BIFROSTCONNECTIONERRORMESSAGE_TYPE_INFO
    }
}

