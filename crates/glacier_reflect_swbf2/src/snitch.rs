use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SnitchSettings {
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub data_providers: Vec<super::web_utils::URLConfigData>,
    pub editorial_config_enabled: bool,
}

pub const SNITCHSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnitchSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SnitchSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SnitchSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SnitchSettings, s_s_l),
            },
            FieldInfoData {
                name: "DataProviders",
                flags: MemberInfoFlags::new(144),
                field_type: URLCONFIGDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SnitchSettings, data_providers),
            },
            FieldInfoData {
                name: "EditorialConfigEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SnitchSettings, editorial_config_enabled),
            },
        ],
    }),
    array_type: Some(SNITCHSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SnitchSettings {
    fn type_info() -> &'static TypeInfo {
        SNITCHSETTINGS_TYPE_INFO
    }
}


pub const SNITCHSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnitchSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("SnitchSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LiveScoreboardProviderSettings {
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub snapshot_refresh_frequency: f32,
    pub gzip_compression: bool,
    pub rollout_modulo: i32,
}

pub const LIVESCOREBOARDPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LiveScoreboardProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LiveScoreboardProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LiveScoreboardProviderSettings, s_s_l),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LiveScoreboardProviderSettings, snapshot_refresh_frequency),
            },
            FieldInfoData {
                name: "GzipCompression",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LiveScoreboardProviderSettings, gzip_compression),
            },
            FieldInfoData {
                name: "RolloutModulo",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LiveScoreboardProviderSettings, rollout_modulo),
            },
        ],
    }),
    array_type: Some(LIVESCOREBOARDPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LiveScoreboardProviderSettings {
    fn type_info() -> &'static TypeInfo {
        LIVESCOREBOARDPROVIDERSETTINGS_TYPE_INFO
    }
}


pub const LIVESCOREBOARDPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("LiveScoreboardProviderSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DistroProviderSettings {
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub snapshot_refresh_frequency: f32,
    pub gzip_compression: bool,
}

pub const DISTROPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistroProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistroProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DistroProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistroProviderSettings, s_s_l),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DistroProviderSettings, snapshot_refresh_frequency),
            },
            FieldInfoData {
                name: "GzipCompression",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DistroProviderSettings, gzip_compression),
            },
        ],
    }),
    array_type: Some(DISTROPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DistroProviderSettings {
    fn type_info() -> &'static TypeInfo {
        DISTROPROVIDERSETTINGS_TYPE_INFO
    }
}


pub const DISTROPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistroProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("DistroProviderSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StatsDProviderSettings {
    pub enabled: bool,
    pub url: String,
    pub snapshot_refresh_frequency: f32,
}

pub const STATSDPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatsDProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StatsDProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StatsDProviderSettings, url),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StatsDProviderSettings, snapshot_refresh_frequency),
            },
        ],
    }),
    array_type: Some(STATSDPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StatsDProviderSettings {
    fn type_info() -> &'static TypeInfo {
        STATSDPROVIDERSETTINGS_TYPE_INFO
    }
}


pub const STATSDPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StatsDProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("StatsDProviderSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ContactProviderSettings {
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
    pub snapshot_refresh_frequency: f32,
    pub gzip_compression: bool,
}

pub const CONTACTPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContactProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ContactProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ContactProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ContactProviderSettings, s_s_l),
            },
            FieldInfoData {
                name: "SnapshotRefreshFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ContactProviderSettings, snapshot_refresh_frequency),
            },
            FieldInfoData {
                name: "GzipCompression",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ContactProviderSettings, gzip_compression),
            },
        ],
    }),
    array_type: Some(CONTACTPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ContactProviderSettings {
    fn type_info() -> &'static TypeInfo {
        CONTACTPROVIDERSETTINGS_TYPE_INFO
    }
}


pub const CONTACTPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContactProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("ContactProviderSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogTransmitterProviderSettings {
    pub enabled: bool,
    pub url: String,
    pub s_s_l: bool,
}

pub const LOGTRANSMITTERPROVIDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogTransmitterProviderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Snitch",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogTransmitterProviderSettings, enabled),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LogTransmitterProviderSettings, url),
            },
            FieldInfoData {
                name: "SSL",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LogTransmitterProviderSettings, s_s_l),
            },
        ],
    }),
    array_type: Some(LOGTRANSMITTERPROVIDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogTransmitterProviderSettings {
    fn type_info() -> &'static TypeInfo {
        LOGTRANSMITTERPROVIDERSETTINGS_TYPE_INFO
    }
}


pub const LOGTRANSMITTERPROVIDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogTransmitterProviderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Snitch",
    data: TypeInfoData::Array("LogTransmitterProviderSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LiveScoreboardProviderDisableMessage {
}

pub const LIVESCOREBOARDPROVIDERDISABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderDisableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snitch",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LiveScoreboardProviderDisableMessage {
    fn type_info() -> &'static TypeInfo {
        LIVESCOREBOARDPROVIDERDISABLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LiveScoreboardProviderEnableMessage {
}

pub const LIVESCOREBOARDPROVIDERENABLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveScoreboardProviderEnableMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snitch",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LiveScoreboardProviderEnableMessage {
    fn type_info() -> &'static TypeInfo {
        LIVESCOREBOARDPROVIDERENABLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MetricsProviderStringMetricMessage {
}

pub const METRICSPROVIDERSTRINGMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderStringMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderStringMetricMessage {
    fn type_info() -> &'static TypeInfo {
        METRICSPROVIDERSTRINGMETRICMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MetricsProviderCounterMetricMessage {
}

pub const METRICSPROVIDERCOUNTERMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderCounterMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderCounterMetricMessage {
    fn type_info() -> &'static TypeInfo {
        METRICSPROVIDERCOUNTERMETRICMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MetricsProviderGaugeMetricMessage {
}

pub const METRICSPROVIDERGAUGEMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderGaugeMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderGaugeMetricMessage {
    fn type_info() -> &'static TypeInfo {
        METRICSPROVIDERGAUGEMETRICMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MetricsProviderTagMetricMessage {
}

pub const METRICSPROVIDERTAGMETRICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MetricsProviderTagMetricMessage",
    flags: MemberInfoFlags::new(73),
    module: "Snitch",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MetricsProviderTagMetricMessage {
    fn type_info() -> &'static TypeInfo {
        METRICSPROVIDERTAGMETRICMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SnitchSettingsUpdatedMessage {
}

pub const SNITCHSETTINGSUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SnitchSettingsUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Snitch",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SnitchSettingsUpdatedMessage {
    fn type_info() -> &'static TypeInfo {
        SNITCHSETTINGSUPDATEDMESSAGE_TYPE_INFO
    }
}

