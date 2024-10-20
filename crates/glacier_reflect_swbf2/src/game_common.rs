use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_game_common_types(registry: &mut TypeRegistry) {
    registry.register_type(ANIMATIONPOSELAYERTRACK_TYPE_INFO);
    registry.register_type(ANIMATIONPOSELAYERTRACK_ARRAY_TYPE_INFO);
    registry.register_type(ANTCONTROLTRACK_TYPE_INFO);
    registry.register_type(ANTCONTROLTRACK_ARRAY_TYPE_INFO);
    registry.register_type(ANTBONETRACK_TYPE_INFO);
    registry.register_type(ANTBONETRACK_ARRAY_TYPE_INFO);
    registry.register_type(ANTBONEANIMATIONTRACK_TYPE_INFO);
    registry.register_type(ANTBONEANIMATIONTRACK_ARRAY_TYPE_INFO);
    registry.register_type(ANIMATIONPOSETRACK_TYPE_INFO);
    registry.register_type(ANIMATIONPOSETRACK_ARRAY_TYPE_INFO);
    registry.register_type(RECORDEDBLOBRESOURCE_TYPE_INFO);
    registry.register_type(RECORDEDBLOBRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(ANTBONETRANSFORMLAYERTRACK_TYPE_INFO);
    registry.register_type(ANTBONETRANSFORMLAYERTRACK_ARRAY_TYPE_INFO);
    registry.register_type(VEHICLEEXITPOINTCOMPONENT_TYPE_INFO);
    registry.register_type(VEHICLEEXITPOINTCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO);
    registry.register_type(PHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MODELANIMATIONENTITY_TYPE_INFO);
    registry.register_type(MODELANIMATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ANTSLOTTRACK_TYPE_INFO);
    registry.register_type(ANTSLOTTRACK_ARRAY_TYPE_INFO);
    registry.register_type(ANTSIGNALTRACK_TYPE_INFO);
    registry.register_type(ANTSIGNALTRACK_ARRAY_TYPE_INFO);
    registry.register_type(ANTEVALTRACK_TYPE_INFO);
    registry.register_type(ANTEVALTRACK_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTAREAQUERYENTITY_TYPE_INFO);
    registry.register_type(OBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BILLBOARDTRANSFORMENTITY_TYPE_INFO);
    registry.register_type(BILLBOARDTRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WATERDEPTHENTITY_TYPE_INFO);
    registry.register_type(WATERDEPTHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMSNAPTOGROUNDENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMSNAPTOGROUNDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INCLUSIONSETTINGENTITY_TYPE_INFO);
    registry.register_type(INCLUSIONSETTINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EVENTSPLITTERENTITY_TYPE_INFO);
    registry.register_type(EVENTSPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EVENTCOMPAREGATEENTITY_TYPE_INFO);
    registry.register_type(EVENTCOMPAREGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMBATAREAENTITY_TYPE_INFO);
    registry.register_type(COMBATAREAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BUILDCONFIGFILTERENTITY_TYPE_INFO);
    registry.register_type(BUILDCONFIGFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AREAPROXIMITYENTITY_TYPE_INFO);
    registry.register_type(AREAPROXIMITYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(WAYPOINTPARAMETEREVENT_TYPE_INFO);
    registry.register_type(WAYPOINTPARAMETEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(MOVEPARAMETEREVENT_TYPE_INFO);
    registry.register_type(MOVEPARAMETEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(TOOL_TYPE_INFO);
    registry.register_type(TOOL_ARRAY_TYPE_INFO);
    registry.register_type(SPHERECOLLISIONENTITY_TYPE_INFO);
    registry.register_type(SPHERECOLLISIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SLOWMOTIONENTITY_TYPE_INFO);
    registry.register_type(SLOWMOTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OBBCOLLISIONENTITY_TYPE_INFO);
    registry.register_type(OBBCOLLISIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(EXPLOSIONENTITY_TYPE_INFO);
    registry.register_type(EXPLOSIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DIFFICULTYINDEXENTITY_TYPE_INFO);
    registry.register_type(DIFFICULTYINDEXENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PLAYERVIEWHITSTATE_TYPE_INFO);
    registry.register_type(PLAYERVIEWHITSTATE_ARRAY_TYPE_INFO);
    registry.register_type(BUCKETSCORECONTAINER_TYPE_INFO);
    registry.register_type(BUCKETSCORECONTAINER_ARRAY_TYPE_INFO);
    registry.register_type(PLAYERSCOREDETAIL_TYPE_INFO);
    registry.register_type(PLAYERSCOREDETAIL_ARRAY_TYPE_INFO);
    registry.register_type(UINETWORKCHECKLEVELINSTALLEDMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKROLLCREDITSMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKSTEALBODYMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKPLAYERMISSIONOBJECTIVETEXTMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKVIDEODONEMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKALLOWSKIPVIDEOMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKSKIPVIDEOMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKSTOPVIDEOMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKPAUSEVIDEOMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKPAUSEALLVIDEOSMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKPLAYERDESERTERTEXTMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKPLAYERTEXTMESSAGE_TYPE_INFO);
    registry.register_type(UINETWORKTEXTINFO_TYPE_INFO);
    registry.register_type(UINETWORKTEXTINFO_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKREMOVEDILATIONMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKADDDILATIONMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKLEVELINSTALLEDMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKMATCHREADYSTATUSCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERRESTARTTIMERMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKMETRICSLEVELCOMPLETEMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKMETRICSLEVELPROGRESSMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSETPLAYERVIEWMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSUICIDEMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKGAMEPLAYCONTINUEMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCELOGMESSAGE_TYPE_INFO);
    registry.register_type(PERFORMANCEFPSHISTOGRAM_TYPE_INFO);
    registry.register_type(PERFORMANCEFPSHISTOGRAM_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKFIRSTPLAYERENTEREDMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKDIFFICULTYCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSETTINGSMESSAGE_TYPE_INFO);
    registry.register_type(DEBUGSPAWNGAMEENTITYMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKMOVEPLAYERMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKJUICESESSIONMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSELECTTEAMMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKONPLAYERSPAWNEDMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSELECTSPAWNGROUPMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSPAWNVEHICLECUSTOMIZATIONMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKUNSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSPAWNONSELECTEDMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSPAWNHEREMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKREQUESTLOADLEVELMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKSCREENFADEMESSAGE_TYPE_INFO);
    registry.register_type(LEVELCONSOLESETACTIVEHEALTHSTATEMESSAGE_TYPE_INFO);
    registry.register_type(NETWORKPERFORMANCEPROFILEMESSAGE_TYPE_INFO);
    registry.register_type(SYNCEDSEQUENCESTATECHANGEMESSAGEBASE_TYPE_INFO);
    registry.register_type(JUICESOLDIERRAGDOLLDEACTIVATEMESSAGE_TYPE_INFO);
    registry.register_type(JUICESOLDIERRAGDOLLACTIVATEMESSAGE_TYPE_INFO);
    registry.register_type(AICLIENTBRIDGEDYNAMICMODELENTITYONUNSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(AICLIENTBRIDGEDYNAMICMODELENTITYONSPAWNMESSAGE_TYPE_INFO);
    registry.register_type(WANTEDAIMSTATE_TYPE_INFO);
    registry.register_type(WANTEDAIMSTATE_ARRAY_TYPE_INFO);
    registry.register_type(AIMTARGETID_TYPE_INFO);
    registry.register_type(AIMTARGETID_ARRAY_TYPE_INFO);
    registry.register_type(SHAREDLOCKOBSERVERENTITY_TYPE_INFO);
    registry.register_type(SHAREDLOCKOBSERVERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SHAREDLOCKGATEENTITY_TYPE_INFO);
    registry.register_type(SHAREDLOCKGATEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SHAREDLOCKCONTROLLERENTITY_TYPE_INFO);
    registry.register_type(SHAREDLOCKCONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SHAREDLOCKBASEENTITY_TYPE_INFO);
    registry.register_type(SHAREDLOCKBASEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INTERSECTIONTRIGGERENTITY_TYPE_INFO);
    registry.register_type(INTERSECTIONTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(USERSPAWNCONTEXT_TYPE_INFO);
    registry.register_type(USERSPAWNCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(PLAYERITERATORENTITY_TYPE_INFO);
    registry.register_type(PLAYERITERATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(PLATFORMSPLITTERENTITY_TYPE_INFO);
    registry.register_type(PLATFORMSPLITTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOGGINGENTITY_TYPE_INFO);
    registry.register_type(LOGGINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DELTAVIEWERTABLEENTITY_TYPE_INFO);
    registry.register_type(DELTAVIEWERTABLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONSOLECOMMANDTRIGGERENTITY_TYPE_INFO);
    registry.register_type(CONSOLECOMMANDTRIGGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONSOLECOMMANDENTITY_TYPE_INFO);
    registry.register_type(CONSOLECOMMANDENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LICENSEEGAMEWORLDRAYCASTER_TYPE_INFO);
    registry.register_type(LICENSEEGAMEWORLDRAYCASTER_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELSTATUSENTITY_TYPE_INFO);
    registry.register_type(SUBLEVELSTATUSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELPRELOADENTITY_TYPE_INFO);
    registry.register_type(SUBLEVELPRELOADENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO);
    registry.register_type(SUBLEVELCOLLECTIONENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLESTATUSENTITY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLESTATUSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTBUNDLEENTITY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTBUNDLEENTITY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLEENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO);
    registry.register_type(SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO);
    registry.register_type(CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO);
    registry.register_type(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(TESTSTATICMODELGROUPENTITYDATA_TYPE_INFO);
    registry.register_type(TESTSTATICMODELGROUPENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TESTRAYCASTENTITYDATA_TYPE_INFO);
    registry.register_type(TESTRAYCASTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ADMINISTRATIONRESTRICTIONLEVEL_TYPE_INFO);
    registry.register_type(ADMINISTRATIONRESTRICTIONLEVEL_ARRAY_TYPE_INFO);
    registry.register_type(ADMINISTRATIONEVENTTYPE_TYPE_INFO);
    registry.register_type(ADMINISTRATIONEVENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERADMINISTRATIONPASSWORDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERADMINISTRATIONEVENTSENABLEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERADMINISTRATIONQUITMESSAGE_TYPE_INFO);
    registry.register_type(SERVERADMINISTRATIONLOGINMESSAGE_TYPE_INFO);
    registry.register_type(SERVERADMINISTRATIONPACKETMESSAGEBASE_TYPE_INFO);
    registry.register_type(GAMEMODULE_TYPE_INFO);
    registry.register_type(GAMEMODULE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnimationPoseLayerTrack {
}

pub const ANIMATIONPOSELAYERTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseLayerTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GROUPTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANIMATIONPOSELAYERTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnimationPoseLayerTrack {
    fn type_info() -> &'static TypeInfo {
        ANIMATIONPOSELAYERTRACK_TYPE_INFO
    }
}


pub const ANIMATIONPOSELAYERTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseLayerTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AnimationPoseLayerTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTControlTrack {
}

pub const ANTCONTROLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControlTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTCONTROLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTControlTrack {
    fn type_info() -> &'static TypeInfo {
        ANTCONTROLTRACK_TYPE_INFO
    }
}


pub const ANTCONTROLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControlTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTControlTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTBoneTrack {
}

pub const ANTBONETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LAYEREDTRANSFORMTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTBONETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTBoneTrack {
    fn type_info() -> &'static TypeInfo {
        ANTBONETRACK_TYPE_INFO
    }
}


pub const ANTBONETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTBoneTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTBoneAnimationTrack {
}

pub const ANTBONEANIMATIONTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneAnimationTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTBONEANIMATIONTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTBoneAnimationTrack {
    fn type_info() -> &'static TypeInfo {
        ANTBONEANIMATIONTRACK_TYPE_INFO
    }
}


pub const ANTBONEANIMATIONTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneAnimationTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTBoneAnimationTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnimationPoseTrack {
}

pub const ANIMATIONPOSETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANIMATIONPOSETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnimationPoseTrack {
    fn type_info() -> &'static TypeInfo {
        ANIMATIONPOSETRACK_TYPE_INFO
    }
}


pub const ANIMATIONPOSETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AnimationPoseTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RecordedBlobResource {
}

pub const RECORDEDBLOBRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordedBlobResource",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(RECORDEDBLOBRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RecordedBlobResource {
    fn type_info() -> &'static TypeInfo {
        RECORDEDBLOBRESOURCE_TYPE_INFO
    }
}


pub const RECORDEDBLOBRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordedBlobResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("RecordedBlobResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTBoneTransformLayerTrack {
}

pub const ANTBONETRANSFORMLAYERTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTransformLayerTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SIMPLETRANSFORMLAYER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTBONETRANSFORMLAYERTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTBoneTransformLayerTrack {
    fn type_info() -> &'static TypeInfo {
        ANTBONETRANSFORMLAYERTRACK_TYPE_INFO
    }
}


pub const ANTBONETRANSFORMLAYERTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTransformLayerTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTBoneTransformLayerTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VehicleExitPointComponent {
}

pub const VEHICLEEXITPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleExitPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEEXITPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehicleExitPointComponent {
    fn type_info() -> &'static TypeInfo {
        VEHICLEEXITPOINTCOMPONENT_TYPE_INFO
    }
}


pub const VEHICLEEXITPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleExitPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("VehicleExitPointComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PhysicsDrivenAnimationEntity {
}

pub const PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDrivenAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsDrivenAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
    }
}


pub const PHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDrivenAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PhysicsDrivenAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ModelAnimationEntity {
}

pub const MODELANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MODELANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ModelAnimationEntity {
    fn type_info() -> &'static TypeInfo {
        MODELANIMATIONENTITY_TYPE_INFO
    }
}


