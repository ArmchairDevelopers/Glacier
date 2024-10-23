use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_l_m_s_cinematic_destruction_types(registry: &mut TypeRegistry) {
    registry.register_type(CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONSEGMENTGROUPORDER_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONSEGMENTGROUPORDER_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTDATAATTRIBUTECPUPARAMETER_TYPE_INFO);
    registry.register_type(EFFECTDATAATTRIBUTECPUPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO);
    registry.register_type(EFFECTDATAATTRIBUTEPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(MESHDATAREPLACETEXTUREATTRIBUTE_TYPE_INFO);
    registry.register_type(MESHDATAREPLACETEXTUREATTRIBUTE_ARRAY_TYPE_INFO);
    registry.register_type(MESHDATAREPLACEVERTEXATTRIBUTE_TYPE_INFO);
    registry.register_type(MESHDATAREPLACEVERTEXATTRIBUTE_ARRAY_TYPE_INFO);
    registry.register_type(MESHDATAREPLACEATTRIBUTE_TYPE_INFO);
    registry.register_type(MESHDATAREPLACEATTRIBUTE_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_ARRAY_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONASSET_TYPE_INFO);
    registry.register_type(CINEMATICDESTRUCTIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO);
    registry.register_type(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionPoolBufferEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub cpu_pool_adjustment: i32,
    pub gpu_pool_adjustment: i32,
    pub enabled: bool,
}

pub trait CinematicDestructionPoolBufferEntityDataTrait: super::entity::EntityDataTrait {
    fn cpu_pool_adjustment(&self) -> &i32;
    fn cpu_pool_adjustment_mut(&mut self) -> &mut i32;
    fn gpu_pool_adjustment(&self) -> &i32;
    fn gpu_pool_adjustment_mut(&mut self) -> &mut i32;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
}

impl CinematicDestructionPoolBufferEntityDataTrait for CinematicDestructionPoolBufferEntityData {
    fn cpu_pool_adjustment(&self) -> &i32 {
        &self.cpu_pool_adjustment
    }
    fn cpu_pool_adjustment_mut(&mut self) -> &mut i32 {
        &mut self.cpu_pool_adjustment
    }
    fn gpu_pool_adjustment(&self) -> &i32 {
        &self.gpu_pool_adjustment
    }
    fn gpu_pool_adjustment_mut(&mut self) -> &mut i32 {
        &mut self.gpu_pool_adjustment
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}

impl super::entity::EntityDataTrait for CinematicDestructionPoolBufferEntityData {
}

impl super::entity::GameObjectDataTrait for CinematicDestructionPoolBufferEntityData {
}

impl super::core::DataBusPeerTrait for CinematicDestructionPoolBufferEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicDestructionPoolBufferEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionPoolBufferEntityData {
}

pub static CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPoolBufferEntityData",
    name_hash: 2630206527,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionPoolBufferEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionPoolBufferEntityData as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionPoolBufferEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CpuPoolAdjustment",
                name_hash: 1490645680,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, cpu_pool_adjustment),
            },
            FieldInfoData {
                name: "GpuPoolAdjustment",
                name_hash: 1174958516,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, gpu_pool_adjustment),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, enabled),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionPoolBufferEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPoolBufferEntityData-Array",
    name_hash: 212359563,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionPoolBufferEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionControllerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub source_asset: Option<LockedTypeObject /* CinematicDestructionAsset */>,
    pub output_pipe_results: Vec<Option<LockedTypeObject /* CinematicDestructionOutputPipeResult */>>,
    pub enabled: bool,
    pub external_time: f32,
    pub start_paused: bool,
    pub active_playback_sequence: i32,
    pub auto_start: bool,
    pub auto_pause: bool,
    pub playback_sequences: Vec<BoxedTypeObject /* CinematicDestructionPlaybackSequence */>,
}

