use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionPoolBufferEntityData {
    pub cpu_pool_adjustment: i32,
    pub gpu_pool_adjustment: i32,
    pub enabled: bool,
}

pub const CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPoolBufferEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CpuPoolAdjustment",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, cpu_pool_adjustment),
            },
            FieldInfoData {
                name: "GpuPoolAdjustment",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, gpu_pool_adjustment),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionPoolBufferEntityData, enabled),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionPoolBufferEntityData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONPOOLBUFFERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPoolBufferEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionPoolBufferEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CinematicDestructionControllerEntityData {
    pub source_asset: CinematicDestructionAsset,
    pub output_pipe_results: Vec<CinematicDestructionOutputPipeResult>,
    pub enabled: bool,
    pub external_time: f32,
    pub start_paused: bool,
    pub active_playback_sequence: i32,
    pub auto_start: bool,
    pub auto_pause: bool,
    pub playback_sequences: Vec<CinematicDestructionPlaybackSequence>,
}

pub const CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionControllerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SourceAsset",
                flags: MemberInfoFlags::new(0),
                field_type: CINEMATICDESTRUCTIONASSET_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, source_asset),
            },
            FieldInfoData {
                name: "OutputPipeResults",
                flags: MemberInfoFlags::new(144),
                field_type: CINEMATICDESTRUCTIONOUTPUTPIPERESULT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, output_pipe_results),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, enabled),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, external_time),
            },
            FieldInfoData {
                name: "StartPaused",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, start_paused),
            },
            FieldInfoData {
                name: "ActivePlaybackSequence",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, active_playback_sequence),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, auto_start),
            },
            FieldInfoData {
                name: "AutoPause",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, auto_pause),
            },
            FieldInfoData {
                name: "PlaybackSequences",
                flags: MemberInfoFlags::new(144),
                field_type: CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionControllerEntityData, playback_sequences),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionControllerEntityData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONCONTROLLERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionControllerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionControllerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionPlaybackSequence {
    pub segment_table: Vec<CinematicDestructionBakedSegmentGroupOrder>,
}

pub const CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPlaybackSequence",
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SegmentTable",
                flags: MemberInfoFlags::new(144),
                field_type: CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionPlaybackSequence, segment_table),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionPlaybackSequence {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONPLAYBACKSEQUENCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionPlaybackSequence-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionPlaybackSequence-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionBakedSegmentGroupOrder {
    pub segment: u32,
    pub next_segment: u32,
}

pub const CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionBakedSegmentGroupOrder",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Segment",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionBakedSegmentGroupOrder, segment),
            },
            FieldInfoData {
                name: "NextSegment",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionBakedSegmentGroupOrder, next_segment),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CinematicDestructionBakedSegmentGroupOrder {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONBAKEDSEGMENTGROUPORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionBakedSegmentGroupOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionBakedSegmentGroupOrder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionSegmentGroupOrder {
    pub segment_group: String,
    pub next_segment_group: String,
}

pub const CINEMATICDESTRUCTIONSEGMENTGROUPORDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionSegmentGroupOrder",
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SegmentGroup",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionSegmentGroupOrder, segment_group),
            },
            FieldInfoData {
                name: "NextSegmentGroup",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionSegmentGroupOrder, next_segment_group),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONSEGMENTGROUPORDER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionSegmentGroupOrder {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONSEGMENTGROUPORDER_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONSEGMENTGROUPORDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionSegmentGroupOrder-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionSegmentGroupOrder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionEmitterOutputPipeResult {
    pub attributes: Vec<CinematicDestructionEmitterOutputPipeAttribute>,
}

pub const CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeResult",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionEmitterOutputPipeResult, attributes),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionEmitterOutputPipeResult {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONEMITTEROUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeResult-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeResult-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionEmitterOutputPipeAttribute {
    pub effect_param: super::effect_base::EffectParameter,
    pub attribute_code: u64,
}

pub const CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeAttribute",
    flags: MemberInfoFlags::new(73),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EffectParam",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTPARAMETER_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionEmitterOutputPipeAttribute, effect_param),
            },
            FieldInfoData {
                name: "AttributeCode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionEmitterOutputPipeAttribute, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionEmitterOutputPipeAttribute {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeAttribute-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionEmitterOutputPipeEntityData {
}

pub const CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionEmitterOutputPipeEntityData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionEmitterOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionEmitterOutputPipeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionAutoMeshOutputPipeEntityData {
}

pub const CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshOutputPipeEntityData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshOutputPipeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionManualMeshOutputPipeEntityData {
}

