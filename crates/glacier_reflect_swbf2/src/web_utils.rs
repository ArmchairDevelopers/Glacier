use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_web_utils_types(registry: &mut TypeRegistry) {
    registry.register_type(URLCONFIGDATA_TYPE_INFO);
    registry.register_type(URLCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEBUTILSENVIRONMENT_TYPE_INFO);
    registry.register_type(WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct URLConfigData {
    pub name: String,
    pub base_url: String,
    pub url: String,
    pub environment: WebUtilsEnvironment,
}

pub const URLCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "URLConfigData",
    flags: MemberInfoFlags::new(101),
    module: "WebUtils",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(URLConfigData, name),
            },
            FieldInfoData {
                name: "BaseUrl",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(URLConfigData, base_url),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(URLConfigData, url),
            },
            FieldInfoData {
                name: "Environment",
                flags: MemberInfoFlags::new(0),
                field_type: WEBUTILSENVIRONMENT_TYPE_INFO,
                rust_offset: offset_of!(URLConfigData, environment),
            },
        ],
    }),
    array_type: Some(URLCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for URLConfigData {
    fn type_info() -> &'static TypeInfo {
        URLCONFIGDATA_TYPE_INFO
    }
}


pub const URLCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "URLConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebUtils",
    data: TypeInfoData::Array("URLConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WebUtilsEnvironment {
    #[default]
    WebUtilsEnvironment_Development = 0,
    WebUtilsEnvironment_Test = 1,
    WebUtilsEnvironment_Certification = 2,
    WebUtilsEnvironment_Production = 3,
    WebUtilsEnvironment_Any = 4,
    WebUtilsEnvironment_Invalid = 5,
    WebUtilsEnvironment_Count = 6,
}

pub const WEBUTILSENVIRONMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebUtilsEnvironment",
    flags: MemberInfoFlags::new(49429),
    module: "WebUtils",
    data: TypeInfoData::Enum,
    array_type: Some(WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WebUtilsEnvironment {
    fn type_info() -> &'static TypeInfo {
        WEBUTILSENVIRONMENT_TYPE_INFO
    }
}


pub const WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebUtilsEnvironment-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebUtils",
    data: TypeInfoData::Array("WebUtilsEnvironment-Array"),
    array_type: None,
    alignment: 8,
};


