use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_entitlements_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCELICENSEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCELICENSEMESSAGEBASE_TYPE_INFO);
    registry.register_type(LICENSECONFIGURATION_TYPE_INFO);
    registry.register_type(LICENSECONFIGURATION_ARRAY_TYPE_INFO);
    registry.register_type(LICENSEINFO_TYPE_INFO);
    registry.register_type(LICENSEINFO_ARRAY_TYPE_INFO);
    registry.register_type(NUCLEUSENTITLEMENTINFO_TYPE_INFO);
    registry.register_type(NUCLEUSENTITLEMENTINFO_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTSSERVERBACKENDDATA_TYPE_INFO);
    registry.register_type(ENTITLEMENTSSERVERBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTSBACKENDDATA_TYPE_INFO);
    registry.register_type(ENTITLEMENTSBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTSETTINGS_TYPE_INFO);
    registry.register_type(ENTITLEMENTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTSETTINGSASSET_TYPE_INFO);
    registry.register_type(ENTITLEMENTSETTINGSASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTPLATFORMTOPROJECTID_TYPE_INFO);
    registry.register_type(ENTITLEMENTPLATFORMTOPROJECTID_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTCONFIGDATA_TYPE_INFO);
    registry.register_type(ENTITLEMENTCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTORIGINCONFIGDATA_TYPE_INFO);
    registry.register_type(ENTITLEMENTORIGINCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTGROUP_TYPE_INFO);
    registry.register_type(ENTITLEMENTGROUP_ARRAY_TYPE_INFO);
    registry.register_type(ENTITLEMENTINFO_TYPE_INFO);
    registry.register_type(ENTITLEMENTINFO_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEENTITLEMENTSSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEENTITLEMENTSSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SERVERENTITLEMENTSBACKEND_TYPE_INFO);
    registry.register_type(SERVERENTITLEMENTSBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(LICENSEMAPPINGEVENT_TYPE_INFO);
    registry.register_type(LICENSEMAPPINGEVENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTENTITLEMENTSSERVICE_TYPE_INFO);
    registry.register_type(CLIENTENTITLEMENTSSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTENTITLEMENTSBACKEND_TYPE_INFO);
    registry.register_type(CLIENTENTITLEMENTSBACKEND_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceLicenseRequestMessageBase {
}

pub trait PresenceLicenseRequestMessageBaseTrait: TypeObject {
}

impl PresenceLicenseRequestMessageBaseTrait for PresenceLicenseRequestMessageBase {
}

pub static PRESENCELICENSEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLicenseRequestMessageBase",
    name_hash: 4245581064,
    flags: MemberInfoFlags::new(36937),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLicenseRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceLicenseRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLicenseRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCELICENSEREQUESTMESSAGEBASE_TYPE_INFO
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
pub struct PresenceLicenseMessageBase {
}

pub trait PresenceLicenseMessageBaseTrait: TypeObject {
}

impl PresenceLicenseMessageBaseTrait for PresenceLicenseMessageBase {
}

pub static PRESENCELICENSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLicenseMessageBase",
    name_hash: 484228217,
    flags: MemberInfoFlags::new(36937),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLicenseMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceLicenseMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLicenseMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCELICENSEMESSAGEBASE_TYPE_INFO
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
pub struct LicenseConfiguration {
    pub licenses: Vec<BoxedTypeObject /* LicenseInfo */>,
}

pub trait LicenseConfigurationTrait: TypeObject {
    fn licenses(&self) -> &Vec<BoxedTypeObject /* LicenseInfo */>;
    fn licenses_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LicenseInfo */>;
}

impl LicenseConfigurationTrait for LicenseConfiguration {
    fn licenses(&self) -> &Vec<BoxedTypeObject /* LicenseInfo */> {
        &self.licenses
    }
    fn licenses_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LicenseInfo */> {
        &mut self.licenses
    }
}

pub static LICENSECONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseConfiguration",
    name_hash: 2064540654,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LicenseConfiguration as Default>::default())),
            create_boxed: || Box::new(<LicenseConfiguration as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Licenses",
                name_hash: 2259172461,
                flags: MemberInfoFlags::new(144),
                field_type: "LicenseInfo-Array",
                rust_offset: offset_of!(LicenseConfiguration, licenses),
            },
        ],
    }),
    array_type: Some(LICENSECONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LicenseConfiguration {
    fn type_info(&self) -> &'static TypeInfo {
        LICENSECONFIGURATION_TYPE_INFO
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


pub static LICENSECONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseConfiguration-Array",
    name_hash: 4204756186,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("LicenseConfiguration"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LicenseInfo {
    pub name: String,
    pub platform: super::core::GamePlatform,
    pub description: String,
    pub content_entitlements: Vec<String>,
    pub nucleus_entitlements: Vec<BoxedTypeObject /* NucleusEntitlementInfo */>,
    pub first_party_entitlements: Vec<String>,
}

pub trait LicenseInfoTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn description(&self) -> &String;
    fn description_mut(&mut self) -> &mut String;
    fn content_entitlements(&self) -> &Vec<String>;
    fn content_entitlements_mut(&mut self) -> &mut Vec<String>;
    fn nucleus_entitlements(&self) -> &Vec<BoxedTypeObject /* NucleusEntitlementInfo */>;
    fn nucleus_entitlements_mut(&mut self) -> &mut Vec<BoxedTypeObject /* NucleusEntitlementInfo */>;
    fn first_party_entitlements(&self) -> &Vec<String>;
    fn first_party_entitlements_mut(&mut self) -> &mut Vec<String>;
}

impl LicenseInfoTrait for LicenseInfo {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn description(&self) -> &String {
        &self.description
    }
    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
    fn content_entitlements(&self) -> &Vec<String> {
        &self.content_entitlements
    }
    fn content_entitlements_mut(&mut self) -> &mut Vec<String> {
        &mut self.content_entitlements
    }
    fn nucleus_entitlements(&self) -> &Vec<BoxedTypeObject /* NucleusEntitlementInfo */> {
        &self.nucleus_entitlements
    }
    fn nucleus_entitlements_mut(&mut self) -> &mut Vec<BoxedTypeObject /* NucleusEntitlementInfo */> {
        &mut self.nucleus_entitlements
    }
    fn first_party_entitlements(&self) -> &Vec<String> {
        &self.first_party_entitlements
    }
    fn first_party_entitlements_mut(&mut self) -> &mut Vec<String> {
        &mut self.first_party_entitlements
    }
}

pub static LICENSEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseInfo",
    name_hash: 113178000,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LicenseInfo as Default>::default())),
            create_boxed: || Box::new(<LicenseInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LicenseInfo, name),
            },
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(LicenseInfo, platform),
            },
            FieldInfoData {
                name: "Description",
                name_hash: 1636673251,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LicenseInfo, description),
            },
            FieldInfoData {
                name: "ContentEntitlements",
                name_hash: 345490822,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LicenseInfo, content_entitlements),
            },
            FieldInfoData {
                name: "NucleusEntitlements",
                name_hash: 1569360120,
                flags: MemberInfoFlags::new(144),
                field_type: "NucleusEntitlementInfo-Array",
                rust_offset: offset_of!(LicenseInfo, nucleus_entitlements),
            },
            FieldInfoData {
                name: "FirstPartyEntitlements",
                name_hash: 605325051,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LicenseInfo, first_party_entitlements),
            },
        ],
    }),
    array_type: Some(LICENSEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LicenseInfo {
    fn type_info(&self) -> &'static TypeInfo {
        LICENSEINFO_TYPE_INFO
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


pub static LICENSEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseInfo-Array",
    name_hash: 3400696740,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("LicenseInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NucleusEntitlementInfo {
    pub tag: String,
    pub group_name: String,
}

pub trait NucleusEntitlementInfoTrait: TypeObject {
    fn tag(&self) -> &String;
    fn tag_mut(&mut self) -> &mut String;
    fn group_name(&self) -> &String;
    fn group_name_mut(&mut self) -> &mut String;
}

impl NucleusEntitlementInfoTrait for NucleusEntitlementInfo {
    fn tag(&self) -> &String {
        &self.tag
    }
    fn tag_mut(&mut self) -> &mut String {
        &mut self.tag
    }
    fn group_name(&self) -> &String {
        &self.group_name
    }
    fn group_name_mut(&mut self) -> &mut String {
        &mut self.group_name
    }
}

pub static NUCLEUSENTITLEMENTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusEntitlementInfo",
    name_hash: 877240229,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusEntitlementInfo as Default>::default())),
            create_boxed: || Box::new(<NucleusEntitlementInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Tag",
                name_hash: 193462807,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NucleusEntitlementInfo, tag),
            },
            FieldInfoData {
                name: "GroupName",
                name_hash: 3178263901,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NucleusEntitlementInfo, group_name),
            },
        ],
    }),
    array_type: Some(NUCLEUSENTITLEMENTINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NucleusEntitlementInfo {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSENTITLEMENTINFO_TYPE_INFO
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


pub static NUCLEUSENTITLEMENTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusEntitlementInfo-Array",
    name_hash: 3711449105,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("NucleusEntitlementInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementsServerBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
}

pub trait EntitlementsServerBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
}

impl EntitlementsServerBackendDataTrait for EntitlementsServerBackendData {
}

impl super::online_shared::PresenceBackendDataTrait for EntitlementsServerBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for EntitlementsServerBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EntitlementsServerBackendData {
}

pub static ENTITLEMENTSSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsServerBackendData",
    name_hash: 1715565470,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntitlementsServerBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementsServerBackendData as Default>::default())),
            create_boxed: || Box::new(<EntitlementsServerBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENTITLEMENTSSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementsServerBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTSSERVERBACKENDDATA_TYPE_INFO
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


pub static ENTITLEMENTSSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsServerBackendData-Array",
    name_hash: 1795605674,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementsServerBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementsBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
}

