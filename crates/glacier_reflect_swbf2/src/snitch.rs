use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_snitch_types(registry: &mut TypeRegistry) {
    registry.register_type(SNITCHSETTINGS_TYPE_INFO);
    registry.register_type(SNITCHSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LIVESCOREBOARDPROVIDERSETTINGS_TYPE_INFO);
    registry.register_type(LIVESCOREBOARDPROVIDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DISTROPROVIDERSETTINGS_TYPE_INFO);
    registry.register_type(DISTROPROVIDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STATSDPROVIDERSETTINGS_TYPE_INFO);
    registry.register_type(STATSDPROVIDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CONTACTPROVIDERSETTINGS_TYPE_INFO);
    registry.register_type(CONTACTPROVIDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOGTRANSMITTERPROVIDERSETTINGS_TYPE_INFO);
    registry.register_type(LOGTRANSMITTERPROVIDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LIVESCOREBOARDPROVIDERDISABLEMESSAGE_TYPE_INFO);
    registry.register_type(LIVESCOREBOARDPROVIDERENABLEMESSAGE_TYPE_INFO);
    registry.register_type(METRICSPROVIDERSTRINGMETRICMESSAGE_TYPE_INFO);
    registry.register_type(METRICSPROVIDERCOUNTERMETRICMESSAGE_TYPE_INFO);
    registry.register_type(METRICSPROVIDERGAUGEMETRICMESSAGE_TYPE_INFO);
    registry.register_type(METRICSPROVIDERTAGMETRICMESSAGE_TYPE_INFO);
    registry.register_type(SNITCHSETTINGSUPDATEDMESSAGE_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct SnitchSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub data_providers: Vec<Option<Arc<Mutex<dyn super::web_utils::URLConfigDataTrait>>>>,
    pub editorial_config_enabled: bool,
}

pub trait SnitchSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn s_s_l(&self) -> &bool;
    fn s_s_l_mut(&mut self) -> &mut bool;
    fn data_providers(&self) -> &Vec<Option<Arc<Mutex<dyn super::web_utils::URLConfigDataTrait>>>>;
    fn data_providers_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::web_utils::URLConfigDataTrait>>>>;
    fn editorial_config_enabled(&self) -> &bool;
    fn editorial_config_enabled_mut(&mut self) -> &mut bool;
}

impl SnitchSettingsTrait for SnitchSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn s_s_l(&self) -> &bool {
        &self.s_s_l
    }
    fn s_s_l_mut(&mut self) -> &mut bool {
        &mut self.s_s_l
    }
    fn data_providers(&self) -> &Vec<Option<Arc<Mutex<dyn super::web_utils::URLConfigDataTrait>>>> {
        &self.data_providers
    }
    fn data_providers_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::web_utils::URLConfigDataTrait>>>> {
        &mut self.data_providers
    }
    fn editorial_config_enabled(&self) -> &bool {
        &self.editorial_config_enabled
    }
    fn editorial_config_enabled_mut(&mut self) -> &mut bool {
        &mut self.editorial_config_enabled
    }
}

impl super::core::SystemSettingsTrait for SnitchSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for SnitchSettings {
}

pub static SNITCHSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnitchSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnitchSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SnitchSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(SnitchSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SnitchSettings, s_s_l),
            },
            FieldInfoData {
                name: "DataProviders",
                flags: MemberInfoFlags::new(144),
                field_type: "URLConfigData-Array",
                rust_offset: offset_of!(SnitchSettings, data_providers),
            },
            FieldInfoData {
                name: "EditorialConfigEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SnitchSettings, editorial_config_enabled),
            },
        ],
    }),
    array_type: Some(SNITCHSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnitchSettings {
    fn type_info(&self) -> &'static TypeInfo {
        SNITCHSETTINGS_TYPE_INFO
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


pub static SNITCHSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnitchSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("SnitchSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LiveScoreboardProviderSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub snapshot_refresh_frequency: f32,
    pub gzip_compression: bool,
    pub rollout_modulo: i32,
}

pub trait LiveScoreboardProviderSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn s_s_l(&self) -> &bool;
    fn s_s_l_mut(&mut self) -> &mut bool;
    fn snapshot_refresh_frequency(&self) -> &f32;
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32;
    fn gzip_compression(&self) -> &bool;
    fn gzip_compression_mut(&mut self) -> &mut bool;
    fn rollout_modulo(&self) -> &i32;
    fn rollout_modulo_mut(&mut self) -> &mut i32;
}

impl LiveScoreboardProviderSettingsTrait for LiveScoreboardProviderSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn s_s_l(&self) -> &bool {
        &self.s_s_l
    }
    fn s_s_l_mut(&mut self) -> &mut bool {
        &mut self.s_s_l
    }
    fn snapshot_refresh_frequency(&self) -> &f32 {
        &self.snapshot_refresh_frequency
    }
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32 {
        &mut self.snapshot_refresh_frequency
    }
    fn gzip_compression(&self) -> &bool {
        &self.gzip_compression
    }
    fn gzip_compression_mut(&mut self) -> &mut bool {
        &mut self.gzip_compression
    }
    fn rollout_modulo(&self) -> &i32 {
        &self.rollout_modulo
    }
    fn rollout_modulo_mut(&mut self) -> &mut i32 {
        &mut self.rollout_modulo
    }
}

