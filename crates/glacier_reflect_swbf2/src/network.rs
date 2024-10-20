use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedTransformEntity as Default>::default())),
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
}


pub static CLIENTSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedTransformEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedTransformEntity as Default>::default())),
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
}


pub static SERVERSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedTransformEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedIntEntity as Default>::default())),
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
}


pub static CLIENTSYNCEDINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedIntEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedIntEntity as Default>::default())),
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
}


pub static SERVERSYNCEDINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedIntEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedFloatEntity as Default>::default())),
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
}


pub static CLIENTSYNCEDFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedFloatEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedFloatEntity as Default>::default())),
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
}


pub static SERVERSYNCEDFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedFloatEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientSyncedBoolEntity as Default>::default())),
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
}


pub static CLIENTSYNCEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedBoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedBoolEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerSyncedBoolEntity as Default>::default())),
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
}


pub static SERVERSYNCEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedBoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedBoolEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONNECTION_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EngineConnectionPeer as Default>::default())),
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
}


pub static ENGINECONNECTIONPEER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnectionPeer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("EngineConnectionPeer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EngineConnection {
}

pub trait EngineConnectionTrait: TypeObject {
}

impl EngineConnectionTrait for EngineConnection {
}

pub static ENGINECONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnection",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EngineConnection as Default>::default())),
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
}


pub static ENGINECONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("EngineConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpikeInternalMessagePartMessage {
}

pub trait SpikeInternalMessagePartMessageTrait: TypeObject {
}

impl SpikeInternalMessagePartMessageTrait for SpikeInternalMessagePartMessage {
}

pub static SPIKEINTERNALMESSAGEPARTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpikeInternalMessagePartMessage",
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpikeInternalMessagePartMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct SpikeInternalMessageWrapperMessage {
}

pub trait SpikeInternalMessageWrapperMessageTrait: TypeObject {
}

impl SpikeInternalMessageWrapperMessageTrait for SpikeInternalMessageWrapperMessage {
}

pub static SPIKEINTERNALMESSAGEWRAPPERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpikeInternalMessageWrapperMessage",
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpikeInternalMessageWrapperMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
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
    fn graph_pos(&self) -> &super::core::Vec2;
    fn update_frequency(&self) -> &f32;
    fn high_latency(&self) -> &f32;
    fn critical_latency(&self) -> &f32;
    fn high_latency_variation(&self) -> &f32;
    fn critical_latency_variation(&self) -> &f32;
    fn high_packet_loss_ratio(&self) -> &f32;
    fn critical_packet_loss_ratio(&self) -> &f32;
    fn server_fps_low_threshold_perc(&self) -> &f32;
    fn server_fps_low_threshold_crit_perc(&self) -> &f32;
}

impl NetworkPerfOverlaySettingsTrait for NetworkPerfOverlaySettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn graph_pos(&self) -> &super::core::Vec2 {
        &self.graph_pos
    }
    fn update_frequency(&self) -> &f32 {
        &self.update_frequency
    }
    fn high_latency(&self) -> &f32 {
        &self.high_latency
    }
    fn critical_latency(&self) -> &f32 {
        &self.critical_latency
    }
    fn high_latency_variation(&self) -> &f32 {
        &self.high_latency_variation
    }
    fn critical_latency_variation(&self) -> &f32 {
        &self.critical_latency_variation
    }
    fn high_packet_loss_ratio(&self) -> &f32 {
        &self.high_packet_loss_ratio
    }
    fn critical_packet_loss_ratio(&self) -> &f32 {
        &self.critical_packet_loss_ratio
    }
    fn server_fps_low_threshold_perc(&self) -> &f32 {
        &self.server_fps_low_threshold_perc
    }
    fn server_fps_low_threshold_crit_perc(&self) -> &f32 {
        &self.server_fps_low_threshold_crit_perc
    }
}

