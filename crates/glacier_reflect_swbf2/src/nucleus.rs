use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusPlatformConfiguration {
    pub platform: super::core::GamePlatform,
    pub client_id: String,
    pub client_secret: String,
    pub login_scope: String,
    pub client_redirect_url: String,
    pub p_s_n_client_id: String,
}

pub const NUCLEUSPLATFORMCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPlatformConfiguration",
    flags: MemberInfoFlags::new(73),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(NucleusPlatformConfiguration, platform),
            },
            FieldInfoData {
                name: "ClientId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NucleusPlatformConfiguration, client_id),
            },
            FieldInfoData {
                name: "ClientSecret",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NucleusPlatformConfiguration, client_secret),
            },
            FieldInfoData {
                name: "LoginScope",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NucleusPlatformConfiguration, login_scope),
            },
            FieldInfoData {
                name: "ClientRedirectUrl",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NucleusPlatformConfiguration, client_redirect_url),
            },
            FieldInfoData {
                name: "PSNClientId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NucleusPlatformConfiguration, p_s_n_client_id),
            },
        ],
    }),
    array_type: Some(NUCLEUSPLATFORMCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NucleusPlatformConfiguration {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSPLATFORMCONFIGURATION_TYPE_INFO
    }
}


pub const NUCLEUSPLATFORMCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPlatformConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "Nucleus",
    data: TypeInfoData::Array("NucleusPlatformConfiguration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusCloseBrowserMessage {
}

pub const NUCLEUSCLOSEBROWSERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusCloseBrowserMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NucleusCloseBrowserMessage {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSCLOSEBROWSERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusGetLoginStatusMessageBase {
}

pub const NUCLEUSGETLOGINSTATUSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusGetLoginStatusMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusGetLoginStatusMessageBase {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSGETLOGINSTATUSMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusResponseLoginUIMessageBase {
}

pub const NUCLEUSRESPONSELOGINUIMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusResponseLoginUIMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusResponseLoginUIMessageBase {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSRESPONSELOGINUIMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusResponseMessageBase {
}

pub const NUCLEUSRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusResponseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusResponseMessageBase {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSRESPONSEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusRequestAuthCodeMessageBase {
}

pub const NUCLEUSREQUESTAUTHCODEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusRequestAuthCodeMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusRequestAuthCodeMessageBase {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSREQUESTAUTHCODEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusRequestLogoutMessageBase {
}

pub const NUCLEUSREQUESTLOGOUTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusRequestLogoutMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusRequestLogoutMessageBase {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSREQUESTLOGOUTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusRequestLoginMessageBase {
}

pub const NUCLEUSREQUESTLOGINMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusRequestLoginMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Nucleus",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for NucleusRequestLoginMessageBase {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSREQUESTLOGINMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum NucleusAsyncRequestType {
    #[default]
    NucleusAsyncRequestType_Login = 0,
    NucleusAsyncRequestType_Logout = 1,
    NucleusAsyncRequestType_AuthCode = 2,
    NucleusAsyncRequestType_AccessToken = 3,
}

pub const NUCLEUSASYNCREQUESTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusAsyncRequestType",
    flags: MemberInfoFlags::new(49429),
    module: "Nucleus",
    data: TypeInfoData::Enum,
    array_type: Some(NUCLEUSASYNCREQUESTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NucleusAsyncRequestType {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSASYNCREQUESTTYPE_TYPE_INFO
    }
}


pub const NUCLEUSASYNCREQUESTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusAsyncRequestType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Nucleus",
    data: TypeInfoData::Array("NucleusAsyncRequestType-Array"),
    array_type: None,
    alignment: 8,
};


