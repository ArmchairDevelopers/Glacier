use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionPoolBufferEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub cpu_pool_adjustment: i32,
    pub gpu_pool_adjustment: i32,
    pub enabled: bool,
}

pub trait CinematicDestructionPoolBufferEntityDataTrait: super::entity::EntityDataTrait {
    fn cpu_pool_adjustment(&self) -> &i32;
    fn gpu_pool_adjustment(&self) -> &i32;
    fn enabled(&self) -> &bool;
}

impl CinematicDestructionPoolBufferEntityDataTrait for CinematicDestructionPoolBufferEntityData {
    fn cpu_pool_adjustment(&self) -> &i32 {
        &self.cpu_pool_adjustment
    }
    fn gpu_pool_adjustment(&self) -> &i32 {
        &self.gpu_pool_adjustment
    }
    fn enabled(&self) -> &bool {
        &self.enabled
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
}

impl super::core::GameDataContainerTrait for CinematicDestructionPoolBufferEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionPoolBufferEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPoolBufferEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionPoolBufferEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CpuPoolAdjustment",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, cpu_pool_adjustment),
            },
            FieldInfoData {
                name: "GpuPoolAdjustment",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, gpu_pool_adjustment),
            },
            FieldInfoData {
                name: "Enabled",
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
}


pub static CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPoolBufferEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionPoolBufferEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionControllerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub source_asset: Option<Arc<Mutex<dyn CinematicDestructionAssetTrait>>>,
    pub output_pipe_results: Vec<Option<Arc<Mutex<dyn CinematicDestructionOutputPipeResultTrait>>>>,
    pub enabled: bool,
    pub external_time: f32,
    pub start_paused: bool,
    pub active_playback_sequence: i32,
    pub auto_start: bool,
    pub auto_pause: bool,
    pub playback_sequences: Vec<CinematicDestructionPlaybackSequence>,
}

pub trait CinematicDestructionControllerEntityDataTrait: super::entity::EntityDataTrait {
    fn source_asset(&self) -> &Option<Arc<Mutex<dyn CinematicDestructionAssetTrait>>>;
    fn output_pipe_results(&self) -> &Vec<Option<Arc<Mutex<dyn CinematicDestructionOutputPipeResultTrait>>>>;
    fn enabled(&self) -> &bool;
    fn external_time(&self) -> &f32;
    fn start_paused(&self) -> &bool;
    fn active_playback_sequence(&self) -> &i32;
    fn auto_start(&self) -> &bool;
    fn auto_pause(&self) -> &bool;
    fn playback_sequences(&self) -> &Vec<CinematicDestructionPlaybackSequence>;
}

impl CinematicDestructionControllerEntityDataTrait for CinematicDestructionControllerEntityData {
    fn source_asset(&self) -> &Option<Arc<Mutex<dyn CinematicDestructionAssetTrait>>> {
        &self.source_asset
    }
    fn output_pipe_results(&self) -> &Vec<Option<Arc<Mutex<dyn CinematicDestructionOutputPipeResultTrait>>>> {
        &self.output_pipe_results
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn start_paused(&self) -> &bool {
        &self.start_paused
    }
    fn active_playback_sequence(&self) -> &i32 {
        &self.active_playback_sequence
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_pause(&self) -> &bool {
        &self.auto_pause
    }
    fn playback_sequences(&self) -> &Vec<CinematicDestructionPlaybackSequence> {
        &self.playback_sequences
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
}

impl super::core::GameDataContainerTrait for CinematicDestructionControllerEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionControllerEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionControllerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionControllerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SourceAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicDestructionAsset",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, source_asset),
            },
            FieldInfoData {
                name: "OutputPipeResults",
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionOutputPipeResult-Array",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, output_pipe_results),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, enabled),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, external_time),
            },
            FieldInfoData {
                name: "StartPaused",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, start_paused),
            },
            FieldInfoData {
                name: "ActivePlaybackSequence",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, active_playback_sequence),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, auto_start),
            },
            FieldInfoData {
                name: "AutoPause",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, auto_pause),
            },
            FieldInfoData {
                name: "PlaybackSequences",
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
}


