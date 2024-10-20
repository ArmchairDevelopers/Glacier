use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSyncedTransformEntity {
}

pub const CLIENTSYNCEDTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedTransformEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSYNCEDTRANSFORMENTITY_TYPE_INFO
    }
}


pub const CLIENTSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedTransformEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSyncedTransformEntity {
}

pub const SERVERSYNCEDTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedTransformEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSYNCEDTRANSFORMENTITY_TYPE_INFO
    }
}


pub const SERVERSYNCEDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedTransformEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSyncedIntEntity {
}

pub const CLIENTSYNCEDINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedIntEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSYNCEDINTENTITY_TYPE_INFO
    }
}


pub const CLIENTSYNCEDINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSyncedIntEntity {
}

pub const SERVERSYNCEDINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedIntEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSYNCEDINTENTITY_TYPE_INFO
    }
}


pub const SERVERSYNCEDINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSyncedFloatEntity {
}

pub const CLIENTSYNCEDFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedFloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedFloatEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSYNCEDFLOATENTITY_TYPE_INFO
    }
}


pub const CLIENTSYNCEDFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedFloatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSyncedFloatEntity {
}

pub const SERVERSYNCEDFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedFloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedFloatEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSYNCEDFLOATENTITY_TYPE_INFO
    }
}


pub const SERVERSYNCEDFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedFloatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientSyncedBoolEntity {
}

pub const CLIENTSYNCEDBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedBoolEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTSYNCEDBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientSyncedBoolEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTSYNCEDBOOLENTITY_TYPE_INFO
    }
}


pub const CLIENTSYNCEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientSyncedBoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ClientSyncedBoolEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerSyncedBoolEntity {
}

pub const SERVERSYNCEDBOOLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedBoolEntity",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERSYNCEDBOOLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerSyncedBoolEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERSYNCEDBOOLENTITY_TYPE_INFO
    }
}


pub const SERVERSYNCEDBOOLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerSyncedBoolEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("ServerSyncedBoolEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EngineConnectionPeer {
}

pub const ENGINECONNECTIONPEER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnectionPeer",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENGINECONNECTION_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENGINECONNECTIONPEER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EngineConnectionPeer {
    fn type_info() -> &'static TypeInfo {
        ENGINECONNECTIONPEER_TYPE_INFO
    }
}


pub const ENGINECONNECTIONPEER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnectionPeer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("EngineConnectionPeer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EngineConnection {
}

pub const ENGINECONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnection",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ENGINECONNECTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EngineConnection {
    fn type_info() -> &'static TypeInfo {
        ENGINECONNECTION_TYPE_INFO
    }
}


pub const ENGINECONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EngineConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("EngineConnection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpikeInternalMessagePartMessage {
}

pub const SPIKEINTERNALMESSAGEPARTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpikeInternalMessagePartMessage",
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SpikeInternalMessagePartMessage {
    fn type_info() -> &'static TypeInfo {
        SPIKEINTERNALMESSAGEPARTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpikeInternalMessageWrapperMessage {
}

pub const SPIKEINTERNALMESSAGEWRAPPERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpikeInternalMessageWrapperMessage",
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SpikeInternalMessageWrapperMessage {
    fn type_info() -> &'static TypeInfo {
        SPIKEINTERNALMESSAGEWRAPPERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NetworkPerfOverlaySettings {
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

pub const NETWORKPERFOVERLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerfOverlaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, enable),
            },
            FieldInfoData {
                name: "GraphPos",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, graph_pos),
            },
            FieldInfoData {
                name: "UpdateFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, update_frequency),
            },
            FieldInfoData {
                name: "HighLatency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_latency),
            },
            FieldInfoData {
                name: "CriticalLatency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_latency),
            },
            FieldInfoData {
                name: "HighLatencyVariation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_latency_variation),
            },
            FieldInfoData {
                name: "CriticalLatencyVariation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_latency_variation),
            },
            FieldInfoData {
                name: "HighPacketLossRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, high_packet_loss_ratio),
            },
            FieldInfoData {
                name: "CriticalPacketLossRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, critical_packet_loss_ratio),
            },
            FieldInfoData {
                name: "ServerFpsLowThresholdPerc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, server_fps_low_threshold_perc),
            },
            FieldInfoData {
                name: "ServerFpsLowThresholdCritPerc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkPerfOverlaySettings, server_fps_low_threshold_crit_perc),
            },
        ],
    }),
    array_type: Some(NETWORKPERFOVERLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkPerfOverlaySettings {
    fn type_info() -> &'static TypeInfo {
        NETWORKPERFOVERLAYSETTINGS_TYPE_INFO
    }
}


