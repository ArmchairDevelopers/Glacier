use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_gameplay_timeline_types(registry: &mut TypeRegistry) {
    registry.register_type(POSETRACKDATA_TYPE_INFO);
    registry.register_type(POSETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(POSETRACKKEYFRAME_TYPE_INFO);
    registry.register_type(POSETRACKKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(CUTPOSETRANSITION_TYPE_INFO);
    registry.register_type(CUTPOSETRANSITION_ARRAY_TYPE_INFO);
    registry.register_type(BLENDEDPOSETRANSITION_TYPE_INFO);
    registry.register_type(BLENDEDPOSETRANSITION_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATEDPOSETRANSITION_TYPE_INFO);
    registry.register_type(ANIMATEDPOSETRANSITION_ARRAY_TYPE_INFO);
    registry.register_type(POSETRANSITIONBASE_TYPE_INFO);
    registry.register_type(POSETRANSITIONBASE_ARRAY_TYPE_INFO);
    registry.register_type(POSEDEFINITION_TYPE_INFO);
    registry.register_type(POSEDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(POSESCONFIGURATION_TYPE_INFO);
    registry.register_type(POSESCONFIGURATION_ARRAY_TYPE_INFO);
    registry.register_type(POSESGLOBALASSET_TYPE_INFO);
    registry.register_type(POSESGLOBALASSET_ARRAY_TYPE_INFO);
    registry.register_type(DEFAULTANTLAYERDATA_TYPE_INFO);
    registry.register_type(DEFAULTANTLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTCLIPKEYFRAMETRACKDATA_TYPE_INFO);
    registry.register_type(ANTCLIPKEYFRAMETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTBLENDKEYFRAMETRACKDATA_TYPE_INFO);
    registry.register_type(ANTBLENDKEYFRAMETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTBLENDKEYFRAME_TYPE_INFO);
    registry.register_type(ANTBLENDKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(ANTCLIPKEYFRAME_TYPE_INFO);
    registry.register_type(ANTCLIPKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(ANTBLENDATTACHMENT_TYPE_INFO);
    registry.register_type(ANTBLENDATTACHMENT_ARRAY_TYPE_INFO);
    registry.register_type(ANTBLENDCURVETYPE_TYPE_INFO);
    registry.register_type(ANTBLENDCURVETYPE_ARRAY_TYPE_INFO);
    registry.register_type(ANTEVALUATORDATA_TYPE_INFO);
    registry.register_type(ANTEVALUATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(BONEINFO_TYPE_INFO);
    registry.register_type(BONEINFO_ARRAY_TYPE_INFO);
    registry.register_type(ANTLAYERDATA_TYPE_INFO);
    registry.register_type(ANTLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTCONTROLLERLAYERDATA_TYPE_INFO);
    registry.register_type(ANTCONTROLLERLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(ANTCONTROLLERKEYFRAME_TYPE_INFO);
    registry.register_type(ANTCONTROLLERKEYFRAME_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct PoseTrackData {
    pub keyframes: Vec<PoseTrackKeyframe>,
}

pub const POSETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTLAYERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: POSETRACKKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PoseTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(POSETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PoseTrackData {
    fn type_info() -> &'static TypeInfo {
        POSETRACKDATA_TYPE_INFO
    }
}


pub const POSETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PoseTrackKeyframe {
    pub time: f32,
    pub transition_to: PoseDefinition,
    pub duration_override: f32,
    pub transition_override: PoseTransitionBase,
}

pub const POSETRACKKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEKEYFRAMEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PoseTrackKeyframe, time),
            },
            FieldInfoData {
                name: "TransitionTo",
                flags: MemberInfoFlags::new(0),
                field_type: POSEDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(PoseTrackKeyframe, transition_to),
            },
            FieldInfoData {
                name: "DurationOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PoseTrackKeyframe, duration_override),
            },
            FieldInfoData {
                name: "TransitionOverride",
                flags: MemberInfoFlags::new(0),
                field_type: POSETRANSITIONBASE_TYPE_INFO,
                rust_offset: offset_of!(PoseTrackKeyframe, transition_override),
            },
        ],
    }),
    array_type: Some(POSETRACKKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PoseTrackKeyframe {
    fn type_info() -> &'static TypeInfo {
        POSETRACKKEYFRAME_TYPE_INFO
    }
}


pub const POSETRACKKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseTrackKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CutPoseTransition {
}

pub const CUTPOSETRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CutPoseTransition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(POSETRANSITIONBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CUTPOSETRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CutPoseTransition {
    fn type_info() -> &'static TypeInfo {
        CUTPOSETRANSITION_TYPE_INFO
    }
}


pub const CUTPOSETRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CutPoseTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("CutPoseTransition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BlendedPoseTransition {
    pub blend_time: f32,
}

pub const BLENDEDPOSETRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlendedPoseTransition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(POSETRANSITIONBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BlendTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BlendedPoseTransition, blend_time),
            },
        ],
    }),
    array_type: Some(BLENDEDPOSETRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlendedPoseTransition {
    fn type_info() -> &'static TypeInfo {
        BLENDEDPOSETRANSITION_TYPE_INFO
    }
}


