use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_pathfinding_types(registry: &mut TypeRegistry) {
    registry.register_type(OBSTACLECONTROLLERENTITYDATA_TYPE_INFO);
    registry.register_type(OBSTACLECONTROLLERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGNAVMESHVOLUMEDATA_TYPE_INFO);
    registry.register_type(PATHFINDINGNAVMESHVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBSTACLEDAT_TYPE_INFO);
    registry.register_type(OBSTACLEDAT_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMOBSTACLEDATA_TYPE_INFO);
    registry.register_type(CUSTOMOBSTACLEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHLINKENTITYDATA_TYPE_INFO);
    registry.register_type(PATHLINKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHLINKDIRECTION_TYPE_INFO);
    registry.register_type(PATHLINKDIRECTION_ARRAY_TYPE_INFO);
    registry.register_type(MOVERFOLLOWLEADERENTITYDATA_TYPE_INFO);
    registry.register_type(MOVERFOLLOWLEADERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVERFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO);
    registry.register_type(MOVERFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVERCOMPONENTDATA_TYPE_INFO);
    registry.register_type(MOVERCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYMOVERTYPE_TYPE_INFO);
    registry.register_type(ENTITYMOVERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MOVERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(MOVERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(FOLLOWERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNEOVERRIDE_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(IDLETUNEOVERRIDE_TYPE_INFO);
    registry.register_type(IDLETUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(GOALTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(GOALTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(PROBERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(PROBERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(JUMPERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(JUMPERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(CAUTIONTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(CAUTIONTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(TURNINPLACETUNEOVERRIDE_TYPE_INFO);
    registry.register_type(TURNINPLACETUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(RADIUSDATAOVERRIDE_TYPE_INFO);
    registry.register_type(RADIUSDATAOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(MOVERTUNE_TYPE_INFO);
    registry.register_type(MOVERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(PATHCREATIONOPTIONS_TYPE_INFO);
    registry.register_type(PATHCREATIONOPTIONS_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWERTUNE_TYPE_INFO);
    registry.register_type(FOLLOWERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNE_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNE_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNE_ARRAY_TYPE_INFO);
    registry.register_type(IDLETUNE_TYPE_INFO);
    registry.register_type(IDLETUNE_ARRAY_TYPE_INFO);
    registry.register_type(GOALTUNE_TYPE_INFO);
    registry.register_type(GOALTUNE_ARRAY_TYPE_INFO);
    registry.register_type(PROBERTUNE_TYPE_INFO);
    registry.register_type(PROBERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(JUMPERTUNE_TYPE_INFO);
    registry.register_type(JUMPERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(CAUTIONTUNE_TYPE_INFO);
    registry.register_type(CAUTIONTUNE_ARRAY_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNE_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNE_ARRAY_TYPE_INFO);
    registry.register_type(TURNINPLACETUNE_TYPE_INFO);
    registry.register_type(TURNINPLACETUNE_ARRAY_TYPE_INFO);
    registry.register_type(RADIUSDATA_TYPE_INFO);
    registry.register_type(RADIUSDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLOCKAGEMODE_TYPE_INFO);
    registry.register_type(BLOCKAGEMODE_ARRAY_TYPE_INFO);
    registry.register_type(ORIENTMODE_TYPE_INFO);
    registry.register_type(ORIENTMODE_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWMOVERSPEC_TYPE_INFO);
    registry.register_type(FOLLOWMOVERSPEC_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWFORMATION_TYPE_INFO);
    registry.register_type(FOLLOWFORMATION_ARRAY_TYPE_INFO);
    registry.register_type(STOPSPEC_TYPE_INFO);
    registry.register_type(STOPSPEC_ARRAY_TYPE_INFO);
    registry.register_type(GOTOPOSSPEC_TYPE_INFO);
    registry.register_type(GOTOPOSSPEC_ARRAY_TYPE_INFO);
    registry.register_type(PATHSPEC_TYPE_INFO);
    registry.register_type(PATHSPEC_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCE_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(SERVEROBSTACLECONTROLLERENTITY_TYPE_INFO);
    registry.register_type(SERVEROBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERNAVPOWERSYSTEMENTITY_TYPE_INFO);
    registry.register_type(SERVERNAVPOWERSYSTEMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OBSTACLECONTROLLERENTITY_TYPE_INFO);
    registry.register_type(OBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(NAVPOWERPATHSPEC_TYPE_INFO);
    registry.register_type(NAVPOWERPATHSPEC_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct ObstacleControllerEntityData {
    pub active_at_start: bool,
    pub obstacle_data: ObstacleDat,
}

pub const OBSTACLECONTROLLERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ActiveAtStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ObstacleControllerEntityData, active_at_start),
            },
            FieldInfoData {
                name: "ObstacleData",
                flags: MemberInfoFlags::new(0),
                field_type: OBSTACLEDAT_TYPE_INFO,
                rust_offset: offset_of!(ObstacleControllerEntityData, obstacle_data),
            },
        ],
    }),
    array_type: Some(OBSTACLECONTROLLERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObstacleControllerEntityData {
    fn type_info() -> &'static TypeInfo {
        OBSTACLECONTROLLERENTITYDATA_TYPE_INFO
    }
}


pub const OBSTACLECONTROLLERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleControllerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PathfindingNavMeshVolumeData {
    pub category: super::entity::PathfindingObjectCategoryAsset,
}

pub const PATHFINDINGNAVMESHVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingNavMeshVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBBDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Category",
                flags: MemberInfoFlags::new(0),
                field_type: PATHFINDINGOBJECTCATEGORYASSET_TYPE_INFO,
                rust_offset: offset_of!(PathfindingNavMeshVolumeData, category),
            },
        ],
    }),
    array_type: Some(PATHFINDINGNAVMESHVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PathfindingNavMeshVolumeData {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGNAVMESHVOLUMEDATA_TYPE_INFO
    }
}


pub const PATHFINDINGNAVMESHVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingNavMeshVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathfindingNavMeshVolumeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ObstacleDat {
    pub layer_mask: u32,
    pub penalty_mult: f32,
    pub obstacle_blockage_flags: u32,
    pub user_data: CustomObstacleData,
    pub obstacle_name: String,
}

pub const OBSTACLEDAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleDat",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ObstacleDat, layer_mask),
            },
            FieldInfoData {
                name: "PenaltyMult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ObstacleDat, penalty_mult),
            },
            FieldInfoData {
                name: "ObstacleBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ObstacleDat, obstacle_blockage_flags),
            },
            FieldInfoData {
                name: "UserData",
                flags: MemberInfoFlags::new(0),
                field_type: CUSTOMOBSTACLEDATA_TYPE_INFO,
                rust_offset: offset_of!(ObstacleDat, user_data),
            },
            FieldInfoData {
                name: "ObstacleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ObstacleDat, obstacle_name),
            },
        ],
    }),
    array_type: Some(OBSTACLEDAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObstacleDat {
    fn type_info() -> &'static TypeInfo {
        OBSTACLEDAT_TYPE_INFO
    }
}


pub const OBSTACLEDAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleDat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleDat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomObstacleData {
}

pub const CUSTOMOBSTACLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomObstacleData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CUSTOMOBSTACLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomObstacleData {
    fn type_info() -> &'static TypeInfo {
        CUSTOMOBSTACLEDATA_TYPE_INFO
    }
}


pub const CUSTOMOBSTACLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomObstacleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CustomObstacleData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PathLinkEntityData {
    pub points: Vec<super::core::Vec3>,
    pub direction: PathLinkDirection,
    pub link_dat: super::pathfinding_shared::LinkDat,
    pub active_at_start: bool,
    pub deferred_creation: bool,
}

pub const PATHLINKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PathLinkEntityData, points),
            },
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: PATHLINKDIRECTION_TYPE_INFO,
                rust_offset: offset_of!(PathLinkEntityData, direction),
            },
            FieldInfoData {
                name: "LinkDat",
                flags: MemberInfoFlags::new(0),
                field_type: LINKDAT_TYPE_INFO,
                rust_offset: offset_of!(PathLinkEntityData, link_dat),
            },
            FieldInfoData {
                name: "ActiveAtStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathLinkEntityData, active_at_start),
            },
            FieldInfoData {
                name: "DeferredCreation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathLinkEntityData, deferred_creation),
            },
        ],
    }),
    array_type: Some(PATHLINKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PathLinkEntityData {
    fn type_info() -> &'static TypeInfo {
        PATHLINKENTITYDATA_TYPE_INFO
    }
}


pub const PATHLINKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathLinkEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PathLinkDirection {
    #[default]
    PathLinkDirection_Forward = 0,
    PathLinkDirection_Backward = 1,
    PathLinkDirection_Both = 2,
}

pub const PATHLINKDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkDirection",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(PATHLINKDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PathLinkDirection {
    fn type_info() -> &'static TypeInfo {
        PATHLINKDIRECTION_TYPE_INFO
    }
}


