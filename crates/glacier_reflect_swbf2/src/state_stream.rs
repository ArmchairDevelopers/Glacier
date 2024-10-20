use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_state_stream_types(registry: &mut TypeRegistry) {
    registry.register_type(REPLAYSETTINGS_TYPE_INFO);
    registry.register_type(REPLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(EMPTYDYNAMICSTATE_TYPE_INFO);
    registry.register_type(EMPTYDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EMPTYSTATICSTATE_TYPE_INFO);
    registry.register_type(EMPTYSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BUNDLEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(BUNDLEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BUNDLESTATICSTATE_TYPE_INFO);
    registry.register_type(BUNDLESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSPACEHANDLE_TYPE_INFO);
    registry.register_type(TRANSFORMSPACEHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(SKELETONHANDLE_TYPE_INFO);
    registry.register_type(SKELETONHANDLE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReplaySettings {
    pub enable: bool,
    pub heap_core_size_in_m_b: u32,
    pub heap_reserve_size_in_m_b: u32,
    pub heap_allow_grow: bool,
    pub clip_max_size_in_k_b: u32,
    pub clip_s_b_a_size_in_k_b: u32,
    pub clip_max_size_compressed_in_k_b: u32,
    pub frames_per_clip: u32,
    pub prefetch_clips: bool,
    pub uncompressed_frame_count: u32,
    pub uncompressed_frame_count_read_only: u32,
    pub toc_entries: u32,
    pub toc_pinned_entries_percentage: u32,
    pub v_f_s_mount_point: String,
    pub buffer_size_in_m_b: u32,
    pub cache_page_size_in_k_b: u32,
    pub cache_size_in_m_b: u32,
    pub l_z4_software_compression_block_size_in_k_b: u32,
    pub z_lib_hardware_compression_block_size_in_k_b: u32,
    pub compress_end_clips: bool,
}

pub const REPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReplaySettings",
    flags: MemberInfoFlags::new(101),
    module: "StateStream",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, enable),
            },
            FieldInfoData {
                name: "HeapCoreSizeInMB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, heap_core_size_in_m_b),
            },
            FieldInfoData {
                name: "HeapReserveSizeInMB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, heap_reserve_size_in_m_b),
            },
            FieldInfoData {
                name: "HeapAllowGrow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, heap_allow_grow),
            },
            FieldInfoData {
                name: "ClipMaxSizeInKB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, clip_max_size_in_k_b),
            },
            FieldInfoData {
                name: "ClipSBASizeInKB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, clip_s_b_a_size_in_k_b),
            },
            FieldInfoData {
                name: "ClipMaxSizeCompressedInKB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, clip_max_size_compressed_in_k_b),
            },
            FieldInfoData {
                name: "FramesPerClip",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, frames_per_clip),
            },
            FieldInfoData {
                name: "PrefetchClips",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, prefetch_clips),
            },
            FieldInfoData {
                name: "UncompressedFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, uncompressed_frame_count),
            },
            FieldInfoData {
                name: "UncompressedFrameCountReadOnly",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, uncompressed_frame_count_read_only),
            },
            FieldInfoData {
                name: "TocEntries",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, toc_entries),
            },
            FieldInfoData {
                name: "TocPinnedEntriesPercentage",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, toc_pinned_entries_percentage),
            },
            FieldInfoData {
                name: "VFSMountPoint",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, v_f_s_mount_point),
            },
            FieldInfoData {
                name: "BufferSizeInMB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, buffer_size_in_m_b),
            },
            FieldInfoData {
                name: "CachePageSizeInKB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, cache_page_size_in_k_b),
            },
            FieldInfoData {
                name: "CacheSizeInMB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, cache_size_in_m_b),
            },
            FieldInfoData {
                name: "LZ4SoftwareCompressionBlockSizeInKB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, l_z4_software_compression_block_size_in_k_b),
            },
            FieldInfoData {
                name: "ZLibHardwareCompressionBlockSizeInKB",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, z_lib_hardware_compression_block_size_in_k_b),
            },
            FieldInfoData {
                name: "CompressEndClips",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ReplaySettings, compress_end_clips),
            },
        ],
    }),
    array_type: Some(REPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReplaySettings {
    fn type_info() -> &'static TypeInfo {
        REPLAYSETTINGS_TYPE_INFO
    }
}


pub const REPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReplaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("ReplaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmptyDynamicState {
}

pub const EMPTYDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "StateStream",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(EMPTYDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmptyDynamicState {
    fn type_info() -> &'static TypeInfo {
        EMPTYDYNAMICSTATE_TYPE_INFO
    }
}


pub const EMPTYDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("EmptyDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmptyStaticState {
}

pub const EMPTYSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "StateStream",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(EMPTYSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmptyStaticState {
    fn type_info() -> &'static TypeInfo {
        EMPTYSTATICSTATE_TYPE_INFO
    }
}


pub const EMPTYSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("EmptyStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BundleDynamicState {
    pub is_loaded: bool,
    pub field_flag_changed0: u8,
}

pub const BUNDLEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "StateStream",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsLoaded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BundleDynamicState, is_loaded),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(BundleDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BUNDLEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BundleDynamicState {
    fn type_info() -> &'static TypeInfo {
        BUNDLEDYNAMICSTATE_TYPE_INFO
    }
}


pub const BUNDLEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("BundleDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BundleStaticState {
    pub name: String,
    pub compartment: i32,
    pub compartment_hash: u32,
    pub bundle_id: i32,
    pub field_flag_changed0: u8,
}

pub const BUNDLESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleStaticState",
    flags: MemberInfoFlags::new(73),
    module: "StateStream",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BundleStaticState, name),
            },
            FieldInfoData {
                name: "Compartment",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BundleStaticState, compartment),
            },
            FieldInfoData {
                name: "CompartmentHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BundleStaticState, compartment_hash),
            },
            FieldInfoData {
                name: "BundleId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BundleStaticState, bundle_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(BundleStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BUNDLESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BundleStaticState {
    fn type_info() -> &'static TypeInfo {
        BUNDLESTATICSTATE_TYPE_INFO
    }
}


pub const BUNDLESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("BundleStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformSpaceHandle {
}

pub const TRANSFORMSPACEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceHandle",
    flags: MemberInfoFlags::new(73),
    module: "StateStream",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMSPACEHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TransformSpaceHandle {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSPACEHANDLE_TYPE_INFO
    }
}


pub const TRANSFORMSPACEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("TransformSpaceHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SkeletonHandle {
}

pub const SKELETONHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonHandle",
    flags: MemberInfoFlags::new(73),
    module: "StateStream",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SKELETONHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SkeletonHandle {
    fn type_info() -> &'static TypeInfo {
        SKELETONHANDLE_TYPE_INFO
    }
}


pub const SKELETONHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("SkeletonHandle-Array"),
    array_type: None,
    alignment: 8,
};


