use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct RailRideHeliWaypointData {
    pub _glacier_base: super::pathfinding_shared::WaypointData,
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

pub trait RailRideHeliWaypointDataTrait: super::pathfinding_shared::WaypointDataTrait {
    fn forward_direction(&self) -> &super::core::Vec3;
    fn forward_direction_mut(&mut self) -> &mut super::core::Vec3;
    fn wind_noise(&self) -> &f32;
    fn wind_noise_mut(&mut self) -> &mut f32;
    fn limit_wind_to_horizontal(&self) -> &bool;
    fn limit_wind_to_horizontal_mut(&mut self) -> &mut bool;
    fn speed(&self) -> &f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn use_velocity_direction(&self) -> &bool;
    fn use_velocity_direction_mut(&mut self) -> &mut bool;
    fn point_forward(&self) -> &bool;
    fn point_forward_mut(&mut self) -> &mut bool;
    fn point_to_distance(&self) -> &f32;
    fn point_to_distance_mut(&mut self) -> &mut f32;
    fn wing_mode(&self) -> &bool;
    fn wing_mode_mut(&mut self) -> &mut bool;
    fn curvature(&self) -> &f32;
    fn curvature_mut(&mut self) -> &mut f32;
    fn constraint(&self) -> &f32;
    fn constraint_mut(&mut self) -> &mut f32;
    fn force_pitch(&self) -> &f32;
    fn force_pitch_mut(&mut self) -> &mut f32;
    fn banking_scale(&self) -> &f32;
    fn banking_scale_mut(&mut self) -> &mut f32;
    fn point_to_index(&self) -> &i32;
    fn point_to_index_mut(&mut self) -> &mut i32;
}

impl RailRideHeliWaypointDataTrait for RailRideHeliWaypointData {
    fn forward_direction(&self) -> &super::core::Vec3 {
        &self.forward_direction
    }
    fn forward_direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.forward_direction
    }
    fn wind_noise(&self) -> &f32 {
        &self.wind_noise
    }
    fn wind_noise_mut(&mut self) -> &mut f32 {
        &mut self.wind_noise
    }
    fn limit_wind_to_horizontal(&self) -> &bool {
        &self.limit_wind_to_horizontal
    }
    fn limit_wind_to_horizontal_mut(&mut self) -> &mut bool {
        &mut self.limit_wind_to_horizontal
    }
    fn speed(&self) -> &f32 {
        &self.speed
    }
    fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }
    fn use_velocity_direction(&self) -> &bool {
        &self.use_velocity_direction
    }
    fn use_velocity_direction_mut(&mut self) -> &mut bool {
        &mut self.use_velocity_direction
    }
    fn point_forward(&self) -> &bool {
        &self.point_forward
    }
    fn point_forward_mut(&mut self) -> &mut bool {
        &mut self.point_forward
    }
    fn point_to_distance(&self) -> &f32 {
        &self.point_to_distance
    }
    fn point_to_distance_mut(&mut self) -> &mut f32 {
        &mut self.point_to_distance
    }
    fn wing_mode(&self) -> &bool {
        &self.wing_mode
    }
    fn wing_mode_mut(&mut self) -> &mut bool {
        &mut self.wing_mode
    }
    fn curvature(&self) -> &f32 {
        &self.curvature
    }
    fn curvature_mut(&mut self) -> &mut f32 {
        &mut self.curvature
    }
    fn constraint(&self) -> &f32 {
        &self.constraint
    }
    fn constraint_mut(&mut self) -> &mut f32 {
        &mut self.constraint
    }
    fn force_pitch(&self) -> &f32 {
        &self.force_pitch
    }
    fn force_pitch_mut(&mut self) -> &mut f32 {
        &mut self.force_pitch
    }
    fn banking_scale(&self) -> &f32 {
        &self.banking_scale
    }
    fn banking_scale_mut(&mut self) -> &mut f32 {
        &mut self.banking_scale
    }
    fn point_to_index(&self) -> &i32 {
        &self.point_to_index
    }
    fn point_to_index_mut(&mut self) -> &mut i32 {
        &mut self.point_to_index
    }
}

impl super::pathfinding_shared::WaypointDataTrait for RailRideHeliWaypointData {
    fn use_clients_position(&self) -> &bool {
        self._glacier_base.use_clients_position()
    }
    fn use_clients_position_mut(&mut self) -> &mut bool {
        self._glacier_base.use_clients_position_mut()
    }
    fn schematics_name_hash(&self) -> &i32 {
        self._glacier_base.schematics_name_hash()
    }
    fn schematics_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.schematics_name_hash_mut()
    }
    fn waypoint_id(&self) -> &u32 {
        self._glacier_base.waypoint_id()
    }
    fn waypoint_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.waypoint_id_mut()
    }
}

