use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct PresenceStatisticsRequestMessageBase {
}

pub trait PresenceStatisticsRequestMessageBaseTrait: TypeObject {
}

impl PresenceStatisticsRequestMessageBaseTrait for PresenceStatisticsRequestMessageBase {
}

pub static PRESENCESTATISTICSREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Statistics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStatisticsRequestMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceStatisticsRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESTATISTICSREQUESTMESSAGEBASE_TYPE_INFO
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
pub struct PresenceStatisticsMessageBase {
}

pub trait PresenceStatisticsMessageBaseTrait: TypeObject {
}

impl PresenceStatisticsMessageBaseTrait for PresenceStatisticsMessageBase {
}

pub static PRESENCESTATISTICSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Statistics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStatisticsMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceStatisticsMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESTATISTICSMESSAGEBASE_TYPE_INFO
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
pub struct PresenceLeaderboardServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
}

pub trait PresenceLeaderboardServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
}

impl PresenceLeaderboardServiceDataTrait for PresenceLeaderboardServiceData {
}

impl super::online_shared::PresenceServiceDataTrait for PresenceLeaderboardServiceData {
}

impl super::core::AssetTrait for PresenceLeaderboardServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceLeaderboardServiceData {
}

pub static PRESENCELEADERBOARDSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeaderboardServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLeaderboardServiceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCELEADERBOARDSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceLeaderboardServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCELEADERBOARDSERVICEDATA_TYPE_INFO
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


pub static PRESENCELEADERBOARDSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeaderboardServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceLeaderboardServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceStatisticsServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
}

pub trait PresenceStatisticsServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
}

impl PresenceStatisticsServiceDataTrait for PresenceStatisticsServiceData {
}

impl super::online_shared::PresenceServiceDataTrait for PresenceStatisticsServiceData {
}

impl super::core::AssetTrait for PresenceStatisticsServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceStatisticsServiceData {
}

pub static PRESENCESTATISTICSSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStatisticsServiceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESTATISTICSSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceStatisticsServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESTATISTICSSERVICEDATA_TYPE_INFO
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


pub static PRESENCESTATISTICSSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceStatisticsServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceGetLeaderboardRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceGetLeaderboardRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceGetLeaderboardRequestParametersTrait for PresenceGetLeaderboardRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceGetLeaderboardRequestParameters {
}

pub static PRESENCEGETLEADERBOARDREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetLeaderboardRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetLeaderboardRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEGETLEADERBOARDREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceGetLeaderboardRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGETLEADERBOARDREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEGETLEADERBOARDREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGetLeaderboardRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceGetLeaderboardRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceDownloadStatisticsRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceDownloadStatisticsRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceDownloadStatisticsRequestParametersTrait for PresenceDownloadStatisticsRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceDownloadStatisticsRequestParameters {
}

pub static PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadStatisticsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDownloadStatisticsRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDownloadStatisticsRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEDOWNLOADSTATISTICSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDownloadStatisticsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceDownloadStatisticsRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientStatisticsService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientStatisticsServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientStatisticsServiceTrait for ClientStatisticsService {
}

impl super::online::PresenceServiceTrait for ClientStatisticsService {
}

pub static CLIENTSTATISTICSSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStatisticsService",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStatisticsService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSTATISTICSSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientStatisticsService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSTATISTICSSERVICE_TYPE_INFO
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


pub static CLIENTSTATISTICSSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientStatisticsService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("ClientStatisticsService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLeaderboardService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientLeaderboardServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientLeaderboardServiceTrait for ClientLeaderboardService {
}

impl super::online::PresenceServiceTrait for ClientLeaderboardService {
}

pub static CLIENTLEADERBOARDSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLeaderboardService",
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLeaderboardService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLEADERBOARDSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLeaderboardService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLEADERBOARDSERVICE_TYPE_INFO
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


pub static CLIENTLEADERBOARDSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLeaderboardService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("ClientLeaderboardService"),
    array_type: None,
    alignment: 8,
};


