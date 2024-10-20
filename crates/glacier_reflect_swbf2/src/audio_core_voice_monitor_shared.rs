use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_audio_core_voice_monitor_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(MONITORNODECONFIGDATA_TYPE_INFO);
    registry.register_type(MONITORNODECONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(MONITORNODEDATA_TYPE_INFO);
    registry.register_type(MONITORNODEDATA_ARRAY_TYPE_INFO);
    registry.register_type(MONITOREDNODEMETADATA_TYPE_INFO);
    registry.register_type(MONITOREDNODEMETADATA_ARRAY_TYPE_INFO);
    registry.register_type(MONITOREDNODEPORTMETADATA_TYPE_INFO);
    registry.register_type(MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO);
    registry.register_type(MONITOREDNODEPORTTYPE_TYPE_INFO);
    registry.register_type(MONITOREDNODEPORTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MONITORINGSORTTYPE_TYPE_INFO);
    registry.register_type(MONITORINGSORTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DEBUGRENDERINGSELECTION_TYPE_INFO);
    registry.register_type(DEBUGRENDERINGSELECTION_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct MonitorNodeConfigData {
    pub monitoring_enabled: f32,
    pub monitoring_priority: f32,
}

pub const MONITORNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeConfigData",
    flags: MemberInfoFlags::new(101),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MonitoringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeConfigData, monitoring_enabled),
            },
            FieldInfoData {
                name: "MonitoringPriority",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeConfigData, monitoring_priority),
            },
        ],
    }),
    array_type: Some(MONITORNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MonitorNodeConfigData {
    fn type_info() -> &'static TypeInfo {
        MONITORNODECONFIGDATA_TYPE_INFO
    }
}


pub const MONITORNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitorNodeConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MonitorNodeData {
    pub monitoring_enabled: super::audio::AudioGraphNodePort,
    pub monitoring_priority: super::audio::AudioGraphNodePort,
    pub port: u16,
    pub node_meta_data: Vec<MonitoredNodeMetaData>,
    pub asset: super::core::Asset,
    pub partition_guid: super::core::Guid,
}

pub const MONITORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeData",
    flags: MemberInfoFlags::new(101),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUDIOGRAPHNODEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MonitoringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeData, monitoring_enabled),
            },
            FieldInfoData {
                name: "MonitoringPriority",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOGRAPHNODEPORT_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeData, monitoring_priority),
            },
            FieldInfoData {
                name: "Port",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeData, port),
            },
            FieldInfoData {
                name: "NodeMetaData",
                flags: MemberInfoFlags::new(144),
                field_type: MONITOREDNODEMETADATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeData, node_meta_data),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: ASSET_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeData, asset),
            },
            FieldInfoData {
                name: "PartitionGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MonitorNodeData, partition_guid),
            },
        ],
    }),
    array_type: Some(MONITORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MonitorNodeData {
    fn type_info() -> &'static TypeInfo {
        MONITORNODEDATA_TYPE_INFO
    }
}


pub const MONITORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitorNodeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MonitoredNodeMetaData {
    pub node_name: String,
    pub node_guid: super::core::Guid,
    pub event_ports: Vec<MonitoredNodePortMetaData>,
    pub param_ports: Vec<MonitoredNodePortMetaData>,
    pub asset_ports: Vec<MonitoredNodePortMetaData>,
    pub selection_ports: Vec<MonitoredNodePortMetaData>,
}

pub const MONITOREDNODEMETADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodeMetaData",
    flags: MemberInfoFlags::new(73),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NodeName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodeMetaData, node_name),
            },
            FieldInfoData {
                name: "NodeGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodeMetaData, node_guid),
            },
            FieldInfoData {
                name: "EventPorts",
                flags: MemberInfoFlags::new(144),
                field_type: MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodeMetaData, event_ports),
            },
            FieldInfoData {
                name: "ParamPorts",
                flags: MemberInfoFlags::new(144),
                field_type: MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodeMetaData, param_ports),
            },
            FieldInfoData {
                name: "AssetPorts",
                flags: MemberInfoFlags::new(144),
                field_type: MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodeMetaData, asset_ports),
            },
            FieldInfoData {
                name: "SelectionPorts",
                flags: MemberInfoFlags::new(144),
                field_type: MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodeMetaData, selection_ports),
            },
        ],
    }),
    array_type: Some(MONITOREDNODEMETADATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MonitoredNodeMetaData {
    fn type_info() -> &'static TypeInfo {
        MONITOREDNODEMETADATA_TYPE_INFO
    }
}


pub const MONITOREDNODEMETADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodeMetaData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoredNodeMetaData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MonitoredNodePortMetaData {
    pub field_index: u8,
    pub field_index_non_meta: u8,
    pub element_index: u8,
    pub sub_field_index: u8,
    pub sub_field_index_non_meta: u8,
}

pub const MONITOREDNODEPORTMETADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortMetaData",
    flags: MemberInfoFlags::new(36937),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "FieldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodePortMetaData, field_index),
            },
            FieldInfoData {
                name: "FieldIndex_NonMeta",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodePortMetaData, field_index_non_meta),
            },
            FieldInfoData {
                name: "ElementIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodePortMetaData, element_index),
            },
            FieldInfoData {
                name: "SubFieldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodePortMetaData, sub_field_index),
            },
            FieldInfoData {
                name: "SubFieldIndex_NonMeta",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MonitoredNodePortMetaData, sub_field_index_non_meta),
            },
        ],
    }),
    array_type: Some(MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MonitoredNodePortMetaData {
    fn type_info() -> &'static TypeInfo {
        MONITOREDNODEPORTMETADATA_TYPE_INFO
    }
}


pub const MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortMetaData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoredNodePortMetaData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MonitoredNodePortType {
    #[default]
    MonitoredNodePortType_Param = 0,
    MonitoredNodePortType_Asset = 1,
    MonitoredNodePortType_Selection = 2,
}

pub const MONITOREDNODEPORTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortType",
    flags: MemberInfoFlags::new(49429),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Enum,
    array_type: Some(MONITOREDNODEPORTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MonitoredNodePortType {
    fn type_info() -> &'static TypeInfo {
        MONITOREDNODEPORTTYPE_TYPE_INFO
    }
}


pub const MONITOREDNODEPORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortType-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoredNodePortType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MonitoringSortType {
    #[default]
    MonitoringSortType_Distance = 0,
    MonitoringSortType_RunningTime = 1,
    MonitoringSortType_PotentialLoudness = 2,
    MonitoringSortType_PerceivedLoudness = 3,
}

pub const MONITORINGSORTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoringSortType",
    flags: MemberInfoFlags::new(49429),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Enum,
    array_type: Some(MONITORINGSORTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MonitoringSortType {
    fn type_info() -> &'static TypeInfo {
        MONITORINGSORTTYPE_TYPE_INFO
    }
}


pub const MONITORINGSORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoringSortType-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoringSortType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DebugRenderingSelection {
    #[default]
    DebugRenderingSelection_PlayingInstances = 0,
    DebugRenderingSelection_MonitoredInstance = 1,
    DebugRenderingSelection_Nothing = 2,
}

pub const DEBUGRENDERINGSELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderingSelection",
    flags: MemberInfoFlags::new(49429),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Enum,
    array_type: Some(DEBUGRENDERINGSELECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DebugRenderingSelection {
    fn type_info() -> &'static TypeInfo {
        DEBUGRENDERINGSELECTION_TYPE_INFO
    }
}


pub const DEBUGRENDERINGSELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderingSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("DebugRenderingSelection-Array"),
    array_type: None,
    alignment: 8,
};


