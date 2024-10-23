use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ReplaySettings {
    pub _glacier_base: super::core::SystemSettings,
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

pub trait ReplaySettingsTrait: super::core::SystemSettingsTrait {
    fn enable(&self) -> &bool;
    fn enable_mut(&mut self) -> &mut bool;
    fn heap_core_size_in_m_b(&self) -> &u32;
    fn heap_core_size_in_m_b_mut(&mut self) -> &mut u32;
    fn heap_reserve_size_in_m_b(&self) -> &u32;
    fn heap_reserve_size_in_m_b_mut(&mut self) -> &mut u32;
    fn heap_allow_grow(&self) -> &bool;
    fn heap_allow_grow_mut(&mut self) -> &mut bool;
    fn clip_max_size_in_k_b(&self) -> &u32;
    fn clip_max_size_in_k_b_mut(&mut self) -> &mut u32;
    fn clip_s_b_a_size_in_k_b(&self) -> &u32;
    fn clip_s_b_a_size_in_k_b_mut(&mut self) -> &mut u32;
    fn clip_max_size_compressed_in_k_b(&self) -> &u32;
    fn clip_max_size_compressed_in_k_b_mut(&mut self) -> &mut u32;
    fn frames_per_clip(&self) -> &u32;
    fn frames_per_clip_mut(&mut self) -> &mut u32;
    fn prefetch_clips(&self) -> &bool;
    fn prefetch_clips_mut(&mut self) -> &mut bool;
    fn uncompressed_frame_count(&self) -> &u32;
    fn uncompressed_frame_count_mut(&mut self) -> &mut u32;
    fn uncompressed_frame_count_read_only(&self) -> &u32;
    fn uncompressed_frame_count_read_only_mut(&mut self) -> &mut u32;
    fn toc_entries(&self) -> &u32;
    fn toc_entries_mut(&mut self) -> &mut u32;
    fn toc_pinned_entries_percentage(&self) -> &u32;
    fn toc_pinned_entries_percentage_mut(&mut self) -> &mut u32;
    fn v_f_s_mount_point(&self) -> &String;
    fn v_f_s_mount_point_mut(&mut self) -> &mut String;
    fn buffer_size_in_m_b(&self) -> &u32;
    fn buffer_size_in_m_b_mut(&mut self) -> &mut u32;
    fn cache_page_size_in_k_b(&self) -> &u32;
    fn cache_page_size_in_k_b_mut(&mut self) -> &mut u32;
    fn cache_size_in_m_b(&self) -> &u32;
    fn cache_size_in_m_b_mut(&mut self) -> &mut u32;
    fn l_z4_software_compression_block_size_in_k_b(&self) -> &u32;
    fn l_z4_software_compression_block_size_in_k_b_mut(&mut self) -> &mut u32;
    fn z_lib_hardware_compression_block_size_in_k_b(&self) -> &u32;
    fn z_lib_hardware_compression_block_size_in_k_b_mut(&mut self) -> &mut u32;
    fn compress_end_clips(&self) -> &bool;
    fn compress_end_clips_mut(&mut self) -> &mut bool;
}

impl ReplaySettingsTrait for ReplaySettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn heap_core_size_in_m_b(&self) -> &u32 {
        &self.heap_core_size_in_m_b
    }
    fn heap_core_size_in_m_b_mut(&mut self) -> &mut u32 {
        &mut self.heap_core_size_in_m_b
    }
    fn heap_reserve_size_in_m_b(&self) -> &u32 {
        &self.heap_reserve_size_in_m_b
    }
    fn heap_reserve_size_in_m_b_mut(&mut self) -> &mut u32 {
        &mut self.heap_reserve_size_in_m_b
    }
    fn heap_allow_grow(&self) -> &bool {
        &self.heap_allow_grow
    }
    fn heap_allow_grow_mut(&mut self) -> &mut bool {
        &mut self.heap_allow_grow
    }
    fn clip_max_size_in_k_b(&self) -> &u32 {
        &self.clip_max_size_in_k_b
    }
    fn clip_max_size_in_k_b_mut(&mut self) -> &mut u32 {
        &mut self.clip_max_size_in_k_b
    }
    fn clip_s_b_a_size_in_k_b(&self) -> &u32 {
        &self.clip_s_b_a_size_in_k_b
    }
    fn clip_s_b_a_size_in_k_b_mut(&mut self) -> &mut u32 {
        &mut self.clip_s_b_a_size_in_k_b
    }
    fn clip_max_size_compressed_in_k_b(&self) -> &u32 {
        &self.clip_max_size_compressed_in_k_b
    }
    fn clip_max_size_compressed_in_k_b_mut(&mut self) -> &mut u32 {
        &mut self.clip_max_size_compressed_in_k_b
    }
    fn frames_per_clip(&self) -> &u32 {
        &self.frames_per_clip
    }
    fn frames_per_clip_mut(&mut self) -> &mut u32 {
        &mut self.frames_per_clip
    }
    fn prefetch_clips(&self) -> &bool {
        &self.prefetch_clips
    }
    fn prefetch_clips_mut(&mut self) -> &mut bool {
        &mut self.prefetch_clips
    }
    fn uncompressed_frame_count(&self) -> &u32 {
        &self.uncompressed_frame_count
    }
    fn uncompressed_frame_count_mut(&mut self) -> &mut u32 {
        &mut self.uncompressed_frame_count
    }
    fn uncompressed_frame_count_read_only(&self) -> &u32 {
        &self.uncompressed_frame_count_read_only
    }
    fn uncompressed_frame_count_read_only_mut(&mut self) -> &mut u32 {
        &mut self.uncompressed_frame_count_read_only
    }
    fn toc_entries(&self) -> &u32 {
        &self.toc_entries
    }
    fn toc_entries_mut(&mut self) -> &mut u32 {
        &mut self.toc_entries
    }
    fn toc_pinned_entries_percentage(&self) -> &u32 {
        &self.toc_pinned_entries_percentage
    }
    fn toc_pinned_entries_percentage_mut(&mut self) -> &mut u32 {
        &mut self.toc_pinned_entries_percentage
    }
    fn v_f_s_mount_point(&self) -> &String {
        &self.v_f_s_mount_point
    }
    fn v_f_s_mount_point_mut(&mut self) -> &mut String {
        &mut self.v_f_s_mount_point
    }
    fn buffer_size_in_m_b(&self) -> &u32 {
        &self.buffer_size_in_m_b
    }
    fn buffer_size_in_m_b_mut(&mut self) -> &mut u32 {
        &mut self.buffer_size_in_m_b
    }
    fn cache_page_size_in_k_b(&self) -> &u32 {
        &self.cache_page_size_in_k_b
    }
    fn cache_page_size_in_k_b_mut(&mut self) -> &mut u32 {
        &mut self.cache_page_size_in_k_b
    }
    fn cache_size_in_m_b(&self) -> &u32 {
        &self.cache_size_in_m_b
    }
    fn cache_size_in_m_b_mut(&mut self) -> &mut u32 {
        &mut self.cache_size_in_m_b
    }
    fn l_z4_software_compression_block_size_in_k_b(&self) -> &u32 {
        &self.l_z4_software_compression_block_size_in_k_b
    }
    fn l_z4_software_compression_block_size_in_k_b_mut(&mut self) -> &mut u32 {
        &mut self.l_z4_software_compression_block_size_in_k_b
    }
    fn z_lib_hardware_compression_block_size_in_k_b(&self) -> &u32 {
        &self.z_lib_hardware_compression_block_size_in_k_b
    }
    fn z_lib_hardware_compression_block_size_in_k_b_mut(&mut self) -> &mut u32 {
        &mut self.z_lib_hardware_compression_block_size_in_k_b
    }
    fn compress_end_clips(&self) -> &bool {
        &self.compress_end_clips
    }
    fn compress_end_clips_mut(&mut self) -> &mut bool {
        &mut self.compress_end_clips
    }
}