pub const NETWORKPERFOVERLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerfOverlaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkPerfOverlaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InterpolationManagerSettings {
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

pub const INTERPOLATIONMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationManagerSettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeNudgeCalculator",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_calculator),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstKPos",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_k_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstKNeg",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_k_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTIPos",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_i_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTINeg",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_i_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTDPos",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_d_pos),
            },
            FieldInfoData {
                name: "TimeNudgePIDConstTDNeg",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_const_t_d_neg),
            },
            FieldInfoData {
                name: "TimeNudgePIDLatencyTol",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_latency_tol),
            },
            FieldInfoData {
                name: "TimeNudgePIDPacketDeltaTimeTol",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_packet_delta_time_tol),
            },
            FieldInfoData {
                name: "TimeNudgePIDMaxChangePerSec",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, time_nudge_p_i_d_max_change_per_sec),
            },
            FieldInfoData {
                name: "AveragePacketSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(InterpolationManagerSettings, average_packet_sample_count),
            },
        ],
    }),
    array_type: Some(INTERPOLATIONMANAGERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InterpolationManagerSettings {
    fn type_info() -> &'static TypeInfo {
        INTERPOLATIONMANAGERSETTINGS_TYPE_INFO
    }
}


pub const INTERPOLATIONMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InterpolationManagerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("InterpolationManagerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const INTERNETSIMULATIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternetSimulationState",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, enabled),
            },
            FieldInfoData {
                name: "ReorderRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, reorder_ratio),
            },
            FieldInfoData {
                name: "LatencyMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, latency_min),
            },
            FieldInfoData {
                name: "LatencyMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, latency_max),
            },
            FieldInfoData {
                name: "DuplicateRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, duplicate_ratio),
            },
            FieldInfoData {
                name: "DropRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, drop_ratio),
            },
            FieldInfoData {
                name: "CorruptRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, corrupt_ratio),
            },
            FieldInfoData {
                name: "SizeRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, size_ratio),
            },
            FieldInfoData {
                name: "SpikeDurationMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, spike_duration_min),
            },
            FieldInfoData {
                name: "SpikeDurationMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, spike_duration_max),
            },
            FieldInfoData {
                name: "SpikeCooldownMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, spike_cooldown_min),
            },
            FieldInfoData {
                name: "SpikeCooldownMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, spike_cooldown_max),
            },
            FieldInfoData {
                name: "BandwidthMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, bandwidth_max),
            },
            FieldInfoData {
                name: "BandwidthDelayMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InternetSimulationState, bandwidth_delay_max),
            },
        ],
    }),
    array_type: Some(INTERNETSIMULATIONSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InternetSimulationState {
    fn type_info() -> &'static TypeInfo {
        INTERNETSIMULATIONSTATE_TYPE_INFO
    }
}


pub const INTERNETSIMULATIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InternetSimulationState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("InternetSimulationState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkCoreSettings {
    pub dedicated_server_max_send_job_count: u32,
    pub server_max_send_job_count: u32,
}

pub const NETWORKCORESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCoreSettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DedicatedServerMaxSendJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkCoreSettings, dedicated_server_max_send_job_count),
            },
            FieldInfoData {
                name: "ServerMaxSendJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetworkCoreSettings, server_max_send_job_count),
            },
        ],
    }),
    array_type: Some(NETWORKCORESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkCoreSettings {
    fn type_info() -> &'static TypeInfo {
        NETWORKCORESETTINGS_TYPE_INFO
    }
}


pub const NETWORKCORESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkCoreSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkCoreSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreDemoStatusMessage {
}

