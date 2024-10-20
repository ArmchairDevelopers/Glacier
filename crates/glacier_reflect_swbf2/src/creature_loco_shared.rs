use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_creature_loco_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(CREATUREWAYPOINTNETSTATE_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINTNETSTATE_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRAALIGNINFO_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRAALIGNINFO_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRACREATURELOCO_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRACREATURELOCO_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONWAYPOINTDATA_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPAUSEWAYPOINTDATA_TYPE_INFO);
    registry.register_type(CREATUREPAUSEWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPAUSEDATA_TYPE_INFO);
    registry.register_type(CREATUREPAUSEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREMOVEWAYPOINTDATA_TYPE_INFO);
    registry.register_type(CREATUREMOVEWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESPEEDLEVEL_TYPE_INFO);
    registry.register_type(CREATURESPEEDLEVEL_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINTSSHAPEDATA_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOSETTINGS_TYPE_INFO);
    registry.register_type(CREATURELOCOSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PUSHSETTINGS_TYPE_INFO);
    registry.register_type(PUSHSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PUSHSETTINGSFILTER_TYPE_INFO);
    registry.register_type(PUSHSETTINGSFILTER_ARRAY_TYPE_INFO);
    registry.register_type(SIZESETTINGS_TYPE_INFO);
    registry.register_type(SIZESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONSSETTINGS_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONSSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONVENTPARAMETERS_TYPE_INFO);
    registry.register_type(WORLDEVENTACTIONVENTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO);
    registry.register_type(CREATURELOCOEXTERNALINFLUENCETYPE_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALMOVEMENTSTATESETTINGS_TYPE_INFO);
    registry.register_type(PROCEDURALMOVEMENTSTATESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOCOSTATESPEEDRANGE_TYPE_INFO);
    registry.register_type(LOCOSTATESPEEDRANGE_ARRAY_TYPE_INFO);
    registry.register_type(AVOIDANCESTEERINGSETTINGS_TYPE_INFO);
    registry.register_type(AVOIDANCESTEERINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(AVOIDANCEDATA_TYPE_INFO);
    registry.register_type(AVOIDANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(AIAVOIDANCEPLAYERBEHAVIOUR_TYPE_INFO);
    registry.register_type(AIAVOIDANCEPLAYERBEHAVIOUR_ARRAY_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGS_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGSDATA_TYPE_INFO);
    registry.register_type(CURVESTEERINGSETTINGSDATA_ARRAY_TYPE_INFO);
    registry.register_type(BASICSTEERINGSETTINGSDATA_TYPE_INFO);
    registry.register_type(BASICSTEERINGSETTINGSDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMMONCLIENTSETTINGS_TYPE_INFO);
    registry.register_type(COMMONCLIENTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PLAYANIMSETTINGS_TYPE_INFO);
    registry.register_type(PLAYANIMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TURNSETTINGS_TYPE_INFO);
    registry.register_type(TURNSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STOPSETTINGS_TYPE_INFO);
    registry.register_type(STOPSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STARTSETTINGS_TYPE_INFO);
    registry.register_type(STARTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(MOVECYCLESETTINGS_TYPE_INFO);
    registry.register_type(MOVECYCLESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(IDLESETTINGS_TYPE_INFO);
    registry.register_type(IDLESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOCOMOTIONSTATESETTINGS_TYPE_INFO);
    registry.register_type(LOCOMOTIONSTATESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOSERVERAUTHENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURELOCOSERVERAUTHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDINGS_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDINGS_ARRAY_TYPE_INFO);
    registry.register_type(EVENTREACTIONPARAMDATA_TYPE_INFO);
    registry.register_type(EVENTREACTIONPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREREACTIONBINDING_TYPE_INFO);
    registry.register_type(CREATUREREACTIONBINDING_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALMOTIONPARAMDATA_TYPE_INFO);
    registry.register_type(PROCEDURALMOTIONPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(COMMONCLIENTBINDINGSPARAMDATA_TYPE_INFO);
    registry.register_type(COMMONCLIENTBINDINGSPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(PLAYANIMPARAMDATA_TYPE_INFO);
    registry.register_type(PLAYANIMPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(TURNPARAMDATA_TYPE_INFO);
    registry.register_type(TURNPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(STOPPARAMDATA_TYPE_INFO);
    registry.register_type(STOPPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(STARTPARAMDATA_TYPE_INFO);
    registry.register_type(STARTPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVECYCLEPARAMDATA_TYPE_INFO);
    registry.register_type(MOVECYCLEPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(IDLEPARAMDATA_TYPE_INFO);
    registry.register_type(IDLEPARAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONBINDING_TYPE_INFO);
    registry.register_type(CREATUREPLAYANIMATIONBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURETURNBINDING_TYPE_INFO);
    registry.register_type(CREATURETURNBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESTOPBINDING_TYPE_INFO);
    registry.register_type(CREATURESTOPBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESTARTBINDING_TYPE_INFO);
    registry.register_type(CREATURESTARTBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREMISCBINDING_TYPE_INFO);
    registry.register_type(CREATUREMISCBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDING_TYPE_INFO);
    registry.register_type(CREATURELOCOBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREIDLEBINDING_TYPE_INFO);
    registry.register_type(CREATUREIDLEBINDING_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECOMMONBINDING_TYPE_INFO);
    registry.register_type(CREATURECOMMONBINDING_ARRAY_TYPE_INFO);
    registry.register_type(EARLYOUTTYPE_TYPE_INFO);
    registry.register_type(EARLYOUTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LOCOMOTIONPARAMBLOCK_TYPE_INFO);
    registry.register_type(LOCOMOTIONPARAMBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLIST_STARTTYPE_TYPE_INFO);
    registry.register_type(CL_WAYPOINTLIST_STARTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECOLLISIONGROUPDATA_TYPE_INFO);
    registry.register_type(CREATURECOLLISIONGROUPDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITYDATA_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITYDATA_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITYDATA_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureWaypointNetState {
    pub g_u_i_d: super::a_i_tools::AIWaypointGUID,
}

pub const CREATUREWAYPOINTNETSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointNetState",
    flags: MemberInfoFlags::new(73),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "GUID",
                flags: MemberInfoFlags::new(0),
                field_type: AIWAYPOINTGUID_TYPE_INFO,
                rust_offset: offset_of!(CreatureWaypointNetState, g_u_i_d),
            },
        ],
    }),
    array_type: Some(CREATUREWAYPOINTNETSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CreatureWaypointNetState {
    fn type_info() -> &'static TypeInfo {
        CREATUREWAYPOINTNETSTATE_TYPE_INFO
    }
}


pub const CREATUREWAYPOINTNETSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointNetState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureWaypointNetState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AIWaypointExtraAlignInfo {
    pub align_pos: super::core::Vec3,
    pub align_facing: f32,
}

pub const AIWAYPOINTEXTRAALIGNINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraAlignInfo",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AlignPos",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraAlignInfo, align_pos),
            },
            FieldInfoData {
                name: "AlignFacing",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraAlignInfo, align_facing),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRAALIGNINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AIWaypointExtraAlignInfo {
    fn type_info() -> &'static TypeInfo {
        AIWAYPOINTEXTRAALIGNINFO_TYPE_INFO
    }
}


pub const AIWAYPOINTEXTRAALIGNINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraAlignInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AIWaypointExtraAlignInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AIWaypointExtraCreatureLoco {
    pub desired_facing: f32,
    pub speed_level: i32,
    pub stop: bool,
    pub backwards: bool,
    pub force_height: bool,
}

pub const AIWAYPOINTEXTRACREATURELOCO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraCreatureLoco",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DesiredFacing",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, desired_facing),
            },
            FieldInfoData {
                name: "SpeedLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, speed_level),
            },
            FieldInfoData {
                name: "Stop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, stop),
            },
            FieldInfoData {
                name: "Backwards",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, backwards),
            },
            FieldInfoData {
                name: "ForceHeight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraCreatureLoco, force_height),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRACREATURELOCO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AIWaypointExtraCreatureLoco {
    fn type_info() -> &'static TypeInfo {
        AIWAYPOINTEXTRACREATURELOCO_TYPE_INFO
    }
}


