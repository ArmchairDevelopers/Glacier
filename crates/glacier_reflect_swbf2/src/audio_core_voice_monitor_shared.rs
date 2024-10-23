use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    fn node(&self) -> &Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
        self._glacier_base.node()
    }
    fn node_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::AudioGraphNodeData */> {
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
    name_hash: 1825587763,
    flags: MemberInfoFlags::new(101),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODECONFIGDATA_TYPE_INFO),
        super_class_offset: offset_of!(MonitorNodeConfigData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitorNodeConfigData as Default>::default())),
            create_boxed: || Box::new(<MonitorNodeConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MonitoringEnabled",
                name_hash: 3739715404,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MonitorNodeConfigData, monitoring_enabled),
            },
            FieldInfoData {
                name: "MonitoringPriority",
                name_hash: 917826203,
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
    name_hash: 39472519,
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitorNodeConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MonitorNodeData {
    pub _glacier_base: super::audio::AudioGraphNodeData,
    pub monitoring_enabled: super::audio::AudioGraphNodePort,
    pub monitoring_priority: super::audio::AudioGraphNodePort,
    pub port: u16,
    pub node_meta_data: Vec<BoxedTypeObject /* MonitoredNodeMetaData */>,
    pub asset: Option<LockedTypeObject /* super::core::Asset */>,
    pub partition_guid: glacier_util::guid::Guid,
}

pub trait MonitorNodeDataTrait: super::audio::AudioGraphNodeDataTrait {
    fn monitoring_enabled(&self) -> &super::audio::AudioGraphNodePort;
    fn monitoring_enabled_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn monitoring_priority(&self) -> &super::audio::AudioGraphNodePort;
    fn monitoring_priority_mut(&mut self) -> &mut super::audio::AudioGraphNodePort;
    fn port(&self) -> &u16;
    fn port_mut(&mut self) -> &mut u16;
    fn node_meta_data(&self) -> &Vec<BoxedTypeObject /* MonitoredNodeMetaData */>;
    fn node_meta_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodeMetaData */>;
    fn asset(&self) -> &Option<LockedTypeObject /* super::core::Asset */>;
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::Asset */>;
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
    fn node_meta_data(&self) -> &Vec<BoxedTypeObject /* MonitoredNodeMetaData */> {
        &self.node_meta_data
    }
    fn node_meta_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodeMetaData */> {
        &mut self.node_meta_data
    }
    fn asset(&self) -> &Option<LockedTypeObject /* super::core::Asset */> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::Asset */> {
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
    name_hash: 1350496793,
    flags: MemberInfoFlags::new(101),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::audio::AUDIOGRAPHNODEDATA_TYPE_INFO),
        super_class_offset: offset_of!(MonitorNodeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitorNodeData as Default>::default())),
            create_boxed: || Box::new(<MonitorNodeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MonitoringEnabled",
                name_hash: 3739715404,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(MonitorNodeData, monitoring_enabled),
            },
            FieldInfoData {
                name: "MonitoringPriority",
                name_hash: 917826203,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioGraphNodePort",
                rust_offset: offset_of!(MonitorNodeData, monitoring_priority),
            },
            FieldInfoData {
                name: "Port",
                name_hash: 2089459004,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(MonitorNodeData, port),
            },
            FieldInfoData {
                name: "NodeMetaData",
                name_hash: 2812589512,
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodeMetaData-Array",
                rust_offset: offset_of!(MonitorNodeData, node_meta_data),
            },
            FieldInfoData {
                name: "Asset",
                name_hash: 205976053,
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(MonitorNodeData, asset),
            },
            FieldInfoData {
                name: "PartitionGuid",
                name_hash: 2880687832,
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
    name_hash: 4142403501,
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitorNodeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MonitoredNodeMetaData {
    pub node_name: String,
    pub node_guid: glacier_util::guid::Guid,
    pub event_ports: Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>,
    pub param_ports: Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>,
    pub asset_ports: Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>,
    pub selection_ports: Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>,
}

pub trait MonitoredNodeMetaDataTrait: TypeObject {
    fn node_name(&self) -> &String;
    fn node_name_mut(&mut self) -> &mut String;
    fn node_guid(&self) -> &glacier_util::guid::Guid;
    fn node_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn event_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
    fn event_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
    fn param_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
    fn param_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
    fn asset_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
    fn asset_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
    fn selection_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
    fn selection_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */>;
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
    fn event_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &self.event_ports
    }
    fn event_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &mut self.event_ports
    }
    fn param_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &self.param_ports
    }
    fn param_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &mut self.param_ports
    }
    fn asset_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &self.asset_ports
    }
    fn asset_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &mut self.asset_ports
    }
    fn selection_ports(&self) -> &Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &self.selection_ports
    }
    fn selection_ports_mut(&mut self) -> &mut Vec<BoxedTypeObject /* MonitoredNodePortMetaData */> {
        &mut self.selection_ports
    }
}

pub static MONITOREDNODEMETADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MonitoredNodeMetaData",
    name_hash: 375471205,
    flags: MemberInfoFlags::new(73),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitoredNodeMetaData as Default>::default())),
            create_boxed: || Box::new(<MonitoredNodeMetaData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "NodeName",
                name_hash: 2598021986,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MonitoredNodeMetaData, node_name),
            },
            FieldInfoData {
                name: "NodeGuid",
                name_hash: 2597796698,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MonitoredNodeMetaData, node_guid),
            },
            FieldInfoData {
                name: "EventPorts",
                name_hash: 3246019619,
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodePortMetaData-Array",
                rust_offset: offset_of!(MonitoredNodeMetaData, event_ports),
            },
            FieldInfoData {
                name: "ParamPorts",
                name_hash: 3375735424,
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodePortMetaData-Array",
                rust_offset: offset_of!(MonitoredNodeMetaData, param_ports),
            },
            FieldInfoData {
                name: "AssetPorts",
                name_hash: 301456063,
                flags: MemberInfoFlags::new(144),
                field_type: "MonitoredNodePortMetaData-Array",
                rust_offset: offset_of!(MonitoredNodeMetaData, asset_ports),
            },
            FieldInfoData {
                name: "SelectionPorts",
                name_hash: 2619978063,
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
    name_hash: 4136805713,
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("MonitoredNodeMetaData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3297819164,
    flags: MemberInfoFlags::new(36937),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MonitoredNodePortMetaData as Default>::default())),
            create_boxed: || Box::new(<MonitoredNodePortMetaData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FieldIndex",
                name_hash: 2151738201,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, field_index),
            },
            FieldInfoData {
                name: "FieldIndex_NonMeta",
                name_hash: 2279489748,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, field_index_non_meta),
            },
            FieldInfoData {
                name: "ElementIndex",
                name_hash: 2692677381,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, element_index),
            },
            FieldInfoData {
                name: "SubFieldIndex",
                name_hash: 271160733,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MonitoredNodePortMetaData, sub_field_index),
            },
            FieldInfoData {
                name: "SubFieldIndex_NonMeta",
                name_hash: 1915865872,
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
    name_hash: 1487188520,
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
    name_hash: 3345644681,
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
    name_hash: 1147056701,
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
    name_hash: 4170887467,
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
    name_hash: 2968031391,
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
    name_hash: 1445077374,
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
    name_hash: 2742952522,
    flags: MemberInfoFlags::new(145),
    module: "AudioCoreVoiceMonitorShared",
    data: TypeInfoData::Array("DebugRenderingSelection"),
    array_type: None,
    alignment: 8,
};


