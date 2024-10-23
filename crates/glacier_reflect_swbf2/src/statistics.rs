use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceStatisticsRequestMessageBase {
}

pub trait PresenceStatisticsRequestMessageBaseTrait: TypeObject {
}

impl PresenceStatisticsRequestMessageBaseTrait for PresenceStatisticsRequestMessageBase {
}

pub static PRESENCESTATISTICSREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsRequestMessageBase",
    name_hash: 1310735958,
    flags: MemberInfoFlags::new(36937),
    module: "Statistics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStatisticsRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceStatisticsRequestMessageBase as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceStatisticsMessageBase {
}

pub trait PresenceStatisticsMessageBaseTrait: TypeObject {
}

impl PresenceStatisticsMessageBaseTrait for PresenceStatisticsMessageBase {
}

pub static PRESENCESTATISTICSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceStatisticsMessageBase",
    name_hash: 2337168487,
    flags: MemberInfoFlags::new(36937),
    module: "Statistics",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStatisticsMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceStatisticsMessageBase as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3936039632,
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceLeaderboardServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLeaderboardServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceLeaderboardServiceData as Default>::default()),
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
    name_hash: 190558564,
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceLeaderboardServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2512026036,
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceStatisticsServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceStatisticsServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceStatisticsServiceData as Default>::default()),
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
    name_hash: 2770343168,
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceStatisticsServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1096192480,
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceGetLeaderboardRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGetLeaderboardRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceGetLeaderboardRequestParameters as Default>::default()),
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
    name_hash: 1348921300,
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceGetLeaderboardRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1914841158,
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceDownloadStatisticsRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDownloadStatisticsRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceDownloadStatisticsRequestParameters as Default>::default()),
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
    name_hash: 2223646194,
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("PresenceDownloadStatisticsRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 4162358756,
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientStatisticsService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientStatisticsService as Default>::default())),
            create_boxed: || Box::new(<ClientStatisticsService as Default>::default()),
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
    name_hash: 3371536848,
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("ClientStatisticsService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3344671360,
    flags: MemberInfoFlags::new(101),
    module: "Statistics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientLeaderboardService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLeaderboardService as Default>::default())),
            create_boxed: || Box::new(<ClientLeaderboardService as Default>::default()),
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
    name_hash: 3687458996,
    flags: MemberInfoFlags::new(145),
    module: "Statistics",
    data: TypeInfoData::Array("ClientLeaderboardService"),
    array_type: None,
    alignment: 8,
};


