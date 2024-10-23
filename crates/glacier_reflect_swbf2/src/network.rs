use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_network_types(registry: &mut TypeRegistry) {
    registry.register_type(CLIENTSYNCEDTRANSFORMENTITY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDTRANSFORMENTITY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDINTENTITY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDINTENTITY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDFLOATENTITY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDFLOATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDFLOATENTITY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDFLOATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDBOOLENTITY_TYPE_INFO);
    registry.register_type(CLIENTSYNCEDBOOLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDBOOLENTITY_TYPE_INFO);
    registry.register_type(SERVERSYNCEDBOOLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ENGINECONNECTIONPEER_TYPE_INFO);
    registry.register_type(ENGINECONNECTIONPEER_ARRAY_TYPE_INFO);
    registry.register_type(ENGINECONNECTION_TYPE_INFO);
    registry.register_type(ENGINECONNECTION_ARRAY_TYPE_INFO);
    registry.register_type(SPIKEINTERNALMESSAGEPARTMESSAGE_TYPE_INFO);
    registry.register_type(SPIKEINTERNALMESSAGEWRAPPERMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKPERFOVERLAYSETTINGS_TYPE_INFO);
    registry.register_type(NETWORKPERFOVERLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(INTERPOLATIONMANAGERSETTINGS_TYPE_INFO);
    registry.register_type(INTERPOLATIONMANAGERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(INTERNETSIMULATIONSTATE_TYPE_INFO);
    registry.register_type(INTERNETSIMULATIONSTATE_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKCORESETTINGS_TYPE_INFO);
    registry.register_type(NETWORKCORESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(COREDEMOSTATUSMESSAGE_TYPE_INFO);
    registry.register_type(NETOBJECTSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(NETOBJECTSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(NETOBJECTSYSTEMDEBUGSETTINGS_TYPE_INFO);
    registry.register_type(NETOBJECTSYSTEMDEBUGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DELTACOMPRESSIONSETTINGS_TYPE_INFO);
    registry.register_type(DELTACOMPRESSIONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(NETOBJECTDEPENDENCYTYPE_TYPE_INFO);
    registry.register_type(NETOBJECTDEPENDENCYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(NETOBJECTPRIORITYSETTINGS_TYPE_INFO);
    registry.register_type(NETOBJECTPRIORITYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(NETOBJECTSENDSTATUS_TYPE_INFO);
    registry.register_type(NETOBJECTSENDSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKCHANNELID_TYPE_INFO);
    registry.register_type(NETWORKCHANNELID_ARRAY_TYPE_INFO);
    registry.register_type(SYNCEDTRANSFORMENTITYDATA_TYPE_INFO);
    registry.register_type(SYNCEDTRANSFORMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SYNCEDINTENTITYDATA_TYPE_INFO);
    registry.register_type(SYNCEDINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SYNCEDFLOATENTITYDATA_TYPE_INFO);
    registry.register_type(SYNCEDFLOATENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SYNCEDBOOLENTITYDATA_TYPE_INFO);
    registry.register_type(SYNCEDBOOLENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSyncedTransformEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSyncedTransformEntityTrait: super::entity::EntityTrait {
}

impl ClientSyncedTransformEntityTrait for ClientSyncedTransformEntity {
}

impl super::entity::EntityTrait for ClientSyncedTransformEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSyncedTransformEntity {
}

pub static CLIENTSYNCEDTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedTransformEntity",
    name_hash: 1776480717,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSyncedTransformEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedTransformEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSyncedTransformEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedTransformEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSYNCEDTRANSFORMENTITY_TYPE_INFO
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


pub static CLIENTSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedTransformEntity-Array",
    name_hash: 2510669305,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedTransformEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSyncedTransformEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSyncedTransformEntityTrait: super::entity::EntityTrait {
}

impl ServerSyncedTransformEntityTrait for ServerSyncedTransformEntity {
}

impl super::entity::EntityTrait for ServerSyncedTransformEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSyncedTransformEntity {
}

pub static SERVERSYNCEDTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedTransformEntity",
    name_hash: 4024262673,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSyncedTransformEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedTransformEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSyncedTransformEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedTransformEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSYNCEDTRANSFORMENTITY_TYPE_INFO
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


pub static SERVERSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedTransformEntity-Array",
    name_hash: 1751228837,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedTransformEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSyncedIntEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSyncedIntEntityTrait: super::entity::EntityTrait {
}

impl ClientSyncedIntEntityTrait for ClientSyncedIntEntity {
}

impl super::entity::EntityTrait for ClientSyncedIntEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSyncedIntEntity {
}

pub static CLIENTSYNCEDINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedIntEntity",
    name_hash: 3960958994,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSyncedIntEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedIntEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSyncedIntEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedIntEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSYNCEDINTENTITY_TYPE_INFO
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


pub static CLIENTSYNCEDINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedIntEntity-Array",
    name_hash: 2848528166,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedIntEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSyncedIntEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSyncedIntEntityTrait: super::entity::EntityTrait {
}

impl ServerSyncedIntEntityTrait for ServerSyncedIntEntity {
}

impl super::entity::EntityTrait for ServerSyncedIntEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSyncedIntEntity {
}

pub static SERVERSYNCEDINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedIntEntity",
    name_hash: 762172494,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSyncedIntEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedIntEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSyncedIntEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedIntEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSYNCEDINTENTITY_TYPE_INFO
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


pub static SERVERSYNCEDINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedIntEntity-Array",
    name_hash: 2942709242,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedIntEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSyncedFloatEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSyncedFloatEntityTrait: super::entity::EntityTrait {
}

impl ClientSyncedFloatEntityTrait for ClientSyncedFloatEntity {
}

impl super::entity::EntityTrait for ClientSyncedFloatEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSyncedFloatEntity {
}

pub static CLIENTSYNCEDFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedFloatEntity",
    name_hash: 274493617,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSyncedFloatEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedFloatEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSyncedFloatEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedFloatEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSYNCEDFLOATENTITY_TYPE_INFO
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


pub static CLIENTSYNCEDFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedFloatEntity-Array",
    name_hash: 4172735237,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedFloatEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSyncedFloatEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSyncedFloatEntityTrait: super::entity::EntityTrait {
}

impl ServerSyncedFloatEntityTrait for ServerSyncedFloatEntity {
}

impl super::entity::EntityTrait for ServerSyncedFloatEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSyncedFloatEntity {
}

pub static SERVERSYNCEDFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedFloatEntity",
    name_hash: 3771005933,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSyncedFloatEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedFloatEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSyncedFloatEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedFloatEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSYNCEDFLOATENTITY_TYPE_INFO
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


pub static SERVERSYNCEDFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedFloatEntity-Array",
    name_hash: 841441497,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedFloatEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientSyncedBoolEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientSyncedBoolEntityTrait: super::entity::EntityTrait {
}

impl ClientSyncedBoolEntityTrait for ClientSyncedBoolEntity {
}

impl super::entity::EntityTrait for ClientSyncedBoolEntity {
}

impl super::entity::EntityBusPeerTrait for ClientSyncedBoolEntity {
}

pub static CLIENTSYNCEDBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedBoolEntity",
    name_hash: 1054394639,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientSyncedBoolEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedBoolEntity as Default>::default())),
            create_boxed: || Box::new(<ClientSyncedBoolEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedBoolEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTSYNCEDBOOLENTITY_TYPE_INFO
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


pub static CLIENTSYNCEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedBoolEntity-Array",
    name_hash: 2565637563,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedBoolEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerSyncedBoolEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerSyncedBoolEntityTrait: super::entity::EntityTrait {
}

impl ServerSyncedBoolEntityTrait for ServerSyncedBoolEntity {
}

impl super::entity::EntityTrait for ServerSyncedBoolEntity {
}

impl super::entity::EntityBusPeerTrait for ServerSyncedBoolEntity {
}

pub static SERVERSYNCEDBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedBoolEntity",
    name_hash: 952774483,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerSyncedBoolEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedBoolEntity as Default>::default())),
            create_boxed: || Box::new(<ServerSyncedBoolEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedBoolEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERSYNCEDBOOLENTITY_TYPE_INFO
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


pub static SERVERSYNCEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedBoolEntity-Array",
    name_hash: 3638893159,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedBoolEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EngineConnectionPeer {
    pub _glacier_base: EngineConnection,
}

pub trait EngineConnectionPeerTrait: EngineConnectionTrait {
}

impl EngineConnectionPeerTrait for EngineConnectionPeer {
}

impl EngineConnectionTrait for EngineConnectionPeer {
}

pub static ENGINECONNECTIONPEER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnectionPeer",
    name_hash: 3930262911,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONNECTION_TYPE_INFO),
        super_class_offset: offset_of!(EngineConnectionPeer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EngineConnectionPeer as Default>::default())),
            create_boxed: || Box::new(<EngineConnectionPeer as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENGINECONNECTIONPEER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EngineConnectionPeer {
    fn type_info(&self) -> &'static TypeInfo {
        ENGINECONNECTIONPEER_TYPE_INFO
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


pub static ENGINECONNECTIONPEER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnectionPeer-Array",
    name_hash: 1720423243,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("EngineConnectionPeer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EngineConnection {
}

pub trait EngineConnectionTrait: TypeObject {
}

impl EngineConnectionTrait for EngineConnection {
}

pub static ENGINECONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnection",
    name_hash: 1372988669,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EngineConnection as Default>::default())),
            create_boxed: || Box::new(<EngineConnection as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENGINECONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EngineConnection {
    fn type_info(&self) -> &'static TypeInfo {
        ENGINECONNECTION_TYPE_INFO
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


pub static ENGINECONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnection-Array",
    name_hash: 1329992137,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("EngineConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SpikeInternalMessagePartMessage {
}

pub trait SpikeInternalMessagePartMessageTrait: TypeObject {
}

impl SpikeInternalMessagePartMessageTrait for SpikeInternalMessagePartMessage {
}

pub static SPIKEINTERNALMESSAGEPARTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpikeInternalMessagePartMessage",
    name_hash: 1784852497,
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpikeInternalMessagePartMessage as Default>::default())),
            create_boxed: || Box::new(<SpikeInternalMessagePartMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SpikeInternalMessagePartMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SPIKEINTERNALMESSAGEPARTMESSAGE_TYPE_INFO
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
pub struct SpikeInternalMessageWrapperMessage {
}

pub trait SpikeInternalMessageWrapperMessageTrait: TypeObject {
}

impl SpikeInternalMessageWrapperMessageTrait for SpikeInternalMessageWrapperMessage {
}

pub static SPIKEINTERNALMESSAGEWRAPPERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpikeInternalMessageWrapperMessage",
    name_hash: 4077307189,
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpikeInternalMessageWrapperMessage as Default>::default())),
            create_boxed: || Box::new(<SpikeInternalMessageWrapperMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SpikeInternalMessageWrapperMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SPIKEINTERNALMESSAGEWRAPPERMESSAGE_TYPE_INFO
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
pub struct NetworkPerfOverlaySettings {
    pub _glacier_base: super::core::DataContainer,
    pub enable: bool,
    pub graph_pos: super::core::Vec2,
    pub update_frequency: f32,
    pub high_latency: f32,
    pub critical_latency: f32,
    pub high_latency_variation: f32,
    pub critical_latency_variation: f32,
    pub high_packet_loss_ratio: f32,
    pub critical_packet_loss_ratio: f32,
    pub server_fps_low_threshold_perc: f32,
    pub server_fps_low_threshold_crit_perc: f32,
}

pub trait NetworkPerfOverlaySettingsTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn graph_pos(&self) -> &super::core::Vec2;
    fn graph_pos_mut(&mut self) -> &mut super::core::Vec2;
    fn update_frequency(&self) -> &f32;
    fn update_frequency_mut(&mut self) -> &mut f32;
    fn high_latency(&self) -> &f32;
    fn high_latency_mut(&mut self) -> &mut f32;
    fn critical_latency(&self) -> &f32;
    fn critical_latency_mut(&mut self) -> &mut f32;
    fn high_latency_variation(&self) -> &f32;
    fn high_latency_variation_mut(&mut self) -> &mut f32;
    fn critical_latency_variation(&self) -> &f32;
    fn critical_latency_variation_mut(&mut self) -> &mut f32;
    fn high_packet_loss_ratio(&self) -> &f32;
    fn high_packet_loss_ratio_mut(&mut self) -> &mut f32;
    fn critical_packet_loss_ratio(&self) -> &f32;
    fn critical_packet_loss_ratio_mut(&mut self) -> &mut f32;
    fn server_fps_low_threshold_perc(&self) -> &f32;
    fn server_fps_low_threshold_perc_mut(&mut self) -> &mut f32;
    fn server_fps_low_threshold_crit_perc(&self) -> &f32;
    fn server_fps_low_threshold_crit_perc_mut(&mut self) -> &mut f32;
}

impl NetworkPerfOverlaySettingsTrait for NetworkPerfOverlaySettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn graph_pos(&self) -> &super::core::Vec2 {
        &self.graph_pos
    }
    fn graph_pos_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.graph_pos
    }
    fn update_frequency(&self) -> &f32 {
        &self.update_frequency
    }
    fn update_frequency_mut(&mut self) -> &mut f32 {
        &mut self.update_frequency
    }
    fn high_latency(&self) -> &f32 {
        &self.high_latency
    }
    fn high_latency_mut(&mut self) -> &mut f32 {
        &mut self.high_latency
    }
    fn critical_latency(&self) -> &f32 {
        &self.critical_latency
    }
    fn critical_latency_mut(&mut self) -> &mut f32 {
        &mut self.critical_latency
    }
    fn high_latency_variation(&self) -> &f32 {
        &self.high_latency_variation
    }
    fn high_latency_variation_mut(&mut self) -> &mut f32 {
        &mut self.high_latency_variation
    }
    fn critical_latency_variation(&self) -> &f32 {
        &self.critical_latency_variation
    }
    fn critical_latency_variation_mut(&mut self) -> &mut f32 {
        &mut self.critical_latency_variation
    }
    fn high_packet_loss_ratio(&self) -> &f32 {
        &self.high_packet_loss_ratio
    }
    fn high_packet_loss_ratio_mut(&mut self) -> &mut f32 {
        &mut self.high_packet_loss_ratio
    }
    fn critical_packet_loss_ratio(&self) -> &f32 {
        &self.critical_packet_loss_ratio
    }
    fn critical_packet_loss_ratio_mut(&mut self) -> &mut f32 {
        &mut self.critical_packet_loss_ratio
    }
    fn server_fps_low_threshold_perc(&self) -> &f32 {
        &self.server_fps_low_threshold_perc
    }
    fn server_fps_low_threshold_perc_mut(&mut self) -> &mut f32 {
        &mut self.server_fps_low_threshold_perc
    }
    fn server_fps_low_threshold_crit_perc(&self) -> &f32 {
        &self.server_fps_low_threshold_crit_perc
    }
    fn server_fps_low_threshold_crit_perc_mut(&mut self) -> &mut f32 {
        &mut self.server_fps_low_threshold_crit_perc
    }
}

impl super::core::DataContainerTrait for NetworkPerfOverlaySettings {
}

pub static NETWORKPERFOVERLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerfOverlaySettings",
    name_hash: 813936613,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(NetworkPerfOverlaySettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkPerfOverlaySettings as Default>::default())),
            create_boxed: || Box::new(<NetworkPerfOverlaySettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, enable),
            },
            FieldInfoData {
                name: "GraphPos",
                name_hash: 1352104293,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, graph_pos),
            },
            FieldInfoData {
                name: "UpdateFrequency",
                name_hash: 219717232,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, update_frequency),
            },
            FieldInfoData {
                name: "HighLatency",
                name_hash: 3787528835,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_latency),
            },
            FieldInfoData {
                name: "CriticalLatency",
                name_hash: 2362833414,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_latency),
            },
            FieldInfoData {
                name: "HighLatencyVariation",
                name_hash: 2284846354,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_latency_variation),
            },
            FieldInfoData {
                name: "CriticalLatencyVariation",
                name_hash: 2417257143,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_latency_variation),
            },
            FieldInfoData {
                name: "HighPacketLossRatio",
                name_hash: 2039687393,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_packet_loss_ratio),
            },
            FieldInfoData {
                name: "CriticalPacketLossRatio",
                name_hash: 1584471204,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_packet_loss_ratio),
            },
            FieldInfoData {
                name: "ServerFpsLowThresholdPerc",
                name_hash: 451424322,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, server_fps_low_threshold_perc),
            },
            FieldInfoData {
                name: "ServerFpsLowThresholdCritPerc",
                name_hash: 2411888046,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, server_fps_low_threshold_crit_perc),
            },
        ],
    }),
    array_type: Some(NETWORKPERFOVERLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkPerfOverlaySettings {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKPERFOVERLAYSETTINGS_TYPE_INFO
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


pub static NETWORKPERFOVERLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerfOverlaySettings-Array",
    name_hash: 2292037841,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkPerfOverlaySettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InterpolationManagerSettings {
    pub _glacier_base: super::core::DataContainer,
    pub time_nudge_calculator: i32,
    pub time_nudge_p_i_d_const_k_pos: f32,
    pub time_nudge_p_i_d_const_k_neg: f32,
    pub time_nudge_p_i_d_const_t_i_pos: f32,
    pub time_nudge_p_i_d_const_t_i_neg: f32,
    pub time_nudge_p_i_d_const_t_d_pos: f32,
    pub time_nudge_p_i_d_const_t_d_neg: f32,
    pub time_nudge_p_i_d_latency_tol: f32,
    pub time_nudge_p_i_d_packet_delta_time_tol: f32,
    pub time_nudge_p_i_d_max_change_per_sec: f32,
    pub average_packet_sample_count: i32,
}

pub trait InterpolationManagerSettingsTrait: super::core::DataContainerTrait {
    fn time_nudge_calculator(&self) -> &i32;
    fn time_nudge_calculator_mut(&mut self) -> &mut i32;
    fn time_nudge_p_i_d_const_k_pos(&self) -> &f32;
    fn time_nudge_p_i_d_const_k_pos_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_const_k_neg(&self) -> &f32;
    fn time_nudge_p_i_d_const_k_neg_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_const_t_i_pos(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_i_pos_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_const_t_i_neg(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_i_neg_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_const_t_d_pos(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_d_pos_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_const_t_d_neg(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_d_neg_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_latency_tol(&self) -> &f32;
    fn time_nudge_p_i_d_latency_tol_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_packet_delta_time_tol(&self) -> &f32;
    fn time_nudge_p_i_d_packet_delta_time_tol_mut(&mut self) -> &mut f32;
    fn time_nudge_p_i_d_max_change_per_sec(&self) -> &f32;
    fn time_nudge_p_i_d_max_change_per_sec_mut(&mut self) -> &mut f32;
    fn average_packet_sample_count(&self) -> &i32;
    fn average_packet_sample_count_mut(&mut self) -> &mut i32;
}

impl InterpolationManagerSettingsTrait for InterpolationManagerSettings {
    fn time_nudge_calculator(&self) -> &i32 {
        &self.time_nudge_calculator
    }
    fn time_nudge_calculator_mut(&mut self) -> &mut i32 {
        &mut self.time_nudge_calculator
    }
    fn time_nudge_p_i_d_const_k_pos(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_k_pos
    }
    fn time_nudge_p_i_d_const_k_pos_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_const_k_pos
    }
    fn time_nudge_p_i_d_const_k_neg(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_k_neg
    }
    fn time_nudge_p_i_d_const_k_neg_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_const_k_neg
    }
    fn time_nudge_p_i_d_const_t_i_pos(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_i_pos
    }
    fn time_nudge_p_i_d_const_t_i_pos_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_const_t_i_pos
    }
    fn time_nudge_p_i_d_const_t_i_neg(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_i_neg
    }
    fn time_nudge_p_i_d_const_t_i_neg_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_const_t_i_neg
    }
    fn time_nudge_p_i_d_const_t_d_pos(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_d_pos
    }
    fn time_nudge_p_i_d_const_t_d_pos_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_const_t_d_pos
    }
    fn time_nudge_p_i_d_const_t_d_neg(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_d_neg
    }
    fn time_nudge_p_i_d_const_t_d_neg_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_const_t_d_neg
    }
    fn time_nudge_p_i_d_latency_tol(&self) -> &f32 {
        &self.time_nudge_p_i_d_latency_tol
    }
    fn time_nudge_p_i_d_latency_tol_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_latency_tol
    }
    fn time_nudge_p_i_d_packet_delta_time_tol(&self) -> &f32 {
        &self.time_nudge_p_i_d_packet_delta_time_tol
    }
    fn time_nudge_p_i_d_packet_delta_time_tol_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_packet_delta_time_tol
    }
    fn time_nudge_p_i_d_max_change_per_sec(&self) -> &f32 {
        &self.time_nudge_p_i_d_max_change_per_sec
    }
    fn time_nudge_p_i_d_max_change_per_sec_mut(&mut self) -> &mut f32 {
        &mut self.time_nudge_p_i_d_max_change_per_sec
    }
    fn average_packet_sample_count(&self) -> &i32 {
        &self.average_packet_sample_count
    }
    fn average_packet_sample_count_mut(&mut self) -> &mut i32 {
        &mut self.average_packet_sample_count
    }
}

