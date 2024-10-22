use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct MonitorNodeConfigData {
    pub _glacier_base: super::audio::AudioGraphNodeConfigData,
    pub monitoring_enabled: f32,
    pub monitoring_priority: f32,
}

pub trait MonitorNodeConfigDataTrait: super::audio::AudioGraphNodeConfigDataTrait {
    fn monitoring_enabled(&self) -> &f32;
    fn monitoring_enabled_mut(&mut self) -> &mut f32;
    fn monitoring_priority(&self) -> &f32;
    fn monitoring_priority_mut(&mut self) -> &mut f32;
}

impl MonitorNodeConfigDataTrait for MonitorNodeConfigData {
    fn monitoring_enabled(&self) -> &f32 {
        &self.monitoring_enabled
    }
    fn monitoring_enabled_mut(&mut self) -> &mut f32 {
        &mut self.monitoring_enabled
    }
    fn monitoring_priority(&self) -> &f32 {
        &self.monitoring_priority
    }
    fn monitoring_priority_mut(&mut self) -> &mut f32 {
        &mut self.monitoring_priority
    }
}

impl super::audio::AudioGraphNodeConfigDataTrait for MonitorNodeConfigData {
    fn node(&self) -> &Option<Arc<Mutex<dyn super::audio::AudioGraphNodeDataTrait>>> {
        self._glacier_base.node()
    }
    fn node_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::audio::AudioGraphNodeDataTrait>>> {
        self._glacier_base.node_mut()
    }
    fn configured_property_flags(&self) -> &u64 {
        self._glacier_base.configured_property_flags()
    }
    fn configured_property_flags_mut(&mut self) -> &mut u64 {
        self._glacier_base.configured_property_flags_mut()
    }
}

impl super::core::DataContainerTrait for MonitorNodeConfigData {
}

pub static MONITORNODECONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeConfigData",
    flags: MemberInfoFlags::new(101),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitorNodeConfigData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MonitoringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MonitorNodeConfigData, monitoring_enabled),
            },
            FieldInfoData {
                name: "MonitoringPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MonitorNodeConfigData, monitoring_priority),
            },
        ],
    }),
    array_type: Some(MONITORNODECONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MonitorNodeConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        MONITORNODECONFIGDATA_TYPE_INFO
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


pub static MONITORNODECONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitorNodeConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MonitorNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub monitoring_enabled: super::audio::AudioGraphNodePort,
    pub monitoring_priority: super::audio::AudioGraphNodePort,
    pub port: u16,
    pub node_meta_data: Vec<MonitoredNodeMetaData>,
    pub asset: Option<Arc<Mutex<dyn super::core::AssetTrait>>>,
    pub partition_guid: glacier_util::guid::Guid,
}

pub trait MonitorNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn monitoring_enabled(&self) -> &super::audio::AudioGraphNodePort;
    fn monitoring_enabled_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn monitoring_priority(&self) -> &super::audio::AudioGraphNodePort;
    fn monitoring_priority_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn port(&self) -> &u16;
    fn port_mut(&mut self) -> &mut u16;
    fn node_meta_data(&self) -> &Vec<MonitoredNodeMetaData>;
    fn node_meta_data_mut(&mut self) -> &mut Vec<MonitoredNodeMetaData>;
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
    fn asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::AssetTrait>>>;
    fn partition_guid(&self) -> &glacier_util::guid::Guid;
    fn partition_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
}

impl MonitorNodeDataTrait for MonitorNodeData {
    fn monitoring_enabled(&self) -> &super::audio::AudioGraphNodePort {
        &self.monitoring_enabled
    }
    fn monitoring_enabled_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.monitoring_enabled
    }
    fn monitoring_priority(&self) -> &super::audio::AudioGraphNodePort {
        &self.monitoring_priority
    }
    fn monitoring_priority_mut(&mut self) -> &mut super::audio::AudioGraphNodePort {
        &mut self.monitoring_priority
    }
    fn port(&self) -> &u16 {
        &self.port
    }
    fn port_mut(&mut self) -> &mut u16 {
        &mut self.port
    }
    fn node_meta_data(&self) -> &Vec<MonitoredNodeMetaData> {
        &self.node_meta_data
    }
    fn node_meta_data_mut(&mut self) -> &mut Vec<MonitoredNodeMetaData> {
        &mut self.node_meta_data
    }
    fn asset(&self) -> &Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::AssetTrait>>> {
        &mut self.asset
    }
    fn partition_guid(&self) -> &glacier_util::guid::Guid {
        &self.partition_guid
    }
    fn partition_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.partition_guid
    }
}

impl super::audio::AudioGraphNodeDataTrait for MonitorNodeData {
}

impl super::core::DataContainerTrait for MonitorNodeData {
}

