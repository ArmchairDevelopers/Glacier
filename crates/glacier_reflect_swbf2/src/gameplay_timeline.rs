use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct PoseTrackData {
    pub _glacier_base: ANTLayerData,
    pub keyframes: Vec<Option<Arc<Mutex<dyn PoseTrackKeyframeTrait>>>>,
}

pub trait PoseTrackDataTrait: ANTLayerDataTrait {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn PoseTrackKeyframeTrait>>>>;
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn PoseTrackKeyframeTrait>>>>;
}

impl PoseTrackDataTrait for PoseTrackData {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn PoseTrackKeyframeTrait>>>> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn PoseTrackKeyframeTrait>>>> {
        &mut self.keyframes
    }
}

impl ANTLayerDataTrait for PoseTrackData {
    fn blend_type(&self) -> &super::gameplay_sim::ANTLayerBlendType {
        self._glacier_base.blend_type()
    }
    fn blend_type_mut(&mut self) -> &mut super::gameplay_sim::ANTLayerBlendType {
        self._glacier_base.blend_type_mut()
    }
}

impl super::timeline::TimelineTrackDataTrait for PoseTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for PoseTrackData {
}

impl super::core::DataBusPeerTrait for PoseTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PoseTrackData {
}

impl super::core::DataContainerTrait for PoseTrackData {
}

pub static POSETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PoseTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "PoseTrackKeyframe-Array",
                rust_offset: offset_of!(PoseTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(POSETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PoseTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        POSETRACKDATA_TYPE_INFO
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


pub static POSETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PoseTrackKeyframe {
    pub _glacier_base: super::timeline::TimelineKeyframeData,
    pub time: f32,
    pub transition_to: Option<Arc<Mutex<dyn PoseDefinitionTrait>>>,
    pub duration_override: f32,
    pub transition_override: Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>,
}

pub trait PoseTrackKeyframeTrait: super::timeline::TimelineKeyframeDataTrait {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn transition_to(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>>;
    fn transition_to_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>>;
    fn duration_override(&self) -> &f32;
    fn duration_override_mut(&mut self) -> &mut f32;
    fn transition_override(&self) -> &Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>;
    fn transition_override_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>;
}

impl PoseTrackKeyframeTrait for PoseTrackKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn transition_to(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        &self.transition_to
    }
    fn transition_to_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        &mut self.transition_to
    }
    fn duration_override(&self) -> &f32 {
        &self.duration_override
    }
    fn duration_override_mut(&mut self) -> &mut f32 {
        &mut self.duration_override
    }
    fn transition_override(&self) -> &Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>> {
        &self.transition_override
    }
    fn transition_override_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>> {
        &mut self.transition_override
    }
}

impl super::timeline::TimelineKeyframeDataTrait for PoseTrackKeyframe {
}

impl super::core::DataContainerTrait for PoseTrackKeyframe {
}

pub static POSETRACKKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINEKEYFRAMEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PoseTrackKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PoseTrackKeyframe, time),
            },
            FieldInfoData {
                name: "TransitionTo",
                flags: MemberInfoFlags::new(0),
                field_type: "PoseDefinition",
                rust_offset: offset_of!(PoseTrackKeyframe, transition_to),
            },
            FieldInfoData {
                name: "DurationOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PoseTrackKeyframe, duration_override),
            },
            FieldInfoData {
                name: "TransitionOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "PoseTransitionBase",
                rust_offset: offset_of!(PoseTrackKeyframe, transition_override),
            },
        ],
    }),
    array_type: Some(POSETRACKKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PoseTrackKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        POSETRACKKEYFRAME_TYPE_INFO
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


pub static POSETRACKKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTrackKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseTrackKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CutPoseTransition {
    pub _glacier_base: PoseTransitionBase,
}

pub trait CutPoseTransitionTrait: PoseTransitionBaseTrait {
}

impl CutPoseTransitionTrait for CutPoseTransition {
}

impl PoseTransitionBaseTrait for CutPoseTransition {
    fn transition_to(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        self._glacier_base.transition_to()
    }
    fn transition_to_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        self._glacier_base.transition_to_mut()
    }
}

impl super::core::DataContainerTrait for CutPoseTransition {
}