impl super::core::DataContainerTrait for NetworkPerfOverlaySettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NETWORKPERFOVERLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerfOverlaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkPerfOverlaySettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, enable),
            },
            FieldInfoData {
                name: "GraphPos",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, graph_pos),
            },
            FieldInfoData {
                name: "UpdateFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, update_frequency),
            },
            FieldInfoData {
                name: "HighLatency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_latency),
            },
            FieldInfoData {
                name: "CriticalLatency",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_latency),
            },
            FieldInfoData {
                name: "HighLatencyVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_latency_variation),
            },
            FieldInfoData {
                name: "CriticalLatencyVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_latency_variation),
            },
            FieldInfoData {
                name: "HighPacketLossRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_packet_loss_ratio),
            },
            FieldInfoData {
                name: "CriticalPacketLossRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_packet_loss_ratio),
            },
            FieldInfoData {
                name: "ServerFpsLowThresholdPerc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetworkPerfOverlaySettings, server_fps_low_threshold_perc),
            },
            FieldInfoData {
                name: "ServerFpsLowThresholdCritPerc",
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
}


pub static NETWORKPERFOVERLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerfOverlaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkPerfOverlaySettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn time_nudge_p_i_d_const_k_pos(&self) -> &f32;
    fn time_nudge_p_i_d_const_k_neg(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_i_pos(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_i_neg(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_d_pos(&self) -> &f32;
    fn time_nudge_p_i_d_const_t_d_neg(&self) -> &f32;
    fn time_nudge_p_i_d_latency_tol(&self) -> &f32;
    fn time_nudge_p_i_d_packet_delta_time_tol(&self) -> &f32;
    fn time_nudge_p_i_d_max_change_per_sec(&self) -> &f32;
    fn average_packet_sample_count(&self) -> &i32;
}

impl InterpolationManagerSettingsTrait for InterpolationManagerSettings {
    fn time_nudge_calculator(&self) -> &i32 {
        &self.time_nudge_calculator
    }
    fn time_nudge_p_i_d_const_k_pos(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_k_pos
    }
    fn time_nudge_p_i_d_const_k_neg(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_k_neg
    }
    fn time_nudge_p_i_d_const_t_i_pos(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_i_pos
    }
    fn time_nudge_p_i_d_const_t_i_neg(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_i_neg
    }
    fn time_nudge_p_i_d_const_t_d_pos(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_d_pos
    }
    fn time_nudge_p_i_d_const_t_d_neg(&self) -> &f32 {
        &self.time_nudge_p_i_d_const_t_d_neg
    }
    fn time_nudge_p_i_d_latency_tol(&self) -> &f32 {
        &self.time_nudge_p_i_d_latency_tol
    }
    fn time_nudge_p_i_d_packet_delta_time_tol(&self) -> &f32 {
        &self.time_nudge_p_i_d_packet_delta_time_tol
    }
    fn time_nudge_p_i_d_max_change_per_sec(&self) -> &f32 {
        &self.time_nudge_p_i_d_max_change_per_sec
    }
    fn average_packet_sample_count(&self) -> &i32 {
        &self.average_packet_sample_count
    }
}

impl super::core::DataContainerTrait for InterpolationManagerSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static INTERPOLATIONMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationManagerSettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InterpolationManagerSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TimeNudgeCalculator",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_calculator),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstKPos",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_k_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstKNeg",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_k_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTIPos",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_i_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTINeg",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_i_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTDPos",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_d_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTDNeg",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_d_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDLatencyTol",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_latency_tol),
            },
            FieldInfoData {
                name: "TimeNudgePIDPacketDeltaTimeTol",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_packet_delta_time_tol),
            },
            FieldInfoData {
                name: "TimeNudgePIDMaxChangePerSec",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_max_change_per_sec),
            },
            FieldInfoData {
                name: "AveragePacketSampleCount",
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
}


pub static INTERPOLATIONMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationManagerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("InterpolationManagerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn reorder_ratio(&self) -> &f32;
    fn latency_min(&self) -> &f32;
    fn latency_max(&self) -> &f32;
    fn duplicate_ratio(&self) -> &f32;
    fn drop_ratio(&self) -> &f32;
    fn corrupt_ratio(&self) -> &f32;
    fn size_ratio(&self) -> &f32;
    fn spike_duration_min(&self) -> &f32;
    fn spike_duration_max(&self) -> &f32;
    fn spike_cooldown_min(&self) -> &f32;
    fn spike_cooldown_max(&self) -> &f32;
    fn bandwidth_max(&self) -> &f32;
    fn bandwidth_delay_max(&self) -> &f32;
}

