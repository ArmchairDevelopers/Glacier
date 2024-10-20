use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_vehicle_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERWINGCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERWINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(SERVERFLAPCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERFLAPCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTWINGCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTWINGCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTFLAPCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTFLAPCOMPONENT_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerWingComponent {
}

pub const SERVERWINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERWINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWingComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERWINGCOMPONENT_TYPE_INFO
    }
}


pub const SERVERWINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ServerWingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerFlapComponent {
}

pub const SERVERFLAPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlapComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERFLAPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFlapComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERFLAPCOMPONENT_TYPE_INFO
    }
}


pub const SERVERFLAPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlapComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ServerFlapComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientWingComponent {
}

pub const CLIENTWINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWingComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTWINGCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTWINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ClientWingComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientFlapComponent {
}

pub const CLIENTFLAPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFlapComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFLAPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFlapComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTFLAPCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTFLAPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFlapComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ClientFlapComponent-Array"),
    array_type: None,
    alignment: 8,
};


