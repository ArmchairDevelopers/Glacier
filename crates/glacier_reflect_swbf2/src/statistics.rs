use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_statistics_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCESTATISTICSREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCESTATISTICSMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCELEADERBOARDSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCELEADERBOARDSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCESTATISTICSSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCESTATISTICSSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEGETLEADERBOARDREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEGETLEADERBOARDREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSTATISTICSSERVICE_TYPE_INFO);
    registry.register_type(CLIENTSTATISTICSSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLEADERBOARDSERVICE_TYPE_INFO);
    registry.register_type(CLIENTLEADERBOARDSERVICE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceStatisticsRequestMessageBase {
}

pub const PRESENCESTATISTICSREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Statistics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceStatisticsRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCESTATISTICSREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceStatisticsMessageBase {
}

pub const PRESENCESTATISTICSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Statistics",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceStatisticsMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCESTATISTICSMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceLeaderboardServiceData {
}

pub const PRESENCELEADERBOARDSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeaderboardServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCELEADERBOARDSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceLeaderboardServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCELEADERBOARDSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCELEADERBOARDSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeaderboardServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceLeaderboardServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceStatisticsServiceData {
}

pub const PRESENCESTATISTICSSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESTATISTICSSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceStatisticsServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCESTATISTICSSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCESTATISTICSSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceStatisticsServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGetLeaderboardRequestParameters {
}

pub const PRESENCEGETLEADERBOARDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetLeaderboardRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETLEADERBOARDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetLeaderboardRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGETLEADERBOARDREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEGETLEADERBOARDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetLeaderboardRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceGetLeaderboardRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceDownloadStatisticsRequestParameters {
}

pub const PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadStatisticsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDownloadStatisticsRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadStatisticsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceDownloadStatisticsRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientStatisticsService {
}

pub const CLIENTSTATISTICSSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStatisticsService",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATISTICSSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStatisticsService {
    fn type_info() -> &'static TypeInfo {
        CLIENTSTATISTICSSERVICE_TYPE_INFO
    }
}


pub const CLIENTSTATISTICSSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStatisticsService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("ClientStatisticsService-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLeaderboardService {
}

pub const CLIENTLEADERBOARDSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLeaderboardService",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLEADERBOARDSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLeaderboardService {
    fn type_info() -> &'static TypeInfo {
        CLIENTLEADERBOARDSERVICE_TYPE_INFO
    }
}


pub const CLIENTLEADERBOARDSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLeaderboardService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("ClientLeaderboardService-Array"),
    array_type: None,
    alignment: 8,
};


