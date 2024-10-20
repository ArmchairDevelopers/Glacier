use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct AnimationPoseLayerTrack {
    pub _glacier_base: super::timeline::GroupTrack,
}

pub trait AnimationPoseLayerTrackTrait: super::timeline::GroupTrackTrait {
}

impl AnimationPoseLayerTrackTrait for AnimationPoseLayerTrack {
}

impl super::timeline::GroupTrackTrait for AnimationPoseLayerTrack {
}

impl super::timeline::TimelineTrackTrait for AnimationPoseLayerTrack {
}

pub static ANIMATIONPOSELAYERTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseLayerTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::GROUPTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimationPoseLayerTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANIMATIONPOSELAYERTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnimationPoseLayerTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATIONPOSELAYERTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANIMATIONPOSELAYERTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseLayerTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AnimationPoseLayerTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTControlTrack {
    pub _glacier_base: super::timeline::LinkTrack,
}

pub trait ANTControlTrackTrait: super::timeline::LinkTrackTrait {
}

impl ANTControlTrackTrait for ANTControlTrack {
}

impl super::timeline::LinkTrackTrait for ANTControlTrack {
}

impl super::timeline::TimelineTrackTrait for ANTControlTrack {
}

pub static ANTCONTROLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControlTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTControlTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTCONTROLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTControlTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANTCONTROLTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTCONTROLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTControlTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTControlTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTBoneTrack {
    pub _glacier_base: super::timeline::LayeredTransformTrack,
}

pub trait ANTBoneTrackTrait: super::timeline::LayeredTransformTrackTrait {
}

impl ANTBoneTrackTrait for ANTBoneTrack {
}

impl super::timeline::LayeredTransformTrackTrait for ANTBoneTrack {
}

impl super::timeline::IPropertyTrackTrait for ANTBoneTrack {
}

impl super::timeline::TimelineTrackTrait for ANTBoneTrack {
}

pub static ANTBONETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::LAYEREDTRANSFORMTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTBoneTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTBONETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTBoneTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANTBONETRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTBONETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTBoneTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTBoneAnimationTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait ANTBoneAnimationTrackTrait: super::timeline::TimelineTrackTrait {
}

impl ANTBoneAnimationTrackTrait for ANTBoneAnimationTrack {
}

impl super::timeline::TimelineTrackTrait for ANTBoneAnimationTrack {
}

pub static ANTBONEANIMATIONTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneAnimationTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTBoneAnimationTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTBONEANIMATIONTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTBoneAnimationTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANTBONEANIMATIONTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTBONEANIMATIONTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneAnimationTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTBoneAnimationTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AnimationPoseTrack {
    pub _glacier_base: super::timeline::LinkTrack,
}

pub trait AnimationPoseTrackTrait: super::timeline::LinkTrackTrait {
}

impl AnimationPoseTrackTrait for AnimationPoseTrack {
}

impl super::timeline::LinkTrackTrait for AnimationPoseTrack {
}

impl super::timeline::TimelineTrackTrait for AnimationPoseTrack {
}

pub static ANIMATIONPOSETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AnimationPoseTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANIMATIONPOSETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AnimationPoseTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANIMATIONPOSETRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANIMATIONPOSETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AnimationPoseTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AnimationPoseTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RecordedBlobResource {
}

pub trait RecordedBlobResourceTrait: TypeObject {
}

impl RecordedBlobResourceTrait for RecordedBlobResource {
}

pub static RECORDEDBLOBRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordedBlobResource",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RecordedBlobResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RECORDEDBLOBRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RecordedBlobResource {
    fn type_info(&self) -> &'static TypeInfo {
        RECORDEDBLOBRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RECORDEDBLOBRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordedBlobResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("RecordedBlobResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTBoneTransformLayerTrack {
    pub _glacier_base: super::timeline::SimpleTransformLayer,
}

pub trait ANTBoneTransformLayerTrackTrait: super::timeline::SimpleTransformLayerTrait {
}

impl ANTBoneTransformLayerTrackTrait for ANTBoneTransformLayerTrack {
}

impl super::timeline::SimpleTransformLayerTrait for ANTBoneTransformLayerTrack {
}

impl super::timeline::TransformLayerTrait for ANTBoneTransformLayerTrack {
}

impl super::timeline::TimelineTrackTrait for ANTBoneTransformLayerTrack {
}

pub static ANTBONETRANSFORMLAYERTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTransformLayerTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::SIMPLETRANSFORMLAYER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTBoneTransformLayerTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTBONETRANSFORMLAYERTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTBoneTransformLayerTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANTBONETRANSFORMLAYERTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTBONETRANSFORMLAYERTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTBoneTransformLayerTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTBoneTransformLayerTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VehicleExitPointComponent {
    pub _glacier_base: super::gameplay_sim::GameComponent,
}

pub trait VehicleExitPointComponentTrait: super::gameplay_sim::GameComponentTrait {
}

impl VehicleExitPointComponentTrait for VehicleExitPointComponent {
}

impl super::gameplay_sim::GameComponentTrait for VehicleExitPointComponent {
}

impl super::entity::ComponentTrait for VehicleExitPointComponent {
}

impl super::entity::EntityBusPeerTrait for VehicleExitPointComponent {
}

pub static VEHICLEEXITPOINTCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleExitPointComponent",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMECOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleExitPointComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEHICLEEXITPOINTCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VehicleExitPointComponent {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEEXITPOINTCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEHICLEEXITPOINTCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleExitPointComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("VehicleExitPointComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsDrivenAnimationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait PhysicsDrivenAnimationEntityTrait: super::entity::EntityTrait {
}

impl PhysicsDrivenAnimationEntityTrait for PhysicsDrivenAnimationEntity {
}

impl super::entity::EntityTrait for PhysicsDrivenAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for PhysicsDrivenAnimationEntity {
}

pub static PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDrivenAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsDrivenAnimationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PhysicsDrivenAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSDRIVENANIMATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PHYSICSDRIVENANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsDrivenAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PhysicsDrivenAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ModelAnimationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ModelAnimationEntityTrait: super::entity::EntityTrait {
}

impl ModelAnimationEntityTrait for ModelAnimationEntity {
}

impl super::entity::EntityTrait for ModelAnimationEntity {
}

impl super::entity::EntityBusPeerTrait for ModelAnimationEntity {
}

pub static MODELANIMATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelAnimationEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MODELANIMATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ModelAnimationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        MODELANIMATIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELANIMATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelAnimationEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ModelAnimationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTSlotTrack {
    pub _glacier_base: super::timeline::IPropertyTrack,
}

pub trait ANTSlotTrackTrait: super::timeline::IPropertyTrackTrait {
}

impl ANTSlotTrackTrait for ANTSlotTrack {
}

impl super::timeline::IPropertyTrackTrait for ANTSlotTrack {
}

impl super::timeline::TimelineTrackTrait for ANTSlotTrack {
}

pub static ANTSLOTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSlotTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::IPROPERTYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTSlotTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTSLOTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTSlotTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANTSLOTTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTSLOTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSlotTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTSlotTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTSignalTrack {
    pub _glacier_base: super::timeline::LinkTrack,
}

pub trait ANTSignalTrackTrait: super::timeline::LinkTrackTrait {
}

impl ANTSignalTrackTrait for ANTSignalTrack {
}

impl super::timeline::LinkTrackTrait for ANTSignalTrack {
}

impl super::timeline::TimelineTrackTrait for ANTSignalTrack {
}

pub static ANTSIGNALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSignalTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTSignalTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTSIGNALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTSignalTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANTSIGNALTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTSIGNALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTSignalTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTSignalTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ANTEvalTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait ANTEvalTrackTrait: super::timeline::TimelineTrackTrait {
}

impl ANTEvalTrackTrait for ANTEvalTrack {
}

impl super::timeline::TimelineTrackTrait for ANTEvalTrack {
}

pub static ANTEVALTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvalTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ANTEvalTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ANTEVALTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ANTEvalTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ANTEVALTRACK_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ANTEVALTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ANTEvalTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ANTEvalTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObjectAreaQueryEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ObjectAreaQueryEntityTrait: super::entity::EntityTrait {
}

impl ObjectAreaQueryEntityTrait for ObjectAreaQueryEntity {
}

impl super::entity::EntityTrait for ObjectAreaQueryEntity {
}

impl super::entity::EntityBusPeerTrait for ObjectAreaQueryEntity {
}