impl InternetSimulationStateTrait for InternetSimulationState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn reorder_ratio(&self) -> &f32 {
        &self.reorder_ratio
    }
    fn latency_min(&self) -> &f32 {
        &self.latency_min
    }
    fn latency_max(&self) -> &f32 {
        &self.latency_max
    }
    fn duplicate_ratio(&self) -> &f32 {
        &self.duplicate_ratio
    }
    fn drop_ratio(&self) -> &f32 {
        &self.drop_ratio
    }
    fn corrupt_ratio(&self) -> &f32 {
        &self.corrupt_ratio
    }
    fn size_ratio(&self) -> &f32 {
        &self.size_ratio
    }
    fn spike_duration_min(&self) -> &f32 {
        &self.spike_duration_min
    }
    fn spike_duration_max(&self) -> &f32 {
        &self.spike_duration_max
    }
    fn spike_cooldown_min(&self) -> &f32 {
        &self.spike_cooldown_min
    }
    fn spike_cooldown_max(&self) -> &f32 {
        &self.spike_cooldown_max
    }
    fn bandwidth_max(&self) -> &f32 {
        &self.bandwidth_max
    }
    fn bandwidth_delay_max(&self) -> &f32 {
        &self.bandwidth_delay_max
    }
}

pub static INTERNETSIMULATIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternetSimulationState",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InternetSimulationState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InternetSimulationState, enabled),
            },
            FieldInfoData {
                name: "ReorderRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, reorder_ratio),
            },
            FieldInfoData {
                name: "LatencyMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, latency_min),
            },
            FieldInfoData {
                name: "LatencyMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, latency_max),
            },
            FieldInfoData {
                name: "DuplicateRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, duplicate_ratio),
            },
            FieldInfoData {
                name: "DropRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, drop_ratio),
            },
            FieldInfoData {
                name: "CorruptRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, corrupt_ratio),
            },
            FieldInfoData {
                name: "SizeRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, size_ratio),
            },
            FieldInfoData {
                name: "SpikeDurationMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_duration_min),
            },
            FieldInfoData {
                name: "SpikeDurationMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_duration_max),
            },
            FieldInfoData {
                name: "SpikeCooldownMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_cooldown_min),
            },
            FieldInfoData {
                name: "SpikeCooldownMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, spike_cooldown_max),
            },
            FieldInfoData {
                name: "BandwidthMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InternetSimulationState, bandwidth_max),
            },
            FieldInfoData {
                name: "BandwidthDelayMax",
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
}


pub static INTERNETSIMULATIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternetSimulationState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("InternetSimulationState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NetworkCoreSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub dedicated_server_max_send_job_count: u32,
    pub server_max_send_job_count: u32,
}

pub trait NetworkCoreSettingsTrait: super::core::SystemSettingsTrait {
    fn dedicated_server_max_send_job_count(&self) -> &u32;
    fn server_max_send_job_count(&self) -> &u32;
}

impl NetworkCoreSettingsTrait for NetworkCoreSettings {
    fn dedicated_server_max_send_job_count(&self) -> &u32 {
        &self.dedicated_server_max_send_job_count
    }
    fn server_max_send_job_count(&self) -> &u32 {
        &self.server_max_send_job_count
    }
}

impl super::core::SystemSettingsTrait for NetworkCoreSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for NetworkCoreSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NETWORKCORESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCoreSettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkCoreSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DedicatedServerMaxSendJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetworkCoreSettings, dedicated_server_max_send_job_count),
            },
            FieldInfoData {
                name: "ServerMaxSendJobCount",
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
}


pub static NETWORKCORESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCoreSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkCoreSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoreDemoStatusMessage {
}

pub trait CoreDemoStatusMessageTrait: TypeObject {
}

impl CoreDemoStatusMessageTrait for CoreDemoStatusMessage {
}

pub static COREDEMOSTATUSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreDemoStatusMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreDemoStatusMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
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
    fn max_static_net_object_count(&self) -> &u32;
    fn max_client_connection_count(&self) -> &u32;
    fn max_server_connection_count(&self) -> &u32;
    fn in_proc_replication_enabled(&self) -> &bool;
    fn in_proc_buffer_size(&self) -> &u32;
    fn game_view_in_proc_buffer_size(&self) -> &u32;
    fn max_remote_authority_net_object_count(&self) -> &u32;
    fn default_dynamic_priority_method(&self) -> &i32;
    fn priority_settings(&self) -> &NetObjectPrioritySettings;
    fn default_filter_method(&self) -> &i32;
    fn delta_compression_settings(&self) -> &DeltaCompressionSettings;
    fn debug(&self) -> &NetObjectSystemDebugSettings;
}