pub const BLENDEDPOSETRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlendedPoseTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("BlendedPoseTransition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AnimatedPoseTransition {
    pub transition_animation: super::ant::AntRef,
    pub animation_blend_in_time: f32,
    pub animation_blend_out_time: f32,
    pub transition_animation_duration: f32,
}

pub const ANIMATEDPOSETRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatedPoseTransition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(POSETRANSITIONBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TransitionAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(AnimatedPoseTransition, transition_animation),
            },
            FieldInfoData {
                name: "AnimationBlendInTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnimatedPoseTransition, animation_blend_in_time),
            },
            FieldInfoData {
                name: "AnimationBlendOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnimatedPoseTransition, animation_blend_out_time),
            },
            FieldInfoData {
                name: "TransitionAnimationDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AnimatedPoseTransition, transition_animation_duration),
            },
        ],
    }),
    array_type: Some(ANIMATEDPOSETRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatedPoseTransition {
    fn type_info() -> &'static TypeInfo {
        ANIMATEDPOSETRANSITION_TYPE_INFO
    }
}


pub const ANIMATEDPOSETRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatedPoseTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("AnimatedPoseTransition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PoseTransitionBase {
    pub transition_to: PoseDefinition,
}

pub const POSETRANSITIONBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionBase",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TransitionTo",
                flags: MemberInfoFlags::new(0),
                field_type: POSEDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(PoseTransitionBase, transition_to),
            },
        ],
    }),
    array_type: Some(POSETRANSITIONBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PoseTransitionBase {
    fn type_info() -> &'static TypeInfo {
        POSETRANSITIONBASE_TYPE_INFO
    }
}


pub const POSETRANSITIONBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseTransitionBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PoseDefinition {
    pub animation: super::ant::AntRef,
    pub animation_duration: f32,
    pub transitions: Vec<PoseTransitionBase>,
}

pub const POSEDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseDefinition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Animation",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(PoseDefinition, animation),
            },
            FieldInfoData {
                name: "AnimationDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PoseDefinition, animation_duration),
            },
            FieldInfoData {
                name: "Transitions",
                flags: MemberInfoFlags::new(144),
                field_type: POSETRANSITIONBASE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PoseDefinition, transitions),
            },
        ],
    }),
    array_type: Some(POSEDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PoseDefinition {
    fn type_info() -> &'static TypeInfo {
        POSEDEFINITION_TYPE_INFO
    }
}


pub const POSEDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PosesConfiguration {
    pub poses_global_asset: PosesGlobalAsset,
}

pub const POSESCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesConfiguration",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PosesGlobalAsset",
                flags: MemberInfoFlags::new(0),
                field_type: POSESGLOBALASSET_TYPE_INFO,
                rust_offset: offset_of!(PosesConfiguration, poses_global_asset),
            },
        ],
    }),
    array_type: Some(POSESCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PosesConfiguration {
    fn type_info() -> &'static TypeInfo {
        POSESCONFIGURATION_TYPE_INFO
    }
}