pub const PATHLINKDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathLinkDirection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MoverFollowLeaderEntityData {
    pub following_parameters: FollowMoverSpec,
    pub flock_id: u32,
}

pub const MOVERFOLLOWLEADERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowLeaderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FollowingParameters",
                flags: MemberInfoFlags::new(0),
                field_type: FOLLOWMOVERSPEC_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowLeaderEntityData, following_parameters),
            },
            FieldInfoData {
                name: "FlockId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowLeaderEntityData, flock_id),
            },
        ],
    }),
    array_type: Some(MOVERFOLLOWLEADERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverFollowLeaderEntityData {
    fn type_info() -> &'static TypeInfo {
        MOVERFOLLOWLEADERENTITYDATA_TYPE_INFO
    }
}


pub const MOVERFOLLOWLEADERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowLeaderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverFollowLeaderEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MoverFollowWaypointsEntityData {
    pub type_of_route: super::pathfinding_shared::RouteType,
    pub stop_at_waypoints: bool,
    pub start_at_geometrically_closest_waypoint: bool,
    pub intermediate_allowed_stop_dist_override: f32,
    pub destination_allowed_stop_dist_override: f32,
    pub destination_set_orientation: bool,
}

pub const MOVERFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowWaypointsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TypeOfRoute",
                flags: MemberInfoFlags::new(0),
                field_type: ROUTETYPE_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, type_of_route),
            },
            FieldInfoData {
                name: "StopAtWaypoints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, stop_at_waypoints),
            },
            FieldInfoData {
                name: "StartAtGeometricallyClosestWaypoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, start_at_geometrically_closest_waypoint),
            },
            FieldInfoData {
                name: "IntermediateAllowedStopDistOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, intermediate_allowed_stop_dist_override),
            },
            FieldInfoData {
                name: "DestinationAllowedStopDistOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, destination_allowed_stop_dist_override),
            },
            FieldInfoData {
                name: "DestinationSetOrientation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, destination_set_orientation),
            },
        ],
    }),
    array_type: Some(MOVERFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverFollowWaypointsEntityData {
    fn type_info() -> &'static TypeInfo {
        MOVERFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO
    }
}