pub const AIWAYPOINTEXTRACREATURELOCO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraCreatureLoco-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AIWaypointExtraCreatureLoco-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreaturePlayAnimationWaypointData {
    pub play_animation: super::game_shared::PlayAnimationData,
    pub stop_at_waypoint: bool,
    pub align_joint: String,
    pub align_transform: super::core::LinearTransform,
    pub enter_position: super::core::Vec3,
    pub exit_position: super::core::Vec3,
}

pub const CREATUREPLAYANIMATIONWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREMOVEWAYPOINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PlayAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: PLAYANIMATIONDATA_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, play_animation),
            },
            FieldInfoData {
                name: "StopAtWaypoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, stop_at_waypoint),
            },
            FieldInfoData {
                name: "AlignJoint",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, align_joint),
            },
            FieldInfoData {
                name: "AlignTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, align_transform),
            },
            FieldInfoData {
                name: "EnterPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, enter_position),
            },
            FieldInfoData {
                name: "ExitPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationWaypointData, exit_position),
            },
        ],
    }),
    array_type: Some(CREATUREPLAYANIMATIONWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CreaturePlayAnimationWaypointData {
    fn type_info() -> &'static TypeInfo {
        CREATUREPLAYANIMATIONWAYPOINTDATA_TYPE_INFO
    }
}


pub const CREATUREPLAYANIMATIONWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePlayAnimationWaypointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreaturePauseWaypointData {
    pub pause_settings_for_slow_speed: CreaturePauseData,
    pub pause_settings_for_fast_speed: CreaturePauseData,
}

pub const CREATUREPAUSEWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREMOVEWAYPOINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PauseSettingsForSlowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: CREATUREPAUSEDATA_TYPE_INFO,
                rust_offset: offset_of!(CreaturePauseWaypointData, pause_settings_for_slow_speed),
            },
            FieldInfoData {
                name: "PauseSettingsForFastSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: CREATUREPAUSEDATA_TYPE_INFO,
                rust_offset: offset_of!(CreaturePauseWaypointData, pause_settings_for_fast_speed),
            },
        ],
    }),
    array_type: Some(CREATUREPAUSEWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreaturePauseWaypointData {
    fn type_info() -> &'static TypeInfo {
        CREATUREPAUSEWAYPOINTDATA_TYPE_INFO
    }
}


pub const CREATUREPAUSEWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePauseWaypointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreaturePauseData {
    pub probability: f32,
    pub minimum_duration: f32,
    pub maximum_duration: f32,
}

pub const CREATUREPAUSEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Probability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CreaturePauseData, probability),
            },
            FieldInfoData {
                name: "MinimumDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CreaturePauseData, minimum_duration),
            },
            FieldInfoData {
                name: "MaximumDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CreaturePauseData, maximum_duration),
            },
        ],
    }),
    array_type: Some(CREATUREPAUSEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreaturePauseData {
    fn type_info() -> &'static TypeInfo {
        CREATUREPAUSEDATA_TYPE_INFO
    }
}


pub const CREATUREPAUSEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePauseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePauseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreatureMoveWaypointData {
    pub override_creature_angle: bool,
    pub world_angle: f32,
    pub radius: f32,
    pub speed_level: CreatureSpeedLevel,
    pub move_backward: bool,
    pub explicit_height: bool,
}

pub const CREATUREMOVEWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMoveWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WAYPOINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OverrideCreatureAngle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureMoveWaypointData, override_creature_angle),
            },
            FieldInfoData {
                name: "WorldAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CreatureMoveWaypointData, world_angle),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CreatureMoveWaypointData, radius),
            },
            FieldInfoData {
                name: "SpeedLevel",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURESPEEDLEVEL_TYPE_INFO,
                rust_offset: offset_of!(CreatureMoveWaypointData, speed_level),
            },
            FieldInfoData {
                name: "MoveBackward",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureMoveWaypointData, move_backward),
            },
            FieldInfoData {
                name: "Explicit_Height",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureMoveWaypointData, explicit_height),
            },
        ],
    }),
    array_type: Some(CREATUREMOVEWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureMoveWaypointData {
    fn type_info() -> &'static TypeInfo {
        CREATUREMOVEWAYPOINTDATA_TYPE_INFO
    }
}


pub const CREATUREMOVEWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMoveWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureMoveWaypointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CreatureSpeedLevel {
    #[default]
    CreatureSpeedLevel_Slow = 0,
    CreatureSpeedLevel_Fast = 2,
    CreatureSpeedLevel_NoChange = 3,
}

pub const CREATURESPEEDLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpeedLevel",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURESPEEDLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureSpeedLevel {
    fn type_info() -> &'static TypeInfo {
        CREATURESPEEDLEVEL_TYPE_INFO
    }
}


pub const CREATURESPEEDLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpeedLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureSpeedLevel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreatureWaypointsShapeData {
}

pub const CREATUREWAYPOINTSSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointsShapeData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WAYPOINTSSHAPEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureWaypointsShapeData {
    fn type_info() -> &'static TypeInfo {
        CREATUREWAYPOINTSSHAPEDATA_TYPE_INFO
    }
}


pub const CREATUREWAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypointsShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureWaypointsShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureLocoSettings {
    pub state_settingss: Vec<LocomotionStateSettings>,
}

pub const CREATURELOCOSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StateSettingss",
                flags: MemberInfoFlags::new(144),
                field_type: LOCOMOTIONSTATESETTINGS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoSettings, state_settingss),
            },
        ],
    }),
    array_type: Some(CREATURELOCOSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureLocoSettings {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOSETTINGS_TYPE_INFO
    }
}


pub const CREATURELOCOSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PushSettings {
    pub can_be_pushed_by: PushSettingsFilter,
    pub is_pushed_by_player: bool,
}