pub const POSESCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PosesConfiguration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PosesGlobalAsset {
    pub poses: Vec<PoseDefinition>,
    pub default_pose: PoseDefinition,
}

pub const POSESGLOBALASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesGlobalAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Poses",
                flags: MemberInfoFlags::new(144),
                field_type: POSEDEFINITION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PosesGlobalAsset, poses),
            },
            FieldInfoData {
                name: "DefaultPose",
                flags: MemberInfoFlags::new(0),
                field_type: POSEDEFINITION_TYPE_INFO,
                rust_offset: offset_of!(PosesGlobalAsset, default_pose),
            },
        ],
    }),
    array_type: Some(POSESGLOBALASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PosesGlobalAsset {
    fn type_info() -> &'static TypeInfo {
        POSESGLOBALASSET_TYPE_INFO
    }
}


pub const POSESGLOBALASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesGlobalAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PosesGlobalAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DefaultANTLayerData {
    pub blend_mask_list: super::ant::AntRef,
    pub clip_track: ANTClipKeyframeTrackData,
    pub blend_track: ANTBlendKeyframeTrackData,
}

pub const DEFAULTANTLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultANTLayerData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTLAYERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BlendMaskList",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(DefaultANTLayerData, blend_mask_list),
            },
            FieldInfoData {
                name: "ClipTrack",
                flags: MemberInfoFlags::new(0),
                field_type: ANTCLIPKEYFRAMETRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(DefaultANTLayerData, clip_track),
            },
            FieldInfoData {
                name: "BlendTrack",
                flags: MemberInfoFlags::new(0),
                field_type: ANTBLENDKEYFRAMETRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(DefaultANTLayerData, blend_track),
            },
        ],
    }),
    array_type: Some(DEFAULTANTLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DefaultANTLayerData {
    fn type_info() -> &'static TypeInfo {
        DEFAULTANTLAYERDATA_TYPE_INFO
    }
}


pub const DEFAULTANTLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultANTLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("DefaultANTLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ANTClipKeyframeTrackData {
    pub keyframes: Vec<ANTClipKeyframe>,
}

pub const ANTCLIPKEYFRAMETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframeTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: ANTCLIPKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframeTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(ANTCLIPKEYFRAMETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTClipKeyframeTrackData {
    fn type_info() -> &'static TypeInfo {
        ANTCLIPKEYFRAMETRACKDATA_TYPE_INFO
    }
}


pub const ANTCLIPKEYFRAMETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframeTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTClipKeyframeTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ANTBlendKeyframeTrackData {
    pub keyframes: Vec<ANTBlendKeyframe>,
}

pub const ANTBLENDKEYFRAMETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframeTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: ANTBLENDKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ANTBlendKeyframeTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(ANTBLENDKEYFRAMETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTBlendKeyframeTrackData {
    fn type_info() -> &'static TypeInfo {
        ANTBLENDKEYFRAMETRACKDATA_TYPE_INFO
    }
}


pub const ANTBLENDKEYFRAMETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframeTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendKeyframeTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ANTBlendKeyframe {
    pub time: f32,
    pub length: f32,
    pub blend_curve_type: ANTBlendCurveType,
    pub blend_scale: f32,
    pub curve_data: super::timeline::CurveData,
}

pub const ANTBLENDKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEKEYFRAMEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTBlendKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTBlendKeyframe, length),
            },
            FieldInfoData {
                name: "BlendCurveType",
                flags: MemberInfoFlags::new(0),
                field_type: ANTBLENDCURVETYPE_TYPE_INFO,
                rust_offset: offset_of!(ANTBlendKeyframe, blend_curve_type),
            },
            FieldInfoData {
                name: "BlendScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTBlendKeyframe, blend_scale),
            },
            FieldInfoData {
                name: "CurveData",
                flags: MemberInfoFlags::new(0),
                field_type: CURVEDATA_TYPE_INFO,
                rust_offset: offset_of!(ANTBlendKeyframe, curve_data),
            },
        ],
    }),
    array_type: Some(ANTBLENDKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTBlendKeyframe {
    fn type_info() -> &'static TypeInfo {
        ANTBLENDKEYFRAME_TYPE_INFO
    }
}