impl super::core::SystemSettingsTrait for ReplaySettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for ReplaySettings {
}

pub static REPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReplaySettings",
    name_hash: 4222882067,
    flags: MemberInfoFlags::new(101),
    module: "StateStream",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(ReplaySettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ReplaySettings as Default>::default())),
            create_boxed: || Box::new(<ReplaySettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ReplaySettings, enable),
            },
            FieldInfoData {
                name: "HeapCoreSizeInMB",
                name_hash: 3644408303,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, heap_core_size_in_m_b),
            },
            FieldInfoData {
                name: "HeapReserveSizeInMB",
                name_hash: 3605175380,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, heap_reserve_size_in_m_b),
            },
            FieldInfoData {
                name: "HeapAllowGrow",
                name_hash: 2346338029,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ReplaySettings, heap_allow_grow),
            },
            FieldInfoData {
                name: "ClipMaxSizeInKB",
                name_hash: 3104249612,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, clip_max_size_in_k_b),
            },
            FieldInfoData {
                name: "ClipSBASizeInKB",
                name_hash: 3779348456,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, clip_s_b_a_size_in_k_b),
            },
            FieldInfoData {
                name: "ClipMaxSizeCompressedInKB",
                name_hash: 2707575915,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, clip_max_size_compressed_in_k_b),
            },
            FieldInfoData {
                name: "FramesPerClip",
                name_hash: 315278938,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, frames_per_clip),
            },
            FieldInfoData {
                name: "PrefetchClips",
                name_hash: 297601627,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ReplaySettings, prefetch_clips),
            },
            FieldInfoData {
                name: "UncompressedFrameCount",
                name_hash: 3153540903,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, uncompressed_frame_count),
            },
            FieldInfoData {
                name: "UncompressedFrameCountReadOnly",
                name_hash: 3305476961,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, uncompressed_frame_count_read_only),
            },
            FieldInfoData {
                name: "TocEntries",
                name_hash: 700864207,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, toc_entries),
            },
            FieldInfoData {
                name: "TocPinnedEntriesPercentage",
                name_hash: 4255737327,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, toc_pinned_entries_percentage),
            },
            FieldInfoData {
                name: "VFSMountPoint",
                name_hash: 1833011271,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ReplaySettings, v_f_s_mount_point),
            },
            FieldInfoData {
                name: "BufferSizeInMB",
                name_hash: 524375912,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, buffer_size_in_m_b),
            },
            FieldInfoData {
                name: "CachePageSizeInKB",
                name_hash: 3072960689,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, cache_page_size_in_k_b),
            },
            FieldInfoData {
                name: "CacheSizeInMB",
                name_hash: 2187541860,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, cache_size_in_m_b),
            },
            FieldInfoData {
                name: "LZ4SoftwareCompressionBlockSizeInKB",
                name_hash: 1543978212,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, l_z4_software_compression_block_size_in_k_b),
            },
            FieldInfoData {
                name: "ZLibHardwareCompressionBlockSizeInKB",
                name_hash: 78462634,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ReplaySettings, z_lib_hardware_compression_block_size_in_k_b),
            },
            FieldInfoData {
                name: "CompressEndClips",
                name_hash: 3021990697,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ReplaySettings, compress_end_clips),
            },
        ],
    }),
    array_type: Some(REPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ReplaySettings {
    fn type_info(&self) -> &'static TypeInfo {
        REPLAYSETTINGS_TYPE_INFO
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


pub static REPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReplaySettings-Array",
    name_hash: 452037543,
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("ReplaySettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EmptyDynamicState {
}

pub trait EmptyDynamicStateTrait: TypeObject {
}

impl EmptyDynamicStateTrait for EmptyDynamicState {
}

pub static EMPTYDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyDynamicState",
    name_hash: 4115241490,
    flags: MemberInfoFlags::new(36937),
    module: "StateStream",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmptyDynamicState as Default>::default())),
            create_boxed: || Box::new(<EmptyDynamicState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMPTYDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmptyDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        EMPTYDYNAMICSTATE_TYPE_INFO
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


pub static EMPTYDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyDynamicState-Array",
    name_hash: 1584578854,
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("EmptyDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EmptyStaticState {
}

pub trait EmptyStaticStateTrait: TypeObject {
}

impl EmptyStaticStateTrait for EmptyStaticState {
}

pub static EMPTYSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyStaticState",
    name_hash: 3176453087,
    flags: MemberInfoFlags::new(36937),
    module: "StateStream",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EmptyStaticState as Default>::default())),
            create_boxed: || Box::new(<EmptyStaticState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EMPTYSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EmptyStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        EMPTYSTATICSTATE_TYPE_INFO
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


pub static EMPTYSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EmptyStaticState-Array",
    name_hash: 1287137515,
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("EmptyStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BundleDynamicState {
    pub is_loaded: bool,
    pub field_flag_changed0: u8,
}

pub trait BundleDynamicStateTrait: TypeObject {
    fn is_loaded(&self) -> &bool;
    fn is_loaded_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl BundleDynamicStateTrait for BundleDynamicState {
    fn is_loaded(&self) -> &bool {
        &self.is_loaded
    }
    fn is_loaded_mut(&mut self) -> &mut bool {
        &mut self.is_loaded
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static BUNDLEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleDynamicState",
    name_hash: 3424609267,
    flags: MemberInfoFlags::new(36937),
    module: "StateStream",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BundleDynamicState as Default>::default())),
            create_boxed: || Box::new(<BundleDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsLoaded",
                name_hash: 815881976,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BundleDynamicState, is_loaded),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(BundleDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BUNDLEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BundleDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        BUNDLEDYNAMICSTATE_TYPE_INFO
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


pub static BUNDLEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleDynamicState-Array",
    name_hash: 1731182535,
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("BundleDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BundleStaticState {
    pub name: String,
    pub compartment: i32,
    pub compartment_hash: u32,
    pub bundle_id: i32,
    pub field_flag_changed0: u8,
}

pub trait BundleStaticStateTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn compartment(&self) -> &i32;
    fn compartment_mut(&mut self) -> &mut i32;
    fn compartment_hash(&self) -> &u32;
    fn compartment_hash_mut(&mut self) -> &mut u32;
    fn bundle_id(&self) -> &i32;
    fn bundle_id_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl BundleStaticStateTrait for BundleStaticState {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn compartment(&self) -> &i32 {
        &self.compartment
    }
    fn compartment_mut(&mut self) -> &mut i32 {
        &mut self.compartment
    }
    fn compartment_hash(&self) -> &u32 {
        &self.compartment_hash
    }
    fn compartment_hash_mut(&mut self) -> &mut u32 {
        &mut self.compartment_hash
    }
    fn bundle_id(&self) -> &i32 {
        &self.bundle_id
    }
    fn bundle_id_mut(&mut self) -> &mut i32 {
        &mut self.bundle_id
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static BUNDLESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleStaticState",
    name_hash: 2612350302,
    flags: MemberInfoFlags::new(73),
    module: "StateStream",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BundleStaticState as Default>::default())),
            create_boxed: || Box::new(<BundleStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BundleStaticState, name),
            },
            FieldInfoData {
                name: "Compartment",
                name_hash: 1500500641,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BundleStaticState, compartment),
            },
            FieldInfoData {
                name: "CompartmentHash",
                name_hash: 1548486771,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BundleStaticState, compartment_hash),
            },
            FieldInfoData {
                name: "BundleId",
                name_hash: 1372919612,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BundleStaticState, bundle_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(BundleStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BUNDLESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BundleStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        BUNDLESTATICSTATE_TYPE_INFO
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


pub static BUNDLESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BundleStaticState-Array",
    name_hash: 2369203946,
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("BundleStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TransformSpaceHandle {
}

pub trait TransformSpaceHandleTrait: TypeObject {
}

impl TransformSpaceHandleTrait for TransformSpaceHandle {
}

pub static TRANSFORMSPACEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceHandle",
    name_hash: 2004014599,
    flags: MemberInfoFlags::new(73),
    module: "StateStream",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformSpaceHandle as Default>::default())),
            create_boxed: || Box::new(<TransformSpaceHandle as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMSPACEHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TransformSpaceHandle {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMSPACEHANDLE_TYPE_INFO
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


pub static TRANSFORMSPACEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSpaceHandle-Array",
    name_hash: 3483170483,
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("TransformSpaceHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SkeletonHandle {
}

pub trait SkeletonHandleTrait: TypeObject {
}

impl SkeletonHandleTrait for SkeletonHandle {
}

pub static SKELETONHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonHandle",
    name_hash: 1121975246,
    flags: MemberInfoFlags::new(73),
    module: "StateStream",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SkeletonHandle as Default>::default())),
            create_boxed: || Box::new(<SkeletonHandle as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SKELETONHANDLE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SkeletonHandle {
    fn type_info(&self) -> &'static TypeInfo {
        SKELETONHANDLE_TYPE_INFO
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


pub static SKELETONHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkeletonHandle-Array",
    name_hash: 783807866,
    flags: MemberInfoFlags::new(145),
    module: "StateStream",
    data: TypeInfoData::Array("SkeletonHandle"),
    array_type: None,
    alignment: 8,
};