pub const COREDEMOSTATUSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreDemoStatusMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreDemoStatusMessage {
    fn type_info() -> &'static TypeInfo {
        COREDEMOSTATUSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NetObjectSystemSettings {
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

pub const NETOBJECTSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxNetObjectCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, max_net_object_count),
            },
            FieldInfoData {
                name: "MaxStaticNetObjectCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, max_static_net_object_count),
            },
            FieldInfoData {
                name: "MaxClientConnectionCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, max_client_connection_count),
            },
            FieldInfoData {
                name: "MaxServerConnectionCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, max_server_connection_count),
            },
            FieldInfoData {
                name: "InProcReplicationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, in_proc_replication_enabled),
            },
            FieldInfoData {
                name: "InProcBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, in_proc_buffer_size),
            },
            FieldInfoData {
                name: "GameViewInProcBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, game_view_in_proc_buffer_size),
            },
            FieldInfoData {
                name: "MaxRemoteAuthorityNetObjectCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, max_remote_authority_net_object_count),
            },
            FieldInfoData {
                name: "DefaultDynamicPriorityMethod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, default_dynamic_priority_method),
            },
            FieldInfoData {
                name: "PrioritySettings",
                flags: MemberInfoFlags::new(0),
                field_type: NETOBJECTPRIORITYSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, priority_settings),
            },
            FieldInfoData {
                name: "DefaultFilterMethod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, default_filter_method),
            },
            FieldInfoData {
                name: "DeltaCompressionSettings",
                flags: MemberInfoFlags::new(0),
                field_type: DELTACOMPRESSIONSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, delta_compression_settings),
            },
            FieldInfoData {
                name: "Debug",
                flags: MemberInfoFlags::new(0),
                field_type: NETOBJECTSYSTEMDEBUGSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemSettings, debug),
            },
        ],
    }),
    array_type: Some(NETOBJECTSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetObjectSystemSettings {
    fn type_info() -> &'static TypeInfo {
        NETOBJECTSYSTEMSETTINGS_TYPE_INFO
    }
}


pub const NETOBJECTSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const NETOBJECTSYSTEMDEBUGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemDebugSettings",
    flags: MemberInfoFlags::new(73),
    module: "Network",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EnableReplicationWarnings",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, enable_replication_warnings),
            },
            FieldInfoData {
                name: "EnableIncomingReplicationStatusReport",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, enable_incoming_replication_status_report),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportMaxDelta",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_max_delta),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportFilter",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_filter),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeSpatial",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_spatial),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeStatic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_static),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportIncludeNonSpatial",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_include_non_spatial),
            },
            FieldInfoData {
                name: "IncomingReplicationStatusReportDrawName",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, incoming_replication_status_report_draw_name),
            },
            FieldInfoData {
                name: "OutputObjectProtocols",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, output_object_protocols),
            },
            FieldInfoData {
                name: "InitialGraceTimeInFrames",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, initial_grace_time_in_frames),
            },
            FieldInfoData {
                name: "ReportReplicationWarningsAfterFrames",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, report_replication_warnings_after_frames),
            },
            FieldInfoData {
                name: "WarnOnMissingInitDependency",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_missing_init_dependency),
            },
            FieldInfoData {
                name: "WarnOnTooLargeNetObject",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_too_large_net_object),
            },
            FieldInfoData {
                name: "WarnOnNoStateCanBeSent",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_no_state_can_be_sent),
            },
            FieldInfoData {
                name: "WarnOnWaitingForCreationAck",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NetObjectSystemDebugSettings, warn_on_waiting_for_creation_ack),
            },
        ],
    }),
    array_type: Some(NETOBJECTSYSTEMDEBUGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetObjectSystemDebugSettings {
    fn type_info() -> &'static TypeInfo {
        NETOBJECTSYSTEMDEBUGSETTINGS_TYPE_INFO
    }
}


pub const NETOBJECTSYSTEMDEBUGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSystemDebugSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSystemDebugSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeltaCompressionSettings {
    pub is_enabled: bool,
    pub share_baselines_across_connections: bool,
    pub baseline_reuse_count: u32,
}

pub const DELTACOMPRESSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaCompressionSettings",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DeltaCompressionSettings, is_enabled),
            },
            FieldInfoData {
                name: "ShareBaselinesAcrossConnections",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DeltaCompressionSettings, share_baselines_across_connections),
            },
            FieldInfoData {
                name: "BaselineReuseCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DeltaCompressionSettings, baseline_reuse_count),
            },
        ],
    }),
    array_type: Some(DELTACOMPRESSIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for DeltaCompressionSettings {
    fn type_info() -> &'static TypeInfo {
        DELTACOMPRESSIONSETTINGS_TYPE_INFO
    }
}


