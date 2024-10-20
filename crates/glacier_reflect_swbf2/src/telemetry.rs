use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_telemetry_types(registry: &mut TypeRegistry) {
    registry.register_type(TRANSACTIONALTELEMETRYHOOKENTITY_TYPE_INFO);
    registry.register_type(TRANSACTIONALTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKENTITY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYGENERICHOOKENTITY_TYPE_INFO);
    registry.register_type(TELEMETRYGENERICHOOKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FIXEDSTREAMTELEMETRYHOOKENTITY_TYPE_INFO);
    registry.register_type(FIXEDSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSACTIONALTELEMETRYSTREAM_TYPE_INFO);
    registry.register_type(TRANSACTIONALTELEMETRYSTREAM_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSTREAM_TYPE_INFO);
    registry.register_type(TELEMETRYSTREAM_ARRAY_TYPE_INFO);
    registry.register_type(EVENTTELEMETRYSTREAM_TYPE_INFO);
    registry.register_type(EVENTTELEMETRYSTREAM_ARRAY_TYPE_INFO);
    registry.register_type(VARSTREAMTELEMETRYHOOKENTITY_TYPE_INFO);
    registry.register_type(VARSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct TransactionalTelemetryHookEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait TransactionalTelemetryHookEntityTrait: super::entity::EntityTrait {
}

impl TransactionalTelemetryHookEntityTrait for TransactionalTelemetryHookEntity {
}

impl super::entity::EntityTrait for TransactionalTelemetryHookEntity {
}

impl super::entity::EntityBusPeerTrait for TransactionalTelemetryHookEntity {
}

pub static TRANSACTIONALTELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransactionalTelemetryHookEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransactionalTelemetryHookEntity {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYHOOKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TRANSACTIONALTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TransactionalTelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait TelemetryHookEntityTrait: super::entity::EntityTrait {
}

impl TelemetryHookEntityTrait for TelemetryHookEntity {
}

impl super::entity::EntityTrait for TelemetryHookEntity {
}

impl super::entity::EntityBusPeerTrait for TelemetryHookEntity {
}

pub static TELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetryHookEntity {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryGenericHookEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait TelemetryGenericHookEntityTrait: super::entity::EntityTrait {
}

impl TelemetryGenericHookEntityTrait for TelemetryGenericHookEntity {
}

impl super::entity::EntityTrait for TelemetryGenericHookEntity {
}

impl super::entity::EntityBusPeerTrait for TelemetryGenericHookEntity {
}

pub static TELEMETRYGENERICHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryGenericHookEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYGENERICHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetryGenericHookEntity {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYGENERICHOOKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TELEMETRYGENERICHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryGenericHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FixedStreamTelemetryHookEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait FixedStreamTelemetryHookEntityTrait: super::entity::EntityTrait {
}

impl FixedStreamTelemetryHookEntityTrait for FixedStreamTelemetryHookEntity {
}

impl super::entity::EntityTrait for FixedStreamTelemetryHookEntity {
}

impl super::entity::EntityBusPeerTrait for FixedStreamTelemetryHookEntity {
}

pub static FIXEDSTREAMTELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FixedStreamTelemetryHookEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FIXEDSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FixedStreamTelemetryHookEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FIXEDSTREAMTELEMETRYHOOKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FIXEDSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("FixedStreamTelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransactionalTelemetryStream {
    pub _glacier_base: TelemetryStream,
}

pub trait TransactionalTelemetryStreamTrait: TelemetryStreamTrait {
}

impl TransactionalTelemetryStreamTrait for TransactionalTelemetryStream {
}

impl TelemetryStreamTrait for TransactionalTelemetryStream {
}

pub static TRANSACTIONALTELEMETRYSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStream",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAM_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransactionalTelemetryStream as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYSTREAM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransactionalTelemetryStream {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYSTREAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TRANSACTIONALTELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TransactionalTelemetryStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryStream {
}

pub trait TelemetryStreamTrait: TypeObject {
}

impl TelemetryStreamTrait for TelemetryStream {
}

pub static TELEMETRYSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStream",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryStream as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYSTREAM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetryStream {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSTREAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventTelemetryStream {
    pub _glacier_base: TelemetryStream,
}

pub trait EventTelemetryStreamTrait: TelemetryStreamTrait {
}

impl EventTelemetryStreamTrait for EventTelemetryStream {
}

impl TelemetryStreamTrait for EventTelemetryStream {
}

pub static EVENTTELEMETRYSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStream",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAM_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventTelemetryStream as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EVENTTELEMETRYSTREAM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventTelemetryStream {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTTELEMETRYSTREAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EVENTTELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("EventTelemetryStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VarStreamTelemetryHookEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait VarStreamTelemetryHookEntityTrait: super::entity::EntityTrait {
}

impl VarStreamTelemetryHookEntityTrait for VarStreamTelemetryHookEntity {
}

impl super::entity::EntityTrait for VarStreamTelemetryHookEntity {
}

impl super::entity::EntityBusPeerTrait for VarStreamTelemetryHookEntity {
}

pub static VARSTREAMTELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VarStreamTelemetryHookEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VARSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VarStreamTelemetryHookEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VARSTREAMTELEMETRYHOOKENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VARSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("VarStreamTelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