impl super::core::SystemSettingsTrait for LiveScoreboardProviderSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for LiveScoreboardProviderSettings {
}

pub static LIVESCOREBOARDPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LiveScoreboardProviderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LiveScoreboardProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LiveScoreboardProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LiveScoreboardProviderSettings, s_s_l),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LiveScoreboardProviderSettings, snapshot_refresh_frequency),
            },
            FieldInfoData {
                name: "GzipCompression",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LiveScoreboardProviderSettings, gzip_compression),
            },
            FieldInfoData {
                name: "RolloutModulo",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LiveScoreboardProviderSettings, rollout_modulo),
            },
        ],
    }),
    array_type: Some(LIVESCOREBOARDPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LiveScoreboardProviderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LIVESCOREBOARDPROVIDERSETTINGS_TYPE_INFO
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


pub static LIVESCOREBOARDPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("LiveScoreboardProviderSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DistroProviderSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub snapshot_refresh_frequency: f32,
    pub gzip_compression: bool,
}

pub trait DistroProviderSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn s_s_l(&self) -> &bool;
    fn s_s_l_mut(&mut self) -> &mut bool;
    fn snapshot_refresh_frequency(&self) -> &f32;
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32;
    fn gzip_compression(&self) -> &bool;
    fn gzip_compression_mut(&mut self) -> &mut bool;
}

impl DistroProviderSettingsTrait for DistroProviderSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn s_s_l(&self) -> &bool {
        &self.s_s_l
    }
    fn s_s_l_mut(&mut self) -> &mut bool {
        &mut self.s_s_l
    }
    fn snapshot_refresh_frequency(&self) -> &f32 {
        &self.snapshot_refresh_frequency
    }
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32 {
        &mut self.snapshot_refresh_frequency
    }
    fn gzip_compression(&self) -> &bool {
        &self.gzip_compression
    }
    fn gzip_compression_mut(&mut self) -> &mut bool {
        &mut self.gzip_compression
    }
}

impl super::core::SystemSettingsTrait for DistroProviderSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for DistroProviderSettings {
}

pub static DISTROPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistroProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DistroProviderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistroProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DistroProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistroProviderSettings, s_s_l),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DistroProviderSettings, snapshot_refresh_frequency),
            },
            FieldInfoData {
                name: "GzipCompression",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DistroProviderSettings, gzip_compression),
            },
        ],
    }),
    array_type: Some(DISTROPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistroProviderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DISTROPROVIDERSETTINGS_TYPE_INFO
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


pub static DISTROPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistroProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("DistroProviderSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StatsDProviderSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub url: String,
    pub snapshot_refresh_frequency: f32,
}

pub trait StatsDProviderSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn snapshot_refresh_frequency(&self) -> &f32;
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32;
}

impl StatsDProviderSettingsTrait for StatsDProviderSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn snapshot_refresh_frequency(&self) -> &f32 {
        &self.snapshot_refresh_frequency
    }
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32 {
        &mut self.snapshot_refresh_frequency
    }
}

impl super::core::SystemSettingsTrait for StatsDProviderSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for StatsDProviderSettings {
}

pub static STATSDPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatsDProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StatsDProviderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StatsDProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StatsDProviderSettings, url),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(StatsDProviderSettings, snapshot_refresh_frequency),
            },
        ],
    }),
    array_type: Some(STATSDPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StatsDProviderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        STATSDPROVIDERSETTINGS_TYPE_INFO
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


pub static STATSDPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatsDProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("StatsDProviderSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ContactProviderSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub snapshot_refresh_frequency: f32,
    pub gzip_compression: bool,
}

pub trait ContactProviderSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn s_s_l(&self) -> &bool;
    fn s_s_l_mut(&mut self) -> &mut bool;
    fn snapshot_refresh_frequency(&self) -> &f32;
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32;
    fn gzip_compression(&self) -> &bool;
    fn gzip_compression_mut(&mut self) -> &mut bool;
}

impl ContactProviderSettingsTrait for ContactProviderSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn s_s_l(&self) -> &bool {
        &self.s_s_l
    }
    fn s_s_l_mut(&mut self) -> &mut bool {
        &mut self.s_s_l
    }
    fn snapshot_refresh_frequency(&self) -> &f32 {
        &self.snapshot_refresh_frequency
    }
    fn snapshot_refresh_frequency_mut(&mut self) -> &mut f32 {
        &mut self.snapshot_refresh_frequency
    }
    fn gzip_compression(&self) -> &bool {
        &self.gzip_compression
    }
    fn gzip_compression_mut(&mut self) -> &mut bool {
        &mut self.gzip_compression
    }
}

impl super::core::SystemSettingsTrait for ContactProviderSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for ContactProviderSettings {
}