impl NetObjectSystemSettingsTrait for NetObjectSystemSettings {
    fn max_net_object_count(&self) -> &u32 {
        &self.max_net_object_count
    }
    fn max_static_net_object_count(&self) -> &u32 {
        &self.max_static_net_object_count
    }
    fn max_client_connection_count(&self) -> &u32 {
        &self.max_client_connection_count
    }
    fn max_server_connection_count(&self) -> &u32 {
        &self.max_server_connection_count
    }
    fn in_proc_replication_enabled(&self) -> &bool {
        &self.in_proc_replication_enabled
    }
    fn in_proc_buffer_size(&self) -> &u32 {
        &self.in_proc_buffer_size
    }
    fn game_view_in_proc_buffer_size(&self) -> &u32 {
        &self.game_view_in_proc_buffer_size
    }
    fn max_remote_authority_net_object_count(&self) -> &u32 {
        &self.max_remote_authority_net_object_count
    }
    fn default_dynamic_priority_method(&self) -> &i32 {
        &self.default_dynamic_priority_method
    }
    fn priority_settings(&self) -> &NetObjectPrioritySettings {
        &self.priority_settings
    }
    fn default_filter_method(&self) -> &i32 {
        &self.default_filter_method
    }
    fn delta_compression_settings(&self) -> &DeltaCompressionSettings {
        &self.delta_compression_settings
    }
    fn debug(&self) -> &NetObjectSystemDebugSettings {
        &self.debug
    }
}

impl super::core::DataContainerTrait for NetObjectSystemSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NETOBJECTSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetObjectSystemSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxNetObjectCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_net_object_count),
            },
            FieldInfoData {
                name: "MaxStaticNetObjectCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_static_net_object_count),
            },
            FieldInfoData {
                name: "MaxClientConnectionCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_client_connection_count),
            },
            FieldInfoData {
                name: "MaxServerConnectionCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_server_connection_count),
            },
            FieldInfoData {
                name: "InProcReplicationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemSettings, in_proc_replication_enabled),
            },
            FieldInfoData {
                name: "InProcBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, in_proc_buffer_size),
            },
            FieldInfoData {
                name: "GameViewInProcBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, game_view_in_proc_buffer_size),
            },
            FieldInfoData {
                name: "MaxRemoteAuthorityNetObjectCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemSettings, max_remote_authority_net_object_count),
            },
            FieldInfoData {
                name: "DefaultDynamicPriorityMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(NetObjectSystemSettings, default_dynamic_priority_method),
            },
            FieldInfoData {
                name: "PrioritySettings",
                flags: MemberInfoFlags::new(0),
                field_type: "NetObjectPrioritySettings",
                rust_offset: offset_of!(NetObjectSystemSettings, priority_settings),
            },
            FieldInfoData {
                name: "DefaultFilterMethod",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(NetObjectSystemSettings, default_filter_method),
            },
            FieldInfoData {
                name: "DeltaCompressionSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "DeltaCompressionSettings",
                rust_offset: offset_of!(NetObjectSystemSettings, delta_compression_settings),
            },
            FieldInfoData {
                name: "Debug",
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
}


pub static NETOBJECTSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn enable_incoming_replication_status_report(&self) -> &bool;
    fn incoming_replication_status_report_max_delta(&self) -> &f32;
    fn incoming_replication_status_report_filter(&self) -> &String;
    fn incoming_replication_status_report_include_spatial(&self) -> &bool;
    fn incoming_replication_status_report_include_static(&self) -> &bool;
    fn incoming_replication_status_report_include_non_spatial(&self) -> &bool;
    fn incoming_replication_status_report_draw_name(&self) -> &bool;
    fn output_object_protocols(&self) -> &bool;
    fn initial_grace_time_in_frames(&self) -> &u32;
    fn report_replication_warnings_after_frames(&self) -> &u32;
    fn warn_on_missing_init_dependency(&self) -> &bool;
    fn warn_on_too_large_net_object(&self) -> &bool;
    fn warn_on_no_state_can_be_sent(&self) -> &bool;
    fn warn_on_waiting_for_creation_ack(&self) -> &bool;
}

