use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_pathfinding_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(VEHICLEWAYPOINTDATA_TYPE_INFO);
    registry.register_type(VEHICLEWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WAYPOINTSSHAPEDATA_TYPE_INFO);
    registry.register_type(WAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(WAYPOINTDATA_TYPE_INFO);
    registry.register_type(WAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ROUTETYPE_TYPE_INFO);
    registry.register_type(ROUTETYPE_ARRAY_TYPE_INFO);
    registry.register_type(WAYPOINTSSNAPPINGSETTINGS_TYPE_INFO);
    registry.register_type(WAYPOINTSSNAPPINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWWAYPOINTSENTITYBASEDATA_TYPE_INFO);
    registry.register_type(FOLLOWWAYPOINTSENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHFOLLOWINGCOMPONENTBASEDATA_TYPE_INFO);
    registry.register_type(PATHFOLLOWINGCOMPONENTBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGCHOICE_TYPE_INFO);
    registry.register_type(PATHFINDINGCHOICE_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGSTREAMENTITYBASEDATA_TYPE_INFO);
    registry.register_type(PATHFINDINGSTREAMENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGDEBUGSETTINGS_TYPE_INFO);
    registry.register_type(PATHFINDINGDEBUGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGREPLAYMODE_TYPE_INFO);
    registry.register_type(PATHFINDINGREPLAYMODE_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGTYPEASSET_TYPE_INFO);
    registry.register_type(PATHFINDINGTYPEASSET_ARRAY_TYPE_INFO);
    registry.register_type(LINKDAT_TYPE_INFO);
    registry.register_type(LINKDAT_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMPATHLINKDATA_TYPE_INFO);
    registry.register_type(CUSTOMPATHLINKDATA_ARRAY_TYPE_INFO);
    registry.register_type(NAVLINKTYPE_TYPE_INFO);
    registry.register_type(NAVLINKTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LINKFLOWTUNE_TYPE_INFO);
    registry.register_type(LINKFLOWTUNE_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCEASSETRESULT_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCEASSETRESULT_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCEASSET_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCEASSET_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGBLOBASSET_TYPE_INFO);
    registry.register_type(PATHFINDINGBLOBASSET_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGBLOB_TYPE_INFO);
    registry.register_type(PATHFINDINGBLOB_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGSYSTEMENTITYDATA_TYPE_INFO);
    registry.register_type(PATHFINDINGSYSTEMENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct VehicleWaypointData {
    pub speed: f32,
    pub speed_override_moving_towards: f32,
    pub speed_limit_on_reached: f32,
    pub speed_limit_moving_towards: f32,
    pub use_heading: bool,
    pub heading: f32,
    pub stop_here: bool,
    pub wait_here: f32,
    pub stopping_deceleration: f32,
    pub min_slowdown_speed: f32,
    pub stop_here_radius: f32,
}

pub const VEHICLEWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WAYPOINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, speed),
            },
            FieldInfoData {
                name: "SpeedOverrideMovingTowards",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, speed_override_moving_towards),
            },
            FieldInfoData {
                name: "SpeedLimitOnReached",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, speed_limit_on_reached),
            },
            FieldInfoData {
                name: "SpeedLimitMovingTowards",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, speed_limit_moving_towards),
            },
            FieldInfoData {
                name: "UseHeading",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, use_heading),
            },
            FieldInfoData {
                name: "Heading",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, heading),
            },
            FieldInfoData {
                name: "StopHere",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, stop_here),
            },
            FieldInfoData {
                name: "WaitHere",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, wait_here),
            },
            FieldInfoData {
                name: "StoppingDeceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, stopping_deceleration),
            },
            FieldInfoData {
                name: "MinSlowdownSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, min_slowdown_speed),
            },
            FieldInfoData {
                name: "StopHereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VehicleWaypointData, stop_here_radius),
            },
        ],
    }),
    array_type: Some(VEHICLEWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehicleWaypointData {
    fn type_info() -> &'static TypeInfo {
        VEHICLEWAYPOINTDATA_TYPE_INFO
    }
}


pub const VEHICLEWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("VehicleWaypointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WaypointsShapeData {
    pub waypoints: Vec<WaypointData>,
}

pub const WAYPOINTSSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsShapeData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(VECTORSHAPEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Waypoints",
                flags: MemberInfoFlags::new(144),
                field_type: WAYPOINTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WaypointsShapeData, waypoints),
            },
        ],
    }),
    array_type: Some(WAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaypointsShapeData {
    fn type_info() -> &'static TypeInfo {
        WAYPOINTSSHAPEDATA_TYPE_INFO
    }
}