pub const CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionManualMeshOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionManualMeshOutputPipeEntityData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionManualMeshOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionManualMeshOutputPipeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionMeshOutputPipeEntityData {
}

pub const CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionMeshOutputPipeEntityData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionMeshOutputPipeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionMeshOutputPipeResult {
    pub vertex_attributes: Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute>,
    pub material_attributes: Vec<CinematicDestructionAutoMeshGeneratedVertexAttribute>,
    pub texture_attributes: Vec<CinematicDestructionAutoMeshGeneratedTextureAttribute>,
    pub replace_index_buffer: bool,
    pub index_buffer: CinematicDestructionAutoMeshGeneratedIndexBuffer,
}

pub const CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeResult",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VertexAttributes",
                flags: MemberInfoFlags::new(144),
                field_type: CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, vertex_attributes),
            },
            FieldInfoData {
                name: "MaterialAttributes",
                flags: MemberInfoFlags::new(144),
                field_type: CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, material_attributes),
            },
            FieldInfoData {
                name: "TextureAttributes",
                flags: MemberInfoFlags::new(144),
                field_type: CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, texture_attributes),
            },
            FieldInfoData {
                name: "ReplaceIndexBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, replace_index_buffer),
            },
            FieldInfoData {
                name: "IndexBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionMeshOutputPipeResult, index_buffer),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionMeshOutputPipeResult {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONMESHOUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionMeshOutputPipeResult-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionMeshOutputPipeResult-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionAutoMeshGeneratedIndexBuffer {
    pub attribute_code: u64,
}

pub const CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedIndexBuffer",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AttributeCode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedIndexBuffer, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshGeneratedIndexBuffer {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONAUTOMESHGENERATEDINDEXBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedIndexBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedIndexBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionAutoMeshGeneratedTextureAttribute {
    pub usage: CinematicDestructionTextureReplaceUsage,
    pub attribute_code: u64,
}

pub const CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedTextureAttribute",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedTextureAttribute, usage),
            },
            FieldInfoData {
                name: "AttributeCode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedTextureAttribute, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshGeneratedTextureAttribute {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONAUTOMESHGENERATEDTEXTUREATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedTextureAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedTextureAttribute-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionAutoMeshGeneratedVertexAttribute {
    pub usage: super::render::VertexElementUsage,
    pub attribute_code: u64,
}

pub const CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedVertexAttribute",
    flags: MemberInfoFlags::new(36937),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXELEMENTUSAGE_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedVertexAttribute, usage),
            },
            FieldInfoData {
                name: "AttributeCode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(CinematicDestructionAutoMeshGeneratedVertexAttribute, attribute_code),
            },
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAutoMeshGeneratedVertexAttribute {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONAUTOMESHGENERATEDVERTEXATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAutoMeshGeneratedVertexAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAutoMeshGeneratedVertexAttribute-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectDataAttributeCpuParameter {
}

pub const EFFECTDATAATTRIBUTECPUPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeCpuParameter",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EFFECTDATAATTRIBUTECPUPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectDataAttributeCpuParameter {
    fn type_info() -> &'static TypeInfo {
        EFFECTDATAATTRIBUTECPUPARAMETER_TYPE_INFO
    }
}


pub const EFFECTDATAATTRIBUTECPUPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeCpuParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("EffectDataAttributeCpuParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectDataAttributeParameter {
}

pub const EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeParameter",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EFFECTDATAATTRIBUTEPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EffectDataAttributeParameter {
    fn type_info() -> &'static TypeInfo {
        EFFECTDATAATTRIBUTEPARAMETER_TYPE_INFO
    }
}


pub const EFFECTDATAATTRIBUTEPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectDataAttributeParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("EffectDataAttributeParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshDataReplaceTextureAttribute {
    pub usage: CinematicDestructionTextureReplaceUsage,
}

pub const MESHDATAREPLACETEXTUREATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceTextureAttribute",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHDATAREPLACEATTRIBUTE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_TYPE_INFO,
                rust_offset: offset_of!(MeshDataReplaceTextureAttribute, usage),
            },
        ],
    }),
    array_type: Some(MESHDATAREPLACETEXTUREATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDataReplaceTextureAttribute {
    fn type_info() -> &'static TypeInfo {
        MESHDATAREPLACETEXTUREATTRIBUTE_TYPE_INFO
    }
}


pub const MESHDATAREPLACETEXTUREATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceTextureAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceTextureAttribute-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshDataReplaceVertexAttribute {
    pub usage: super::render::VertexElementUsage,
}

pub const MESHDATAREPLACEVERTEXATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceVertexAttribute",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHDATAREPLACEATTRIBUTE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXELEMENTUSAGE_TYPE_INFO,
                rust_offset: offset_of!(MeshDataReplaceVertexAttribute, usage),
            },
        ],
    }),
    array_type: Some(MESHDATAREPLACEVERTEXATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDataReplaceVertexAttribute {
    fn type_info() -> &'static TypeInfo {
        MESHDATAREPLACEVERTEXATTRIBUTE_TYPE_INFO
    }
}


pub const MESHDATAREPLACEVERTEXATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceVertexAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceVertexAttribute-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshDataReplaceAttribute {
}

pub const MESHDATAREPLACEATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceAttribute",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHDATAREPLACEATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshDataReplaceAttribute {
    fn type_info() -> &'static TypeInfo {
        MESHDATAREPLACEATTRIBUTE_TYPE_INFO
    }
}


pub const MESHDATAREPLACEATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshDataReplaceAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("MeshDataReplaceAttribute-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionOutputPipeEntityData {
}

pub const CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionOutputPipeEntityData {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONOUTPUTPIPEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionOutputPipeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionOutputPipeResult {
}

pub const CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeResult",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONOUTPUTPIPERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionOutputPipeResult {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONOUTPUTPIPERESULT_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONOUTPUTPIPERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionOutputPipeResult-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionOutputPipeResult-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CinematicDestructionTextureReplaceUsage {
    #[default]
    CinematicDestructionTextureReplaceUsage_Albedo = 0,
    CinematicDestructionTextureReplaceUsage_Normal = 1,
    CinematicDestructionTextureReplaceUsage_Texture0 = 2,
    CinematicDestructionTextureReplaceUsage_Texture1 = 3,
    CinematicDestructionTextureReplaceUsage_Texture2 = 4,
    CinematicDestructionTextureReplaceUsage_Texture3 = 5,
}

pub const CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionTextureReplaceUsage",
    flags: MemberInfoFlags::new(49429),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Enum,
    array_type: Some(CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CinematicDestructionTextureReplaceUsage {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONTEXTUREREPLACEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionTextureReplaceUsage-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionTextureReplaceUsage-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CinematicDestructionAsset {
}

pub const CINEMATICDESTRUCTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAsset",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINEARMEDIAASSETDESC_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CINEMATICDESTRUCTIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CinematicDestructionAsset {
    fn type_info() -> &'static TypeInfo {
        CINEMATICDESTRUCTIONASSET_TYPE_INFO
    }
}


pub const CINEMATICDESTRUCTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CinematicDestructionAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("CinematicDestructionAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCinematicDestructionPoolBufferEntity {
}

pub const CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionPoolBufferEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionPoolBufferEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_TYPE_INFO
    }
}


pub const CLIENTCINEMATICDESTRUCTIONPOOLBUFFERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionPoolBufferEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionPoolBufferEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCinematicDestructionMeshOutputPipeEntity {
}

pub const CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionMeshOutputPipeEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionMeshOutputPipeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO
    }
}


pub const CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionMeshOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionMeshOutputPipeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCinematicDestructionManualMeshOutputPipeEntity {
}

pub const CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionManualMeshOutputPipeEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionManualMeshOutputPipeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_TYPE_INFO
    }
}


pub const CLIENTCINEMATICDESTRUCTIONMANUALMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionManualMeshOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionManualMeshOutputPipeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCinematicDestructionAutoMeshOutputPipeEntity {
}

pub const CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionAutoMeshOutputPipeEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONMESHOUTPUTPIPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionAutoMeshOutputPipeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_TYPE_INFO
    }
}


pub const CLIENTCINEMATICDESTRUCTIONAUTOMESHOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionAutoMeshOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionAutoMeshOutputPipeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCinematicDestructionEmitterOutputPipeEntity {
}

pub const CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionEmitterOutputPipeEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionEmitterOutputPipeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_TYPE_INFO
    }
}


pub const CLIENTCINEMATICDESTRUCTIONEMITTEROUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionEmitterOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionEmitterOutputPipeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCinematicDestructionControllerEntity {
}

pub const CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionControllerEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_TYPE_INFO
    }
}


pub const CLIENTCINEMATICDESTRUCTIONCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionControllerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCinematicDestructionOutputPipeEntity {
}

pub const CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionOutputPipeEntity",
    flags: MemberInfoFlags::new(101),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCinematicDestructionOutputPipeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_TYPE_INFO
    }
}


pub const CLIENTCINEMATICDESTRUCTIONOUTPUTPIPEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCinematicDestructionOutputPipeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LMSCinematicDestruction",
    data: TypeInfoData::Array("ClientCinematicDestructionOutputPipeEntity-Array"),
    array_type: None,
    alignment: 8,
};