pub const MODELANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ModelAnimationEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTSlotTrack {
}

pub const ANTSLOTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSlotTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTSLOTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTSlotTrack {
    fn type_info() -> &'static TypeInfo {
        ANTSLOTTRACK_TYPE_INFO
    }
}


pub const ANTSLOTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSlotTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTSlotTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTSignalTrack {
}

pub const ANTSIGNALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSignalTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTSIGNALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTSignalTrack {
    fn type_info() -> &'static TypeInfo {
        ANTSIGNALTRACK_TYPE_INFO
    }
}


pub const ANTSIGNALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSignalTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTSignalTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANTEvalTrack {
}

pub const ANTEVALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvalTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ANTEVALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTEvalTrack {
    fn type_info() -> &'static TypeInfo {
        ANTEVALTRACK_TYPE_INFO
    }
}


pub const ANTEVALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvalTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTEvalTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectAreaQueryEntity {
}

pub const OBJECTAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectAreaQueryEntity {
    fn type_info() -> &'static TypeInfo {
        OBJECTAREAQUERYENTITY_TYPE_INFO
    }
}


pub const OBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ObjectAreaQueryEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BillboardTransformEntity {
}

pub const BILLBOARDTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BillboardTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BILLBOARDTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BillboardTransformEntity {
    fn type_info() -> &'static TypeInfo {
        BILLBOARDTRANSFORMENTITY_TYPE_INFO
    }
}


