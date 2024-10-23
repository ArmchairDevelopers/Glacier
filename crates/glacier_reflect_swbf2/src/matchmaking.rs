use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_matchmaking_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCEMATCHMAKINGSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEMATCHMAKINGSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEMATCHMAKERMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEMATCHMAKINGREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(CLIENTMATCHMAKINGSERVICE_TYPE_INFO);
    registry.register_type(CLIENTMATCHMAKINGSERVICE_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceMatchmakingServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
    pub dummy_bool: bool,
}

pub trait PresenceMatchmakingServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
    fn dummy_bool(&self) -> &bool;
    fn dummy_bool_mut(&mut self) -> &mut bool;
}

impl PresenceMatchmakingServiceDataTrait for PresenceMatchmakingServiceData {
    fn dummy_bool(&self) -> &bool {
        &self.dummy_bool
    }
    fn dummy_bool_mut(&mut self) -> &mut bool {
        &mut self.dummy_bool
    }
}

impl super::online_shared::PresenceServiceDataTrait for PresenceMatchmakingServiceData {
}

impl super::core::AssetTrait for PresenceMatchmakingServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceMatchmakingServiceData {
}

pub static PRESENCEMATCHMAKINGSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakingServiceData",
    name_hash: 3477134309,
    flags: MemberInfoFlags::new(101),
    module: "Matchmaking",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceMatchmakingServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceMatchmakingServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceMatchmakingServiceData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "dummyBool",
                name_hash: 3418807235,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PresenceMatchmakingServiceData, dummy_bool),
            },
        ],
    }),
    array_type: Some(PRESENCEMATCHMAKINGSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceMatchmakingServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEMATCHMAKINGSERVICEDATA_TYPE_INFO
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


pub static PRESENCEMATCHMAKINGSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakingServiceData-Array",
    name_hash: 2206983377,
    flags: MemberInfoFlags::new(145),
    module: "Matchmaking",
    data: TypeInfoData::Array("PresenceMatchmakingServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceMatchmakerMessageBase {
}

pub trait PresenceMatchmakerMessageBaseTrait: TypeObject {
}

impl PresenceMatchmakerMessageBaseTrait for PresenceMatchmakerMessageBase {
}

pub static PRESENCEMATCHMAKERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakerMessageBase",
    name_hash: 2170770177,
    flags: MemberInfoFlags::new(36937),
    module: "Matchmaking",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceMatchmakerMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceMatchmakerMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceMatchmakerMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEMATCHMAKERMESSAGEBASE_TYPE_INFO
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
pub struct PresenceMatchmakingRequestMessageBase {
}

pub trait PresenceMatchmakingRequestMessageBaseTrait: TypeObject {
}

impl PresenceMatchmakingRequestMessageBaseTrait for PresenceMatchmakingRequestMessageBase {
}

pub static PRESENCEMATCHMAKINGREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMatchmakingRequestMessageBase",
    name_hash: 2060763687,
    flags: MemberInfoFlags::new(36937),
    module: "Matchmaking",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceMatchmakingRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceMatchmakingRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceMatchmakingRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEMATCHMAKINGREQUESTMESSAGEBASE_TYPE_INFO
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
pub struct ClientMatchmakingService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientMatchmakingServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientMatchmakingServiceTrait for ClientMatchmakingService {
}

impl super::online::PresenceServiceTrait for ClientMatchmakingService {
}

pub static CLIENTMATCHMAKINGSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMatchmakingService",
    name_hash: 1535026741,
    flags: MemberInfoFlags::new(101),
    module: "Matchmaking",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientMatchmakingService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMatchmakingService as Default>::default())),
            create_boxed: || Box::new(<ClientMatchmakingService as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMATCHMAKINGSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMatchmakingService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMATCHMAKINGSERVICE_TYPE_INFO
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


pub static CLIENTMATCHMAKINGSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMatchmakingService-Array",
    name_hash: 38463105,
    flags: MemberInfoFlags::new(145),
    module: "Matchmaking",
    data: TypeInfoData::Array("ClientMatchmakingService"),
    array_type: None,
    alignment: 8,
};


