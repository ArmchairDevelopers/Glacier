use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_web_utils_types(registry: &mut TypeRegistry) {
    registry.register_type(URLCONFIGDATA_TYPE_INFO);
    registry.register_type(URLCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEBUTILSENVIRONMENT_TYPE_INFO);
    registry.register_type(WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct URLConfigData {
    pub _glacier_base: super::core::DataContainer,
    pub name: String,
    pub base_url: String,
    pub url: String,
    pub environment: WebUtilsEnvironment,
}

pub trait URLConfigDataTrait: super::core::DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn base_url(&self) -> &String;
    fn base_url_mut(&mut self) -> &mut String;
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn environment(&self) -> &WebUtilsEnvironment;
    fn environment_mut(&mut self) -> &mut WebUtilsEnvironment;
}

impl URLConfigDataTrait for URLConfigData {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn base_url(&self) -> &String {
        &self.base_url
    }
    fn base_url_mut(&mut self) -> &mut String {
        &mut self.base_url
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn environment(&self) -> &WebUtilsEnvironment {
        &self.environment
    }
    fn environment_mut(&mut self) -> &mut WebUtilsEnvironment {
        &mut self.environment
    }
}

impl super::core::DataContainerTrait for URLConfigData {
}

pub static URLCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "URLConfigData",
    name_hash: 931746420,
    flags: MemberInfoFlags::new(101),
    module: "WebUtils",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(URLConfigData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<URLConfigData as Default>::default())),
            create_boxed: || Box::new(<URLConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(URLConfigData, name),
            },
            FieldInfoData {
                name: "BaseUrl",
                name_hash: 2308759387,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(URLConfigData, base_url),
            },
            FieldInfoData {
                name: "Url",
                name_hash: 193455022,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(URLConfigData, url),
            },
            FieldInfoData {
                name: "Environment",
                name_hash: 2480382480,
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


pub static URLCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "URLConfigData-Array",
    name_hash: 3214573120,
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
    name_hash: 2108023703,
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


pub static WEBUTILSENVIRONMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebUtilsEnvironment-Array",
    name_hash: 1909910563,
    flags: MemberInfoFlags::new(145),
    module: "WebUtils",
    data: TypeInfoData::Array("WebUtilsEnvironment"),
    array_type: None,
    alignment: 8,
};