impl super::core::DataContainerTrait for InterpolationManagerSettings {
}

pub static INTERPOLATIONMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationManagerSettings",
    name_hash: 3197599801,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(InterpolationManagerSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InterpolationManagerSettings as Default>::default())),
            create_boxed: || Box::new(<InterpolationManagerSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TimeNudgeCalculator",
                name_hash: 2386089905,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_calculator),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstKPos",
                name_hash: 317353362,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_k_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstKNeg",
                name_hash: 317352018,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_k_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTIPos",
                name_hash: 1849093924,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_i_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTINeg",
                name_hash: 1849070180,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_i_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTDPos",
                name_hash: 1848982025,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_d_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTDNeg",
                name_hash: 1848975433,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_d_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDLatencyTol",
                name_hash: 133853199,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_latency_tol),
            },
            FieldInfoData {
                name: "TimeNudgePIDPacketDeltaTimeTol",
                name_hash: 2633655426,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_packet_delta_time_tol),
            },
            FieldInfoData {
                name: "TimeNudgePIDMaxChangePerSec",
                name_hash: 828391824,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_max_change_per_sec),
            },
            FieldInfoData {
                name: "AveragePacketSampleCount",
                name_hash: 1637518571,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InterpolationManagerSettings, average_packet_sample_count),
            },
        ],
    }),
    array_type: Some(INTERPOLATIONMANAGERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InterpolationManagerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        INTERPOLATIONMANAGERSETTINGS_TYPE_INFO
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


pub static INTERPOLATIONMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationManagerSettings-Array",
    name_hash: 2422140557,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("InterpolationManagerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InternetSimulationState {
    pub enabled: bool,
    pub reorder_ratio: f32,
    pub latency_min: f32,
    pub latency_max: f32,
    pub duplicate_ratio: f32,
    pub drop_ratio: f32,
    pub corrupt_ratio: f32,
    pub size_ratio: f32,
    pub spike_duration_min: f32,
    pub spike_duration_max: f32,
    pub spike_cooldown_min: f32,
    pub spike_cooldown_max: f32,
    pub bandwidth_max: f32,
    pub bandwidth_delay_max: f32,
}

pub trait InternetSimulationStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn reorder_ratio(&self) -> &f32;
    fn reorder_ratio_mut(&mut self) -> &mut f32;
    fn latency_min(&self) -> &f32;
    fn latency_min_mut(&mut self) -> &mut f32;
    fn latency_max(&self) -> &f32;
    fn latency_max_mut(&mut self) -> &mut f32;
    fn duplicate_ratio(&self) -> &f32;
    fn duplicate_ratio_mut(&mut self) -> &mut f32;
    fn drop_ratio(&self) -> &f32;
    fn drop_ratio_mut(&mut self) -> &mut f32;
    fn corrupt_ratio(&self) -> &f32;
    fn corrupt_ratio_mut(&mut self) -> &mut f32;
    fn size_ratio(&self) -> &f32;
    fn size_ratio_mut(&mut self) -> &mut f32;
    fn spike_duration_min(&self) -> &f32;
    fn spike_duration_min_mut(&mut self) -> &mut f32;
    fn spike_duration_max(&self) -> &f32;
    fn spike_duration_max_mut(&mut self) -> &mut f32;
    fn spike_cooldown_min(&self) -> &f32;
    fn spike_cooldown_min_mut(&mut self) -> &mut f32;
    fn spike_cooldown_max(&self) -> &f32;
    fn spike_cooldown_max_mut(&mut self) -> &mut f32;
    fn bandwidth_max(&self) -> &f32;
    fn bandwidth_max_mut(&mut self) -> &mut f32;
    fn bandwidth_delay_max(&self) -> &f32;
    fn bandwidth_delay_max_mut(&mut self) -> &mut f32;
}

