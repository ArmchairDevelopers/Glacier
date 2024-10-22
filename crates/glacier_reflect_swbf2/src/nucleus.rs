use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_nucleus_types(registry: &mut TypeRegistry) {
    registry.register_type(NUCLEUSPLATFORMCONFIGURATION_TYPE_INFO);
    registry.register_type(NUCLEUSPLATFORMCONFIGURATION_ARRAY_TYPE_INFO);
    registry.register_type(NUCLEUSCLOSEBROWSERMESSAGE_TYPE_INFO);
    registry.register_type(NUCLEUSGETLOGINSTATUSMESSAGEBASE_TYPE_INFO);
    registry.register_type(NUCLEUSRESPONSELOGINUIMESSAGEBASE_TYPE_INFO);
    registry.register_type(NUCLEUSRESPONSEMESSAGEBASE_TYPE_INFO);
    registry.register_type(NUCLEUSREQUESTAUTHCODEMESSAGEBASE_TYPE_INFO);
    registry.register_type(NUCLEUSREQUESTLOGOUTMESSAGEBASE_TYPE_INFO);
    registry.register_type(NUCLEUSREQUESTLOGINMESSAGEBASE_TYPE_INFO);
    registry.register_type(NUCLEUSASYNCREQUESTTYPE_TYPE_INFO);
    registry.register_type(NUCLEUSASYNCREQUESTTYPE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct NucleusPlatformConfiguration {
    pub platform: super::core::GamePlatform,
    pub client_id: String,
    pub client_secret: String,
    pub login_scope: String,
    pub client_redirect_url: String,
    pub p_s_n_client_id: String,
}

pub trait NucleusPlatformConfigurationTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn client_id(&self) -> &String;
    fn client_id_mut(&mut self) -> &mut String;
    fn client_secret(&self) -> &String;
    fn client_secret_mut(&mut self) -> &mut String;
    fn login_scope(&self) -> &String;
    fn login_scope_mut(&mut self) -> &mut String;
    fn client_redirect_url(&self) -> &String;
    fn client_redirect_url_mut(&mut self) -> &mut String;
    fn p_s_n_client_id(&self) -> &String;
    fn p_s_n_client_id_mut(&mut self) -> &mut String;
}

impl NucleusPlatformConfigurationTrait for NucleusPlatformConfiguration {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn client_id(&self) -> &String {
        &self.client_id
    }
    fn client_id_mut(&mut self) -> &mut String {
        &mut self.client_id
    }
    fn client_secret(&self) -> &String {
        &self.client_secret
    }
    fn client_secret_mut(&mut self) -> &mut String {
        &mut self.client_secret
    }
    fn login_scope(&self) -> &String {
        &self.login_scope
    }
    fn login_scope_mut(&mut self) -> &mut String {
        &mut self.login_scope
    }
    fn client_redirect_url(&self) -> &String {
        &self.client_redirect_url
    }
    fn client_redirect_url_mut(&mut self) -> &mut String {
        &mut self.client_redirect_url
    }
    fn p_s_n_client_id(&self) -> &String {
        &self.p_s_n_client_id
    }
    fn p_s_n_client_id_mut(&mut self) -> &mut String {
        &mut self.p_s_n_client_id
    }
}

pub static NUCLEUSPLATFORMCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPlatformConfiguration",
    flags: MemberInfoFlags::new(73),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusPlatformConfiguration as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(NucleusPlatformConfiguration, platform),
            },
            FieldInfoData {
                name: "ClientId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NucleusPlatformConfiguration, client_id),
            },
            FieldInfoData {
                name: "ClientSecret",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NucleusPlatformConfiguration, client_secret),
            },
            FieldInfoData {
                name: "LoginScope",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NucleusPlatformConfiguration, login_scope),
            },
            FieldInfoData {
                name: "ClientRedirectUrl",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NucleusPlatformConfiguration, client_redirect_url),
            },
            FieldInfoData {
                name: "PSNClientId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NucleusPlatformConfiguration, p_s_n_client_id),
            },
        ],
    }),
    array_type: Some(NUCLEUSPLATFORMCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NucleusPlatformConfiguration {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSPLATFORMCONFIGURATION_TYPE_INFO
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


pub static NUCLEUSPLATFORMCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPlatformConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "Nucleus",
    data: TypeInfoData::Array("NucleusPlatformConfiguration"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NucleusCloseBrowserMessage {
}

pub trait NucleusCloseBrowserMessageTrait: TypeObject {
}

impl NucleusCloseBrowserMessageTrait for NucleusCloseBrowserMessage {
}

pub static NUCLEUSCLOSEBROWSERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusCloseBrowserMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusCloseBrowserMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NucleusCloseBrowserMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSCLOSEBROWSERMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct NucleusGetLoginStatusMessageBase {
}

pub trait NucleusGetLoginStatusMessageBaseTrait: TypeObject {
}

impl NucleusGetLoginStatusMessageBaseTrait for NucleusGetLoginStatusMessageBase {
}

pub static NUCLEUSGETLOGINSTATUSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusGetLoginStatusMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusGetLoginStatusMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusGetLoginStatusMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSGETLOGINSTATUSMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct NucleusResponseLoginUIMessageBase {
}

pub trait NucleusResponseLoginUIMessageBaseTrait: TypeObject {
}

impl NucleusResponseLoginUIMessageBaseTrait for NucleusResponseLoginUIMessageBase {
}

pub static NUCLEUSRESPONSELOGINUIMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusResponseLoginUIMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusResponseLoginUIMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusResponseLoginUIMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSRESPONSELOGINUIMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct NucleusResponseMessageBase {
}

pub trait NucleusResponseMessageBaseTrait: TypeObject {
}

impl NucleusResponseMessageBaseTrait for NucleusResponseMessageBase {
}

pub static NUCLEUSRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusResponseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusResponseMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusResponseMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSRESPONSEMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct NucleusRequestAuthCodeMessageBase {
}

pub trait NucleusRequestAuthCodeMessageBaseTrait: TypeObject {
}

impl NucleusRequestAuthCodeMessageBaseTrait for NucleusRequestAuthCodeMessageBase {
}

pub static NUCLEUSREQUESTAUTHCODEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusRequestAuthCodeMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusRequestAuthCodeMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusRequestAuthCodeMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSREQUESTAUTHCODEMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct NucleusRequestLogoutMessageBase {
}

pub trait NucleusRequestLogoutMessageBaseTrait: TypeObject {
}

impl NucleusRequestLogoutMessageBaseTrait for NucleusRequestLogoutMessageBase {
}

pub static NUCLEUSREQUESTLOGOUTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusRequestLogoutMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusRequestLogoutMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusRequestLogoutMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSREQUESTLOGOUTMESSAGEBASE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct NucleusRequestLoginMessageBase {
}

pub trait NucleusRequestLoginMessageBaseTrait: TypeObject {
}

impl NucleusRequestLoginMessageBaseTrait for NucleusRequestLoginMessageBase {
}

pub static NUCLEUSREQUESTLOGINMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusRequestLoginMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusRequestLoginMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusRequestLoginMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSREQUESTLOGINMESSAGEBASE_TYPE_INFO
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

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum NucleusAsyncRequestType {
    #[default]
    NucleusAsyncRequestType_Login = 0,
    NucleusAsyncRequestType_Logout = 1,
    NucleusAsyncRequestType_AuthCode = 2,
    NucleusAsyncRequestType_AccessToken = 3,
}

pub static NUCLEUSASYNCREQUESTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusAsyncRequestType",
    flags: MemberInfoFlags::new(49429),
    module: "Nucleus",
    data: TypeInfoData::Enum,
    array_type: Some(NUCLEUSASYNCREQUESTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NucleusAsyncRequestType {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSASYNCREQUESTTYPE_TYPE_INFO
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


pub static NUCLEUSASYNCREQUESTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusAsyncRequestType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Nucleus",
    data: TypeInfoData::Array("NucleusAsyncRequestType"),
    array_type: None,
    alignment: 8,
};