pub const PUSHSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CanBePushedBy",
                flags: MemberInfoFlags::new(0),
                field_type: PUSHSETTINGSFILTER_TYPE_INFO,
                rust_offset: offset_of!(PushSettings, can_be_pushed_by),
            },
            FieldInfoData {
                name: "IsPushedByPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PushSettings, is_pushed_by_player),
            },
        ],
    }),
    array_type: Some(PUSHSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PushSettings {
    fn type_info() -> &'static TypeInfo {
        PUSHSETTINGS_TYPE_INFO
    }
}


pub const PUSHSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PushSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PushSettingsFilter {
    #[default]
    PushFilter_Everyone = 0,
    PushFilter_LargerThanMe = 1,
    PushFilter_EqualToOrLargerThanMe = 2,
    PushFilter_NoOne = 3,
}

pub const PUSHSETTINGSFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettingsFilter",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(PUSHSETTINGSFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PushSettingsFilter {
    fn type_info() -> &'static TypeInfo {
        PUSHSETTINGSFILTER_TYPE_INFO
    }
}


pub const PUSHSETTINGSFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PushSettingsFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PushSettingsFilter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SizeSettings {
    pub radius: f32,
    pub length: f32,
}

pub const SIZESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SizeSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SizeSettings, radius),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SizeSettings, length),
            },
        ],
    }),
    array_type: Some(SIZESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SizeSettings {
    fn type_info() -> &'static TypeInfo {
        SIZESETTINGS_TYPE_INFO
    }
}


pub const SIZESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SizeSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("SizeSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WorldEventActionsSettings {
    pub event_actions: Vec<WorldEventActionventParameters>,
}

pub const WORLDEVENTACTIONSSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionsSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EventActions",
                flags: MemberInfoFlags::new(144),
                field_type: WORLDEVENTACTIONVENTPARAMETERS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionsSettings, event_actions),
            },
        ],
    }),
    array_type: Some(WORLDEVENTACTIONSSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldEventActionsSettings {
    fn type_info() -> &'static TypeInfo {
        WORLDEVENTACTIONSSETTINGS_TYPE_INFO
    }
}


pub const WORLDEVENTACTIONSSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionsSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("WorldEventActionsSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WorldEventActionventParameters {
    pub event_type: CreatureLocoExternalInfluenceType,
    pub consideration_range: f32,
    pub minimum_number: i32,
    pub time_window: f32,
    pub probability_of_action: f32,
    pub alignment_rate: f32,
    pub minimum_impact_radius: f32,
    pub fake_physiscs: bool,
    pub fake_mass: f32,
    pub stop_delay: f32,
    pub cooldown_time: f32,
    pub action_type: CreatureLocoExternalInfluenceReactionType,
    pub action_alignment: CreatureLocoExternalInfluenceReactionAlignment,
}

pub const WORLDEVENTACTIONVENTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionventParameters",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EventType",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, event_type),
            },
            FieldInfoData {
                name: "ConsiderationRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, consideration_range),
            },
            FieldInfoData {
                name: "MinimumNumber",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, minimum_number),
            },
            FieldInfoData {
                name: "TimeWindow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, time_window),
            },
            FieldInfoData {
                name: "ProbabilityOfAction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, probability_of_action),
            },
            FieldInfoData {
                name: "AlignmentRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, alignment_rate),
            },
            FieldInfoData {
                name: "MinimumImpactRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, minimum_impact_radius),
            },
            FieldInfoData {
                name: "fakePhysiscs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, fake_physiscs),
            },
            FieldInfoData {
                name: "FakeMass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, fake_mass),
            },
            FieldInfoData {
                name: "StopDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, stop_delay),
            },
            FieldInfoData {
                name: "CooldownTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, cooldown_time),
            },
            FieldInfoData {
                name: "ActionType",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, action_type),
            },
            FieldInfoData {
                name: "ActionAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_TYPE_INFO,
                rust_offset: offset_of!(WorldEventActionventParameters, action_alignment),
            },
        ],
    }),
    array_type: Some(WORLDEVENTACTIONVENTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WorldEventActionventParameters {
    fn type_info() -> &'static TypeInfo {
        WORLDEVENTACTIONVENTPARAMETERS_TYPE_INFO
    }
}


pub const WORLDEVENTACTIONVENTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldEventActionventParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("WorldEventActionventParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CreatureLocoExternalInfluenceReactionAlignment {
    #[default]
    ExternalInfluence_AlignNone = 0,
    ExternalInfluence_AlignTowards = 1,
    ExternalInfluence_AlignAway = 2,
}

pub const CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionAlignment",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureLocoExternalInfluenceReactionAlignment {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_TYPE_INFO
    }
}


pub const CREATURELOCOEXTERNALINFLUENCEREACTIONALIGNMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionAlignment-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoExternalInfluenceReactionAlignment-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CreatureLocoExternalInfluenceReactionType {
    #[default]
    ExternalInfluence_Act = 0,
    ExternalInfluence_StopAndAct = 1,
}

pub const CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureLocoExternalInfluenceReactionType {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_TYPE_INFO
    }
}


pub const CREATURELOCOEXTERNALINFLUENCEREACTIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceReactionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoExternalInfluenceReactionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CreatureLocoExternalInfluenceType {
    #[default]
    ExternalInfluence_ShotHit = 0,
    ExternalInfluence_ExplosionHit = 1,
    ExternalInfluence_HumanPlayer = 2,
    ExternalInfluence_PushAway = 3,
    ExternalInfluence_PullIn = 4,
    ExternalInfluence_Freeze = 5,
    ExternalInfluence_UnknownEvent = 6,
    ExternalInfluence_Stun = 7,
    ExternalInfluence_StunMachines = 8,
    ExternalInfluence_NonHumanNear = 9,
    ExternalInfluence_Shoved = 10,
    ExternalInfluence_Count = 11,
}

pub const CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CREATURELOCOEXTERNALINFLUENCETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CreatureLocoExternalInfluenceType {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO
    }
}


pub const CREATURELOCOEXTERNALINFLUENCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoExternalInfluenceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoExternalInfluenceType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralMovementStateSettings {
    pub slow_speed: LocoStateSpeedRange,
    pub medium_speed: LocoStateSpeedRange,
    pub fast_speed: LocoStateSpeedRange,
    pub acceleration: f32,
    pub deceleration: f32,
    pub forward_speed_rate: f32,
    pub lateral_speed_rate: f32,
    pub reverse_speed_rate: f32,
    pub height_change_on_centerpoint: bool,
}

pub const PROCEDURALMOVEMENTSTATESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMovementStateSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SlowSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: LOCOSTATESPEEDRANGE_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, slow_speed),
            },
            FieldInfoData {
                name: "MediumSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: LOCOSTATESPEEDRANGE_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, medium_speed),
            },
            FieldInfoData {
                name: "FastSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: LOCOSTATESPEEDRANGE_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, fast_speed),
            },
            FieldInfoData {
                name: "Acceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, acceleration),
            },
            FieldInfoData {
                name: "Deceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, deceleration),
            },
            FieldInfoData {
                name: "ForwardSpeedRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, forward_speed_rate),
            },
            FieldInfoData {
                name: "LateralSpeedRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, lateral_speed_rate),
            },
            FieldInfoData {
                name: "ReverseSpeedRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, reverse_speed_rate),
            },
            FieldInfoData {
                name: "HeightChangeOnCenterpoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProceduralMovementStateSettings, height_change_on_centerpoint),
            },
        ],
    }),
    array_type: Some(PROCEDURALMOVEMENTSTATESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralMovementStateSettings {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALMOVEMENTSTATESETTINGS_TYPE_INFO
    }
}


pub const PROCEDURALMOVEMENTSTATESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMovementStateSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("ProceduralMovementStateSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LocoStateSpeedRange {
    pub max_speed: f32,
    pub min_speed: f32,
}

pub const LOCOSTATESPEEDRANGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoStateSpeedRange",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocoStateSpeedRange, max_speed),
            },
            FieldInfoData {
                name: "MinSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LocoStateSpeedRange, min_speed),
            },
        ],
    }),
    array_type: Some(LOCOSTATESPEEDRANGE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocoStateSpeedRange {
    fn type_info() -> &'static TypeInfo {
        LOCOSTATESPEEDRANGE_TYPE_INFO
    }
}


pub const LOCOSTATESPEEDRANGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoStateSpeedRange-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("LocoStateSpeedRange-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AvoidanceSteeringSettings {
    pub settings_data: AvoidanceData,
}

pub const AVOIDANCESTEERINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceSteeringSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SettingsData",
                flags: MemberInfoFlags::new(0),
                field_type: AVOIDANCEDATA_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceSteeringSettings, settings_data),
            },
        ],
    }),
    array_type: Some(AVOIDANCESTEERINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AvoidanceSteeringSettings {
    fn type_info() -> &'static TypeInfo {
        AVOIDANCESTEERINGSETTINGS_TYPE_INFO
    }
}


pub const AVOIDANCESTEERINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceSteeringSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AvoidanceSteeringSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AvoidanceData {
    pub time_to_consider: f32,
    pub obstical_width: f32,
    pub avoids_weapons_fire: bool,
    pub behaviour_towards_players: AIAvoidancePlayerBehaviour,
    pub avoidance_cone_width: f32,
    pub max_repulsion_weight: f32,
    pub repulsion_gain_rate: f32,
    pub repulsion_decay_rate: f32,
}

pub const AVOIDANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TimeToConsider",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, time_to_consider),
            },
            FieldInfoData {
                name: "ObsticalWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, obstical_width),
            },
            FieldInfoData {
                name: "AvoidsWeaponsFire",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, avoids_weapons_fire),
            },
            FieldInfoData {
                name: "BehaviourTowardsPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: AIAVOIDANCEPLAYERBEHAVIOUR_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, behaviour_towards_players),
            },
            FieldInfoData {
                name: "AvoidanceConeWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, avoidance_cone_width),
            },
            FieldInfoData {
                name: "MaxRepulsionWeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, max_repulsion_weight),
            },
            FieldInfoData {
                name: "RepulsionGainRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, repulsion_gain_rate),
            },
            FieldInfoData {
                name: "RepulsionDecayRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AvoidanceData, repulsion_decay_rate),
            },
        ],
    }),
    array_type: Some(AVOIDANCEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AvoidanceData {
    fn type_info() -> &'static TypeInfo {
        AVOIDANCEDATA_TYPE_INFO
    }
}


pub const AVOIDANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AvoidanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AvoidanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AIAvoidancePlayerBehaviour {
    #[default]
    IGNORES = 0,
    AVOIDS = 1,
    APPROACHES = 2,
}

pub const AIAVOIDANCEPLAYERBEHAVIOUR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIAvoidancePlayerBehaviour",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(AIAVOIDANCEPLAYERBEHAVIOUR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AIAvoidancePlayerBehaviour {
    fn type_info() -> &'static TypeInfo {
        AIAVOIDANCEPLAYERBEHAVIOUR_TYPE_INFO
    }
}


pub const AIAVOIDANCEPLAYERBEHAVIOUR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIAvoidancePlayerBehaviour-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("AIAvoidancePlayerBehaviour-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CurveSteeringSettings {
    pub base_settings_data: BasicSteeringSettingsData,
    pub settings_data: CurveSteeringSettingsData,
}

pub const CURVESTEERINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BaseSettingsData",
                flags: MemberInfoFlags::new(0),
                field_type: BASICSTEERINGSETTINGSDATA_TYPE_INFO,
                rust_offset: offset_of!(CurveSteeringSettings, base_settings_data),
            },
            FieldInfoData {
                name: "SettingsData",
                flags: MemberInfoFlags::new(0),
                field_type: CURVESTEERINGSETTINGSDATA_TYPE_INFO,
                rust_offset: offset_of!(CurveSteeringSettings, settings_data),
            },
        ],
    }),
    array_type: Some(CURVESTEERINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CurveSteeringSettings {
    fn type_info() -> &'static TypeInfo {
        CURVESTEERINGSETTINGS_TYPE_INFO
    }
}


pub const CURVESTEERINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CurveSteeringSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CurveSteeringSettingsData {
    pub foo: bool,
}

pub const CURVESTEERINGSETTINGSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettingsData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "foo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CurveSteeringSettingsData, foo),
            },
        ],
    }),
    array_type: Some(CURVESTEERINGSETTINGSDATA_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CurveSteeringSettingsData {
    fn type_info() -> &'static TypeInfo {
        CURVESTEERINGSETTINGSDATA_TYPE_INFO
    }
}


pub const CURVESTEERINGSETTINGSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveSteeringSettingsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CurveSteeringSettingsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BasicSteeringSettingsData {
    pub response_time: f32,
    pub maximum_angle_deflection: f32,
    pub error_distance: f32,
}

pub const BASICSTEERINGSETTINGSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BasicSteeringSettingsData",
    flags: MemberInfoFlags::new(36937),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ResponseTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BasicSteeringSettingsData, response_time),
            },
            FieldInfoData {
                name: "MaximumAngleDeflection",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BasicSteeringSettingsData, maximum_angle_deflection),
            },
            FieldInfoData {
                name: "ErrorDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BasicSteeringSettingsData, error_distance),
            },
        ],
    }),
    array_type: Some(BASICSTEERINGSETTINGSDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BasicSteeringSettingsData {
    fn type_info() -> &'static TypeInfo {
        BASICSTEERINGSETTINGSDATA_TYPE_INFO
    }
}


pub const BASICSTEERINGSETTINGSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BasicSteeringSettingsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("BasicSteeringSettingsData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CommonClientSettings {
}

pub const COMMONCLIENTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMMONCLIENTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CommonClientSettings {
    fn type_info() -> &'static TypeInfo {
        COMMONCLIENTSETTINGS_TYPE_INFO
    }
}


pub const COMMONCLIENTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CommonClientSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayAnimSettings {
}

pub const PLAYANIMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PLAYANIMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayAnimSettings {
    fn type_info() -> &'static TypeInfo {
        PLAYANIMSETTINGS_TYPE_INFO
    }
}


pub const PLAYANIMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PlayAnimSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TurnSettings {
}

