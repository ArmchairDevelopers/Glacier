use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct BifrostSettings {
    pub _glacier_base: BifrostInternal,
    pub prod: Option<LockedTypeObject /* BifrostInternal */>,
    pub dev: Option<LockedTypeObject /* BifrostInternal */>,
    pub test: Option<LockedTypeObject /* BifrostInternal */>,
    pub cert: Option<LockedTypeObject /* BifrostInternal */>,
    pub title: String,
}

pub trait BifrostSettingsTrait: BifrostInternalTrait {
    fn prod(&self) -> &Option<LockedTypeObject /* BifrostInternal */>;
    fn prod_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */>;
    fn dev(&self) -> &Option<LockedTypeObject /* BifrostInternal */>;
    fn dev_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */>;
    fn test(&self) -> &Option<LockedTypeObject /* BifrostInternal */>;
    fn test_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */>;
    fn cert(&self) -> &Option<LockedTypeObject /* BifrostInternal */>;
    fn cert_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */>;
    fn title(&self) -> &String;
    fn title_mut(&mut self) -> &mut String;
}

impl BifrostSettingsTrait for BifrostSettings {
    fn prod(&self) -> &Option<LockedTypeObject /* BifrostInternal */> {
        &self.prod
    }
    fn prod_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */> {
        &mut self.prod
    }
    fn dev(&self) -> &Option<LockedTypeObject /* BifrostInternal */> {
        &self.dev
    }
    fn dev_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */> {
        &mut self.dev
    }
    fn test(&self) -> &Option<LockedTypeObject /* BifrostInternal */> {
        &self.test
    }
    fn test_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */> {
        &mut self.test
    }
    fn cert(&self) -> &Option<LockedTypeObject /* BifrostInternal */> {
        &self.cert
    }
    fn cert_mut(&mut self) -> &mut Option<LockedTypeObject /* BifrostInternal */> {
        &mut self.cert
    }
    fn title(&self) -> &String {
        &self.title
    }
    fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }
}

impl BifrostInternalTrait for BifrostSettings {
    fn host(&self) -> &String {
        self._glacier_base.host()
    }
    fn host_mut(&mut self) -> &mut String {
        self._glacier_base.host_mut()
    }
    fn root_env(&self) -> &String {
        self._glacier_base.root_env()
    }
    fn root_env_mut(&mut self) -> &mut String {
        self._glacier_base.root_env_mut()
    }
    fn blaze_env(&self) -> &String {
        self._glacier_base.blaze_env()
    }
    fn blaze_env_mut(&mut self) -> &mut String {
        self._glacier_base.blaze_env_mut()
    }
    fn s_s_l(&self) -> &bool {
        self._glacier_base.s_s_l()
    }
    fn s_s_l_mut(&mut self) -> &mut bool {
        self._glacier_base.s_s_l_mut()
    }
}

impl super::core::SystemSettingsTrait for BifrostSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for BifrostSettings {
}

pub static BIFROSTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostSettings",
    name_hash: 900938487,
    flags: MemberInfoFlags::new(101),
    module: "Bifrost",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BIFROSTINTERNAL_TYPE_INFO),
        super_class_offset: offset_of!(BifrostSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostSettings as Default>::default())),
            create_boxed: || Box::new(<BifrostSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Prod",
                name_hash: 2089436876,
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, prod),
            },
            FieldInfoData {
                name: "Dev",
                name_hash: 193445522,
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, dev),
            },
            FieldInfoData {
                name: "Test",
                name_hash: 2089308947,
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, test),
            },
            FieldInfoData {
                name: "Cert",
                name_hash: 2088842501,
                flags: MemberInfoFlags::new(0),
                field_type: "BifrostInternal",
                rust_offset: offset_of!(BifrostSettings, cert),
            },
            FieldInfoData {
                name: "Title",
                name_hash: 227868805,
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


pub static BIFROSTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostSettings-Array",
    name_hash: 3175646403,
    flags: MemberInfoFlags::new(145),
    module: "Bifrost",
    data: TypeInfoData::Array("BifrostSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BifrostInternal {
    pub _glacier_base: super::core::SystemSettings,
    pub host: String,
    pub root_env: String,
    pub blaze_env: String,
    pub s_s_l: bool,
}

pub trait BifrostInternalTrait: super::core::SystemSettingsTrait {
    fn host(&self) -> &String;
    fn host_mut(&mut self) -> &mut String;
    fn root_env(&self) -> &String;
    fn root_env_mut(&mut self) -> &mut String;
    fn blaze_env(&self) -> &String;
    fn blaze_env_mut(&mut self) -> &mut String;
    fn s_s_l(&self) -> &bool;
    fn s_s_l_mut(&mut self) -> &mut bool;
}

impl BifrostInternalTrait for BifrostInternal {
    fn host(&self) -> &String {
        &self.host
    }
    fn host_mut(&mut self) -> &mut String {
        &mut self.host
    }
    fn root_env(&self) -> &String {
        &self.root_env
    }
    fn root_env_mut(&mut self) -> &mut String {
        &mut self.root_env
    }
    fn blaze_env(&self) -> &String {
        &self.blaze_env
    }
    fn blaze_env_mut(&mut self) -> &mut String {
        &mut self.blaze_env
    }
    fn s_s_l(&self) -> &bool {
        &self.s_s_l
    }
    fn s_s_l_mut(&mut self) -> &mut bool {
        &mut self.s_s_l
    }
}

impl super::core::SystemSettingsTrait for BifrostInternal {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for BifrostInternal {
}

pub static BIFROSTINTERNAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostInternal",
    name_hash: 1362488725,
    flags: MemberInfoFlags::new(101),
    module: "Bifrost",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(BifrostInternal, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostInternal as Default>::default())),
            create_boxed: || Box::new(<BifrostInternal as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Host",
                name_hash: 2089155077,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BifrostInternal, host),
            },
            FieldInfoData {
                name: "RootEnv",
                name_hash: 1708119358,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BifrostInternal, root_env),
            },
            FieldInfoData {
                name: "BlazeEnv",
                name_hash: 3649525832,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BifrostInternal, blaze_env),
            },
            FieldInfoData {
                name: "SSL",
                name_hash: 193466825,
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


pub static BIFROSTINTERNAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostInternal-Array",
    name_hash: 3717070113,
    flags: MemberInfoFlags::new(145),
    module: "Bifrost",
    data: TypeInfoData::Array("BifrostInternal"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BifrostHttpErrorMessage {
}

pub trait BifrostHttpErrorMessageTrait: TypeObject {
}

impl BifrostHttpErrorMessageTrait for BifrostHttpErrorMessage {
}

pub static BIFROSTHTTPERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostHttpErrorMessage",
    name_hash: 3717452185,
    flags: MemberInfoFlags::new(36937),
    module: "Bifrost",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostHttpErrorMessage as Default>::default())),
            create_boxed: || Box::new(<BifrostHttpErrorMessage as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct BifrostConnectionErrorMessage {
}

pub trait BifrostConnectionErrorMessageTrait: TypeObject {
}

impl BifrostConnectionErrorMessageTrait for BifrostConnectionErrorMessage {
}

pub static BIFROSTCONNECTIONERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BifrostConnectionErrorMessage",
    name_hash: 3654170551,
    flags: MemberInfoFlags::new(36937),
    module: "Bifrost",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BifrostConnectionErrorMessage as Default>::default())),
            create_boxed: || Box::new(<BifrostConnectionErrorMessage as Default>::default()),
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

