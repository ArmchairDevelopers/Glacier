use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCreatureFollowWaypointsEntity {
}

pub const SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointsEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO
    }
}


pub const SERVERCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCreatureFollowWaypointSegmentEntity {
}

pub const SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointSegmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointSegmentEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO
    }
}


pub const SERVERCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointSegmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointSegmentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCreatureCollisionGroupEntity {
}

pub const SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureCollisionGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureCollisionGroupEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCREATURECOLLISIONGROUPENTITY_TYPE_INFO
    }
}


pub const SERVERCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureCollisionGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureCollisionGroupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IMovementProvider {
}

pub const IMOVEMENTPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMovementProvider",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IMOVEMENTPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IMovementProvider {
    fn type_info() -> &'static TypeInfo {
        IMOVEMENTPROVIDER_TYPE_INFO
    }
}


pub const IMOVEMENTPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMovementProvider-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("IMovementProvider-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ISteeringProvider {
}

pub const ISTEERINGPROVIDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ISteeringProvider",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ISTEERINGPROVIDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ISteeringProvider {
    fn type_info() -> &'static TypeInfo {
        ISTEERINGPROVIDER_TYPE_INFO
    }
}


pub const ISTEERINGPROVIDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ISteeringProvider-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ISteeringProvider-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureWaypoint_PlayAnimation {
}

pub const CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_PlayAnimation",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREWAYPOINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypoint_PlayAnimation {
    fn type_info() -> &'static TypeInfo {
        CREATUREWAYPOINT_PLAYANIMATION_TYPE_INFO
    }
}


pub const CREATUREWAYPOINT_PLAYANIMATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_PlayAnimation-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint_PlayAnimation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureWaypoint_Pause {
}

pub const CREATUREWAYPOINT_PAUSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_Pause",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREWAYPOINT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypoint_Pause {
    fn type_info() -> &'static TypeInfo {
        CREATUREWAYPOINT_PAUSE_TYPE_INFO
    }
}


pub const CREATUREWAYPOINT_PAUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint_Pause-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint_Pause-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureWaypoint {
}

pub const CREATUREWAYPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(CREATUREWAYPOINT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureWaypoint {
    fn type_info() -> &'static TypeInfo {
        CREATUREWAYPOINT_TYPE_INFO
    }
}


pub const CREATUREWAYPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureWaypoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureWaypoint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CL_ProceduralMotion {
}

pub const CL_PROCEDURALMOTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_ProceduralMotion",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOVEMENTPROVIDER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CL_PROCEDURALMOTION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CL_ProceduralMotion {
    fn type_info() -> &'static TypeInfo {
        CL_PROCEDURALMOTION_TYPE_INFO
    }
}


pub const CL_PROCEDURALMOTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_ProceduralMotion-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CL_ProceduralMotion-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CL_CurveSteering {
}

pub const CL_CURVESTEERING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_CurveSteering",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ISTEERINGPROVIDER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CL_CURVESTEERING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CL_CurveSteering {
    fn type_info() -> &'static TypeInfo {
        CL_CURVESTEERING_TYPE_INFO
    }
}


pub const CL_CURVESTEERING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CL_CurveSteering-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CL_CurveSteering-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLColAvoidingSteering {
}

pub const CLCOLAVOIDINGSTEERING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLColAvoidingSteering",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CL_CURVESTEERING_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLColAvoidingSteering {
    fn type_info() -> &'static TypeInfo {
        CLCOLAVOIDINGSTEERING_TYPE_INFO
    }
}


pub const CLCOLAVOIDINGSTEERING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLColAvoidingSteering-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLColAvoidingSteering-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IAssessor {
}

pub const IASSESSOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IAssessor",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IASSESSOR_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IAssessor {
    fn type_info() -> &'static TypeInfo {
        IASSESSOR_TYPE_INFO
    }
}


pub const IASSESSOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IAssessor-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("IAssessor-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLClientState {
}

pub const CLCLIENTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLClientState",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLSTATE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLCLIENTSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLClientState {
    fn type_info() -> &'static TypeInfo {
        CLCLIENTSTATE_TYPE_INFO
    }
}


pub const CLCLIENTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLClientState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLClientState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLState {
}

pub const CLSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLState",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IASSESSOR_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLState {
    fn type_info() -> &'static TypeInfo {
        CLSTATE_TYPE_INFO
    }
}


pub const CLSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLConduitState {
}

pub const CLCONDUITSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLConduitState",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLSTATE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLCONDUITSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLConduitState {
    fn type_info() -> &'static TypeInfo {
        CLCONDUITSTATE_TYPE_INFO
    }
}


pub const CLCONDUITSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLConduitState-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLConduitState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCreatureSpawnEntity {
}

pub const CLIENTCREATURESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTBLUEPRINTSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCREATURESPAWNENTITY_TYPE_INFO
    }
}


pub const CLIENTCREATURESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCreatureSpawnEntity {
}

pub const SERVERCREATURESPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureSpawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SERVERBLUEPRINTSPAWNENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureSpawnEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCREATURESPAWNENTITY_TYPE_INFO
    }
}


pub const SERVERCREATURESPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureSpawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureSpawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureLocoEntity {
}

pub const CREATURELOCOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCOENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATURELOCOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureLocoEntity {
    fn type_info() -> &'static TypeInfo {
        CREATURELOCOENTITY_TYPE_INFO
    }
}