pub const MOVERFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowWaypointsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverFollowWaypointsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MoverComponentData {
    pub r#type: EntityMoverType,
    pub mover_tune: MoverTune,
    pub goal_plan_failure_treshold: f32,
    pub goal_height_failure_treshold: f32,
    pub radius_data: RadiusData,
    pub enable_puppet_mode: bool,
    pub move_speed_modifier: f32,
    pub desired_movement_angle_game_state: super::ant::AntRef,
    pub desired_relative_movement_angle_game_state: super::ant::AntRef,
    pub desired_movement_speed_game_state: super::ant::AntRef,
    pub desired_facing_angle_game_state: super::ant::AntRef,
    pub desired_relative_facing_angle_game_state: super::ant::AntRef,
    pub distance_to_goal_game_state: super::ant::AntRef,
}

pub const MOVERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYMOVERTYPE_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, r#type),
            },
            FieldInfoData {
                name: "moverTune",
                flags: MemberInfoFlags::new(0),
                field_type: MOVERTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, mover_tune),
            },
            FieldInfoData {
                name: "goalPlanFailureTreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, goal_plan_failure_treshold),
            },
            FieldInfoData {
                name: "goalHeightFailureTreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, goal_height_failure_treshold),
            },
            FieldInfoData {
                name: "radiusData",
                flags: MemberInfoFlags::new(0),
                field_type: RADIUSDATA_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, radius_data),
            },
            FieldInfoData {
                name: "EnablePuppetMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, enable_puppet_mode),
            },
            FieldInfoData {
                name: "MoveSpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, move_speed_modifier),
            },
            FieldInfoData {
                name: "DesiredMovementAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, desired_movement_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredRelativeMovementAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, desired_relative_movement_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredMovementSpeedGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, desired_movement_speed_game_state),
            },
            FieldInfoData {
                name: "DesiredFacingAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, desired_facing_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredRelativeFacingAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, desired_relative_facing_angle_game_state),
            },
            FieldInfoData {
                name: "DistanceToGoalGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(MoverComponentData, distance_to_goal_game_state),
            },
        ],
    }),
    array_type: Some(MOVERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MoverComponentData {
    fn type_info() -> &'static TypeInfo {
        MOVERCOMPONENTDATA_TYPE_INFO
    }
}


pub const MOVERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EntityMoverType {
    #[default]
    EntityMoverType_Only_Repulsor = 0,
    EntityMoverType_Both_MoverRepulsor = 1,
}

pub const ENTITYMOVERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityMoverType",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(ENTITYMOVERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntityMoverType {
    fn type_info() -> &'static TypeInfo {
        ENTITYMOVERTYPE_TYPE_INFO
    }
}


pub const ENTITYMOVERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityMoverType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("EntityMoverType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MoverTuneOverride {
}

pub const MOVERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVERTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MOVERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverTuneOverride {
    fn type_info() -> &'static TypeInfo {
        MOVERTUNEOVERRIDE_TYPE_INFO
    }
}


pub const MOVERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FollowerTuneOverride {
}