pub const BILLBOARDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BillboardTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BillboardTransformEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaterDepthEntity {
}

pub const WATERDEPTHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDepthEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WATERDEPTHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterDepthEntity {
    fn type_info() -> &'static TypeInfo {
        WATERDEPTHENTITY_TYPE_INFO
    }
}


pub const WATERDEPTHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDepthEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("WaterDepthEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformSnapToGroundEntity {
}

pub const TRANSFORMSNAPTOGROUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSnapToGroundEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMSNAPTOGROUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformSnapToGroundEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMSNAPTOGROUNDENTITY_TYPE_INFO
    }
}


pub const TRANSFORMSNAPTOGROUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSnapToGroundEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("TransformSnapToGroundEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InclusionSettingEntity {
}

pub const INCLUSIONSETTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InclusionSettingEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INCLUSIONSETTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InclusionSettingEntity {
    fn type_info() -> &'static TypeInfo {
        INCLUSIONSETTINGENTITY_TYPE_INFO
    }
}


pub const INCLUSIONSETTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InclusionSettingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("InclusionSettingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventSplitterEntity {
}

pub const EVENTSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventSplitterEntity {
    fn type_info() -> &'static TypeInfo {
        EVENTSPLITTERENTITY_TYPE_INFO
    }
}


pub const EVENTSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("EventSplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventCompareGateEntity {
}

pub const EVENTCOMPAREGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventCompareGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTCOMPAREGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventCompareGateEntity {
    fn type_info() -> &'static TypeInfo {
        EVENTCOMPAREGATEENTITY_TYPE_INFO
    }
}


pub const EVENTCOMPAREGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventCompareGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("EventCompareGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CombatAreaEntity {
}

pub const COMBATAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombatAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMBATAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CombatAreaEntity {
    fn type_info() -> &'static TypeInfo {
        COMBATAREAENTITY_TYPE_INFO
    }
}


pub const COMBATAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombatAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("CombatAreaEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BuildConfigFilterEntity {
}

pub const BUILDCONFIGFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BuildConfigFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BUILDCONFIGFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BuildConfigFilterEntity {
    fn type_info() -> &'static TypeInfo {
        BUILDCONFIGFILTERENTITY_TYPE_INFO
    }
}


pub const BUILDCONFIGFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BuildConfigFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BuildConfigFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AreaProximityEntity {
}

pub const AREAPROXIMITYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaProximityEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AREAPROXIMITYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AreaProximityEntity {
    fn type_info() -> &'static TypeInfo {
        AREAPROXIMITYENTITY_TYPE_INFO
    }
}


pub const AREAPROXIMITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaProximityEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AreaProximityEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaypointParameterEvent {
}

pub const WAYPOINTPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WAYPOINTPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaypointParameterEvent {
    fn type_info() -> &'static TypeInfo {
        WAYPOINTPARAMETEREVENT_TYPE_INFO
    }
}