impl super::core::DataContainerTrait for RailRideHeliWaypointData {
}

pub static RAILRIDEHELIWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliWaypointData",
    name_hash: 3014952754,
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::pathfinding_shared::WAYPOINTDATA_TYPE_INFO),
        super_class_offset: offset_of!(RailRideHeliWaypointData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RailRideHeliWaypointData as Default>::default())),
            create_boxed: || Box::new(<RailRideHeliWaypointData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ForwardDirection",
                name_hash: 4200871771,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RailRideHeliWaypointData, forward_direction),
            },
            FieldInfoData {
                name: "WindNoise",
                name_hash: 3215343151,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliWaypointData, wind_noise),
            },
            FieldInfoData {
                name: "LimitWindToHorizontal",
                name_hash: 2899700673,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliWaypointData, limit_wind_to_horizontal),
            },
            FieldInfoData {
                name: "Speed",
                name_hash: 230887042,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliWaypointData, speed),
            },
            FieldInfoData {
                name: "UseVelocityDirection",
                name_hash: 1075717460,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliWaypointData, use_velocity_direction),
            },
            FieldInfoData {
                name: "PointForward",
                name_hash: 1482929074,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliWaypointData, point_forward),
            },
            FieldInfoData {
                name: "PointToDistance",
                name_hash: 2371114353,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliWaypointData, point_to_distance),
            },
            FieldInfoData {
                name: "WingMode",
                name_hash: 354218001,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliWaypointData, wing_mode),
            },
            FieldInfoData {
                name: "Curvature",
                name_hash: 2405441856,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliWaypointData, curvature),
            },
            FieldInfoData {
                name: "Constraint",
                name_hash: 1858096416,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliWaypointData, constraint),
            },
            FieldInfoData {
                name: "ForcePitch",
                name_hash: 1030831486,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliWaypointData, force_pitch),
            },
            FieldInfoData {
                name: "BankingScale",
                name_hash: 3470475451,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliWaypointData, banking_scale),
            },
            FieldInfoData {
                name: "PointToIndex",
                name_hash: 3277302860,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RailRideHeliWaypointData, point_to_index),
            },
        ],
    }),
    array_type: Some(RAILRIDEHELIWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideHeliWaypointData {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDEHELIWAYPOINTDATA_TYPE_INFO
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


pub static RAILRIDEHELIWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliWaypointData-Array",
    name_hash: 2177305990,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliWaypointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RailRideHeliSegmentData {
    pub _glacier_base: super::entity::EntityData,
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

pub trait RailRideHeliSegmentDataTrait: super::entity::EntityDataTrait {
    fn editor_color(&self) -> &super::core::Vec4;
    fn editor_color_mut(&mut self) -> &mut super::core::Vec4;
    fn looping(&self) -> &bool;
    fn looping_mut(&mut self) -> &mut bool;
    fn start_index(&self) -> &i32;
    fn start_index_mut(&mut self) -> &mut i32;
    fn end_index(&self) -> &i32;
    fn end_index_mut(&mut self) -> &mut i32;
    fn teleport_on_activate(&self) -> &bool;
    fn teleport_on_activate_mut(&mut self) -> &mut bool;
    fn force_speed_on_activate(&self) -> &bool;
    fn force_speed_on_activate_mut(&mut self) -> &mut bool;
    fn force_direction_on_activate(&self) -> &bool;
    fn force_direction_on_activate_mut(&mut self) -> &mut bool;
    fn teleport_on_loop(&self) -> &bool;
    fn teleport_on_loop_mut(&mut self) -> &mut bool;
    fn node_index1(&self) -> &i32;
    fn node_index1_mut(&mut self) -> &mut i32;
    fn node_index2(&self) -> &i32;
    fn node_index2_mut(&mut self) -> &mut i32;
    fn node_index3(&self) -> &i32;
    fn node_index3_mut(&mut self) -> &mut i32;
    fn node_on_reached(&self) -> &Vec<i32>;
    fn node_on_reached_mut(&mut self) -> &mut Vec<i32>;
    fn node_on_reached_event_hashes(&self) -> &Vec<i32>;
    fn node_on_reached_event_hashes_mut(&mut self) -> &mut Vec<i32>;
    fn node_on_reached_property_hashes(&self) -> &Vec<i32>;
    fn node_on_reached_property_hashes_mut(&mut self) -> &mut Vec<i32>;
    fn external_time(&self) -> &f32;
    fn external_time_mut(&mut self) -> &mut f32;
    fn waypoints_speed_scale(&self) -> &f32;
    fn waypoints_speed_scale_mut(&mut self) -> &mut f32;
    fn update_waypoints_speed_scale_every_frame(&self) -> &bool;
    fn update_waypoints_speed_scale_every_frame_mut(&mut self) -> &mut bool;
}

impl RailRideHeliSegmentDataTrait for RailRideHeliSegmentData {
    fn editor_color(&self) -> &super::core::Vec4 {
        &self.editor_color
    }
    fn editor_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.editor_color
    }
    fn looping(&self) -> &bool {
        &self.looping
    }
    fn looping_mut(&mut self) -> &mut bool {
        &mut self.looping
    }
    fn start_index(&self) -> &i32 {
        &self.start_index
    }
    fn start_index_mut(&mut self) -> &mut i32 {
        &mut self.start_index
    }
    fn end_index(&self) -> &i32 {
        &self.end_index
    }
    fn end_index_mut(&mut self) -> &mut i32 {
        &mut self.end_index
    }
    fn teleport_on_activate(&self) -> &bool {
        &self.teleport_on_activate
    }
    fn teleport_on_activate_mut(&mut self) -> &mut bool {
        &mut self.teleport_on_activate
    }
    fn force_speed_on_activate(&self) -> &bool {
        &self.force_speed_on_activate
    }
    fn force_speed_on_activate_mut(&mut self) -> &mut bool {
        &mut self.force_speed_on_activate
    }
    fn force_direction_on_activate(&self) -> &bool {
        &self.force_direction_on_activate
    }
    fn force_direction_on_activate_mut(&mut self) -> &mut bool {
        &mut self.force_direction_on_activate
    }
    fn teleport_on_loop(&self) -> &bool {
        &self.teleport_on_loop
    }
    fn teleport_on_loop_mut(&mut self) -> &mut bool {
        &mut self.teleport_on_loop
    }
    fn node_index1(&self) -> &i32 {
        &self.node_index1
    }
    fn node_index1_mut(&mut self) -> &mut i32 {
        &mut self.node_index1
    }
    fn node_index2(&self) -> &i32 {
        &self.node_index2
    }
    fn node_index2_mut(&mut self) -> &mut i32 {
        &mut self.node_index2
    }
    fn node_index3(&self) -> &i32 {
        &self.node_index3
    }
    fn node_index3_mut(&mut self) -> &mut i32 {
        &mut self.node_index3
    }
    fn node_on_reached(&self) -> &Vec<i32> {
        &self.node_on_reached
    }
    fn node_on_reached_mut(&mut self) -> &mut Vec<i32> {
        &mut self.node_on_reached
    }
    fn node_on_reached_event_hashes(&self) -> &Vec<i32> {
        &self.node_on_reached_event_hashes
    }
    fn node_on_reached_event_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.node_on_reached_event_hashes
    }
    fn node_on_reached_property_hashes(&self) -> &Vec<i32> {
        &self.node_on_reached_property_hashes
    }
    fn node_on_reached_property_hashes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.node_on_reached_property_hashes
    }
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn external_time_mut(&mut self) -> &mut f32 {
        &mut self.external_time
    }
    fn waypoints_speed_scale(&self) -> &f32 {
        &self.waypoints_speed_scale
    }
    fn waypoints_speed_scale_mut(&mut self) -> &mut f32 {
        &mut self.waypoints_speed_scale
    }
    fn update_waypoints_speed_scale_every_frame(&self) -> &bool {
        &self.update_waypoints_speed_scale_every_frame
    }
    fn update_waypoints_speed_scale_every_frame_mut(&mut self) -> &mut bool {
        &mut self.update_waypoints_speed_scale_every_frame
    }
}