pub const ANTBLENDKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ANTClipKeyframe {
    pub time: f32,
    pub length: f32,
    pub controller: super::ant::AntRef,
    pub clip_start_trim: f32,
    pub clip_end_trim: f32,
    pub clip_cycle_start_offset: f32,
    pub clip_time_scale: f32,
    pub clip_start_rule: super::gameplay_sim::ANTClipStartRule,
    pub clip_end_rule: super::gameplay_sim::ANTClipEndRule,
}

pub const ANTCLIPKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEKEYFRAMEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, length),
            },
            FieldInfoData {
                name: "Controller",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, controller),
            },
            FieldInfoData {
                name: "ClipStartTrim",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, clip_start_trim),
            },
            FieldInfoData {
                name: "ClipEndTrim",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, clip_end_trim),
            },
            FieldInfoData {
                name: "ClipCycleStartOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, clip_cycle_start_offset),
            },
            FieldInfoData {
                name: "ClipTimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, clip_time_scale),
            },
            FieldInfoData {
                name: "ClipStartRule",
                flags: MemberInfoFlags::new(0),
                field_type: ANTCLIPSTARTRULE_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, clip_start_rule),
            },
            FieldInfoData {
                name: "ClipEndRule",
                flags: MemberInfoFlags::new(0),
                field_type: ANTCLIPENDRULE_TYPE_INFO,
                rust_offset: offset_of!(ANTClipKeyframe, clip_end_rule),
            },
        ],
    }),
    array_type: Some(ANTCLIPKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTClipKeyframe {
    fn type_info() -> &'static TypeInfo {
        ANTCLIPKEYFRAME_TYPE_INFO
    }
}


pub const ANTCLIPKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTClipKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ANTBlendAttachment {
    #[default]
    ANTBlendAttachment_Both = 0,
    ANTBlendAttachment_Left = 1,
    ANTBlendAttachment_Right = 2,
}

pub const ANTBLENDATTACHMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendAttachment",
    flags: MemberInfoFlags::new(49429),
    module: "GameplayTimeline",
    data: TypeInfoData::Enum,
    array_type: Some(ANTBLENDATTACHMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTBlendAttachment {
    fn type_info() -> &'static TypeInfo {
        ANTBLENDATTACHMENT_TYPE_INFO
    }
}


pub const ANTBLENDATTACHMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendAttachment-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendAttachment-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ANTBlendCurveType {
    #[default]
    ANTBlendCurveType_SnapIn = 0,
    ANTBlendCurveType_SnapOut = 1,
    ANTBlendCurveType_LinearIn = 2,
    ANTBlendCurveType_LinearOut = 3,
    ANTBlendCurveType_EaseIn = 4,
    ANTBlendCurveType_EaseOut = 5,
    ANTBlendCurveType_Custom = 6,
}

pub const ANTBLENDCURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendCurveType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplayTimeline",
    data: TypeInfoData::Enum,
    array_type: Some(ANTBLENDCURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTBlendCurveType {
    fn type_info() -> &'static TypeInfo {
        ANTBLENDCURVETYPE_TYPE_INFO
    }
}


pub const ANTBLENDCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendCurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendCurveType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ANTEvaluatorData {
    pub layer_tracks: Vec<ANTLayerData>,
    pub actor: super::ant::AntRef,
    pub bone_infos: Vec<BoneInfo>,
    pub use_default_pose_as_base: bool,
}

pub const ANTEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LayerTracks",
                flags: MemberInfoFlags::new(144),
                field_type: ANTLAYERDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ANTEvaluatorData, layer_tracks),
            },
            FieldInfoData {
                name: "Actor",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(ANTEvaluatorData, actor),
            },
            FieldInfoData {
                name: "BoneInfos",
                flags: MemberInfoFlags::new(144),
                field_type: BONEINFO_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ANTEvaluatorData, bone_infos),
            },
            FieldInfoData {
                name: "UseDefaultPoseAsBase",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ANTEvaluatorData, use_default_pose_as_base),
            },
        ],
    }),
    array_type: Some(ANTEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTEvaluatorData {
    fn type_info() -> &'static TypeInfo {
        ANTEVALUATORDATA_TYPE_INFO
    }
}


