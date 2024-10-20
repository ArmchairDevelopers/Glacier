use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransactionalTelemetryHookEntity {
}

pub const TRANSACTIONALTELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransactionalTelemetryHookEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYHOOKENTITY_TYPE_INFO
    }
}


pub const TRANSACTIONALTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TransactionalTelemetryHookEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookEntity {
}

pub const TELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetryHookEntity {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKENTITY_TYPE_INFO
    }
}


pub const TELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryHookEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryGenericHookEntity {
}

pub const TELEMETRYGENERICHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYGENERICHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetryGenericHookEntity {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYGENERICHOOKENTITY_TYPE_INFO
    }
}


pub const TELEMETRYGENERICHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryGenericHookEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FixedStreamTelemetryHookEntity {
}

pub const FIXEDSTREAMTELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FIXEDSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FixedStreamTelemetryHookEntity {
    fn type_info() -> &'static TypeInfo {
        FIXEDSTREAMTELEMETRYHOOKENTITY_TYPE_INFO
    }
}


pub const FIXEDSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("FixedStreamTelemetryHookEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransactionalTelemetryStream {
}

pub const TRANSACTIONALTELEMETRYSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStream",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYSTREAM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransactionalTelemetryStream {
    fn type_info() -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYSTREAM_TYPE_INFO
    }
}


pub const TRANSACTIONALTELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TransactionalTelemetryStream-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryStream {
}

pub const TELEMETRYSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStream",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYSTREAM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetryStream {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSTREAM_TYPE_INFO
    }
}


pub const TELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("TelemetryStream-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventTelemetryStream {
}

pub const EVENTTELEMETRYSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStream",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTTELEMETRYSTREAM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventTelemetryStream {
    fn type_info() -> &'static TypeInfo {
        EVENTTELEMETRYSTREAM_TYPE_INFO
    }
}


pub const EVENTTELEMETRYSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("EventTelemetryStream-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VarStreamTelemetryHookEntity {
}

pub const VARSTREAMTELEMETRYHOOKENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntity",
    flags: MemberInfoFlags::new(101),
    module: "Telemetry",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VARSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VarStreamTelemetryHookEntity {
    fn type_info() -> &'static TypeInfo {
        VARSTREAMTELEMETRYHOOKENTITY_TYPE_INFO
    }
}


pub const VARSTREAMTELEMETRYHOOKENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Telemetry",
    data: TypeInfoData::Array("VarStreamTelemetryHookEntity-Array"),
    array_type: None,
    alignment: 8,
};