pub const WAYPOINTPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("WaypointParameterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MoveParameterEvent {
}

pub const MOVEPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYEVENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MOVEPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MoveParameterEvent {
    fn type_info() -> &'static TypeInfo {
        MOVEPARAMETEREVENT_TYPE_INFO
    }
}


pub const MOVEPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("MoveParameterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tool {
}

pub const TOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Tool",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TOOL_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Tool {
    fn type_info() -> &'static TypeInfo {
        TOOL_TYPE_INFO
    }
}


pub const TOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Tool-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("Tool-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SphereCollisionEntity {
}

pub const SPHERECOLLISIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SPHERECOLLISIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SphereCollisionEntity {
    fn type_info() -> &'static TypeInfo {
        SPHERECOLLISIONENTITY_TYPE_INFO
    }
}


pub const SPHERECOLLISIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SphereCollisionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SlowMotionEntity {
}

pub const SLOWMOTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlowMotionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SLOWMOTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SlowMotionEntity {
    fn type_info() -> &'static TypeInfo {
        SLOWMOTIONENTITY_TYPE_INFO
    }
}


pub const SLOWMOTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlowMotionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SlowMotionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OBBCollisionEntity {
}

pub const OBBCOLLISIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OBBCollisionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBBCOLLISIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OBBCollisionEntity {
    fn type_info() -> &'static TypeInfo {
        OBBCOLLISIONENTITY_TYPE_INFO
    }
}


pub const OBBCOLLISIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OBBCollisionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("OBBCollisionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExplosionEntity {
}

pub const EXPLOSIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExplosionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EXPLOSIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ExplosionEntity {
    fn type_info() -> &'static TypeInfo {
        EXPLOSIONENTITY_TYPE_INFO
    }
}


pub const EXPLOSIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExplosionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ExplosionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DifficultyIndexEntity {
}

pub const DIFFICULTYINDEXENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyIndexEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DIFFICULTYINDEXENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DifficultyIndexEntity {
    fn type_info() -> &'static TypeInfo {
        DIFFICULTYINDEXENTITY_TYPE_INFO
    }
}


pub const DIFFICULTYINDEXENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyIndexEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("DifficultyIndexEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PlayerViewHitState {
    pub position: super::core::Vec3,
    pub giver_origin: super::core::Vec3,
    pub damage: f32,
    pub health: f32,
    pub counter: u32,
    pub is_in_vehicle: bool,
    pub is_bullet_damage: bool,
}

pub const PLAYERVIEWHITSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewHitState",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewHitState, position),
            },
            FieldInfoData {
                name: "GiverOrigin",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewHitState, giver_origin),
            },
            FieldInfoData {
                name: "Damage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewHitState, damage),
            },
            FieldInfoData {
                name: "Health",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewHitState, health),
            },
            FieldInfoData {
                name: "Counter",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewHitState, counter),
            },
            FieldInfoData {
                name: "IsInVehicle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewHitState, is_in_vehicle),
            },
            FieldInfoData {
                name: "IsBulletDamage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayerViewHitState, is_bullet_damage),
            },
        ],
    }),
    array_type: Some(PLAYERVIEWHITSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlayerViewHitState {
    fn type_info() -> &'static TypeInfo {
        PLAYERVIEWHITSTATE_TYPE_INFO
    }
}


pub const PLAYERVIEWHITSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewHitState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlayerViewHitState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BucketScoreContainer {
    pub bucket_score: i32,
    pub raw_bucket_score: i32,
}

pub const BUCKETSCORECONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BucketScoreContainer",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BucketScore",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BucketScoreContainer, bucket_score),
            },
            FieldInfoData {
                name: "RawBucketScore",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BucketScoreContainer, raw_bucket_score),
            },
        ],
    }),
    array_type: Some(BUCKETSCORECONTAINER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BucketScoreContainer {
    fn type_info() -> &'static TypeInfo {
        BUCKETSCORECONTAINER_TYPE_INFO
    }
}


pub const BUCKETSCORECONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BucketScoreContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BucketScoreContainer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PlayerScoreDetail {
    #[default]
    PlayerScoreDetail_MaxPlayers = 256,
}

pub const PLAYERSCOREDETAIL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerScoreDetail",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(PLAYERSCOREDETAIL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlayerScoreDetail {
    fn type_info() -> &'static TypeInfo {
        PLAYERSCOREDETAIL_TYPE_INFO
    }
}


pub const PLAYERSCOREDETAIL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerScoreDetail-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlayerScoreDetail-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkCheckLevelInstalledMessage {
}

pub const UINETWORKCHECKLEVELINSTALLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkCheckLevelInstalledMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkCheckLevelInstalledMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKCHECKLEVELINSTALLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkRollCreditsMessage {
}

pub const UINETWORKROLLCREDITSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkRollCreditsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkRollCreditsMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKROLLCREDITSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkStealBodyMessage {
}

pub const UINETWORKSTEALBODYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkStealBodyMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkStealBodyMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKSTEALBODYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkPlayerMissionObjectiveTextMessage {
}

pub const UINETWORKPLAYERMISSIONOBJECTIVETEXTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerMissionObjectiveTextMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerMissionObjectiveTextMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKPLAYERMISSIONOBJECTIVETEXTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkVideoDoneMessage {
}

pub const UINETWORKVIDEODONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkVideoDoneMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkVideoDoneMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKVIDEODONEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkAllowSkipVideoMessage {
}

pub const UINETWORKALLOWSKIPVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkAllowSkipVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkAllowSkipVideoMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKALLOWSKIPVIDEOMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkSkipVideoMessage {
}

pub const UINETWORKSKIPVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkSkipVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkSkipVideoMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKSKIPVIDEOMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkStopVideoMessage {
}

pub const UINETWORKSTOPVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkStopVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkStopVideoMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKSTOPVIDEOMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkPauseVideoMessage {
}

pub const UINETWORKPAUSEVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPauseVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPauseVideoMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKPAUSEVIDEOMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkPauseAllVideosMessage {
}