pub const WAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsShapeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("WaypointsShapeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WaypointData {
    pub use_clients_position: bool,
    pub schematics_name_hash: i32,
    pub waypoint_id: u32,
}

pub const WAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UseClientsPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WaypointData, use_clients_position),
            },
            FieldInfoData {
                name: "SchematicsNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WaypointData, schematics_name_hash),
            },
            FieldInfoData {
                name: "WaypointId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WaypointData, waypoint_id),
            },
        ],
    }),
    array_type: Some(WAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaypointData {
    fn type_info() -> &'static TypeInfo {
        WAYPOINTDATA_TYPE_INFO
    }
}


pub const WAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("WaypointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RouteType {
    #[default]
    RouteStop = 0,
    RouteCircular = 1,
    RouteBackAndForth = 2,
}

pub const ROUTETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RouteType",
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(ROUTETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RouteType {
    fn type_info() -> &'static TypeInfo {
        ROUTETYPE_TYPE_INFO
    }
}


pub const ROUTETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RouteType-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("RouteType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WaypointsSnappingSettings {
    #[default]
    UseShapeSettings = 0,
    SnapToTerrain = 1,
    NoSnap = 2,
}

pub const WAYPOINTSSNAPPINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsSnappingSettings",
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(WAYPOINTSSNAPPINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaypointsSnappingSettings {
    fn type_info() -> &'static TypeInfo {
        WAYPOINTSSNAPPINGSETTINGS_TYPE_INFO
    }
}


pub const WAYPOINTSSNAPPINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsSnappingSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("WaypointsSnappingSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FollowWaypointsEntityBaseData {
    pub type_of_route: RouteType,
    pub use_path_finding: bool,
    pub start_at_geometrically_closest_waypoint: bool,
}

pub const FOLLOWWAYPOINTSENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowWaypointsEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TypeOfRoute",
                flags: MemberInfoFlags::new(0),
                field_type: ROUTETYPE_TYPE_INFO,
                rust_offset: offset_of!(FollowWaypointsEntityBaseData, type_of_route),
            },
            FieldInfoData {
                name: "UsePathFinding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FollowWaypointsEntityBaseData, use_path_finding),
            },
            FieldInfoData {
                name: "StartAtGeometricallyClosestWaypoint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FollowWaypointsEntityBaseData, start_at_geometrically_closest_waypoint),
            },
        ],
    }),
    array_type: Some(FOLLOWWAYPOINTSENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FollowWaypointsEntityBaseData {
    fn type_info() -> &'static TypeInfo {
        FOLLOWWAYPOINTSENTITYBASEDATA_TYPE_INFO
    }
}


pub const FOLLOWWAYPOINTSENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowWaypointsEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("FollowWaypointsEntityBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PathFollowingComponentBaseData {
    pub update_path_at_distance_percent: f32,
    pub preferred_pathfinding_index: u32,
    pub alternate_pathfinding_indices: Vec<u32>,
    pub movement_corridor_radius: f32,
}

pub const PATHFOLLOWINGCOMPONENTBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathFollowingComponentBaseData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UpdatePathAtDistancePercent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathFollowingComponentBaseData, update_path_at_distance_percent),
            },
            FieldInfoData {
                name: "PreferredPathfindingIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PathFollowingComponentBaseData, preferred_pathfinding_index),
            },
            FieldInfoData {
                name: "AlternatePathfindingIndices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PathFollowingComponentBaseData, alternate_pathfinding_indices),
            },
            FieldInfoData {
                name: "MovementCorridorRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathFollowingComponentBaseData, movement_corridor_radius),
            },
        ],
    }),
    array_type: Some(PATHFOLLOWINGCOMPONENTBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathFollowingComponentBaseData {
    fn type_info() -> &'static TypeInfo {
        PATHFOLLOWINGCOMPONENTBASEDATA_TYPE_INFO
    }
}


pub const PATHFOLLOWINGCOMPONENTBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathFollowingComponentBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathFollowingComponentBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PathfindingChoice {
    #[default]
    PathfindingChoice_Off = 0,
    PathfindingChoice_OnlyToStartPoint = 1,
    PathfindingChoice_On = 2,
}

pub const PATHFINDINGCHOICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingChoice",
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(PATHFINDINGCHOICE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PathfindingChoice {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGCHOICE_TYPE_INFO
    }
}


pub const PATHFINDINGCHOICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingChoice-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingChoice-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingStreamEntityBaseData {
    pub pathfinding_blobs: PathfindingBlobAsset,
    pub autoload: bool,
    pub realm: super::core::Realm,
}

pub const PATHFINDINGSTREAMENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingStreamEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PathfindingBlobs",
                flags: MemberInfoFlags::new(0),
                field_type: PATHFINDINGBLOBASSET_TYPE_INFO,
                rust_offset: offset_of!(PathfindingStreamEntityBaseData, pathfinding_blobs),
            },
            FieldInfoData {
                name: "Autoload",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingStreamEntityBaseData, autoload),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PathfindingStreamEntityBaseData, realm),
            },
        ],
    }),
    array_type: Some(PATHFINDINGSTREAMENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathfindingStreamEntityBaseData {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGSTREAMENTITYBASEDATA_TYPE_INFO
    }
}


pub const PATHFINDINGSTREAMENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingStreamEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingStreamEntityBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PathfindingDebugSettings {
    pub draw_nav_mesh: i32,
    pub draw_polygon_outline: bool,
    pub draw_filled_polygons: bool,
    pub draw_connections: bool,
    pub draw_obstacles: bool,
    pub draw_area_normals: bool,
    pub draw_obstacle_flags: bool,
    pub draw_area_penalty_mults: bool,
    pub draw_area_usage_flags: bool,
    pub colorize_area_usage_flags: bool,
    pub draw_link_usage_distances: bool,
    pub draw_link_usage_flags: bool,
    pub draw_original_link_locations: bool,
    pub draw_recent_nav_probes: bool,
    pub draw_recent_create_poly_line_paths: bool,
    pub draw_mover_cylinders: bool,
    pub draw_mover_goals: bool,
    pub draw_mover_goals_reached: bool,
    pub draw_mover_state: bool,
    pub draw_mover_attractions: bool,
    pub draw_repulsors: bool,
    pub draw_client_motion: bool,
    pub draw_cur_path_section: bool,
    pub draw_follower_goals: bool,
    pub draw_distance: f32,
    pub depth_test: bool,
    pub draw_stats: bool,
    pub draw_memory: bool,
    pub draw_timings: bool,
    pub text_start_x: i32,
    pub text_start_y: i32,
    pub text_offset_y: i32,
    pub replay_mode: PathfindingReplayMode,
    pub original_paths: bool,
    pub random_positions: bool,
    pub potential_obstacles: bool,
}

pub const PATHFINDINGDEBUGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingDebugSettings",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DrawNavMesh",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_nav_mesh),
            },
            FieldInfoData {
                name: "DrawPolygonOutline",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_polygon_outline),
            },
            FieldInfoData {
                name: "DrawFilledPolygons",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_filled_polygons),
            },
            FieldInfoData {
                name: "DrawConnections",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_connections),
            },
            FieldInfoData {
                name: "DrawObstacles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_obstacles),
            },
            FieldInfoData {
                name: "DrawAreaNormals",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_area_normals),
            },
            FieldInfoData {
                name: "DrawObstacleFlags",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_obstacle_flags),
            },
            FieldInfoData {
                name: "DrawAreaPenaltyMults",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_area_penalty_mults),
            },
            FieldInfoData {
                name: "DrawAreaUsageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_area_usage_flags),
            },
            FieldInfoData {
                name: "ColorizeAreaUsageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, colorize_area_usage_flags),
            },
            FieldInfoData {
                name: "DrawLinkUsageDistances",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_link_usage_distances),
            },
            FieldInfoData {
                name: "DrawLinkUsageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_link_usage_flags),
            },
            FieldInfoData {
                name: "DrawOriginalLinkLocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_original_link_locations),
            },
            FieldInfoData {
                name: "DrawRecentNavProbes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_recent_nav_probes),
            },
            FieldInfoData {
                name: "DrawRecentCreatePolyLinePaths",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_recent_create_poly_line_paths),
            },
            FieldInfoData {
                name: "DrawMoverCylinders",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_cylinders),
            },
            FieldInfoData {
                name: "DrawMoverGoals",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_goals),
            },
            FieldInfoData {
                name: "DrawMoverGoalsReached",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_goals_reached),
            },
            FieldInfoData {
                name: "DrawMoverState",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_state),
            },
            FieldInfoData {
                name: "DrawMoverAttractions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_attractions),
            },
            FieldInfoData {
                name: "DrawRepulsors",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_repulsors),
            },
            FieldInfoData {
                name: "DrawClientMotion",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_client_motion),
            },
            FieldInfoData {
                name: "DrawCurPathSection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_cur_path_section),
            },
            FieldInfoData {
                name: "DrawFollowerGoals",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_follower_goals),
            },
            FieldInfoData {
                name: "DrawDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_distance),
            },
            FieldInfoData {
                name: "DepthTest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, depth_test),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawMemory",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_memory),
            },
            FieldInfoData {
                name: "DrawTimings",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, draw_timings),
            },
            FieldInfoData {
                name: "TextStartX",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, text_start_x),
            },
            FieldInfoData {
                name: "TextStartY",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, text_start_y),
            },
            FieldInfoData {
                name: "TextOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, text_offset_y),
            },
            FieldInfoData {
                name: "ReplayMode",
                flags: MemberInfoFlags::new(0),
                field_type: PATHFINDINGREPLAYMODE_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, replay_mode),
            },
            FieldInfoData {
                name: "OriginalPaths",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, original_paths),
            },
            FieldInfoData {
                name: "RandomPositions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, random_positions),
            },
            FieldInfoData {
                name: "PotentialObstacles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingDebugSettings, potential_obstacles),
            },
        ],
    }),
    array_type: Some(PATHFINDINGDEBUGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingDebugSettings {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGDEBUGSETTINGS_TYPE_INFO
    }
}


pub const PATHFINDINGDEBUGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingDebugSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingDebugSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PathfindingReplayMode {
    #[default]
    PathfindingReplayMode_Disabled = 0,
    PathfindingReplayMode_Binary = 1,
    PathfindingReplayMode_Text = 2,
}

pub const PATHFINDINGREPLAYMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingReplayMode",
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(PATHFINDINGREPLAYMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PathfindingReplayMode {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGREPLAYMODE_TYPE_INFO
    }
}


pub const PATHFINDINGREPLAYMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingReplayMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingReplayMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingTypeAsset {
    pub index: u32,
}

pub const PATHFINDINGTYPEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingTypeAsset",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingTypeAsset, index),
            },
        ],
    }),
    array_type: Some(PATHFINDINGTYPEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingTypeAsset {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGTYPEASSET_TYPE_INFO
    }
}


pub const PATHFINDINGTYPEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingTypeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingTypeAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinkDat {
    pub layer_mask: u32,
    pub link_type: NavLinkType,
    pub forward_link_usage_flags: u32,
    pub backward_link_usage_flags: u32,
    pub penalty_mult: f32,
    pub max_snap_dist: f32,
    pub may_use_dist: f32,
    pub must_use_dist: f32,
    pub stop_to_use_link: bool,
    pub user_data: CustomPathLinkData,
    pub link_flow_tune: LinkFlowTune,
}

pub const LINKDAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkDat",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, layer_mask),
            },
            FieldInfoData {
                name: "LinkType",
                flags: MemberInfoFlags::new(0),
                field_type: NAVLINKTYPE_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, link_type),
            },
            FieldInfoData {
                name: "ForwardLinkUsageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, forward_link_usage_flags),
            },
            FieldInfoData {
                name: "BackwardLinkUsageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, backward_link_usage_flags),
            },
            FieldInfoData {
                name: "PenaltyMult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, penalty_mult),
            },
            FieldInfoData {
                name: "MaxSnapDist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, max_snap_dist),
            },
            FieldInfoData {
                name: "MayUseDist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, may_use_dist),
            },
            FieldInfoData {
                name: "MustUseDist",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, must_use_dist),
            },
            FieldInfoData {
                name: "StopToUseLink",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, stop_to_use_link),
            },
            FieldInfoData {
                name: "UserData",
                flags: MemberInfoFlags::new(0),
                field_type: CUSTOMPATHLINKDATA_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, user_data),
            },
            FieldInfoData {
                name: "LinkFlowTune",
                flags: MemberInfoFlags::new(0),
                field_type: LINKFLOWTUNE_TYPE_INFO,
                rust_offset: offset_of!(LinkDat, link_flow_tune),
            },
        ],
    }),
    array_type: Some(LINKDAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkDat {
    fn type_info() -> &'static TypeInfo {
        LINKDAT_TYPE_INFO
    }
}


pub const LINKDAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkDat-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("LinkDat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomPathLinkData {
}

pub const CUSTOMPATHLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomPathLinkData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CUSTOMPATHLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomPathLinkData {
    fn type_info() -> &'static TypeInfo {
        CUSTOMPATHLINKDATA_TYPE_INFO
    }
}


pub const CUSTOMPATHLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomPathLinkData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("CustomPathLinkData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum NavLinkType {
    #[default]
    JUMP_LINK = 0,
    CUSTOM_LINK = 1,
}