pub static CUTPOSETRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CutPoseTransition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(POSETRANSITIONBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CutPoseTransition as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CUTPOSETRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CutPoseTransition {
    fn type_info(&self) -> &'static TypeInfo {
        CUTPOSETRANSITION_TYPE_INFO
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


pub static CUTPOSETRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CutPoseTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("CutPoseTransition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlendedPoseTransition {
    pub _glacier_base: PoseTransitionBase,
    pub blend_time: f32,
}

pub trait BlendedPoseTransitionTrait: PoseTransitionBaseTrait {
    fn blend_time(&self) -> &f32;
    fn blend_time_mut(&mut self) -> &mut f32;
}

impl BlendedPoseTransitionTrait for BlendedPoseTransition {
    fn blend_time(&self) -> &f32 {
        &self.blend_time
    }
    fn blend_time_mut(&mut self) -> &mut f32 {
        &mut self.blend_time
    }
}

impl PoseTransitionBaseTrait for BlendedPoseTransition {
    fn transition_to(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        self._glacier_base.transition_to()
    }
    fn transition_to_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        self._glacier_base.transition_to_mut()
    }
}

impl super::core::DataContainerTrait for BlendedPoseTransition {
}

pub static BLENDEDPOSETRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlendedPoseTransition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(POSETRANSITIONBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlendedPoseTransition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BlendTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BlendedPoseTransition, blend_time),
            },
        ],
    }),
    array_type: Some(BLENDEDPOSETRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlendedPoseTransition {
    fn type_info(&self) -> &'static TypeInfo {
        BLENDEDPOSETRANSITION_TYPE_INFO
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


pub static BLENDEDPOSETRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlendedPoseTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("BlendedPoseTransition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AnimatedPoseTransition {
    pub _glacier_base: PoseTransitionBase,
    pub transition_animation: super::ant::AntRef,
    pub animation_blend_in_time: f32,
    pub animation_blend_out_time: f32,
    pub transition_animation_duration: f32,
}

pub trait AnimatedPoseTransitionTrait: PoseTransitionBaseTrait {
    fn transition_animation(&self) -> &super::ant::AntRef;
    fn transition_animation_mut(&mut self) -> &mut super::ant::AntRef;
    fn animation_blend_in_time(&self) -> &f32;
    fn animation_blend_in_time_mut(&mut self) -> &mut f32;
    fn animation_blend_out_time(&self) -> &f32;
    fn animation_blend_out_time_mut(&mut self) -> &mut f32;
    fn transition_animation_duration(&self) -> &f32;
    fn transition_animation_duration_mut(&mut self) -> &mut f32;
}

impl AnimatedPoseTransitionTrait for AnimatedPoseTransition {
    fn transition_animation(&self) -> &super::ant::AntRef {
        &self.transition_animation
    }
    fn transition_animation_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.transition_animation
    }
    fn animation_blend_in_time(&self) -> &f32 {
        &self.animation_blend_in_time
    }
    fn animation_blend_in_time_mut(&mut self) -> &mut f32 {
        &mut self.animation_blend_in_time
    }
    fn animation_blend_out_time(&self) -> &f32 {
        &self.animation_blend_out_time
    }
    fn animation_blend_out_time_mut(&mut self) -> &mut f32 {
        &mut self.animation_blend_out_time
    }
    fn transition_animation_duration(&self) -> &f32 {
        &self.transition_animation_duration
    }
    fn transition_animation_duration_mut(&mut self) -> &mut f32 {
        &mut self.transition_animation_duration
    }
}

impl PoseTransitionBaseTrait for AnimatedPoseTransition {
    fn transition_to(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        self._glacier_base.transition_to()
    }
    fn transition_to_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        self._glacier_base.transition_to_mut()
    }
}

impl super::core::DataContainerTrait for AnimatedPoseTransition {
}