pub const UINETWORKPAUSEALLVIDEOSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPauseAllVideosMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPauseAllVideosMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKPAUSEALLVIDEOSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkPlayerDeserterTextMessage {
}

pub const UINETWORKPLAYERDESERTERTEXTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerDeserterTextMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerDeserterTextMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKPLAYERDESERTERTEXTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UINetworkPlayerTextMessage {
}

pub const UINETWORKPLAYERTEXTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerTextMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerTextMessage {
    fn type_info() -> &'static TypeInfo {
        UINETWORKPLAYERTEXTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UINetworkTextInfo {
    pub string_id: String,
    pub display_time: f32,
}

pub const UINETWORKTEXTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkTextInfo",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StringId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UINetworkTextInfo, string_id),
            },
            FieldInfoData {
                name: "DisplayTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UINetworkTextInfo, display_time),
            },
        ],
    }),
    array_type: Some(UINETWORKTEXTINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UINetworkTextInfo {
    fn type_info() -> &'static TypeInfo {
        UINETWORKTEXTINFO_TYPE_INFO
    }
}


pub const UINETWORKTEXTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkTextInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("UINetworkTextInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkRemoveDilationMessage {
}

pub const NETWORKREMOVEDILATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkRemoveDilationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkRemoveDilationMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKREMOVEDILATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkAddDilationMessage {
}

pub const NETWORKADDDILATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkAddDilationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkAddDilationMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKADDDILATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkLevelInstalledMessage {
}

pub const NETWORKLEVELINSTALLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLevelInstalledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkLevelInstalledMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKLEVELINSTALLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkMatchReadyStatusChangedMessage {
}

pub const NETWORKMATCHREADYSTATUSCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMatchReadyStatusChangedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMatchReadyStatusChangedMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKMATCHREADYSTATUSCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRestartTimerMessage {
}

pub const SERVERRESTARTTIMERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRestartTimerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRestartTimerMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERRESTARTTIMERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkMetricsLevelCompleteMessage {
}

pub const NETWORKMETRICSLEVELCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMetricsLevelCompleteMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMetricsLevelCompleteMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKMETRICSLEVELCOMPLETEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkMetricsLevelProgressMessage {
}

pub const NETWORKMETRICSLEVELPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMetricsLevelProgressMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMetricsLevelProgressMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKMETRICSLEVELPROGRESSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkMetricsSaveGameSavedMessage {
}

pub const NETWORKMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMetricsSaveGameSavedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMetricsSaveGameSavedMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSetPlayerViewMessage {
}

pub const NETWORKSETPLAYERVIEWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSetPlayerViewMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSetPlayerViewMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSETPLAYERVIEWMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSuicideMessage {
}

pub const NETWORKSUICIDEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSuicideMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSuicideMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSUICIDEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkGameplayContinueMessage {
}

pub const NETWORKGAMEPLAYCONTINUEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkGameplayContinueMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkGameplayContinueMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKGAMEPLAYCONTINUEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerformanceLogMessage {
}

pub const PERFORMANCELOGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceLogMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for PerformanceLogMessage {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCELOGMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PerformanceFpsHistogram {
    pub below5: f32,
    pub below10: f32,
    pub below15: f32,
    pub below20: f32,
    pub below25: f32,
    pub below30: f32,
    pub below35: f32,
    pub below40: f32,
    pub below45: f32,
    pub below50: f32,
    pub below55: f32,
    pub below60: f32,
    pub above60: f32,
}

pub const PERFORMANCEFPSHISTOGRAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceFpsHistogram",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Below5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below5),
            },
            FieldInfoData {
                name: "Below10",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below10),
            },
            FieldInfoData {
                name: "Below15",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below15),
            },
            FieldInfoData {
                name: "Below20",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below20),
            },
            FieldInfoData {
                name: "Below25",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below25),
            },
            FieldInfoData {
                name: "Below30",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below30),
            },
            FieldInfoData {
                name: "Below35",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below35),
            },
            FieldInfoData {
                name: "Below40",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below40),
            },
            FieldInfoData {
                name: "Below45",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below45),
            },
            FieldInfoData {
                name: "Below50",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below50),
            },
            FieldInfoData {
                name: "Below55",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below55),
            },
            FieldInfoData {
                name: "Below60",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, below60),
            },
            FieldInfoData {
                name: "Above60",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerformanceFpsHistogram, above60),
            },
        ],
    }),
    array_type: Some(PERFORMANCEFPSHISTOGRAM_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PerformanceFpsHistogram {
    fn type_info() -> &'static TypeInfo {
        PERFORMANCEFPSHISTOGRAM_TYPE_INFO
    }
}


pub const PERFORMANCEFPSHISTOGRAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceFpsHistogram-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PerformanceFpsHistogram-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkFirstPlayerEnteredMessage {
}

pub const NETWORKFIRSTPLAYERENTEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkFirstPlayerEnteredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkFirstPlayerEnteredMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKFIRSTPLAYERENTEREDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkDifficultyChangedMessage {
}

pub const NETWORKDIFFICULTYCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkDifficultyChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkDifficultyChangedMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKDIFFICULTYCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSettingsMessage {
}

pub const NETWORKSETTINGSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSettingsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSettingsMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSETTINGSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DebugSpawnGameEntityMessage {
}

pub const DEBUGSPAWNGAMEENTITYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugSpawnGameEntityMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for DebugSpawnGameEntityMessage {
    fn type_info() -> &'static TypeInfo {
        DEBUGSPAWNGAMEENTITYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkMovePlayerMessage {
}

pub const NETWORKMOVEPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMovePlayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for NetworkMovePlayerMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKMOVEPLAYERMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkJuiceSessionMessage {
}

pub const NETWORKJUICESESSIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkJuiceSessionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkJuiceSessionMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKJUICESESSIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSelectTeamMessage {
}

pub const NETWORKSELECTTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSelectTeamMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSelectTeamMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSELECTTEAMMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkOnPlayerSpawnedMessage {
}

pub const NETWORKONPLAYERSPAWNEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkOnPlayerSpawnedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkOnPlayerSpawnedMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKONPLAYERSPAWNEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSelectSpawnGroupMessage {
}

pub const NETWORKSELECTSPAWNGROUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSelectSpawnGroupMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSelectSpawnGroupMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSELECTSPAWNGROUPMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSpawnVehicleCustomizationMessage {
}

pub const NETWORKSPAWNVEHICLECUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnVehicleCustomizationMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnVehicleCustomizationMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSPAWNVEHICLECUSTOMIZATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkUnSpawnCustomizationMessage {
}

pub const NETWORKUNSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkUnSpawnCustomizationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkUnSpawnCustomizationMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKUNSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSpawnCustomizationMessage {
}

pub const NETWORKSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnCustomizationMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnCustomizationMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSpawnOnSelectedMessage {
}

pub const NETWORKSPAWNONSELECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnOnSelectedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnOnSelectedMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSPAWNONSELECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSpawnHereMessage {
}

pub const NETWORKSPAWNHEREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnHereMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for NetworkSpawnHereMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSPAWNHEREMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkSpawnMessage {
}

pub const NETWORKSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkRequestLoadLevelMessage {
}

pub const NETWORKREQUESTLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkRequestLoadLevelMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkRequestLoadLevelMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKREQUESTLOADLEVELMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkScreenFadeMessage {
}

pub const NETWORKSCREENFADEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkScreenFadeMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkScreenFadeMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKSCREENFADEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelConsoleSetActiveHealthStateMessage {
}

pub const LEVELCONSOLESETACTIVEHEALTHSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelConsoleSetActiveHealthStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LevelConsoleSetActiveHealthStateMessage {
    fn type_info() -> &'static TypeInfo {
        LEVELCONSOLESETACTIVEHEALTHSTATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkPerformanceProfileMessage {
}

pub const NETWORKPERFORMANCEPROFILEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerformanceProfileMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkPerformanceProfileMessage {
    fn type_info() -> &'static TypeInfo {
        NETWORKPERFORMANCEPROFILEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SyncedSequenceStateChangeMessageBase {
}

pub const SYNCEDSEQUENCESTATECHANGEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedSequenceStateChangeMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for SyncedSequenceStateChangeMessageBase {
    fn type_info() -> &'static TypeInfo {
        SYNCEDSEQUENCESTATECHANGEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JuiceSoldierRagdollDeactivateMessage {
}

pub const JUICESOLDIERRAGDOLLDEACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JuiceSoldierRagdollDeactivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for JuiceSoldierRagdollDeactivateMessage {
    fn type_info() -> &'static TypeInfo {
        JUICESOLDIERRAGDOLLDEACTIVATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JuiceSoldierRagdollActivateMessage {
}

pub const JUICESOLDIERRAGDOLLACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JuiceSoldierRagdollActivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for JuiceSoldierRagdollActivateMessage {
    fn type_info() -> &'static TypeInfo {
        JUICESOLDIERRAGDOLLACTIVATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIClientBridgeDynamicModelEntityOnUnspawnMessage {
}

pub const AICLIENTBRIDGEDYNAMICMODELENTITYONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIClientBridgeDynamicModelEntityOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIClientBridgeDynamicModelEntityOnUnspawnMessage {
    fn type_info() -> &'static TypeInfo {
        AICLIENTBRIDGEDYNAMICMODELENTITYONUNSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIClientBridgeDynamicModelEntityOnSpawnMessage {
}

pub const AICLIENTBRIDGEDYNAMICMODELENTITYONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIClientBridgeDynamicModelEntityOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIClientBridgeDynamicModelEntityOnSpawnMessage {
    fn type_info() -> &'static TypeInfo {
        AICLIENTBRIDGEDYNAMICMODELENTITYONSPAWNMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WantedAimState {
    #[default]
    WantedAimState_NotAiming = 0,
    WantedAimState_Aiming = 1,
    WantedAimState_ForcedAiming = 2,
}

pub const WANTEDAIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WantedAimState",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(WANTEDAIMSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WantedAimState {
    fn type_info() -> &'static TypeInfo {
        WANTEDAIMSTATE_TYPE_INFO
    }
}


pub const WANTEDAIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WantedAimState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("WantedAimState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AimTargetId {
    #[default]
    AimTargetId_NoTarget = 0,
    AimTargetId_Position = 1,
    AimTargetId_Entity1 = 2,
    AimTargetId_Entity2 = 3,
}

pub const AIMTARGETID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimTargetId",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(AIMTARGETID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AimTargetId {
    fn type_info() -> &'static TypeInfo {
        AIMTARGETID_TYPE_INFO
    }
}


pub const AIMTARGETID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimTargetId-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AimTargetId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SharedLockObserverEntity {
}

pub const SHAREDLOCKOBSERVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockObserverEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHAREDLOCKBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKOBSERVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockObserverEntity {
    fn type_info() -> &'static TypeInfo {
        SHAREDLOCKOBSERVERENTITY_TYPE_INFO
    }
}


pub const SHAREDLOCKOBSERVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockObserverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockObserverEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SharedLockGateEntity {
}

pub const SHAREDLOCKGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHAREDLOCKBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockGateEntity {
    fn type_info() -> &'static TypeInfo {
        SHAREDLOCKGATEENTITY_TYPE_INFO
    }
}


pub const SHAREDLOCKGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockGateEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SharedLockControllerEntity {
}

pub const SHAREDLOCKCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHAREDLOCKBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockControllerEntity {
    fn type_info() -> &'static TypeInfo {
        SHAREDLOCKCONTROLLERENTITY_TYPE_INFO
    }
}


pub const SHAREDLOCKCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockControllerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SharedLockBaseEntity {
}

pub const SHAREDLOCKBASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockBaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKBASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockBaseEntity {
    fn type_info() -> &'static TypeInfo {
        SHAREDLOCKBASEENTITY_TYPE_INFO
    }
}


pub const SHAREDLOCKBASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockBaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockBaseEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntersectionTriggerEntity {
}