pub static MONITORNODEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeData",
    flags: MemberInfoFlags::new(101),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitorNodeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MonitoringEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(MonitorNodeData, monitoring_enabled),
            },
            FieldInfoData {
                name: "MonitoringPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(MonitorNodeData, monitoring_priority),
            },
            FieldInfoData {
                name: "Port",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(MonitorNodeData, port),
            },
            FieldInfoData {
                name: "NodeMetaData",
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodeMetaData-Array",
                rust_offset: offset_of!(MonitorNodeData, node_meta_data),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(MonitorNodeData, asset),
            },
            FieldInfoData {
                name: "PartitionGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MonitorNodeData, partition_guid),
            },
        ],
    }),
    array_type: Some(MONITORNODEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MonitorNodeData {
    fn type_info(&self) -> &'static TypeInfo {
        MONITORNODEDATA_TYPE_INFO
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


pub static MONITORNODEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitorNodeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitorNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MonitoredNodeMetaData {
    pub node_name: String,
    pub node_guid: glacier_util::guid::Guid,
    pub event_ports: Vec<MonitoredNodePortMetaData>,
    pub param_ports: Vec<MonitoredNodePortMetaData>,
    pub asset_ports: Vec<MonitoredNodePortMetaData>,
    pub selection_ports: Vec<MonitoredNodePortMetaData>,
}

pub trait MonitoredNodeMetaDataTrait: TypeObject {
    fn node_name(&self) -> &String;
    fn node_name_mut(&mut self) -> &mut String;
    fn node_guid(&self) -> &glacier_util::guid::Guid;
    fn node_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn event_ports(&self) -> &Vec<MonitoredNodePortMetaData>;
    fn event_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData>;
    fn param_ports(&self) -> &Vec<MonitoredNodePortMetaData>;
    fn param_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData>;
    fn asset_ports(&self) -> &Vec<MonitoredNodePortMetaData>;
    fn asset_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData>;
    fn selection_ports(&self) -> &Vec<MonitoredNodePortMetaData>;
    fn selection_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData>;
}

impl MonitoredNodeMetaDataTrait for MonitoredNodeMetaData {
    fn node_name(&self) -> &String {
        &self.node_name
    }
    fn node_name_mut(&mut self) -> &mut String {
        &mut self.node_name
    }
    fn node_guid(&self) -> &glacier_util::guid::Guid {
        &self.node_guid
    }
    fn node_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.node_guid
    }
    fn event_ports(&self) -> &Vec<MonitoredNodePortMetaData> {
        &self.event_ports
    }
    fn event_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData> {
        &mut self.event_ports
    }
    fn param_ports(&self) -> &Vec<MonitoredNodePortMetaData> {
        &self.param_ports
    }
    fn param_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData> {
        &mut self.param_ports
    }
    fn asset_ports(&self) -> &Vec<MonitoredNodePortMetaData> {
        &self.asset_ports
    }
    fn asset_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData> {
        &mut self.asset_ports
    }
    fn selection_ports(&self) -> &Vec<MonitoredNodePortMetaData> {
        &self.selection_ports
    }
    fn selection_ports_mut(&mut self) -> &mut Vec<MonitoredNodePortMetaData> {
        &mut self.selection_ports
    }
}

pub static MONITOREDNODEMETADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodeMetaData",
    flags: MemberInfoFlags::new(73),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitoredNodeMetaData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "NodeName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MonitoredNodeMetaData, node_name),
            },
            FieldInfoData {
                name: "NodeGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MonitoredNodeMetaData, node_guid),
            },
            FieldInfoData {
                name: "EventPorts",
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodePortMetaData-Array",
                rust_offset: offset_of!(MonitoredNodeMetaData, event_ports),
            },
            FieldInfoData {
                name: "ParamPorts",
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodePortMetaData-Array",
                rust_offset: offset_of!(MonitoredNodeMetaData, param_ports),
            },
            FieldInfoData {
                name: "AssetPorts",
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodePortMetaData-Array",
                rust_offset: offset_of!(MonitoredNodeMetaData, asset_ports),
            },
            FieldInfoData {
                name: "SelectionPorts",
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodePortMetaData-Array",
                rust_offset: offset_of!(MonitoredNodeMetaData, selection_ports),
            },
        ],
    }),
    array_type: Some(MONITOREDNODEMETADATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MonitoredNodeMetaData {
    fn type_info(&self) -> &'static TypeInfo {
        MONITOREDNODEMETADATA_TYPE_INFO
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


pub static MONITOREDNODEMETADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodeMetaData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoredNodeMetaData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MonitoredNodePortMetaData {
    pub field_index: u8,
    pub field_index_non_meta: u8,
    pub element_index: u8,
    pub sub_field_index: u8,
    pub sub_field_index_non_meta: u8,
}

pub trait MonitoredNodePortMetaDataTrait: TypeObject {
    fn field_index(&self) -> &u8;
    fn field_index_mut(&mut self) -> &mut u8;
    fn field_index_non_meta(&self) -> &u8;
    fn field_index_non_meta_mut(&mut self) -> &mut u8;
    fn element_index(&self) -> &u8;
    fn element_index_mut(&mut self) -> &mut u8;
    fn sub_field_index(&self) -> &u8;
    fn sub_field_index_mut(&mut self) -> &mut u8;
    fn sub_field_index_non_meta(&self) -> &u8;
    fn sub_field_index_non_meta_mut(&mut self) -> &mut u8;
}

impl MonitoredNodePortMetaDataTrait for MonitoredNodePortMetaData {
    fn field_index(&self) -> &u8 {
        &self.field_index
    }
    fn field_index_mut(&mut self) -> &mut u8 {
        &mut self.field_index
    }
    fn field_index_non_meta(&self) -> &u8 {
        &self.field_index_non_meta
    }
    fn field_index_non_meta_mut(&mut self) -> &mut u8 {
        &mut self.field_index_non_meta
    }
    fn element_index(&self) -> &u8 {
        &self.element_index
    }
    fn element_index_mut(&mut self) -> &mut u8 {
        &mut self.element_index
    }
    fn sub_field_index(&self) -> &u8 {
        &self.sub_field_index
    }
    fn sub_field_index_mut(&mut self) -> &mut u8 {
        &mut self.sub_field_index
    }
    fn sub_field_index_non_meta(&self) -> &u8 {
        &self.sub_field_index_non_meta
    }
    fn sub_field_index_non_meta_mut(&mut self) -> &mut u8 {
        &mut self.sub_field_index_non_meta
    }
}

pub static MONITOREDNODEPORTMETADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortMetaData",
    flags: MemberInfoFlags::new(36937),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitoredNodePortMetaData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FieldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, field_index),
            },
            FieldInfoData {
                name: "FieldIndex_NonMeta",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, field_index_non_meta),
            },
            FieldInfoData {
                name: "ElementIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, element_index),
            },
            FieldInfoData {
                name: "SubFieldIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, sub_field_index),
            },
            FieldInfoData {
                name: "SubFieldIndex_NonMeta",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, sub_field_index_non_meta),
            },
        ],
    }),
    array_type: Some(MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MonitoredNodePortMetaData {
    fn type_info(&self) -> &'static TypeInfo {
        MONITOREDNODEPORTMETADATA_TYPE_INFO
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


pub static MONITOREDNODEPORTMETADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortMetaData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoredNodePortMetaData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MonitoredNodePortType {
    #[default]
    MonitoredNodePortType_Param = 0,
    MonitoredNodePortType_Asset = 1,
    MonitoredNodePortType_Selection = 2,
}

pub static MONITOREDNODEPORTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortType",
    flags: MemberInfoFlags::new(49429),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Enum,
    array_type: Some(MONITOREDNODEPORTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MonitoredNodePortType {
    fn type_info(&self) -> &'static TypeInfo {
        MONITOREDNODEPORTTYPE_TYPE_INFO
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


pub static MONITOREDNODEPORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodePortType-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoredNodePortType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MonitoringSortType {
    #[default]
    MonitoringSortType_Distance = 0,
    MonitoringSortType_RunningTime = 1,
    MonitoringSortType_PotentialLoudness = 2,
    MonitoringSortType_PerceivedLoudness = 3,
}

pub static MONITORINGSORTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoringSortType",
    flags: MemberInfoFlags::new(49429),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Enum,
    array_type: Some(MONITORINGSORTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MonitoringSortType {
    fn type_info(&self) -> &'static TypeInfo {
        MONITORINGSORTTYPE_TYPE_INFO
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


pub static MONITORINGSORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoringSortType-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoringSortType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DebugRenderingSelection {
    #[default]
    DebugRenderingSelection_PlayingInstances = 0,
    DebugRenderingSelection_MonitoredInstance = 1,
    DebugRenderingSelection_Nothing = 2,
}

pub static DEBUGRENDERINGSELECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderingSelection",
    flags: MemberInfoFlags::new(49429),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Enum,
    array_type: Some(DEBUGRENDERINGSELECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DebugRenderingSelection {
    fn type_info(&self) -> &'static TypeInfo {
        DEBUGRENDERINGSELECTION_TYPE_INFO
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


pub static DEBUGRENDERINGSELECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderingSelection-Array",
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("DebugRenderingSelection"),
    array_type: None,
    alignment: 8,
};