pub const TURNSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TURNSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnSettings {
    fn type_info() -> &'static TypeInfo {
        TURNSETTINGS_TYPE_INFO
    }
}


pub const TURNSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("TurnSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StopSettings {
    pub base_deceleration_rate: f32,
}

pub const STOPSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BaseDecelerationRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StopSettings, base_deceleration_rate),
            },
        ],
    }),
    array_type: Some(STOPSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StopSettings {
    fn type_info() -> &'static TypeInfo {
        STOPSETTINGS_TYPE_INFO
    }
}


pub const STOPSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StopSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StartSettings {
    pub base_acceleration_rate: f32,
}

pub const STARTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BaseAccelerationRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(StartSettings, base_acceleration_rate),
            },
        ],
    }),
    array_type: Some(STARTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StartSettings {
    fn type_info() -> &'static TypeInfo {
        STARTSETTINGS_TYPE_INFO
    }
}


pub const STARTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StartSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MoveCycleSettings {
}

pub const MOVECYCLESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MOVECYCLESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoveCycleSettings {
    fn type_info() -> &'static TypeInfo {
        MOVECYCLESETTINGS_TYPE_INFO
    }
}


pub const MOVECYCLESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("MoveCycleSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IdleSettings {
}

pub const IDLESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONSTATESETTINGS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IDLESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleSettings {
    fn type_info() -> &'static TypeInfo {
        IDLESETTINGS_TYPE_INFO
    }
}


pub const IDLESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("IdleSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocomotionStateSettings {
}

pub const LOCOMOTIONSTATESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionStateSettings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCOMOTIONSTATESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocomotionStateSettings {
    fn type_info() -> &'static TypeInfo {
        LOCOMOTIONSTATESETTINGS_TYPE_INFO
    }
}


pub const LOCOMOTIONSTATESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionStateSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("LocomotionStateSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureLocoServerAuthEntityData {
    pub realm: super::core::Realm,
}

pub const CREATURELOCOSERVERAUTHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoServerAuthEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoServerAuthEntityData, realm),
            },
        ],
    }),
    array_type: Some(CREATURELOCOSERVERAUTHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureLocoServerAuthEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOSERVERAUTHENTITYDATA_TYPE_INFO
    }
}


pub const CREATURELOCOSERVERAUTHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoServerAuthEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoServerAuthEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureLocoEntityData {
    pub pin_to_ground: bool,
    pub enable_updates: bool,
}

pub const CREATURELOCOENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PinToGround",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoEntityData, pin_to_ground),
            },
            FieldInfoData {
                name: "EnableUpdates",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoEntityData, enable_updates),
            },
        ],
    }),
    array_type: Some(CREATURELOCOENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureLocoEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOENTITYDATA_TYPE_INFO
    }
}


pub const CREATURELOCOENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureBaseWaypointProviderEntityData {
}

pub const CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureBaseWaypointProviderEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREBASEWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureBaseWaypointProviderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureConfigurationProviderEntityData {
    pub default_settings: CreatureLocoSettings,
    pub ant_bindings: CreatureLocoBindings,
}

pub const CREATURECONFIGURATIONPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefaultSettings",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(CreatureConfigurationProviderEntityData, default_settings),
            },
            FieldInfoData {
                name: "AntBindings",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOBINDINGS_TYPE_INFO,
                rust_offset: offset_of!(CreatureConfigurationProviderEntityData, ant_bindings),
            },
        ],
    }),
    array_type: Some(CREATURECONFIGURATIONPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureConfigurationProviderEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATURECONFIGURATIONPROVIDERENTITYDATA_TYPE_INFO
    }
}


pub const CREATURECONFIGURATIONPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureConfigurationProviderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureLocoBindings {
    pub state_parameters: Vec<LocomotionParamBlock>,
}

pub const CREATURELOCOBINDINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBindings",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StateParameters",
                flags: MemberInfoFlags::new(144),
                field_type: LOCOMOTIONPARAMBLOCK_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoBindings, state_parameters),
            },
        ],
    }),
    array_type: Some(CREATURELOCOBINDINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureLocoBindings {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOBINDINGS_TYPE_INFO
    }
}


pub const CREATURELOCOBINDINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBindings-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoBindings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventReactionParamData {
    pub reaction_binding: CreatureReactionBinding,
}

pub const EVENTREACTIONPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventReactionParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ReactionBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATUREREACTIONBINDING_TYPE_INFO,
                rust_offset: offset_of!(EventReactionParamData, reaction_binding),
            },
        ],
    }),
    array_type: Some(EVENTREACTIONPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventReactionParamData {
    fn type_info() -> &'static TypeInfo {
        EVENTREACTIONPARAMDATA_TYPE_INFO
    }
}


pub const EVENTREACTIONPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventReactionParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("EventReactionParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureReactionBinding {
    pub reaction_trigger: super::ant::AntRef,
    pub reaction_from_stop: super::ant::AntRef,
    pub reaction_event_type: super::ant::AntRef,
}

pub const CREATUREREACTIONBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureReactionBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ReactionTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureReactionBinding, reaction_trigger),
            },
            FieldInfoData {
                name: "ReactionFromStop",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureReactionBinding, reaction_from_stop),
            },
            FieldInfoData {
                name: "ReactionEventType",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureReactionBinding, reaction_event_type),
            },
        ],
    }),
    array_type: Some(CREATUREREACTIONBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureReactionBinding {
    fn type_info() -> &'static TypeInfo {
        CREATUREREACTIONBINDING_TYPE_INFO
    }
}


pub const CREATUREREACTIONBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureReactionBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureReactionBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProceduralMotionParamData {
}

pub const PROCEDURALMOTIONPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMotionParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROCEDURALMOTIONPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralMotionParamData {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALMOTIONPARAMDATA_TYPE_INFO
    }
}


pub const PROCEDURALMOTIONPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralMotionParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("ProceduralMotionParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CommonClientBindingsParamData {
    pub common_binding: CreatureCommonBinding,
    pub misc_binding: CreatureMiscBinding,
}

pub const COMMONCLIENTBINDINGSPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientBindingsParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CommonBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURECOMMONBINDING_TYPE_INFO,
                rust_offset: offset_of!(CommonClientBindingsParamData, common_binding),
            },
            FieldInfoData {
                name: "MiscBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATUREMISCBINDING_TYPE_INFO,
                rust_offset: offset_of!(CommonClientBindingsParamData, misc_binding),
            },
        ],
    }),
    array_type: Some(COMMONCLIENTBINDINGSPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CommonClientBindingsParamData {
    fn type_info() -> &'static TypeInfo {
        COMMONCLIENTBINDINGSPARAMDATA_TYPE_INFO
    }
}


pub const COMMONCLIENTBINDINGSPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CommonClientBindingsParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CommonClientBindingsParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayAnimParamData {
    pub play_animation_binding: CreaturePlayAnimationBinding,
}

