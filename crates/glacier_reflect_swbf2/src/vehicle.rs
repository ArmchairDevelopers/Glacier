use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ServerWingComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerWingComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerWingComponentTrait for ServerWingComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerWingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerWingComponent {
}

impl super::entity::ComponentTrait for ServerWingComponent {
}

impl super::entity::EntityBusPeerTrait for ServerWingComponent {
}

pub static SERVERWINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerWingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERWINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerWingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERWINGCOMPONENT_TYPE_INFO
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


pub static SERVERWINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerWingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ServerWingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerFlapComponent {
    pub _glacier_base: super::gameplay_client_server::ServerGameComponent,
}

pub trait ServerFlapComponentTrait: super::gameplay_client_server::ServerGameComponentTrait {
}

impl ServerFlapComponentTrait for ServerFlapComponent {
}

impl super::gameplay_client_server::ServerGameComponentTrait for ServerFlapComponent {
}

impl super::gameplay_sim::GameComponentTrait for ServerFlapComponent {
}

impl super::entity::ComponentTrait for ServerFlapComponent {
}

impl super::entity::EntityBusPeerTrait for ServerFlapComponent {
}

pub static SERVERFLAPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlapComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::SERVERGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerFlapComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERFLAPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerFlapComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERFLAPCOMPONENT_TYPE_INFO
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


pub static SERVERFLAPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerFlapComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ServerFlapComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientWingComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientWingComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientWingComponentTrait for ClientWingComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientWingComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientWingComponent {
}

impl super::entity::ComponentTrait for ClientWingComponent {
}

impl super::entity::EntityBusPeerTrait for ClientWingComponent {
}

pub static CLIENTWINGCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWingComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientWingComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTWINGCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientWingComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTWINGCOMPONENT_TYPE_INFO
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


pub static CLIENTWINGCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientWingComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ClientWingComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientFlapComponent {
    pub _glacier_base: super::gameplay_client_server::ClientGameComponent,
}

pub trait ClientFlapComponentTrait: super::gameplay_client_server::ClientGameComponentTrait {
}

impl ClientFlapComponentTrait for ClientFlapComponent {
}

impl super::gameplay_client_server::ClientGameComponentTrait for ClientFlapComponent {
}

impl super::gameplay_sim::GameComponentTrait for ClientFlapComponent {
}

impl super::entity::ComponentTrait for ClientFlapComponent {
}

impl super::entity::EntityBusPeerTrait for ClientFlapComponent {
}

pub static CLIENTFLAPCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFlapComponent",
    flags: MemberInfoFlags::new(101),
    module: "Vehicle",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_client_server::CLIENTGAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientFlapComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTFLAPCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientFlapComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTFLAPCOMPONENT_TYPE_INFO
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


pub static CLIENTFLAPCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientFlapComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Vehicle",
    data: TypeInfoData::Array("ClientFlapComponent"),
    array_type: None,
    alignment: 8,
};