pub const FOLLOWERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FOLLOWERTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FOLLOWERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FollowerTuneOverride {
    fn type_info() -> &'static TypeInfo {
        FOLLOWERTUNEOVERRIDE_TYPE_INFO
    }
}


pub const FOLLOWERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowerTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceOrientTuneOverride {
}

pub const SURFACEORIENTTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SURFACEORIENTTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SURFACEORIENTTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceOrientTuneOverride {
    fn type_info() -> &'static TypeInfo {
        SURFACEORIENTTUNEOVERRIDE_TYPE_INFO
    }
}


pub const SURFACEORIENTTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("SurfaceOrientTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoObstacleTuneOverride {
}

pub const AUTOOBSTACLETUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOOBSTACLETUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOOBSTACLETUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoObstacleTuneOverride {
    fn type_info() -> &'static TypeInfo {
        AUTOOBSTACLETUNEOVERRIDE_TYPE_INFO
    }
}


pub const AUTOOBSTACLETUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("AutoObstacleTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IdleTuneOverride {
}

pub const IDLETUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IDLETUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IDLETUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleTuneOverride {
    fn type_info() -> &'static TypeInfo {
        IDLETUNEOVERRIDE_TYPE_INFO
    }
}


pub const IDLETUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("IdleTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GoalTuneOverride {
}

pub const GOALTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GOALTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GOALTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GoalTuneOverride {
    fn type_info() -> &'static TypeInfo {
        GOALTUNEOVERRIDE_TYPE_INFO
    }
}


pub const GOALTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GoalTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProberTuneOverride {
}

pub const PROBERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROBERTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROBERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProberTuneOverride {
    fn type_info() -> &'static TypeInfo {
        PROBERTUNEOVERRIDE_TYPE_INFO
    }
}


pub const PROBERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ProberTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct JumperTuneOverride {
}

pub const JUMPERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(JUMPERTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(JUMPERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for JumperTuneOverride {
    fn type_info() -> &'static TypeInfo {
        JUMPERTUNEOVERRIDE_TYPE_INFO
    }
}


pub const JUMPERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("JumperTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CautionTuneOverride {
}

pub const CAUTIONTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAUTIONTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CAUTIONTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CautionTuneOverride {
    fn type_info() -> &'static TypeInfo {
        CAUTIONTUNEOVERRIDE_TYPE_INFO
    }
}


pub const CAUTIONTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CautionTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RepulsionAccelerationTuneOverride {
}

pub const REPULSIONACCELERATIONTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REPULSIONACCELERATIONTUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(REPULSIONACCELERATIONTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RepulsionAccelerationTuneOverride {
    fn type_info() -> &'static TypeInfo {
        REPULSIONACCELERATIONTUNEOVERRIDE_TYPE_INFO
    }
}


pub const REPULSIONACCELERATIONTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RepulsionAccelerationTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TurnInPlaceTuneOverride {
}

pub const TURNINPLACETUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TURNINPLACETUNE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TURNINPLACETUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnInPlaceTuneOverride {
    fn type_info() -> &'static TypeInfo {
        TURNINPLACETUNEOVERRIDE_TYPE_INFO
    }
}


pub const TURNINPLACETUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("TurnInPlaceTuneOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiusDataOverride {
}

pub const RADIUSDATAOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusDataOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RADIUSDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RADIUSDATAOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RadiusDataOverride {
    fn type_info() -> &'static TypeInfo {
        RADIUSDATAOVERRIDE_TYPE_INFO
    }
}


pub const RADIUSDATAOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusDataOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RadiusDataOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MoverTune {
    pub speed: f32,
    pub max_speed_fraction: f32,
    pub radius_data: RadiusData,
    pub bulk: f32,
    pub cruise_acc: f32,
    pub start_stop_acc: f32,
    pub repulsor_type: i32,
    pub flock_acc: f32,
    pub max_flock_acc_dist: f32,
    pub path_acc: f32,
    pub caution_tune: CautionTune,
    pub backpedal_fraction: f32,
    pub plan_layer: u32,
    pub path_sharing_penalty: f32,
    pub obstacle_mode: BlockageMode,
    pub obstacle_blockage_flags: u32,
    pub auto_ob_tune: AutoObstacleTune,
    pub repulsor_blockage_flags: u32,
    pub repulsor_identity_flags: u32,
    pub link_usage_flags: u32,
    pub path_options: PathCreationOptions,
    pub jumper_tune: JumperTune,
    pub exit_puppet_in_obstacles: bool,
    pub prober_tune: ProberTune,
    pub allow_detour: bool,
    pub goal_tune: GoalTune,
    pub idle_tune: IdleTune,
    pub turn_in_place: TurnInPlaceTune,
    pub repulsion_acceleration_tune: RepulsionAccelerationTune,
    pub surface_orient_tune: SurfaceOrientTune,
    pub sidestep_fraction: f32,
    pub custom_geo_match_flags: u32,
    pub client_motion: bool,
    pub follower_tune: FollowerTune,
}