pub trait EntitlementsBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
}

impl EntitlementsBackendDataTrait for EntitlementsBackendData {
}

impl super::online_shared::PresenceBackendDataTrait for EntitlementsBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for EntitlementsBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EntitlementsBackendData {
}

pub static ENTITLEMENTSBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsBackendData",
    name_hash: 755735643,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(EntitlementsBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementsBackendData as Default>::default())),
            create_boxed: || Box::new(<EntitlementsBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENTITLEMENTSBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementsBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTSBACKENDDATA_TYPE_INFO
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


pub static ENTITLEMENTSBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsBackendData-Array",
    name_hash: 138650479,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementsBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub settings: Option<LockedTypeObject /* EntitlementSettingsAsset */>,
}

pub trait EntitlementSettingsTrait: super::core::SystemSettingsTrait {
    fn settings(&self) -> &Option<LockedTypeObject /* EntitlementSettingsAsset */>;
    fn settings_mut(&mut self) -> &mut Option<LockedTypeObject /* EntitlementSettingsAsset */>;
}

impl EntitlementSettingsTrait for EntitlementSettings {
    fn settings(&self) -> &Option<LockedTypeObject /* EntitlementSettingsAsset */> {
        &self.settings
    }
    fn settings_mut(&mut self) -> &mut Option<LockedTypeObject /* EntitlementSettingsAsset */> {
        &mut self.settings
    }
}