impl super::entity::EntityDataTrait for RailRideHeliSegmentData {
}

impl super::entity::GameObjectDataTrait for RailRideHeliSegmentData {
}

impl super::core::DataBusPeerTrait for RailRideHeliSegmentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RailRideHeliSegmentData {
}

impl super::core::DataContainerTrait for RailRideHeliSegmentData {
}

pub static RAILRIDEHELISEGMENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentData",
    name_hash: 2103559314,
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(RailRideHeliSegmentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RailRideHeliSegmentData as Default>::default())),
            create_boxed: || Box::new(<RailRideHeliSegmentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EditorColor",
                name_hash: 3118032409,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(RailRideHeliSegmentData, editor_color),
            },
            FieldInfoData {
                name: "Looping",
                name_hash: 1366646169,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliSegmentData, looping),
            },
            FieldInfoData {
                name: "StartIndex",
                name_hash: 2746865595,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RailRideHeliSegmentData, start_index),
            },
            FieldInfoData {
                name: "EndIndex",
                name_hash: 3992262036,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RailRideHeliSegmentData, end_index),
            },
            FieldInfoData {
                name: "TeleportOnActivate",
                name_hash: 321538620,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliSegmentData, teleport_on_activate),
            },
            FieldInfoData {
                name: "ForceSpeedOnActivate",
                name_hash: 1822409991,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliSegmentData, force_speed_on_activate),
            },
            FieldInfoData {
                name: "ForceDirectionOnActivate",
                name_hash: 3063057349,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliSegmentData, force_direction_on_activate),
            },
            FieldInfoData {
                name: "TeleportOnLoop",
                name_hash: 664053881,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliSegmentData, teleport_on_loop),
            },
            FieldInfoData {
                name: "NodeIndex1",
                name_hash: 3416174378,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RailRideHeliSegmentData, node_index1),
            },
            FieldInfoData {
                name: "NodeIndex2",
                name_hash: 3416174377,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RailRideHeliSegmentData, node_index2),
            },
            FieldInfoData {
                name: "NodeIndex3",
                name_hash: 3416174376,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RailRideHeliSegmentData, node_index3),
            },
            FieldInfoData {
                name: "NodeOnReached",
                name_hash: 3635116824,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(RailRideHeliSegmentData, node_on_reached),
            },
            FieldInfoData {
                name: "NodeOnReachedEventHashes",
                name_hash: 1140056496,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(RailRideHeliSegmentData, node_on_reached_event_hashes),
            },
            FieldInfoData {
                name: "NodeOnReachedPropertyHashes",
                name_hash: 45692123,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(RailRideHeliSegmentData, node_on_reached_property_hashes),
            },
            FieldInfoData {
                name: "ExternalTime",
                name_hash: 2162678253,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliSegmentData, external_time),
            },
            FieldInfoData {
                name: "WaypointsSpeedScale",
                name_hash: 1854800842,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliSegmentData, waypoints_speed_scale),
            },
            FieldInfoData {
                name: "UpdateWaypointsSpeedScaleEveryFrame",
                name_hash: 40937019,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliSegmentData, update_waypoints_speed_scale_every_frame),
            },
        ],
    }),
    array_type: Some(RAILRIDEHELISEGMENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideHeliSegmentData {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDEHELISEGMENTDATA_TYPE_INFO
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


pub static RAILRIDEHELISEGMENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentData-Array",
    name_hash: 597740454,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliSegmentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RailRideNodePointToData {
    pub _glacier_base: super::entity::GameComponentEntityData,
    pub side: RailRideHeliPointToSide,
}

pub trait RailRideNodePointToDataTrait: super::entity::GameComponentEntityDataTrait {
    fn side(&self) -> &RailRideHeliPointToSide;
    fn side_mut(&mut self) -> &mut RailRideHeliPointToSide;
}

impl RailRideNodePointToDataTrait for RailRideNodePointToData {
    fn side(&self) -> &RailRideHeliPointToSide {
        &self.side
    }
    fn side_mut(&mut self) -> &mut RailRideHeliPointToSide {
        &mut self.side
    }
}

impl super::entity::GameComponentEntityDataTrait for RailRideNodePointToData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for RailRideNodePointToData {
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn part_bounding_boxes(&self) -> &Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes()
    }
    fn part_bounding_boxes_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::AxisAlignedBox */> {
        self._glacier_base.part_bounding_boxes_mut()
    }
    fn client_runtime_component_count(&self) -> &u8 {
        self._glacier_base.client_runtime_component_count()
    }
    fn client_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_component_count_mut()
    }
    fn server_runtime_component_count(&self) -> &u8 {
        self._glacier_base.server_runtime_component_count()
    }
    fn server_runtime_component_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_component_count_mut()
    }
    fn client_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.client_runtime_transformation_count()
    }
    fn client_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_runtime_transformation_count_mut()
    }
    fn server_runtime_transformation_count(&self) -> &u8 {
        self._glacier_base.server_runtime_transformation_count()
    }
    fn server_runtime_transformation_count_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_runtime_transformation_count_mut()
    }
}