pub static CONTACTPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContactProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ContactProviderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ContactProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ContactProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ContactProviderSettings, s_s_l),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ContactProviderSettings, snapshot_refresh_frequency),
            },
            FieldInfoData {
                name: "GzipCompression",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ContactProviderSettings, gzip_compression),
            },
        ],
    }),
    array_type: Some(CONTACTPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ContactProviderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CONTACTPROVIDERSETTINGS_TYPE_INFO
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


pub static CONTACTPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContactProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("ContactProviderSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LogTransmitterProviderSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
}

pub trait LogTransmitterProviderSettingsTrait: super::core::SystemSettingsTrait {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn s_s_l(&self) -> &bool;
    fn s_s_l_mut(&mut self) -> &mut bool;
}

impl LogTransmitterProviderSettingsTrait for LogTransmitterProviderSettings {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn s_s_l(&self) -> &bool {
        &self.s_s_l
    }
    fn s_s_l_mut(&mut self) -> &mut bool {
        &mut self.s_s_l
    }
}

impl super::core::SystemSettingsTrait for LogTransmitterProviderSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for LogTransmitterProviderSettings {
}

pub static LOGTRANSMITTERPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogTransmitterProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogTransmitterProviderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogTransmitterProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LogTransmitterProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LogTransmitterProviderSettings, s_s_l),
            },
        ],
    }),
    array_type: Some(LOGTRANSMITTERPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogTransmitterProviderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LOGTRANSMITTERPROVIDERSETTINGS_TYPE_INFO
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


pub static LOGTRANSMITTERPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogTransmitterProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("LogTransmitterProviderSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LiveScoreboardProviderDisableMessage {
}

pub trait LiveScoreboardProviderDisableMessageTrait: TypeObject {
}

impl LiveScoreboardProviderDisableMessageTrait for LiveScoreboardProviderDisableMessage {
}

pub static LIVESCOREBOARDPROVIDERDISABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderDisableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snitch",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LiveScoreboardProviderDisableMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LiveScoreboardProviderDisableMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LIVESCOREBOARDPROVIDERDISABLEMESSAGE_TYPE_INFO
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
pub struct LiveScoreboardProviderEnableMessage {
}

pub trait LiveScoreboardProviderEnableMessageTrait: TypeObject {
}

impl LiveScoreboardProviderEnableMessageTrait for LiveScoreboardProviderEnableMessage {
}

pub static LIVESCOREBOARDPROVIDERENABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderEnableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snitch",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LiveScoreboardProviderEnableMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LiveScoreboardProviderEnableMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LIVESCOREBOARDPROVIDERENABLEMESSAGE_TYPE_INFO
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
pub struct MetricsProviderStringMetricMessage {
}

pub trait MetricsProviderStringMetricMessageTrait: TypeObject {
}

impl MetricsProviderStringMetricMessageTrait for MetricsProviderStringMetricMessage {
}

pub static METRICSPROVIDERSTRINGMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderStringMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MetricsProviderStringMetricMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderStringMetricMessage {
    fn type_info(&self) -> &'static TypeInfo {
        METRICSPROVIDERSTRINGMETRICMESSAGE_TYPE_INFO
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
pub struct MetricsProviderCounterMetricMessage {
}

pub trait MetricsProviderCounterMetricMessageTrait: TypeObject {
}

impl MetricsProviderCounterMetricMessageTrait for MetricsProviderCounterMetricMessage {
}

pub static METRICSPROVIDERCOUNTERMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderCounterMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MetricsProviderCounterMetricMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderCounterMetricMessage {
    fn type_info(&self) -> &'static TypeInfo {
        METRICSPROVIDERCOUNTERMETRICMESSAGE_TYPE_INFO
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
pub struct MetricsProviderGaugeMetricMessage {
}

pub trait MetricsProviderGaugeMetricMessageTrait: TypeObject {
}

impl MetricsProviderGaugeMetricMessageTrait for MetricsProviderGaugeMetricMessage {
}

pub static METRICSPROVIDERGAUGEMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderGaugeMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MetricsProviderGaugeMetricMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderGaugeMetricMessage {
    fn type_info(&self) -> &'static TypeInfo {
        METRICSPROVIDERGAUGEMETRICMESSAGE_TYPE_INFO
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
pub struct MetricsProviderTagMetricMessage {
}

pub trait MetricsProviderTagMetricMessageTrait: TypeObject {
}

impl MetricsProviderTagMetricMessageTrait for MetricsProviderTagMetricMessage {
}

pub static METRICSPROVIDERTAGMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderTagMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MetricsProviderTagMetricMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderTagMetricMessage {
    fn type_info(&self) -> &'static TypeInfo {
        METRICSPROVIDERTAGMETRICMESSAGE_TYPE_INFO
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
pub struct SnitchSettingsUpdatedMessage {
}

pub trait SnitchSettingsUpdatedMessageTrait: TypeObject {
}

impl SnitchSettingsUpdatedMessageTrait for SnitchSettingsUpdatedMessage {
}

pub static SNITCHSETTINGSUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnitchSettingsUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snitch",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SnitchSettingsUpdatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SnitchSettingsUpdatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SNITCHSETTINGSUPDATEDMESSAGE_TYPE_INFO
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