pub static ANIMATEDPOSETRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatedPoseTransition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(POSETRANSITIONBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimatedPoseTransition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransitionAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(AnimatedPoseTransition, transition_animation),
            },
            FieldInfoData {
                name: "AnimationBlendInTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnimatedPoseTransition, animation_blend_in_time),
            },
            FieldInfoData {
                name: "AnimationBlendOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnimatedPoseTransition, animation_blend_out_time),
            },
            FieldInfoData {
                name: "TransitionAnimationDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AnimatedPoseTransition, transition_animation_duration),
            },
        ],
    }),
    array_type: Some(ANIMATEDPOSETRANSITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AnimatedPoseTransition {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATEDPOSETRANSITION_TYPE_INFO
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


pub static ANIMATEDPOSETRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimatedPoseTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("AnimatedPoseTransition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PoseTransitionBase {
    pub _glacier_base: super::core::DataContainer,
    pub transition_to: Option<Arc<Mutex<dyn PoseDefinitionTrait>>>,
}

pub trait PoseTransitionBaseTrait: super::core::DataContainerTrait {
    fn transition_to(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>>;
    fn transition_to_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>>;
}

impl PoseTransitionBaseTrait for PoseTransitionBase {
    fn transition_to(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        &self.transition_to
    }
    fn transition_to_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        &mut self.transition_to
    }
}

impl super::core::DataContainerTrait for PoseTransitionBase {
}

pub static POSETRANSITIONBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionBase",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PoseTransitionBase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransitionTo",
                flags: MemberInfoFlags::new(0),
                field_type: "PoseDefinition",
                rust_offset: offset_of!(PoseTransitionBase, transition_to),
            },
        ],
    }),
    array_type: Some(POSETRANSITIONBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PoseTransitionBase {
    fn type_info(&self) -> &'static TypeInfo {
        POSETRANSITIONBASE_TYPE_INFO
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


pub static POSETRANSITIONBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseTransitionBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseTransitionBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PoseDefinition {
    pub _glacier_base: super::core::DataContainer,
    pub animation: super::ant::AntRef,
    pub animation_duration: f32,
    pub transitions: Vec<Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>>,
}

pub trait PoseDefinitionTrait: super::core::DataContainerTrait {
    fn animation(&self) -> &super::ant::AntRef;
    fn animation_mut(&mut self) -> &mut super::ant::AntRef;
    fn animation_duration(&self) -> &f32;
    fn animation_duration_mut(&mut self) -> &mut f32;
    fn transitions(&self) -> &Vec<Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>>;
    fn transitions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>>;
}

impl PoseDefinitionTrait for PoseDefinition {
    fn animation(&self) -> &super::ant::AntRef {
        &self.animation
    }
    fn animation_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.animation
    }
    fn animation_duration(&self) -> &f32 {
        &self.animation_duration
    }
    fn animation_duration_mut(&mut self) -> &mut f32 {
        &mut self.animation_duration
    }
    fn transitions(&self) -> &Vec<Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>> {
        &self.transitions
    }
    fn transitions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn PoseTransitionBaseTrait>>>> {
        &mut self.transitions
    }
}

impl super::core::DataContainerTrait for PoseDefinition {
}

pub static POSEDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseDefinition",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PoseDefinition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Animation",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(PoseDefinition, animation),
            },
            FieldInfoData {
                name: "AnimationDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PoseDefinition, animation_duration),
            },
            FieldInfoData {
                name: "Transitions",
                flags: MemberInfoFlags::new(144),
                field_type: "PoseTransitionBase-Array",
                rust_offset: offset_of!(PoseDefinition, transitions),
            },
        ],
    }),
    array_type: Some(POSEDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PoseDefinition {
    fn type_info(&self) -> &'static TypeInfo {
        POSEDEFINITION_TYPE_INFO
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


pub static POSEDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PoseDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PoseDefinition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PosesConfiguration {
    pub _glacier_base: super::core::SystemSettings,
    pub poses_global_asset: Option<Arc<Mutex<dyn PosesGlobalAssetTrait>>>,
}

pub trait PosesConfigurationTrait: super::core::SystemSettingsTrait {
    fn poses_global_asset(&self) -> &Option<Arc<Mutex<dyn PosesGlobalAssetTrait>>>;
    fn poses_global_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PosesGlobalAssetTrait>>>;
}

impl PosesConfigurationTrait for PosesConfiguration {
    fn poses_global_asset(&self) -> &Option<Arc<Mutex<dyn PosesGlobalAssetTrait>>> {
        &self.poses_global_asset
    }
    fn poses_global_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PosesGlobalAssetTrait>>> {
        &mut self.poses_global_asset
    }
}

impl super::core::SystemSettingsTrait for PosesConfiguration {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for PosesConfiguration {
}

pub static POSESCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesConfiguration",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PosesConfiguration as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PosesGlobalAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "PosesGlobalAsset",
                rust_offset: offset_of!(PosesConfiguration, poses_global_asset),
            },
        ],
    }),
    array_type: Some(POSESCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PosesConfiguration {
    fn type_info(&self) -> &'static TypeInfo {
        POSESCONFIGURATION_TYPE_INFO
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


pub static POSESCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PosesConfiguration"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PosesGlobalAsset {
    pub _glacier_base: super::core::Asset,
    pub poses: Vec<Option<Arc<Mutex<dyn PoseDefinitionTrait>>>>,
    pub default_pose: Option<Arc<Mutex<dyn PoseDefinitionTrait>>>,
}

pub trait PosesGlobalAssetTrait: super::core::AssetTrait {
    fn poses(&self) -> &Vec<Option<Arc<Mutex<dyn PoseDefinitionTrait>>>>;
    fn poses_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn PoseDefinitionTrait>>>>;
    fn default_pose(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>>;
    fn default_pose_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>>;
}

impl PosesGlobalAssetTrait for PosesGlobalAsset {
    fn poses(&self) -> &Vec<Option<Arc<Mutex<dyn PoseDefinitionTrait>>>> {
        &self.poses
    }
    fn poses_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn PoseDefinitionTrait>>>> {
        &mut self.poses
    }
    fn default_pose(&self) -> &Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        &self.default_pose
    }
    fn default_pose_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PoseDefinitionTrait>>> {
        &mut self.default_pose
    }
}

impl super::core::AssetTrait for PosesGlobalAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PosesGlobalAsset {
}

pub static POSESGLOBALASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesGlobalAsset",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PosesGlobalAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Poses",
                flags: MemberInfoFlags::new(144),
                field_type: "PoseDefinition-Array",
                rust_offset: offset_of!(PosesGlobalAsset, poses),
            },
            FieldInfoData {
                name: "DefaultPose",
                flags: MemberInfoFlags::new(0),
                field_type: "PoseDefinition",
                rust_offset: offset_of!(PosesGlobalAsset, default_pose),
            },
        ],
    }),
    array_type: Some(POSESGLOBALASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PosesGlobalAsset {
    fn type_info(&self) -> &'static TypeInfo {
        POSESGLOBALASSET_TYPE_INFO
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


pub static POSESGLOBALASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PosesGlobalAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("PosesGlobalAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DefaultANTLayerData {
    pub _glacier_base: ANTLayerData,
    pub blend_mask_list: super::ant::AntRef,
    pub clip_track: Option<Arc<Mutex<dyn ANTClipKeyframeTrackDataTrait>>>,
    pub blend_track: Option<Arc<Mutex<dyn ANTBlendKeyframeTrackDataTrait>>>,
}

pub trait DefaultANTLayerDataTrait: ANTLayerDataTrait {
    fn blend_mask_list(&self) -> &super::ant::AntRef;
    fn blend_mask_list_mut(&mut self) -> &mut super::ant::AntRef;
    fn clip_track(&self) -> &Option<Arc<Mutex<dyn ANTClipKeyframeTrackDataTrait>>>;
    fn clip_track_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ANTClipKeyframeTrackDataTrait>>>;
    fn blend_track(&self) -> &Option<Arc<Mutex<dyn ANTBlendKeyframeTrackDataTrait>>>;
    fn blend_track_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ANTBlendKeyframeTrackDataTrait>>>;
}

impl DefaultANTLayerDataTrait for DefaultANTLayerData {
    fn blend_mask_list(&self) -> &super::ant::AntRef {
        &self.blend_mask_list
    }
    fn blend_mask_list_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.blend_mask_list
    }
    fn clip_track(&self) -> &Option<Arc<Mutex<dyn ANTClipKeyframeTrackDataTrait>>> {
        &self.clip_track
    }
    fn clip_track_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ANTClipKeyframeTrackDataTrait>>> {
        &mut self.clip_track
    }
    fn blend_track(&self) -> &Option<Arc<Mutex<dyn ANTBlendKeyframeTrackDataTrait>>> {
        &self.blend_track
    }
    fn blend_track_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ANTBlendKeyframeTrackDataTrait>>> {
        &mut self.blend_track
    }
}

impl ANTLayerDataTrait for DefaultANTLayerData {
    fn blend_type(&self) -> &super::gameplay_sim::ANTLayerBlendType {
        self._glacier_base.blend_type()
    }
    fn blend_type_mut(&mut self) -> &mut super::gameplay_sim::ANTLayerBlendType {
        self._glacier_base.blend_type_mut()
    }
}

impl super::timeline::TimelineTrackDataTrait for DefaultANTLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for DefaultANTLayerData {
}

impl super::core::DataBusPeerTrait for DefaultANTLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DefaultANTLayerData {
}

impl super::core::DataContainerTrait for DefaultANTLayerData {
}

pub static DEFAULTANTLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultANTLayerData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DefaultANTLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BlendMaskList",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(DefaultANTLayerData, blend_mask_list),
            },
            FieldInfoData {
                name: "ClipTrack",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTClipKeyframeTrackData",
                rust_offset: offset_of!(DefaultANTLayerData, clip_track),
            },
            FieldInfoData {
                name: "BlendTrack",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTBlendKeyframeTrackData",
                rust_offset: offset_of!(DefaultANTLayerData, blend_track),
            },
        ],
    }),
    array_type: Some(DEFAULTANTLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DefaultANTLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        DEFAULTANTLAYERDATA_TYPE_INFO
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


pub static DEFAULTANTLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DefaultANTLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("DefaultANTLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTClipKeyframeTrackData {
    pub _glacier_base: super::timeline::TimelineTrackData,
    pub keyframes: Vec<Option<Arc<Mutex<dyn ANTClipKeyframeTrait>>>>,
}

pub trait ANTClipKeyframeTrackDataTrait: super::timeline::TimelineTrackDataTrait {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn ANTClipKeyframeTrait>>>>;
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTClipKeyframeTrait>>>>;
}

impl ANTClipKeyframeTrackDataTrait for ANTClipKeyframeTrackData {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn ANTClipKeyframeTrait>>>> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTClipKeyframeTrait>>>> {
        &mut self.keyframes
    }
}

impl super::timeline::TimelineTrackDataTrait for ANTClipKeyframeTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ANTClipKeyframeTrackData {
}

impl super::core::DataBusPeerTrait for ANTClipKeyframeTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ANTClipKeyframeTrackData {
}

impl super::core::DataContainerTrait for ANTClipKeyframeTrackData {
}

pub static ANTCLIPKEYFRAMETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframeTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTClipKeyframeTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "ANTClipKeyframe-Array",
                rust_offset: offset_of!(ANTClipKeyframeTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(ANTCLIPKEYFRAMETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTClipKeyframeTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCLIPKEYFRAMETRACKDATA_TYPE_INFO
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


pub static ANTCLIPKEYFRAMETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframeTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTClipKeyframeTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTBlendKeyframeTrackData {
    pub _glacier_base: super::timeline::TimelineTrackData,
    pub keyframes: Vec<Option<Arc<Mutex<dyn ANTBlendKeyframeTrait>>>>,
}

pub trait ANTBlendKeyframeTrackDataTrait: super::timeline::TimelineTrackDataTrait {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn ANTBlendKeyframeTrait>>>>;
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTBlendKeyframeTrait>>>>;
}

impl ANTBlendKeyframeTrackDataTrait for ANTBlendKeyframeTrackData {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn ANTBlendKeyframeTrait>>>> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTBlendKeyframeTrait>>>> {
        &mut self.keyframes
    }
}

impl super::timeline::TimelineTrackDataTrait for ANTBlendKeyframeTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ANTBlendKeyframeTrackData {
}

impl super::core::DataBusPeerTrait for ANTBlendKeyframeTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ANTBlendKeyframeTrackData {
}

impl super::core::DataContainerTrait for ANTBlendKeyframeTrackData {
}

pub static ANTBLENDKEYFRAMETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframeTrackData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTBlendKeyframeTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "ANTBlendKeyframe-Array",
                rust_offset: offset_of!(ANTBlendKeyframeTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(ANTBLENDKEYFRAMETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTBlendKeyframeTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTBLENDKEYFRAMETRACKDATA_TYPE_INFO
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


pub static ANTBLENDKEYFRAMETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframeTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendKeyframeTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTBlendKeyframe {
    pub _glacier_base: super::timeline::TimelineKeyframeData,
    pub time: f32,
    pub length: f32,
    pub blend_curve_type: ANTBlendCurveType,
    pub blend_scale: f32,
    pub curve_data: Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>>,
}

pub trait ANTBlendKeyframeTrait: super::timeline::TimelineKeyframeDataTrait {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
    fn blend_curve_type(&self) -> &ANTBlendCurveType;
    fn blend_curve_type_mut(&mut self) -> &mut ANTBlendCurveType;
    fn blend_scale(&self) -> &f32;
    fn blend_scale_mut(&mut self) -> &mut f32;
    fn curve_data(&self) -> &Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>>;
    fn curve_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>>;
}

impl ANTBlendKeyframeTrait for ANTBlendKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
    fn blend_curve_type(&self) -> &ANTBlendCurveType {
        &self.blend_curve_type
    }
    fn blend_curve_type_mut(&mut self) -> &mut ANTBlendCurveType {
        &mut self.blend_curve_type
    }
    fn blend_scale(&self) -> &f32 {
        &self.blend_scale
    }
    fn blend_scale_mut(&mut self) -> &mut f32 {
        &mut self.blend_scale
    }
    fn curve_data(&self) -> &Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>> {
        &self.curve_data
    }
    fn curve_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>> {
        &mut self.curve_data
    }
}

impl super::timeline::TimelineKeyframeDataTrait for ANTBlendKeyframe {
}

impl super::core::DataContainerTrait for ANTBlendKeyframe {
}

pub static ANTBLENDKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINEKEYFRAMEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTBlendKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTBlendKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTBlendKeyframe, length),
            },
            FieldInfoData {
                name: "BlendCurveType",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTBlendCurveType",
                rust_offset: offset_of!(ANTBlendKeyframe, blend_curve_type),
            },
            FieldInfoData {
                name: "BlendScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTBlendKeyframe, blend_scale),
            },
            FieldInfoData {
                name: "CurveData",
                flags: MemberInfoFlags::new(0),
                field_type: "CurveData",
                rust_offset: offset_of!(ANTBlendKeyframe, curve_data),
            },
        ],
    }),
    array_type: Some(ANTBLENDKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTBlendKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        ANTBLENDKEYFRAME_TYPE_INFO
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


pub static ANTBLENDKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTClipKeyframe {
    pub _glacier_base: super::timeline::TimelineKeyframeData,
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

pub trait ANTClipKeyframeTrait: super::timeline::TimelineKeyframeDataTrait {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
    fn controller(&self) -> &super::ant::AntRef;
    fn controller_mut(&mut self) -> &mut super::ant::AntRef;
    fn clip_start_trim(&self) -> &f32;
    fn clip_start_trim_mut(&mut self) -> &mut f32;
    fn clip_end_trim(&self) -> &f32;
    fn clip_end_trim_mut(&mut self) -> &mut f32;
    fn clip_cycle_start_offset(&self) -> &f32;
    fn clip_cycle_start_offset_mut(&mut self) -> &mut f32;
    fn clip_time_scale(&self) -> &f32;
    fn clip_time_scale_mut(&mut self) -> &mut f32;
    fn clip_start_rule(&self) -> &super::gameplay_sim::ANTClipStartRule;
    fn clip_start_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipStartRule;
    fn clip_end_rule(&self) -> &super::gameplay_sim::ANTClipEndRule;
    fn clip_end_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipEndRule;
}

impl ANTClipKeyframeTrait for ANTClipKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
    fn controller(&self) -> &super::ant::AntRef {
        &self.controller
    }
    fn controller_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.controller
    }
    fn clip_start_trim(&self) -> &f32 {
        &self.clip_start_trim
    }
    fn clip_start_trim_mut(&mut self) -> &mut f32 {
        &mut self.clip_start_trim
    }
    fn clip_end_trim(&self) -> &f32 {
        &self.clip_end_trim
    }
    fn clip_end_trim_mut(&mut self) -> &mut f32 {
        &mut self.clip_end_trim
    }
    fn clip_cycle_start_offset(&self) -> &f32 {
        &self.clip_cycle_start_offset
    }
    fn clip_cycle_start_offset_mut(&mut self) -> &mut f32 {
        &mut self.clip_cycle_start_offset
    }
    fn clip_time_scale(&self) -> &f32 {
        &self.clip_time_scale
    }
    fn clip_time_scale_mut(&mut self) -> &mut f32 {
        &mut self.clip_time_scale
    }
    fn clip_start_rule(&self) -> &super::gameplay_sim::ANTClipStartRule {
        &self.clip_start_rule
    }
    fn clip_start_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipStartRule {
        &mut self.clip_start_rule
    }
    fn clip_end_rule(&self) -> &super::gameplay_sim::ANTClipEndRule {
        &self.clip_end_rule
    }
    fn clip_end_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipEndRule {
        &mut self.clip_end_rule
    }
}