pub const PLAYANIMPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PlayAnimationBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATUREPLAYANIMATIONBINDING_TYPE_INFO,
                rust_offset: offset_of!(PlayAnimParamData, play_animation_binding),
            },
        ],
    }),
    array_type: Some(PLAYANIMPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayAnimParamData {
    fn type_info() -> &'static TypeInfo {
        PLAYANIMPARAMDATA_TYPE_INFO
    }
}


pub const PLAYANIMPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayAnimParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("PlayAnimParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TurnParamData {
    pub turn_binding: CreatureTurnBinding,
    pub turn_context_database: super::ant::AntRef,
}

pub const TURNPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TurnBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURETURNBINDING_TYPE_INFO,
                rust_offset: offset_of!(TurnParamData, turn_binding),
            },
            FieldInfoData {
                name: "TurnContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(TurnParamData, turn_context_database),
            },
        ],
    }),
    array_type: Some(TURNPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnParamData {
    fn type_info() -> &'static TypeInfo {
        TURNPARAMDATA_TYPE_INFO
    }
}


pub const TURNPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("TurnParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StopParamData {
    pub stop_binding: CreatureStopBinding,
    pub stop_context_database: super::ant::AntRef,
}

pub const STOPPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StopBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURESTOPBINDING_TYPE_INFO,
                rust_offset: offset_of!(StopParamData, stop_binding),
            },
            FieldInfoData {
                name: "StopContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(StopParamData, stop_context_database),
            },
        ],
    }),
    array_type: Some(STOPPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StopParamData {
    fn type_info() -> &'static TypeInfo {
        STOPPARAMDATA_TYPE_INFO
    }
}


pub const STOPPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StopParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StartParamData {
    pub start_binding: CreatureStartBinding,
    pub start_context_database: super::ant::AntRef,
}

pub const STARTPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StartBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURESTARTBINDING_TYPE_INFO,
                rust_offset: offset_of!(StartParamData, start_binding),
            },
            FieldInfoData {
                name: "StartContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(StartParamData, start_context_database),
            },
        ],
    }),
    array_type: Some(STARTPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StartParamData {
    fn type_info() -> &'static TypeInfo {
        STARTPARAMDATA_TYPE_INFO
    }
}


pub const STARTPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("StartParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MoveCycleParamData {
    pub loco_binding: CreatureLocoBinding,
    pub loco_context_database: super::ant::AntRef,
    pub accel_context_database: super::ant::AntRef,
}

pub const MOVECYCLEPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocoBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOBINDING_TYPE_INFO,
                rust_offset: offset_of!(MoveCycleParamData, loco_binding),
            },
            FieldInfoData {
                name: "LocoContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoveCycleParamData, loco_context_database),
            },
            FieldInfoData {
                name: "AccelContextDatabase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoveCycleParamData, accel_context_database),
            },
        ],
    }),
    array_type: Some(MOVECYCLEPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoveCycleParamData {
    fn type_info() -> &'static TypeInfo {
        MOVECYCLEPARAMDATA_TYPE_INFO
    }
}


pub const MOVECYCLEPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveCycleParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("MoveCycleParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IdleParamData {
    pub idle_binding: CreatureIdleBinding,
}

pub const IDLEPARAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleParamData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOMOTIONPARAMBLOCK_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IdleBinding",
                flags: MemberInfoFlags::new(0),
                field_type: CREATUREIDLEBINDING_TYPE_INFO,
                rust_offset: offset_of!(IdleParamData, idle_binding),
            },
        ],
    }),
    array_type: Some(IDLEPARAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleParamData {
    fn type_info() -> &'static TypeInfo {
        IDLEPARAMDATA_TYPE_INFO
    }
}


pub const IDLEPARAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleParamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("IdleParamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreaturePlayAnimationBinding {
    pub branch_in_phase: super::ant::AntRef,
    pub align_translation: super::ant::AntRef,
    pub align_facing_rotation: super::ant::AntRef,
    pub ant_in_play_animation: super::ant::AntRef,
}

pub const CREATUREPLAYANIMATIONBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BranchInPhase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationBinding, branch_in_phase),
            },
            FieldInfoData {
                name: "AlignTranslation",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationBinding, align_translation),
            },
            FieldInfoData {
                name: "AlignFacingRotation",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationBinding, align_facing_rotation),
            },
            FieldInfoData {
                name: "AntInPlayAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreaturePlayAnimationBinding, ant_in_play_animation),
            },
        ],
    }),
    array_type: Some(CREATUREPLAYANIMATIONBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreaturePlayAnimationBinding {
    fn type_info() -> &'static TypeInfo {
        CREATUREPLAYANIMATIONBINDING_TYPE_INFO
    }
}


pub const CREATUREPLAYANIMATIONBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreaturePlayAnimationBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreaturePlayAnimationBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureTurnBinding {
    pub turn_angle: super::ant::AntRef,
    pub turn_phase: super::ant::AntRef,
    pub turn_trigger: super::ant::AntRef,
}

pub const CREATURETURNBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureTurnBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TurnAngle",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureTurnBinding, turn_angle),
            },
            FieldInfoData {
                name: "TurnPhase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureTurnBinding, turn_phase),
            },
            FieldInfoData {
                name: "TurnTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureTurnBinding, turn_trigger),
            },
        ],
    }),
    array_type: Some(CREATURETURNBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureTurnBinding {
    fn type_info() -> &'static TypeInfo {
        CREATURETURNBINDING_TYPE_INFO
    }
}


pub const CREATURETURNBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureTurnBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureTurnBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureStopBinding {
    pub stop_angle: super::ant::AntRef,
    pub stop_relative_facing_angle: super::ant::AntRef,
    pub stop_phase: super::ant::AntRef,
    pub stop_trigger: super::ant::AntRef,
}

pub const CREATURESTOPBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStopBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StopAngle",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureStopBinding, stop_angle),
            },
            FieldInfoData {
                name: "StopRelativeFacingAngle",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureStopBinding, stop_relative_facing_angle),
            },
            FieldInfoData {
                name: "StopPhase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureStopBinding, stop_phase),
            },
            FieldInfoData {
                name: "StopTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureStopBinding, stop_trigger),
            },
        ],
    }),
    array_type: Some(CREATURESTOPBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureStopBinding {
    fn type_info() -> &'static TypeInfo {
        CREATURESTOPBINDING_TYPE_INFO
    }
}


pub const CREATURESTOPBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStopBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureStopBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureStartBinding {
    pub start_facing_offset: super::ant::AntRef,
    pub start_motion_delta: super::ant::AntRef,
    pub start_trigger: super::ant::AntRef,
}

pub const CREATURESTARTBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStartBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StartFacingOffset",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureStartBinding, start_facing_offset),
            },
            FieldInfoData {
                name: "StartMotionDelta",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureStartBinding, start_motion_delta),
            },
            FieldInfoData {
                name: "StartTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureStartBinding, start_trigger),
            },
        ],
    }),
    array_type: Some(CREATURESTARTBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureStartBinding {
    fn type_info() -> &'static TypeInfo {
        CREATURESTARTBINDING_TYPE_INFO
    }
}


