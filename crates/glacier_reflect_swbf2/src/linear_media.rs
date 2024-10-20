use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaSettings {
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

pub const LINEARMEDIASETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaSettings",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InlineMessageProcessCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, inline_message_process_count_max),
            },
            FieldInfoData {
                name: "PerJobMessageProcessCountMax",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, per_job_message_process_count_max),
            },
            FieldInfoData {
                name: "InlineMessageProcessTimeMaxNS",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, inline_message_process_time_max_n_s),
            },
            FieldInfoData {
                name: "PerJobMessageProcessTimeMaxNS",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, per_job_message_process_time_max_n_s),
            },
            FieldInfoData {
                name: "TurboLoaderChunkPollFrequencyNS",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, turbo_loader_chunk_poll_frequency_n_s),
            },
            FieldInfoData {
                name: "QueueMemoryReapFrequencyNS",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, queue_memory_reap_frequency_n_s),
            },
            FieldInfoData {
                name: "MemoryRebalanceFrequencyNS",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, memory_rebalance_frequency_n_s),
            },
            FieldInfoData {
                name: "MemoryRebalanceTimeMaxNS",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, memory_rebalance_time_max_n_s),
            },
            FieldInfoData {
                name: "MemoryRebalanceMaxItemCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, memory_rebalance_max_item_count),
            },
            FieldInfoData {
                name: "MessageProcessSpinCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, message_process_spin_count),
            },
            FieldInfoData {
                name: "QueuePriority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, queue_priority),
            },
            FieldInfoData {
                name: "QueueAffinity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, queue_affinity),
            },
            FieldInfoData {
                name: "ProcessPriority",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, process_priority),
            },
            FieldInfoData {
                name: "ProcessAffinity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, process_affinity),
            },
            FieldInfoData {
                name: "MaxConcurrentDispatchJobs",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, max_concurrent_dispatch_jobs),
            },
            FieldInfoData {
                name: "CpuPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, cpu_pool_size),
            },
            FieldInfoData {
                name: "GpuPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, gpu_pool_size),
            },
            FieldInfoData {
                name: "TrackHeapsS",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaSettings, track_heaps_s),
            },
        ],
    }),
    array_type: Some(LINEARMEDIASETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaSettings {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIASETTINGS_TYPE_INFO
    }
}


pub const LINEARMEDIASETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaAssetDesc {
    pub resources: Vec<LinearMediaRuntimeResource>,
}

pub const LINEARMEDIAASSETDESC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAssetDesc",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Resources",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARMEDIARUNTIMERESOURCE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaAssetDesc, resources),
            },
        ],
    }),
    array_type: Some(LINEARMEDIAASSETDESC_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaAssetDesc {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIAASSETDESC_TYPE_INFO
    }
}


pub const LINEARMEDIAASSETDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAssetDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaAssetDesc-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaRuntimeResource {
    pub resource: super::core::ResourceRef,
}

pub const LINEARMEDIARUNTIMERESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaRuntimeResource",
    flags: MemberInfoFlags::new(73),
    module: "LinearMedia",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(LinearMediaRuntimeResource, resource),
            },
        ],
    }),
    array_type: Some(LINEARMEDIARUNTIMERESOURCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinearMediaRuntimeResource {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIARUNTIMERESOURCE_TYPE_INFO
    }
}


pub const LINEARMEDIARUNTIMERESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaRuntimeResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaRuntimeResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LinearMediaPipelineAssetDescAttributeSamplingRate {
    #[default]
    LinearMediaPipelineAssetDescAttributeSamplingRate_Frame = 0,
    LinearMediaPipelineAssetDescAttributeSamplingRate_Static = 1,
}

pub const LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaPipelineAssetDescAttributeSamplingRate",
    flags: MemberInfoFlags::new(49429),
    module: "LinearMedia",
    data: TypeInfoData::Enum,
    array_type: Some(LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LinearMediaPipelineAssetDescAttributeSamplingRate {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_TYPE_INFO
    }
}


pub const LINEARMEDIAPIPELINEASSETDESCATTRIBUTESAMPLINGRATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaPipelineAssetDescAttributeSamplingRate-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaPipelineAssetDescAttributeSamplingRate-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaChannelRuntime {
}

pub const LINEARMEDIACHANNELRUNTIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaChannelRuntime",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIACHANNELRUNTIME_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaChannelRuntime {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIACHANNELRUNTIME_TYPE_INFO
    }
}


pub const LINEARMEDIACHANNELRUNTIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaChannelRuntime-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaChannelRuntime-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearMediaAsset {
}

pub const LINEARMEDIAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAsset",
    flags: MemberInfoFlags::new(101),
    module: "LinearMedia",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(LINEARMEDIAASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinearMediaAsset {
    fn type_info() -> &'static TypeInfo {
        LINEARMEDIAASSET_TYPE_INFO
    }
}


pub const LINEARMEDIAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearMediaAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "LinearMedia",
    data: TypeInfoData::Array("LinearMediaAsset-Array"),
    array_type: None,
    alignment: 8,
};