pub static CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionControllerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionControllerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionPlaybackSequence {
    pub segment_table: Vec<CinematicDestructionBakedSegmentGroupOrder>,
}

pub trait CinematicDestructionPlaybackSequenceTrait: TypeObject {
    fn segment_table(&self) -> &Vec<CinematicDestructionBakedSegmentGroupOrder>;
}

impl CinematicDestructionPlaybackSequenceTrait for CinematicDestructionPlaybackSequence {
    fn segment_table(&self) -> &Vec<CinematicDestructionBakedSegmentGroupOrder> {
        &self.segment_table
    }
}

pub static CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPlaybackSequence",
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionPlaybackSequence as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SegmentTable",
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
}


pub static CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPlaybackSequence-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionPlaybackSequence"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionBakedSegmentGroupOrder {
    pub segment: u32,
    pub next_segment: u32,
}

pub trait CinematicDestructionBakedSegmentGroupOrderTrait: TypeObject {
    fn segment(&self) -> &u32;
    fn next_segment(&self) -> &u32;
}

impl CinematicDestructionBakedSegmentGroupOrderTrait for CinematicDestructionBakedSegmentGroupOrder {
    fn segment(&self) -> &u32 {
        &self.segment
    }
    fn next_segment(&self) -> &u32 {
        &self.next_segment
    }
}

pub static CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionBakedSegmentGroupOrder",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionBakedSegmentGroupOrder as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Segment",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CinematicDestructionBakedSegmentGroupOrder, segment),
            },
            FieldInfoData {
                name: "NextSegment",
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
}


pub static CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionBakedSegmentGroupOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionBakedSegmentGroupOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionSegmentGroupOrder {
    pub segment_group: String,
    pub next_segment_group: String,
}

pub trait CinematicDestructionSegmentGroupOrderTrait: TypeObject {
    fn segment_group(&self) -> &String;
    fn next_segment_group(&self) -> &String;
}

impl CinematicDestructionSegmentGroupOrderTrait for CinematicDestructionSegmentGroupOrder {
    fn segment_group(&self) -> &String {
        &self.segment_group
    }
    fn next_segment_group(&self) -> &String {
        &self.next_segment_group
    }
}

pub static CINEMATICDESTRUCTIONSEGMENTGROUPORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionSegmentGroupOrder",
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionSegmentGroupOrder as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SegmentGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CinematicDestructionSegmentGroupOrder, segment_group),
            },
            FieldInfoData {
                name: "NextSegmentGroup",
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
}


pub static CINEMATICDESTRUCTIONSEGMENTGROUPORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionSegmentGroupOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionSegmentGroupOrder"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionEmitterOutputPipeResult {
    pub _glacier_base: CinematicDestructionOutputPipeResult,
    pub attributes: Vec<CinematicDestructionEmitterOutputPipeAttribute>,
}

pub trait CinematicDestructionEmitterOutputPipeResultTrait: CinematicDestructionOutputPipeResultTrait {
    fn attributes(&self) -> &Vec<CinematicDestructionEmitterOutputPipeAttribute>;
}

impl CinematicDestructionEmitterOutputPipeResultTrait for CinematicDestructionEmitterOutputPipeResult {
    fn attributes(&self) -> &Vec<CinematicDestructionEmitterOutputPipeAttribute> {
        &self.attributes
    }
}

impl CinematicDestructionOutputPipeResultTrait for CinematicDestructionEmitterOutputPipeResult {
}

impl super::core::DataContainerTrait for CinematicDestructionEmitterOutputPipeResult {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeResult",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionEmitterOutputPipeResult as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Attributes",
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
}


pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeResult-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeResult"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionEmitterOutputPipeAttribute {
    pub effect_param: Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>,
    pub attribute_code: u64,
}

pub trait CinematicDestructionEmitterOutputPipeAttributeTrait: TypeObject {
    fn effect_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>>;
    fn attribute_code(&self) -> &u64;
}