impl InternetSimulationStateTrait for InternetSimulationState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn reorder_ratio(&self) -> &f32 {
        &self.reorder_ratio
    }
    fn reorder_ratio_mut(&mut self) -> &mut f32 {
        &mut self.reorder_ratio
    }
    fn latency_min(&self) -> &f32 {
        &self.latency_min
    }
    fn latency_min_mut(&mut self) -> &mut f32 {
        &mut self.latency_min
    }
    fn latency_max(&self) -> &f32 {
        &self.latency_max
    }
    fn latency_max_mut(&mut self) -> &mut f32 {
        &mut self.latency_max
    }
    fn duplicate_ratio(&self) -> &f32 {
        &self.duplicate_ratio
    }
    fn duplicate_ratio_mut(&mut self) -> &mut f32 {
        &mut self.duplicate_ratio
    }
    fn drop_ratio(&self) -> &f32 {
        &self.drop_ratio
    }
    fn drop_ratio_mut(&mut self) -> &mut f32 {
        &mut self.drop_ratio
    }
    fn corrupt_ratio(&self) -> &f32 {
        &self.corrupt_ratio
    }
    fn corrupt_ratio_mut(&mut self) -> &mut f32 {
        &mut self.corrupt_ratio
    }
    fn size_ratio(&self) -> &f32 {
        &self.size_ratio
    }
    fn size_ratio_mut(&mut self) -> &mut f32 {
        &mut self.size_ratio
    }
    fn spike_duration_min(&self) -> &f32 {
        &self.spike_duration_min
    }
    fn spike_duration_min_mut(&mut self) -> &mut f32 {
        &mut self.spike_duration_min
    }
    fn spike_duration_max(&self) -> &f32 {
        &self.spike_duration_max
    }
    fn spike_duration_max_mut(&mut self) -> &mut f32 {
        &mut self.spike_duration_max
    }
    fn spike_cooldown_min(&self) -> &f32 {
        &self.spike_cooldown_min
    }
    fn spike_cooldown_min_mut(&mut self) -> &mut f32 {
        &mut self.spike_cooldown_min
    }
    fn spike_cooldown_max(&self) -> &f32 {
        &self.spike_cooldown_max
    }
    fn spike_cooldown_max_mut(&mut self) -> &mut f32 {
        &mut self.spike_cooldown_max
    }
    fn bandwidth_max(&self) -> &f32 {
        &self.bandwidth_max
    }
    fn bandwidth_max_mut(&mut self) -> &mut f32 {
        &mut self.bandwidth_max
    }
    fn bandwidth_delay_max(&self) -> &f32 {
        &self.bandwidth_delay_max
    }
    fn bandwidth_delay_max_mut(&mut self) -> &mut f32 {
        &mut self.bandwidth_delay_max
    }
}