impl super::core::SystemSettingsTrait for EntitlementSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for EntitlementSettings {
}

pub static ENTITLEMENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettings",
    name_hash: 4143593465,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(EntitlementSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementSettings as Default>::default())),
            create_boxed: || Box::new(<EntitlementSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Settings",
                name_hash: 649772672,
                flags: MemberInfoFlags::new(0),
                field_type: "EntitlementSettingsAsset",
                rust_offset: offset_of!(EntitlementSettings, settings),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTSETTINGS_TYPE_INFO
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


pub static ENTITLEMENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettings-Array",
    name_hash: 2964109005,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementSettingsAsset {
    pub _glacier_base: super::core::Asset,
    pub license_config: LicenseConfiguration,
    pub entitlement_config: EntitlementConfigData,
    pub entitlement_origin_config: EntitlementOriginConfigData,
    pub entitlements_info_list: Vec<BoxedTypeObject /* EntitlementInfo */>,
}

pub trait EntitlementSettingsAssetTrait: super::core::AssetTrait {
    fn license_config(&self) -> &LicenseConfiguration;
    fn license_config_mut(&mut self) -> &mut LicenseConfiguration;
    fn entitlement_config(&self) -> &EntitlementConfigData;
    fn entitlement_config_mut(&mut self) -> &mut EntitlementConfigData;
    fn entitlement_origin_config(&self) -> &EntitlementOriginConfigData;
    fn entitlement_origin_config_mut(&mut self) -> &mut EntitlementOriginConfigData;
    fn entitlements_info_list(&self) -> &Vec<BoxedTypeObject /* EntitlementInfo */>;
    fn entitlements_info_list_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EntitlementInfo */>;
}

impl EntitlementSettingsAssetTrait for EntitlementSettingsAsset {
    fn license_config(&self) -> &LicenseConfiguration {
        &self.license_config
    }
    fn license_config_mut(&mut self) -> &mut LicenseConfiguration {
        &mut self.license_config
    }
    fn entitlement_config(&self) -> &EntitlementConfigData {
        &self.entitlement_config
    }
    fn entitlement_config_mut(&mut self) -> &mut EntitlementConfigData {
        &mut self.entitlement_config
    }
    fn entitlement_origin_config(&self) -> &EntitlementOriginConfigData {
        &self.entitlement_origin_config
    }
    fn entitlement_origin_config_mut(&mut self) -> &mut EntitlementOriginConfigData {
        &mut self.entitlement_origin_config
    }
    fn entitlements_info_list(&self) -> &Vec<BoxedTypeObject /* EntitlementInfo */> {
        &self.entitlements_info_list
    }
    fn entitlements_info_list_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EntitlementInfo */> {
        &mut self.entitlements_info_list
    }
}

impl super::core::AssetTrait for EntitlementSettingsAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EntitlementSettingsAsset {
}

pub static ENTITLEMENTSETTINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettingsAsset",
    name_hash: 4163254857,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(EntitlementSettingsAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementSettingsAsset as Default>::default())),
            create_boxed: || Box::new(<EntitlementSettingsAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LicenseConfig",
                name_hash: 3061955380,
                flags: MemberInfoFlags::new(0),
                field_type: "LicenseConfiguration",
                rust_offset: offset_of!(EntitlementSettingsAsset, license_config),
            },
            FieldInfoData {
                name: "EntitlementConfig",
                name_hash: 70497366,
                flags: MemberInfoFlags::new(0),
                field_type: "EntitlementConfigData",
                rust_offset: offset_of!(EntitlementSettingsAsset, entitlement_config),
            },
            FieldInfoData {
                name: "EntitlementOriginConfig",
                name_hash: 3587774850,
                flags: MemberInfoFlags::new(0),
                field_type: "EntitlementOriginConfigData",
                rust_offset: offset_of!(EntitlementSettingsAsset, entitlement_origin_config),
            },
            FieldInfoData {
                name: "EntitlementsInfoList",
                name_hash: 2121593731,
                flags: MemberInfoFlags::new(144),
                field_type: "EntitlementInfo-Array",
                rust_offset: offset_of!(EntitlementSettingsAsset, entitlements_info_list),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTSETTINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementSettingsAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTSETTINGSASSET_TYPE_INFO
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


pub static ENTITLEMENTSETTINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettingsAsset-Array",
    name_hash: 1193132157,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementSettingsAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementPlatformToProjectId {
    pub platform: super::core::GamePlatform,
    pub project_id: String,
}

pub trait EntitlementPlatformToProjectIdTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn project_id(&self) -> &String;
    fn project_id_mut(&mut self) -> &mut String;
}

