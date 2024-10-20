use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_web_utils_types(registry: &mut TypeRegistry) {
    registry.register_type(URLCONFIGDATA_TYPE_INFO);
    registry.register_type(URLCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEBUTILSENVIRONMENT_TYPE_INFO);
    registry.register_type(WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct URLConfigData {
    pub _glacier_base: super::core::DataContainer,
    pub name: String,
    pub base_url: String,
    pub url: String,
    pub environment: WebUtilsEnvironment,
}

pub trait URLConfigDataTrait: super::core::DataContainerTrait {
    fn name(&self) -> &String;
    fn base_url(&self) -> &String;
    fn url(&self) -> &String;
    fn environment(&self) -> &WebUtilsEnvironment;
}

impl URLConfigDataTrait for URLConfigData {
    fn name(&self) -> &String {
        &self.name
    }
    fn base_url(&self) -> &String {
        &self.base_url
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn environment(&self) -> &WebUtilsEnvironment {
        &self.environment
    }
}

impl super::core::DataContainerTrait for URLConfigData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static URLCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "URLConfigData",
    flags: MemberInfoFlags::new(101),
    module: "WebUtils",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<URLConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(URLConfigData, name),
            },
            FieldInfoData {
                name: "BaseUrl",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(URLConfigData, base_url),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(URLConfigData, url),
            },
            FieldInfoData {
                name: "Environment",
                flags: MemberInfoFlags::new(0),
                field_type: "WebUtilsEnvironment",
                rust_offset: offset_of!(URLConfigData, environment),
            },
        ],
    }),
    array_type: Some(URLCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for URLConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        URLCONFIGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static URLCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "URLConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebUtils",
    data: TypeInfoData::Array("URLConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static WEBUTILSENVIRONMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebUtilsEnvironment",
    flags: MemberInfoFlags::new(49429),
    module: "WebUtils",
    data: TypeInfoData::Enum,
    array_type: Some(WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WebUtilsEnvironment {
    fn type_info(&self) -> &'static TypeInfo {
        WEBUTILSENVIRONMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebUtilsEnvironment-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebUtils",
    data: TypeInfoData::Array("WebUtilsEnvironment"),
    array_type: None,
    alignment: 8,
};