pub static OBJECTAREAQUERYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaQueryEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObjectAreaQueryEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectAreaQueryEntity {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTAREAQUERYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBJECTAREAQUERYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectAreaQueryEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ObjectAreaQueryEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BillboardTransformEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait BillboardTransformEntityTrait: super::entity::SpatialEntityTrait {
}

impl BillboardTransformEntityTrait for BillboardTransformEntity {
}

impl super::entity::SpatialEntityTrait for BillboardTransformEntity {
}

impl super::entity::EntityTrait for BillboardTransformEntity {
}

impl super::entity::EntityBusPeerTrait for BillboardTransformEntity {
}

pub static BILLBOARDTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BillboardTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BillboardTransformEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BILLBOARDTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BillboardTransformEntity {
    fn type_info(&self) -> &'static TypeInfo {
        BILLBOARDTRANSFORMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BILLBOARDTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BillboardTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BillboardTransformEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaterDepthEntity {
    pub _glacier_base: super::entity::SpatialEntity,
}

pub trait WaterDepthEntityTrait: super::entity::SpatialEntityTrait {
}

impl WaterDepthEntityTrait for WaterDepthEntity {
}

impl super::entity::SpatialEntityTrait for WaterDepthEntity {
}

impl super::entity::EntityTrait for WaterDepthEntity {
}

impl super::entity::EntityBusPeerTrait for WaterDepthEntity {
}

pub static WATERDEPTHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDepthEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterDepthEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERDEPTHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaterDepthEntity {
    fn type_info(&self) -> &'static TypeInfo {
        WATERDEPTHENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WATERDEPTHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterDepthEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("WaterDepthEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransformSnapToGroundEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait TransformSnapToGroundEntityTrait: super::entity::EntityTrait {
}

impl TransformSnapToGroundEntityTrait for TransformSnapToGroundEntity {
}

impl super::entity::EntityTrait for TransformSnapToGroundEntity {
}

impl super::entity::EntityBusPeerTrait for TransformSnapToGroundEntity {
}

pub static TRANSFORMSNAPTOGROUNDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSnapToGroundEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformSnapToGroundEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMSNAPTOGROUNDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformSnapToGroundEntity {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMSNAPTOGROUNDENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TRANSFORMSNAPTOGROUNDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformSnapToGroundEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("TransformSnapToGroundEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InclusionSettingEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait InclusionSettingEntityTrait: super::entity::EntityTrait {
}

impl InclusionSettingEntityTrait for InclusionSettingEntity {
}

impl super::entity::EntityTrait for InclusionSettingEntity {
}

impl super::entity::EntityBusPeerTrait for InclusionSettingEntity {
}

pub static INCLUSIONSETTINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InclusionSettingEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InclusionSettingEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INCLUSIONSETTINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InclusionSettingEntity {
    fn type_info(&self) -> &'static TypeInfo {
        INCLUSIONSETTINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INCLUSIONSETTINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InclusionSettingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("InclusionSettingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventSplitterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait EventSplitterEntityTrait: super::entity::EntityTrait {
}

impl EventSplitterEntityTrait for EventSplitterEntity {
}

impl super::entity::EntityTrait for EventSplitterEntity {
}

impl super::entity::EntityBusPeerTrait for EventSplitterEntity {
}

pub static EVENTSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventSplitterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EVENTSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventSplitterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTSPLITTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EVENTSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("EventSplitterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventCompareGateEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait EventCompareGateEntityTrait: super::entity::EntityTrait {
}

impl EventCompareGateEntityTrait for EventCompareGateEntity {
}

impl super::entity::EntityTrait for EventCompareGateEntity {
}

impl super::entity::EntityBusPeerTrait for EventCompareGateEntity {
}

pub static EVENTCOMPAREGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventCompareGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventCompareGateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EVENTCOMPAREGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventCompareGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTCOMPAREGATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EVENTCOMPAREGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventCompareGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("EventCompareGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CombatAreaEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CombatAreaEntityTrait: super::entity::EntityTrait {
}

impl CombatAreaEntityTrait for CombatAreaEntity {
}

impl super::entity::EntityTrait for CombatAreaEntity {
}

impl super::entity::EntityBusPeerTrait for CombatAreaEntity {
}

pub static COMBATAREAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombatAreaEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CombatAreaEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMBATAREAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CombatAreaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        COMBATAREAENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COMBATAREAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CombatAreaEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("CombatAreaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BuildConfigFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait BuildConfigFilterEntityTrait: super::entity::EntityTrait {
}

impl BuildConfigFilterEntityTrait for BuildConfigFilterEntity {
}

impl super::entity::EntityTrait for BuildConfigFilterEntity {
}

impl super::entity::EntityBusPeerTrait for BuildConfigFilterEntity {
}

pub static BUILDCONFIGFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BuildConfigFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BuildConfigFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BUILDCONFIGFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BuildConfigFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        BUILDCONFIGFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BUILDCONFIGFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BuildConfigFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BuildConfigFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AreaProximityEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AreaProximityEntityTrait: super::entity::EntityTrait {
}

impl AreaProximityEntityTrait for AreaProximityEntity {
}

impl super::entity::EntityTrait for AreaProximityEntity {
}

impl super::entity::EntityBusPeerTrait for AreaProximityEntity {
}

pub static AREAPROXIMITYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaProximityEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AreaProximityEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AREAPROXIMITYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AreaProximityEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AREAPROXIMITYENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AREAPROXIMITYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AreaProximityEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AreaProximityEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WaypointParameterEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait WaypointParameterEventTrait: super::entity::EntityEventTrait {
}

impl WaypointParameterEventTrait for WaypointParameterEvent {
}

impl super::entity::EntityEventTrait for WaypointParameterEvent {
}

pub static WAYPOINTPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaypointParameterEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WAYPOINTPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for WaypointParameterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        WAYPOINTPARAMETEREVENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WAYPOINTPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("WaypointParameterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoveParameterEvent {
    pub _glacier_base: super::entity::EntityEvent,
}

pub trait MoveParameterEventTrait: super::entity::EntityEventTrait {
}

impl MoveParameterEventTrait for MoveParameterEvent {
}

impl super::entity::EntityEventTrait for MoveParameterEvent {
}

pub static MOVEPARAMETEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveParameterEvent",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoveParameterEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MOVEPARAMETEREVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MoveParameterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        MOVEPARAMETEREVENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVEPARAMETEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoveParameterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("MoveParameterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Tool {
}

pub trait ToolTrait: TypeObject {
}

impl ToolTrait for Tool {
}

pub static TOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Tool",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Tool as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TOOL_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Tool {
    fn type_info(&self) -> &'static TypeInfo {
        TOOL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Tool-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("Tool"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SphereCollisionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SphereCollisionEntityTrait: super::entity::EntityTrait {
}

impl SphereCollisionEntityTrait for SphereCollisionEntity {
}

impl super::entity::EntityTrait for SphereCollisionEntity {
}

impl super::entity::EntityBusPeerTrait for SphereCollisionEntity {
}

pub static SPHERECOLLISIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereCollisionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SPHERECOLLISIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SphereCollisionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SPHERECOLLISIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPHERECOLLISIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereCollisionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SphereCollisionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SlowMotionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SlowMotionEntityTrait: super::entity::EntityTrait {
}

impl SlowMotionEntityTrait for SlowMotionEntity {
}

impl super::entity::EntityTrait for SlowMotionEntity {
}

impl super::entity::EntityBusPeerTrait for SlowMotionEntity {
}

pub static SLOWMOTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlowMotionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SlowMotionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SLOWMOTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SlowMotionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SLOWMOTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SLOWMOTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlowMotionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SlowMotionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OBBCollisionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait OBBCollisionEntityTrait: super::entity::EntityTrait {
}

impl OBBCollisionEntityTrait for OBBCollisionEntity {
}

impl super::entity::EntityTrait for OBBCollisionEntity {
}

impl super::entity::EntityBusPeerTrait for OBBCollisionEntity {
}

pub static OBBCOLLISIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OBBCollisionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OBBCollisionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OBBCOLLISIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OBBCollisionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        OBBCOLLISIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBBCOLLISIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OBBCollisionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("OBBCollisionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExplosionEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ExplosionEntityTrait: super::entity::EntityTrait {
}

impl ExplosionEntityTrait for ExplosionEntity {
}

impl super::entity::EntityTrait for ExplosionEntity {
}

impl super::entity::EntityBusPeerTrait for ExplosionEntity {
}

pub static EXPLOSIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExplosionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExplosionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EXPLOSIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ExplosionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        EXPLOSIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EXPLOSIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExplosionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ExplosionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DifficultyIndexEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DifficultyIndexEntityTrait: super::entity::EntityTrait {
}

impl DifficultyIndexEntityTrait for DifficultyIndexEntity {
}

impl super::entity::EntityTrait for DifficultyIndexEntity {
}

impl super::entity::EntityBusPeerTrait for DifficultyIndexEntity {
}

pub static DIFFICULTYINDEXENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyIndexEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DifficultyIndexEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DIFFICULTYINDEXENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DifficultyIndexEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DIFFICULTYINDEXENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DIFFICULTYINDEXENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DifficultyIndexEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("DifficultyIndexEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlayerViewHitState {
    pub position: super::core::Vec3,
    pub giver_origin: super::core::Vec3,
    pub damage: f32,
    pub health: f32,
    pub counter: u32,
    pub is_in_vehicle: bool,
    pub is_bullet_damage: bool,
}

pub trait PlayerViewHitStateTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn giver_origin(&self) -> &super::core::Vec3;
    fn damage(&self) -> &f32;
    fn health(&self) -> &f32;
    fn counter(&self) -> &u32;
    fn is_in_vehicle(&self) -> &bool;
    fn is_bullet_damage(&self) -> &bool;
}

impl PlayerViewHitStateTrait for PlayerViewHitState {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn giver_origin(&self) -> &super::core::Vec3 {
        &self.giver_origin
    }
    fn damage(&self) -> &f32 {
        &self.damage
    }
    fn health(&self) -> &f32 {
        &self.health
    }
    fn counter(&self) -> &u32 {
        &self.counter
    }
    fn is_in_vehicle(&self) -> &bool {
        &self.is_in_vehicle
    }
    fn is_bullet_damage(&self) -> &bool {
        &self.is_bullet_damage
    }
}

pub static PLAYERVIEWHITSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewHitState",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayerViewHitState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PlayerViewHitState, position),
            },
            FieldInfoData {
                name: "GiverOrigin",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(PlayerViewHitState, giver_origin),
            },
            FieldInfoData {
                name: "Damage",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlayerViewHitState, damage),
            },
            FieldInfoData {
                name: "Health",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlayerViewHitState, health),
            },
            FieldInfoData {
                name: "Counter",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PlayerViewHitState, counter),
            },
            FieldInfoData {
                name: "IsInVehicle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayerViewHitState, is_in_vehicle),
            },
            FieldInfoData {
                name: "IsBulletDamage",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayerViewHitState, is_bullet_damage),
            },
        ],
    }),
    array_type: Some(PLAYERVIEWHITSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PlayerViewHitState {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYERVIEWHITSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLAYERVIEWHITSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerViewHitState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlayerViewHitState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BucketScoreContainer {
    pub bucket_score: i32,
    pub raw_bucket_score: i32,
}

pub trait BucketScoreContainerTrait: TypeObject {
    fn bucket_score(&self) -> &i32;
    fn raw_bucket_score(&self) -> &i32;
}

impl BucketScoreContainerTrait for BucketScoreContainer {
    fn bucket_score(&self) -> &i32 {
        &self.bucket_score
    }
    fn raw_bucket_score(&self) -> &i32 {
        &self.raw_bucket_score
    }
}

pub static BUCKETSCORECONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BucketScoreContainer",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BucketScoreContainer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BucketScore",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BucketScoreContainer, bucket_score),
            },
            FieldInfoData {
                name: "RawBucketScore",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BucketScoreContainer, raw_bucket_score),
            },
        ],
    }),
    array_type: Some(BUCKETSCORECONTAINER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BucketScoreContainer {
    fn type_info(&self) -> &'static TypeInfo {
        BUCKETSCORECONTAINER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BUCKETSCORECONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BucketScoreContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BucketScoreContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PlayerScoreDetail {
    #[default]
    PlayerScoreDetail_MaxPlayers = 256,
}

pub static PLAYERSCOREDETAIL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerScoreDetail",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(PLAYERSCOREDETAIL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlayerScoreDetail {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYERSCOREDETAIL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLAYERSCOREDETAIL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerScoreDetail-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlayerScoreDetail"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UINetworkCheckLevelInstalledMessage {
}

pub trait UINetworkCheckLevelInstalledMessageTrait: TypeObject {
}

impl UINetworkCheckLevelInstalledMessageTrait for UINetworkCheckLevelInstalledMessage {
}

pub static UINETWORKCHECKLEVELINSTALLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkCheckLevelInstalledMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkCheckLevelInstalledMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkCheckLevelInstalledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKCHECKLEVELINSTALLEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkRollCreditsMessage {
}

pub trait UINetworkRollCreditsMessageTrait: TypeObject {
}

impl UINetworkRollCreditsMessageTrait for UINetworkRollCreditsMessage {
}

pub static UINETWORKROLLCREDITSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkRollCreditsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkRollCreditsMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkRollCreditsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKROLLCREDITSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkStealBodyMessage {
}

pub trait UINetworkStealBodyMessageTrait: TypeObject {
}

impl UINetworkStealBodyMessageTrait for UINetworkStealBodyMessage {
}

pub static UINETWORKSTEALBODYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkStealBodyMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkStealBodyMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkStealBodyMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKSTEALBODYMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkPlayerMissionObjectiveTextMessage {
}

pub trait UINetworkPlayerMissionObjectiveTextMessageTrait: TypeObject {
}

impl UINetworkPlayerMissionObjectiveTextMessageTrait for UINetworkPlayerMissionObjectiveTextMessage {
}

pub static UINETWORKPLAYERMISSIONOBJECTIVETEXTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerMissionObjectiveTextMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkPlayerMissionObjectiveTextMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerMissionObjectiveTextMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKPLAYERMISSIONOBJECTIVETEXTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkVideoDoneMessage {
}

pub trait UINetworkVideoDoneMessageTrait: TypeObject {
}

impl UINetworkVideoDoneMessageTrait for UINetworkVideoDoneMessage {
}

pub static UINETWORKVIDEODONEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkVideoDoneMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkVideoDoneMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkVideoDoneMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKVIDEODONEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkAllowSkipVideoMessage {
}

pub trait UINetworkAllowSkipVideoMessageTrait: TypeObject {
}

impl UINetworkAllowSkipVideoMessageTrait for UINetworkAllowSkipVideoMessage {
}

pub static UINETWORKALLOWSKIPVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkAllowSkipVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkAllowSkipVideoMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkAllowSkipVideoMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKALLOWSKIPVIDEOMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkSkipVideoMessage {
}

pub trait UINetworkSkipVideoMessageTrait: TypeObject {
}

impl UINetworkSkipVideoMessageTrait for UINetworkSkipVideoMessage {
}

pub static UINETWORKSKIPVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkSkipVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkSkipVideoMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkSkipVideoMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKSKIPVIDEOMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkStopVideoMessage {
}

pub trait UINetworkStopVideoMessageTrait: TypeObject {
}

impl UINetworkStopVideoMessageTrait for UINetworkStopVideoMessage {
}

pub static UINETWORKSTOPVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkStopVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkStopVideoMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkStopVideoMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKSTOPVIDEOMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkPauseVideoMessage {
}

pub trait UINetworkPauseVideoMessageTrait: TypeObject {
}

impl UINetworkPauseVideoMessageTrait for UINetworkPauseVideoMessage {
}

pub static UINETWORKPAUSEVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPauseVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkPauseVideoMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPauseVideoMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKPAUSEVIDEOMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkPauseAllVideosMessage {
}

pub trait UINetworkPauseAllVideosMessageTrait: TypeObject {
}

impl UINetworkPauseAllVideosMessageTrait for UINetworkPauseAllVideosMessage {
}

pub static UINETWORKPAUSEALLVIDEOSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPauseAllVideosMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkPauseAllVideosMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPauseAllVideosMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKPAUSEALLVIDEOSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkPlayerDeserterTextMessage {
}

pub trait UINetworkPlayerDeserterTextMessageTrait: TypeObject {
}

impl UINetworkPlayerDeserterTextMessageTrait for UINetworkPlayerDeserterTextMessage {
}

pub static UINETWORKPLAYERDESERTERTEXTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerDeserterTextMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkPlayerDeserterTextMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerDeserterTextMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKPLAYERDESERTERTEXTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkPlayerTextMessage {
}

pub trait UINetworkPlayerTextMessageTrait: TypeObject {
}

impl UINetworkPlayerTextMessageTrait for UINetworkPlayerTextMessage {
}

pub static UINETWORKPLAYERTEXTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkPlayerTextMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkPlayerTextMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UINetworkPlayerTextMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKPLAYERTEXTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UINetworkTextInfo {
    pub string_id: String,
    pub display_time: f32,
}

pub trait UINetworkTextInfoTrait: TypeObject {
    fn string_id(&self) -> &String;
    fn display_time(&self) -> &f32;
}

impl UINetworkTextInfoTrait for UINetworkTextInfo {
    fn string_id(&self) -> &String {
        &self.string_id
    }
    fn display_time(&self) -> &f32 {
        &self.display_time
    }
}

pub static UINETWORKTEXTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkTextInfo",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UINetworkTextInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StringId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UINetworkTextInfo, string_id),
            },
            FieldInfoData {
                name: "DisplayTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UINetworkTextInfo, display_time),
            },
        ],
    }),
    array_type: Some(UINETWORKTEXTINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UINetworkTextInfo {
    fn type_info(&self) -> &'static TypeInfo {
        UINETWORKTEXTINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UINETWORKTEXTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UINetworkTextInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("UINetworkTextInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NetworkRemoveDilationMessage {
}

pub trait NetworkRemoveDilationMessageTrait: TypeObject {
}

impl NetworkRemoveDilationMessageTrait for NetworkRemoveDilationMessage {
}

pub static NETWORKREMOVEDILATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkRemoveDilationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkRemoveDilationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkRemoveDilationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKREMOVEDILATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkAddDilationMessage {
}

pub trait NetworkAddDilationMessageTrait: TypeObject {
}

impl NetworkAddDilationMessageTrait for NetworkAddDilationMessage {
}

pub static NETWORKADDDILATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkAddDilationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkAddDilationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkAddDilationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKADDDILATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkLevelInstalledMessage {
}

pub trait NetworkLevelInstalledMessageTrait: TypeObject {
}

impl NetworkLevelInstalledMessageTrait for NetworkLevelInstalledMessage {
}

pub static NETWORKLEVELINSTALLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkLevelInstalledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkLevelInstalledMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkLevelInstalledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKLEVELINSTALLEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkMatchReadyStatusChangedMessage {
}

pub trait NetworkMatchReadyStatusChangedMessageTrait: TypeObject {
}

impl NetworkMatchReadyStatusChangedMessageTrait for NetworkMatchReadyStatusChangedMessage {
}

pub static NETWORKMATCHREADYSTATUSCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMatchReadyStatusChangedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkMatchReadyStatusChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMatchReadyStatusChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKMATCHREADYSTATUSCHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerRestartTimerMessage {
}

pub trait ServerRestartTimerMessageTrait: TypeObject {
}

impl ServerRestartTimerMessageTrait for ServerRestartTimerMessage {
}

pub static SERVERRESTARTTIMERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRestartTimerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRestartTimerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerRestartTimerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERRESTARTTIMERMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkMetricsLevelCompleteMessage {
}

pub trait NetworkMetricsLevelCompleteMessageTrait: TypeObject {
}

impl NetworkMetricsLevelCompleteMessageTrait for NetworkMetricsLevelCompleteMessage {
}

pub static NETWORKMETRICSLEVELCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMetricsLevelCompleteMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkMetricsLevelCompleteMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMetricsLevelCompleteMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKMETRICSLEVELCOMPLETEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkMetricsLevelProgressMessage {
}

pub trait NetworkMetricsLevelProgressMessageTrait: TypeObject {
}

impl NetworkMetricsLevelProgressMessageTrait for NetworkMetricsLevelProgressMessage {
}

pub static NETWORKMETRICSLEVELPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMetricsLevelProgressMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkMetricsLevelProgressMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMetricsLevelProgressMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKMETRICSLEVELPROGRESSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkMetricsSaveGameSavedMessage {
}

pub trait NetworkMetricsSaveGameSavedMessageTrait: TypeObject {
}

impl NetworkMetricsSaveGameSavedMessageTrait for NetworkMetricsSaveGameSavedMessage {
}

pub static NETWORKMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMetricsSaveGameSavedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkMetricsSaveGameSavedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkMetricsSaveGameSavedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKMETRICSSAVEGAMESAVEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSetPlayerViewMessage {
}

pub trait NetworkSetPlayerViewMessageTrait: TypeObject {
}

impl NetworkSetPlayerViewMessageTrait for NetworkSetPlayerViewMessage {
}

pub static NETWORKSETPLAYERVIEWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSetPlayerViewMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSetPlayerViewMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSetPlayerViewMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSETPLAYERVIEWMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSuicideMessage {
}

pub trait NetworkSuicideMessageTrait: TypeObject {
}

impl NetworkSuicideMessageTrait for NetworkSuicideMessage {
}

pub static NETWORKSUICIDEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSuicideMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSuicideMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSuicideMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSUICIDEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkGameplayContinueMessage {
}

pub trait NetworkGameplayContinueMessageTrait: TypeObject {
}

impl NetworkGameplayContinueMessageTrait for NetworkGameplayContinueMessage {
}

pub static NETWORKGAMEPLAYCONTINUEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkGameplayContinueMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkGameplayContinueMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkGameplayContinueMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKGAMEPLAYCONTINUEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PerformanceLogMessage {
}

pub trait PerformanceLogMessageTrait: TypeObject {
}

impl PerformanceLogMessageTrait for PerformanceLogMessage {
}

pub static PERFORMANCELOGMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceLogMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceLogMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for PerformanceLogMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCELOGMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
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

pub trait PerformanceFpsHistogramTrait: TypeObject {
    fn below5(&self) -> &f32;
    fn below10(&self) -> &f32;
    fn below15(&self) -> &f32;
    fn below20(&self) -> &f32;
    fn below25(&self) -> &f32;
    fn below30(&self) -> &f32;
    fn below35(&self) -> &f32;
    fn below40(&self) -> &f32;
    fn below45(&self) -> &f32;
    fn below50(&self) -> &f32;
    fn below55(&self) -> &f32;
    fn below60(&self) -> &f32;
    fn above60(&self) -> &f32;
}

impl PerformanceFpsHistogramTrait for PerformanceFpsHistogram {
    fn below5(&self) -> &f32 {
        &self.below5
    }
    fn below10(&self) -> &f32 {
        &self.below10
    }
    fn below15(&self) -> &f32 {
        &self.below15
    }
    fn below20(&self) -> &f32 {
        &self.below20
    }
    fn below25(&self) -> &f32 {
        &self.below25
    }
    fn below30(&self) -> &f32 {
        &self.below30
    }
    fn below35(&self) -> &f32 {
        &self.below35
    }
    fn below40(&self) -> &f32 {
        &self.below40
    }
    fn below45(&self) -> &f32 {
        &self.below45
    }
    fn below50(&self) -> &f32 {
        &self.below50
    }
    fn below55(&self) -> &f32 {
        &self.below55
    }
    fn below60(&self) -> &f32 {
        &self.below60
    }
    fn above60(&self) -> &f32 {
        &self.above60
    }
}

pub static PERFORMANCEFPSHISTOGRAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceFpsHistogram",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerformanceFpsHistogram as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Below5",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below5),
            },
            FieldInfoData {
                name: "Below10",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below10),
            },
            FieldInfoData {
                name: "Below15",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below15),
            },
            FieldInfoData {
                name: "Below20",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below20),
            },
            FieldInfoData {
                name: "Below25",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below25),
            },
            FieldInfoData {
                name: "Below30",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below30),
            },
            FieldInfoData {
                name: "Below35",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below35),
            },
            FieldInfoData {
                name: "Below40",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below40),
            },
            FieldInfoData {
                name: "Below45",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below45),
            },
            FieldInfoData {
                name: "Below50",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below50),
            },
            FieldInfoData {
                name: "Below55",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below55),
            },
            FieldInfoData {
                name: "Below60",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, below60),
            },
            FieldInfoData {
                name: "Above60",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerformanceFpsHistogram, above60),
            },
        ],
    }),
    array_type: Some(PERFORMANCEFPSHISTOGRAM_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PerformanceFpsHistogram {
    fn type_info(&self) -> &'static TypeInfo {
        PERFORMANCEFPSHISTOGRAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PERFORMANCEFPSHISTOGRAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerformanceFpsHistogram-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PerformanceFpsHistogram"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NetworkFirstPlayerEnteredMessage {
}

pub trait NetworkFirstPlayerEnteredMessageTrait: TypeObject {
}

impl NetworkFirstPlayerEnteredMessageTrait for NetworkFirstPlayerEnteredMessage {
}

pub static NETWORKFIRSTPLAYERENTEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkFirstPlayerEnteredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkFirstPlayerEnteredMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkFirstPlayerEnteredMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKFIRSTPLAYERENTEREDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkDifficultyChangedMessage {
}

pub trait NetworkDifficultyChangedMessageTrait: TypeObject {
}

impl NetworkDifficultyChangedMessageTrait for NetworkDifficultyChangedMessage {
}

pub static NETWORKDIFFICULTYCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkDifficultyChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkDifficultyChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkDifficultyChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKDIFFICULTYCHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSettingsMessage {
}

pub trait NetworkSettingsMessageTrait: TypeObject {
}

impl NetworkSettingsMessageTrait for NetworkSettingsMessage {
}

pub static NETWORKSETTINGSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSettingsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSettingsMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSettingsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSETTINGSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct DebugSpawnGameEntityMessage {
}

pub trait DebugSpawnGameEntityMessageTrait: TypeObject {
}

impl DebugSpawnGameEntityMessageTrait for DebugSpawnGameEntityMessage {
}

pub static DEBUGSPAWNGAMEENTITYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugSpawnGameEntityMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebugSpawnGameEntityMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for DebugSpawnGameEntityMessage {
    fn type_info(&self) -> &'static TypeInfo {
        DEBUGSPAWNGAMEENTITYMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkMovePlayerMessage {
}

pub trait NetworkMovePlayerMessageTrait: TypeObject {
}

impl NetworkMovePlayerMessageTrait for NetworkMovePlayerMessage {
}

pub static NETWORKMOVEPLAYERMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkMovePlayerMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkMovePlayerMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for NetworkMovePlayerMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKMOVEPLAYERMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkJuiceSessionMessage {
}

pub trait NetworkJuiceSessionMessageTrait: TypeObject {
}

impl NetworkJuiceSessionMessageTrait for NetworkJuiceSessionMessage {
}

pub static NETWORKJUICESESSIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkJuiceSessionMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkJuiceSessionMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkJuiceSessionMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKJUICESESSIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSelectTeamMessage {
}

pub trait NetworkSelectTeamMessageTrait: TypeObject {
}

impl NetworkSelectTeamMessageTrait for NetworkSelectTeamMessage {
}

pub static NETWORKSELECTTEAMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSelectTeamMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSelectTeamMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSelectTeamMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSELECTTEAMMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkOnPlayerSpawnedMessage {
}

pub trait NetworkOnPlayerSpawnedMessageTrait: TypeObject {
}

impl NetworkOnPlayerSpawnedMessageTrait for NetworkOnPlayerSpawnedMessage {
}

pub static NETWORKONPLAYERSPAWNEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkOnPlayerSpawnedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkOnPlayerSpawnedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkOnPlayerSpawnedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKONPLAYERSPAWNEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSelectSpawnGroupMessage {
}

pub trait NetworkSelectSpawnGroupMessageTrait: TypeObject {
}

impl NetworkSelectSpawnGroupMessageTrait for NetworkSelectSpawnGroupMessage {
}

pub static NETWORKSELECTSPAWNGROUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSelectSpawnGroupMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSelectSpawnGroupMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSelectSpawnGroupMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSELECTSPAWNGROUPMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSpawnVehicleCustomizationMessage {
}

pub trait NetworkSpawnVehicleCustomizationMessageTrait: TypeObject {
}

impl NetworkSpawnVehicleCustomizationMessageTrait for NetworkSpawnVehicleCustomizationMessage {
}

pub static NETWORKSPAWNVEHICLECUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnVehicleCustomizationMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSpawnVehicleCustomizationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnVehicleCustomizationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSPAWNVEHICLECUSTOMIZATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkUnSpawnCustomizationMessage {
}

pub trait NetworkUnSpawnCustomizationMessageTrait: TypeObject {
}

impl NetworkUnSpawnCustomizationMessageTrait for NetworkUnSpawnCustomizationMessage {
}

pub static NETWORKUNSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkUnSpawnCustomizationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkUnSpawnCustomizationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkUnSpawnCustomizationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKUNSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSpawnCustomizationMessage {
}

pub trait NetworkSpawnCustomizationMessageTrait: TypeObject {
}

impl NetworkSpawnCustomizationMessageTrait for NetworkSpawnCustomizationMessage {
}

pub static NETWORKSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnCustomizationMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSpawnCustomizationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnCustomizationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSPAWNCUSTOMIZATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSpawnOnSelectedMessage {
}

pub trait NetworkSpawnOnSelectedMessageTrait: TypeObject {
}

impl NetworkSpawnOnSelectedMessageTrait for NetworkSpawnOnSelectedMessage {
}

pub static NETWORKSPAWNONSELECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnOnSelectedMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSpawnOnSelectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnOnSelectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSPAWNONSELECTEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSpawnHereMessage {
}

pub trait NetworkSpawnHereMessageTrait: TypeObject {
}

impl NetworkSpawnHereMessageTrait for NetworkSpawnHereMessage {
}

pub static NETWORKSPAWNHEREMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnHereMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSpawnHereMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for NetworkSpawnHereMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSPAWNHEREMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkSpawnMessage {
}

pub trait NetworkSpawnMessageTrait: TypeObject {
}

impl NetworkSpawnMessageTrait for NetworkSpawnMessage {
}

pub static NETWORKSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkSpawnMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkSpawnMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSPAWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkRequestLoadLevelMessage {
}

pub trait NetworkRequestLoadLevelMessageTrait: TypeObject {
}

impl NetworkRequestLoadLevelMessageTrait for NetworkRequestLoadLevelMessage {
}

pub static NETWORKREQUESTLOADLEVELMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkRequestLoadLevelMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkRequestLoadLevelMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkRequestLoadLevelMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKREQUESTLOADLEVELMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkScreenFadeMessage {
}

pub trait NetworkScreenFadeMessageTrait: TypeObject {
}

impl NetworkScreenFadeMessageTrait for NetworkScreenFadeMessage {
}

pub static NETWORKSCREENFADEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkScreenFadeMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkScreenFadeMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkScreenFadeMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKSCREENFADEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct LevelConsoleSetActiveHealthStateMessage {
}

pub trait LevelConsoleSetActiveHealthStateMessageTrait: TypeObject {
}

impl LevelConsoleSetActiveHealthStateMessageTrait for LevelConsoleSetActiveHealthStateMessage {
}

pub static LEVELCONSOLESETACTIVEHEALTHSTATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelConsoleSetActiveHealthStateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelConsoleSetActiveHealthStateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LevelConsoleSetActiveHealthStateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELCONSOLESETACTIVEHEALTHSTATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NetworkPerformanceProfileMessage {
}

pub trait NetworkPerformanceProfileMessageTrait: TypeObject {
}

impl NetworkPerformanceProfileMessageTrait for NetworkPerformanceProfileMessage {
}

pub static NETWORKPERFORMANCEPROFILEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkPerformanceProfileMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkPerformanceProfileMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for NetworkPerformanceProfileMessage {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKPERFORMANCEPROFILEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct SyncedSequenceStateChangeMessageBase {
}

pub trait SyncedSequenceStateChangeMessageBaseTrait: TypeObject {
}

impl SyncedSequenceStateChangeMessageBaseTrait for SyncedSequenceStateChangeMessageBase {
}

pub static SYNCEDSEQUENCESTATECHANGEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SyncedSequenceStateChangeMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SyncedSequenceStateChangeMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for SyncedSequenceStateChangeMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SYNCEDSEQUENCESTATECHANGEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct JuiceSoldierRagdollDeactivateMessage {
}

pub trait JuiceSoldierRagdollDeactivateMessageTrait: TypeObject {
}

impl JuiceSoldierRagdollDeactivateMessageTrait for JuiceSoldierRagdollDeactivateMessage {
}

pub static JUICESOLDIERRAGDOLLDEACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JuiceSoldierRagdollDeactivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JuiceSoldierRagdollDeactivateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for JuiceSoldierRagdollDeactivateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        JUICESOLDIERRAGDOLLDEACTIVATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct JuiceSoldierRagdollActivateMessage {
}

pub trait JuiceSoldierRagdollActivateMessageTrait: TypeObject {
}

impl JuiceSoldierRagdollActivateMessageTrait for JuiceSoldierRagdollActivateMessage {
}

pub static JUICESOLDIERRAGDOLLACTIVATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JuiceSoldierRagdollActivateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JuiceSoldierRagdollActivateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for JuiceSoldierRagdollActivateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        JUICESOLDIERRAGDOLLACTIVATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct AIClientBridgeDynamicModelEntityOnUnspawnMessage {
}

pub trait AIClientBridgeDynamicModelEntityOnUnspawnMessageTrait: TypeObject {
}

impl AIClientBridgeDynamicModelEntityOnUnspawnMessageTrait for AIClientBridgeDynamicModelEntityOnUnspawnMessage {
}

pub static AICLIENTBRIDGEDYNAMICMODELENTITYONUNSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIClientBridgeDynamicModelEntityOnUnspawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIClientBridgeDynamicModelEntityOnUnspawnMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIClientBridgeDynamicModelEntityOnUnspawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        AICLIENTBRIDGEDYNAMICMODELENTITYONUNSPAWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct AIClientBridgeDynamicModelEntityOnSpawnMessage {
}

pub trait AIClientBridgeDynamicModelEntityOnSpawnMessageTrait: TypeObject {
}

impl AIClientBridgeDynamicModelEntityOnSpawnMessageTrait for AIClientBridgeDynamicModelEntityOnSpawnMessage {
}

pub static AICLIENTBRIDGEDYNAMICMODELENTITYONSPAWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIClientBridgeDynamicModelEntityOnSpawnMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIClientBridgeDynamicModelEntityOnSpawnMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for AIClientBridgeDynamicModelEntityOnSpawnMessage {
    fn type_info(&self) -> &'static TypeInfo {
        AICLIENTBRIDGEDYNAMICMODELENTITYONSPAWNMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WantedAimState {
    #[default]
    WantedAimState_NotAiming = 0,
    WantedAimState_Aiming = 1,
    WantedAimState_ForcedAiming = 2,
}

pub static WANTEDAIMSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WantedAimState",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(WANTEDAIMSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WantedAimState {
    fn type_info(&self) -> &'static TypeInfo {
        WANTEDAIMSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WANTEDAIMSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WantedAimState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("WantedAimState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AimTargetId {
    #[default]
    AimTargetId_NoTarget = 0,
    AimTargetId_Position = 1,
    AimTargetId_Entity1 = 2,
    AimTargetId_Entity2 = 3,
}

pub static AIMTARGETID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimTargetId",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(AIMTARGETID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AimTargetId {
    fn type_info(&self) -> &'static TypeInfo {
        AIMTARGETID_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AIMTARGETID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AimTargetId-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AimTargetId"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SharedLockObserverEntity {
    pub _glacier_base: SharedLockBaseEntity,
}

pub trait SharedLockObserverEntityTrait: SharedLockBaseEntityTrait {
}

impl SharedLockObserverEntityTrait for SharedLockObserverEntity {
}

impl SharedLockBaseEntityTrait for SharedLockObserverEntity {
}

impl super::entity::EntityTrait for SharedLockObserverEntity {
}

impl super::entity::EntityBusPeerTrait for SharedLockObserverEntity {
}

pub static SHAREDLOCKOBSERVERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockObserverEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHAREDLOCKBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SharedLockObserverEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKOBSERVERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockObserverEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHAREDLOCKOBSERVERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHAREDLOCKOBSERVERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockObserverEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockObserverEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SharedLockGateEntity {
    pub _glacier_base: SharedLockBaseEntity,
}

pub trait SharedLockGateEntityTrait: SharedLockBaseEntityTrait {
}

impl SharedLockGateEntityTrait for SharedLockGateEntity {
}

impl SharedLockBaseEntityTrait for SharedLockGateEntity {
}

impl super::entity::EntityTrait for SharedLockGateEntity {
}

impl super::entity::EntityBusPeerTrait for SharedLockGateEntity {
}

pub static SHAREDLOCKGATEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockGateEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHAREDLOCKBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SharedLockGateEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKGATEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockGateEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHAREDLOCKGATEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHAREDLOCKGATEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockGateEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockGateEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SharedLockControllerEntity {
    pub _glacier_base: SharedLockBaseEntity,
}

pub trait SharedLockControllerEntityTrait: SharedLockBaseEntityTrait {
}

impl SharedLockControllerEntityTrait for SharedLockControllerEntity {
}

impl SharedLockBaseEntityTrait for SharedLockControllerEntity {
}

impl super::entity::EntityTrait for SharedLockControllerEntity {
}

impl super::entity::EntityBusPeerTrait for SharedLockControllerEntity {
}

pub static SHAREDLOCKCONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHAREDLOCKBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SharedLockControllerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKCONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockControllerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHAREDLOCKCONTROLLERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHAREDLOCKCONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SharedLockBaseEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SharedLockBaseEntityTrait: super::entity::EntityTrait {
}

impl SharedLockBaseEntityTrait for SharedLockBaseEntity {
}

impl super::entity::EntityTrait for SharedLockBaseEntity {
}

impl super::entity::EntityBusPeerTrait for SharedLockBaseEntity {
}

pub static SHAREDLOCKBASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockBaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SharedLockBaseEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SHAREDLOCKBASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SharedLockBaseEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SHAREDLOCKBASEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SHAREDLOCKBASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SharedLockBaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SharedLockBaseEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntersectionTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait IntersectionTriggerEntityTrait: super::entity::EntityTrait {
}

impl IntersectionTriggerEntityTrait for IntersectionTriggerEntity {
}

impl super::entity::EntityTrait for IntersectionTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for IntersectionTriggerEntity {
}

pub static INTERSECTIONTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntersectionTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntersectionTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INTERSECTIONTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntersectionTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        INTERSECTIONTRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INTERSECTIONTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntersectionTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("IntersectionTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UserSpawnContext {
}

pub trait UserSpawnContextTrait: TypeObject {
}

impl UserSpawnContextTrait for UserSpawnContext {
}

pub static USERSPAWNCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserSpawnContext",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UserSpawnContext as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(USERSPAWNCONTEXT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UserSpawnContext {
    fn type_info(&self) -> &'static TypeInfo {
        USERSPAWNCONTEXT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static USERSPAWNCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserSpawnContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("UserSpawnContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlayerIteratorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait PlayerIteratorEntityTrait: super::entity::EntityTrait {
}

impl PlayerIteratorEntityTrait for PlayerIteratorEntity {
}

impl super::entity::EntityTrait for PlayerIteratorEntity {
}

impl super::entity::EntityBusPeerTrait for PlayerIteratorEntity {
}

pub static PLAYERITERATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerIteratorEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayerIteratorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PLAYERITERATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlayerIteratorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYERITERATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLAYERITERATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayerIteratorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlayerIteratorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlatformSplitterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait PlatformSplitterEntityTrait: super::entity::EntityTrait {
}

impl PlatformSplitterEntityTrait for PlatformSplitterEntity {
}

impl super::entity::EntityTrait for PlatformSplitterEntity {
}

impl super::entity::EntityBusPeerTrait for PlatformSplitterEntity {
}

pub static PLATFORMSPLITTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformSplitterEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformSplitterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PLATFORMSPLITTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PlatformSplitterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        PLATFORMSPLITTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLATFORMSPLITTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformSplitterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("PlatformSplitterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LoggingEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LoggingEntityTrait: super::entity::EntityTrait {
}

impl LoggingEntityTrait for LoggingEntity {
}

impl super::entity::EntityTrait for LoggingEntity {
}

impl super::entity::EntityBusPeerTrait for LoggingEntity {
}

pub static LOGGINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoggingEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LoggingEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOGGINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LoggingEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOGGINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOGGINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LoggingEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("LoggingEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DeltaViewerTableEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DeltaViewerTableEntityTrait: super::entity::EntityTrait {
}

impl DeltaViewerTableEntityTrait for DeltaViewerTableEntity {
}

impl super::entity::EntityTrait for DeltaViewerTableEntity {
}

impl super::entity::EntityBusPeerTrait for DeltaViewerTableEntity {
}

pub static DELTAVIEWERTABLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaViewerTableEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DeltaViewerTableEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DELTAVIEWERTABLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DeltaViewerTableEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DELTAVIEWERTABLEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DELTAVIEWERTABLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeltaViewerTableEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("DeltaViewerTableEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConsoleCommandTriggerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConsoleCommandTriggerEntityTrait: super::entity::EntityTrait {
}

impl ConsoleCommandTriggerEntityTrait for ConsoleCommandTriggerEntity {
}

impl super::entity::EntityTrait for ConsoleCommandTriggerEntity {
}

impl super::entity::EntityBusPeerTrait for ConsoleCommandTriggerEntity {
}

pub static CONSOLECOMMANDTRIGGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandTriggerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConsoleCommandTriggerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONSOLECOMMANDTRIGGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConsoleCommandTriggerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONSOLECOMMANDTRIGGERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONSOLECOMMANDTRIGGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandTriggerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ConsoleCommandTriggerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConsoleCommandEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConsoleCommandEntityTrait: super::entity::EntityTrait {
}

impl ConsoleCommandEntityTrait for ConsoleCommandEntity {
}

impl super::entity::EntityTrait for ConsoleCommandEntity {
}

impl super::entity::EntityBusPeerTrait for ConsoleCommandEntity {
}

pub static CONSOLECOMMANDENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConsoleCommandEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONSOLECOMMANDENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConsoleCommandEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONSOLECOMMANDENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONSOLECOMMANDENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConsoleCommandEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ConsoleCommandEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LicenseeGameWorldRayCaster {
    pub _glacier_base: super::gameplay_sim::GameWorldRayCaster,
}

pub trait LicenseeGameWorldRayCasterTrait: super::gameplay_sim::GameWorldRayCasterTrait {
}

impl LicenseeGameWorldRayCasterTrait for LicenseeGameWorldRayCaster {
}

impl super::gameplay_sim::GameWorldRayCasterTrait for LicenseeGameWorldRayCaster {
}

pub static LICENSEEGAMEWORLDRAYCASTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseeGameWorldRayCaster",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMEWORLDRAYCASTER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LicenseeGameWorldRayCaster as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LICENSEEGAMEWORLDRAYCASTER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LicenseeGameWorldRayCaster {
    fn type_info(&self) -> &'static TypeInfo {
        LICENSEEGAMEWORLDRAYCASTER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LICENSEEGAMEWORLDRAYCASTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LicenseeGameWorldRayCaster-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("LicenseeGameWorldRayCaster"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SubLevelStatusEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SubLevelStatusEntityTrait: super::entity::EntityTrait {
}

impl SubLevelStatusEntityTrait for SubLevelStatusEntity {
}

impl super::entity::EntityTrait for SubLevelStatusEntity {
}

impl super::entity::EntityBusPeerTrait for SubLevelStatusEntity {
}

pub static SUBLEVELSTATUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelStatusEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelStatusEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELSTATUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelStatusEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELSTATUSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUBLEVELSTATUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelStatusEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SubLevelStatusEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SubLevelPreloadEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SubLevelPreloadEntityTrait: super::entity::EntityTrait {
}

impl SubLevelPreloadEntityTrait for SubLevelPreloadEntity {
}

impl super::entity::EntityTrait for SubLevelPreloadEntity {
}

impl super::entity::EntityBusPeerTrait for SubLevelPreloadEntity {
}

pub static SUBLEVELPRELOADENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelPreloadEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELPRELOADENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelPreloadEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELPRELOADENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUBLEVELPRELOADENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelPreloadEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SubLevelPreloadEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SubLevelCollectionEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait SubLevelCollectionEntityBaseTrait: super::entity::EntityTrait {
}

impl SubLevelCollectionEntityBaseTrait for SubLevelCollectionEntityBase {
}

impl super::entity::EntityTrait for SubLevelCollectionEntityBase {
}

impl super::entity::EntityBusPeerTrait for SubLevelCollectionEntityBase {
}

pub static SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SubLevelCollectionEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUBLEVELCOLLECTIONENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SubLevelCollectionEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        SUBLEVELCOLLECTIONENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUBLEVELCOLLECTIONENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubLevelCollectionEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("SubLevelCollectionEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlueprintBundleStatusEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait BlueprintBundleStatusEntityTrait: super::entity::EntityTrait {
}

impl BlueprintBundleStatusEntityTrait for BlueprintBundleStatusEntity {
}

impl super::entity::EntityTrait for BlueprintBundleStatusEntity {
}

impl super::entity::EntityBusPeerTrait for BlueprintBundleStatusEntity {
}

pub static BLUEPRINTBUNDLESTATUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleStatusEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLESTATUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintBundleStatusEntity {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLESTATUSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BLUEPRINTBUNDLESTATUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleStatusEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BlueprintBundleStatusEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerBlueprintBundleEntity {
    pub _glacier_base: BlueprintBundleEntityBase,
}

pub trait ServerBlueprintBundleEntityTrait: BlueprintBundleEntityBaseTrait {
}

impl ServerBlueprintBundleEntityTrait for ServerBlueprintBundleEntity {
}

impl BlueprintBundleEntityBaseTrait for ServerBlueprintBundleEntity {
}

impl super::entity::EntityTrait for ServerBlueprintBundleEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBlueprintBundleEntity {
}

pub static SERVERBLUEPRINTBUNDLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBlueprintBundleEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintBundleEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBLUEPRINTBUNDLEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ServerBlueprintBundleEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBlueprintBundleEntity {
    pub _glacier_base: BlueprintBundleEntityBase,
}

pub trait ClientBlueprintBundleEntityTrait: BlueprintBundleEntityBaseTrait {
}

impl ClientBlueprintBundleEntityTrait for ClientBlueprintBundleEntity {
}

impl BlueprintBundleEntityBaseTrait for ClientBlueprintBundleEntity {
}

impl super::entity::EntityTrait for ClientBlueprintBundleEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBlueprintBundleEntity {
}

pub static CLIENTBLUEPRINTBUNDLEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlueprintBundleEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintBundleEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLUEPRINTBUNDLEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBLUEPRINTBUNDLEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ClientBlueprintBundleEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlueprintBundleEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait BlueprintBundleEntityBaseTrait: super::entity::EntityTrait {
}

impl BlueprintBundleEntityBaseTrait for BlueprintBundleEntityBase {
}

impl super::entity::EntityTrait for BlueprintBundleEntityBase {
}

impl super::entity::EntityBusPeerTrait for BlueprintBundleEntityBase {
}

pub static BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLEENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintBundleEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLEENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BLUEPRINTBUNDLEENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BlueprintBundleEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerBlueprintBundleCollectionEntity {
    pub _glacier_base: BlueprintBundleCollectionEntityBase,
}

pub trait ServerBlueprintBundleCollectionEntityTrait: BlueprintBundleCollectionEntityBaseTrait {
}

impl ServerBlueprintBundleCollectionEntityTrait for ServerBlueprintBundleCollectionEntity {
}

impl BlueprintBundleCollectionEntityBaseTrait for ServerBlueprintBundleCollectionEntity {
}

impl super::entity::EntityTrait for ServerBlueprintBundleCollectionEntity {
}

impl super::entity::EntityBusPeerTrait for ServerBlueprintBundleCollectionEntity {
}

pub static SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleCollectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerBlueprintBundleCollectionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerBlueprintBundleCollectionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerBlueprintBundleCollectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ServerBlueprintBundleCollectionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientBlueprintBundleCollectionEntity {
    pub _glacier_base: BlueprintBundleCollectionEntityBase,
}

pub trait ClientBlueprintBundleCollectionEntityTrait: BlueprintBundleCollectionEntityBaseTrait {
}

impl ClientBlueprintBundleCollectionEntityTrait for ClientBlueprintBundleCollectionEntity {
}

impl BlueprintBundleCollectionEntityBaseTrait for ClientBlueprintBundleCollectionEntity {
}

impl super::entity::EntityTrait for ClientBlueprintBundleCollectionEntity {
}

impl super::entity::EntityBusPeerTrait for ClientBlueprintBundleCollectionEntity {
}

pub static CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleCollectionEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientBlueprintBundleCollectionEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientBlueprintBundleCollectionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTBLUEPRINTBUNDLECOLLECTIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientBlueprintBundleCollectionEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("ClientBlueprintBundleCollectionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BlueprintBundleCollectionEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait BlueprintBundleCollectionEntityBaseTrait: super::entity::EntityTrait {
}

impl BlueprintBundleCollectionEntityBaseTrait for BlueprintBundleCollectionEntityBase {
}

impl super::entity::EntityTrait for BlueprintBundleCollectionEntityBase {
}

impl super::entity::EntityBusPeerTrait for BlueprintBundleCollectionEntityBase {
}

pub static BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlueprintBundleCollectionEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLUEPRINTBUNDLECOLLECTIONENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BlueprintBundleCollectionEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        BLUEPRINTBUNDLECOLLECTIONENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BLUEPRINTBUNDLECOLLECTIONENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlueprintBundleCollectionEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("BlueprintBundleCollectionEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TestStaticModelGroupEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub realm: super::core::Realm,
    pub expected_material: super::entity::MaterialDecl,
}

pub trait TestStaticModelGroupEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn expected_material(&self) -> &super::entity::MaterialDecl;
}

impl TestStaticModelGroupEntityDataTrait for TestStaticModelGroupEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn expected_material(&self) -> &super::entity::MaterialDecl {
        &self.expected_material
    }
}

impl super::entity::SpatialEntityDataTrait for TestStaticModelGroupEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::entity::EntityDataTrait for TestStaticModelGroupEntityData {
}

impl super::entity::GameObjectDataTrait for TestStaticModelGroupEntityData {
}

impl super::core::DataBusPeerTrait for TestStaticModelGroupEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for TestStaticModelGroupEntityData {
}

impl super::core::DataContainerTrait for TestStaticModelGroupEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TESTSTATICMODELGROUPENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestStaticModelGroupEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TestStaticModelGroupEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(TestStaticModelGroupEntityData, realm),
            },
            FieldInfoData {
                name: "ExpectedMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(TestStaticModelGroupEntityData, expected_material),
            },
        ],
    }),
    array_type: Some(TESTSTATICMODELGROUPENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TestStaticModelGroupEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TESTSTATICMODELGROUPENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TESTSTATICMODELGROUPENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestStaticModelGroupEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("TestStaticModelGroupEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TestRayCastEntityData {
    pub _glacier_base: super::entity::EntityData,
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

pub trait TestRayCastEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn asynchronous(&self) -> &bool;
    fn ray_start(&self) -> &super::core::Vec3;
    fn ray_end(&self) -> &super::core::Vec3;
    fn output_index(&self) -> &u32;
    fn check_detail_mesh(&self) -> &bool;
    fn check_normal(&self) -> &bool;
    fn check_material(&self) -> &bool;
    fn check_part_index(&self) -> &bool;
    fn check_water(&self) -> &bool;
    fn check_terrain(&self) -> &bool;
    fn check_ragdoll(&self) -> &bool;
    fn check_character(&self) -> &bool;
    fn check_group(&self) -> &bool;
    fn penetrate_penetrable(&self) -> &bool;
    fn penetrate_client_destructible(&self) -> &bool;
    fn penetrate_bashable(&self) -> &bool;
    fn penetrate_see_through(&self) -> &bool;
    fn penetrate_no_collision_response(&self) -> &bool;
    fn penetrate_no_collision_response_combined(&self) -> &bool;
    fn penetrate_attachable(&self) -> &bool;
    fn expected_position(&self) -> &super::core::Vec3;
    fn expected_normal(&self) -> &super::core::Vec3;
    fn expected_part_index(&self) -> &u32;
    fn expected_material(&self) -> &super::entity::MaterialDecl;
    fn tolerance(&self) -> &f32;
}

impl TestRayCastEntityDataTrait for TestRayCastEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn asynchronous(&self) -> &bool {
        &self.asynchronous
    }
    fn ray_start(&self) -> &super::core::Vec3 {
        &self.ray_start
    }
    fn ray_end(&self) -> &super::core::Vec3 {
        &self.ray_end
    }
    fn output_index(&self) -> &u32 {
        &self.output_index
    }
    fn check_detail_mesh(&self) -> &bool {
        &self.check_detail_mesh
    }
    fn check_normal(&self) -> &bool {
        &self.check_normal
    }
    fn check_material(&self) -> &bool {
        &self.check_material
    }
    fn check_part_index(&self) -> &bool {
        &self.check_part_index
    }
    fn check_water(&self) -> &bool {
        &self.check_water
    }
    fn check_terrain(&self) -> &bool {
        &self.check_terrain
    }
    fn check_ragdoll(&self) -> &bool {
        &self.check_ragdoll
    }
    fn check_character(&self) -> &bool {
        &self.check_character
    }
    fn check_group(&self) -> &bool {
        &self.check_group
    }
    fn penetrate_penetrable(&self) -> &bool {
        &self.penetrate_penetrable
    }
    fn penetrate_client_destructible(&self) -> &bool {
        &self.penetrate_client_destructible
    }
    fn penetrate_bashable(&self) -> &bool {
        &self.penetrate_bashable
    }
    fn penetrate_see_through(&self) -> &bool {
        &self.penetrate_see_through
    }
    fn penetrate_no_collision_response(&self) -> &bool {
        &self.penetrate_no_collision_response
    }
    fn penetrate_no_collision_response_combined(&self) -> &bool {
        &self.penetrate_no_collision_response_combined
    }
    fn penetrate_attachable(&self) -> &bool {
        &self.penetrate_attachable
    }
    fn expected_position(&self) -> &super::core::Vec3 {
        &self.expected_position
    }
    fn expected_normal(&self) -> &super::core::Vec3 {
        &self.expected_normal
    }
    fn expected_part_index(&self) -> &u32 {
        &self.expected_part_index
    }
    fn expected_material(&self) -> &super::entity::MaterialDecl {
        &self.expected_material
    }
    fn tolerance(&self) -> &f32 {
        &self.tolerance
    }
}