impl CinematicDestructionEmitterOutputPipeAttributeTrait for CinematicDestructionEmitterOutputPipeAttribute {
    fn effect_param(&self) -> &Option<Arc<Mutex<dyn super::effect_base::EffectParameterTrait>>> {
        &self.effect_param
    }
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeAttribute",
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionEmitterOutputPipeAttribute as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EffectParam",
                flags: MemberInfoFlags::new(0),
                field_type: "EffectParameter",
                rust_offset: offset_of!(CinematicDestructionEmitterOutputPipeAttribute, effect_param),
            },
            FieldInfoData {
                name: "AttributeCode",
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
}


pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for CinematicDestructionEmitterOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionEmitterOutputPipeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionEmitterOutputPipeEntityData as Default>::default())),
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
}


pub static CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionAutoMeshOutputPipeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshOutputPipeEntityData as Default>::default())),
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
}


pub static CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for CinematicDestructionManualMeshOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionManualMeshOutputPipeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionManualMeshOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionManualMeshOutputPipeEntityData as Default>::default())),
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
}


pub static CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionManualMeshOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionManualMeshOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for CinematicDestructionMeshOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionMeshOutputPipeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionMeshOutputPipeEntityData as Default>::default())),
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
}


pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionMeshOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionMeshOutputPipeResult {
    pub _glacier_base: CinematicDestructionOutputPipeResult,
    pub vertex_attributes: Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute>,
    pub material_attributes: Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute>,
    pub texture_attributes: Vec<CinematicDestructionAutoMeshGeneratedTextureAttribute>,
    pub replace_index_buffer: bool,
    pub index_buffer: CinematicDestructionAutoMeshGeneratedIndexBuffer,
}

pub trait CinematicDestructionMeshOutputPipeResultTrait: CinematicDestructionOutputPipeResultTrait {
    fn vertex_attributes(&self) -> &Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute>;
    fn material_attributes(&self) -> &Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute>;
    fn texture_attributes(&self) -> &Vec<CinematicDestructionAutoMeshGeneratedTextureAttribute>;
    fn replace_index_buffer(&self) -> &bool;
    fn index_buffer(&self) -> &CinematicDestructionAutoMeshGeneratedIndexBuffer;
}

impl CinematicDestructionMeshOutputPipeResultTrait for CinematicDestructionMeshOutputPipeResult {
    fn vertex_attributes(&self) -> &Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute> {
        &self.vertex_attributes
    }
    fn material_attributes(&self) -> &Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute> {
        &self.material_attributes
    }
    fn texture_attributes(&self) -> &Vec<CinematicDestructionAutoMeshGeneratedTextureAttribute> {
        &self.texture_attributes
    }
    fn replace_index_buffer(&self) -> &bool {
        &self.replace_index_buffer
    }
    fn index_buffer(&self) -> &CinematicDestructionAutoMeshGeneratedIndexBuffer {
        &self.index_buffer
    }
}

impl CinematicDestructionOutputPipeResultTrait for CinematicDestructionMeshOutputPipeResult {
}

impl super::core::DataContainerTrait for CinematicDestructionMeshOutputPipeResult {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeResult",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionMeshOutputPipeResult as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "VertexAttributes",
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionAutoMeshGeneratedVertexAttribute-Array",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, vertex_attributes),
            },
            FieldInfoData {
                name: "MaterialAttributes",
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionAutoMeshGeneratedVertexAttribute-Array",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, material_attributes),
            },
            FieldInfoData {
                name: "TextureAttributes",
                flags: MemberInfoFlags::new(144),
                field_type: "CinematicDestructionAutoMeshGeneratedTextureAttribute-Array",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, texture_attributes),
            },
            FieldInfoData {
                name: "ReplaceIndexBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, replace_index_buffer),
            },
            FieldInfoData {
                name: "IndexBuffer",
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
}