impl EntitlementPlatformToProjectIdTrait for EntitlementPlatformToProjectId {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn project_id(&self) -> &String {
        &self.project_id
    }
    fn project_id_mut(&mut self) -> &mut String {
        &mut self.project_id
    }
}

pub static ENTITLEMENTPLATFORMTOPROJECTID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementPlatformToProjectId",
    name_hash: 3491867680,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementPlatformToProjectId as Default>::default())),
            create_boxed: || Box::new(<EntitlementPlatformToProjectId as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(EntitlementPlatformToProjectId, platform),
            },
            FieldInfoData {
                name: "ProjectId",
                name_hash: 4152360413,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntitlementPlatformToProjectId, project_id),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTPLATFORMTOPROJECTID_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementPlatformToProjectId {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTPLATFORMTOPROJECTID_TYPE_INFO
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


pub static ENTITLEMENTPLATFORMTOPROJECTID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementPlatformToProjectId-Array",
    name_hash: 1022407060,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementPlatformToProjectId"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementConfigData {
    pub groups: Vec<BoxedTypeObject /* EntitlementGroup */>,
    pub page_size: u32,
}

pub trait EntitlementConfigDataTrait: TypeObject {
    fn groups(&self) -> &Vec<BoxedTypeObject /* EntitlementGroup */>;
    fn groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EntitlementGroup */>;
    fn page_size(&self) -> &u32;
    fn page_size_mut(&mut self) -> &mut u32;
}

