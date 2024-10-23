use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2968593019,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(TransactionalTelemetryHookEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransactionalTelemetryHookEntity as Default>::default())),
            create_boxed: || Box::new(<TransactionalTelemetryHookEntity as Default>::default()),
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


pub static TRANSACTIONALTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntity-Array",
    name_hash: 1567996495,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TransactionalTelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1053119506,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(TelemetryHookEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookEntity as Default>::default())),
            create_boxed: || Box::new(<TelemetryHookEntity as Default>::default()),
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


pub static TELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntity-Array",
    name_hash: 1750203174,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 154338211,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(TelemetryGenericHookEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryGenericHookEntity as Default>::default())),
            create_boxed: || Box::new(<TelemetryGenericHookEntity as Default>::default()),
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


pub static TELEMETRYGENERICHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntity-Array",
    name_hash: 898983703,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryGenericHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3488252696,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(FixedStreamTelemetryHookEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FixedStreamTelemetryHookEntity as Default>::default())),
            create_boxed: || Box::new(<FixedStreamTelemetryHookEntity as Default>::default()),
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


pub static FIXEDSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntity-Array",
    name_hash: 456232236,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("FixedStreamTelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3182041951,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAM_TYPE_INFO),
        super_class_offset: offset_of!(TransactionalTelemetryStream, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransactionalTelemetryStream as Default>::default())),
            create_boxed: || Box::new(<TransactionalTelemetryStream as Default>::default()),
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


pub static TRANSACTIONALTELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStream-Array",
    name_hash: 1556418667,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TransactionalTelemetryStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TelemetryStream {
}

pub trait TelemetryStreamTrait: TypeObject {
}

impl TelemetryStreamTrait for TelemetryStream {
}

pub static TELEMETRYSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStream",
    name_hash: 158194678,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryStream as Default>::default())),
            create_boxed: || Box::new(<TelemetryStream as Default>::default()),
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


pub static TELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStream-Array",
    name_hash: 1842110658,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1061062426,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAM_TYPE_INFO),
        super_class_offset: offset_of!(EventTelemetryStream, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventTelemetryStream as Default>::default())),
            create_boxed: || Box::new(<EventTelemetryStream as Default>::default()),
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


pub static EVENTTELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStream-Array",
    name_hash: 1009044014,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("EventTelemetryStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 409578507,
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(VarStreamTelemetryHookEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VarStreamTelemetryHookEntity as Default>::default())),
            create_boxed: || Box::new(<VarStreamTelemetryHookEntity as Default>::default()),
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


pub static VARSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntity-Array",
    name_hash: 555481791,
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("VarStreamTelemetryHookEntity"),
    array_type: None,
    alignment: 8,
};