pub static CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeResult-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionMeshOutputPipeResult"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionAutoMeshGeneratedIndexBuffer {
    pub attribute_code: u64,
}

pub trait CinematicDestructionAutoMeshGeneratedIndexBufferTrait: TypeObject {
    fn attribute_code(&self) -> &u64;
}

impl CinematicDestructionAutoMeshGeneratedIndexBufferTrait for CinematicDestructionAutoMeshGeneratedIndexBuffer {
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedIndexBuffer",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshGeneratedIndexBuffer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AttributeCode",
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
}


pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedIndexBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedIndexBuffer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionAutoMeshGeneratedTextureAttribute {
    pub usage: CinematicDestructionTextureReplaceUsage,
    pub attribute_code: u64,
}

pub trait CinematicDestructionAutoMeshGeneratedTextureAttributeTrait: TypeObject {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage;
    fn attribute_code(&self) -> &u64;
}

impl CinematicDestructionAutoMeshGeneratedTextureAttributeTrait for CinematicDestructionAutoMeshGeneratedTextureAttribute {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage {
        &self.usage
    }
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedTextureAttribute",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshGeneratedTextureAttribute as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: "CinematicDestructionTextureReplaceUsage",
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedTextureAttribute, usage),
            },
            FieldInfoData {
                name: "AttributeCode",
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
}


pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedTextureAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedTextureAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionAutoMeshGeneratedVertexAttribute {
    pub usage: super::render::VertexElementUsage,
    pub attribute_code: u64,
}

pub trait CinematicDestructionAutoMeshGeneratedVertexAttributeTrait: TypeObject {
    fn usage(&self) -> &super::render::VertexElementUsage;
    fn attribute_code(&self) -> &u64;
}

impl CinematicDestructionAutoMeshGeneratedVertexAttributeTrait for CinematicDestructionAutoMeshGeneratedVertexAttribute {
    fn usage(&self) -> &super::render::VertexElementUsage {
        &self.usage
    }
    fn attribute_code(&self) -> &u64 {
        &self.attribute_code
    }
}

pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedVertexAttribute",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAutoMeshGeneratedVertexAttribute as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: "VertexElementUsage",
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedVertexAttribute, usage),
            },
            FieldInfoData {
                name: "AttributeCode",
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
}


pub static CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedVertexAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedVertexAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EFFECTDATAATTRIBUTECPUPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeCpuParameter",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectDataAttributeCpuParameter as Default>::default())),
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
}


pub static EFFECTDATAATTRIBUTECPUPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeCpuParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("EffectDataAttributeCpuParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectDataAttributeParameter {
    pub _glacier_base: super::core::DataContainer,
}

pub trait EffectDataAttributeParameterTrait: super::core::DataContainerTrait {
}

impl EffectDataAttributeParameterTrait for EffectDataAttributeParameter {
}

impl super::core::DataContainerTrait for EffectDataAttributeParameter {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeParameter",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectDataAttributeParameter as Default>::default())),
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
}


pub static EFFECTDATAATTRIBUTEPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("EffectDataAttributeParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshDataReplaceTextureAttribute {
    pub _glacier_base: MeshDataReplaceAttribute,
    pub usage: CinematicDestructionTextureReplaceUsage,
}

pub trait MeshDataReplaceTextureAttributeTrait: MeshDataReplaceAttributeTrait {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage;
}

impl MeshDataReplaceTextureAttributeTrait for MeshDataReplaceTextureAttribute {
    fn usage(&self) -> &CinematicDestructionTextureReplaceUsage {
        &self.usage
    }
}

impl MeshDataReplaceAttributeTrait for MeshDataReplaceTextureAttribute {
}

impl super::core::DataContainerTrait for MeshDataReplaceTextureAttribute {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MESHDATAREPLACETEXTUREATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceTextureAttribute",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHDATAREPLACEATTRIBUTE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDataReplaceTextureAttribute as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
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
}


pub static MESHDATAREPLACETEXTUREATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceTextureAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceTextureAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshDataReplaceVertexAttribute {
    pub _glacier_base: MeshDataReplaceAttribute,
    pub usage: super::render::VertexElementUsage,
}