pub const MOVERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, speed),
            },
            FieldInfoData {
                name: "maxSpeedFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, max_speed_fraction),
            },
            FieldInfoData {
                name: "radiusData",
                flags: MemberInfoFlags::new(0),
                field_type: RADIUSDATA_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, radius_data),
            },
            FieldInfoData {
                name: "bulk",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, bulk),
            },
            FieldInfoData {
                name: "cruiseAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, cruise_acc),
            },
            FieldInfoData {
                name: "startStopAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, start_stop_acc),
            },
            FieldInfoData {
                name: "repulsorType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, repulsor_type),
            },
            FieldInfoData {
                name: "flockAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, flock_acc),
            },
            FieldInfoData {
                name: "maxFlockAccDist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, max_flock_acc_dist),
            },
            FieldInfoData {
                name: "pathAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, path_acc),
            },
            FieldInfoData {
                name: "cautionTune",
                flags: MemberInfoFlags::new(0),
                field_type: CAUTIONTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, caution_tune),
            },
            FieldInfoData {
                name: "backpedalFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, backpedal_fraction),
            },
            FieldInfoData {
                name: "planLayer",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, plan_layer),
            },
            FieldInfoData {
                name: "pathSharingPenalty",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, path_sharing_penalty),
            },
            FieldInfoData {
                name: "obstacleMode",
                flags: MemberInfoFlags::new(0),
                field_type: BLOCKAGEMODE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, obstacle_mode),
            },
            FieldInfoData {
                name: "obstacleBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, obstacle_blockage_flags),
            },
            FieldInfoData {
                name: "autoObTune",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOOBSTACLETUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, auto_ob_tune),
            },
            FieldInfoData {
                name: "repulsorBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, repulsor_blockage_flags),
            },
            FieldInfoData {
                name: "repulsorIdentityFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, repulsor_identity_flags),
            },
            FieldInfoData {
                name: "linkUsageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, link_usage_flags),
            },
            FieldInfoData {
                name: "pathOptions",
                flags: MemberInfoFlags::new(0),
                field_type: PATHCREATIONOPTIONS_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, path_options),
            },
            FieldInfoData {
                name: "jumperTune",
                flags: MemberInfoFlags::new(0),
                field_type: JUMPERTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, jumper_tune),
            },
            FieldInfoData {
                name: "exitPuppetInObstacles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, exit_puppet_in_obstacles),
            },
            FieldInfoData {
                name: "proberTune",
                flags: MemberInfoFlags::new(0),
                field_type: PROBERTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, prober_tune),
            },
            FieldInfoData {
                name: "allowDetour",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, allow_detour),
            },
            FieldInfoData {
                name: "goalTune",
                flags: MemberInfoFlags::new(0),
                field_type: GOALTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, goal_tune),
            },
            FieldInfoData {
                name: "idleTune",
                flags: MemberInfoFlags::new(0),
                field_type: IDLETUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, idle_tune),
            },
            FieldInfoData {
                name: "turnInPlace",
                flags: MemberInfoFlags::new(0),
                field_type: TURNINPLACETUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, turn_in_place),
            },
            FieldInfoData {
                name: "repulsionAccelerationTune",
                flags: MemberInfoFlags::new(0),
                field_type: REPULSIONACCELERATIONTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, repulsion_acceleration_tune),
            },
            FieldInfoData {
                name: "surfaceOrientTune",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACEORIENTTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, surface_orient_tune),
            },
            FieldInfoData {
                name: "sidestepFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, sidestep_fraction),
            },
            FieldInfoData {
                name: "customGeoMatchFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, custom_geo_match_flags),
            },
            FieldInfoData {
                name: "clientMotion",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, client_motion),
            },
            FieldInfoData {
                name: "followerTune",
                flags: MemberInfoFlags::new(0),
                field_type: FOLLOWERTUNE_TYPE_INFO,
                rust_offset: offset_of!(MoverTune, follower_tune),
            },
        ],
    }),
    array_type: Some(MOVERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverTune {
    fn type_info() -> &'static TypeInfo {
        MOVERTUNE_TYPE_INFO
    }
}


pub const MOVERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathCreationOptions {
    pub perform_initial_nav_probe: bool,
}

pub const PATHCREATIONOPTIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathCreationOptions",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "performInitialNavProbe",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathCreationOptions, perform_initial_nav_probe),
            },
        ],
    }),
    array_type: Some(PATHCREATIONOPTIONS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathCreationOptions {
    fn type_info() -> &'static TypeInfo {
        PATHCREATIONOPTIONS_TYPE_INFO
    }
}