impl NetObjectSystemDebugSettingsTrait for NetObjectSystemDebugSettings {
    fn enable_replication_warnings(&self) -> &bool {
        &self.enable_replication_warnings
    }
    fn enable_incoming_replication_status_report(&self) -> &bool {
        &self.enable_incoming_replication_status_report
    }
    fn incoming_replication_status_report_max_delta(&self) -> &f32 {
        &self.incoming_replication_status_report_max_delta
    }
    fn incoming_replication_status_report_filter(&self) -> &String {
        &self.incoming_replication_status_report_filter
    }
    fn incoming_replication_status_report_include_spatial(&self) -> &bool {
        &self.incoming_replication_status_report_include_spatial
    }
    fn incoming_replication_status_report_include_static(&self) -> &bool {
        &self.incoming_replication_status_report_include_static
    }
    fn incoming_replication_status_report_include_non_spatial(&self) -> &bool {
        &self.incoming_replication_status_report_include_non_spatial
    }
    fn incoming_replication_status_report_draw_name(&self) -> &bool {
        &self.incoming_replication_status_report_draw_name
    }
    fn output_object_protocols(&self) -> &bool {
        &self.output_object_protocols
    }
    fn initial_grace_time_in_frames(&self) -> &u32 {
        &self.initial_grace_time_in_frames
    }
    fn report_replication_warnings_after_frames(&self) -> &u32 {
        &self.report_replication_warnings_after_frames
    }
    fn warn_on_missing_init_dependency(&self) -> &bool {
        &self.warn_on_missing_init_dependency
    }
    fn warn_on_too_large_net_object(&self) -> &bool {
        &self.warn_on_too_large_net_object
    }
    fn warn_on_no_state_can_be_sent(&self) -> &bool {
        &self.warn_on_no_state_can_be_sent
    }
    fn warn_on_waiting_for_creation_ack(&self) -> &bool {
        &self.warn_on_waiting_for_creation_ack
    }
}

pub static NETOBJECTSYSTEMDEBUGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemDebugSettings",
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetObjectSystemDebugSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EnableReplicationWarnings",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, enable_replication_warnings),
            },
            FieldInfoData {
                name: "EnableIncomingReplicationStatusReport",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, enable_incoming_replication_status_report),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportMaxDelta",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_max_delta),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_filter),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeSpatial",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_spatial),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeStatic",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_static),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeNonSpatial",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_non_spatial),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportDrawName",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_draw_name),
            },
            FieldInfoData {
                name: "OutputObjectProtocols",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, output_object_protocols),
            },
            FieldInfoData {
                name: "InitialGraceTimeInFrames",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, initial_grace_time_in_frames),
            },
            FieldInfoData {
                name: "ReportReplicationWarningsAfterFrames",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, report_replication_warnings_after_frames),
            },
            FieldInfoData {
                name: "WarnOnMissingInitDependency",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_missing_init_dependency),
            },
            FieldInfoData {
                name: "WarnOnTooLargeNetObject",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_too_large_net_object),
            },
            FieldInfoData {
                name: "WarnOnNoStateCanBeSent",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_no_state_can_be_sent),
            },
            FieldInfoData {
                name: "WarnOnWaitingForCreationAck",
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
}


pub static NETOBJECTSYSTEMDEBUGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemDebugSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSystemDebugSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DeltaCompressionSettings {
    pub is_enabled: bool,
    pub share_baselines_across_connections: bool,
    pub baseline_reuse_count: u32,
}

pub trait DeltaCompressionSettingsTrait: TypeObject {
    fn is_enabled(&self) -> &bool;
    fn share_baselines_across_connections(&self) -> &bool;
    fn baseline_reuse_count(&self) -> &u32;
}

impl DeltaCompressionSettingsTrait for DeltaCompressionSettings {
    fn is_enabled(&self) -> &bool {
        &self.is_enabled
    }
    fn share_baselines_across_connections(&self) -> &bool {
        &self.share_baselines_across_connections
    }
    fn baseline_reuse_count(&self) -> &u32 {
        &self.baseline_reuse_count
    }
}

pub static DELTACOMPRESSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaCompressionSettings",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DeltaCompressionSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DeltaCompressionSettings, is_enabled),
            },
            FieldInfoData {
                name: "ShareBaselinesAcrossConnections",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DeltaCompressionSettings, share_baselines_across_connections),
            },
            FieldInfoData {
                name: "BaselineReuseCount",
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
}


