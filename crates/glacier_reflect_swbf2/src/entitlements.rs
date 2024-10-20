use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceLicenseRequestMessageBase {
}

pub const PRESENCELICENSEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLicenseRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLicenseRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCELICENSEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceLicenseMessageBase {
}

pub const PRESENCELICENSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLicenseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLicenseMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCELICENSEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LicenseConfiguration {
    pub licenses: Vec<LicenseInfo>,
}

pub const LICENSECONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseConfiguration",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Licenses",
                flags: MemberInfoFlags::new(144),
                field_type: LICENSEINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LicenseConfiguration, licenses),
            },
        ],
    }),
    array_type: Some(LICENSECONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LicenseConfiguration {
    fn type_info() -> &'static TypeInfo {
        LICENSECONFIGURATION_TYPE_INFO
    }
}


pub const LICENSECONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("LicenseConfiguration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LicenseInfo {
    pub name: String,
    pub platform: super::core::GamePlatform,
    pub description: String,
    pub content_entitlements: Vec<String>,
    pub nucleus_entitlements: Vec<NucleusEntitlementInfo>,
    pub first_party_entitlements: Vec<String>,
}

pub const LICENSEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseInfo",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LicenseInfo, name),
            },
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(LicenseInfo, platform),
            },
            FieldInfoData {
                name: "Description",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LicenseInfo, description),
            },
            FieldInfoData {
                name: "ContentEntitlements",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LicenseInfo, content_entitlements),
            },
            FieldInfoData {
                name: "NucleusEntitlements",
                flags: MemberInfoFlags::new(144),
                field_type: NUCLEUSENTITLEMENTINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LicenseInfo, nucleus_entitlements),
            },
            FieldInfoData {
                name: "FirstPartyEntitlements",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LicenseInfo, first_party_entitlements),
            },
        ],
    }),
    array_type: Some(LICENSEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LicenseInfo {
    fn type_info() -> &'static TypeInfo {
        LICENSEINFO_TYPE_INFO
    }
}


pub const LICENSEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("LicenseInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusEntitlementInfo {
    pub tag: String,
    pub group_name: String,
}

pub const NUCLEUSENTITLEMENTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusEntitlementInfo",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Tag",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NucleusEntitlementInfo, tag),
            },
            FieldInfoData {
                name: "GroupName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NucleusEntitlementInfo, group_name),
            },
        ],
    }),
    array_type: Some(NUCLEUSENTITLEMENTINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NucleusEntitlementInfo {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSENTITLEMENTINFO_TYPE_INFO
    }
}


pub const NUCLEUSENTITLEMENTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusEntitlementInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("NucleusEntitlementInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementsServerBackendData {
}

pub const ENTITLEMENTSSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsServerBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITLEMENTSSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementsServerBackendData {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTSSERVERBACKENDDATA_TYPE_INFO
    }
}


pub const ENTITLEMENTSSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsServerBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementsServerBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementsBackendData {
}

pub const ENTITLEMENTSBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITLEMENTSBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementsBackendData {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTSBACKENDDATA_TYPE_INFO
    }
}


pub const ENTITLEMENTSBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementsBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementsBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementSettings {
    pub settings: EntitlementSettingsAsset,
}

pub const ENTITLEMENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettings",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITLEMENTSETTINGSASSET_TYPE_INFO,
                rust_offset: offset_of!(EntitlementSettings, settings),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementSettings {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTSETTINGS_TYPE_INFO
    }
}


pub const ENTITLEMENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementSettingsAsset {
    pub license_config: LicenseConfiguration,
    pub entitlement_config: EntitlementConfigData,
    pub entitlement_origin_config: EntitlementOriginConfigData,
    pub entitlements_info_list: Vec<EntitlementInfo>,
}

pub const ENTITLEMENTSETTINGSASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettingsAsset",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LicenseConfig",
                flags: MemberInfoFlags::new(0),
                field_type: LICENSECONFIGURATION_TYPE_INFO,
                rust_offset: offset_of!(EntitlementSettingsAsset, license_config),
            },
            FieldInfoData {
                name: "EntitlementConfig",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITLEMENTCONFIGDATA_TYPE_INFO,
                rust_offset: offset_of!(EntitlementSettingsAsset, entitlement_config),
            },
            FieldInfoData {
                name: "EntitlementOriginConfig",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITLEMENTORIGINCONFIGDATA_TYPE_INFO,
                rust_offset: offset_of!(EntitlementSettingsAsset, entitlement_origin_config),
            },
            FieldInfoData {
                name: "EntitlementsInfoList",
                flags: MemberInfoFlags::new(144),
                field_type: ENTITLEMENTINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EntitlementSettingsAsset, entitlements_info_list),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTSETTINGSASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementSettingsAsset {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTSETTINGSASSET_TYPE_INFO
    }
}


pub const ENTITLEMENTSETTINGSASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementSettingsAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementSettingsAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementPlatformToProjectId {
    pub platform: super::core::GamePlatform,
    pub project_id: String,
}

pub const ENTITLEMENTPLATFORMTOPROJECTID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementPlatformToProjectId",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(EntitlementPlatformToProjectId, platform),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntitlementPlatformToProjectId, project_id),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTPLATFORMTOPROJECTID_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementPlatformToProjectId {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTPLATFORMTOPROJECTID_TYPE_INFO
    }
}