pub const PATHCREATIONOPTIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathCreationOptions-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathCreationOptions-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FollowerTune {
    pub circulate_enable: bool,
    pub circulate_min_time: f32,
    pub circulate_max_time: f32,
    pub startup_slowness: f32,
    pub startup_bulk: f32,
    pub packing_padding: f32,
}

pub const FOLLOWERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "circulate_enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FollowerTune, circulate_enable),
            },
            FieldInfoData {
                name: "circulate_minTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowerTune, circulate_min_time),
            },
            FieldInfoData {
                name: "circulate_maxTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowerTune, circulate_max_time),
            },
            FieldInfoData {
                name: "startupSlowness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowerTune, startup_slowness),
            },
            FieldInfoData {
                name: "startupBulk",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowerTune, startup_bulk),
            },
            FieldInfoData {
                name: "packingPadding",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowerTune, packing_padding),
            },
        ],
    }),
    array_type: Some(FOLLOWERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FollowerTune {
    fn type_info() -> &'static TypeInfo {
        FOLLOWERTUNE_TYPE_INFO
    }
}


pub const FOLLOWERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowerTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceOrientTune {
    pub surface_orient_threshold: f32,
    pub always_vertical_on_auto_gen: bool,
    pub surface_orient_slerp_time: f32,
}

pub const SURFACEORIENTTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "surfaceOrientThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SurfaceOrientTune, surface_orient_threshold),
            },
            FieldInfoData {
                name: "alwaysVerticalOnAutoGen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SurfaceOrientTune, always_vertical_on_auto_gen),
            },
            FieldInfoData {
                name: "surfaceOrientSlerpTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SurfaceOrientTune, surface_orient_slerp_time),
            },
        ],
    }),
    array_type: Some(SURFACEORIENTTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceOrientTune {
    fn type_info() -> &'static TypeInfo {
        SURFACEORIENTTUNE_TYPE_INFO
    }
}


pub const SURFACEORIENTTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("SurfaceOrientTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoObstacleTune {
    pub auto_create_obstacle: bool,
    pub delay: f32,
    pub obstacle_blockage_flags: u32,
}

pub const AUTOOBSTACLETUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "autoCreateObstacle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoObstacleTune, auto_create_obstacle),
            },
            FieldInfoData {
                name: "delay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoObstacleTune, delay),
            },
            FieldInfoData {
                name: "obstacleBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AutoObstacleTune, obstacle_blockage_flags),
            },
        ],
    }),
    array_type: Some(AUTOOBSTACLETUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoObstacleTune {
    fn type_info() -> &'static TypeInfo {
        AUTOOBSTACLETUNE_TYPE_INFO
    }
}


pub const AUTOOBSTACLETUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("AutoObstacleTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IdleTune {
    pub tether_dist: f32,
    pub return_delay: f32,
}

pub const IDLETUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "tetherDist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IdleTune, tether_dist),
            },
            FieldInfoData {
                name: "returnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IdleTune, return_delay),
            },
        ],
    }),
    array_type: Some(IDLETUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleTune {
    fn type_info() -> &'static TypeInfo {
        IDLETUNE_TYPE_INFO
    }
}


pub const IDLETUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("IdleTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GoalTune {
    pub use_circular_approach: bool,
    pub preferred_turning_radius: f32,
}

pub const GOALTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "useCircularApproach",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GoalTune, use_circular_approach),
            },
            FieldInfoData {
                name: "preferredTurningRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GoalTune, preferred_turning_radius),
            },
        ],
    }),
    array_type: Some(GOALTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GoalTune {
    fn type_info() -> &'static TypeInfo {
        GOALTUNE_TYPE_INFO
    }
}


pub const GOALTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GoalTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProberTune {
    pub probe_for_ground: bool,
    pub probe_interval: f32,
}

pub const PROBERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "probeForGround",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProberTune, probe_for_ground),
            },
            FieldInfoData {
                name: "probeInterval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProberTune, probe_interval),
            },
        ],
    }),
    array_type: Some(PROBERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProberTune {
    fn type_info() -> &'static TypeInfo {
        PROBERTUNE_TYPE_INFO
    }
}


pub const PROBERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ProberTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct JumperTune {
    pub speed: f32,
    pub arc_fraction: f32,
    pub turn_before_jump_angle: f32,
    pub keep_speed_when_swap_to_default: bool,
    pub only_jump_to_end_point: bool,
}