impl super::timeline::TimelineKeyframeDataTrait for ANTClipKeyframe {
}

impl super::core::DataContainerTrait for ANTClipKeyframe {
}

pub static ANTCLIPKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINEKEYFRAMEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTClipKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTClipKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTClipKeyframe, length),
            },
            FieldInfoData {
                name: "Controller",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(ANTClipKeyframe, controller),
            },
            FieldInfoData {
                name: "ClipStartTrim",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTClipKeyframe, clip_start_trim),
            },
            FieldInfoData {
                name: "ClipEndTrim",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTClipKeyframe, clip_end_trim),
            },
            FieldInfoData {
                name: "ClipCycleStartOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTClipKeyframe, clip_cycle_start_offset),
            },
            FieldInfoData {
                name: "ClipTimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTClipKeyframe, clip_time_scale),
            },
            FieldInfoData {
                name: "ClipStartRule",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTClipStartRule",
                rust_offset: offset_of!(ANTClipKeyframe, clip_start_rule),
            },
            FieldInfoData {
                name: "ClipEndRule",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTClipEndRule",
                rust_offset: offset_of!(ANTClipKeyframe, clip_end_rule),
            },
        ],
    }),
    array_type: Some(ANTCLIPKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTClipKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCLIPKEYFRAME_TYPE_INFO
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


pub static ANTCLIPKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTClipKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTClipKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ANTBlendAttachment {
    #[default]
    ANTBlendAttachment_Both = 0,
    ANTBlendAttachment_Left = 1,
    ANTBlendAttachment_Right = 2,
}

pub static ANTBLENDATTACHMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendAttachment",
    flags: MemberInfoFlags::new(49429),
    module: "GameplayTimeline",
    data: TypeInfoData::Enum,
    array_type: Some(ANTBLENDATTACHMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTBlendAttachment {
    fn type_info(&self) -> &'static TypeInfo {
        ANTBLENDATTACHMENT_TYPE_INFO
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


pub static ANTBLENDATTACHMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendAttachment-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendAttachment"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static ANTBLENDCURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendCurveType",
    flags: MemberInfoFlags::new(49429),
    module: "GameplayTimeline",
    data: TypeInfoData::Enum,
    array_type: Some(ANTBLENDCURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ANTBlendCurveType {
    fn type_info(&self) -> &'static TypeInfo {
        ANTBLENDCURVETYPE_TYPE_INFO
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


pub static ANTBLENDCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBlendCurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTBlendCurveType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTEvaluatorData {
    pub _glacier_base: super::core::DataContainer,
    pub layer_tracks: Vec<Option<Arc<Mutex<dyn ANTLayerDataTrait>>>>,
    pub actor: super::ant::AntRef,
    pub bone_infos: Vec<BoneInfo>,
    pub use_default_pose_as_base: bool,
}

pub trait ANTEvaluatorDataTrait: super::core::DataContainerTrait {
    fn layer_tracks(&self) -> &Vec<Option<Arc<Mutex<dyn ANTLayerDataTrait>>>>;
    fn layer_tracks_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTLayerDataTrait>>>>;
    fn actor(&self) -> &super::ant::AntRef;
    fn actor_mut(&mut self) -> &mut super::ant::AntRef;
    fn bone_infos(&self) -> &Vec<BoneInfo>;
    fn bone_infos_mut(&mut self) -> &mut Vec<BoneInfo>;
    fn use_default_pose_as_base(&self) -> &bool;
    fn use_default_pose_as_base_mut(&mut self) -> &mut bool;
}

impl ANTEvaluatorDataTrait for ANTEvaluatorData {
    fn layer_tracks(&self) -> &Vec<Option<Arc<Mutex<dyn ANTLayerDataTrait>>>> {
        &self.layer_tracks
    }
    fn layer_tracks_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTLayerDataTrait>>>> {
        &mut self.layer_tracks
    }
    fn actor(&self) -> &super::ant::AntRef {
        &self.actor
    }
    fn actor_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.actor
    }
    fn bone_infos(&self) -> &Vec<BoneInfo> {
        &self.bone_infos
    }
    fn bone_infos_mut(&mut self) -> &mut Vec<BoneInfo> {
        &mut self.bone_infos
    }
    fn use_default_pose_as_base(&self) -> &bool {
        &self.use_default_pose_as_base
    }
    fn use_default_pose_as_base_mut(&mut self) -> &mut bool {
        &mut self.use_default_pose_as_base
    }
}

impl super::core::DataContainerTrait for ANTEvaluatorData {
}

pub static ANTEVALUATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvaluatorData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTEvaluatorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LayerTracks",
                flags: MemberInfoFlags::new(144),
                field_type: "ANTLayerData-Array",
                rust_offset: offset_of!(ANTEvaluatorData, layer_tracks),
            },
            FieldInfoData {
                name: "Actor",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(ANTEvaluatorData, actor),
            },
            FieldInfoData {
                name: "BoneInfos",
                flags: MemberInfoFlags::new(144),
                field_type: "BoneInfo-Array",
                rust_offset: offset_of!(ANTEvaluatorData, bone_infos),
            },
            FieldInfoData {
                name: "UseDefaultPoseAsBase",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ANTEvaluatorData, use_default_pose_as_base),
            },
        ],
    }),
    array_type: Some(ANTEVALUATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTEvaluatorData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTEVALUATORDATA_TYPE_INFO
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


pub static ANTEVALUATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvaluatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTEvaluatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoneInfo {
    pub bone_index: i32,
    pub bone_name_hash: u32,
    pub transform: super::core::LinearTransform,
}

pub trait BoneInfoTrait: TypeObject {
    fn bone_index(&self) -> &i32;
    fn bone_index_mut(&mut self) -> &mut i32;
    fn bone_name_hash(&self) -> &u32;
    fn bone_name_hash_mut(&mut self) -> &mut u32;
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl BoneInfoTrait for BoneInfo {
    fn bone_index(&self) -> &i32 {
        &self.bone_index
    }
    fn bone_index_mut(&mut self) -> &mut i32 {
        &mut self.bone_index
    }
    fn bone_name_hash(&self) -> &u32 {
        &self.bone_name_hash
    }
    fn bone_name_hash_mut(&mut self) -> &mut u32 {
        &mut self.bone_name_hash
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
}

pub static BONEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneInfo",
    flags: MemberInfoFlags::new(36937),
    module: "GameplayTimeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoneInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BoneInfo, bone_index),
            },
            FieldInfoData {
                name: "BoneNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BoneInfo, bone_name_hash),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(BoneInfo, transform),
            },
        ],
    }),
    array_type: Some(BONEINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoneInfo {
    fn type_info(&self) -> &'static TypeInfo {
        BONEINFO_TYPE_INFO
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


pub static BONEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("BoneInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTLayerData {
    pub _glacier_base: super::timeline::TimelineTrackData,
    pub blend_type: super::gameplay_sim::ANTLayerBlendType,
}

pub trait ANTLayerDataTrait: super::timeline::TimelineTrackDataTrait {
    fn blend_type(&self) -> &super::gameplay_sim::ANTLayerBlendType;
    fn blend_type_mut(&mut self) -> &mut super::gameplay_sim::ANTLayerBlendType;
}

impl ANTLayerDataTrait for ANTLayerData {
    fn blend_type(&self) -> &super::gameplay_sim::ANTLayerBlendType {
        &self.blend_type
    }
    fn blend_type_mut(&mut self) -> &mut super::gameplay_sim::ANTLayerBlendType {
        &mut self.blend_type
    }
}

impl super::timeline::TimelineTrackDataTrait for ANTLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ANTLayerData {
}

impl super::core::DataBusPeerTrait for ANTLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ANTLayerData {
}

impl super::core::DataContainerTrait for ANTLayerData {
}

pub static ANTLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BlendType",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTLayerBlendType",
                rust_offset: offset_of!(ANTLayerData, blend_type),
            },
        ],
    }),
    array_type: Some(ANTLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTLAYERDATA_TYPE_INFO
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


pub static ANTLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTControllerLayerData {
    pub _glacier_base: ANTLayerData,
    pub blend_mask_list: super::ant::AntRef,
    pub keyframes: Vec<Option<Arc<Mutex<dyn ANTControllerKeyframeTrait>>>>,
}

pub trait ANTControllerLayerDataTrait: ANTLayerDataTrait {
    fn blend_mask_list(&self) -> &super::ant::AntRef;
    fn blend_mask_list_mut(&mut self) -> &mut super::ant::AntRef;
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn ANTControllerKeyframeTrait>>>>;
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTControllerKeyframeTrait>>>>;
}

impl ANTControllerLayerDataTrait for ANTControllerLayerData {
    fn blend_mask_list(&self) -> &super::ant::AntRef {
        &self.blend_mask_list
    }
    fn blend_mask_list_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.blend_mask_list
    }
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn ANTControllerKeyframeTrait>>>> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ANTControllerKeyframeTrait>>>> {
        &mut self.keyframes
    }
}