impl super::entity::SpatialEntityDataTrait for RailRideNodePointToData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for RailRideNodePointToData {
}

impl super::entity::GameObjectDataTrait for RailRideNodePointToData {
}

impl super::core::DataBusPeerTrait for RailRideNodePointToData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RailRideNodePointToData {
}

impl super::core::DataContainerTrait for RailRideNodePointToData {
}

pub static RAILRIDENODEPOINTTODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideNodePointToData",
    name_hash: 2588246222,
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(RailRideNodePointToData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RailRideNodePointToData as Default>::default())),
            create_boxed: || Box::new(<RailRideNodePointToData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Side",
                name_hash: 2089430110,
                flags: MemberInfoFlags::new(0),
                field_type: "RailRideHeliPointToSide",
                rust_offset: offset_of!(RailRideNodePointToData, side),
            },
        ],
    }),
    array_type: Some(RAILRIDENODEPOINTTODATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideNodePointToData {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDENODEPOINTTODATA_TYPE_INFO
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


pub static RAILRIDENODEPOINTTODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideNodePointToData-Array",
    name_hash: 3211606650,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideNodePointToData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RailRideHeliPointToSide {
    #[default]
    Front = 0,
    Left = 1,
    Right = 2,
}

pub static RAILRIDEHELIPOINTTOSIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliPointToSide",
    name_hash: 3587824301,
    flags: MemberInfoFlags::new(49429),
    module: "RailRideHeli",
    data: TypeInfoData::Enum,
    array_type: Some(RAILRIDEHELIPOINTTOSIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RailRideHeliPointToSide {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDEHELIPOINTTOSIDE_TYPE_INFO
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


pub static RAILRIDEHELIPOINTTOSIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliPointToSide-Array",
    name_hash: 1497894681,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliPointToSide"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RailRideHeliData {
    pub _glacier_base: super::entity::EntityData,
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

pub trait RailRideHeliDataTrait: super::entity::EntityDataTrait {
    fn control_type(&self) -> &RailRideHeliControlType;
    fn control_type_mut(&mut self) -> &mut RailRideHeliControlType;
    fn network_control_transform(&self) -> &bool;
    fn network_control_transform_mut(&mut self) -> &mut bool;
    fn wait_on_client_activation(&self) -> &bool;
    fn wait_on_client_activation_mut(&mut self) -> &mut bool;
    fn client_activation_blend_time(&self) -> &f32;
    fn client_activation_blend_time_mut(&mut self) -> &mut f32;
    fn angular_approach_slow(&self) -> &f32;
    fn angular_approach_slow_mut(&mut self) -> &mut f32;
    fn angular_approach_fast(&self) -> &f32;
    fn angular_approach_fast_mut(&mut self) -> &mut f32;
    fn tilt_max(&self) -> &f32;
    fn tilt_max_mut(&mut self) -> &mut f32;
    fn tilt_accel_mult(&self) -> &f32;
    fn tilt_accel_mult_mut(&mut self) -> &mut f32;
    fn tilt_speed_mult(&self) -> &f32;
    fn tilt_speed_mult_mut(&mut self) -> &mut f32;
    fn veer_duration_min(&self) -> &f32;
    fn veer_duration_min_mut(&mut self) -> &mut f32;
    fn veer_duration_max(&self) -> &f32;
    fn veer_duration_max_mut(&mut self) -> &mut f32;
    fn veer_position_offset_min(&self) -> &f32;
    fn veer_position_offset_min_mut(&mut self) -> &mut f32;
    fn veer_position_offset_max(&self) -> &f32;
    fn veer_position_offset_max_mut(&mut self) -> &mut f32;
    fn veer_heading_offset_min(&self) -> &f32;
    fn veer_heading_offset_min_mut(&mut self) -> &mut f32;
    fn veer_heading_offset_max(&self) -> &f32;
    fn veer_heading_offset_max_mut(&mut self) -> &mut f32;
    fn simulation_steps_per_frame(&self) -> &i32;
    fn simulation_steps_per_frame_mut(&mut self) -> &mut i32;
    fn sound_accel_to_throttle_mult(&self) -> &f32;
    fn sound_accel_to_throttle_mult_mut(&mut self) -> &mut f32;
    fn sound_speed_to_throttle_mult(&self) -> &f32;
    fn sound_speed_to_throttle_mult_mut(&mut self) -> &mut f32;
    fn speed_multiplier(&self) -> &f32;
    fn speed_multiplier_mut(&mut self) -> &mut f32;
    fn wing_mode_on_activate(&self) -> &bool;
    fn wing_mode_on_activate_mut(&mut self) -> &mut bool;
    fn root_transform(&self) -> &super::core::LinearTransform;
    fn root_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn reference_transform(&self) -> &super::core::LinearTransform;
    fn reference_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn blend_transform(&self) -> &super::core::LinearTransform;
    fn blend_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn blend(&self) -> &f32;
    fn blend_mut(&mut self) -> &mut f32;
    fn point_to_override(&self) -> &super::core::Vec3;
    fn point_to_override_mut(&mut self) -> &mut super::core::Vec3;
    fn enable_point_to_override(&self) -> &bool;
    fn enable_point_to_override_mut(&mut self) -> &mut bool;
    fn use_segment_external_time(&self) -> &bool;
    fn use_segment_external_time_mut(&mut self) -> &mut bool;
    fn braking(&self) -> &bool;
    fn braking_mut(&mut self) -> &mut bool;
}

impl RailRideHeliDataTrait for RailRideHeliData {
    fn control_type(&self) -> &RailRideHeliControlType {
        &self.control_type
    }
    fn control_type_mut(&mut self) -> &mut RailRideHeliControlType {
        &mut self.control_type
    }
    fn network_control_transform(&self) -> &bool {
        &self.network_control_transform
    }
    fn network_control_transform_mut(&mut self) -> &mut bool {
        &mut self.network_control_transform
    }
    fn wait_on_client_activation(&self) -> &bool {
        &self.wait_on_client_activation
    }
    fn wait_on_client_activation_mut(&mut self) -> &mut bool {
        &mut self.wait_on_client_activation
    }
    fn client_activation_blend_time(&self) -> &f32 {
        &self.client_activation_blend_time
    }
    fn client_activation_blend_time_mut(&mut self) -> &mut f32 {
        &mut self.client_activation_blend_time
    }
    fn angular_approach_slow(&self) -> &f32 {
        &self.angular_approach_slow
    }
    fn angular_approach_slow_mut(&mut self) -> &mut f32 {
        &mut self.angular_approach_slow
    }
    fn angular_approach_fast(&self) -> &f32 {
        &self.angular_approach_fast
    }
    fn angular_approach_fast_mut(&mut self) -> &mut f32 {
        &mut self.angular_approach_fast
    }
    fn tilt_max(&self) -> &f32 {
        &self.tilt_max
    }
    fn tilt_max_mut(&mut self) -> &mut f32 {
        &mut self.tilt_max
    }
    fn tilt_accel_mult(&self) -> &f32 {
        &self.tilt_accel_mult
    }
    fn tilt_accel_mult_mut(&mut self) -> &mut f32 {
        &mut self.tilt_accel_mult
    }
    fn tilt_speed_mult(&self) -> &f32 {
        &self.tilt_speed_mult
    }
    fn tilt_speed_mult_mut(&mut self) -> &mut f32 {
        &mut self.tilt_speed_mult
    }
    fn veer_duration_min(&self) -> &f32 {
        &self.veer_duration_min
    }
    fn veer_duration_min_mut(&mut self) -> &mut f32 {
        &mut self.veer_duration_min
    }
    fn veer_duration_max(&self) -> &f32 {
        &self.veer_duration_max
    }
    fn veer_duration_max_mut(&mut self) -> &mut f32 {
        &mut self.veer_duration_max
    }
    fn veer_position_offset_min(&self) -> &f32 {
        &self.veer_position_offset_min
    }
    fn veer_position_offset_min_mut(&mut self) -> &mut f32 {
        &mut self.veer_position_offset_min
    }
    fn veer_position_offset_max(&self) -> &f32 {
        &self.veer_position_offset_max
    }
    fn veer_position_offset_max_mut(&mut self) -> &mut f32 {
        &mut self.veer_position_offset_max
    }
    fn veer_heading_offset_min(&self) -> &f32 {
        &self.veer_heading_offset_min
    }
    fn veer_heading_offset_min_mut(&mut self) -> &mut f32 {
        &mut self.veer_heading_offset_min
    }
    fn veer_heading_offset_max(&self) -> &f32 {
        &self.veer_heading_offset_max
    }
    fn veer_heading_offset_max_mut(&mut self) -> &mut f32 {
        &mut self.veer_heading_offset_max
    }
    fn simulation_steps_per_frame(&self) -> &i32 {
        &self.simulation_steps_per_frame
    }
    fn simulation_steps_per_frame_mut(&mut self) -> &mut i32 {
        &mut self.simulation_steps_per_frame
    }
    fn sound_accel_to_throttle_mult(&self) -> &f32 {
        &self.sound_accel_to_throttle_mult
    }
    fn sound_accel_to_throttle_mult_mut(&mut self) -> &mut f32 {
        &mut self.sound_accel_to_throttle_mult
    }
    fn sound_speed_to_throttle_mult(&self) -> &f32 {
        &self.sound_speed_to_throttle_mult
    }
    fn sound_speed_to_throttle_mult_mut(&mut self) -> &mut f32 {
        &mut self.sound_speed_to_throttle_mult
    }
    fn speed_multiplier(&self) -> &f32 {
        &self.speed_multiplier
    }
    fn speed_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.speed_multiplier
    }
    fn wing_mode_on_activate(&self) -> &bool {
        &self.wing_mode_on_activate
    }
    fn wing_mode_on_activate_mut(&mut self) -> &mut bool {
        &mut self.wing_mode_on_activate
    }
    fn root_transform(&self) -> &super::core::LinearTransform {
        &self.root_transform
    }
    fn root_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.root_transform
    }
    fn reference_transform(&self) -> &super::core::LinearTransform {
        &self.reference_transform
    }
    fn reference_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.reference_transform
    }
    fn blend_transform(&self) -> &super::core::LinearTransform {
        &self.blend_transform
    }
    fn blend_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.blend_transform
    }
    fn blend(&self) -> &f32 {
        &self.blend
    }
    fn blend_mut(&mut self) -> &mut f32 {
        &mut self.blend
    }
    fn point_to_override(&self) -> &super::core::Vec3 {
        &self.point_to_override
    }
    fn point_to_override_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.point_to_override
    }
    fn enable_point_to_override(&self) -> &bool {
        &self.enable_point_to_override
    }
    fn enable_point_to_override_mut(&mut self) -> &mut bool {
        &mut self.enable_point_to_override
    }
    fn use_segment_external_time(&self) -> &bool {
        &self.use_segment_external_time
    }
    fn use_segment_external_time_mut(&mut self) -> &mut bool {
        &mut self.use_segment_external_time
    }
    fn braking(&self) -> &bool {
        &self.braking
    }
    fn braking_mut(&mut self) -> &mut bool {
        &mut self.braking
    }
}