pub const JUMPERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JumperTune, speed),
            },
            FieldInfoData {
                name: "arcFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JumperTune, arc_fraction),
            },
            FieldInfoData {
                name: "turnBeforeJumpAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(JumperTune, turn_before_jump_angle),
            },
            FieldInfoData {
                name: "keepSpeedWhenSwapToDefault",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(JumperTune, keep_speed_when_swap_to_default),
            },
            FieldInfoData {
                name: "onlyJumpToEndPoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(JumperTune, only_jump_to_end_point),
            },
        ],
    }),
    array_type: Some(JUMPERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for JumperTune {
    fn type_info() -> &'static TypeInfo {
        JUMPERTUNE_TYPE_INFO
    }
}


pub const JUMPERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("JumperTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CautionTune {
    pub speed_x: f32,
    pub tight_turn_degrees: f32,
}

pub const CAUTIONTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "speedX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CautionTune, speed_x),
            },
            FieldInfoData {
                name: "tightTurnDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CautionTune, tight_turn_degrees),
            },
        ],
    }),
    array_type: Some(CAUTIONTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CautionTune {
    fn type_info() -> &'static TypeInfo {
        CAUTIONTUNE_TYPE_INFO
    }
}


pub const CAUTIONTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CautionTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RepulsionAccelerationTune {
    pub initial_acc: f32,
    pub outer_cushion_acc: f32,
    pub inner_cushion_acc: f32,
}

pub const REPULSIONACCELERATIONTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "initialAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RepulsionAccelerationTune, initial_acc),
            },
            FieldInfoData {
                name: "outerCushionAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RepulsionAccelerationTune, outer_cushion_acc),
            },
            FieldInfoData {
                name: "innerCushionAcc",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RepulsionAccelerationTune, inner_cushion_acc),
            },
        ],
    }),
    array_type: Some(REPULSIONACCELERATIONTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RepulsionAccelerationTune {
    fn type_info() -> &'static TypeInfo {
        REPULSIONACCELERATIONTUNE_TYPE_INFO
    }
}


pub const REPULSIONACCELERATIONTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RepulsionAccelerationTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TurnInPlaceTune {
    pub when_moving_angle: f32,
    pub when_stopped_angle: f32,
    pub speed: f32,
    pub accel_angle: f32,
    pub slide_spin_threshold: f32,
    pub enable_u_turn: bool,
}

pub const TURNINPLACETUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "whenMovingAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurnInPlaceTune, when_moving_angle),
            },
            FieldInfoData {
                name: "whenStoppedAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurnInPlaceTune, when_stopped_angle),
            },
            FieldInfoData {
                name: "speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurnInPlaceTune, speed),
            },
            FieldInfoData {
                name: "accelAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurnInPlaceTune, accel_angle),
            },
            FieldInfoData {
                name: "slideSpinThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TurnInPlaceTune, slide_spin_threshold),
            },
            FieldInfoData {
                name: "enableUTurn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TurnInPlaceTune, enable_u_turn),
            },
        ],
    }),
    array_type: Some(TURNINPLACETUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnInPlaceTune {
    fn type_info() -> &'static TypeInfo {
        TURNINPLACETUNE_TYPE_INFO
    }
}


pub const TURNINPLACETUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("TurnInPlaceTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiusData {
    pub radius: f32,
    pub outer_cushion: f32,
    pub inner_cushion: f32,
}

pub const RADIUSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiusData, radius),
            },
            FieldInfoData {
                name: "outerCushion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiusData, outer_cushion),
            },
            FieldInfoData {
                name: "innerCushion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiusData, inner_cushion),
            },
        ],
    }),
    array_type: Some(RADIUSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RadiusData {
    fn type_info() -> &'static TypeInfo {
        RADIUSDATA_TYPE_INFO
    }
}


pub const RADIUSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RadiusData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BlockageMode {
    #[default]
    BLOCKED_IF_ANY_MATCH = 0,
    BLOCKED_IF_ALL_MATCH = 1,
}

pub const BLOCKAGEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockageMode",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(BLOCKAGEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BlockageMode {
    fn type_info() -> &'static TypeInfo {
        BLOCKAGEMODE_TYPE_INFO
    }
}


pub const BLOCKAGEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockageMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("BlockageMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OrientMode {
    #[default]
    ORIENT_IN_TRAVEL_DIR = 0,
    ORIENT_STRICTLY_IN_TRAVEL_DIR = 1,
    ORIENT_TARGET = 2,
    ORIENT_IN_DIR = 3,
}

pub const ORIENTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrientMode",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(ORIENTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OrientMode {
    fn type_info() -> &'static TypeInfo {
        ORIENTMODE_TYPE_INFO
    }
}


pub const ORIENTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrientMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("OrientMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FollowMoverSpec {
    pub formation: FollowFormation,
    pub follow_distance: f32,
    pub arc_spread: f32,
}

