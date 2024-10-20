use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_rail_ride_heli_types(registry: &mut TypeRegistry) {
    registry.register_type(RAILRIDEHELIWAYPOINTDATA_TYPE_INFO);
    registry.register_type(RAILRIDEHELIWAYPOINTDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAILRIDEHELISEGMENTDATA_TYPE_INFO);
    registry.register_type(RAILRIDEHELISEGMENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAILRIDENODEPOINTTODATA_TYPE_INFO);
    registry.register_type(RAILRIDENODEPOINTTODATA_ARRAY_TYPE_INFO);
    registry.register_type(RAILRIDEHELIPOINTTOSIDE_TYPE_INFO);
    registry.register_type(RAILRIDEHELIPOINTTOSIDE_ARRAY_TYPE_INFO);
    registry.register_type(RAILRIDEHELIDATA_TYPE_INFO);
    registry.register_type(RAILRIDEHELIDATA_ARRAY_TYPE_INFO);
    registry.register_type(RAILRIDEHELICONTROLTYPE_TYPE_INFO);
    registry.register_type(RAILRIDEHELICONTROLTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RAILRIDEHELICLIENTACTIVATEDMESSAGE_TYPE_INFO);
    registry.register_type(SERVERRAILRIDEHELIENTITY_TYPE_INFO);
    registry.register_type(SERVERRAILRIDEHELIENTITY_ARRAY_TYPE_INFO);
    registry.register_type(RAILRIDEHELISEGMENTENTITY_TYPE_INFO);
    registry.register_type(RAILRIDEHELISEGMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTRAILRIDEHELIENTITY_TYPE_INFO);
    registry.register_type(CLIENTRAILRIDEHELIENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct RailRideHeliWaypointData {
    pub forward_direction: super::core::Vec3,
    pub wind_noise: f32,
    pub limit_wind_to_horizontal: bool,
    pub speed: f32,
    pub use_velocity_direction: bool,
    pub point_forward: bool,
    pub point_to_distance: f32,
    pub wing_mode: bool,
    pub curvature: f32,
    pub constraint: f32,
    pub force_pitch: f32,
    pub banking_scale: f32,
    pub point_to_index: i32,
}

pub const RAILRIDEHELIWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliWaypointData",
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WAYPOINTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ForwardDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, forward_direction),
            },
            FieldInfoData {
                name: "WindNoise",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, wind_noise),
            },
            FieldInfoData {
                name: "LimitWindToHorizontal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, limit_wind_to_horizontal),
            },
            FieldInfoData {
                name: "Speed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, speed),
            },
            FieldInfoData {
                name: "UseVelocityDirection",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, use_velocity_direction),
            },
            FieldInfoData {
                name: "PointForward",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, point_forward),
            },
            FieldInfoData {
                name: "PointToDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, point_to_distance),
            },
            FieldInfoData {
                name: "WingMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, wing_mode),
            },
            FieldInfoData {
                name: "Curvature",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, curvature),
            },
            FieldInfoData {
                name: "Constraint",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, constraint),
            },
            FieldInfoData {
                name: "ForcePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, force_pitch),
            },
            FieldInfoData {
                name: "BankingScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, banking_scale),
            },
            FieldInfoData {
                name: "PointToIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliWaypointData, point_to_index),
            },
        ],
    }),
    array_type: Some(RAILRIDEHELIWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideHeliWaypointData {
    fn type_info() -> &'static TypeInfo {
        RAILRIDEHELIWAYPOINTDATA_TYPE_INFO
    }
}


pub const RAILRIDEHELIWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliWaypointData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliWaypointData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RailRideHeliSegmentData {
    pub editor_color: super::core::Vec4,
    pub looping: bool,
    pub start_index: i32,
    pub end_index: i32,
    pub teleport_on_activate: bool,
    pub force_speed_on_activate: bool,
    pub force_direction_on_activate: bool,
    pub teleport_on_loop: bool,
    pub node_index1: i32,
    pub node_index2: i32,
    pub node_index3: i32,
    pub node_on_reached: Vec<i32>,
    pub node_on_reached_event_hashes: Vec<i32>,
    pub node_on_reached_property_hashes: Vec<i32>,
    pub external_time: f32,
    pub waypoints_speed_scale: f32,
    pub update_waypoints_speed_scale_every_frame: bool,
}

pub const RAILRIDEHELISEGMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentData",
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EditorColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, editor_color),
            },
            FieldInfoData {
                name: "Looping",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, looping),
            },
            FieldInfoData {
                name: "StartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, start_index),
            },
            FieldInfoData {
                name: "EndIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, end_index),
            },
            FieldInfoData {
                name: "TeleportOnActivate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, teleport_on_activate),
            },
            FieldInfoData {
                name: "ForceSpeedOnActivate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, force_speed_on_activate),
            },
            FieldInfoData {
                name: "ForceDirectionOnActivate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, force_direction_on_activate),
            },
            FieldInfoData {
                name: "TeleportOnLoop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, teleport_on_loop),
            },
            FieldInfoData {
                name: "NodeIndex1",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, node_index1),
            },
            FieldInfoData {
                name: "NodeIndex2",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, node_index2),
            },
            FieldInfoData {
                name: "NodeIndex3",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, node_index3),
            },
            FieldInfoData {
                name: "NodeOnReached",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, node_on_reached),
            },
            FieldInfoData {
                name: "NodeOnReachedEventHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, node_on_reached_event_hashes),
            },
            FieldInfoData {
                name: "NodeOnReachedPropertyHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, node_on_reached_property_hashes),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, external_time),
            },
            FieldInfoData {
                name: "WaypointsSpeedScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, waypoints_speed_scale),
            },
            FieldInfoData {
                name: "UpdateWaypointsSpeedScaleEveryFrame",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliSegmentData, update_waypoints_speed_scale_every_frame),
            },
        ],
    }),
    array_type: Some(RAILRIDEHELISEGMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideHeliSegmentData {
    fn type_info() -> &'static TypeInfo {
        RAILRIDEHELISEGMENTDATA_TYPE_INFO
    }
}


pub const RAILRIDEHELISEGMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliSegmentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RailRideNodePointToData {
    pub side: RailRideHeliPointToSide,
}

pub const RAILRIDENODEPOINTTODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideNodePointToData",
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Side",
                flags: MemberInfoFlags::new(0),
                field_type: RAILRIDEHELIPOINTTOSIDE_TYPE_INFO,
                rust_offset: offset_of!(RailRideNodePointToData, side),
            },
        ],
    }),
    array_type: Some(RAILRIDENODEPOINTTODATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideNodePointToData {
    fn type_info() -> &'static TypeInfo {
        RAILRIDENODEPOINTTODATA_TYPE_INFO
    }
}


pub const RAILRIDENODEPOINTTODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideNodePointToData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideNodePointToData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RailRideHeliPointToSide {
    #[default]
    Front = 0,
    Left = 1,
    Right = 2,
}

pub const RAILRIDEHELIPOINTTOSIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliPointToSide",
    flags: MemberInfoFlags::new(49429),
    module: "RailRideHeli",
    data: TypeInfoData::Enum,
    array_type: Some(RAILRIDEHELIPOINTTOSIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RailRideHeliPointToSide {
    fn type_info() -> &'static TypeInfo {
        RAILRIDEHELIPOINTTOSIDE_TYPE_INFO
    }
}


pub const RAILRIDEHELIPOINTTOSIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliPointToSide-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliPointToSide-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RailRideHeliData {
    pub control_type: RailRideHeliControlType,
    pub network_control_transform: bool,
    pub wait_on_client_activation: bool,
    pub client_activation_blend_time: f32,
    pub angular_approach_slow: f32,
    pub angular_approach_fast: f32,
    pub tilt_max: f32,
    pub tilt_accel_mult: f32,
    pub tilt_speed_mult: f32,
    pub veer_duration_min: f32,
    pub veer_duration_max: f32,
    pub veer_position_offset_min: f32,
    pub veer_position_offset_max: f32,
    pub veer_heading_offset_min: f32,
    pub veer_heading_offset_max: f32,
    pub simulation_steps_per_frame: i32,
    pub sound_accel_to_throttle_mult: f32,
    pub sound_speed_to_throttle_mult: f32,
    pub speed_multiplier: f32,
    pub wing_mode_on_activate: bool,
    pub root_transform: super::core::LinearTransform,
    pub reference_transform: super::core::LinearTransform,
    pub blend_transform: super::core::LinearTransform,
    pub blend: f32,
    pub point_to_override: super::core::Vec3,
    pub enable_point_to_override: bool,
    pub use_segment_external_time: bool,
    pub braking: bool,
}

