use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_e_a_character_physics_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVEREACHARACTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(SERVEREACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTEACHARACTERPHYSICSCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTEACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerEACharacterPhysicsComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerEACharacterPhysicsComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerEACharacterPhysicsComponentTrait for ServerEACharacterPhysicsComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerEACharacterPhysicsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerEACharacterPhysicsComponent {
}

impl super::entity::ComponentTrait for ServerEACharacterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ServerEACharacterPhysicsComponent {
}

pub static SERVEREACHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEACharacterPhysicsComponent",
    name_hash: 169660697,
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ServerEACharacterPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerEACharacterPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ServerEACharacterPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEREACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerEACharacterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEREACHARACTERPHYSICSCOMPONENT_TYPE_INFO
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


pub static SERVEREACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerEACharacterPhysicsComponent-Array",
    name_hash: 2905681581,
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysics",
    data: TypeInfoData::Array("ServerEACharacterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientEACharacterPhysicsComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientEACharacterPhysicsComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientEACharacterPhysicsComponentTrait for ClientEACharacterPhysicsComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientEACharacterPhysicsComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientEACharacterPhysicsComponent {
}

impl super::entity::ComponentTrait for ClientEACharacterPhysicsComponent {
}

impl super::entity::EntityBusPeerTrait for ClientEACharacterPhysicsComponent {
}

pub static CLIENTEACHARACTERPHYSICSCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEACharacterPhysicsComponent",
    name_hash: 1225328837,
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysics",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(ClientEACharacterPhysicsComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientEACharacterPhysicsComponent as Default>::default())),
            create_boxed: || Box::new(<ClientEACharacterPhysicsComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTEACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientEACharacterPhysicsComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTEACHARACTERPHYSICSCOMPONENT_TYPE_INFO
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


pub static CLIENTEACHARACTERPHYSICSCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientEACharacterPhysicsComponent-Array",
    name_hash: 4064395505,
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysics",
    data: TypeInfoData::Array("ClientEACharacterPhysicsComponent"),
    array_type: None,
    alignment: 8,
};


