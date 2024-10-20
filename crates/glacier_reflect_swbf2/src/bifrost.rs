use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct BifrostSettings {
    pub _glacier_base: BifrostInternal,
    pub prod: Option<Arc<Mutex<dyn BifrostInternalTrait>>>,
    pub dev: Option<Arc<Mutex<dyn BifrostInternalTrait>>>,
    pub test: Option<Arc<Mutex<dyn BifrostInternalTrait>>>,
    pub cert: Option<Arc<Mutex<dyn BifrostInternalTrait>>>,
    pub title: String,
}

pub trait BifrostSettingsTrait: BifrostInternalTrait {
    fn prod(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>>;
    fn dev(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>>;
    fn test(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>>;
    fn cert(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>>;
    fn title(&self) -> &String;
}

impl BifrostSettingsTrait for BifrostSettings {
    fn prod(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>> {
        &self.prod
    }
    fn dev(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>> {
        &self.dev
    }
    fn test(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>> {
        &self.test
    }
    fn cert(&self) -> &Option<Arc<Mutex<dyn BifrostInternalTrait>>> {
        &self.cert
    }
    fn title(&self) -> &String {
        &self.title
    }
}

impl BifrostInternalTrait for BifrostSettings {
    fn host(&self) -> &String {
        self._glacier_base.host()
    }
    fn root_env(&self) -> &String {
        self._glacier_base.root_env()
    }
    fn blaze_env(&self) -> &String {
        self._glacier_base.blaze_env()
    }
    fn s_s_l(&self) -> &bool {
        self._glacier_base.s_s_l()
    }
}

impl super::core::SystemSettingsTrait for BifrostSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for BifrostSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BIFROSTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostSettings",
    flags: MemberInfoFlags::new(101),
    module: "Bifrost",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BIFROSTINTERNAL_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Prod",
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, prod),
            },
            FieldInfoData {
                name: "Dev",
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, dev),
            },
            FieldInfoData {
                name: "Test",
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, test),
            },
            FieldInfoData {
                name: "Cert",
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, cert),
            },
            FieldInfoData {
                name: "Title",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BifrostSettings, title),
            },
        ],
    }),
    array_type: Some(BIFROSTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BifrostSettings {
    fn type_info(&self) -> &'static TypeInfo {
        BIFROSTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BIFROSTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Bifrost",
    data: TypeInfoData::Array("BifrostSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BifrostInternal {
    pub _glacier_base: super::core::SystemSettings,
    pub host: String,
    pub root_env: String,
    pub blaze_env: String,
    pub s_s_l: bool,
}

pub trait BifrostInternalTrait: super::core::SystemSettingsTrait {
    fn host(&self) -> &String;
    fn root_env(&self) -> &String;
    fn blaze_env(&self) -> &String;
    fn s_s_l(&self) -> &bool;
}

impl BifrostInternalTrait for BifrostInternal {
    fn host(&self) -> &String {
        &self.host
    }
    fn root_env(&self) -> &String {
        &self.root_env
    }
    fn blaze_env(&self) -> &String {
        &self.blaze_env
    }
    fn s_s_l(&self) -> &bool {
        &self.s_s_l
    }
}

impl super::core::SystemSettingsTrait for BifrostInternal {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for BifrostInternal {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BIFROSTINTERNAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostInternal",
    flags: MemberInfoFlags::new(101),
    module: "Bifrost",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostInternal as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Host",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BifrostInternal, host),
            },
            FieldInfoData {
                name: "RootEnv",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BifrostInternal, root_env),
            },
            FieldInfoData {
                name: "BlazeEnv",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BifrostInternal, blaze_env),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BifrostInternal, s_s_l),
            },
        ],
    }),
    array_type: Some(BIFROSTINTERNAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BifrostInternal {
    fn type_info(&self) -> &'static TypeInfo {
        BIFROSTINTERNAL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BIFROSTINTERNAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostInternal-Array",
    flags: MemberInfoFlags::new(145),
    module: "Bifrost",
    data: TypeInfoData::Array("BifrostInternal"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BifrostHttpErrorMessage {
}

pub trait BifrostHttpErrorMessageTrait: TypeObject {
}

impl BifrostHttpErrorMessageTrait for BifrostHttpErrorMessage {
}

pub static BIFROSTHTTPERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostHttpErrorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Bifrost",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostHttpErrorMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for BifrostHttpErrorMessage {
    fn type_info(&self) -> &'static TypeInfo {
        BIFROSTHTTPERRORMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct BifrostConnectionErrorMessage {
}

pub trait BifrostConnectionErrorMessageTrait: TypeObject {
}

impl BifrostConnectionErrorMessageTrait for BifrostConnectionErrorMessage {
}

pub static BIFROSTCONNECTIONERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostConnectionErrorMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Bifrost",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostConnectionErrorMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for BifrostConnectionErrorMessage {
    fn type_info(&self) -> &'static TypeInfo {
        BIFROSTCONNECTIONERRORMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