pub const ANTEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTEvaluatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoneInfo {
    pub bone_index: i32,
    pub bone_name_hash: u32,
    pub transform: super::core::LinearTransform,
}

pub const BONEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneInfo",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayTimeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BoneInfo, bone_index),
            },
            FieldInfoData {
                name: "BoneNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BoneInfo, bone_name_hash),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(BoneInfo, transform),
            },
        ],
    }),
    array_type: Some(BONEINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoneInfo {
    fn type_info() -> &'static TypeInfo {
        BONEINFO_TYPE_INFO
    }
}


pub const BONEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("BoneInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTLayerData {
    pub blend_type: super::gameplay_sim::ANTLayerBlendType,
}

pub const ANTLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BlendType",
                flags: MemberInfoFlags::new(0),
                field_type: ANTLAYERBLENDTYPE_TYPE_INFO,
                rust_offset: offset_of!(ANTLayerData, blend_type),
            },
        ],
    }),
    array_type: Some(ANTLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTLayerData {
    fn type_info() -> &'static TypeInfo {
        ANTLAYERDATA_TYPE_INFO
    }
}


pub const ANTLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ANTControllerLayerData {
    pub blend_mask_list: super::ant::AntRef,
    pub keyframes: Vec<ANTControllerKeyframe>,
}

pub const ANTCONTROLLERLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerLayerData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTLAYERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BlendMaskList",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerLayerData, blend_mask_list),
            },
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: ANTCONTROLLERKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerLayerData, keyframes),
            },
        ],
    }),
    array_type: Some(ANTCONTROLLERLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTControllerLayerData {
    fn type_info() -> &'static TypeInfo {
        ANTCONTROLLERLAYERDATA_TYPE_INFO
    }
}


pub const ANTCONTROLLERLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTControllerLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ANTControllerKeyframe {
    pub time: f32,
    pub length: f32,
    pub controller: super::ant::AntRef,
    pub clip_start_trim: f32,
    pub clip_end_trim: f32,
    pub clip_cycle_start_offset: f32,
    pub clip_time_scale: f32,
    pub clip_start_rule: super::gameplay_sim::ANTClipStartRule,
    pub runtime_clip_end_rule: super::gameplay_sim::ANTClipEndRule,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub curve_data: super::timeline::CurveData,
}

pub const ANTCONTROLLERKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEKEYFRAMEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, length),
            },
            FieldInfoData {
                name: "Controller",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, controller),
            },
            FieldInfoData {
                name: "ClipStartTrim",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, clip_start_trim),
            },
            FieldInfoData {
                name: "ClipEndTrim",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, clip_end_trim),
            },
            FieldInfoData {
                name: "ClipCycleStartOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, clip_cycle_start_offset),
            },
            FieldInfoData {
                name: "ClipTimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, clip_time_scale),
            },
            FieldInfoData {
                name: "ClipStartRule",
                flags: MemberInfoFlags::new(0),
                field_type: ANTCLIPSTARTRULE_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, clip_start_rule),
            },
            FieldInfoData {
                name: "RuntimeClipEndRule",
                flags: MemberInfoFlags::new(0),
                field_type: ANTCLIPENDRULE_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, runtime_clip_end_rule),
            },
            FieldInfoData {
                name: "BlendInTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, blend_in_time),
            },
            FieldInfoData {
                name: "BlendOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, blend_out_time),
            },
            FieldInfoData {
                name: "CurveData",
                flags: MemberInfoFlags::new(0),
                field_type: CURVEDATA_TYPE_INFO,
                rust_offset: offset_of!(ANTControllerKeyframe, curve_data),
            },
        ],
    }),
    array_type: Some(ANTCONTROLLERKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTControllerKeyframe {
    fn type_info() -> &'static TypeInfo {
        ANTCONTROLLERKEYFRAME_TYPE_INFO
    }
}


pub const ANTCONTROLLERKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTControllerKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