pub const CREATURESTARTBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureStartBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureStartBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureMiscBinding {
    pub current_speed: super::ant::AntRef,
    pub loco_speed_multiplier: super::ant::AntRef,
    pub g_force: super::ant::AntRef,
    pub relative_pitch: super::ant::AntRef,
    pub path_steering: super::ant::AntRef,
    pub relative_steering: super::ant::AntRef,
    pub raw_delat_traj: super::ant::AntRef,
    pub speed_mode: super::ant::AntRef,
    pub awareness_x_target: super::ant::AntRef,
    pub awareness_y_target: super::ant::AntRef,
}

pub const CREATUREMISCBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMiscBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CurrentSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, current_speed),
            },
            FieldInfoData {
                name: "LocoSpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, loco_speed_multiplier),
            },
            FieldInfoData {
                name: "G_Force",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, g_force),
            },
            FieldInfoData {
                name: "RelativePitch",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, relative_pitch),
            },
            FieldInfoData {
                name: "PathSteering",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, path_steering),
            },
            FieldInfoData {
                name: "RelativeSteering",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, relative_steering),
            },
            FieldInfoData {
                name: "RawDelatTraj",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, raw_delat_traj),
            },
            FieldInfoData {
                name: "SpeedMode",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, speed_mode),
            },
            FieldInfoData {
                name: "AwarenessXTarget",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, awareness_x_target),
            },
            FieldInfoData {
                name: "AwarenessYTarget",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureMiscBinding, awareness_y_target),
            },
        ],
    }),
    array_type: Some(CREATUREMISCBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureMiscBinding {
    fn type_info() -> &'static TypeInfo {
        CREATUREMISCBINDING_TYPE_INFO
    }
}


pub const CREATUREMISCBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureMiscBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureMiscBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureLocoBinding {
    pub transition_speed_mode: super::ant::AntRef,
    pub trigger_speed_transition: super::ant::AntRef,
    pub accel_decel_phase: super::ant::AntRef,
    pub loco_turn_phase: super::ant::AntRef,
    pub loco_end_phase: super::ant::AntRef,
    pub loco_trigger: super::ant::AntRef,
}

pub const CREATURELOCOBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TransitionSpeedMode",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoBinding, transition_speed_mode),
            },
            FieldInfoData {
                name: "TriggerSpeedTransition",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoBinding, trigger_speed_transition),
            },
            FieldInfoData {
                name: "AccelDecelPhase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoBinding, accel_decel_phase),
            },
            FieldInfoData {
                name: "LocoTurnPhase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoBinding, loco_turn_phase),
            },
            FieldInfoData {
                name: "LocoEndPhase",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoBinding, loco_end_phase),
            },
            FieldInfoData {
                name: "LocoTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureLocoBinding, loco_trigger),
            },
        ],
    }),
    array_type: Some(CREATURELOCOBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureLocoBinding {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOBINDING_TYPE_INFO
    }
}


pub const CREATURELOCOBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureLocoBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureIdleBinding {
    pub idle_turn_angle: super::ant::AntRef,
    pub idle_turn_trigger: super::ant::AntRef,
}

pub const CREATUREIDLEBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureIdleBinding",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IdleTurnAngle",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureIdleBinding, idle_turn_angle),
            },
            FieldInfoData {
                name: "IdleTurnTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureIdleBinding, idle_turn_trigger),
            },
        ],
    }),
    array_type: Some(CREATUREIDLEBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for CreatureIdleBinding {
    fn type_info() -> &'static TypeInfo {
        CREATUREIDLEBINDING_TYPE_INFO
    }
}


pub const CREATUREIDLEBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureIdleBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureIdleBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureCommonBinding {
    pub warp_angle: super::ant::AntRef,
    pub breakout_early: super::ant::AntRef,
    pub early_out_branch_types: Vec<EarlyOutType>,
}

pub const CREATURECOMMONBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCommonBinding",
    flags: MemberInfoFlags::new(73),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WarpAngle",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureCommonBinding, warp_angle),
            },
            FieldInfoData {
                name: "BreakoutEarly",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(CreatureCommonBinding, breakout_early),
            },
            FieldInfoData {
                name: "EarlyOutBranchTypes",
                flags: MemberInfoFlags::new(144),
                field_type: EARLYOUTTYPE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CreatureCommonBinding, early_out_branch_types),
            },
        ],
    }),
    array_type: Some(CREATURECOMMONBINDING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureCommonBinding {
    fn type_info() -> &'static TypeInfo {
        CREATURECOMMONBINDING_TYPE_INFO
    }
}


pub const CREATURECOMMONBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCommonBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureCommonBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EarlyOutType {
    pub early_out_branch_type: super::ant::AntRef,
}

pub const EARLYOUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EarlyOutType",
    flags: MemberInfoFlags::new(32841),
    module: "CreatureLocoShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EarlyOutBranchType",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(EarlyOutType, early_out_branch_type),
            },
        ],
    }),
    array_type: Some(EARLYOUTTYPE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EarlyOutType {
    fn type_info() -> &'static TypeInfo {
        EARLYOUTTYPE_TYPE_INFO
    }
}


pub const EARLYOUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EarlyOutType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("EarlyOutType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocomotionParamBlock {
}

pub const LOCOMOTIONPARAMBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionParamBlock",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCOMOTIONPARAMBLOCK_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocomotionParamBlock {
    fn type_info() -> &'static TypeInfo {
        LOCOMOTIONPARAMBLOCK_TYPE_INFO
    }
}


pub const LOCOMOTIONPARAMBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocomotionParamBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("LocomotionParamBlock-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointUnspawnEntityData {
    pub realm: super::core::Realm,
}

pub const CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointUnspawnEntityData, realm),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointUnspawnEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTUNSPAWNENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointUnspawnEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointClosestChooserEntityData {
    pub realm: super::core::Realm,
    pub behaviour_type: CL_WaypointListChooser_ClosestToType,
}

pub const CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointClosestChooserEntityData, realm),
            },
            FieldInfoData {
                name: "BehaviourType",
                flags: MemberInfoFlags::new(0),
                field_type: CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointClosestChooserEntityData, behaviour_type),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointClosestChooserEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointClosestChooserEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CL_WaypointListChooser_ClosestToType {
    #[default]
    Closest_To_Current_Character = 0,
    Closest_To_Nearest_PLayer = 1,
    Farthest_From_Nearest_Player = 2,
}

pub const CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointListChooser_ClosestToType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CL_WaypointListChooser_ClosestToType {
    fn type_info() -> &'static TypeInfo {
        CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_TYPE_INFO
    }
}


pub const CL_WAYPOINTLISTCHOOSER_CLOSESTTOTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointListChooser_ClosestToType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CL_WaypointListChooser_ClosestToType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointOccupancyChooserEntityData {
    pub realm: super::core::Realm,
    pub enable_available: bool,
    pub occupancy_limit: i32,
}