pub trait MeshDataReplaceVertexAttributeTrait: MeshDataReplaceAttributeTrait {
    fn usage(&self) -> &super::render::VertexElementUsage;
}

impl MeshDataReplaceVertexAttributeTrait for MeshDataReplaceVertexAttribute {
    fn usage(&self) -> &super::render::VertexElementUsage {
        &self.usage
    }
}

impl MeshDataReplaceAttributeTrait for MeshDataReplaceVertexAttribute {
}

impl super::core::DataContainerTrait for MeshDataReplaceVertexAttribute {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MESHDATAREPLACEVERTEXATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceVertexAttribute",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHDATAREPLACEATTRIBUTE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDataReplaceVertexAttribute as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Usage",
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
}


pub static MESHDATAREPLACEVERTEXATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceVertexAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceVertexAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshDataReplaceAttribute {
    pub _glacier_base: super::core::DataContainer,
}

pub trait MeshDataReplaceAttributeTrait: super::core::DataContainerTrait {
}

impl MeshDataReplaceAttributeTrait for MeshDataReplaceAttribute {
}

impl super::core::DataContainerTrait for MeshDataReplaceAttribute {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MESHDATAREPLACEATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceAttribute",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshDataReplaceAttribute as Default>::default())),
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
}


pub static MESHDATAREPLACEATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::GameDataContainerTrait for CinematicDestructionOutputPipeEntityData {
}

impl super::core::DataContainerTrait for CinematicDestructionOutputPipeEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionOutputPipeEntityData as Default>::default())),
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
}


pub static CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionOutputPipeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionOutputPipeResult {
    pub _glacier_base: super::core::DataContainer,
}

pub trait CinematicDestructionOutputPipeResultTrait: super::core::DataContainerTrait {
}

impl CinematicDestructionOutputPipeResultTrait for CinematicDestructionOutputPipeResult {
}

impl super::core::DataContainerTrait for CinematicDestructionOutputPipeResult {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeResult",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionOutputPipeResult as Default>::default())),
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
}


pub static CINEMATICDESTRUCTIONOUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeResult-Array",
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
}


pub static CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionTextureReplaceUsage-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionTextureReplaceUsage"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CinematicDestructionAsset {
    pub _glacier_base: super::linear_media::LinearMediaAssetDesc,
}

pub trait CinematicDestructionAssetTrait: super::linear_media::LinearMediaAssetDescTrait {
}

impl CinematicDestructionAssetTrait for CinematicDestructionAsset {
}

impl super::linear_media::LinearMediaAssetDescTrait for CinematicDestructionAsset {
    fn resources(&self) -> &Vec<super::linear_media::LinearMediaRuntimeResource> {
        self._glacier_base.resources()
    }
}

impl super::core::AssetTrait for CinematicDestructionAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for CinematicDestructionAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CINEMATICDESTRUCTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAsset",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::linear_media::LINEARMEDIAASSETDESC_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CinematicDestructionAsset as Default>::default())),
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
}


pub static CINEMATICDESTRUCTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionPoolBufferEntity as Default>::default())),
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
}


pub static CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionPoolBufferEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionPoolBufferEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionMeshOutputPipeEntity as Default>::default())),
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
}


pub static CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionMeshOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionMeshOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionManualMeshOutputPipeEntity as Default>::default())),
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
}


pub static CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionManualMeshOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionManualMeshOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionAutoMeshOutputPipeEntity as Default>::default())),
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
}


pub static CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionAutoMeshOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionAutoMeshOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionEmitterOutputPipeEntity as Default>::default())),
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
}


pub static CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionEmitterOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionEmitterOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionControllerEntity as Default>::default())),
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
}


pub static CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCinematicDestructionOutputPipeEntity as Default>::default())),
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
}


pub static CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionOutputPipeEntity"),
    array_type: None,
    alignment: 8,
};


