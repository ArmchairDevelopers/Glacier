use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_creature_loco_types(registry: &mut TypeRegistry) {
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(IMOVEMENTPROVIDER_TYPE_INFO);
    registry.register_type(IMOVEMENTPROVIDER_ARRAY_TYPE_INFO);
    registry.register_type(ISTEERINGPROVIDER_TYPE_INFO);
    registry.register_type(ISTEERINGPROVIDER_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PAUSE_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_TYPE_INFO);
    registry.register_type(CREATUREWAYPOINT_ARRAY_TYPE_INFO);
    registry.register_type(CL_PROCEDURALMOTION_TYPE_INFO);
    registry.register_type(CL_PROCEDURALMOTION_ARRAY_TYPE_INFO);
    registry.register_type(CL_CURVESTEERING_TYPE_INFO);
    registry.register_type(CL_CURVESTEERING_ARRAY_TYPE_INFO);
    registry.register_type(CLCOLAVOIDINGSTEERING_TYPE_INFO);
    registry.register_type(CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO);
    registry.register_type(IASSESSOR_TYPE_INFO);
    registry.register_type(IASSESSOR_ARRAY_TYPE_INFO);
    registry.register_type(CLCLIENTSTATE_TYPE_INFO);
    registry.register_type(CLCLIENTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLSTATE_TYPE_INFO);
    registry.register_type(CLSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLCONDUITSTATE_TYPE_INFO);
    registry.register_type(CLCONDUITSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATURESPAWNENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATURESPAWNENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITY_TYPE_INFO);
    registry.register_type(CREATURELOCOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO);
    registry.register_type(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO);
    registry.register_type(CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITY_TYPE_INFO);
    registry.register_type(CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITY_TYPE_INFO);
    registry.register_type(CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITY_TYPE_INFO);
    registry.register_type(CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO);
    registry.register_type(CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CREATURESPAWNENTITYDATA_TYPE_INFO);
    registry.register_type(CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(SERVERCREATURELOCOMOTORENTITY_TYPE_INFO);
    registry.register_type(SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct ServerCreatureFollowWaypointsEntity {
    pub _glacier_base: ServerCreatureFollowWaypointSegmentEntity,
}

pub trait ServerCreatureFollowWaypointsEntityTrait: ServerCreatureFollowWaypointSegmentEntityTrait {
}

impl ServerCreatureFollowWaypointsEntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl ServerCreatureFollowWaypointSegmentEntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl CreatureFollowBaseEntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl super::entity::EntityTrait for ServerCreatureFollowWaypointsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureFollowWaypointsEntity {
}

pub static SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureFollowWaypointsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCreatureFollowWaypointSegmentEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait ServerCreatureFollowWaypointSegmentEntityTrait: CreatureFollowBaseEntityTrait {
}

impl ServerCreatureFollowWaypointSegmentEntityTrait for ServerCreatureFollowWaypointSegmentEntity {
}

impl CreatureFollowBaseEntityTrait for ServerCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityTrait for ServerCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureFollowWaypointSegmentEntity {
}

pub static SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointSegmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureFollowWaypointSegmentEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointSegmentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointSegmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointSegmentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCreatureCollisionGroupEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerCreatureCollisionGroupEntityTrait: super::entity::EntityTrait {
}

impl ServerCreatureCollisionGroupEntityTrait for ServerCreatureCollisionGroupEntity {
}

impl super::entity::EntityTrait for ServerCreatureCollisionGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureCollisionGroupEntity {
}

pub static SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureCollisionGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureCollisionGroupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureCollisionGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureCollisionGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureCollisionGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IMovementProvider {
}

pub trait IMovementProviderTrait: TypeObject {
}

impl IMovementProviderTrait for IMovementProvider {
}

pub static IMOVEMENTPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMovementProvider",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IMovementProvider as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IMOVEMENTPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IMovementProvider {
    fn type_info(&self) -> &'static TypeInfo {
        IMOVEMENTPROVIDER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IMOVEMENTPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMovementProvider-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("IMovementProvider"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ISteeringProvider {
}

pub trait ISteeringProviderTrait: TypeObject {
}

impl ISteeringProviderTrait for ISteeringProvider {
}

pub static ISTEERINGPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ISteeringProvider",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ISteeringProvider as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ISTEERINGPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ISteeringProvider {
    fn type_info(&self) -> &'static TypeInfo {
        ISTEERINGPROVIDER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ISTEERINGPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ISteeringProvider-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ISteeringProvider"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureWaypoint_PlayAnimation {
    pub _glacier_base: CreatureWaypoint,
}

pub trait CreatureWaypoint_PlayAnimationTrait: CreatureWaypointTrait {
}

impl CreatureWaypoint_PlayAnimationTrait for CreatureWaypoint_PlayAnimation {
}

impl CreatureWaypointTrait for CreatureWaypoint_PlayAnimation {
}

pub static CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_PlayAnimation",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREWAYPOINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypoint_PlayAnimation as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypoint_PlayAnimation {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_PlayAnimation-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint_PlayAnimation"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureWaypoint_Pause {
    pub _glacier_base: CreatureWaypoint,
}

pub trait CreatureWaypoint_PauseTrait: CreatureWaypointTrait {
}

impl CreatureWaypoint_PauseTrait for CreatureWaypoint_Pause {
}

impl CreatureWaypointTrait for CreatureWaypoint_Pause {
}

pub static CREATUREWAYPOINT_PAUSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_Pause",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREWAYPOINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypoint_Pause as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypoint_Pause {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINT_PAUSE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_Pause-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint_Pause"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureWaypoint {
}

pub trait CreatureWaypointTrait: TypeObject {
}

impl CreatureWaypointTrait for CreatureWaypoint {
}

pub static CREATUREWAYPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureWaypoint as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypoint {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREWAYPOINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREWAYPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CL_ProceduralMotion {
    pub _glacier_base: IMovementProvider,
}

pub trait CL_ProceduralMotionTrait: IMovementProviderTrait {
}

impl CL_ProceduralMotionTrait for CL_ProceduralMotion {
}

impl IMovementProviderTrait for CL_ProceduralMotion {
}

pub static CL_PROCEDURALMOTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_ProceduralMotion",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOVEMENTPROVIDER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CL_ProceduralMotion as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CL_PROCEDURALMOTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CL_ProceduralMotion {
    fn type_info(&self) -> &'static TypeInfo {
        CL_PROCEDURALMOTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CL_PROCEDURALMOTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_ProceduralMotion-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CL_ProceduralMotion"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CL_CurveSteering {
    pub _glacier_base: ISteeringProvider,
}

pub trait CL_CurveSteeringTrait: ISteeringProviderTrait {
}

impl CL_CurveSteeringTrait for CL_CurveSteering {
}

impl ISteeringProviderTrait for CL_CurveSteering {
}

pub static CL_CURVESTEERING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_CurveSteering",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ISTEERINGPROVIDER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CL_CurveSteering as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CL_CURVESTEERING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CL_CurveSteering {
    fn type_info(&self) -> &'static TypeInfo {
        CL_CURVESTEERING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CL_CURVESTEERING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_CurveSteering-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CL_CurveSteering"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLColAvoidingSteering {
    pub _glacier_base: CL_CurveSteering,
}

pub trait CLColAvoidingSteeringTrait: CL_CurveSteeringTrait {
}

impl CLColAvoidingSteeringTrait for CLColAvoidingSteering {
}

impl CL_CurveSteeringTrait for CLColAvoidingSteering {
}

impl ISteeringProviderTrait for CLColAvoidingSteering {
}

pub static CLCOLAVOIDINGSTEERING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLColAvoidingSteering",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CL_CURVESTEERING_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLColAvoidingSteering as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLColAvoidingSteering {
    fn type_info(&self) -> &'static TypeInfo {
        CLCOLAVOIDINGSTEERING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLColAvoidingSteering-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLColAvoidingSteering"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IAssessor {
}

pub trait IAssessorTrait: TypeObject {
}

impl IAssessorTrait for IAssessor {
}

pub static IASSESSOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IAssessor",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IAssessor as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IASSESSOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IAssessor {
    fn type_info(&self) -> &'static TypeInfo {
        IASSESSOR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IASSESSOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IAssessor-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("IAssessor"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLClientState {
    pub _glacier_base: CLState,
}

pub trait CLClientStateTrait: CLStateTrait {
}

impl CLClientStateTrait for CLClientState {
}

impl CLStateTrait for CLClientState {
}

impl IAssessorTrait for CLClientState {
}

pub static CLCLIENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLClientState",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLSTATE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLClientState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLCLIENTSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLClientState {
    fn type_info(&self) -> &'static TypeInfo {
        CLCLIENTSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLCLIENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLClientState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLClientState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLState {
    pub _glacier_base: IAssessor,
}

pub trait CLStateTrait: IAssessorTrait {
}

impl CLStateTrait for CLState {
}

impl IAssessorTrait for CLState {
}

pub static CLSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLState",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IASSESSOR_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLState {
    fn type_info(&self) -> &'static TypeInfo {
        CLSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLConduitState {
    pub _glacier_base: CLState,
}

pub trait CLConduitStateTrait: CLStateTrait {
}

impl CLConduitStateTrait for CLConduitState {
}

impl CLStateTrait for CLConduitState {
}

impl IAssessorTrait for CLConduitState {
}

pub static CLCONDUITSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLConduitState",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLSTATE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLConduitState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLCONDUITSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLConduitState {
    fn type_info(&self) -> &'static TypeInfo {
        CLCONDUITSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLCONDUITSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLConduitState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLConduitState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCreatureSpawnEntity {
    pub _glacier_base: super::dice_commons::ClientBlueprintSpawnEntity,
}

pub trait ClientCreatureSpawnEntityTrait: super::dice_commons::ClientBlueprintSpawnEntityTrait {
}

impl ClientCreatureSpawnEntityTrait for ClientCreatureSpawnEntity {
}

impl super::dice_commons::ClientBlueprintSpawnEntityTrait for ClientCreatureSpawnEntity {
}

impl super::dice_commons::BlueprintSpawnEntityTrait for ClientCreatureSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ClientCreatureSpawnEntity {
}

impl super::entity::EntityTrait for ClientCreatureSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureSpawnEntity {
}

pub static CLIENTCREATURESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons::CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureSpawnEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATURESPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCreatureSpawnEntity {
    pub _glacier_base: super::dice_commons::ServerBlueprintSpawnEntity,
}

pub trait ServerCreatureSpawnEntityTrait: super::dice_commons::ServerBlueprintSpawnEntityTrait {
}

impl ServerCreatureSpawnEntityTrait for ServerCreatureSpawnEntity {
}

impl super::dice_commons::ServerBlueprintSpawnEntityTrait for ServerCreatureSpawnEntity {
}

impl super::dice_commons::BlueprintSpawnEntityTrait for ServerCreatureSpawnEntity {
}

impl super::entity::SpatialEntityTrait for ServerCreatureSpawnEntity {
}

impl super::entity::EntityTrait for ServerCreatureSpawnEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureSpawnEntity {
}

pub static SERVERCREATURESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons::SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureSpawnEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureSpawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATURESPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureSpawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureLocoEntity {
    pub _glacier_base: super::a_i_tools::LocoEntity,
}

pub trait CreatureLocoEntityTrait: super::a_i_tools::LocoEntityTrait {
}

impl CreatureLocoEntityTrait for CreatureLocoEntity {
}

impl super::a_i_tools::LocoEntityTrait for CreatureLocoEntity {
}

impl super::entity::EntityTrait for CreatureLocoEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureLocoEntity {
}

pub static CREATURELOCOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::a_i_tools::LOCOENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureLocoEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATURELOCOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureLocoEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURELOCOENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATURELOCOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureLocoEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointUnspawnEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointUnspawnEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointUnspawnEntityTrait for CreatureFollowWaypointUnspawnEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointUnspawnEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointUnspawnEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointUnspawnEntity {
}

pub static CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointUnspawnEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointUnspawnEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointUnspawnEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowBaseEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CreatureFollowBaseEntityTrait: super::entity::EntityTrait {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowBaseEntity {
}

impl super::entity::EntityTrait for CreatureFollowBaseEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowBaseEntity {
}

pub static CREATUREFOLLOWBASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowBaseEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowBaseEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWBASEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowBaseEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCreatureFollowWaypointProviderEntity {
    pub _glacier_base: CreatureBaseWaypointProviderEntity,
}

pub trait ServerCreatureFollowWaypointProviderEntityTrait: CreatureBaseWaypointProviderEntityTrait {
}

impl ServerCreatureFollowWaypointProviderEntityTrait for ServerCreatureFollowWaypointProviderEntity {
}

impl CreatureBaseWaypointProviderEntityTrait for ServerCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityTrait for ServerCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureFollowWaypointProviderEntity {
}

pub static SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureFollowWaypointProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCreatureFollowWaypointProviderEntity {
    pub _glacier_base: CreatureBaseWaypointProviderEntity,
}

pub trait ClientCreatureFollowWaypointProviderEntityTrait: CreatureBaseWaypointProviderEntityTrait {
}

impl ClientCreatureFollowWaypointProviderEntityTrait for ClientCreatureFollowWaypointProviderEntity {
}

impl CreatureBaseWaypointProviderEntityTrait for ClientCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityTrait for ClientCreatureFollowWaypointProviderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureFollowWaypointProviderEntity {
}

pub static CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureFollowWaypointProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointClosestChooserEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointClosestChooserEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointClosestChooserEntityTrait for CreatureFollowWaypointClosestChooserEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointClosestChooserEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointClosestChooserEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointClosestChooserEntity {
}

pub static CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointClosestChooserEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointClosestChooserEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointClosestChooserEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointOccupancyChooserEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointOccupancyChooserEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointOccupancyChooserEntityTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointOccupancyChooserEntity {
}

pub static CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointOccupancyChooserEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointOccupancyChooserEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointOccupancyChooserEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureFollowWaypointBoolChooserEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait CreatureFollowWaypointBoolChooserEntityTrait: CreatureFollowBaseEntityTrait {
}

impl CreatureFollowWaypointBoolChooserEntityTrait for CreatureFollowWaypointBoolChooserEntity {
}

impl CreatureFollowBaseEntityTrait for CreatureFollowWaypointBoolChooserEntity {
}

impl super::entity::EntityTrait for CreatureFollowWaypointBoolChooserEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureFollowWaypointBoolChooserEntity {
}

pub static CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureFollowWaypointBoolChooserEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointBoolChooserEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointBoolChooserEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureConfigurationProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CreatureConfigurationProviderEntityTrait: super::entity::EntityTrait {
}

impl CreatureConfigurationProviderEntityTrait for CreatureConfigurationProviderEntity {
}

impl super::entity::EntityTrait for CreatureConfigurationProviderEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureConfigurationProviderEntity {
}

pub static CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureConfigurationProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureConfigurationProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureConfigurationProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureBaseWaypointProviderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CreatureBaseWaypointProviderEntityTrait: super::entity::EntityTrait {
}

impl CreatureBaseWaypointProviderEntityTrait for CreatureBaseWaypointProviderEntity {
}

impl super::entity::EntityTrait for CreatureBaseWaypointProviderEntity {
}

impl super::entity::EntityBusPeerTrait for CreatureBaseWaypointProviderEntity {
}

pub static CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureBaseWaypointProviderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureBaseWaypointProviderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureBaseWaypointProviderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLInfluenceFilterEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CLInfluenceFilterEntityTrait: super::entity::EntityTrait {
}

impl CLInfluenceFilterEntityTrait for CLInfluenceFilterEntity {
}

impl super::entity::EntityTrait for CLInfluenceFilterEntity {
}

impl super::entity::EntityBusPeerTrait for CLInfluenceFilterEntity {
}

pub static CLINFLUENCEFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLInfluenceFilterEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLInfluenceFilterEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLINFLUENCEFILTERENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLInfluenceFilterEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLInfluenceCompareEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CLInfluenceCompareEntityTrait: super::entity::EntityTrait {
}

impl CLInfluenceCompareEntityTrait for CLInfluenceCompareEntity {
}

impl super::entity::EntityTrait for CLInfluenceCompareEntity {
}

impl super::entity::EntityBusPeerTrait for CLInfluenceCompareEntity {
}

pub static CLINFLUENCECOMPAREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLInfluenceCompareEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLInfluenceCompareEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLINFLUENCECOMPAREENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLInfluenceCompareEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CLApplyInfluenceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait CLApplyInfluenceEntityTrait: super::entity::EntityTrait {
}

impl CLApplyInfluenceEntityTrait for CLApplyInfluenceEntity {
}

impl super::entity::EntityTrait for CLApplyInfluenceEntity {
}

impl super::entity::EntityBusPeerTrait for CLApplyInfluenceEntity {
}

pub static CLAPPLYINFLUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CLApplyInfluenceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLApplyInfluenceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLAPPLYINFLUENCEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLApplyInfluenceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCreatureLocoMotorEntity {
    pub _glacier_base: CreatureLocoEntity,
}

pub trait ClientCreatureLocoMotorEntityTrait: CreatureLocoEntityTrait {
}

impl ClientCreatureLocoMotorEntityTrait for ClientCreatureLocoMotorEntity {
}

impl CreatureLocoEntityTrait for ClientCreatureLocoMotorEntity {
}

impl super::a_i_tools::LocoEntityTrait for ClientCreatureLocoMotorEntity {
}

impl super::entity::EntityTrait for ClientCreatureLocoMotorEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureLocoMotorEntity {
}

pub static CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureLocoMotorEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureLocoMotorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureLocoMotorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureLocoMotorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureLocoMotorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCreatureFollowWaypointsEntity {
    pub _glacier_base: ClientCreatureFollowWaypointSegmentEntity,
}

pub trait ClientCreatureFollowWaypointsEntityTrait: ClientCreatureFollowWaypointSegmentEntityTrait {
}

impl ClientCreatureFollowWaypointsEntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl ClientCreatureFollowWaypointSegmentEntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl CreatureFollowBaseEntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl super::entity::EntityTrait for ClientCreatureFollowWaypointsEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureFollowWaypointsEntity {
}

pub static CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureFollowWaypointsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCreatureFollowWaypointSegmentEntity {
    pub _glacier_base: CreatureFollowBaseEntity,
}

pub trait ClientCreatureFollowWaypointSegmentEntityTrait: CreatureFollowBaseEntityTrait {
}

impl ClientCreatureFollowWaypointSegmentEntityTrait for ClientCreatureFollowWaypointSegmentEntity {
}

impl CreatureFollowBaseEntityTrait for ClientCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityTrait for ClientCreatureFollowWaypointSegmentEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureFollowWaypointSegmentEntity {
}

pub static CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointSegmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureFollowWaypointSegmentEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointSegmentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointSegmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointSegmentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientCreatureCollisionGroupEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientCreatureCollisionGroupEntityTrait: super::entity::EntityTrait {
}

impl ClientCreatureCollisionGroupEntityTrait for ClientCreatureCollisionGroupEntity {
}

impl super::entity::EntityTrait for ClientCreatureCollisionGroupEntity {
}

impl super::entity::EntityBusPeerTrait for ClientCreatureCollisionGroupEntity {
}

pub static CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureCollisionGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientCreatureCollisionGroupEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureCollisionGroupEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureCollisionGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureCollisionGroupEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CreatureSpawnEntityData {
    pub _glacier_base: super::dice_commons_shared::BlueprintSpawnReferenceObjectData,
}

pub trait CreatureSpawnEntityDataTrait: super::dice_commons_shared::BlueprintSpawnReferenceObjectDataTrait {
}

impl CreatureSpawnEntityDataTrait for CreatureSpawnEntityData {
}

impl super::dice_commons_shared::BlueprintSpawnReferenceObjectDataTrait for CreatureSpawnEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn initial_auto_spawn(&self) -> &bool {
        self._glacier_base.initial_auto_spawn()
    }
    fn auto_spawn(&self) -> &bool {
        self._glacier_base.auto_spawn()
    }
    fn queue_spawn_event(&self) -> &bool {
        self._glacier_base.queue_spawn_event()
    }
    fn use_as_spawn_point(&self) -> &bool {
        self._glacier_base.use_as_spawn_point()
    }
    fn spawns_occupy_locations(&self) -> &bool {
        self._glacier_base.spawns_occupy_locations()
    }
    fn initial_spawn_delay(&self) -> &f32 {
        self._glacier_base.initial_spawn_delay()
    }
    fn spawn_delay(&self) -> &f32 {
        self._glacier_base.spawn_delay()
    }
    fn max_count(&self) -> &i32 {
        self._glacier_base.max_count()
    }
    fn max_count_simultaneously(&self) -> &i32 {
        self._glacier_base.max_count_simultaneously()
    }
}

impl super::entity::ReferenceObjectDataTrait for CreatureSpawnEntityData {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::BlueprintTrait>>> {
        self._glacier_base.blueprint()
    }
    fn object_variation(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectVariationTrait>>> {
        self._glacier_base.object_variation()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
}

impl super::entity::GameObjectDataTrait for CreatureSpawnEntityData {
}

impl super::core::DataBusPeerTrait for CreatureSpawnEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for CreatureSpawnEntityData {
}

impl super::core::DataContainerTrait for CreatureSpawnEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CREATURESPAWNENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpawnEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::dice_commons_shared::BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CreatureSpawnEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CreatureSpawnEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CREATURESPAWNENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpawnEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureSpawnEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerCreatureLocoMotorEntity {
    pub _glacier_base: CreatureLocoEntity,
}

pub trait ServerCreatureLocoMotorEntityTrait: CreatureLocoEntityTrait {
}

impl ServerCreatureLocoMotorEntityTrait for ServerCreatureLocoMotorEntity {
}

impl CreatureLocoEntityTrait for ServerCreatureLocoMotorEntity {
}

impl super::a_i_tools::LocoEntityTrait for ServerCreatureLocoMotorEntity {
}

impl super::entity::EntityTrait for ServerCreatureLocoMotorEntity {
}

impl super::entity::EntityBusPeerTrait for ServerCreatureLocoMotorEntity {
}

pub static SERVERCREATURELOCOMOTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureLocoMotorEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerCreatureLocoMotorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureLocoMotorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERCREATURELOCOMOTORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureLocoMotorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureLocoMotorEntity"),
    array_type: None,
    alignment: 8,
};