impl ANTLayerDataTrait for ANTControllerLayerData {
    fn blend_type(&self) -> &super::gameplay_sim::ANTLayerBlendType {
        self._glacier_base.blend_type()
    }
    fn blend_type_mut(&mut self) -> &mut super::gameplay_sim::ANTLayerBlendType {
        self._glacier_base.blend_type_mut()
    }
}

impl super::timeline::TimelineTrackDataTrait for ANTControllerLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::timeline::TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ANTControllerLayerData {
}

impl super::core::DataBusPeerTrait for ANTControllerLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ANTControllerLayerData {
}

impl super::core::DataContainerTrait for ANTControllerLayerData {
}

pub static ANTCONTROLLERLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerLayerData",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ANTLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTControllerLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BlendMaskList",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(ANTControllerLayerData, blend_mask_list),
            },
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "ANTControllerKeyframe-Array",
                rust_offset: offset_of!(ANTControllerLayerData, keyframes),
            },
        ],
    }),
    array_type: Some(ANTCONTROLLERLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTControllerLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCONTROLLERLAYERDATA_TYPE_INFO
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


pub static ANTCONTROLLERLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTControllerLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTControllerKeyframe {
    pub _glacier_base: super::timeline::TimelineKeyframeData,
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
    pub curve_data: Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>>,
}

pub trait ANTControllerKeyframeTrait: super::timeline::TimelineKeyframeDataTrait {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
    fn controller(&self) -> &super::ant::AntRef;
    fn controller_mut(&mut self) -> &mut super::ant::AntRef;
    fn clip_start_trim(&self) -> &f32;
    fn clip_start_trim_mut(&mut self) -> &mut f32;
    fn clip_end_trim(&self) -> &f32;
    fn clip_end_trim_mut(&mut self) -> &mut f32;
    fn clip_cycle_start_offset(&self) -> &f32;
    fn clip_cycle_start_offset_mut(&mut self) -> &mut f32;
    fn clip_time_scale(&self) -> &f32;
    fn clip_time_scale_mut(&mut self) -> &mut f32;
    fn clip_start_rule(&self) -> &super::gameplay_sim::ANTClipStartRule;
    fn clip_start_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipStartRule;
    fn runtime_clip_end_rule(&self) -> &super::gameplay_sim::ANTClipEndRule;
    fn runtime_clip_end_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipEndRule;
    fn blend_in_time(&self) -> &f32;
    fn blend_in_time_mut(&mut self) -> &mut f32;
    fn blend_out_time(&self) -> &f32;
    fn blend_out_time_mut(&mut self) -> &mut f32;
    fn curve_data(&self) -> &Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>>;
    fn curve_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>>;
}

impl ANTControllerKeyframeTrait for ANTControllerKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
    fn controller(&self) -> &super::ant::AntRef {
        &self.controller
    }
    fn controller_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.controller
    }
    fn clip_start_trim(&self) -> &f32 {
        &self.clip_start_trim
    }
    fn clip_start_trim_mut(&mut self) -> &mut f32 {
        &mut self.clip_start_trim
    }
    fn clip_end_trim(&self) -> &f32 {
        &self.clip_end_trim
    }
    fn clip_end_trim_mut(&mut self) -> &mut f32 {
        &mut self.clip_end_trim
    }
    fn clip_cycle_start_offset(&self) -> &f32 {
        &self.clip_cycle_start_offset
    }
    fn clip_cycle_start_offset_mut(&mut self) -> &mut f32 {
        &mut self.clip_cycle_start_offset
    }
    fn clip_time_scale(&self) -> &f32 {
        &self.clip_time_scale
    }
    fn clip_time_scale_mut(&mut self) -> &mut f32 {
        &mut self.clip_time_scale
    }
    fn clip_start_rule(&self) -> &super::gameplay_sim::ANTClipStartRule {
        &self.clip_start_rule
    }
    fn clip_start_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipStartRule {
        &mut self.clip_start_rule
    }
    fn runtime_clip_end_rule(&self) -> &super::gameplay_sim::ANTClipEndRule {
        &self.runtime_clip_end_rule
    }
    fn runtime_clip_end_rule_mut(&mut self) -> &mut super::gameplay_sim::ANTClipEndRule {
        &mut self.runtime_clip_end_rule
    }
    fn blend_in_time(&self) -> &f32 {
        &self.blend_in_time
    }
    fn blend_in_time_mut(&mut self) -> &mut f32 {
        &mut self.blend_in_time
    }
    fn blend_out_time(&self) -> &f32 {
        &self.blend_out_time
    }
    fn blend_out_time_mut(&mut self) -> &mut f32 {
        &mut self.blend_out_time
    }
    fn curve_data(&self) -> &Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>> {
        &self.curve_data
    }
    fn curve_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::timeline::CurveDataTrait>>> {
        &mut self.curve_data
    }
}