pub static INTERNETSIMULATIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternetSimulationState",
    name_hash: 1267347930,
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InternetSimulationState as Default>::default())),
            create_boxed: || Box::new(<InternetSimulationState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InternetSimulationState, enabled),
            },
            FieldInfoData {
                name: "ReorderRatio",
                name_hash: 4055254845,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, reorder_ratio),
            },
            FieldInfoData {
                name: "LatencyMin",
                name_hash: 786382599,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, latency_min),
            },
            FieldInfoData {
                name: "LatencyMax",
                name_hash: 786382361,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, latency_max),
            },
            FieldInfoData {
                name: "DuplicateRatio",
                name_hash: 1765163763,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, duplicate_ratio),
            },
            FieldInfoData {
                name: "DropRatio",
                name_hash: 2887696013,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, drop_ratio),
            },
            FieldInfoData {
                name: "CorruptRatio",
                name_hash: 3938068633,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, corrupt_ratio),
            },
            FieldInfoData {
                name: "SizeRatio",
                name_hash: 2353800513,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, size_ratio),
            },
            FieldInfoData {
                name: "SpikeDurationMin",
                name_hash: 3352376597,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_duration_min),
            },
            FieldInfoData {
                name: "SpikeDurationMax",
                name_hash: 3352376331,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_duration_max),
            },
            FieldInfoData {
                name: "SpikeCooldownMin",
                name_hash: 2870776246,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_cooldown_min),
            },
            FieldInfoData {
                name: "SpikeCooldownMax",
                name_hash: 2870775976,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_cooldown_max),
            },
            FieldInfoData {
                name: "BandwidthMax",
                name_hash: 773802078,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, bandwidth_max),
            },
            FieldInfoData {
                name: "BandwidthDelayMax",
                name_hash: 1436090027,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, bandwidth_delay_max),
            },
        ],
    }),
    array_type: Some(INTERNETSIMULATIONSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InternetSimulationState {
    fn type_info(&self) -> &'static TypeInfo {
        INTERNETSIMULATIONSTATE_TYPE_INFO
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


pub static INTERNETSIMULATIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternetSimulationState-Array",
    name_hash: 101041518,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("InternetSimulationState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkCoreSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub dedicated_server_max_send_job_count: u32,
    pub server_max_send_job_count: u32,
}

pub trait NetworkCoreSettingsTrait: super::core::SystemSettingsTrait {
    fn dedicated_server_max_send_job_count(&self) -> &u32;
    fn dedicated_server_max_send_job_count_mut(&mut self) -> &mut u32;
    fn server_max_send_job_count(&self) -> &u32;
    fn server_max_send_job_count_mut(&mut self) -> &mut u32;
}

impl NetworkCoreSettingsTrait for NetworkCoreSettings {
    fn dedicated_server_max_send_job_count(&self) -> &u32 {
        &self.dedicated_server_max_send_job_count
    }
    fn dedicated_server_max_send_job_count_mut(&mut self) -> &mut u32 {
        &mut self.dedicated_server_max_send_job_count
    }
    fn server_max_send_job_count(&self) -> &u32 {
        &self.server_max_send_job_count
    }
    fn server_max_send_job_count_mut(&mut self) -> &mut u32 {
        &mut self.server_max_send_job_count
    }
}

impl super::core::SystemSettingsTrait for NetworkCoreSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for NetworkCoreSettings {
}

pub static NETWORKCORESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCoreSettings",
    name_hash: 3860884837,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(NetworkCoreSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkCoreSettings as Default>::default())),
            create_boxed: || Box::new(<NetworkCoreSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DedicatedServerMaxSendJobCount",
                name_hash: 3154848759,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkCoreSettings, dedicated_server_max_send_job_count),
            },
            FieldInfoData {
                name: "ServerMaxSendJobCount",
                name_hash: 225925612,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkCoreSettings, server_max_send_job_count),
            },
        ],
    }),
    array_type: Some(NETWORKCORESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkCoreSettings {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKCORESETTINGS_TYPE_INFO
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


pub static NETWORKCORESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCoreSettings-Array",
    name_hash: 1060279889,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkCoreSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreDemoStatusMessage {
}

pub trait CoreDemoStatusMessageTrait: TypeObject {
}

impl CoreDemoStatusMessageTrait for CoreDemoStatusMessage {
}

pub static COREDEMOSTATUSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreDemoStatusMessage",
    name_hash: 3073441026,
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreDemoStatusMessage as Default>::default())),
            create_boxed: || Box::new(<CoreDemoStatusMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreDemoStatusMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREDEMOSTATUSMESSAGE_TYPE_INFO
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
pub struct NetObjectSystemSettings {
    pub _glacier_base: super::core::DataContainer,
    pub max_net_object_count: u32,
    pub max_static_net_object_count: u32,
    pub max_client_connection_count: u32,
    pub max_server_connection_count: u32,
    pub in_proc_replication_enabled: bool,
    pub in_proc_buffer_size: u32,
    pub game_view_in_proc_buffer_size: u32,
    pub max_remote_authority_net_object_count: u32,
    pub default_dynamic_priority_method: i32,
    pub priority_settings: NetObjectPrioritySettings,
    pub default_filter_method: i32,
    pub delta_compression_settings: DeltaCompressionSettings,
    pub debug: NetObjectSystemDebugSettings,
}

pub trait NetObjectSystemSettingsTrait: super::core::DataContainerTrait {
    fn max_net_object_count(&self) -> &u32;
    fn max_net_object_count_mut(&mut self) -> &mut u32;
    fn max_static_net_object_count(&self) -> &u32;
    fn max_static_net_object_count_mut(&mut self) -> &mut u32;
    fn max_client_connection_count(&self) -> &u32;
    fn max_client_connection_count_mut(&mut self) -> &mut u32;
    fn max_server_connection_count(&self) -> &u32;
    fn max_server_connection_count_mut(&mut self) -> &mut u32;
    fn in_proc_replication_enabled(&self) -> &bool;
    fn in_proc_replication_enabled_mut(&mut self) -> &mut bool;
    fn in_proc_buffer_size(&self) -> &u32;
    fn in_proc_buffer_size_mut(&mut self) -> &mut u32;
    fn game_view_in_proc_buffer_size(&self) -> &u32;
    fn game_view_in_proc_buffer_size_mut(&mut self) -> &mut u32;
    fn max_remote_authority_net_object_count(&self) -> &u32;
    fn max_remote_authority_net_object_count_mut(&mut self) -> &mut u32;
    fn default_dynamic_priority_method(&self) -> &i32;
    fn default_dynamic_priority_method_mut(&mut self) -> &mut i32;
    fn priority_settings(&self) -> &NetObjectPrioritySettings;
    fn priority_settings_mut(&mut self) -> &mut NetObjectPrioritySettings;
    fn default_filter_method(&self) -> &i32;
    fn default_filter_method_mut(&mut self) -> &mut i32;
    fn delta_compression_settings(&self) -> &DeltaCompressionSettings;
    fn delta_compression_settings_mut(&mut self) -> &mut DeltaCompressionSettings;
    fn debug(&self) -> &NetObjectSystemDebugSettings;
    fn debug_mut(&mut self) -> &mut NetObjectSystemDebugSettings;
}

impl NetObjectSystemSettingsTrait for NetObjectSystemSettings {
    fn max_net_object_count(&self) -> &u32 {
        &self.max_net_object_count
    }
    fn max_net_object_count_mut(&mut self) -> &mut u32 {
        &mut self.max_net_object_count
    }
    fn max_static_net_object_count(&self) -> &u32 {
        &self.max_static_net_object_count
    }
    fn max_static_net_object_count_mut(&mut self) -> &mut u32 {
        &mut self.max_static_net_object_count
    }
    fn max_client_connection_count(&self) -> &u32 {
        &self.max_client_connection_count
    }
    fn max_client_connection_count_mut(&mut self) -> &mut u32 {
        &mut self.max_client_connection_count
    }
    fn max_server_connection_count(&self) -> &u32 {
        &self.max_server_connection_count
    }
    fn max_server_connection_count_mut(&mut self) -> &mut u32 {
        &mut self.max_server_connection_count
    }
    fn in_proc_replication_enabled(&self) -> &bool {
        &self.in_proc_replication_enabled
    }
    fn in_proc_replication_enabled_mut(&mut self) -> &mut bool {
        &mut self.in_proc_replication_enabled
    }
    fn in_proc_buffer_size(&self) -> &u32 {
        &self.in_proc_buffer_size
    }
    fn in_proc_buffer_size_mut(&mut self) -> &mut u32 {
        &mut self.in_proc_buffer_size
    }
    fn game_view_in_proc_buffer_size(&self) -> &u32 {
        &self.game_view_in_proc_buffer_size
    }
    fn game_view_in_proc_buffer_size_mut(&mut self) -> &mut u32 {
        &mut self.game_view_in_proc_buffer_size
    }
    fn max_remote_authority_net_object_count(&self) -> &u32 {
        &self.max_remote_authority_net_object_count
    }
    fn max_remote_authority_net_object_count_mut(&mut self) -> &mut u32 {
        &mut self.max_remote_authority_net_object_count
    }
    fn default_dynamic_priority_method(&self) -> &i32 {
        &self.default_dynamic_priority_method
    }
    fn default_dynamic_priority_method_mut(&mut self) -> &mut i32 {
        &mut self.default_dynamic_priority_method
    }
    fn priority_settings(&self) -> &NetObjectPrioritySettings {
        &self.priority_settings
    }
    fn priority_settings_mut(&mut self) -> &mut NetObjectPrioritySettings {
        &mut self.priority_settings
    }
    fn default_filter_method(&self) -> &i32 {
        &self.default_filter_method
    }
    fn default_filter_method_mut(&mut self) -> &mut i32 {
        &mut self.default_filter_method
    }
    fn delta_compression_settings(&self) -> &DeltaCompressionSettings {
        &self.delta_compression_settings
    }
    fn delta_compression_settings_mut(&mut self) -> &mut DeltaCompressionSettings {
        &mut self.delta_compression_settings
    }
    fn debug(&self) -> &NetObjectSystemDebugSettings {
        &self.debug
    }
    fn debug_mut(&mut self) -> &mut NetObjectSystemDebugSettings {
        &mut self.debug
    }
}

impl super::core::DataContainerTrait for NetObjectSystemSettings {
}

pub static NETOBJECTSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemSettings",
    name_hash: 2656923439,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(NetObjectSystemSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetObjectSystemSettings as Default>::default())),
            create_boxed: || Box::new(<NetObjectSystemSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxNetObjectCount",
                name_hash: 114573048,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_net_object_count),
            },
            FieldInfoData {
                name: "MaxStaticNetObjectCount",
                name_hash: 2158798752,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_static_net_object_count),
            },
            FieldInfoData {
                name: "MaxClientConnectionCount",
                name_hash: 2983938397,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_client_connection_count),
            },
            FieldInfoData {
                name: "MaxServerConnectionCount",
                name_hash: 1572697729,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_server_connection_count),
            },
            FieldInfoData {
                name: "InProcReplicationEnabled",
                name_hash: 3273877269,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemSettings, in_proc_replication_enabled),
            },
            FieldInfoData {
                name: "InProcBufferSize",
                name_hash: 3394060937,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, in_proc_buffer_size),
            },
            FieldInfoData {
                name: "GameViewInProcBufferSize",
                name_hash: 4250647594,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, game_view_in_proc_buffer_size),
            },
            FieldInfoData {
                name: "MaxRemoteAuthorityNetObjectCount",
                name_hash: 1936911405,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_remote_authority_net_object_count),
            },
            FieldInfoData {
                name: "DefaultDynamicPriorityMethod",
                name_hash: 1901858614,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(NetObjectSystemSettings, default_dynamic_priority_method),
            },
            FieldInfoData {
                name: "PrioritySettings",
                name_hash: 1452554962,
                flags: MemberInfoFlags::new(0),
                field_type: "NetObjectPrioritySettings",
                rust_offset: offset_of!(NetObjectSystemSettings, priority_settings),
            },
            FieldInfoData {
                name: "DefaultFilterMethod",
                name_hash: 2026384401,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(NetObjectSystemSettings, default_filter_method),
            },
            FieldInfoData {
                name: "DeltaCompressionSettings",
                name_hash: 158055158,
                flags: MemberInfoFlags::new(0),
                field_type: "DeltaCompressionSettings",
                rust_offset: offset_of!(NetObjectSystemSettings, delta_compression_settings),
            },
            FieldInfoData {
                name: "Debug",
                name_hash: 208762356,
                flags: MemberInfoFlags::new(0),
                field_type: "NetObjectSystemDebugSettings",
                rust_offset: offset_of!(NetObjectSystemSettings, debug),
            },
        ],
    }),
    array_type: Some(NETOBJECTSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetObjectSystemSettings {
    fn type_info(&self) -> &'static TypeInfo {
        NETOBJECTSYSTEMSETTINGS_TYPE_INFO
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


pub static NETOBJECTSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemSettings-Array",
    name_hash: 4202790043,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetObjectSystemDebugSettings {
    pub enable_replication_warnings: bool,
    pub enable_incoming_replication_status_report: bool,
    pub incoming_replication_status_report_max_delta: f32,
    pub incoming_replication_status_report_filter: String,
    pub incoming_replication_status_report_include_spatial: bool,
    pub incoming_replication_status_report_include_static: bool,
    pub incoming_replication_status_report_include_non_spatial: bool,
    pub incoming_replication_status_report_draw_name: bool,
    pub output_object_protocols: bool,
    pub initial_grace_time_in_frames: u32,
    pub report_replication_warnings_after_frames: u32,
    pub warn_on_missing_init_dependency: bool,
    pub warn_on_too_large_net_object: bool,
    pub warn_on_no_state_can_be_sent: bool,
    pub warn_on_waiting_for_creation_ack: bool,
}

pub trait NetObjectSystemDebugSettingsTrait: TypeObject {
    fn enable_replication_warnings(&self) -> &bool;
    fn enable_replication_warnings_mut(&mut self) -> &mut bool;
    fn enable_incoming_replication_status_report(&self) -> &bool;
    fn enable_incoming_replication_status_report_mut(&mut self) -> &mut bool;
    fn incoming_replication_status_report_max_delta(&self) -> &f32;
    fn incoming_replication_status_report_max_delta_mut(&mut self) -> &mut f32;
    fn incoming_replication_status_report_filter(&self) -> &String;
    fn incoming_replication_status_report_filter_mut(&mut self) -> &mut String;
    fn incoming_replication_status_report_include_spatial(&self) -> &bool;
    fn incoming_replication_status_report_include_spatial_mut(&mut self) -> &mut bool;
    fn incoming_replication_status_report_include_static(&self) -> &bool;
    fn incoming_replication_status_report_include_static_mut(&mut self) -> &mut bool;
    fn incoming_replication_status_report_include_non_spatial(&self) -> &bool;
    fn incoming_replication_status_report_include_non_spatial_mut(&mut self) -> &mut bool;
    fn incoming_replication_status_report_draw_name(&self) -> &bool;
    fn incoming_replication_status_report_draw_name_mut(&mut self) -> &mut bool;
    fn output_object_protocols(&self) -> &bool;
    fn output_object_protocols_mut(&mut self) -> &mut bool;
    fn initial_grace_time_in_frames(&self) -> &u32;
    fn initial_grace_time_in_frames_mut(&mut self) -> &mut u32;
    fn report_replication_warnings_after_frames(&self) -> &u32;
    fn report_replication_warnings_after_frames_mut(&mut self) -> &mut u32;
    fn warn_on_missing_init_dependency(&self) -> &bool;
    fn warn_on_missing_init_dependency_mut(&mut self) -> &mut bool;
    fn warn_on_too_large_net_object(&self) -> &bool;
    fn warn_on_too_large_net_object_mut(&mut self) -> &mut bool;
    fn warn_on_no_state_can_be_sent(&self) -> &bool;
    fn warn_on_no_state_can_be_sent_mut(&mut self) -> &mut bool;
    fn warn_on_waiting_for_creation_ack(&self) -> &bool;
    fn warn_on_waiting_for_creation_ack_mut(&mut self) -> &mut bool;
}

impl NetObjectSystemDebugSettingsTrait for NetObjectSystemDebugSettings {
    fn enable_replication_warnings(&self) -> &bool {
        &self.enable_replication_warnings
    }
    fn enable_replication_warnings_mut(&mut self) -> &mut bool {
        &mut self.enable_replication_warnings
    }
    fn enable_incoming_replication_status_report(&self) -> &bool {
        &self.enable_incoming_replication_status_report
    }
    fn enable_incoming_replication_status_report_mut(&mut self) -> &mut bool {
        &mut self.enable_incoming_replication_status_report
    }
    fn incoming_replication_status_report_max_delta(&self) -> &f32 {
        &self.incoming_replication_status_report_max_delta
    }
    fn incoming_replication_status_report_max_delta_mut(&mut self) -> &mut f32 {
        &mut self.incoming_replication_status_report_max_delta
    }
    fn incoming_replication_status_report_filter(&self) -> &String {
        &self.incoming_replication_status_report_filter
    }
    fn incoming_replication_status_report_filter_mut(&mut self) -> &mut String {
        &mut self.incoming_replication_status_report_filter
    }
    fn incoming_replication_status_report_include_spatial(&self) -> &bool {
        &self.incoming_replication_status_report_include_spatial
    }
    fn incoming_replication_status_report_include_spatial_mut(&mut self) -> &mut bool {
        &mut self.incoming_replication_status_report_include_spatial
    }
    fn incoming_replication_status_report_include_static(&self) -> &bool {
        &self.incoming_replication_status_report_include_static
    }
    fn incoming_replication_status_report_include_static_mut(&mut self) -> &mut bool {
        &mut self.incoming_replication_status_report_include_static
    }
    fn incoming_replication_status_report_include_non_spatial(&self) -> &bool {
        &self.incoming_replication_status_report_include_non_spatial
    }
    fn incoming_replication_status_report_include_non_spatial_mut(&mut self) -> &mut bool {
        &mut self.incoming_replication_status_report_include_non_spatial
    }
    fn incoming_replication_status_report_draw_name(&self) -> &bool {
        &self.incoming_replication_status_report_draw_name
    }
    fn incoming_replication_status_report_draw_name_mut(&mut self) -> &mut bool {
        &mut self.incoming_replication_status_report_draw_name
    }
    fn output_object_protocols(&self) -> &bool {
        &self.output_object_protocols
    }
    fn output_object_protocols_mut(&mut self) -> &mut bool {
        &mut self.output_object_protocols
    }
    fn initial_grace_time_in_frames(&self) -> &u32 {
        &self.initial_grace_time_in_frames
    }
    fn initial_grace_time_in_frames_mut(&mut self) -> &mut u32 {
        &mut self.initial_grace_time_in_frames
    }
    fn report_replication_warnings_after_frames(&self) -> &u32 {
        &self.report_replication_warnings_after_frames
    }
    fn report_replication_warnings_after_frames_mut(&mut self) -> &mut u32 {
        &mut self.report_replication_warnings_after_frames
    }
    fn warn_on_missing_init_dependency(&self) -> &bool {
        &self.warn_on_missing_init_dependency
    }
    fn warn_on_missing_init_dependency_mut(&mut self) -> &mut bool {
        &mut self.warn_on_missing_init_dependency
    }
    fn warn_on_too_large_net_object(&self) -> &bool {
        &self.warn_on_too_large_net_object
    }
    fn warn_on_too_large_net_object_mut(&mut self) -> &mut bool {
        &mut self.warn_on_too_large_net_object
    }
    fn warn_on_no_state_can_be_sent(&self) -> &bool {
        &self.warn_on_no_state_can_be_sent
    }
    fn warn_on_no_state_can_be_sent_mut(&mut self) -> &mut bool {
        &mut self.warn_on_no_state_can_be_sent
    }
    fn warn_on_waiting_for_creation_ack(&self) -> &bool {
        &self.warn_on_waiting_for_creation_ack
    }
    fn warn_on_waiting_for_creation_ack_mut(&mut self) -> &mut bool {
        &mut self.warn_on_waiting_for_creation_ack
    }
}

pub static NETOBJECTSYSTEMDEBUGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemDebugSettings",
    name_hash: 4238457150,
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetObjectSystemDebugSettings as Default>::default())),
            create_boxed: || Box::new(<NetObjectSystemDebugSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnableReplicationWarnings",
                name_hash: 3336095937,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, enable_replication_warnings),
            },
            FieldInfoData {
                name: "EnableIncomingReplicationStatusReport",
                name_hash: 2013071140,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, enable_incoming_replication_status_report),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportMaxDelta",
                name_hash: 4040482281,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_max_delta),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportFilter",
                name_hash: 3680027237,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_filter),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeSpatial",
                name_hash: 1106721547,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_spatial),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeStatic",
                name_hash: 3942787713,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_static),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeNonSpatial",
                name_hash: 1322820772,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_non_spatial),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportDrawName",
                name_hash: 2956613026,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_draw_name),
            },
            FieldInfoData {
                name: "OutputObjectProtocols",
                name_hash: 3513042698,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, output_object_protocols),
            },
            FieldInfoData {
                name: "InitialGraceTimeInFrames",
                name_hash: 1355287029,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, initial_grace_time_in_frames),
            },
            FieldInfoData {
                name: "ReportReplicationWarningsAfterFrames",
                name_hash: 2144099364,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, report_replication_warnings_after_frames),
            },
            FieldInfoData {
                name: "WarnOnMissingInitDependency",
                name_hash: 4228312863,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_missing_init_dependency),
            },
            FieldInfoData {
                name: "WarnOnTooLargeNetObject",
                name_hash: 3016126797,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_too_large_net_object),
            },
            FieldInfoData {
                name: "WarnOnNoStateCanBeSent",
                name_hash: 4178208863,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_no_state_can_be_sent),
            },
            FieldInfoData {
                name: "WarnOnWaitingForCreationAck",
                name_hash: 445089150,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_waiting_for_creation_ack),
            },
        ],
    }),
    array_type: Some(NETOBJECTSYSTEMDEBUGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetObjectSystemDebugSettings {
    fn type_info(&self) -> &'static TypeInfo {
        NETOBJECTSYSTEMDEBUGSETTINGS_TYPE_INFO
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


pub static NETOBJECTSYSTEMDEBUGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemDebugSettings-Array",
    name_hash: 1276168586,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSystemDebugSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DeltaCompressionSettings {
    pub is_enabled: bool,
    pub share_baselines_across_connections: bool,
    pub baseline_reuse_count: u32,
}

pub trait DeltaCompressionSettingsTrait: TypeObject {
    fn is_enabled(&self) -> &bool;
    fn is_enabled_mut(&mut self) -> &mut bool;
    fn share_baselines_across_connections(&self) -> &bool;
    fn share_baselines_across_connections_mut(&mut self) -> &mut bool;
    fn baseline_reuse_count(&self) -> &u32;
    fn baseline_reuse_count_mut(&mut self) -> &mut u32;
}

impl DeltaCompressionSettingsTrait for DeltaCompressionSettings {
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn is_enabled_mut(&mut self) -> &mut bool {
        &mut self.is_enabled
    }
    fn share_baselines_across_connections(&self) -> &bool {
        &self.share_baselines_across_connections
    }
    fn share_baselines_across_connections_mut(&mut self) -> &mut bool {
        &mut self.share_baselines_across_connections
    }
    fn baseline_reuse_count(&self) -> &u32 {
        &self.baseline_reuse_count
    }
    fn baseline_reuse_count_mut(&mut self) -> &mut u32 {
        &mut self.baseline_reuse_count
    }
}

pub static DELTACOMPRESSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaCompressionSettings",
    name_hash: 158055158,
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DeltaCompressionSettings as Default>::default())),
            create_boxed: || Box::new(<DeltaCompressionSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsEnabled",
                name_hash: 2323834330,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DeltaCompressionSettings, is_enabled),
            },
            FieldInfoData {
                name: "ShareBaselinesAcrossConnections",
                name_hash: 4242071194,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DeltaCompressionSettings, share_baselines_across_connections),
            },
            FieldInfoData {
                name: "BaselineReuseCount",
                name_hash: 364780841,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DeltaCompressionSettings, baseline_reuse_count),
            },
        ],
    }),
    array_type: Some(DELTACOMPRESSIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DeltaCompressionSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DELTACOMPRESSIONSETTINGS_TYPE_INFO
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


pub static DELTACOMPRESSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaCompressionSettings-Array",
    name_hash: 2994044866,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("DeltaCompressionSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum NetObjectDependencyType {
    #[default]
    NetObjectDependencyType_None = 0,
    NetObjectDependencyType_Init = 1,
    NetObjectDependencyType_InitFlushDependencyFirst = 2,
    NetObjectDependencyType_ResolveBeforeSend = 3,
    NetObjectDependencyType_SendDependencyFirst = 4,
    NetObjectDependencyType_FlushDependencyFirst = 5,
    NetObjectDependencyType_ResolveOnRemote = 6,
}

pub static NETOBJECTDEPENDENCYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectDependencyType",
    name_hash: 2775823864,
    flags: MemberInfoFlags::new(49429),
    module: "Network",
    data: TypeInfoData::Enum,
    array_type: Some(NETOBJECTDEPENDENCYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NetObjectDependencyType {
    fn type_info(&self) -> &'static TypeInfo {
        NETOBJECTDEPENDENCYTYPE_TYPE_INFO
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


pub static NETOBJECTDEPENDENCYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectDependencyType-Array",
    name_hash: 3638636492,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectDependencyType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetObjectPrioritySettings {
    pub min_frequency_factor: f32,
    pub max_frequency_factor: f32,
    pub min_frequency_factor_radius: f32,
    pub max_frequency_factor_radius: f32,
    pub max_frequency_factor_cone_radius: f32,
    pub min_cone_frequency_factor: f32,
    pub camera_fov_bias_degrees: f32,
    pub max_camera_fov_degrees: f32,
    pub min_camera_fov_degrees: f32,
}

pub trait NetObjectPrioritySettingsTrait: TypeObject {
    fn min_frequency_factor(&self) -> &f32;
    fn min_frequency_factor_mut(&mut self) -> &mut f32;
    fn max_frequency_factor(&self) -> &f32;
    fn max_frequency_factor_mut(&mut self) -> &mut f32;
    fn min_frequency_factor_radius(&self) -> &f32;
    fn min_frequency_factor_radius_mut(&mut self) -> &mut f32;
    fn max_frequency_factor_radius(&self) -> &f32;
    fn max_frequency_factor_radius_mut(&mut self) -> &mut f32;
    fn max_frequency_factor_cone_radius(&self) -> &f32;
    fn max_frequency_factor_cone_radius_mut(&mut self) -> &mut f32;
    fn min_cone_frequency_factor(&self) -> &f32;
    fn min_cone_frequency_factor_mut(&mut self) -> &mut f32;
    fn camera_fov_bias_degrees(&self) -> &f32;
    fn camera_fov_bias_degrees_mut(&mut self) -> &mut f32;
    fn max_camera_fov_degrees(&self) -> &f32;
    fn max_camera_fov_degrees_mut(&mut self) -> &mut f32;
    fn min_camera_fov_degrees(&self) -> &f32;
    fn min_camera_fov_degrees_mut(&mut self) -> &mut f32;
}

impl NetObjectPrioritySettingsTrait for NetObjectPrioritySettings {
    fn min_frequency_factor(&self) -> &f32 {
        &self.min_frequency_factor
    }
    fn min_frequency_factor_mut(&mut self) -> &mut f32 {
        &mut self.min_frequency_factor
    }
    fn max_frequency_factor(&self) -> &f32 {
        &self.max_frequency_factor
    }
    fn max_frequency_factor_mut(&mut self) -> &mut f32 {
        &mut self.max_frequency_factor
    }
    fn min_frequency_factor_radius(&self) -> &f32 {
        &self.min_frequency_factor_radius
    }
    fn min_frequency_factor_radius_mut(&mut self) -> &mut f32 {
        &mut self.min_frequency_factor_radius
    }
    fn max_frequency_factor_radius(&self) -> &f32 {
        &self.max_frequency_factor_radius
    }
    fn max_frequency_factor_radius_mut(&mut self) -> &mut f32 {
        &mut self.max_frequency_factor_radius
    }
    fn max_frequency_factor_cone_radius(&self) -> &f32 {
        &self.max_frequency_factor_cone_radius
    }
    fn max_frequency_factor_cone_radius_mut(&mut self) -> &mut f32 {
        &mut self.max_frequency_factor_cone_radius
    }
    fn min_cone_frequency_factor(&self) -> &f32 {
        &self.min_cone_frequency_factor
    }
    fn min_cone_frequency_factor_mut(&mut self) -> &mut f32 {
        &mut self.min_cone_frequency_factor
    }
    fn camera_fov_bias_degrees(&self) -> &f32 {
        &self.camera_fov_bias_degrees
    }
    fn camera_fov_bias_degrees_mut(&mut self) -> &mut f32 {
        &mut self.camera_fov_bias_degrees
    }
    fn max_camera_fov_degrees(&self) -> &f32 {
        &self.max_camera_fov_degrees
    }
    fn max_camera_fov_degrees_mut(&mut self) -> &mut f32 {
        &mut self.max_camera_fov_degrees
    }
    fn min_camera_fov_degrees(&self) -> &f32 {
        &self.min_camera_fov_degrees
    }
    fn min_camera_fov_degrees_mut(&mut self) -> &mut f32 {
        &mut self.min_camera_fov_degrees
    }
}

pub static NETOBJECTPRIORITYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectPrioritySettings",
    name_hash: 981067256,
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetObjectPrioritySettings as Default>::default())),
            create_boxed: || Box::new(<NetObjectPrioritySettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MinFrequencyFactor",
                name_hash: 950240998,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, min_frequency_factor),
            },
            FieldInfoData {
                name: "MaxFrequencyFactor",
                name_hash: 2744889912,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor),
            },
            FieldInfoData {
                name: "MinFrequencyFactorRadius",
                name_hash: 3133919230,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, min_frequency_factor_radius),
            },
            FieldInfoData {
                name: "MaxFrequencyFactorRadius",
                name_hash: 4158767520,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor_radius),
            },
            FieldInfoData {
                name: "MaxFrequencyFactorConeRadius",
                name_hash: 3335439719,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor_cone_radius),
            },
            FieldInfoData {
                name: "MinConeFrequencyFactor",
                name_hash: 2784041281,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, min_cone_frequency_factor),
            },
            FieldInfoData {
                name: "CameraFovBiasDegrees",
                name_hash: 3359994077,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, camera_fov_bias_degrees),
            },
            FieldInfoData {
                name: "MaxCameraFovDegrees",
                name_hash: 1872683184,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_camera_fov_degrees),
            },
            FieldInfoData {
                name: "MinCameraFovDegrees",
                name_hash: 2783691566,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, min_camera_fov_degrees),
            },
        ],
    }),
    array_type: Some(NETOBJECTPRIORITYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for NetObjectPrioritySettings {
    fn type_info(&self) -> &'static TypeInfo {
        NETOBJECTPRIORITYSETTINGS_TYPE_INFO
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


pub static NETOBJECTPRIORITYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectPrioritySettings-Array",
    name_hash: 2244698060,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectPrioritySettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum NetObjectSendStatus {
    #[default]
    NetObjectSendStatus_Pause = 0,
    NetObjectSendStatus_Send = 1,
    NetObjectSendStatus_DeleteAll = 2,
}

pub static NETOBJECTSENDSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSendStatus",
    name_hash: 593998055,
    flags: MemberInfoFlags::new(49429),
    module: "Network",
    data: TypeInfoData::Enum,
    array_type: Some(NETOBJECTSENDSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NetObjectSendStatus {
    fn type_info(&self) -> &'static TypeInfo {
        NETOBJECTSENDSTATUS_TYPE_INFO
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


pub static NETOBJECTSENDSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSendStatus-Array",
    name_hash: 2859119315,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSendStatus"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum NetworkChannelId {
    #[default]
    NetworkChannelId_Invalid = 0,
    NetworkChannelId_ValidOffset = 1,
}

pub static NETWORKCHANNELID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkChannelId",
    name_hash: 2246145845,
    flags: MemberInfoFlags::new(49429),
    module: "Network",
    data: TypeInfoData::Enum,
    array_type: Some(NETWORKCHANNELID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NetworkChannelId {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKCHANNELID_TYPE_INFO
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


pub static NETWORKCHANNELID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkChannelId-Array",
    name_hash: 969308033,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkChannelId"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SyncedTransformEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub interpolate: bool,
}

pub trait SyncedTransformEntityDataTrait: super::entity::EntityDataTrait {
    fn interpolate(&self) -> &bool;
    fn interpolate_mut(&mut self) -> &mut bool;
}

impl SyncedTransformEntityDataTrait for SyncedTransformEntityData {
    fn interpolate(&self) -> &bool {
        &self.interpolate
    }
    fn interpolate_mut(&mut self) -> &mut bool {
        &mut self.interpolate
    }
}

impl super::entity::EntityDataTrait for SyncedTransformEntityData {
}

impl super::entity::GameObjectDataTrait for SyncedTransformEntityData {
}

impl super::core::DataBusPeerTrait for SyncedTransformEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SyncedTransformEntityData {
}

impl super::core::DataContainerTrait for SyncedTransformEntityData {
}

pub static SYNCEDTRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedTransformEntityData",
    name_hash: 1832943140,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SyncedTransformEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedTransformEntityData as Default>::default())),
            create_boxed: || Box::new(<SyncedTransformEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Interpolate",
                name_hash: 2332438914,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SyncedTransformEntityData, interpolate),
            },
        ],
    }),
    array_type: Some(SYNCEDTRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedTransformEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SYNCEDTRANSFORMENTITYDATA_TYPE_INFO
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


pub static SYNCEDTRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedTransformEntityData-Array",
    name_hash: 530887056,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedTransformEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SyncedIntEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait SyncedIntEntityDataTrait: super::entity::EntityDataTrait {
}

impl SyncedIntEntityDataTrait for SyncedIntEntityData {
}

impl super::entity::EntityDataTrait for SyncedIntEntityData {
}

impl super::entity::GameObjectDataTrait for SyncedIntEntityData {
}

impl super::core::DataBusPeerTrait for SyncedIntEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SyncedIntEntityData {
}

impl super::core::DataContainerTrait for SyncedIntEntityData {
}

pub static SYNCEDINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedIntEntityData",
    name_hash: 4229977915,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SyncedIntEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedIntEntityData as Default>::default())),
            create_boxed: || Box::new(<SyncedIntEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SYNCEDINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedIntEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SYNCEDINTENTITYDATA_TYPE_INFO
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


pub static SYNCEDINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedIntEntityData-Array",
    name_hash: 2535816847,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedIntEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SyncedFloatEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait SyncedFloatEntityDataTrait: super::entity::EntityDataTrait {
}

impl SyncedFloatEntityDataTrait for SyncedFloatEntityData {
}

impl super::entity::EntityDataTrait for SyncedFloatEntityData {
}

impl super::entity::GameObjectDataTrait for SyncedFloatEntityData {
}

impl super::core::DataBusPeerTrait for SyncedFloatEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SyncedFloatEntityData {
}

impl super::core::DataContainerTrait for SyncedFloatEntityData {
}

pub static SYNCEDFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedFloatEntityData",
    name_hash: 1238364696,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SyncedFloatEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedFloatEntityData as Default>::default())),
            create_boxed: || Box::new(<SyncedFloatEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SYNCEDFLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedFloatEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SYNCEDFLOATENTITYDATA_TYPE_INFO
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


pub static SYNCEDFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedFloatEntityData-Array",
    name_hash: 758004780,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedFloatEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SyncedBoolEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait SyncedBoolEntityDataTrait: super::entity::EntityDataTrait {
}

impl SyncedBoolEntityDataTrait for SyncedBoolEntityData {
}

impl super::entity::EntityDataTrait for SyncedBoolEntityData {
}

impl super::entity::GameObjectDataTrait for SyncedBoolEntityData {
}

impl super::core::DataBusPeerTrait for SyncedBoolEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SyncedBoolEntityData {
}

impl super::core::DataContainerTrait for SyncedBoolEntityData {
}

pub static SYNCEDBOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedBoolEntityData",
    name_hash: 3175993478,
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(SyncedBoolEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedBoolEntityData as Default>::default())),
            create_boxed: || Box::new(<SyncedBoolEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SYNCEDBOOLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedBoolEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SYNCEDBOOLENTITYDATA_TYPE_INFO
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


pub static SYNCEDBOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedBoolEntityData-Array",
    name_hash: 2970795442,
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedBoolEntityData"),
    array_type: None,
    alignment: 8,
};