pub const CREATURELOCOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureLocoEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureLocoEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointUnspawnEntity {
}

pub const CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointUnspawnEntity {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTUNSPAWNENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointUnspawnEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointUnspawnEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowBaseEntity {
}

pub const CREATUREFOLLOWBASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowBaseEntity {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWBASEENTITY_TYPE_INFO
    }
}


pub const CREATUREFOLLOWBASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowBaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowBaseEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCreatureFollowWaypointProviderEntity {
}

pub const SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureFollowWaypointProviderEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO
    }
}


pub const SERVERCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureFollowWaypointProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureFollowWaypointProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCreatureFollowWaypointProviderEntity {
}

pub const CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointProviderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_TYPE_INFO
    }
}


pub const CLIENTCREATUREFOLLOWWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointClosestChooserEntity {
}

pub const CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointClosestChooserEntity {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTCLOSESTCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointClosestChooserEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointClosestChooserEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointOccupancyChooserEntity {
}

pub const CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointOccupancyChooserEntity {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTOCCUPANCYCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointOccupancyChooserEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointOccupancyChooserEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureFollowWaypointBoolChooserEntity {
}

pub const CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureFollowWaypointBoolChooserEntity {
    fn type_info() -> &'static TypeInfo {
        CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_TYPE_INFO
    }
}


pub const CREATUREFOLLOWWAYPOINTBOOLCHOOSERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureFollowWaypointBoolChooserEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureFollowWaypointBoolChooserEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureConfigurationProviderEntity {
}

pub const CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureConfigurationProviderEntity {
    fn type_info() -> &'static TypeInfo {
        CREATURECONFIGURATIONPROVIDERENTITY_TYPE_INFO
    }
}


pub const CREATURECONFIGURATIONPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureConfigurationProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureConfigurationProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreatureBaseWaypointProviderEntity {
}

pub const CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CreatureBaseWaypointProviderEntity {
    fn type_info() -> &'static TypeInfo {
        CREATUREBASEWAYPOINTPROVIDERENTITY_TYPE_INFO
    }
}


pub const CREATUREBASEWAYPOINTPROVIDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureBaseWaypointProviderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureBaseWaypointProviderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLInfluenceFilterEntity {
}

pub const CLINFLUENCEFILTERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLInfluenceFilterEntity {
    fn type_info() -> &'static TypeInfo {
        CLINFLUENCEFILTERENTITY_TYPE_INFO
    }
}


pub const CLINFLUENCEFILTERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceFilterEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLInfluenceFilterEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLInfluenceCompareEntity {
}

pub const CLINFLUENCECOMPAREENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLInfluenceCompareEntity {
    fn type_info() -> &'static TypeInfo {
        CLINFLUENCECOMPAREENTITY_TYPE_INFO
    }
}


pub const CLINFLUENCECOMPAREENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLInfluenceCompareEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLInfluenceCompareEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CLApplyInfluenceEntity {
}

pub const CLAPPLYINFLUENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CLApplyInfluenceEntity {
    fn type_info() -> &'static TypeInfo {
        CLAPPLYINFLUENCEENTITY_TYPE_INFO
    }
}


pub const CLAPPLYINFLUENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CLApplyInfluenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CLApplyInfluenceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCreatureLocoMotorEntity {
}

pub const CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureLocoMotorEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureLocoMotorEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCREATURELOCOMOTORENTITY_TYPE_INFO
    }
}


pub const CLIENTCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureLocoMotorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureLocoMotorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCreatureFollowWaypointsEntity {
}

pub const CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointsEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointsEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTSENTITY_TYPE_INFO
    }
}


pub const CLIENTCREATUREFOLLOWWAYPOINTSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCreatureFollowWaypointSegmentEntity {
}

pub const CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointSegmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATUREFOLLOWBASEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureFollowWaypointSegmentEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_TYPE_INFO
    }
}


pub const CLIENTCREATUREFOLLOWWAYPOINTSEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureFollowWaypointSegmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureFollowWaypointSegmentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientCreatureCollisionGroupEntity {
}

pub const CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureCollisionGroupEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientCreatureCollisionGroupEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTCREATURECOLLISIONGROUPENTITY_TYPE_INFO
    }
}


pub const CLIENTCREATURECOLLISIONGROUPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientCreatureCollisionGroupEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ClientCreatureCollisionGroupEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CreatureSpawnEntityData {
}

pub const CREATURESPAWNENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpawnEntityData",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BLUEPRINTSPAWNREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CreatureSpawnEntityData {
    fn type_info() -> &'static TypeInfo {
        CREATURESPAWNENTITYDATA_TYPE_INFO
    }
}


pub const CREATURESPAWNENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CreatureSpawnEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("CreatureSpawnEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerCreatureLocoMotorEntity {
}

pub const SERVERCREATURELOCOMOTORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureLocoMotorEntity",
    flags: MemberInfoFlags::new(101),
    module: "CreatureLoco",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CREATURELOCOENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerCreatureLocoMotorEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERCREATURELOCOMOTORENTITY_TYPE_INFO
    }
}


pub const SERVERCREATURELOCOMOTORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerCreatureLocoMotorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "CreatureLoco",
    data: TypeInfoData::Array("ServerCreatureLocoMotorEntity-Array"),
    array_type: None,
    alignment: 8,
};