pub static DELTACOMPRESSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaCompressionSettings-Array",
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
}


pub static NETOBJECTDEPENDENCYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectDependencyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectDependencyType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn max_frequency_factor(&self) -> &f32;
    fn min_frequency_factor_radius(&self) -> &f32;
    fn max_frequency_factor_radius(&self) -> &f32;
    fn max_frequency_factor_cone_radius(&self) -> &f32;
    fn min_cone_frequency_factor(&self) -> &f32;
    fn camera_fov_bias_degrees(&self) -> &f32;
    fn max_camera_fov_degrees(&self) -> &f32;
    fn min_camera_fov_degrees(&self) -> &f32;
}

impl NetObjectPrioritySettingsTrait for NetObjectPrioritySettings {
    fn min_frequency_factor(&self) -> &f32 {
        &self.min_frequency_factor
    }
    fn max_frequency_factor(&self) -> &f32 {
        &self.max_frequency_factor
    }
    fn min_frequency_factor_radius(&self) -> &f32 {
        &self.min_frequency_factor_radius
    }
    fn max_frequency_factor_radius(&self) -> &f32 {
        &self.max_frequency_factor_radius
    }
    fn max_frequency_factor_cone_radius(&self) -> &f32 {
        &self.max_frequency_factor_cone_radius
    }
    fn min_cone_frequency_factor(&self) -> &f32 {
        &self.min_cone_frequency_factor
    }
    fn camera_fov_bias_degrees(&self) -> &f32 {
        &self.camera_fov_bias_degrees
    }
    fn max_camera_fov_degrees(&self) -> &f32 {
        &self.max_camera_fov_degrees
    }
    fn min_camera_fov_degrees(&self) -> &f32 {
        &self.min_camera_fov_degrees
    }
}

pub static NETOBJECTPRIORITYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectPrioritySettings",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetObjectPrioritySettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MinFrequencyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, min_frequency_factor),
            },
            FieldInfoData {
                name: "MaxFrequencyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor),
            },
            FieldInfoData {
                name: "MinFrequencyFactorRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, min_frequency_factor_radius),
            },
            FieldInfoData {
                name: "MaxFrequencyFactorRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor_radius),
            },
            FieldInfoData {
                name: "MaxFrequencyFactorConeRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor_cone_radius),
            },
            FieldInfoData {
                name: "MinConeFrequencyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, min_cone_frequency_factor),
            },
            FieldInfoData {
                name: "CameraFovBiasDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, camera_fov_bias_degrees),
            },
            FieldInfoData {
                name: "MaxCameraFovDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NetObjectPrioritySettings, max_camera_fov_degrees),
            },
            FieldInfoData {
                name: "MinCameraFovDegrees",
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
}


pub static NETOBJECTPRIORITYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectPrioritySettings-Array",
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
}


pub static NETOBJECTSENDSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSendStatus-Array",
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
}


pub static NETWORKCHANNELID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkChannelId-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkChannelId"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SyncedTransformEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub interpolate: bool,
}

pub trait SyncedTransformEntityDataTrait: super::entity::EntityDataTrait {
    fn interpolate(&self) -> &bool;
}

impl SyncedTransformEntityDataTrait for SyncedTransformEntityData {
    fn interpolate(&self) -> &bool {
        &self.interpolate
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
}

impl super::core::GameDataContainerTrait for SyncedTransformEntityData {
}

impl super::core::DataContainerTrait for SyncedTransformEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SYNCEDTRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedTransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedTransformEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Interpolate",
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
}


pub static SYNCEDTRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedTransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedTransformEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for SyncedIntEntityData {
}

impl super::core::DataContainerTrait for SyncedIntEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SYNCEDINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedIntEntityData as Default>::default())),
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
}


pub static SYNCEDINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedIntEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for SyncedFloatEntityData {
}

impl super::core::DataContainerTrait for SyncedFloatEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SYNCEDFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedFloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedFloatEntityData as Default>::default())),
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
}


pub static SYNCEDFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedFloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedFloatEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for SyncedBoolEntityData {
}

impl super::core::DataContainerTrait for SyncedBoolEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SYNCEDBOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedBoolEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedBoolEntityData as Default>::default())),
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
}


pub static SYNCEDBOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedBoolEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedBoolEntityData"),
    array_type: None,
    alignment: 8,
};