pub const FOLLOWMOVERSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowMoverSpec",
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "formation",
                flags: MemberInfoFlags::new(0),
                field_type: FOLLOWFORMATION_TYPE_INFO,
                rust_offset: offset_of!(FollowMoverSpec, formation),
            },
            FieldInfoData {
                name: "followDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowMoverSpec, follow_distance),
            },
            FieldInfoData {
                name: "arcSpread",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FollowMoverSpec, arc_spread),
            },
        ],
    }),
    array_type: Some(FOLLOWMOVERSPEC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FollowMoverSpec {
    fn type_info() -> &'static TypeInfo {
        FOLLOWMOVERSPEC_TYPE_INFO
    }
}


pub const FOLLOWMOVERSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowMoverSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowMoverSpec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FollowFormation {
    #[default]
    FORMATION_CIRCLE = 0,
    FORMATION_BLOB = 1,
}

pub const FOLLOWFORMATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowFormation",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(FOLLOWFORMATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FollowFormation {
    fn type_info() -> &'static TypeInfo {
        FOLLOWFORMATION_TYPE_INFO
    }
}


pub const FOLLOWFORMATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowFormation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowFormation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StopSpec {
    pub stop_immediately: bool,
}

pub const STOPSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSpec",
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "stopImmediately",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StopSpec, stop_immediately),
            },
        ],
    }),
    array_type: Some(STOPSPEC_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StopSpec {
    fn type_info() -> &'static TypeInfo {
        STOPSPEC_TYPE_INFO
    }
}


pub const STOPSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("StopSpec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GotoPosSpec {
    pub allowed_to_stop_dist: f32,
    pub desired_stop_dist: f32,
    pub stop_at_goal: bool,
    pub push_through_crowd_at_goal: bool,
    pub orient_at_goal_enable: bool,
    pub orient_at_goal_dir: super::core::Vec3,
    pub try_flank: bool,
}

pub const GOTOPOSSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GotoPosSpec",
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "allowedToStopDist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GotoPosSpec, allowed_to_stop_dist),
            },
            FieldInfoData {
                name: "desiredStopDist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GotoPosSpec, desired_stop_dist),
            },
            FieldInfoData {
                name: "stopAtGoal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GotoPosSpec, stop_at_goal),
            },
            FieldInfoData {
                name: "pushThroughCrowdAtGoal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GotoPosSpec, push_through_crowd_at_goal),
            },
            FieldInfoData {
                name: "orientAtGoalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GotoPosSpec, orient_at_goal_enable),
            },
            FieldInfoData {
                name: "orientAtGoalDir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(GotoPosSpec, orient_at_goal_dir),
            },
            FieldInfoData {
                name: "tryFlank",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GotoPosSpec, try_flank),
            },
        ],
    }),
    array_type: Some(GOTOPOSSPEC_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GotoPosSpec {
    fn type_info() -> &'static TypeInfo {
        GOTOPOSSPEC_TYPE_INFO
    }
}


pub const GOTOPOSSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GotoPosSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GotoPosSpec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathSpec {
}

pub const PATHSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathSpec",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PATHSPEC_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathSpec {
    fn type_info() -> &'static TypeInfo {
        PATHSPEC_TYPE_INFO
    }
}


pub const PATHSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathSpec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingRuntimeResource {
}

pub const PATHFINDINGRUNTIMERESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResource",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PATHFINDINGRUNTIMERESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathfindingRuntimeResource {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGRUNTIMERESOURCE_TYPE_INFO
    }
}


pub const PATHFINDINGRUNTIMERESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathfindingRuntimeResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerObstacleControllerEntity {
}

pub const SERVEROBSTACLECONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObstacleControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBSTACLECONTROLLERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVEROBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerObstacleControllerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVEROBSTACLECONTROLLERENTITY_TYPE_INFO
    }
}


pub const SERVEROBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObstacleControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ServerObstacleControllerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerNavPowerSystemEntity {
}

pub const SERVERNAVPOWERSYSTEMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerSystemEntity",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERNAVPOWERSYSTEMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerNavPowerSystemEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERNAVPOWERSYSTEMENTITY_TYPE_INFO
    }
}


pub const SERVERNAVPOWERSYSTEMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerSystemEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ServerNavPowerSystemEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObstacleControllerEntity {
}

pub const OBSTACLECONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObstacleControllerEntity {
    fn type_info() -> &'static TypeInfo {
        OBSTACLECONTROLLERENTITY_TYPE_INFO
    }
}


pub const OBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleControllerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NavPowerPathSpec {
}

pub const NAVPOWERPATHSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavPowerPathSpec",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PATHSPEC_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NAVPOWERPATHSPEC_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NavPowerPathSpec {
    fn type_info() -> &'static TypeInfo {
        NAVPOWERPATHSPEC_TYPE_INFO
    }
}


pub const NAVPOWERPATHSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavPowerPathSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("NavPowerPathSpec-Array"),
    array_type: None,
    alignment: 8,
};