pub const RAILRIDEHELIDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliData",
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ControlType",
                flags: MemberInfoFlags::new(0),
                field_type: RAILRIDEHELICONTROLTYPE_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, control_type),
            },
            FieldInfoData {
                name: "NetworkControlTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, network_control_transform),
            },
            FieldInfoData {
                name: "WaitOnClientActivation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, wait_on_client_activation),
            },
            FieldInfoData {
                name: "ClientActivationBlendTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, client_activation_blend_time),
            },
            FieldInfoData {
                name: "AngularApproachSlow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, angular_approach_slow),
            },
            FieldInfoData {
                name: "AngularApproachFast",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, angular_approach_fast),
            },
            FieldInfoData {
                name: "TiltMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, tilt_max),
            },
            FieldInfoData {
                name: "TiltAccelMult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, tilt_accel_mult),
            },
            FieldInfoData {
                name: "TiltSpeedMult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, tilt_speed_mult),
            },
            FieldInfoData {
                name: "VeerDurationMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, veer_duration_min),
            },
            FieldInfoData {
                name: "VeerDurationMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, veer_duration_max),
            },
            FieldInfoData {
                name: "VeerPositionOffsetMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, veer_position_offset_min),
            },
            FieldInfoData {
                name: "VeerPositionOffsetMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, veer_position_offset_max),
            },
            FieldInfoData {
                name: "VeerHeadingOffsetMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, veer_heading_offset_min),
            },
            FieldInfoData {
                name: "VeerHeadingOffsetMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, veer_heading_offset_max),
            },
            FieldInfoData {
                name: "SimulationStepsPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, simulation_steps_per_frame),
            },
            FieldInfoData {
                name: "SoundAccelToThrottleMult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, sound_accel_to_throttle_mult),
            },
            FieldInfoData {
                name: "SoundSpeedToThrottleMult",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, sound_speed_to_throttle_mult),
            },
            FieldInfoData {
                name: "SpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, speed_multiplier),
            },
            FieldInfoData {
                name: "WingModeOnActivate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, wing_mode_on_activate),
            },
            FieldInfoData {
                name: "RootTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, root_transform),
            },
            FieldInfoData {
                name: "ReferenceTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, reference_transform),
            },
            FieldInfoData {
                name: "BlendTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, blend_transform),
            },
            FieldInfoData {
                name: "Blend",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, blend),
            },
            FieldInfoData {
                name: "PointToOverride",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, point_to_override),
            },
            FieldInfoData {
                name: "EnablePointToOverride",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, enable_point_to_override),
            },
            FieldInfoData {
                name: "UseSegmentExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, use_segment_external_time),
            },
            FieldInfoData {
                name: "Braking",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RailRideHeliData, braking),
            },
        ],
    }),
    array_type: Some(RAILRIDEHELIDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideHeliData {
    fn type_info() -> &'static TypeInfo {
        RAILRIDEHELIDATA_TYPE_INFO
    }
}


pub const RAILRIDEHELIDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliData-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RailRideHeliControlType {
    #[default]
    RailRideHeliControl_KeyframedVehicle = 0,
    RailRideHeliControl_RawTransform = 1,
}

pub const RAILRIDEHELICONTROLTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliControlType",
    flags: MemberInfoFlags::new(49429),
    module: "RailRideHeli",
    data: TypeInfoData::Enum,
    array_type: Some(RAILRIDEHELICONTROLTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RailRideHeliControlType {
    fn type_info() -> &'static TypeInfo {
        RAILRIDEHELICONTROLTYPE_TYPE_INFO
    }
}


pub const RAILRIDEHELICONTROLTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliControlType-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliControlType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RailRideHeliClientActivatedMessage {
}

pub const RAILRIDEHELICLIENTACTIVATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliClientActivatedMessage",
    flags: MemberInfoFlags::new(73),
    module: "RailRideHeli",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for RailRideHeliClientActivatedMessage {
    fn type_info() -> &'static TypeInfo {
        RAILRIDEHELICLIENTACTIVATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerRailRideHeliEntity {
}

pub const SERVERRAILRIDEHELIENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRailRideHeliEntity",
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERRAILRIDEHELIENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRailRideHeliEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERRAILRIDEHELIENTITY_TYPE_INFO
    }
}


pub const SERVERRAILRIDEHELIENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRailRideHeliEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("ServerRailRideHeliEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RailRideHeliSegmentEntity {
}

pub const RAILRIDEHELISEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentEntity",
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RAILRIDEHELISEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RailRideHeliSegmentEntity {
    fn type_info() -> &'static TypeInfo {
        RAILRIDEHELISEGMENTENTITY_TYPE_INFO
    }
}


pub const RAILRIDEHELISEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliSegmentEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientRailRideHeliEntity {
}

pub const CLIENTRAILRIDEHELIENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRailRideHeliEntity",
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTRAILRIDEHELIENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientRailRideHeliEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTRAILRIDEHELIENTITY_TYPE_INFO
    }
}


pub const CLIENTRAILRIDEHELIENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRailRideHeliEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("ClientRailRideHeliEntity-Array"),
    array_type: None,
    alignment: 8,
};