pub trait CinematicDestructionControllerEntityDataTrait: super::entity::EntityDataTrait {
    fn source_asset(&self) -> &Option<LockedTypeObject /* CinematicDestructionAsset */>;
    fn source_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* CinematicDestructionAsset */>;
    fn output_pipe_results(&self) -> &Vec<Option<LockedTypeObject /* CinematicDestructionOutputPipeResult */>>;
    fn output_pipe_results_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CinematicDestructionOutputPipeResult */>>;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn external_time(&self) -> &f32;
    fn external_time_mut(&mut self) -> &mut f32;
    fn start_paused(&self) -> &bool;
    fn start_paused_mut(&mut self) -> &mut bool;
    fn active_playback_sequence(&self) -> &i32;
    fn active_playback_sequence_mut(&mut self) -> &mut i32;
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
    fn auto_pause(&self) -> &bool;
    fn auto_pause_mut(&mut self) -> &mut bool;
    fn playback_sequences(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionPlaybackSequence */>;
    fn playback_sequences_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionPlaybackSequence */>;
}

impl CinematicDestructionControllerEntityDataTrait for CinematicDestructionControllerEntityData {
    fn source_asset(&self) -> &Option<LockedTypeObject /* CinematicDestructionAsset */> {
        &self.source_asset
    }
    fn source_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* CinematicDestructionAsset */> {
        &mut self.source_asset
    }
    fn output_pipe_results(&self) -> &Vec<Option<LockedTypeObject /* CinematicDestructionOutputPipeResult */>> {
        &self.output_pipe_results
    }
    fn output_pipe_results_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* CinematicDestructionOutputPipeResult */>> {
        &mut self.output_pipe_results
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn external_time_mut(&mut self) -> &mut f32 {
        &mut self.external_time
    }
    fn start_paused(&self) -> &bool {
        &self.start_paused
    }
    fn start_paused_mut(&mut self) -> &mut bool {
        &mut self.start_paused
    }
    fn active_playback_sequence(&self) -> &i32 {
        &self.active_playback_sequence
    }
    fn active_playback_sequence_mut(&mut self) -> &mut i32 {
        &mut self.active_playback_sequence
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
    fn auto_pause(&self) -> &bool {
        &self.auto_pause
    }
    fn auto_pause_mut(&mut self) -> &mut bool {
        &mut self.auto_pause
    }
    fn playback_sequences(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionPlaybackSequence */> {
        &self.playback_sequences
    }
    fn playback_sequences_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionPlaybackSequence */> {
        &mut self.playback_sequences
    }
}

impl super::entity::EntityDataTrait for CinematicDestructionControllerEntityData {
}

impl super::entity::GameObjectDataTrait for CinematicDestructionControllerEntityData {
}

impl super::core::DataBusPeerTrait for CinematicDestructionControllerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicDestructionControllerEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionControllerEntityData {
}

pub static CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionControllerEntityData",
    name_hash: 2655061855,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionControllerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionControllerEntityData as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionControllerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SourceAsset",
                name_hash: 2996111944,
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicDestructionAsset",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, source_asset),
            },
            FieldInfoData {
                name: "OutputPipeResults",
                name_hash: 3990442124,
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionOutputPipeResult-Array",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, output_pipe_results),
            },
            FieldInfoData {
                name: "Enabled",
                name_hash: 2662400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, enabled),
            },
            FieldInfoData {
                name: "ExternalTime",
                name_hash: 2162678253,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, external_time),
            },
            FieldInfoData {
                name: "StartPaused",
                name_hash: 735997331,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, start_paused),
            },
            FieldInfoData {
                name: "ActivePlaybackSequence",
                name_hash: 1484959193,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, active_playback_sequence),
            },
            FieldInfoData {
                name: "AutoStart",
                name_hash: 792615882,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, auto_start),
            },
            FieldInfoData {
                name: "AutoPause",
                name_hash: 792162712,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, auto_pause),
            },
            FieldInfoData {
                name: "PlaybackSequences",
                name_hash: 473671750,
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionPlaybackSequence-Array",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, playback_sequences),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionControllerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionControllerEntityData-Array",
    name_hash: 2245852267,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionControllerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionPlaybackSequence {
    pub segment_table: Vec<BoxedTypeObject /* CinematicDestructionBakedSegmentGroupOrder */>,
}

pub trait CinematicDestructionPlaybackSequenceTrait: TypeObject {
    fn segment_table(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionBakedSegmentGroupOrder */>;
    fn segment_table_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionBakedSegmentGroupOrder */>;
}

impl CinematicDestructionPlaybackSequenceTrait for CinematicDestructionPlaybackSequence {
    fn segment_table(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionBakedSegmentGroupOrder */> {
        &self.segment_table
    }
    fn segment_table_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionBakedSegmentGroupOrder */> {
        &mut self.segment_table
    }
}

pub static CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPlaybackSequence",
    name_hash: 1625168408,
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionPlaybackSequence as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionPlaybackSequence as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SegmentTable",
                name_hash: 3503285848,
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionBakedSegmentGroupOrder-Array",
                rust_offset: offset_of!(CinematicDestructionPlaybackSequence, segment_table),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionPlaybackSequence {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPlaybackSequence-Array",
    name_hash: 4030358572,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionPlaybackSequence"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionBakedSegmentGroupOrder {
    pub segment: u32,
    pub next_segment: u32,
}

pub trait CinematicDestructionBakedSegmentGroupOrderTrait: TypeObject {
    fn segment(&self) -> &u32;
    fn segment_mut(&mut self) -> &mut u32;
    fn next_segment(&self) -> &u32;
    fn next_segment_mut(&mut self) -> &mut u32;
}

impl CinematicDestructionBakedSegmentGroupOrderTrait for CinematicDestructionBakedSegmentGroupOrder {
    fn segment(&self) -> &u32 {
        &self.segment
    }
    fn segment_mut(&mut self) -> &mut u32 {
        &mut self.segment
    }
    fn next_segment(&self) -> &u32 {
        &self.next_segment
    }
    fn next_segment_mut(&mut self) -> &mut u32 {
        &mut self.next_segment
    }
}

pub static CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionBakedSegmentGroupOrder",
    name_hash: 3464133011,
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionBakedSegmentGroupOrder as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionBakedSegmentGroupOrder as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Segment",
                name_hash: 2765640422,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CinematicDestructionBakedSegmentGroupOrder, segment),
            },
            FieldInfoData {
                name: "NextSegment",
                name_hash: 3895044129,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CinematicDestructionBakedSegmentGroupOrder, next_segment),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CinematicDestructionBakedSegmentGroupOrder {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionBakedSegmentGroupOrder-Array",
    name_hash: 2416202791,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionBakedSegmentGroupOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionSegmentGroupOrder {
    pub segment_group: String,
    pub next_segment_group: String,
}

pub trait CinematicDestructionSegmentGroupOrderTrait: TypeObject {
    fn segment_group(&self) -> &String;
    fn segment_group_mut(&mut self) -> &mut String;
    fn next_segment_group(&self) -> &String;
    fn next_segment_group_mut(&mut self) -> &mut String;
}

impl CinematicDestructionSegmentGroupOrderTrait for CinematicDestructionSegmentGroupOrder {
    fn segment_group(&self) -> &String {
        &self.segment_group
    }
    fn segment_group_mut(&mut self) -> &mut String {
        &mut self.segment_group
    }
    fn next_segment_group(&self) -> &String {
        &self.next_segment_group
    }
    fn next_segment_group_mut(&mut self) -> &mut String {
        &mut self.next_segment_group
    }
}

pub static CINEMATICDESTRUCTIONSEGMENTGROUPORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionSegmentGroupOrder",
    name_hash: 2240060282,
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionSegmentGroupOrder as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionSegmentGroupOrder as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SegmentGroup",
                name_hash: 3485981465,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CinematicDestructionSegmentGroupOrder, segment_group),
            },
            FieldInfoData {
                name: "NextSegmentGroup",
                name_hash: 2503477502,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CinematicDestructionSegmentGroupOrder, next_segment_group),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONSEGMENTGROUPORDER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionSegmentGroupOrder {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONSEGMENTGROUPORDER_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONSEGMENTGROUPORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionSegmentGroupOrder-Array",
    name_hash: 1947496014,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionSegmentGroupOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionEmitterOutputPipeResult {
    pub _glacier_base: CinematicDestructionOutputPipeResult,
    pub attributes: Vec<BoxedTypeObject /* CinematicDestructionEmitterOutputPipeAttribute */>,
}

pub trait CinematicDestructionEmitterOutputPipeResultTrait: CinematicDestructionOutputPipeResultTrait {
    fn attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionEmitterOutputPipeAttribute */>;
    fn attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionEmitterOutputPipeAttribute */>;
}

impl CinematicDestructionEmitterOutputPipeResultTrait for CinematicDestructionEmitterOutputPipeResult {
    fn attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionEmitterOutputPipeAttribute */> {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionEmitterOutputPipeAttribute */> {
        &mut self.attributes
    }
}

impl CinematicDestructionOutputPipeResultTrait for CinematicDestructionEmitterOutputPipeResult {
}

impl super::core::DataContainerTrait for CinematicDestructionEmitterOutputPipeResult {
}

pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeResult",
    name_hash: 678645636,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionEmitterOutputPipeResult, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionEmitterOutputPipeResult as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionEmitterOutputPipeResult as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Attributes",
                name_hash: 3723762538,
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionEmitterOutputPipeAttribute-Array",
                rust_offset: offset_of!(CinematicDestructionEmitterOutputPipeResult, attributes),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionEmitterOutputPipeResult {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeResult-Array",
    name_hash: 2926527408,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeResult"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionEmitterOutputPipeAttribute {
    pub effect_param: Option<LockedTypeObject /* super::effect_base::EffectParameter */>,
    pub attribute_code: u64,
}

pub trait CinematicDestructionEmitterOutputPipeAttributeTrait: TypeObject {
    fn effect_param(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectParameter */>;
    fn effect_param_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectParameter */>;
    fn attribute_code(&self) -> &u64;
    fn attribute_code_mut(&mut self) -> &mut u64;
}

impl CinematicDestructionEmitterOutputPipeAttributeTrait for CinematicDestructionEmitterOutputPipeAttribute {
    fn effect_param(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectParameter */> {
        &self.effect_param
    }
    fn effect_param_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectParameter */> {
        &mut self.effect_param
    }
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
    fn attribute_code_mut(&mut self) -> &mut u64 {
        &mut self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeAttribute",
    name_hash: 3371185073,
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionEmitterOutputPipeAttribute as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionEmitterOutputPipeAttribute as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EffectParam",
                name_hash: 1371190589,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameter",
                rust_offset: offset_of!(CinematicDestructionEmitterOutputPipeAttribute, effect_param),
            },
            FieldInfoData {
                name: "AttributeCode",
                name_hash: 2557813876,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(CinematicDestructionEmitterOutputPipeAttribute, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionEmitterOutputPipeAttribute {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeAttribute-Array",
    name_hash: 802219525,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionEmitterOutputPipeEntityData {
    pub _glacier_base: CinematicDestructionOutputPipeEntityData,
}

pub trait CinematicDestructionEmitterOutputPipeEntityDataTrait: CinematicDestructionOutputPipeEntityDataTrait {
}

impl CinematicDestructionEmitterOutputPipeEntityDataTrait for CinematicDestructionEmitterOutputPipeEntityData {
}

impl CinematicDestructionOutputPipeEntityDataTrait for CinematicDestructionEmitterOutputPipeEntityData {
}

impl super::entity::EntityDataTrait for CinematicDestructionEmitterOutputPipeEntityData {
}

impl super::entity::GameObjectDataTrait for CinematicDestructionEmitterOutputPipeEntityData {
}

impl super::core::DataBusPeerTrait for CinematicDestructionEmitterOutputPipeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicDestructionEmitterOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionEmitterOutputPipeEntityData {
}

pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeEntityData",
    name_hash: 1857829094,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionEmitterOutputPipeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionEmitterOutputPipeEntityData as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionEmitterOutputPipeEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionEmitterOutputPipeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeEntityData-Array",
    name_hash: 773436882,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionAutoMeshOutputPipeEntityData {
    pub _glacier_base: CinematicDestructionMeshOutputPipeEntityData,
}

pub trait CinematicDestructionAutoMeshOutputPipeEntityDataTrait: CinematicDestructionMeshOutputPipeEntityDataTrait {
}

impl CinematicDestructionAutoMeshOutputPipeEntityDataTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

impl CinematicDestructionMeshOutputPipeEntityDataTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

impl CinematicDestructionOutputPipeEntityDataTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

impl super::entity::EntityDataTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

impl super::entity::GameObjectDataTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

impl super::core::DataBusPeerTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

pub static CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshOutputPipeEntityData",
    name_hash: 1249477196,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionAutoMeshOutputPipeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshOutputPipeEntityData as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionAutoMeshOutputPipeEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshOutputPipeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshOutputPipeEntityData-Array",
    name_hash: 2747523832,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionManualMeshOutputPipeEntityData {
    pub _glacier_base: CinematicDestructionMeshOutputPipeEntityData,
}

pub trait CinematicDestructionManualMeshOutputPipeEntityDataTrait: CinematicDestructionMeshOutputPipeEntityDataTrait {
}

impl CinematicDestructionManualMeshOutputPipeEntityDataTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

impl CinematicDestructionMeshOutputPipeEntityDataTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

impl CinematicDestructionOutputPipeEntityDataTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

impl super::entity::EntityDataTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

impl super::entity::GameObjectDataTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

impl super::core::DataBusPeerTrait for CinematicDestructionManualMeshOutputPipeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

pub static CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionManualMeshOutputPipeEntityData",
    name_hash: 2857415801,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionManualMeshOutputPipeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionManualMeshOutputPipeEntityData as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionManualMeshOutputPipeEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionManualMeshOutputPipeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionManualMeshOutputPipeEntityData-Array",
    name_hash: 3205403469,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionManualMeshOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionMeshOutputPipeEntityData {
    pub _glacier_base: CinematicDestructionOutputPipeEntityData,
}

pub trait CinematicDestructionMeshOutputPipeEntityDataTrait: CinematicDestructionOutputPipeEntityDataTrait {
}

impl CinematicDestructionMeshOutputPipeEntityDataTrait for CinematicDestructionMeshOutputPipeEntityData {
}

impl CinematicDestructionOutputPipeEntityDataTrait for CinematicDestructionMeshOutputPipeEntityData {
}

impl super::entity::EntityDataTrait for CinematicDestructionMeshOutputPipeEntityData {
}

impl super::entity::GameObjectDataTrait for CinematicDestructionMeshOutputPipeEntityData {
}

impl super::core::DataBusPeerTrait for CinematicDestructionMeshOutputPipeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicDestructionMeshOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionMeshOutputPipeEntityData {
}

pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeEntityData",
    name_hash: 1368205379,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionMeshOutputPipeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionMeshOutputPipeEntityData as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionMeshOutputPipeEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionMeshOutputPipeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeEntityData-Array",
    name_hash: 2427194231,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionMeshOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionMeshOutputPipeResult {
    pub _glacier_base: CinematicDestructionOutputPipeResult,
    pub vertex_attributes: Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */>,
    pub material_attributes: Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */>,
    pub texture_attributes: Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedTextureAttribute */>,
    pub replace_index_buffer: bool,
    pub index_buffer: CinematicDestructionAutoMeshGeneratedIndexBuffer,
}

pub trait CinematicDestructionMeshOutputPipeResultTrait: CinematicDestructionOutputPipeResultTrait {
    fn vertex_attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */>;
    fn vertex_attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */>;
    fn material_attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */>;
    fn material_attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */>;
    fn texture_attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedTextureAttribute */>;
    fn texture_attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedTextureAttribute */>;
    fn replace_index_buffer(&self) -> &bool;
    fn replace_index_buffer_mut(&mut self) -> &mut bool;
    fn index_buffer(&self) -> &CinematicDestructionAutoMeshGeneratedIndexBuffer;
    fn index_buffer_mut(&mut self) -> &mut CinematicDestructionAutoMeshGeneratedIndexBuffer;
}

impl CinematicDestructionMeshOutputPipeResultTrait for CinematicDestructionMeshOutputPipeResult {
    fn vertex_attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */> {
        &self.vertex_attributes
    }
    fn vertex_attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */> {
        &mut self.vertex_attributes
    }
    fn material_attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */> {
        &self.material_attributes
    }
    fn material_attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedVertexAttribute */> {
        &mut self.material_attributes
    }
    fn texture_attributes(&self) -> &Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedTextureAttribute */> {
        &self.texture_attributes
    }
    fn texture_attributes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CinematicDestructionAutoMeshGeneratedTextureAttribute */> {
        &mut self.texture_attributes
    }
    fn replace_index_buffer(&self) -> &bool {
        &self.replace_index_buffer
    }
    fn replace_index_buffer_mut(&mut self) -> &mut bool {
        &mut self.replace_index_buffer
    }
    fn index_buffer(&self) -> &CinematicDestructionAutoMeshGeneratedIndexBuffer {
        &self.index_buffer
    }
    fn index_buffer_mut(&mut self) -> &mut CinematicDestructionAutoMeshGeneratedIndexBuffer {
        &mut self.index_buffer
    }
}

impl CinematicDestructionOutputPipeResultTrait for CinematicDestructionMeshOutputPipeResult {
}

impl super::core::DataContainerTrait for CinematicDestructionMeshOutputPipeResult {
}

pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeResult",
    name_hash: 3981244513,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionMeshOutputPipeResult as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionMeshOutputPipeResult as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "VertexAttributes",
                name_hash: 1676558626,
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionAutoMeshGeneratedVertexAttribute-Array",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, vertex_attributes),
            },
            FieldInfoData {
                name: "MaterialAttributes",
                name_hash: 3599326529,
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionAutoMeshGeneratedVertexAttribute-Array",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, material_attributes),
            },
            FieldInfoData {
                name: "TextureAttributes",
                name_hash: 3920023413,
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionAutoMeshGeneratedTextureAttribute-Array",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, texture_attributes),
            },
            FieldInfoData {
                name: "ReplaceIndexBuffer",
                name_hash: 138989687,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, replace_index_buffer),
            },
            FieldInfoData {
                name: "IndexBuffer",
                name_hash: 3711725403,
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicDestructionAutoMeshGeneratedIndexBuffer",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, index_buffer),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionMeshOutputPipeResult {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeResult-Array",
    name_hash: 3038491477,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionMeshOutputPipeResult"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionAutoMeshGeneratedIndexBuffer {
    pub attribute_code: u64,
}

pub trait CinematicDestructionAutoMeshGeneratedIndexBufferTrait: TypeObject {
    fn attribute_code(&self) -> &u64;
    fn attribute_code_mut(&mut self) -> &mut u64;
}

impl CinematicDestructionAutoMeshGeneratedIndexBufferTrait for CinematicDestructionAutoMeshGeneratedIndexBuffer {
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
    fn attribute_code_mut(&mut self) -> &mut u64 {
        &mut self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedIndexBuffer",
    name_hash: 3824341093,
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshGeneratedIndexBuffer as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionAutoMeshGeneratedIndexBuffer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AttributeCode",
                name_hash: 2557813876,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedIndexBuffer, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshGeneratedIndexBuffer {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedIndexBuffer-Array",
    name_hash: 2550348113,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedIndexBuffer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionAutoMeshGeneratedTextureAttribute {
    pub usage: CinematicDestructionTextureReplaceUsage,
    pub attribute_code: u64,
}

pub trait CinematicDestructionAutoMeshGeneratedTextureAttributeTrait: TypeObject {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage;
    fn usage_mut(&mut self) -> &mut CinematicDestructionTextureReplaceUsage;
    fn attribute_code(&self) -> &u64;
    fn attribute_code_mut(&mut self) -> &mut u64;
}

impl CinematicDestructionAutoMeshGeneratedTextureAttributeTrait for CinematicDestructionAutoMeshGeneratedTextureAttribute {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage {
        &self.usage
    }
    fn usage_mut(&mut self) -> &mut CinematicDestructionTextureReplaceUsage {
        &mut self.usage
    }
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
    fn attribute_code_mut(&mut self) -> &mut u64 {
        &mut self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedTextureAttribute",
    name_hash: 669056376,
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshGeneratedTextureAttribute as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionAutoMeshGeneratedTextureAttribute as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
                name_hash: 219072544,
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicDestructionTextureReplaceUsage",
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedTextureAttribute, usage),
            },
            FieldInfoData {
                name: "AttributeCode",
                name_hash: 2557813876,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedTextureAttribute, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshGeneratedTextureAttribute {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedTextureAttribute-Array",
    name_hash: 2634422092,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedTextureAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionAutoMeshGeneratedVertexAttribute {
    pub usage: super::render::VertexElementUsage,
    pub attribute_code: u64,
}

pub trait CinematicDestructionAutoMeshGeneratedVertexAttributeTrait: TypeObject {
    fn usage(&self) -> &super::render::VertexElementUsage;
    fn usage_mut(&mut self) -> &mut super::render::VertexElementUsage;
    fn attribute_code(&self) -> &u64;
    fn attribute_code_mut(&mut self) -> &mut u64;
}

impl CinematicDestructionAutoMeshGeneratedVertexAttributeTrait for CinematicDestructionAutoMeshGeneratedVertexAttribute {
    fn usage(&self) -> &super::render::VertexElementUsage {
        &self.usage
    }
    fn usage_mut(&mut self) -> &mut super::render::VertexElementUsage {
        &mut self.usage
    }
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
    fn attribute_code_mut(&mut self) -> &mut u64 {
        &mut self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedVertexAttribute",
    name_hash: 1514423631,
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshGeneratedVertexAttribute as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionAutoMeshGeneratedVertexAttribute as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
                name_hash: 219072544,
                flags: MemberInfoFlags::new(0),
                field_type: "VertexElementUsage",
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedVertexAttribute, usage),
            },
            FieldInfoData {
                name: "AttributeCode",
                name_hash: 2557813876,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedVertexAttribute, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshGeneratedVertexAttribute {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedVertexAttribute-Array",
    name_hash: 3276010107,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedVertexAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectDataAttributeCpuParameter {
    pub _glacier_base: EffectDataAttributeParameter,
}

pub trait EffectDataAttributeCpuParameterTrait: EffectDataAttributeParameterTrait {
}

impl EffectDataAttributeCpuParameterTrait for EffectDataAttributeCpuParameter {
}

impl EffectDataAttributeParameterTrait for EffectDataAttributeCpuParameter {
}

impl super::core::DataContainerTrait for EffectDataAttributeCpuParameter {
}

pub static EFFECTDATAATTRIBUTECPUPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeCpuParameter",
    name_hash: 3322690769,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO),
        super_class_offset: offset_of!(EffectDataAttributeCpuParameter, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectDataAttributeCpuParameter as Default>::default())),
            create_boxed: || Box::new(<EffectDataAttributeCpuParameter as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EFFECTDATAATTRIBUTECPUPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectDataAttributeCpuParameter {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTDATAATTRIBUTECPUPARAMETER_TYPE_INFO
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


pub static EFFECTDATAATTRIBUTECPUPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeCpuParameter-Array",
    name_hash: 1747914469,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("EffectDataAttributeCpuParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectDataAttributeParameter {
    pub _glacier_base: super::core::DataContainer,
}

pub trait EffectDataAttributeParameterTrait: super::core::DataContainerTrait {
}

impl EffectDataAttributeParameterTrait for EffectDataAttributeParameter {
}

impl super::core::DataContainerTrait for EffectDataAttributeParameter {
}

pub static EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeParameter",
    name_hash: 2691587223,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(EffectDataAttributeParameter, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectDataAttributeParameter as Default>::default())),
            create_boxed: || Box::new(<EffectDataAttributeParameter as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EFFECTDATAATTRIBUTEPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectDataAttributeParameter {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO
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


pub static EFFECTDATAATTRIBUTEPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeParameter-Array",
    name_hash: 3512083747,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("EffectDataAttributeParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MeshDataReplaceTextureAttribute {
    pub _glacier_base: MeshDataReplaceAttribute,
    pub usage: CinematicDestructionTextureReplaceUsage,
}

pub trait MeshDataReplaceTextureAttributeTrait: MeshDataReplaceAttributeTrait {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage;
    fn usage_mut(&mut self) -> &mut CinematicDestructionTextureReplaceUsage;
}

impl MeshDataReplaceTextureAttributeTrait for MeshDataReplaceTextureAttribute {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage {
        &self.usage
    }
    fn usage_mut(&mut self) -> &mut CinematicDestructionTextureReplaceUsage {
        &mut self.usage
    }
}

impl MeshDataReplaceAttributeTrait for MeshDataReplaceTextureAttribute {
}

impl super::core::DataContainerTrait for MeshDataReplaceTextureAttribute {
}

pub static MESHDATAREPLACETEXTUREATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceTextureAttribute",
    name_hash: 524972617,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHDATAREPLACEATTRIBUTE_TYPE_INFO),
        super_class_offset: offset_of!(MeshDataReplaceTextureAttribute, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDataReplaceTextureAttribute as Default>::default())),
            create_boxed: || Box::new(<MeshDataReplaceTextureAttribute as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
                name_hash: 219072544,
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicDestructionTextureReplaceUsage",
                rust_offset: offset_of!(MeshDataReplaceTextureAttribute, usage),
            },
        ],
    }),
    array_type: Some(MESHDATAREPLACETEXTUREATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDataReplaceTextureAttribute {
    fn type_info(&self) -> &'static TypeInfo {
        MESHDATAREPLACETEXTUREATTRIBUTE_TYPE_INFO
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


pub static MESHDATAREPLACETEXTUREATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceTextureAttribute-Array",
    name_hash: 4136368253,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceTextureAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MeshDataReplaceVertexAttribute {
    pub _glacier_base: MeshDataReplaceAttribute,
    pub usage: super::render::VertexElementUsage,
}

pub trait MeshDataReplaceVertexAttributeTrait: MeshDataReplaceAttributeTrait {
    fn usage(&self) -> &super::render::VertexElementUsage;
    fn usage_mut(&mut self) -> &mut super::render::VertexElementUsage;
}

impl MeshDataReplaceVertexAttributeTrait for MeshDataReplaceVertexAttribute {
    fn usage(&self) -> &super::render::VertexElementUsage {
        &self.usage
    }
    fn usage_mut(&mut self) -> &mut super::render::VertexElementUsage {
        &mut self.usage
    }
}

impl MeshDataReplaceAttributeTrait for MeshDataReplaceVertexAttribute {
}

impl super::core::DataContainerTrait for MeshDataReplaceVertexAttribute {
}

pub static MESHDATAREPLACEVERTEXATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceVertexAttribute",
    name_hash: 2285573918,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHDATAREPLACEATTRIBUTE_TYPE_INFO),
        super_class_offset: offset_of!(MeshDataReplaceVertexAttribute, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDataReplaceVertexAttribute as Default>::default())),
            create_boxed: || Box::new(<MeshDataReplaceVertexAttribute as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
                name_hash: 219072544,
                flags: MemberInfoFlags::new(0),
                field_type: "VertexElementUsage",
                rust_offset: offset_of!(MeshDataReplaceVertexAttribute, usage),
            },
        ],
    }),
    array_type: Some(MESHDATAREPLACEVERTEXATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDataReplaceVertexAttribute {
    fn type_info(&self) -> &'static TypeInfo {
        MESHDATAREPLACEVERTEXATTRIBUTE_TYPE_INFO
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


pub static MESHDATAREPLACEVERTEXATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceVertexAttribute-Array",
    name_hash: 898435114,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceVertexAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MeshDataReplaceAttribute {
    pub _glacier_base: super::core::DataContainer,
}

pub trait MeshDataReplaceAttributeTrait: super::core::DataContainerTrait {
}

impl MeshDataReplaceAttributeTrait for MeshDataReplaceAttribute {
}

impl super::core::DataContainerTrait for MeshDataReplaceAttribute {
}

pub static MESHDATAREPLACEATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceAttribute",
    name_hash: 778177366,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(MeshDataReplaceAttribute, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDataReplaceAttribute as Default>::default())),
            create_boxed: || Box::new(<MeshDataReplaceAttribute as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHDATAREPLACEATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDataReplaceAttribute {
    fn type_info(&self) -> &'static TypeInfo {
        MESHDATAREPLACEATTRIBUTE_TYPE_INFO
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


pub static MESHDATAREPLACEATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceAttribute-Array",
    name_hash: 3883602146,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionOutputPipeEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait CinematicDestructionOutputPipeEntityDataTrait: super::entity::EntityDataTrait {
}

impl CinematicDestructionOutputPipeEntityDataTrait for CinematicDestructionOutputPipeEntityData {
}

impl super::entity::EntityDataTrait for CinematicDestructionOutputPipeEntityData {
}

impl super::entity::GameObjectDataTrait for CinematicDestructionOutputPipeEntityData {
}

impl super::core::DataBusPeerTrait for CinematicDestructionOutputPipeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CinematicDestructionOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionOutputPipeEntityData {
}

pub static CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeEntityData",
    name_hash: 3090921072,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionOutputPipeEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionOutputPipeEntityData as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionOutputPipeEntityData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionOutputPipeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeEntityData-Array",
    name_hash: 474494020,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionOutputPipeResult {
    pub _glacier_base: super::core::DataContainer,
}

pub trait CinematicDestructionOutputPipeResultTrait: super::core::DataContainerTrait {
}

impl CinematicDestructionOutputPipeResultTrait for CinematicDestructionOutputPipeResult {
}

impl super::core::DataContainerTrait for CinematicDestructionOutputPipeResult {
}

pub static CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeResult",
    name_hash: 182749970,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionOutputPipeResult, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionOutputPipeResult as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionOutputPipeResult as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionOutputPipeResult {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONOUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeResult-Array",
    name_hash: 1388212774,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionOutputPipeResult"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CinematicDestructionTextureReplaceUsage {
    #[default]
    CinematicDestructionTextureReplaceUsage_Albedo = 0,
    CinematicDestructionTextureReplaceUsage_Normal = 1,
    CinematicDestructionTextureReplaceUsage_Texture0 = 2,
    CinematicDestructionTextureReplaceUsage_Texture1 = 3,
    CinematicDestructionTextureReplaceUsage_Texture2 = 4,
    CinematicDestructionTextureReplaceUsage_Texture3 = 5,
}

pub static CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionTextureReplaceUsage",
    name_hash: 3207163006,
    flags: MemberInfoFlags::new(49429),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Enum,
    array_type: Some(CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CinematicDestructionTextureReplaceUsage {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionTextureReplaceUsage-Array",
    name_hash: 3314658634,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionTextureReplaceUsage"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CinematicDestructionAsset {
    pub _glacier_base: super::linear_media::LinearMediaAssetDesc,
}

pub trait CinematicDestructionAssetTrait: super::linear_media::LinearMediaAssetDescTrait {
}

impl CinematicDestructionAssetTrait for CinematicDestructionAsset {
}

impl super::linear_media::LinearMediaAssetDescTrait for CinematicDestructionAsset {
    fn resources(&self) -> &Vec<BoxedTypeObject /* super::linear_media::LinearMediaRuntimeResource */> {
        self._glacier_base.resources()
    }
    fn resources_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::linear_media::LinearMediaRuntimeResource */> {
        self._glacier_base.resources_mut()
    }
}

impl super::core::AssetTrait for CinematicDestructionAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for CinematicDestructionAsset {
}

pub static CINEMATICDESTRUCTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAsset",
    name_hash: 2780106840,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::linear_media::LINEARMEDIAASSETDESC_TYPE_INFO),
        super_class_offset: offset_of!(CinematicDestructionAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAsset as Default>::default())),
            create_boxed: || Box::new(<CinematicDestructionAsset as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAsset {
    fn type_info(&self) -> &'static TypeInfo {
        CINEMATICDESTRUCTIONASSET_TYPE_INFO
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


pub static CINEMATICDESTRUCTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAsset-Array",
    name_hash: 3141864172,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCinematicDestructionPoolBufferEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCinematicDestructionPoolBufferEntityTrait: super::entity::EntityTrait {
}

impl ClientCinematicDestructionPoolBufferEntityTrait for ClientCinematicDestructionPoolBufferEntity {
}

impl super::entity::EntityTrait for ClientCinematicDestructionPoolBufferEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCinematicDestructionPoolBufferEntity {
}

pub static CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionPoolBufferEntity",
    name_hash: 3240388918,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCinematicDestructionPoolBufferEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionPoolBufferEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCinematicDestructionPoolBufferEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionPoolBufferEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_TYPE_INFO
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


pub static CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionPoolBufferEntity-Array",
    name_hash: 2975341954,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionPoolBufferEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCinematicDestructionMeshOutputPipeEntity {
    pub _glacier_base: ClientCinematicDestructionOutputPipeEntity,
}

pub trait ClientCinematicDestructionMeshOutputPipeEntityTrait: ClientCinematicDestructionOutputPipeEntityTrait {
}

impl ClientCinematicDestructionMeshOutputPipeEntityTrait for ClientCinematicDestructionMeshOutputPipeEntity {
}

impl ClientCinematicDestructionOutputPipeEntityTrait for ClientCinematicDestructionMeshOutputPipeEntity {
}

impl super::entity::EntityTrait for ClientCinematicDestructionMeshOutputPipeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCinematicDestructionMeshOutputPipeEntity {
}

pub static CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionMeshOutputPipeEntity",
    name_hash: 4180416906,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCinematicDestructionMeshOutputPipeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionMeshOutputPipeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCinematicDestructionMeshOutputPipeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionMeshOutputPipeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO
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


pub static CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionMeshOutputPipeEntity-Array",
    name_hash: 1476621502,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionMeshOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCinematicDestructionManualMeshOutputPipeEntity {
    pub _glacier_base: ClientCinematicDestructionMeshOutputPipeEntity,
}

pub trait ClientCinematicDestructionManualMeshOutputPipeEntityTrait: ClientCinematicDestructionMeshOutputPipeEntityTrait {
}

impl ClientCinematicDestructionManualMeshOutputPipeEntityTrait for ClientCinematicDestructionManualMeshOutputPipeEntity {
}

impl ClientCinematicDestructionMeshOutputPipeEntityTrait for ClientCinematicDestructionManualMeshOutputPipeEntity {
}

impl ClientCinematicDestructionOutputPipeEntityTrait for ClientCinematicDestructionManualMeshOutputPipeEntity {
}

impl super::entity::EntityTrait for ClientCinematicDestructionManualMeshOutputPipeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCinematicDestructionManualMeshOutputPipeEntity {
}

pub static CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionManualMeshOutputPipeEntity",
    name_hash: 374399472,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCinematicDestructionManualMeshOutputPipeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionManualMeshOutputPipeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCinematicDestructionManualMeshOutputPipeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionManualMeshOutputPipeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_TYPE_INFO
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


pub static CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionManualMeshOutputPipeEntity-Array",
    name_hash: 2070937540,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionManualMeshOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCinematicDestructionAutoMeshOutputPipeEntity {
    pub _glacier_base: ClientCinematicDestructionMeshOutputPipeEntity,
}

pub trait ClientCinematicDestructionAutoMeshOutputPipeEntityTrait: ClientCinematicDestructionMeshOutputPipeEntityTrait {
}

impl ClientCinematicDestructionAutoMeshOutputPipeEntityTrait for ClientCinematicDestructionAutoMeshOutputPipeEntity {
}

impl ClientCinematicDestructionMeshOutputPipeEntityTrait for ClientCinematicDestructionAutoMeshOutputPipeEntity {
}

impl ClientCinematicDestructionOutputPipeEntityTrait for ClientCinematicDestructionAutoMeshOutputPipeEntity {
}

impl super::entity::EntityTrait for ClientCinematicDestructionAutoMeshOutputPipeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCinematicDestructionAutoMeshOutputPipeEntity {
}

pub static CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionAutoMeshOutputPipeEntity",
    name_hash: 3897157253,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCinematicDestructionAutoMeshOutputPipeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionAutoMeshOutputPipeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCinematicDestructionAutoMeshOutputPipeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionAutoMeshOutputPipeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_TYPE_INFO
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


pub static CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionAutoMeshOutputPipeEntity-Array",
    name_hash: 3429232177,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionAutoMeshOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCinematicDestructionEmitterOutputPipeEntity {
    pub _glacier_base: ClientCinematicDestructionOutputPipeEntity,
}

pub trait ClientCinematicDestructionEmitterOutputPipeEntityTrait: ClientCinematicDestructionOutputPipeEntityTrait {
}

impl ClientCinematicDestructionEmitterOutputPipeEntityTrait for ClientCinematicDestructionEmitterOutputPipeEntity {
}

impl ClientCinematicDestructionOutputPipeEntityTrait for ClientCinematicDestructionEmitterOutputPipeEntity {
}

impl super::entity::EntityTrait for ClientCinematicDestructionEmitterOutputPipeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCinematicDestructionEmitterOutputPipeEntity {
}

pub static CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionEmitterOutputPipeEntity",
    name_hash: 382424847,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCinematicDestructionEmitterOutputPipeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionEmitterOutputPipeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCinematicDestructionEmitterOutputPipeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionEmitterOutputPipeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_TYPE_INFO
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


pub static CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionEmitterOutputPipeEntity-Array",
    name_hash: 3960574907,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionEmitterOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCinematicDestructionControllerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCinematicDestructionControllerEntityTrait: super::entity::EntityTrait {
}

impl ClientCinematicDestructionControllerEntityTrait for ClientCinematicDestructionControllerEntity {
}

impl super::entity::EntityTrait for ClientCinematicDestructionControllerEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCinematicDestructionControllerEntity {
}

pub static CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionControllerEntity",
    name_hash: 2065437334,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCinematicDestructionControllerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionControllerEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCinematicDestructionControllerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionControllerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_TYPE_INFO
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


pub static CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionControllerEntity-Array",
    name_hash: 1688029090,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientCinematicDestructionOutputPipeEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCinematicDestructionOutputPipeEntityTrait: super::entity::EntityTrait {
}

impl ClientCinematicDestructionOutputPipeEntityTrait for ClientCinematicDestructionOutputPipeEntity {
}

impl super::entity::EntityTrait for ClientCinematicDestructionOutputPipeEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCinematicDestructionOutputPipeEntity {
}

pub static CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionOutputPipeEntity",
    name_hash: 1990061753,
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientCinematicDestructionOutputPipeEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionOutputPipeEntity as Default>::default())),
            create_boxed: || Box::new(<ClientCinematicDestructionOutputPipeEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionOutputPipeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO
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


pub static CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionOutputPipeEntity-Array",
    name_hash: 3088989453,
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