impl EntitlementConfigDataTrait for EntitlementConfigData {
    fn groups(&self) -> &Vec<BoxedTypeObject /* EntitlementGroup */> {
        &self.groups
    }
    fn groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EntitlementGroup */> {
        &mut self.groups
    }
    fn page_size(&self) -> &u32 {
        &self.page_size
    }
    fn page_size_mut(&mut self) -> &mut u32 {
        &mut self.page_size
    }
}

pub static ENTITLEMENTCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementConfigData",
    name_hash: 2770627942,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementConfigData as Default>::default())),
            create_boxed: || Box::new(<EntitlementConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Groups",
                name_hash: 2573340009,
                flags: MemberInfoFlags::new(144),
                field_type: "EntitlementGroup-Array",
                rust_offset: offset_of!(EntitlementConfigData, groups),
            },
            FieldInfoData {
                name: "PageSize",
                name_hash: 4162799635,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntitlementConfigData, page_size),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTCONFIGDATA_TYPE_INFO
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


pub static ENTITLEMENTCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementConfigData-Array",
    name_hash: 3824634450,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementOriginConfigData {
    pub groups: Vec<BoxedTypeObject /* EntitlementGroup */>,
    pub tag_name: String,
}

pub trait EntitlementOriginConfigDataTrait: TypeObject {
    fn groups(&self) -> &Vec<BoxedTypeObject /* EntitlementGroup */>;
    fn groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EntitlementGroup */>;
    fn tag_name(&self) -> &String;
    fn tag_name_mut(&mut self) -> &mut String;
}

impl EntitlementOriginConfigDataTrait for EntitlementOriginConfigData {
    fn groups(&self) -> &Vec<BoxedTypeObject /* EntitlementGroup */> {
        &self.groups
    }
    fn groups_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EntitlementGroup */> {
        &mut self.groups
    }
    fn tag_name(&self) -> &String {
        &self.tag_name
    }
    fn tag_name_mut(&mut self) -> &mut String {
        &mut self.tag_name
    }
}

pub static ENTITLEMENTORIGINCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementOriginConfigData",
    name_hash: 3888950706,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementOriginConfigData as Default>::default())),
            create_boxed: || Box::new(<EntitlementOriginConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Groups",
                name_hash: 2573340009,
                flags: MemberInfoFlags::new(144),
                field_type: "EntitlementGroup-Array",
                rust_offset: offset_of!(EntitlementOriginConfigData, groups),
            },
            FieldInfoData {
                name: "TagName",
                name_hash: 3040259344,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntitlementOriginConfigData, tag_name),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTORIGINCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementOriginConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTORIGINCONFIGDATA_TYPE_INFO
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


pub static ENTITLEMENTORIGINCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementOriginConfigData-Array",
    name_hash: 167908358,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementOriginConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementGroup {
    pub platform: super::core::GamePlatform,
    pub group_name: String,
}

pub trait EntitlementGroupTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn group_name(&self) -> &String;
    fn group_name_mut(&mut self) -> &mut String;
}

impl EntitlementGroupTrait for EntitlementGroup {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn group_name(&self) -> &String {
        &self.group_name
    }
    fn group_name_mut(&mut self) -> &mut String {
        &mut self.group_name
    }
}

pub static ENTITLEMENTGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementGroup",
    name_hash: 2470050819,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementGroup as Default>::default())),
            create_boxed: || Box::new(<EntitlementGroup as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(EntitlementGroup, platform),
            },
            FieldInfoData {
                name: "GroupName",
                name_hash: 3178263901,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntitlementGroup, group_name),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementGroup {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTGROUP_TYPE_INFO
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


pub static ENTITLEMENTGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementGroup-Array",
    name_hash: 1751707319,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EntitlementInfo {
    pub platform: super::core::GamePlatform,
    pub entitlement_tag: String,
    pub group_name: String,
    pub product_id: String,
    pub project_id: String,
}

pub trait EntitlementInfoTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn entitlement_tag(&self) -> &String;
    fn entitlement_tag_mut(&mut self) -> &mut String;
    fn group_name(&self) -> &String;
    fn group_name_mut(&mut self) -> &mut String;
    fn product_id(&self) -> &String;
    fn product_id_mut(&mut self) -> &mut String;
    fn project_id(&self) -> &String;
    fn project_id_mut(&mut self) -> &mut String;
}