impl super::timeline::TimelineKeyframeDataTrait for ANTControllerKeyframe {
}

impl super::core::DataContainerTrait for ANTControllerKeyframe {
}

pub static ANTCONTROLLERKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "GameplayTimeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINEKEYFRAMEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTControllerKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, length),
            },
            FieldInfoData {
                name: "Controller",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(ANTControllerKeyframe, controller),
            },
            FieldInfoData {
                name: "ClipStartTrim",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, clip_start_trim),
            },
            FieldInfoData {
                name: "ClipEndTrim",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, clip_end_trim),
            },
            FieldInfoData {
                name: "ClipCycleStartOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, clip_cycle_start_offset),
            },
            FieldInfoData {
                name: "ClipTimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, clip_time_scale),
            },
            FieldInfoData {
                name: "ClipStartRule",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTClipStartRule",
                rust_offset: offset_of!(ANTControllerKeyframe, clip_start_rule),
            },
            FieldInfoData {
                name: "RuntimeClipEndRule",
                flags: MemberInfoFlags::new(0),
                field_type: "ANTClipEndRule",
                rust_offset: offset_of!(ANTControllerKeyframe, runtime_clip_end_rule),
            },
            FieldInfoData {
                name: "BlendInTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, blend_in_time),
            },
            FieldInfoData {
                name: "BlendOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ANTControllerKeyframe, blend_out_time),
            },
            FieldInfoData {
                name: "CurveData",
                flags: MemberInfoFlags::new(0),
                field_type: "CurveData",
                rust_offset: offset_of!(ANTControllerKeyframe, curve_data),
            },
        ],
    }),
    array_type: Some(ANTCONTROLLERKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ANTControllerKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCONTROLLERKEYFRAME_TYPE_INFO
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


pub static ANTCONTROLLERKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControllerKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameplayTimeline",
    data: TypeInfoData::Array("ANTControllerKeyframe"),
    array_type: None,
    alignment: 8,
};


