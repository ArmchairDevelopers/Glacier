use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_e_a_character_physics_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVEREACHARACTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVEREACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEACHARACTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTEACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerEACharacterPhysicsComponent {
}

pub const SERVEREACHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEACharacterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEREACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEACharacterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        SERVEREACHARACTERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const SERVEREACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEACharacterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysics",
    data: TypeInfoData::Array("ServerEACharacterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientEACharacterPhysicsComponent {
}

pub const CLIENTEACHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEACharacterPhysicsComponent",
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTGAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEACharacterPhysicsComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTEACHARACTERPHYSICSCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTEACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEACharacterPhysicsComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysics",
    data: TypeInfoData::Array("ClientEACharacterPhysicsComponent-Array"),
    array_type: None,
    alignment: 8,
};