impl EntitlementInfoTrait for EntitlementInfo {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn entitlement_tag(&self) -> &String {
        &self.entitlement_tag
    }
    fn entitlement_tag_mut(&mut self) -> &mut String {
        &mut self.entitlement_tag
    }
    fn group_name(&self) -> &String {
        &self.group_name
    }
    fn group_name_mut(&mut self) -> &mut String {
        &mut self.group_name
    }
    fn product_id(&self) -> &String {
        &self.product_id
    }
    fn product_id_mut(&mut self) -> &mut String {
        &mut self.product_id
    }
    fn project_id(&self) -> &String {
        &self.project_id
    }
    fn project_id_mut(&mut self) -> &mut String {
        &mut self.project_id
    }
}

pub static ENTITLEMENTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementInfo",
    name_hash: 3068262194,
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntitlementInfo as Default>::default())),
            create_boxed: || Box::new(<EntitlementInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(EntitlementInfo, platform),
            },
            FieldInfoData {
                name: "EntitlementTag",
                name_hash: 1654763342,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntitlementInfo, entitlement_tag),
            },
            FieldInfoData {
                name: "GroupName",
                name_hash: 3178263901,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntitlementInfo, group_name),
            },
            FieldInfoData {
                name: "ProductId",
                name_hash: 106414083,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntitlementInfo, product_id),
            },
            FieldInfoData {
                name: "ProjectId",
                name_hash: 4152360413,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EntitlementInfo, project_id),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementInfo {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITLEMENTINFO_TYPE_INFO
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


pub static ENTITLEMENTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementInfo-Array",
    name_hash: 3980950918,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceEntitlementsServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
}

pub trait PresenceEntitlementsServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
}

impl PresenceEntitlementsServiceDataTrait for PresenceEntitlementsServiceData {
}

impl super::online_shared::PresenceServiceDataTrait for PresenceEntitlementsServiceData {
}

impl super::core::AssetTrait for PresenceEntitlementsServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceEntitlementsServiceData {
}

pub static PRESENCEENTITLEMENTSSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEntitlementsServiceData",
    name_hash: 1135850427,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceEntitlementsServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceEntitlementsServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceEntitlementsServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEENTITLEMENTSSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceEntitlementsServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEENTITLEMENTSSERVICEDATA_TYPE_INFO
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


pub static PRESENCEENTITLEMENTSSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEntitlementsServiceData-Array",
    name_hash: 1810643727,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceEntitlementsServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEntitlementsBackend {
    pub _glacier_base: super::online::PresenceBackend,
}

pub trait ServerEntitlementsBackendTrait: super::online::PresenceBackendTrait {
}

impl ServerEntitlementsBackendTrait for ServerEntitlementsBackend {
}

impl super::online::PresenceBackendTrait for ServerEntitlementsBackend {
}

pub static SERVERENTITLEMENTSBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntitlementsBackend",
    name_hash: 1122433390,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(ServerEntitlementsBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEntitlementsBackend as Default>::default())),
            create_boxed: || Box::new(<ServerEntitlementsBackend as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERENTITLEMENTSBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEntitlementsBackend {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERENTITLEMENTSBACKEND_TYPE_INFO
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


pub static SERVERENTITLEMENTSBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntitlementsBackend-Array",
    name_hash: 1727148634,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("ServerEntitlementsBackend"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceGetNucleusEntitlementsRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceGetNucleusEntitlementsRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceGetNucleusEntitlementsRequestParametersTrait for PresenceGetNucleusEntitlementsRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceGetNucleusEntitlementsRequestParameters {
}

pub static PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNucleusEntitlementsRequestParameters",
    name_hash: 2161422908,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetNucleusEntitlementsRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetNucleusEntitlementsRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetNucleusEntitlementsRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetNucleusEntitlementsRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNucleusEntitlementsRequestParameters-Array",
    name_hash: 2023885192,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGetNucleusEntitlementsRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceGrantNucleusEntitlementRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceGrantNucleusEntitlementRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceGrantNucleusEntitlementRequestParametersTrait for PresenceGrantNucleusEntitlementRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceGrantNucleusEntitlementRequestParameters {
}

pub static PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGrantNucleusEntitlementRequestParameters",
    name_hash: 809744599,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGrantNucleusEntitlementRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGrantNucleusEntitlementRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGrantNucleusEntitlementRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGrantNucleusEntitlementRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGrantNucleusEntitlementRequestParameters-Array",
    name_hash: 1342140899,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGrantNucleusEntitlementRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceGetOriginEntitlementsRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceGetOriginEntitlementsRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceGetOriginEntitlementsRequestParametersTrait for PresenceGetOriginEntitlementsRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceGetOriginEntitlementsRequestParameters {
}

pub static PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetOriginEntitlementsRequestParameters",
    name_hash: 1989754271,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetOriginEntitlementsRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetOriginEntitlementsRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetOriginEntitlementsRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetOriginEntitlementsRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetOriginEntitlementsRequestParameters-Array",
    name_hash: 1357193771,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGetOriginEntitlementsRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceGetFirstPartyEntitlementsRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceGetFirstPartyEntitlementsRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceGetFirstPartyEntitlementsRequestParametersTrait for PresenceGetFirstPartyEntitlementsRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceGetFirstPartyEntitlementsRequestParameters {
}

pub static PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetFirstPartyEntitlementsRequestParameters",
    name_hash: 11595711,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetFirstPartyEntitlementsRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetFirstPartyEntitlementsRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetFirstPartyEntitlementsRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetFirstPartyEntitlementsRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetFirstPartyEntitlementsRequestParameters-Array",
    name_hash: 4196803851,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGetFirstPartyEntitlementsRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LicenseMappingEvent {
    pub _glacier_base: super::online::PresenceEvent,
}

pub trait LicenseMappingEventTrait: super::online::PresenceEventTrait {
}

impl LicenseMappingEventTrait for LicenseMappingEvent {
}

impl super::online::PresenceEventTrait for LicenseMappingEvent {
}

pub static LICENSEMAPPINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseMappingEvent",
    name_hash: 1535362366,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(LicenseMappingEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LicenseMappingEvent as Default>::default())),
            create_boxed: || Box::new(<LicenseMappingEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LICENSEMAPPINGEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LicenseMappingEvent {
    fn type_info(&self) -> &'static TypeInfo {
        LICENSEMAPPINGEVENT_TYPE_INFO
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


pub static LICENSEMAPPINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseMappingEvent-Array",
    name_hash: 2269894026,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("LicenseMappingEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientEntitlementsService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientEntitlementsServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientEntitlementsServiceTrait for ClientEntitlementsService {
}

impl super::online::PresenceServiceTrait for ClientEntitlementsService {
}

pub static CLIENTENTITLEMENTSSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsService",
    name_hash: 1888291115,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientEntitlementsService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEntitlementsService as Default>::default())),
            create_boxed: || Box::new(<ClientEntitlementsService as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTITLEMENTSSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntitlementsService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENTITLEMENTSSERVICE_TYPE_INFO
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


pub static CLIENTENTITLEMENTSSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsService-Array",
    name_hash: 3678202015,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("ClientEntitlementsService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientEntitlementsBackend {
    pub _glacier_base: super::online::PresenceBackend,
}

pub trait ClientEntitlementsBackendTrait: super::online::PresenceBackendTrait {
}

impl ClientEntitlementsBackendTrait for ClientEntitlementsBackend {
}

impl super::online::PresenceBackendTrait for ClientEntitlementsBackend {
}

pub static CLIENTENTITLEMENTSBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsBackend",
    name_hash: 1233271858,
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(ClientEntitlementsBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEntitlementsBackend as Default>::default())),
            create_boxed: || Box::new(<ClientEntitlementsBackend as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTITLEMENTSBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntitlementsBackend {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTENTITLEMENTSBACKEND_TYPE_INFO
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


pub static CLIENTENTITLEMENTSBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsBackend-Array",
    name_hash: 1783136902,
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("ClientEntitlementsBackend"),
    array_type: None,
    alignment: 8,
};