pub const DELTACOMPRESSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaCompressionSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("DeltaCompressionSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const NETOBJECTDEPENDENCYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectDependencyType",
    flags: MemberInfoFlags::new(49429),
    module: "Network",
    data: TypeInfoData::Enum,
    array_type: Some(NETOBJECTDEPENDENCYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NetObjectDependencyType {
    fn type_info() -> &'static TypeInfo {
        NETOBJECTDEPENDENCYTYPE_TYPE_INFO
    }
}


pub const NETOBJECTDEPENDENCYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectDependencyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectDependencyType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const NETOBJECTPRIORITYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectPrioritySettings",
    flags: MemberInfoFlags::new(36937),
    module: "Network",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MinFrequencyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, min_frequency_factor),
            },
            FieldInfoData {
                name: "MaxFrequencyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor),
            },
            FieldInfoData {
                name: "MinFrequencyFactorRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, min_frequency_factor_radius),
            },
            FieldInfoData {
                name: "MaxFrequencyFactorRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor_radius),
            },
            FieldInfoData {
                name: "MaxFrequencyFactorConeRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, max_frequency_factor_cone_radius),
            },
            FieldInfoData {
                name: "MinConeFrequencyFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, min_cone_frequency_factor),
            },
            FieldInfoData {
                name: "CameraFovBiasDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, camera_fov_bias_degrees),
            },
            FieldInfoData {
                name: "MaxCameraFovDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, max_camera_fov_degrees),
            },
            FieldInfoData {
                name: "MinCameraFovDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NetObjectPrioritySettings, min_camera_fov_degrees),
            },
        ],
    }),
    array_type: Some(NETOBJECTPRIORITYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for NetObjectPrioritySettings {
    fn type_info() -> &'static TypeInfo {
        NETOBJECTPRIORITYSETTINGS_TYPE_INFO
    }
}


pub const NETOBJECTPRIORITYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectPrioritySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectPrioritySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum NetObjectSendStatus {
    #[default]
    NetObjectSendStatus_Pause = 0,
    NetObjectSendStatus_Send = 1,
    NetObjectSendStatus_DeleteAll = 2,
}

pub const NETOBJECTSENDSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSendStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Network",
    data: TypeInfoData::Enum,
    array_type: Some(NETOBJECTSENDSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NetObjectSendStatus {
    fn type_info() -> &'static TypeInfo {
        NETOBJECTSENDSTATUS_TYPE_INFO
    }
}


pub const NETOBJECTSENDSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetObjectSendStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetObjectSendStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum NetworkChannelId {
    #[default]
    NetworkChannelId_Invalid = 0,
    NetworkChannelId_ValidOffset = 1,
}

pub const NETWORKCHANNELID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkChannelId",
    flags: MemberInfoFlags::new(49429),
    module: "Network",
    data: TypeInfoData::Enum,
    array_type: Some(NETWORKCHANNELID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NetworkChannelId {
    fn type_info() -> &'static TypeInfo {
        NETWORKCHANNELID_TYPE_INFO
    }
}


pub const NETWORKCHANNELID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkChannelId-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("NetworkChannelId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SyncedTransformEntityData {
    pub interpolate: bool,
}

pub const SYNCEDTRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedTransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Interpolate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SyncedTransformEntityData, interpolate),
            },
        ],
    }),
    array_type: Some(SYNCEDTRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedTransformEntityData {
    fn type_info() -> &'static TypeInfo {
        SYNCEDTRANSFORMENTITYDATA_TYPE_INFO
    }
}


pub const SYNCEDTRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedTransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedTransformEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SyncedIntEntityData {
}

pub const SYNCEDINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SYNCEDINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedIntEntityData {
    fn type_info() -> &'static TypeInfo {
        SYNCEDINTENTITYDATA_TYPE_INFO
    }
}


pub const SYNCEDINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedIntEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SyncedFloatEntityData {
}

pub const SYNCEDFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedFloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SYNCEDFLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedFloatEntityData {
    fn type_info() -> &'static TypeInfo {
        SYNCEDFLOATENTITYDATA_TYPE_INFO
    }
}


pub const SYNCEDFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedFloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedFloatEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SyncedBoolEntityData {
}

pub const SYNCEDBOOLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedBoolEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Network",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SYNCEDBOOLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SyncedBoolEntityData {
    fn type_info() -> &'static TypeInfo {
        SYNCEDBOOLENTITYDATA_TYPE_INFO
    }
}


pub const SYNCEDBOOLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedBoolEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Network",
    data: TypeInfoData::Array("SyncedBoolEntityData-Array"),
    array_type: None,
    alignment: 8,
};