pub const ENTITLEMENTPLATFORMTOPROJECTID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementPlatformToProjectId-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementPlatformToProjectId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementConfigData {
    pub groups: Vec<EntitlementGroup>,
    pub page_size: u32,
}

pub const ENTITLEMENTCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementConfigData",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Groups",
                flags: MemberInfoFlags::new(144),
                field_type: ENTITLEMENTGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EntitlementConfigData, groups),
            },
            FieldInfoData {
                name: "PageSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EntitlementConfigData, page_size),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementConfigData {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTCONFIGDATA_TYPE_INFO
    }
}


pub const ENTITLEMENTCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementOriginConfigData {
    pub groups: Vec<EntitlementGroup>,
    pub tag_name: String,
}

pub const ENTITLEMENTORIGINCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementOriginConfigData",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Groups",
                flags: MemberInfoFlags::new(144),
                field_type: ENTITLEMENTGROUP_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EntitlementOriginConfigData, groups),
            },
            FieldInfoData {
                name: "TagName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntitlementOriginConfigData, tag_name),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTORIGINCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementOriginConfigData {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTORIGINCONFIGDATA_TYPE_INFO
    }
}


pub const ENTITLEMENTORIGINCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementOriginConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementOriginConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementGroup {
    pub platform: super::core::GamePlatform,
    pub group_name: String,
}

pub const ENTITLEMENTGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementGroup",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(EntitlementGroup, platform),
            },
            FieldInfoData {
                name: "GroupName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntitlementGroup, group_name),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementGroup {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTGROUP_TYPE_INFO
    }
}


pub const ENTITLEMENTGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitlementInfo {
    pub platform: super::core::GamePlatform,
    pub entitlement_tag: String,
    pub group_name: String,
    pub product_id: String,
    pub project_id: String,
}

pub const ENTITLEMENTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementInfo",
    flags: MemberInfoFlags::new(73),
    module: "Entitlements",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(EntitlementInfo, platform),
            },
            FieldInfoData {
                name: "EntitlementTag",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntitlementInfo, entitlement_tag),
            },
            FieldInfoData {
                name: "GroupName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntitlementInfo, group_name),
            },
            FieldInfoData {
                name: "ProductId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntitlementInfo, product_id),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EntitlementInfo, project_id),
            },
        ],
    }),
    array_type: Some(ENTITLEMENTINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntitlementInfo {
    fn type_info() -> &'static TypeInfo {
        ENTITLEMENTINFO_TYPE_INFO
    }
}


pub const ENTITLEMENTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntitlementInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("EntitlementInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceEntitlementsServiceData {
}

pub const PRESENCEENTITLEMENTSSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEntitlementsServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEENTITLEMENTSSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceEntitlementsServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEENTITLEMENTSSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEENTITLEMENTSSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceEntitlementsServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceEntitlementsServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEntitlementsBackend {
}

pub const SERVERENTITLEMENTSBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntitlementsBackend",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERENTITLEMENTSBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEntitlementsBackend {
    fn type_info() -> &'static TypeInfo {
        SERVERENTITLEMENTSBACKEND_TYPE_INFO
    }
}


pub const SERVERENTITLEMENTSBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEntitlementsBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("ServerEntitlementsBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetNucleusEntitlementsRequestParameters {
}

pub const PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNucleusEntitlementsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetNucleusEntitlementsRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETNUCLEUSENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetNucleusEntitlementsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGetNucleusEntitlementsRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGrantNucleusEntitlementRequestParameters {
}

pub const PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGrantNucleusEntitlementRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGrantNucleusEntitlementRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGRANTNUCLEUSENTITLEMENTREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGrantNucleusEntitlementRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGrantNucleusEntitlementRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetOriginEntitlementsRequestParameters {
}

pub const PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetOriginEntitlementsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetOriginEntitlementsRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETORIGINENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetOriginEntitlementsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGetOriginEntitlementsRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetFirstPartyEntitlementsRequestParameters {
}

pub const PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetFirstPartyEntitlementsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetFirstPartyEntitlementsRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETFIRSTPARTYENTITLEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetFirstPartyEntitlementsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("PresenceGetFirstPartyEntitlementsRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LicenseMappingEvent {
}

pub const LICENSEMAPPINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseMappingEvent",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LICENSEMAPPINGEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LicenseMappingEvent {
    fn type_info() -> &'static TypeInfo {
        LICENSEMAPPINGEVENT_TYPE_INFO
    }
}


pub const LICENSEMAPPINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseMappingEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("LicenseMappingEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEntitlementsService {
}

pub const CLIENTENTITLEMENTSSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsService",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTITLEMENTSSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntitlementsService {
    fn type_info() -> &'static TypeInfo {
        CLIENTENTITLEMENTSSERVICE_TYPE_INFO
    }
}


pub const CLIENTENTITLEMENTSSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("ClientEntitlementsService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEntitlementsBackend {
}

pub const CLIENTENTITLEMENTSBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsBackend",
    flags: MemberInfoFlags::new(101),
    module: "Entitlements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKEND_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTENTITLEMENTSBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEntitlementsBackend {
    fn type_info() -> &'static TypeInfo {
        CLIENTENTITLEMENTSBACKEND_TYPE_INFO
    }
}


pub const CLIENTENTITLEMENTSBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEntitlementsBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Entitlements",
    data: TypeInfoData::Array("ClientEntitlementsBackend-Array"),
    array_type: None,
    alignment: 8,
};