pub const NAVLINKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavLinkType",
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(NAVLINKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NavLinkType {
    fn type_info() -> &'static TypeInfo {
        NAVLINKTYPE_TYPE_INFO
    }
}


pub const NAVLINKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavLinkType-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("NavLinkType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkFlowTune {
    pub max_simultaneous: u32,
}

pub const LINKFLOWTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkFlowTune",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxSimultaneous",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(LinkFlowTune, max_simultaneous),
            },
        ],
    }),
    array_type: Some(LINKFLOWTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkFlowTune {
    fn type_info() -> &'static TypeInfo {
        LINKFLOWTUNE_TYPE_INFO
    }
}


pub const LINKFLOWTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkFlowTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("LinkFlowTune-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingRuntimeResourceAssetResult {
    pub has_path_data: bool,
}

pub const PATHFINDINGRUNTIMERESOURCEASSETRESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAssetResult",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HasPathData",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PathfindingRuntimeResourceAssetResult, has_path_data),
            },
        ],
    }),
    array_type: Some(PATHFINDINGRUNTIMERESOURCEASSETRESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingRuntimeResourceAssetResult {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGRUNTIMERESOURCEASSETRESULT_TYPE_INFO
    }
}


pub const PATHFINDINGRUNTIMERESOURCEASSETRESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAssetResult-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingRuntimeResourceAssetResult-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingRuntimeResourceAsset {
    pub resource: super::core::ResourceRef,
    pub blob_size: u32,
    pub chunk_sizes: Vec<u32>,
}

pub const PATHFINDINGRUNTIMERESOURCEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAsset",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(PathfindingRuntimeResourceAsset, resource),
            },
            FieldInfoData {
                name: "BlobSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingRuntimeResourceAsset, blob_size),
            },
            FieldInfoData {
                name: "ChunkSizes",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PathfindingRuntimeResourceAsset, chunk_sizes),
            },
        ],
    }),
    array_type: Some(PATHFINDINGRUNTIMERESOURCEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingRuntimeResourceAsset {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGRUNTIMERESOURCEASSET_TYPE_INFO
    }
}


pub const PATHFINDINGRUNTIMERESOURCEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingRuntimeResourceAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingBlobAsset {
    pub blob: PathfindingBlob,
}

pub const PATHFINDINGBLOBASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlobAsset",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Blob",
                flags: MemberInfoFlags::new(0),
                field_type: PATHFINDINGBLOB_TYPE_INFO,
                rust_offset: offset_of!(PathfindingBlobAsset, blob),
            },
        ],
    }),
    array_type: Some(PATHFINDINGBLOBASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingBlobAsset {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGBLOBASSET_TYPE_INFO
    }
}


pub const PATHFINDINGBLOBASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlobAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingBlobAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingBlob {
    pub blob_id: super::core::Guid,
    pub blob_size: u32,
    pub chunk_sizes: Vec<u32>,
}

pub const PATHFINDINGBLOB_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlob",
    flags: MemberInfoFlags::new(73),
    module: "PathfindingShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BlobId",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(PathfindingBlob, blob_id),
            },
            FieldInfoData {
                name: "BlobSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PathfindingBlob, blob_size),
            },
            FieldInfoData {
                name: "ChunkSizes",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PathfindingBlob, chunk_sizes),
            },
        ],
    }),
    array_type: Some(PATHFINDINGBLOB_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingBlob {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGBLOB_TYPE_INFO
    }
}


pub const PATHFINDINGBLOB_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlob-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingBlob-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathfindingSystemEntityData {
    pub pathfinding_types_on_level: Vec<u32>,
    pub realm: super::core::Realm,
    pub resource_asset: PathfindingRuntimeResourceAsset,
}

pub const PATHFINDINGSYSTEMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingSystemEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PathfindingTypesOnLevel",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(PathfindingSystemEntityData, pathfinding_types_on_level),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PathfindingSystemEntityData, realm),
            },
            FieldInfoData {
                name: "ResourceAsset",
                flags: MemberInfoFlags::new(0),
                field_type: PATHFINDINGRUNTIMERESOURCEASSET_TYPE_INFO,
                rust_offset: offset_of!(PathfindingSystemEntityData, resource_asset),
            },
        ],
    }),
    array_type: Some(PATHFINDINGSYSTEMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingSystemEntityData {
    fn type_info() -> &'static TypeInfo {
        PATHFINDINGSYSTEMENTITYDATA_TYPE_INFO
    }
}


pub const PATHFINDINGSYSTEMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingSystemEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingSystemEntityData-Array"),
    array_type: None,
    alignment: 8,
};