impl super::entity::EntityDataTrait for TestRayCastEntityData {
}

impl super::entity::GameObjectDataTrait for TestRayCastEntityData {
}

impl super::core::DataBusPeerTrait for TestRayCastEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for TestRayCastEntityData {
}

impl super::core::DataContainerTrait for TestRayCastEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TESTRAYCASTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestRayCastEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TestRayCastEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(TestRayCastEntityData, realm),
            },
            FieldInfoData {
                name: "Asynchronous",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, asynchronous),
            },
            FieldInfoData {
                name: "RayStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TestRayCastEntityData, ray_start),
            },
            FieldInfoData {
                name: "RayEnd",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TestRayCastEntityData, ray_end),
            },
            FieldInfoData {
                name: "OutputIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TestRayCastEntityData, output_index),
            },
            FieldInfoData {
                name: "CheckDetailMesh",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_detail_mesh),
            },
            FieldInfoData {
                name: "CheckNormal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_normal),
            },
            FieldInfoData {
                name: "CheckMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_material),
            },
            FieldInfoData {
                name: "CheckPartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_part_index),
            },
            FieldInfoData {
                name: "CheckWater",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_water),
            },
            FieldInfoData {
                name: "CheckTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_terrain),
            },
            FieldInfoData {
                name: "CheckRagdoll",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_ragdoll),
            },
            FieldInfoData {
                name: "CheckCharacter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_character),
            },
            FieldInfoData {
                name: "CheckGroup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, check_group),
            },
            FieldInfoData {
                name: "PenetratePenetrable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_penetrable),
            },
            FieldInfoData {
                name: "PenetrateClientDestructible",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_client_destructible),
            },
            FieldInfoData {
                name: "PenetrateBashable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_bashable),
            },
            FieldInfoData {
                name: "PenetrateSeeThrough",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_see_through),
            },
            FieldInfoData {
                name: "PenetrateNoCollisionResponse",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_no_collision_response),
            },
            FieldInfoData {
                name: "PenetrateNoCollisionResponseCombined",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_no_collision_response_combined),
            },
            FieldInfoData {
                name: "PenetrateAttachable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TestRayCastEntityData, penetrate_attachable),
            },
            FieldInfoData {
                name: "ExpectedPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TestRayCastEntityData, expected_position),
            },
            FieldInfoData {
                name: "ExpectedNormal",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TestRayCastEntityData, expected_normal),
            },
            FieldInfoData {
                name: "ExpectedPartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TestRayCastEntityData, expected_part_index),
            },
            FieldInfoData {
                name: "ExpectedMaterial",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(TestRayCastEntityData, expected_material),
            },
            FieldInfoData {
                name: "Tolerance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TestRayCastEntityData, tolerance),
            },
        ],
    }),
    array_type: Some(TESTRAYCASTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TestRayCastEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TESTRAYCASTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TESTRAYCASTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TestRayCastEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("TestRayCastEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AdministrationRestrictionLevel {
    #[default]
    AdministrationRestrictionLevel_Zero = 0,
    AdministrationRestrictionLevel_One = 1,
    AdministrationRestrictionLevel_Two = 2,
    AdministrationRestrictionLevel_Three = 3,
    AdministrationRestrictionLevel_Count = 4,
}

pub static ADMINISTRATIONRESTRICTIONLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationRestrictionLevel",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(ADMINISTRATIONRESTRICTIONLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AdministrationRestrictionLevel {
    fn type_info(&self) -> &'static TypeInfo {
        ADMINISTRATIONRESTRICTIONLEVEL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ADMINISTRATIONRESTRICTIONLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationRestrictionLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AdministrationRestrictionLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AdministrationEventType {
    #[default]
    AdministrationEventType_Add = 0,
    AdministrationEventType_Remove = 1,
    AdministrationEventType_Clear = 2,
    AdministrationEventType_List = 3,
    AdministrationEventType_Load = 4,
    AdministrationEventType_Save = 5,
}

pub static ADMINISTRATIONEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationEventType",
    flags: MemberInfoFlags::new(49429),
    module: "GameCommon",
    data: TypeInfoData::Enum,
    array_type: Some(ADMINISTRATIONEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AdministrationEventType {
    fn type_info(&self) -> &'static TypeInfo {
        ADMINISTRATIONEVENTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ADMINISTRATIONEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AdministrationEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("AdministrationEventType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAdministrationPasswordMessage {
}

pub trait ServerAdministrationPasswordMessageTrait: TypeObject {
}

impl ServerAdministrationPasswordMessageTrait for ServerAdministrationPasswordMessage {
}

pub static SERVERADMINISTRATIONPASSWORDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationPasswordMessage",
    flags: MemberInfoFlags::new(73),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdministrationPasswordMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationPasswordMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINISTRATIONPASSWORDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerAdministrationEventsEnabledMessage {
}

pub trait ServerAdministrationEventsEnabledMessageTrait: TypeObject {
}

impl ServerAdministrationEventsEnabledMessageTrait for ServerAdministrationEventsEnabledMessage {
}

pub static SERVERADMINISTRATIONEVENTSENABLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationEventsEnabledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdministrationEventsEnabledMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationEventsEnabledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINISTRATIONEVENTSENABLEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerAdministrationQuitMessage {
}

pub trait ServerAdministrationQuitMessageTrait: TypeObject {
}

impl ServerAdministrationQuitMessageTrait for ServerAdministrationQuitMessage {
}

pub static SERVERADMINISTRATIONQUITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationQuitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdministrationQuitMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationQuitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINISTRATIONQUITMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerAdministrationLoginMessage {
}

pub trait ServerAdministrationLoginMessageTrait: TypeObject {
}

impl ServerAdministrationLoginMessageTrait for ServerAdministrationLoginMessage {
}

pub static SERVERADMINISTRATIONLOGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationLoginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdministrationLoginMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ServerAdministrationLoginMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINISTRATIONLOGINMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServerAdministrationPacketMessageBase {
}

pub trait ServerAdministrationPacketMessageBaseTrait: TypeObject {
}

impl ServerAdministrationPacketMessageBaseTrait for ServerAdministrationPacketMessageBase {
}

pub static SERVERADMINISTRATIONPACKETMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAdministrationPacketMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "GameCommon",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAdministrationPacketMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for ServerAdministrationPacketMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERADMINISTRATIONPACKETMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct GameModule {
    pub _glacier_base: super::core::RuntimeModule,
}

pub trait GameModuleTrait: super::core::RuntimeModuleTrait {
}

impl GameModuleTrait for GameModule {
}

impl super::core::RuntimeModuleTrait for GameModule {
}

pub static GAMEMODULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModule",
    flags: MemberInfoFlags::new(101),
    module: "GameCommon",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::RUNTIMEMODULE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameModule as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMEMODULE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GameModule {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEMODULE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GAMEMODULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameModule-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameCommon",
    data: TypeInfoData::Array("GameModule"),
    array_type: None,
    alignment: 8,
};


