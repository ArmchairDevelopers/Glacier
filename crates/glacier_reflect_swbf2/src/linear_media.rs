use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_linear_media_types(registry: &mut TypeRegistry) {
    registry.register_type(LINEARMEDIASETTINGS_TYPE_INFO);
    registry.register_type(LINEARMEDIASETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIAASSETDESC_TYPE_INFO);
    registry.register_type(LINEARMEDIAASSETDESC_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIARUNTIMERESOURCE_TYPE_INFO);
    registry.register_type(LINEARMEDIARUNTIMERESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_TYPE_INFO);
    registry.register_type(LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIACHANNELRUNTIME_TYPE_INFO);
    registry.register_type(LINEARMEDIACHANNELRUNTIME_ARRAY_TYPE_INFO);
    registry.register_type(LINEARMEDIAASSET_TYPE_INFO);
    registry.register_type(LINEARMEDIAASSET_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct LinearMediaSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub inline_message_process_count_max: i32,
    pub per_job_message_process_count_max: i32,
    pub inline_message_process_time_max_n_s: i64,
    pub per_job_message_process_time_max_n_s: i64,
    pub turbo_loader_chunk_poll_frequency_n_s: i64,
    pub queue_memory_reap_frequency_n_s: i64,
    pub memory_rebalance_frequency_n_s: i64,
    pub memory_rebalance_time_max_n_s: i64,
    pub memory_rebalance_max_item_count: i32,
    pub message_process_spin_count: i32,
    pub queue_priority: u8,
    pub queue_affinity: u32,
    pub process_priority: u8,
    pub process_affinity: u32,
    pub max_concurrent_dispatch_jobs: u32,
    pub cpu_pool_size: u64,
    pub gpu_pool_size: u64,
    pub track_heaps_s: bool,
}

pub trait LinearMediaSettingsTrait: super::core::SystemSettingsTrait {
    fn inline_message_process_count_max(&self) -> &i32;
    fn per_job_message_process_count_max(&self) -> &i32;
    fn inline_message_process_time_max_n_s(&self) -> &i64;
    fn per_job_message_process_time_max_n_s(&self) -> &i64;
    fn turbo_loader_chunk_poll_frequency_n_s(&self) -> &i64;
    fn queue_memory_reap_frequency_n_s(&self) -> &i64;
    fn memory_rebalance_frequency_n_s(&self) -> &i64;
    fn memory_rebalance_time_max_n_s(&self) -> &i64;
    fn memory_rebalance_max_item_count(&self) -> &i32;
    fn message_process_spin_count(&self) -> &i32;
    fn queue_priority(&self) -> &u8;
    fn queue_affinity(&self) -> &u32;
    fn process_priority(&self) -> &u8;
    fn process_affinity(&self) -> &u32;
    fn max_concurrent_dispatch_jobs(&self) -> &u32;
    fn cpu_pool_size(&self) -> &u64;
    fn gpu_pool_size(&self) -> &u64;
    fn track_heaps_s(&self) -> &bool;
}

impl LinearMediaSettingsTrait for LinearMediaSettings {
    fn inline_message_process_count_max(&self) -> &i32 {
        &self.inline_message_process_count_max
    }
    fn per_job_message_process_count_max(&self) -> &i32 {
        &self.per_job_message_process_count_max
    }
    fn inline_message_process_time_max_n_s(&self) -> &i64 {
        &self.inline_message_process_time_max_n_s
    }
    fn per_job_message_process_time_max_n_s(&self) -> &i64 {
        &self.per_job_message_process_time_max_n_s
    }
    fn turbo_loader_chunk_poll_frequency_n_s(&self) -> &i64 {
        &self.turbo_loader_chunk_poll_frequency_n_s
    }
    fn queue_memory_reap_frequency_n_s(&self) -> &i64 {
        &self.queue_memory_reap_frequency_n_s
    }
    fn memory_rebalance_frequency_n_s(&self) -> &i64 {
        &self.memory_rebalance_frequency_n_s
    }
    fn memory_rebalance_time_max_n_s(&self) -> &i64 {
        &self.memory_rebalance_time_max_n_s
    }
    fn memory_rebalance_max_item_count(&self) -> &i32 {
        &self.memory_rebalance_max_item_count
    }
    fn message_process_spin_count(&self) -> &i32 {
        &self.message_process_spin_count
    }
    fn queue_priority(&self) -> &u8 {
        &self.queue_priority
    }
    fn queue_affinity(&self) -> &u32 {
        &self.queue_affinity
    }
    fn process_priority(&self) -> &u8 {
        &self.process_priority
    }
    fn process_affinity(&self) -> &u32 {
        &self.process_affinity
    }
    fn max_concurrent_dispatch_jobs(&self) -> &u32 {
        &self.max_concurrent_dispatch_jobs
    }
    fn cpu_pool_size(&self) -> &u64 {
        &self.cpu_pool_size
    }
    fn gpu_pool_size(&self) -> &u64 {
        &self.gpu_pool_size
    }
    fn track_heaps_s(&self) -> &bool {
        &self.track_heaps_s
    }
}

impl super::core::SystemSettingsTrait for LinearMediaSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for LinearMediaSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LINEARMEDIASETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaSettings",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InlineMessageProcessCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaSettings, inline_message_process_count_max),
            },
            FieldInfoData {
                name: "PerJobMessageProcessCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaSettings, per_job_message_process_count_max),
            },
            FieldInfoData {
                name: "InlineMessageProcessTimeMaxNS",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(LinearMediaSettings, inline_message_process_time_max_n_s),
            },
            FieldInfoData {
                name: "PerJobMessageProcessTimeMaxNS",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(LinearMediaSettings, per_job_message_process_time_max_n_s),
            },
            FieldInfoData {
                name: "TurboLoaderChunkPollFrequencyNS",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(LinearMediaSettings, turbo_loader_chunk_poll_frequency_n_s),
            },
            FieldInfoData {
                name: "QueueMemoryReapFrequencyNS",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(LinearMediaSettings, queue_memory_reap_frequency_n_s),
            },
            FieldInfoData {
                name: "MemoryRebalanceFrequencyNS",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(LinearMediaSettings, memory_rebalance_frequency_n_s),
            },
            FieldInfoData {
                name: "MemoryRebalanceTimeMaxNS",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(LinearMediaSettings, memory_rebalance_time_max_n_s),
            },
            FieldInfoData {
                name: "MemoryRebalanceMaxItemCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaSettings, memory_rebalance_max_item_count),
            },
            FieldInfoData {
                name: "MessageProcessSpinCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinearMediaSettings, message_process_spin_count),
            },
            FieldInfoData {
                name: "QueuePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LinearMediaSettings, queue_priority),
            },
            FieldInfoData {
                name: "QueueAffinity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LinearMediaSettings, queue_affinity),
            },
            FieldInfoData {
                name: "ProcessPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(LinearMediaSettings, process_priority),
            },
            FieldInfoData {
                name: "ProcessAffinity",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LinearMediaSettings, process_affinity),
            },
            FieldInfoData {
                name: "MaxConcurrentDispatchJobs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LinearMediaSettings, max_concurrent_dispatch_jobs),
            },
            FieldInfoData {
                name: "CpuPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(LinearMediaSettings, cpu_pool_size),
            },
            FieldInfoData {
                name: "GpuPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(LinearMediaSettings, gpu_pool_size),
            },
            FieldInfoData {
                name: "TrackHeapsS",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinearMediaSettings, track_heaps_s),
            },
        ],
    }),
    array_type: Some(LINEARMEDIASETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIASETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARMEDIASETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaAssetDesc {
    pub _glacier_base: super::core::Asset,
    pub resources: Vec<LinearMediaRuntimeResource>,
}

pub trait LinearMediaAssetDescTrait: super::core::AssetTrait {
    fn resources(&self) -> &Vec<LinearMediaRuntimeResource>;
}

impl LinearMediaAssetDescTrait for LinearMediaAssetDesc {
    fn resources(&self) -> &Vec<LinearMediaRuntimeResource> {
        &self.resources
    }
}

impl super::core::AssetTrait for LinearMediaAssetDesc {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for LinearMediaAssetDesc {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LINEARMEDIAASSETDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAssetDesc",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaAssetDesc as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Resources",
                flags: MemberInfoFlags::new(144),
                field_type: "LinearMediaRuntimeResource-Array",
                rust_offset: offset_of!(LinearMediaAssetDesc, resources),
            },
        ],
    }),
    array_type: Some(LINEARMEDIAASSETDESC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaAssetDesc {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIAASSETDESC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARMEDIAASSETDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAssetDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaAssetDesc"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaRuntimeResource {
    pub resource: glacier_reflect::builtin::ResourceRef,
}

pub trait LinearMediaRuntimeResourceTrait: TypeObject {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl LinearMediaRuntimeResourceTrait for LinearMediaRuntimeResource {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
}

pub static LINEARMEDIARUNTIMERESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaRuntimeResource",
    flags: MemberInfoFlags::new(73),
    module: "LinearMedia",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaRuntimeResource as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(LinearMediaRuntimeResource, resource),
            },
        ],
    }),
    array_type: Some(LINEARMEDIARUNTIMERESOURCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaRuntimeResource {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIARUNTIMERESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARMEDIARUNTIMERESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaRuntimeResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaRuntimeResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LinearMediaPipelineAssetDescAttributeSamplingRate {
    #[default]
    LinearMediaPipelineAssetDescAttributeSamplingRate_Frame = 0,
    LinearMediaPipelineAssetDescAttributeSamplingRate_Static = 1,
}

pub static LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaPipelineAssetDescAttributeSamplingRate",
    flags: MemberInfoFlags::new(49429),
    module: "LinearMedia",
    data: TypeInfoData::Enum,
    array_type: Some(LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LinearMediaPipelineAssetDescAttributeSamplingRate {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaPipelineAssetDescAttributeSamplingRate-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaPipelineAssetDescAttributeSamplingRate"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaChannelRuntime {
}

pub trait LinearMediaChannelRuntimeTrait: TypeObject {
}

impl LinearMediaChannelRuntimeTrait for LinearMediaChannelRuntime {
}

pub static LINEARMEDIACHANNELRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaChannelRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaChannelRuntime as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIACHANNELRUNTIME_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaChannelRuntime {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIACHANNELRUNTIME_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARMEDIACHANNELRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaChannelRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaChannelRuntime"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearMediaAsset {
}

pub trait LinearMediaAssetTrait: TypeObject {
}

impl LinearMediaAssetTrait for LinearMediaAsset {
}

pub static LINEARMEDIAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAsset",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearMediaAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIAASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaAsset {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARMEDIAASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARMEDIAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaAsset"),
    array_type: None,
    alignment: 8,
};