pub const CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointOccupancyChooserEntityData, realm),
            },
            FieldInfoData {
                name: "EnableAvailable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointOccupancyChooserEntityData, enable_available),
            },
            FieldInfoData {
                name: "OccupancyLimit",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointOccupancyChooserEntityData, occupancy_limit),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointOccupancyChooserEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointOccupancyChooserEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreatureFollowWaypointBoolChooserEntityData {
    pub realm: super::core::Realm,
    pub chance_of_true: f32,
    pub enable_random_choice: bool,
    pub selection_condition: bool,
}

pub const CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, realm),
            },
            FieldInfoData {
                name: "ChanceOfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, chance_of_true),
            },
            FieldInfoData {
                name: "EnableRandomChoice",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, enable_random_choice),
            },
            FieldInfoData {
                name: "SelectionCondition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointBoolChooserEntityData, selection_condition),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointBoolChooserEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointBoolChooserEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointsEntityData {
}

pub const CREATUREFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointsEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointSegmentEntityData {
    pub realm: super::core::Realm,
    pub type_of_route: super::pathfinding_shared::RouteType,
    pub start_point: CL_WaypointList_StartType,
    pub is_reversable: bool,
    pub max_repititions: u32,
    pub speed_override: CreatureSpeedLevel,
    pub force_explicit_height: bool,
}

pub const CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointSegmentEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, realm),
            },
            FieldInfoData {
                name: "TypeOfRoute",
                flags: MemberInfoFlags::new(0),
                field_type: ROUTETYPE_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, type_of_route),
            },
            FieldInfoData {
                name: "Start_Point",
                flags: MemberInfoFlags::new(0),
                field_type: CL_WAYPOINTLIST_STARTTYPE_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, start_point),
            },
            FieldInfoData {
                name: "IsReversable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, is_reversable),
            },
            FieldInfoData {
                name: "MaxRepititions",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, max_repititions),
            },
            FieldInfoData {
                name: "SpeedOverride",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURESPEEDLEVEL_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, speed_override),
            },
            FieldInfoData {
                name: "ForceExplicitHeight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CreatureFollowWaypointSegmentEntityData, force_explicit_height),
            },
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointSegmentEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTSEGMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointSegmentEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointSegmentEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowBaseData {
}

pub const CREATUREFOLLOWBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowBaseData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWBASEDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointProviderEntityData {
}

pub const CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointProviderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureFollowWaypointProviderEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTPROVIDERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointProviderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureFollowWaypointProviderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CL_WaypointList_StartType {
    #[default]
    First_Point = 0,
    Nearest_Point = 1,
    Nearest_Point_Ahead = 2,
}

pub const CL_WAYPOINTLIST_STARTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointList_StartType",
    flags: MemberInfoFlags::new(49429),
    module: "CreatureLocoShared",
    data: TypeInfoData::Enum,
    array_type: Some(CL_WAYPOINTLIST_STARTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CL_WaypointList_StartType {
    fn type_info() -> &'static TypeInfo {
        CL_WAYPOINTLIST_STARTTYPE_TYPE_INFO
    }
}


pub const CL_WAYPOINTLIST_STARTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_WaypointList_StartType-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CL_WaypointList_StartType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreatureCollisionGroupData {
    pub realm: super::core::Realm,
    pub avoidance_threshold: f32,
    pub average_group_size: i32,
}

pub const CREATURECOLLISIONGROUPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCollisionGroupData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CreatureCollisionGroupData, realm),
            },
            FieldInfoData {
                name: "AvoidanceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CreatureCollisionGroupData, avoidance_threshold),
            },
            FieldInfoData {
                name: "AverageGroupSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CreatureCollisionGroupData, average_group_size),
            },
        ],
    }),
    array_type: Some(CREATURECOLLISIONGROUPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CreatureCollisionGroupData {
    fn type_info() -> &'static TypeInfo {
        CREATURECOLLISIONGROUPDATA_TYPE_INFO
    }
}


pub const CREATURECOLLISIONGROUPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureCollisionGroupData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CreatureCollisionGroupData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLInfluenceFilterEntityData {
    pub realm: super::core::Realm,
    pub influence_type: CreatureLocoExternalInfluenceType,
}

pub const CLINFLUENCEFILTERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CLInfluenceFilterEntityData, realm),
            },
            FieldInfoData {
                name: "InfluenceType",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO,
                rust_offset: offset_of!(CLInfluenceFilterEntityData, influence_type),
            },
        ],
    }),
    array_type: Some(CLINFLUENCEFILTERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CLInfluenceFilterEntityData {
    fn type_info() -> &'static TypeInfo {
        CLINFLUENCEFILTERENTITYDATA_TYPE_INFO
    }
}


pub const CLINFLUENCEFILTERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CLInfluenceFilterEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLInfluenceCompareEntityData {
    pub realm: super::core::Realm,
    pub influence_a: CreatureLocoExternalInfluenceType,
    pub influence_b: CreatureLocoExternalInfluenceType,
}

pub const CLINFLUENCECOMPAREENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CLInfluenceCompareEntityData, realm),
            },
            FieldInfoData {
                name: "Influence_A",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO,
                rust_offset: offset_of!(CLInfluenceCompareEntityData, influence_a),
            },
            FieldInfoData {
                name: "Influence_B",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO,
                rust_offset: offset_of!(CLInfluenceCompareEntityData, influence_b),
            },
        ],
    }),
    array_type: Some(CLINFLUENCECOMPAREENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CLInfluenceCompareEntityData {
    fn type_info() -> &'static TypeInfo {
        CLINFLUENCECOMPAREENTITYDATA_TYPE_INFO
    }
}


pub const CLINFLUENCECOMPAREENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CLInfluenceCompareEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CLApplyInfluenceEntityData {
    pub influence_type: CreatureLocoExternalInfluenceType,
    pub location: super::core::Vec3,
    pub radius: f32,
    pub direction: super::core::Vec3,
    pub cone_angle: f32,
    pub is_omnidirectional: bool,
    pub realm: super::core::Realm,
}

pub const CLAPPLYINFLUENCEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLocoShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InfluenceType",
                flags: MemberInfoFlags::new(0),
                field_type: CREATURELOCOEXTERNALINFLUENCETYPE_TYPE_INFO,
                rust_offset: offset_of!(CLApplyInfluenceEntityData, influence_type),
            },
            FieldInfoData {
                name: "Location",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CLApplyInfluenceEntityData, location),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CLApplyInfluenceEntityData, radius),
            },
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(CLApplyInfluenceEntityData, direction),
            },
            FieldInfoData {
                name: "ConeAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CLApplyInfluenceEntityData, cone_angle),
            },
            FieldInfoData {
                name: "IsOmnidirectional",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CLApplyInfluenceEntityData, is_omnidirectional),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CLApplyInfluenceEntityData, realm),
            },
        ],
    }),
    array_type: Some(CLAPPLYINFLUENCEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CLApplyInfluenceEntityData {
    fn type_info() -> &'static TypeInfo {
        CLAPPLYINFLUENCEENTITYDATA_TYPE_INFO
    }
}


pub const CLAPPLYINFLUENCEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLocoShared",
    data: TypeInfoData::Array("CLApplyInfluenceEntityData-Array"),
    array_type: None,
    alignment: 8,
};