pub const INTERSECTIONTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntersectionTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTERSECTIONTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntersectionTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        INTERSECTIONTRIGGERENTITY_TYPE_INFO
    }
}


pub const INTERSECTIONTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntersectionTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("IntersectionTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UserSpawnContext {
}

pub const USERSPAWNCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserSpawnContext",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(USERSPAWNCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UserSpawnContext {
    fn type_info() -> &'static TypeInfo {
        USERSPAWNCONTEXT_TYPE_INFO
    }
}


pub const USERSPAWNCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserSpawnContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("UserSpawnContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayerIteratorEntity {
}

pub const PLAYERITERATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerIteratorEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PLAYERITERATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlayerIteratorEntity {
    fn type_info() -> &'static TypeInfo {
        PLAYERITERATORENTITY_TYPE_INFO
    }
}


pub const PLAYERITERATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerIteratorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlayerIteratorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlatformSplitterEntity {
}

pub const PLATFORMSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PLATFORMSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlatformSplitterEntity {
    fn type_info() -> &'static TypeInfo {
        PLATFORMSPLITTERENTITY_TYPE_INFO
    }
}


pub const PLATFORMSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlatformSplitterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoggingEntity {
}

pub const LOGGINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoggingEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOGGINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LoggingEntity {
    fn type_info() -> &'static TypeInfo {
        LOGGINGENTITY_TYPE_INFO
    }
}


pub const LOGGINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoggingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("LoggingEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeltaViewerTableEntity {
}

pub const DELTAVIEWERTABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaViewerTableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DELTAVIEWERTABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DeltaViewerTableEntity {
    fn type_info() -> &'static TypeInfo {
        DELTAVIEWERTABLEENTITY_TYPE_INFO
    }
}


pub const DELTAVIEWERTABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaViewerTableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("DeltaViewerTableEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConsoleCommandTriggerEntity {
}

pub const CONSOLECOMMANDTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONSOLECOMMANDTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConsoleCommandTriggerEntity {
    fn type_info() -> &'static TypeInfo {
        CONSOLECOMMANDTRIGGERENTITY_TYPE_INFO
    }
}


pub const CONSOLECOMMANDTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ConsoleCommandTriggerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConsoleCommandEntity {
}

pub const CONSOLECOMMANDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONSOLECOMMANDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConsoleCommandEntity {
    fn type_info() -> &'static TypeInfo {
        CONSOLECOMMANDENTITY_TYPE_INFO
    }
}


pub const CONSOLECOMMANDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ConsoleCommandEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LicenseeGameWorldRayCaster {
}

pub const LICENSEEGAMEWORLDRAYCASTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseeGameWorldRayCaster",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEWORLDRAYCASTER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LICENSEEGAMEWORLDRAYCASTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LicenseeGameWorldRayCaster {
    fn type_info() -> &'static TypeInfo {
        LICENSEEGAMEWORLDRAYCASTER_TYPE_INFO
    }
}


pub const LICENSEEGAMEWORLDRAYCASTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseeGameWorldRayCaster-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("LicenseeGameWorldRayCaster-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelStatusEntity {
}

pub const SUBLEVELSTATUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelStatusEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELSTATUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelStatusEntity {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELSTATUSENTITY_TYPE_INFO
    }
}


pub const SUBLEVELSTATUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelStatusEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SubLevelStatusEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelPreloadEntity {
}

pub const SUBLEVELPRELOADENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELPRELOADENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelPreloadEntity {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELPRELOADENTITY_TYPE_INFO
    }
}


pub const SUBLEVELPRELOADENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SubLevelPreloadEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubLevelCollectionEntityBase {
}

pub const SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELCOLLECTIONENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelCollectionEntityBase {
    fn type_info() -> &'static TypeInfo {
        SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO
    }
}


pub const SUBLEVELCOLLECTIONENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SubLevelCollectionEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleStatusEntity {
}

pub const BLUEPRINTBUNDLESTATUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLESTATUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintBundleStatusEntity {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLESTATUSENTITY_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLESTATUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BlueprintBundleStatusEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBlueprintBundleEntity {
}

pub const SERVERBLUEPRINTBUNDLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintBundleEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBLUEPRINTBUNDLEENTITY_TYPE_INFO
    }
}


pub const SERVERBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ServerBlueprintBundleEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlueprintBundleEntity {
}

pub const CLIENTBLUEPRINTBUNDLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintBundleEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLUEPRINTBUNDLEENTITY_TYPE_INFO
    }
}


pub const CLIENTBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ClientBlueprintBundleEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleEntityBase {
}

pub const BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintBundleEntityBase {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLEENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BlueprintBundleEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerBlueprintBundleCollectionEntity {
}

pub const SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleCollectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintBundleCollectionEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO
    }
}


pub const SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleCollectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ServerBlueprintBundleCollectionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientBlueprintBundleCollectionEntity {
}

pub const CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleCollectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintBundleCollectionEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO
    }
}


pub const CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleCollectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ClientBlueprintBundleCollectionEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlueprintBundleCollectionEntityBase {
}

pub const BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintBundleCollectionEntityBase {
    fn type_info() -> &'static TypeInfo {
        BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO
    }
}


pub const BLUEPRINTBUNDLECOLLECTIONENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BlueprintBundleCollectionEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TestStaticModelGroupEntityData {
    pub realm: super::core::Realm,
    pub expected_material: super::entity::MaterialDecl,
}

pub const TESTSTATICMODELGROUPENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestStaticModelGroupEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TestStaticModelGroupEntityData, realm),
            },
            FieldInfoData {
                name: "ExpectedMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(TestStaticModelGroupEntityData, expected_material),
            },
        ],
    }),
    array_type: Some(TESTSTATICMODELGROUPENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TestStaticModelGroupEntityData {
    fn type_info() -> &'static TypeInfo {
        TESTSTATICMODELGROUPENTITYDATA_TYPE_INFO
    }
}


pub const TESTSTATICMODELGROUPENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestStaticModelGroupEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("TestStaticModelGroupEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TestRayCastEntityData {
    pub realm: super::core::Realm,
    pub asynchronous: bool,
    pub ray_start: super::core::Vec3,
    pub ray_end: super::core::Vec3,
    pub output_index: u32,
    pub check_detail_mesh: bool,
    pub check_normal: bool,
    pub check_material: bool,
    pub check_part_index: bool,
    pub check_water: bool,
    pub check_terrain: bool,
    pub check_ragdoll: bool,
    pub check_character: bool,
    pub check_group: bool,
    pub penetrate_penetrable: bool,
    pub penetrate_client_destructible: bool,
    pub penetrate_bashable: bool,
    pub penetrate_see_through: bool,
    pub penetrate_no_collision_response: bool,
    pub penetrate_no_collision_response_combined: bool,
    pub penetrate_attachable: bool,
    pub expected_position: super::core::Vec3,
    pub expected_normal: super::core::Vec3,
    pub expected_part_index: u32,
    pub expected_material: super::entity::MaterialDecl,
    pub tolerance: f32,
}

pub const TESTRAYCASTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestRayCastEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, realm),
            },
            FieldInfoData {
                name: "Asynchronous",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, asynchronous),
            },
            FieldInfoData {
                name: "RayStart",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, ray_start),
            },
            FieldInfoData {
                name: "RayEnd",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, ray_end),
            },
            FieldInfoData {
                name: "OutputIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, output_index),
            },
            FieldInfoData {
                name: "CheckDetailMesh",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_detail_mesh),
            },
            FieldInfoData {
                name: "CheckNormal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_normal),
            },
            FieldInfoData {
                name: "CheckMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_material),
            },
            FieldInfoData {
                name: "CheckPartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_part_index),
            },
            FieldInfoData {
                name: "CheckWater",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_water),
            },
            FieldInfoData {
                name: "CheckTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_terrain),
            },
            FieldInfoData {
                name: "CheckRagdoll",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_ragdoll),
            },
            FieldInfoData {
                name: "CheckCharacter",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_character),
            },
            FieldInfoData {
                name: "CheckGroup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, check_group),
            },
            FieldInfoData {
                name: "PenetratePenetrable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_penetrable),
            },
            FieldInfoData {
                name: "PenetrateClientDestructible",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_client_destructible),
            },
            FieldInfoData {
                name: "PenetrateBashable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_bashable),
            },
            FieldInfoData {
                name: "PenetrateSeeThrough",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_see_through),
            },
            FieldInfoData {
                name: "PenetrateNoCollisionResponse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_no_collision_response),
            },
            FieldInfoData {
                name: "PenetrateNoCollisionResponseCombined",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_no_collision_response_combined),
            },
            FieldInfoData {
                name: "PenetrateAttachable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_attachable),
            },
            FieldInfoData {
                name: "ExpectedPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, expected_position),
            },
            FieldInfoData {
                name: "ExpectedNormal",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, expected_normal),
            },
            FieldInfoData {
                name: "ExpectedPartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, expected_part_index),
            },
            FieldInfoData {
                name: "ExpectedMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, expected_material),
            },
            FieldInfoData {
                name: "Tolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TestRayCastEntityData, tolerance),
            },
        ],
    }),
    array_type: Some(TESTRAYCASTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TestRayCastEntityData {
    fn type_info() -> &'static TypeInfo {
        TESTRAYCASTENTITYDATA_TYPE_INFO
    }
}


pub const TESTRAYCASTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestRayCastEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("TestRayCastEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AdministrationRestrictionLevel {
    #[default]
    AdministrationRestrictionLevel_Zero = 0,
    AdministrationRestrictionLevel_One = 1,
    AdministrationRestrictionLevel_Two = 2,
    AdministrationRestrictionLevel_Three = 3,
    AdministrationRestrictionLevel_Count = 4,
}

pub const ADMINISTRATIONRESTRICTIONLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationRestrictionLevel",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(ADMINISTRATIONRESTRICTIONLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AdministrationRestrictionLevel {
    fn type_info() -> &'static TypeInfo {
        ADMINISTRATIONRESTRICTIONLEVEL_TYPE_INFO
    }
}


pub const ADMINISTRATIONRESTRICTIONLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationRestrictionLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AdministrationRestrictionLevel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AdministrationEventType {
    #[default]
    AdministrationEventType_Add = 0,
    AdministrationEventType_Remove = 1,
    AdministrationEventType_Clear = 2,
    AdministrationEventType_List = 3,
    AdministrationEventType_Load = 4,
    AdministrationEventType_Save = 5,
}

pub const ADMINISTRATIONEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationEventType",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(ADMINISTRATIONEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AdministrationEventType {
    fn type_info() -> &'static TypeInfo {
        ADMINISTRATIONEVENTTYPE_TYPE_INFO
    }
}


pub const ADMINISTRATIONEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AdministrationEventType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdministrationPasswordMessage {
}

pub const SERVERADMINISTRATIONPASSWORDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationPasswordMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationPasswordMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINISTRATIONPASSWORDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdministrationEventsEnabledMessage {
}

pub const SERVERADMINISTRATIONEVENTSENABLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationEventsEnabledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationEventsEnabledMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINISTRATIONEVENTSENABLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdministrationQuitMessage {
}

pub const SERVERADMINISTRATIONQUITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationQuitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationQuitMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINISTRATIONQUITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdministrationLoginMessage {
}

pub const SERVERADMINISTRATIONLOGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationLoginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationLoginMessage {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINISTRATIONLOGINMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAdministrationPacketMessageBase {
}

pub const SERVERADMINISTRATIONPACKETMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationPacketMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerAdministrationPacketMessageBase {
    fn type_info() -> &'static TypeInfo {
        SERVERADMINISTRATIONPACKETMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameModule {
}

pub const GAMEMODULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModule",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RUNTIMEMODULE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMEMODULE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameModule {
    fn type_info() -> &'static TypeInfo {
        GAMEMODULE_TYPE_INFO
    }
}


pub const GAMEMODULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModule-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("GameModule-Array"),
    array_type: None,
    alignment: 8,
};