impl super::entity::EntityDataTrait for RailRideHeliData {
}

impl super::entity::GameObjectDataTrait for RailRideHeliData {
}

impl super::core::DataBusPeerTrait for RailRideHeliData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RailRideHeliData {
}

impl super::core::DataContainerTrait for RailRideHeliData {
}

pub static RAILRIDEHELIDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliData",
    name_hash: 3224187057,
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(RailRideHeliData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RailRideHeliData as Default>::default())),
            create_boxed: || Box::new(<RailRideHeliData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ControlType",
                name_hash: 1607477722,
                flags: MemberInfoFlags::new(0),
                field_type: "RailRideHeliControlType",
                rust_offset: offset_of!(RailRideHeliData, control_type),
            },
            FieldInfoData {
                name: "NetworkControlTransform",
                name_hash: 2118676048,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliData, network_control_transform),
            },
            FieldInfoData {
                name: "WaitOnClientActivation",
                name_hash: 3532761762,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliData, wait_on_client_activation),
            },
            FieldInfoData {
                name: "ClientActivationBlendTime",
                name_hash: 4208139292,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, client_activation_blend_time),
            },
            FieldInfoData {
                name: "AngularApproachSlow",
                name_hash: 3814374870,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, angular_approach_slow),
            },
            FieldInfoData {
                name: "AngularApproachFast",
                name_hash: 3815047601,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, angular_approach_fast),
            },
            FieldInfoData {
                name: "TiltMax",
                name_hash: 3345235156,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, tilt_max),
            },
            FieldInfoData {
                name: "TiltAccelMult",
                name_hash: 2825680360,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, tilt_accel_mult),
            },
            FieldInfoData {
                name: "TiltSpeedMult",
                name_hash: 2127594503,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, tilt_speed_mult),
            },
            FieldInfoData {
                name: "VeerDurationMin",
                name_hash: 932416213,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, veer_duration_min),
            },
            FieldInfoData {
                name: "VeerDurationMax",
                name_hash: 932415947,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, veer_duration_max),
            },
            FieldInfoData {
                name: "VeerPositionOffsetMin",
                name_hash: 341595359,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, veer_position_offset_min),
            },
            FieldInfoData {
                name: "VeerPositionOffsetMax",
                name_hash: 341595585,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, veer_position_offset_max),
            },
            FieldInfoData {
                name: "VeerHeadingOffsetMin",
                name_hash: 3108076750,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, veer_heading_offset_min),
            },
            FieldInfoData {
                name: "VeerHeadingOffsetMax",
                name_hash: 3108077008,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, veer_heading_offset_max),
            },
            FieldInfoData {
                name: "SimulationStepsPerFrame",
                name_hash: 2564698285,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RailRideHeliData, simulation_steps_per_frame),
            },
            FieldInfoData {
                name: "SoundAccelToThrottleMult",
                name_hash: 3826277853,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, sound_accel_to_throttle_mult),
            },
            FieldInfoData {
                name: "SoundSpeedToThrottleMult",
                name_hash: 3099079282,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, sound_speed_to_throttle_mult),
            },
            FieldInfoData {
                name: "SpeedMultiplier",
                name_hash: 1688256841,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, speed_multiplier),
            },
            FieldInfoData {
                name: "WingModeOnActivate",
                name_hash: 3826891529,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliData, wing_mode_on_activate),
            },
            FieldInfoData {
                name: "RootTransform",
                name_hash: 3407367919,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(RailRideHeliData, root_transform),
            },
            FieldInfoData {
                name: "ReferenceTransform",
                name_hash: 2231497250,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(RailRideHeliData, reference_transform),
            },
            FieldInfoData {
                name: "BlendTransform",
                name_hash: 1068622440,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(RailRideHeliData, blend_transform),
            },
            FieldInfoData {
                name: "Blend",
                name_hash: 211525156,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RailRideHeliData, blend),
            },
            FieldInfoData {
                name: "PointToOverride",
                name_hash: 3171831750,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RailRideHeliData, point_to_override),
            },
            FieldInfoData {
                name: "EnablePointToOverride",
                name_hash: 465597511,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliData, enable_point_to_override),
            },
            FieldInfoData {
                name: "UseSegmentExternalTime",
                name_hash: 1318838765,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliData, use_segment_external_time),
            },
            FieldInfoData {
                name: "Braking",
                name_hash: 2953397663,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RailRideHeliData, braking),
            },
        ],
    }),
    array_type: Some(RAILRIDEHELIDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RailRideHeliData {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDEHELIDATA_TYPE_INFO
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


pub static RAILRIDEHELIDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliData-Array",
    name_hash: 4061635333,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RailRideHeliControlType {
    #[default]
    RailRideHeliControl_KeyframedVehicle = 0,
    RailRideHeliControl_RawTransform = 1,
}

pub static RAILRIDEHELICONTROLTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliControlType",
    name_hash: 1881554590,
    flags: MemberInfoFlags::new(49429),
    module: "RailRideHeli",
    data: TypeInfoData::Enum,
    array_type: Some(RAILRIDEHELICONTROLTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RailRideHeliControlType {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDEHELICONTROLTYPE_TYPE_INFO
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


pub static RAILRIDEHELICONTROLTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliControlType-Array",
    name_hash: 3189231530,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliControlType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RailRideHeliClientActivatedMessage {
}

pub trait RailRideHeliClientActivatedMessageTrait: TypeObject {
}

impl RailRideHeliClientActivatedMessageTrait for RailRideHeliClientActivatedMessage {
}

pub static RAILRIDEHELICLIENTACTIVATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliClientActivatedMessage",
    name_hash: 391446446,
    flags: MemberInfoFlags::new(73),
    module: "RailRideHeli",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RailRideHeliClientActivatedMessage as Default>::default())),
            create_boxed: || Box::new(<RailRideHeliClientActivatedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for RailRideHeliClientActivatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDEHELICLIENTACTIVATEDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerRailRideHeliEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerRailRideHeliEntityTrait: super::entity::EntityTrait {
}

impl ServerRailRideHeliEntityTrait for ServerRailRideHeliEntity {
}

impl super::entity::EntityTrait for ServerRailRideHeliEntity {
}

impl super::entity::EntityBusPeerTrait for ServerRailRideHeliEntity {
}

pub static SERVERRAILRIDEHELIENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRailRideHeliEntity",
    name_hash: 1409555455,
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerRailRideHeliEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerRailRideHeliEntity as Default>::default())),
            create_boxed: || Box::new(<ServerRailRideHeliEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERRAILRIDEHELIENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerRailRideHeliEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERRAILRIDEHELIENTITY_TYPE_INFO
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


pub static SERVERRAILRIDEHELIENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerRailRideHeliEntity-Array",
    name_hash: 2306703819,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("ServerRailRideHeliEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RailRideHeliSegmentEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait RailRideHeliSegmentEntityTrait: super::entity::EntityTrait {
}

impl RailRideHeliSegmentEntityTrait for RailRideHeliSegmentEntity {
}

impl super::entity::EntityTrait for RailRideHeliSegmentEntity {
}

impl super::entity::EntityBusPeerTrait for RailRideHeliSegmentEntity {
}

pub static RAILRIDEHELISEGMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentEntity",
    name_hash: 1833926393,
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(RailRideHeliSegmentEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RailRideHeliSegmentEntity as Default>::default())),
            create_boxed: || Box::new(<RailRideHeliSegmentEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(RAILRIDEHELISEGMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RailRideHeliSegmentEntity {
    fn type_info(&self) -> &'static TypeInfo {
        RAILRIDEHELISEGMENTENTITY_TYPE_INFO
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


pub static RAILRIDEHELISEGMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RailRideHeliSegmentEntity-Array",
    name_hash: 966475213,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("RailRideHeliSegmentEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientRailRideHeliEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientRailRideHeliEntityTrait: super::entity::EntityTrait {
}

impl ClientRailRideHeliEntityTrait for ClientRailRideHeliEntity {
}

impl super::entity::EntityTrait for ClientRailRideHeliEntity {
}

impl super::entity::EntityBusPeerTrait for ClientRailRideHeliEntity {
}

pub static CLIENTRAILRIDEHELIENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRailRideHeliEntity",
    name_hash: 2895764387,
    flags: MemberInfoFlags::new(101),
    module: "RailRideHeli",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientRailRideHeliEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientRailRideHeliEntity as Default>::default())),
            create_boxed: || Box::new(<ClientRailRideHeliEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTRAILRIDEHELIENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientRailRideHeliEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTRAILRIDEHELIENTITY_TYPE_INFO
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


pub static CLIENTRAILRIDEHELIENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientRailRideHeliEntity-Array",
    name_hash: 1891516183,
    flags: MemberInfoFlags::new(145),
    module: "RailRideHeli",
    data: TypeInfoData::Array("ClientRailRideHeliEntity"),
    array_type: None,
    alignment: 8,
};


