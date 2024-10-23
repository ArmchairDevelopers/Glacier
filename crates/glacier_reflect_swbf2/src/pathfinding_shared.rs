use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct VehicleWaypointData {
    pub _glacier_base: WaypointData,
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

pub trait VehicleWaypointDataTrait: WaypointDataTrait {
    fn speed(&self) -> &f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn speed_override_moving_towards(&self) -> &f32;
    fn speed_override_moving_towards_mut(&mut self) -> &mut f32;
    fn speed_limit_on_reached(&self) -> &f32;
    fn speed_limit_on_reached_mut(&mut self) -> &mut f32;
    fn speed_limit_moving_towards(&self) -> &f32;
    fn speed_limit_moving_towards_mut(&mut self) -> &mut f32;
    fn use_heading(&self) -> &bool;
    fn use_heading_mut(&mut self) -> &mut bool;
    fn heading(&self) -> &f32;
    fn heading_mut(&mut self) -> &mut f32;
    fn stop_here(&self) -> &bool;
    fn stop_here_mut(&mut self) -> &mut bool;
    fn wait_here(&self) -> &f32;
    fn wait_here_mut(&mut self) -> &mut f32;
    fn stopping_deceleration(&self) -> &f32;
    fn stopping_deceleration_mut(&mut self) -> &mut f32;
    fn min_slowdown_speed(&self) -> &f32;
    fn min_slowdown_speed_mut(&mut self) -> &mut f32;
    fn stop_here_radius(&self) -> &f32;
    fn stop_here_radius_mut(&mut self) -> &mut f32;
}

impl VehicleWaypointDataTrait for VehicleWaypointData {
    fn speed(&self) -> &f32 {
        &self.speed
    }
    fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }
    fn speed_override_moving_towards(&self) -> &f32 {
        &self.speed_override_moving_towards
    }
    fn speed_override_moving_towards_mut(&mut self) -> &mut f32 {
        &mut self.speed_override_moving_towards
    }
    fn speed_limit_on_reached(&self) -> &f32 {
        &self.speed_limit_on_reached
    }
    fn speed_limit_on_reached_mut(&mut self) -> &mut f32 {
        &mut self.speed_limit_on_reached
    }
    fn speed_limit_moving_towards(&self) -> &f32 {
        &self.speed_limit_moving_towards
    }
    fn speed_limit_moving_towards_mut(&mut self) -> &mut f32 {
        &mut self.speed_limit_moving_towards
    }
    fn use_heading(&self) -> &bool {
        &self.use_heading
    }
    fn use_heading_mut(&mut self) -> &mut bool {
        &mut self.use_heading
    }
    fn heading(&self) -> &f32 {
        &self.heading
    }
    fn heading_mut(&mut self) -> &mut f32 {
        &mut self.heading
    }
    fn stop_here(&self) -> &bool {
        &self.stop_here
    }
    fn stop_here_mut(&mut self) -> &mut bool {
        &mut self.stop_here
    }
    fn wait_here(&self) -> &f32 {
        &self.wait_here
    }
    fn wait_here_mut(&mut self) -> &mut f32 {
        &mut self.wait_here
    }
    fn stopping_deceleration(&self) -> &f32 {
        &self.stopping_deceleration
    }
    fn stopping_deceleration_mut(&mut self) -> &mut f32 {
        &mut self.stopping_deceleration
    }
    fn min_slowdown_speed(&self) -> &f32 {
        &self.min_slowdown_speed
    }
    fn min_slowdown_speed_mut(&mut self) -> &mut f32 {
        &mut self.min_slowdown_speed
    }
    fn stop_here_radius(&self) -> &f32 {
        &self.stop_here_radius
    }
    fn stop_here_radius_mut(&mut self) -> &mut f32 {
        &mut self.stop_here_radius
    }
}

impl WaypointDataTrait for VehicleWaypointData {
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

impl super::core::DataContainerTrait for VehicleWaypointData {
}

pub static VEHICLEWAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleWaypointData",
    name_hash: 2414246990,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WAYPOINTDATA_TYPE_INFO),
        super_class_offset: offset_of!(VehicleWaypointData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VehicleWaypointData as Default>::default())),
            create_boxed: || Box::new(<VehicleWaypointData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Speed",
                name_hash: 230887042,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, speed),
            },
            FieldInfoData {
                name: "SpeedOverrideMovingTowards",
                name_hash: 4149709546,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, speed_override_moving_towards),
            },
            FieldInfoData {
                name: "SpeedLimitOnReached",
                name_hash: 952337194,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, speed_limit_on_reached),
            },
            FieldInfoData {
                name: "SpeedLimitMovingTowards",
                name_hash: 1517902571,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, speed_limit_moving_towards),
            },
            FieldInfoData {
                name: "UseHeading",
                name_hash: 3595199662,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleWaypointData, use_heading),
            },
            FieldInfoData {
                name: "Heading",
                name_hash: 2009143629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, heading),
            },
            FieldInfoData {
                name: "StopHere",
                name_hash: 3655484487,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(VehicleWaypointData, stop_here),
            },
            FieldInfoData {
                name: "WaitHere",
                name_hash: 3637155732,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, wait_here),
            },
            FieldInfoData {
                name: "StoppingDeceleration",
                name_hash: 2524085644,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, stopping_deceleration),
            },
            FieldInfoData {
                name: "MinSlowdownSpeed",
                name_hash: 3207717373,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, min_slowdown_speed),
            },
            FieldInfoData {
                name: "StopHereRadius",
                name_hash: 3378547103,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VehicleWaypointData, stop_here_radius),
            },
        ],
    }),
    array_type: Some(VEHICLEWAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VehicleWaypointData {
    fn type_info(&self) -> &'static TypeInfo {
        VEHICLEWAYPOINTDATA_TYPE_INFO
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


pub static VEHICLEWAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VehicleWaypointData-Array",
    name_hash: 1136449018,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("VehicleWaypointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WaypointsShapeData {
    pub _glacier_base: super::entity::VectorShapeData,
    pub waypoints: Vec<Option<LockedTypeObject /* WaypointData */>>,
}

pub trait WaypointsShapeDataTrait: super::entity::VectorShapeDataTrait {
    fn waypoints(&self) -> &Vec<Option<LockedTypeObject /* WaypointData */>>;
    fn waypoints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* WaypointData */>>;
}

impl WaypointsShapeDataTrait for WaypointsShapeData {
    fn waypoints(&self) -> &Vec<Option<LockedTypeObject /* WaypointData */>> {
        &self.waypoints
    }
    fn waypoints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* WaypointData */>> {
        &mut self.waypoints
    }
}

impl super::entity::VectorShapeDataTrait for WaypointsShapeData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points()
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        self._glacier_base.points_mut()
    }
    fn tension(&self) -> &f32 {
        self._glacier_base.tension()
    }
    fn tension_mut(&mut self) -> &mut f32 {
        self._glacier_base.tension_mut()
    }
    fn is_closed(&self) -> &bool {
        self._glacier_base.is_closed()
    }
    fn is_closed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_closed_mut()
    }
    fn allow_roll(&self) -> &bool {
        self._glacier_base.allow_roll()
    }
    fn allow_roll_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_roll_mut()
    }
    fn allow_yaw_pitch(&self) -> &bool {
        self._glacier_base.allow_yaw_pitch()
    }
    fn allow_yaw_pitch_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_yaw_pitch_mut()
    }
}

impl super::entity::BaseShapeDataTrait for WaypointsShapeData {
}

impl super::entity::BaseShapeDataBaseTrait for WaypointsShapeData {
}

impl super::entity::GameObjectDataTrait for WaypointsShapeData {
}

impl super::core::DataBusPeerTrait for WaypointsShapeData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WaypointsShapeData {
}

impl super::core::DataContainerTrait for WaypointsShapeData {
}

pub static WAYPOINTSSHAPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsShapeData",
    name_hash: 1536517738,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::VECTORSHAPEDATA_TYPE_INFO),
        super_class_offset: offset_of!(WaypointsShapeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaypointsShapeData as Default>::default())),
            create_boxed: || Box::new(<WaypointsShapeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Waypoints",
                name_hash: 1635609973,
                flags: MemberInfoFlags::new(144),
                field_type: "WaypointData-Array",
                rust_offset: offset_of!(WaypointsShapeData, waypoints),
            },
        ],
    }),
    array_type: Some(WAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaypointsShapeData {
    fn type_info(&self) -> &'static TypeInfo {
        WAYPOINTSSHAPEDATA_TYPE_INFO
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


pub static WAYPOINTSSHAPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsShapeData-Array",
    name_hash: 884912478,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("WaypointsShapeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WaypointData {
    pub _glacier_base: super::core::DataContainer,
    pub use_clients_position: bool,
    pub schematics_name_hash: i32,
    pub waypoint_id: u32,
}

pub trait WaypointDataTrait: super::core::DataContainerTrait {
    fn use_clients_position(&self) -> &bool;
    fn use_clients_position_mut(&mut self) -> &mut bool;
    fn schematics_name_hash(&self) -> &i32;
    fn schematics_name_hash_mut(&mut self) -> &mut i32;
    fn waypoint_id(&self) -> &u32;
    fn waypoint_id_mut(&mut self) -> &mut u32;
}

impl WaypointDataTrait for WaypointData {
    fn use_clients_position(&self) -> &bool {
        &self.use_clients_position
    }
    fn use_clients_position_mut(&mut self) -> &mut bool {
        &mut self.use_clients_position
    }
    fn schematics_name_hash(&self) -> &i32 {
        &self.schematics_name_hash
    }
    fn schematics_name_hash_mut(&mut self) -> &mut i32 {
        &mut self.schematics_name_hash
    }
    fn waypoint_id(&self) -> &u32 {
        &self.waypoint_id
    }
    fn waypoint_id_mut(&mut self) -> &mut u32 {
        &mut self.waypoint_id
    }
}

impl super::core::DataContainerTrait for WaypointData {
}

pub static WAYPOINTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointData",
    name_hash: 2286426486,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(WaypointData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaypointData as Default>::default())),
            create_boxed: || Box::new(<WaypointData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UseClientsPosition",
                name_hash: 2175538773,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WaypointData, use_clients_position),
            },
            FieldInfoData {
                name: "SchematicsNameHash",
                name_hash: 2506236300,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WaypointData, schematics_name_hash),
            },
            FieldInfoData {
                name: "WaypointId",
                name_hash: 2435520331,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WaypointData, waypoint_id),
            },
        ],
    }),
    array_type: Some(WAYPOINTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaypointData {
    fn type_info(&self) -> &'static TypeInfo {
        WAYPOINTDATA_TYPE_INFO
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


pub static WAYPOINTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointData-Array",
    name_hash: 1170128450,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("WaypointData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RouteType {
    #[default]
    RouteStop = 0,
    RouteCircular = 1,
    RouteBackAndForth = 2,
}

pub static ROUTETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RouteType",
    name_hash: 887271396,
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(ROUTETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RouteType {
    fn type_info(&self) -> &'static TypeInfo {
        ROUTETYPE_TYPE_INFO
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


pub static ROUTETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RouteType-Array",
    name_hash: 827405264,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("RouteType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WaypointsSnappingSettings {
    #[default]
    UseShapeSettings = 0,
    SnapToTerrain = 1,
    NoSnap = 2,
}

pub static WAYPOINTSSNAPPINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsSnappingSettings",
    name_hash: 2928229388,
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(WAYPOINTSSNAPPINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WaypointsSnappingSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WAYPOINTSSNAPPINGSETTINGS_TYPE_INFO
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


pub static WAYPOINTSSNAPPINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaypointsSnappingSettings-Array",
    name_hash: 535030840,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("WaypointsSnappingSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FollowWaypointsEntityBaseData {
    pub _glacier_base: super::entity::EntityData,
    pub type_of_route: RouteType,
    pub use_path_finding: bool,
    pub start_at_geometrically_closest_waypoint: bool,
}

pub trait FollowWaypointsEntityBaseDataTrait: super::entity::EntityDataTrait {
    fn type_of_route(&self) -> &RouteType;
    fn type_of_route_mut(&mut self) -> &mut RouteType;
    fn use_path_finding(&self) -> &bool;
    fn use_path_finding_mut(&mut self) -> &mut bool;
    fn start_at_geometrically_closest_waypoint(&self) -> &bool;
    fn start_at_geometrically_closest_waypoint_mut(&mut self) -> &mut bool;
}

impl FollowWaypointsEntityBaseDataTrait for FollowWaypointsEntityBaseData {
    fn type_of_route(&self) -> &RouteType {
        &self.type_of_route
    }
    fn type_of_route_mut(&mut self) -> &mut RouteType {
        &mut self.type_of_route
    }
    fn use_path_finding(&self) -> &bool {
        &self.use_path_finding
    }
    fn use_path_finding_mut(&mut self) -> &mut bool {
        &mut self.use_path_finding
    }
    fn start_at_geometrically_closest_waypoint(&self) -> &bool {
        &self.start_at_geometrically_closest_waypoint
    }
    fn start_at_geometrically_closest_waypoint_mut(&mut self) -> &mut bool {
        &mut self.start_at_geometrically_closest_waypoint
    }
}

impl super::entity::EntityDataTrait for FollowWaypointsEntityBaseData {
}

impl super::entity::GameObjectDataTrait for FollowWaypointsEntityBaseData {
}

impl super::core::DataBusPeerTrait for FollowWaypointsEntityBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FollowWaypointsEntityBaseData {
}

impl super::core::DataContainerTrait for FollowWaypointsEntityBaseData {
}

pub static FOLLOWWAYPOINTSENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowWaypointsEntityBaseData",
    name_hash: 344209146,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(FollowWaypointsEntityBaseData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowWaypointsEntityBaseData as Default>::default())),
            create_boxed: || Box::new(<FollowWaypointsEntityBaseData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TypeOfRoute",
                name_hash: 2152665933,
                flags: MemberInfoFlags::new(0),
                field_type: "RouteType",
                rust_offset: offset_of!(FollowWaypointsEntityBaseData, type_of_route),
            },
            FieldInfoData {
                name: "UsePathFinding",
                name_hash: 2941510446,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FollowWaypointsEntityBaseData, use_path_finding),
            },
            FieldInfoData {
                name: "StartAtGeometricallyClosestWaypoint",
                name_hash: 4268018707,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FollowWaypointsEntityBaseData, start_at_geometrically_closest_waypoint),
            },
        ],
    }),
    array_type: Some(FOLLOWWAYPOINTSENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FollowWaypointsEntityBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        FOLLOWWAYPOINTSENTITYBASEDATA_TYPE_INFO
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


pub static FOLLOWWAYPOINTSENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowWaypointsEntityBaseData-Array",
    name_hash: 104507854,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("FollowWaypointsEntityBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathFollowingComponentBaseData {
    pub _glacier_base: super::entity::GameComponentData,
    pub update_path_at_distance_percent: f32,
    pub preferred_pathfinding_index: u32,
    pub alternate_pathfinding_indices: Vec<u32>,
    pub movement_corridor_radius: f32,
}

pub trait PathFollowingComponentBaseDataTrait: super::entity::GameComponentDataTrait {
    fn update_path_at_distance_percent(&self) -> &f32;
    fn update_path_at_distance_percent_mut(&mut self) -> &mut f32;
    fn preferred_pathfinding_index(&self) -> &u32;
    fn preferred_pathfinding_index_mut(&mut self) -> &mut u32;
    fn alternate_pathfinding_indices(&self) -> &Vec<u32>;
    fn alternate_pathfinding_indices_mut(&mut self) -> &mut Vec<u32>;
    fn movement_corridor_radius(&self) -> &f32;
    fn movement_corridor_radius_mut(&mut self) -> &mut f32;
}

impl PathFollowingComponentBaseDataTrait for PathFollowingComponentBaseData {
    fn update_path_at_distance_percent(&self) -> &f32 {
        &self.update_path_at_distance_percent
    }
    fn update_path_at_distance_percent_mut(&mut self) -> &mut f32 {
        &mut self.update_path_at_distance_percent
    }
    fn preferred_pathfinding_index(&self) -> &u32 {
        &self.preferred_pathfinding_index
    }
    fn preferred_pathfinding_index_mut(&mut self) -> &mut u32 {
        &mut self.preferred_pathfinding_index
    }
    fn alternate_pathfinding_indices(&self) -> &Vec<u32> {
        &self.alternate_pathfinding_indices
    }
    fn alternate_pathfinding_indices_mut(&mut self) -> &mut Vec<u32> {
        &mut self.alternate_pathfinding_indices
    }
    fn movement_corridor_radius(&self) -> &f32 {
        &self.movement_corridor_radius
    }
    fn movement_corridor_radius_mut(&mut self) -> &mut f32 {
        &mut self.movement_corridor_radius
    }
}

impl super::entity::GameComponentDataTrait for PathFollowingComponentBaseData {
}

impl super::entity::ComponentDataTrait for PathFollowingComponentBaseData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for PathFollowingComponentBaseData {
}

impl super::core::DataBusPeerTrait for PathFollowingComponentBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PathFollowingComponentBaseData {
}

impl super::core::DataContainerTrait for PathFollowingComponentBaseData {
}

pub static PATHFOLLOWINGCOMPONENTBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathFollowingComponentBaseData",
    name_hash: 302790611,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathFollowingComponentBaseData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathFollowingComponentBaseData as Default>::default())),
            create_boxed: || Box::new(<PathFollowingComponentBaseData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UpdatePathAtDistancePercent",
                name_hash: 324611860,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathFollowingComponentBaseData, update_path_at_distance_percent),
            },
            FieldInfoData {
                name: "PreferredPathfindingIndex",
                name_hash: 233434614,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PathFollowingComponentBaseData, preferred_pathfinding_index),
            },
            FieldInfoData {
                name: "AlternatePathfindingIndices",
                name_hash: 1978090370,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(PathFollowingComponentBaseData, alternate_pathfinding_indices),
            },
            FieldInfoData {
                name: "MovementCorridorRadius",
                name_hash: 1392343010,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathFollowingComponentBaseData, movement_corridor_radius),
            },
        ],
    }),
    array_type: Some(PATHFOLLOWINGCOMPONENTBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathFollowingComponentBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFOLLOWINGCOMPONENTBASEDATA_TYPE_INFO
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


pub static PATHFOLLOWINGCOMPONENTBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathFollowingComponentBaseData-Array",
    name_hash: 1789625063,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathFollowingComponentBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PathfindingChoice {
    #[default]
    PathfindingChoice_Off = 0,
    PathfindingChoice_OnlyToStartPoint = 1,
    PathfindingChoice_On = 2,
}

pub static PATHFINDINGCHOICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingChoice",
    name_hash: 1991735526,
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(PATHFINDINGCHOICE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PathfindingChoice {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGCHOICE_TYPE_INFO
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


pub static PATHFINDINGCHOICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingChoice-Array",
    name_hash: 2508518866,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingChoice"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingStreamEntityBaseData {
    pub _glacier_base: super::entity::EntityData,
    pub pathfinding_blobs: Option<LockedTypeObject /* PathfindingBlobAsset */>,
    pub autoload: bool,
    pub realm: super::core::Realm,
}

pub trait PathfindingStreamEntityBaseDataTrait: super::entity::EntityDataTrait {
    fn pathfinding_blobs(&self) -> &Option<LockedTypeObject /* PathfindingBlobAsset */>;
    fn pathfinding_blobs_mut(&mut self) -> &mut Option<LockedTypeObject /* PathfindingBlobAsset */>;
    fn autoload(&self) -> &bool;
    fn autoload_mut(&mut self) -> &mut bool;
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl PathfindingStreamEntityBaseDataTrait for PathfindingStreamEntityBaseData {
    fn pathfinding_blobs(&self) -> &Option<LockedTypeObject /* PathfindingBlobAsset */> {
        &self.pathfinding_blobs
    }
    fn pathfinding_blobs_mut(&mut self) -> &mut Option<LockedTypeObject /* PathfindingBlobAsset */> {
        &mut self.pathfinding_blobs
    }
    fn autoload(&self) -> &bool {
        &self.autoload
    }
    fn autoload_mut(&mut self) -> &mut bool {
        &mut self.autoload
    }
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl super::entity::EntityDataTrait for PathfindingStreamEntityBaseData {
}

impl super::entity::GameObjectDataTrait for PathfindingStreamEntityBaseData {
}

impl super::core::DataBusPeerTrait for PathfindingStreamEntityBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PathfindingStreamEntityBaseData {
}

impl super::core::DataContainerTrait for PathfindingStreamEntityBaseData {
}

pub static PATHFINDINGSTREAMENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingStreamEntityBaseData",
    name_hash: 1247166127,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingStreamEntityBaseData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingStreamEntityBaseData as Default>::default())),
            create_boxed: || Box::new(<PathfindingStreamEntityBaseData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PathfindingBlobs",
                name_hash: 59106909,
                flags: MemberInfoFlags::new(0),
                field_type: "PathfindingBlobAsset",
                rust_offset: offset_of!(PathfindingStreamEntityBaseData, pathfinding_blobs),
            },
            FieldInfoData {
                name: "Autoload",
                name_hash: 3538431020,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingStreamEntityBaseData, autoload),
            },
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PathfindingStreamEntityBaseData, realm),
            },
        ],
    }),
    array_type: Some(PATHFINDINGSTREAMENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathfindingStreamEntityBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGSTREAMENTITYBASEDATA_TYPE_INFO
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


pub static PATHFINDINGSTREAMENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingStreamEntityBaseData-Array",
    name_hash: 2495081499,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingStreamEntityBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingDebugSettings {
    pub _glacier_base: super::core::SystemSettings,
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

pub trait PathfindingDebugSettingsTrait: super::core::SystemSettingsTrait {
    fn draw_nav_mesh(&self) -> &i32;
    fn draw_nav_mesh_mut(&mut self) -> &mut i32;
    fn draw_polygon_outline(&self) -> &bool;
    fn draw_polygon_outline_mut(&mut self) -> &mut bool;
    fn draw_filled_polygons(&self) -> &bool;
    fn draw_filled_polygons_mut(&mut self) -> &mut bool;
    fn draw_connections(&self) -> &bool;
    fn draw_connections_mut(&mut self) -> &mut bool;
    fn draw_obstacles(&self) -> &bool;
    fn draw_obstacles_mut(&mut self) -> &mut bool;
    fn draw_area_normals(&self) -> &bool;
    fn draw_area_normals_mut(&mut self) -> &mut bool;
    fn draw_obstacle_flags(&self) -> &bool;
    fn draw_obstacle_flags_mut(&mut self) -> &mut bool;
    fn draw_area_penalty_mults(&self) -> &bool;
    fn draw_area_penalty_mults_mut(&mut self) -> &mut bool;
    fn draw_area_usage_flags(&self) -> &bool;
    fn draw_area_usage_flags_mut(&mut self) -> &mut bool;
    fn colorize_area_usage_flags(&self) -> &bool;
    fn colorize_area_usage_flags_mut(&mut self) -> &mut bool;
    fn draw_link_usage_distances(&self) -> &bool;
    fn draw_link_usage_distances_mut(&mut self) -> &mut bool;
    fn draw_link_usage_flags(&self) -> &bool;
    fn draw_link_usage_flags_mut(&mut self) -> &mut bool;
    fn draw_original_link_locations(&self) -> &bool;
    fn draw_original_link_locations_mut(&mut self) -> &mut bool;
    fn draw_recent_nav_probes(&self) -> &bool;
    fn draw_recent_nav_probes_mut(&mut self) -> &mut bool;
    fn draw_recent_create_poly_line_paths(&self) -> &bool;
    fn draw_recent_create_poly_line_paths_mut(&mut self) -> &mut bool;
    fn draw_mover_cylinders(&self) -> &bool;
    fn draw_mover_cylinders_mut(&mut self) -> &mut bool;
    fn draw_mover_goals(&self) -> &bool;
    fn draw_mover_goals_mut(&mut self) -> &mut bool;
    fn draw_mover_goals_reached(&self) -> &bool;
    fn draw_mover_goals_reached_mut(&mut self) -> &mut bool;
    fn draw_mover_state(&self) -> &bool;
    fn draw_mover_state_mut(&mut self) -> &mut bool;
    fn draw_mover_attractions(&self) -> &bool;
    fn draw_mover_attractions_mut(&mut self) -> &mut bool;
    fn draw_repulsors(&self) -> &bool;
    fn draw_repulsors_mut(&mut self) -> &mut bool;
    fn draw_client_motion(&self) -> &bool;
    fn draw_client_motion_mut(&mut self) -> &mut bool;
    fn draw_cur_path_section(&self) -> &bool;
    fn draw_cur_path_section_mut(&mut self) -> &mut bool;
    fn draw_follower_goals(&self) -> &bool;
    fn draw_follower_goals_mut(&mut self) -> &mut bool;
    fn draw_distance(&self) -> &f32;
    fn draw_distance_mut(&mut self) -> &mut f32;
    fn depth_test(&self) -> &bool;
    fn depth_test_mut(&mut self) -> &mut bool;
    fn draw_stats(&self) -> &bool;
    fn draw_stats_mut(&mut self) -> &mut bool;
    fn draw_memory(&self) -> &bool;
    fn draw_memory_mut(&mut self) -> &mut bool;
    fn draw_timings(&self) -> &bool;
    fn draw_timings_mut(&mut self) -> &mut bool;
    fn text_start_x(&self) -> &i32;
    fn text_start_x_mut(&mut self) -> &mut i32;
    fn text_start_y(&self) -> &i32;
    fn text_start_y_mut(&mut self) -> &mut i32;
    fn text_offset_y(&self) -> &i32;
    fn text_offset_y_mut(&mut self) -> &mut i32;
    fn replay_mode(&self) -> &PathfindingReplayMode;
    fn replay_mode_mut(&mut self) -> &mut PathfindingReplayMode;
    fn original_paths(&self) -> &bool;
    fn original_paths_mut(&mut self) -> &mut bool;
    fn random_positions(&self) -> &bool;
    fn random_positions_mut(&mut self) -> &mut bool;
    fn potential_obstacles(&self) -> &bool;
    fn potential_obstacles_mut(&mut self) -> &mut bool;
}

impl PathfindingDebugSettingsTrait for PathfindingDebugSettings {
    fn draw_nav_mesh(&self) -> &i32 {
        &self.draw_nav_mesh
    }
    fn draw_nav_mesh_mut(&mut self) -> &mut i32 {
        &mut self.draw_nav_mesh
    }
    fn draw_polygon_outline(&self) -> &bool {
        &self.draw_polygon_outline
    }
    fn draw_polygon_outline_mut(&mut self) -> &mut bool {
        &mut self.draw_polygon_outline
    }
    fn draw_filled_polygons(&self) -> &bool {
        &self.draw_filled_polygons
    }
    fn draw_filled_polygons_mut(&mut self) -> &mut bool {
        &mut self.draw_filled_polygons
    }
    fn draw_connections(&self) -> &bool {
        &self.draw_connections
    }
    fn draw_connections_mut(&mut self) -> &mut bool {
        &mut self.draw_connections
    }
    fn draw_obstacles(&self) -> &bool {
        &self.draw_obstacles
    }
    fn draw_obstacles_mut(&mut self) -> &mut bool {
        &mut self.draw_obstacles
    }
    fn draw_area_normals(&self) -> &bool {
        &self.draw_area_normals
    }
    fn draw_area_normals_mut(&mut self) -> &mut bool {
        &mut self.draw_area_normals
    }
    fn draw_obstacle_flags(&self) -> &bool {
        &self.draw_obstacle_flags
    }
    fn draw_obstacle_flags_mut(&mut self) -> &mut bool {
        &mut self.draw_obstacle_flags
    }
    fn draw_area_penalty_mults(&self) -> &bool {
        &self.draw_area_penalty_mults
    }
    fn draw_area_penalty_mults_mut(&mut self) -> &mut bool {
        &mut self.draw_area_penalty_mults
    }
    fn draw_area_usage_flags(&self) -> &bool {
        &self.draw_area_usage_flags
    }
    fn draw_area_usage_flags_mut(&mut self) -> &mut bool {
        &mut self.draw_area_usage_flags
    }
    fn colorize_area_usage_flags(&self) -> &bool {
        &self.colorize_area_usage_flags
    }
    fn colorize_area_usage_flags_mut(&mut self) -> &mut bool {
        &mut self.colorize_area_usage_flags
    }
    fn draw_link_usage_distances(&self) -> &bool {
        &self.draw_link_usage_distances
    }
    fn draw_link_usage_distances_mut(&mut self) -> &mut bool {
        &mut self.draw_link_usage_distances
    }
    fn draw_link_usage_flags(&self) -> &bool {
        &self.draw_link_usage_flags
    }
    fn draw_link_usage_flags_mut(&mut self) -> &mut bool {
        &mut self.draw_link_usage_flags
    }
    fn draw_original_link_locations(&self) -> &bool {
        &self.draw_original_link_locations
    }
    fn draw_original_link_locations_mut(&mut self) -> &mut bool {
        &mut self.draw_original_link_locations
    }
    fn draw_recent_nav_probes(&self) -> &bool {
        &self.draw_recent_nav_probes
    }
    fn draw_recent_nav_probes_mut(&mut self) -> &mut bool {
        &mut self.draw_recent_nav_probes
    }
    fn draw_recent_create_poly_line_paths(&self) -> &bool {
        &self.draw_recent_create_poly_line_paths
    }
    fn draw_recent_create_poly_line_paths_mut(&mut self) -> &mut bool {
        &mut self.draw_recent_create_poly_line_paths
    }
    fn draw_mover_cylinders(&self) -> &bool {
        &self.draw_mover_cylinders
    }
    fn draw_mover_cylinders_mut(&mut self) -> &mut bool {
        &mut self.draw_mover_cylinders
    }
    fn draw_mover_goals(&self) -> &bool {
        &self.draw_mover_goals
    }
    fn draw_mover_goals_mut(&mut self) -> &mut bool {
        &mut self.draw_mover_goals
    }
    fn draw_mover_goals_reached(&self) -> &bool {
        &self.draw_mover_goals_reached
    }
    fn draw_mover_goals_reached_mut(&mut self) -> &mut bool {
        &mut self.draw_mover_goals_reached
    }
    fn draw_mover_state(&self) -> &bool {
        &self.draw_mover_state
    }
    fn draw_mover_state_mut(&mut self) -> &mut bool {
        &mut self.draw_mover_state
    }
    fn draw_mover_attractions(&self) -> &bool {
        &self.draw_mover_attractions
    }
    fn draw_mover_attractions_mut(&mut self) -> &mut bool {
        &mut self.draw_mover_attractions
    }
    fn draw_repulsors(&self) -> &bool {
        &self.draw_repulsors
    }
    fn draw_repulsors_mut(&mut self) -> &mut bool {
        &mut self.draw_repulsors
    }
    fn draw_client_motion(&self) -> &bool {
        &self.draw_client_motion
    }
    fn draw_client_motion_mut(&mut self) -> &mut bool {
        &mut self.draw_client_motion
    }
    fn draw_cur_path_section(&self) -> &bool {
        &self.draw_cur_path_section
    }
    fn draw_cur_path_section_mut(&mut self) -> &mut bool {
        &mut self.draw_cur_path_section
    }
    fn draw_follower_goals(&self) -> &bool {
        &self.draw_follower_goals
    }
    fn draw_follower_goals_mut(&mut self) -> &mut bool {
        &mut self.draw_follower_goals
    }
    fn draw_distance(&self) -> &f32 {
        &self.draw_distance
    }
    fn draw_distance_mut(&mut self) -> &mut f32 {
        &mut self.draw_distance
    }
    fn depth_test(&self) -> &bool {
        &self.depth_test
    }
    fn depth_test_mut(&mut self) -> &mut bool {
        &mut self.depth_test
    }
    fn draw_stats(&self) -> &bool {
        &self.draw_stats
    }
    fn draw_stats_mut(&mut self) -> &mut bool {
        &mut self.draw_stats
    }
    fn draw_memory(&self) -> &bool {
        &self.draw_memory
    }
    fn draw_memory_mut(&mut self) -> &mut bool {
        &mut self.draw_memory
    }
    fn draw_timings(&self) -> &bool {
        &self.draw_timings
    }
    fn draw_timings_mut(&mut self) -> &mut bool {
        &mut self.draw_timings
    }
    fn text_start_x(&self) -> &i32 {
        &self.text_start_x
    }
    fn text_start_x_mut(&mut self) -> &mut i32 {
        &mut self.text_start_x
    }
    fn text_start_y(&self) -> &i32 {
        &self.text_start_y
    }
    fn text_start_y_mut(&mut self) -> &mut i32 {
        &mut self.text_start_y
    }
    fn text_offset_y(&self) -> &i32 {
        &self.text_offset_y
    }
    fn text_offset_y_mut(&mut self) -> &mut i32 {
        &mut self.text_offset_y
    }
    fn replay_mode(&self) -> &PathfindingReplayMode {
        &self.replay_mode
    }
    fn replay_mode_mut(&mut self) -> &mut PathfindingReplayMode {
        &mut self.replay_mode
    }
    fn original_paths(&self) -> &bool {
        &self.original_paths
    }
    fn original_paths_mut(&mut self) -> &mut bool {
        &mut self.original_paths
    }
    fn random_positions(&self) -> &bool {
        &self.random_positions
    }
    fn random_positions_mut(&mut self) -> &mut bool {
        &mut self.random_positions
    }
    fn potential_obstacles(&self) -> &bool {
        &self.potential_obstacles
    }
    fn potential_obstacles_mut(&mut self) -> &mut bool {
        &mut self.potential_obstacles
    }
}

impl super::core::SystemSettingsTrait for PathfindingDebugSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for PathfindingDebugSettings {
}

pub static PATHFINDINGDEBUGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingDebugSettings",
    name_hash: 3409887129,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingDebugSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingDebugSettings as Default>::default())),
            create_boxed: || Box::new(<PathfindingDebugSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DrawNavMesh",
                name_hash: 399582895,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_nav_mesh),
            },
            FieldInfoData {
                name: "DrawPolygonOutline",
                name_hash: 4098331465,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_polygon_outline),
            },
            FieldInfoData {
                name: "DrawFilledPolygons",
                name_hash: 297524372,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_filled_polygons),
            },
            FieldInfoData {
                name: "DrawConnections",
                name_hash: 2313499072,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_connections),
            },
            FieldInfoData {
                name: "DrawObstacles",
                name_hash: 3626078967,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_obstacles),
            },
            FieldInfoData {
                name: "DrawAreaNormals",
                name_hash: 3339751346,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_area_normals),
            },
            FieldInfoData {
                name: "DrawObstacleFlags",
                name_hash: 1619848251,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_obstacle_flags),
            },
            FieldInfoData {
                name: "DrawAreaPenaltyMults",
                name_hash: 4160520634,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_area_penalty_mults),
            },
            FieldInfoData {
                name: "DrawAreaUsageFlags",
                name_hash: 1345352424,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_area_usage_flags),
            },
            FieldInfoData {
                name: "ColorizeAreaUsageFlags",
                name_hash: 2057064867,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, colorize_area_usage_flags),
            },
            FieldInfoData {
                name: "DrawLinkUsageDistances",
                name_hash: 2571739152,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_link_usage_distances),
            },
            FieldInfoData {
                name: "DrawLinkUsageFlags",
                name_hash: 3804088831,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_link_usage_flags),
            },
            FieldInfoData {
                name: "DrawOriginalLinkLocations",
                name_hash: 1621564274,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_original_link_locations),
            },
            FieldInfoData {
                name: "DrawRecentNavProbes",
                name_hash: 1760468430,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_recent_nav_probes),
            },
            FieldInfoData {
                name: "DrawRecentCreatePolyLinePaths",
                name_hash: 3213180816,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_recent_create_poly_line_paths),
            },
            FieldInfoData {
                name: "DrawMoverCylinders",
                name_hash: 3778264951,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_cylinders),
            },
            FieldInfoData {
                name: "DrawMoverGoals",
                name_hash: 3546906768,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_goals),
            },
            FieldInfoData {
                name: "DrawMoverGoalsReached",
                name_hash: 2226153452,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_goals_reached),
            },
            FieldInfoData {
                name: "DrawMoverState",
                name_hash: 3569512113,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_state),
            },
            FieldInfoData {
                name: "DrawMoverAttractions",
                name_hash: 2486806904,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_mover_attractions),
            },
            FieldInfoData {
                name: "DrawRepulsors",
                name_hash: 1415133958,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_repulsors),
            },
            FieldInfoData {
                name: "DrawClientMotion",
                name_hash: 3060076418,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_client_motion),
            },
            FieldInfoData {
                name: "DrawCurPathSection",
                name_hash: 700170789,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_cur_path_section),
            },
            FieldInfoData {
                name: "DrawFollowerGoals",
                name_hash: 3150686453,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_follower_goals),
            },
            FieldInfoData {
                name: "DrawDistance",
                name_hash: 3007537990,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_distance),
            },
            FieldInfoData {
                name: "DepthTest",
                name_hash: 969307550,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, depth_test),
            },
            FieldInfoData {
                name: "DrawStats",
                name_hash: 2413142628,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawMemory",
                name_hash: 1651442660,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_memory),
            },
            FieldInfoData {
                name: "DrawTimings",
                name_hash: 560544582,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, draw_timings),
            },
            FieldInfoData {
                name: "TextStartX",
                name_hash: 1212530848,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PathfindingDebugSettings, text_start_x),
            },
            FieldInfoData {
                name: "TextStartY",
                name_hash: 1212530849,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PathfindingDebugSettings, text_start_y),
            },
            FieldInfoData {
                name: "TextOffsetY",
                name_hash: 3116283628,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PathfindingDebugSettings, text_offset_y),
            },
            FieldInfoData {
                name: "ReplayMode",
                name_hash: 1723041685,
                flags: MemberInfoFlags::new(0),
                field_type: "PathfindingReplayMode",
                rust_offset: offset_of!(PathfindingDebugSettings, replay_mode),
            },
            FieldInfoData {
                name: "OriginalPaths",
                name_hash: 3948559810,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, original_paths),
            },
            FieldInfoData {
                name: "RandomPositions",
                name_hash: 4000636404,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, random_positions),
            },
            FieldInfoData {
                name: "PotentialObstacles",
                name_hash: 1779192423,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingDebugSettings, potential_obstacles),
            },
        ],
    }),
    array_type: Some(PATHFINDINGDEBUGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingDebugSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGDEBUGSETTINGS_TYPE_INFO
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


pub static PATHFINDINGDEBUGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingDebugSettings-Array",
    name_hash: 615251245,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingDebugSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PathfindingReplayMode {
    #[default]
    PathfindingReplayMode_Disabled = 0,
    PathfindingReplayMode_Binary = 1,
    PathfindingReplayMode_Text = 2,
}

pub static PATHFINDINGREPLAYMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingReplayMode",
    name_hash: 2436216637,
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(PATHFINDINGREPLAYMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PathfindingReplayMode {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGREPLAYMODE_TYPE_INFO
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


pub static PATHFINDINGREPLAYMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingReplayMode-Array",
    name_hash: 1170871177,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingReplayMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingTypeAsset {
    pub _glacier_base: super::core::Asset,
    pub index: u32,
}

pub trait PathfindingTypeAssetTrait: super::core::AssetTrait {
    fn index(&self) -> &u32;
    fn index_mut(&mut self) -> &mut u32;
}

impl PathfindingTypeAssetTrait for PathfindingTypeAsset {
    fn index(&self) -> &u32 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut u32 {
        &mut self.index
    }
}

impl super::core::AssetTrait for PathfindingTypeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PathfindingTypeAsset {
}

pub static PATHFINDINGTYPEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingTypeAsset",
    name_hash: 4165431461,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingTypeAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingTypeAsset as Default>::default())),
            create_boxed: || Box::new(<PathfindingTypeAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PathfindingTypeAsset, index),
            },
        ],
    }),
    array_type: Some(PATHFINDINGTYPEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingTypeAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGTYPEASSET_TYPE_INFO
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


pub static PATHFINDINGTYPEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingTypeAsset-Array",
    name_hash: 915327249,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingTypeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LinkDat {
    pub _glacier_base: super::core::DataContainer,
    pub layer_mask: u32,
    pub link_type: NavLinkType,
    pub forward_link_usage_flags: u32,
    pub backward_link_usage_flags: u32,
    pub penalty_mult: f32,
    pub max_snap_dist: f32,
    pub may_use_dist: f32,
    pub must_use_dist: f32,
    pub stop_to_use_link: bool,
    pub user_data: Option<LockedTypeObject /* CustomPathLinkData */>,
    pub link_flow_tune: Option<LockedTypeObject /* LinkFlowTune */>,
}

pub trait LinkDatTrait: super::core::DataContainerTrait {
    fn layer_mask(&self) -> &u32;
    fn layer_mask_mut(&mut self) -> &mut u32;
    fn link_type(&self) -> &NavLinkType;
    fn link_type_mut(&mut self) -> &mut NavLinkType;
    fn forward_link_usage_flags(&self) -> &u32;
    fn forward_link_usage_flags_mut(&mut self) -> &mut u32;
    fn backward_link_usage_flags(&self) -> &u32;
    fn backward_link_usage_flags_mut(&mut self) -> &mut u32;
    fn penalty_mult(&self) -> &f32;
    fn penalty_mult_mut(&mut self) -> &mut f32;
    fn max_snap_dist(&self) -> &f32;
    fn max_snap_dist_mut(&mut self) -> &mut f32;
    fn may_use_dist(&self) -> &f32;
    fn may_use_dist_mut(&mut self) -> &mut f32;
    fn must_use_dist(&self) -> &f32;
    fn must_use_dist_mut(&mut self) -> &mut f32;
    fn stop_to_use_link(&self) -> &bool;
    fn stop_to_use_link_mut(&mut self) -> &mut bool;
    fn user_data(&self) -> &Option<LockedTypeObject /* CustomPathLinkData */>;
    fn user_data_mut(&mut self) -> &mut Option<LockedTypeObject /* CustomPathLinkData */>;
    fn link_flow_tune(&self) -> &Option<LockedTypeObject /* LinkFlowTune */>;
    fn link_flow_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* LinkFlowTune */>;
}

impl LinkDatTrait for LinkDat {
    fn layer_mask(&self) -> &u32 {
        &self.layer_mask
    }
    fn layer_mask_mut(&mut self) -> &mut u32 {
        &mut self.layer_mask
    }
    fn link_type(&self) -> &NavLinkType {
        &self.link_type
    }
    fn link_type_mut(&mut self) -> &mut NavLinkType {
        &mut self.link_type
    }
    fn forward_link_usage_flags(&self) -> &u32 {
        &self.forward_link_usage_flags
    }
    fn forward_link_usage_flags_mut(&mut self) -> &mut u32 {
        &mut self.forward_link_usage_flags
    }
    fn backward_link_usage_flags(&self) -> &u32 {
        &self.backward_link_usage_flags
    }
    fn backward_link_usage_flags_mut(&mut self) -> &mut u32 {
        &mut self.backward_link_usage_flags
    }
    fn penalty_mult(&self) -> &f32 {
        &self.penalty_mult
    }
    fn penalty_mult_mut(&mut self) -> &mut f32 {
        &mut self.penalty_mult
    }
    fn max_snap_dist(&self) -> &f32 {
        &self.max_snap_dist
    }
    fn max_snap_dist_mut(&mut self) -> &mut f32 {
        &mut self.max_snap_dist
    }
    fn may_use_dist(&self) -> &f32 {
        &self.may_use_dist
    }
    fn may_use_dist_mut(&mut self) -> &mut f32 {
        &mut self.may_use_dist
    }
    fn must_use_dist(&self) -> &f32 {
        &self.must_use_dist
    }
    fn must_use_dist_mut(&mut self) -> &mut f32 {
        &mut self.must_use_dist
    }
    fn stop_to_use_link(&self) -> &bool {
        &self.stop_to_use_link
    }
    fn stop_to_use_link_mut(&mut self) -> &mut bool {
        &mut self.stop_to_use_link
    }
    fn user_data(&self) -> &Option<LockedTypeObject /* CustomPathLinkData */> {
        &self.user_data
    }
    fn user_data_mut(&mut self) -> &mut Option<LockedTypeObject /* CustomPathLinkData */> {
        &mut self.user_data
    }
    fn link_flow_tune(&self) -> &Option<LockedTypeObject /* LinkFlowTune */> {
        &self.link_flow_tune
    }
    fn link_flow_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* LinkFlowTune */> {
        &mut self.link_flow_tune
    }
}

impl super::core::DataContainerTrait for LinkDat {
}

pub static LINKDAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkDat",
    name_hash: 994385204,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(LinkDat, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkDat as Default>::default())),
            create_boxed: || Box::new(<LinkDat as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LayerMask",
                name_hash: 3033747218,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LinkDat, layer_mask),
            },
            FieldInfoData {
                name: "LinkType",
                name_hash: 2750506845,
                flags: MemberInfoFlags::new(0),
                field_type: "NavLinkType",
                rust_offset: offset_of!(LinkDat, link_type),
            },
            FieldInfoData {
                name: "ForwardLinkUsageFlags",
                name_hash: 3012744388,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LinkDat, forward_link_usage_flags),
            },
            FieldInfoData {
                name: "BackwardLinkUsageFlags",
                name_hash: 312798420,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LinkDat, backward_link_usage_flags),
            },
            FieldInfoData {
                name: "PenaltyMult",
                name_hash: 5088702,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinkDat, penalty_mult),
            },
            FieldInfoData {
                name: "MaxSnapDist",
                name_hash: 2887793335,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinkDat, max_snap_dist),
            },
            FieldInfoData {
                name: "MayUseDist",
                name_hash: 4088976185,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinkDat, may_use_dist),
            },
            FieldInfoData {
                name: "MustUseDist",
                name_hash: 3506816851,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinkDat, must_use_dist),
            },
            FieldInfoData {
                name: "StopToUseLink",
                name_hash: 2767125189,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LinkDat, stop_to_use_link),
            },
            FieldInfoData {
                name: "UserData",
                name_hash: 307929860,
                flags: MemberInfoFlags::new(0),
                field_type: "CustomPathLinkData",
                rust_offset: offset_of!(LinkDat, user_data),
            },
            FieldInfoData {
                name: "LinkFlowTune",
                name_hash: 2461085533,
                flags: MemberInfoFlags::new(0),
                field_type: "LinkFlowTune",
                rust_offset: offset_of!(LinkDat, link_flow_tune),
            },
        ],
    }),
    array_type: Some(LINKDAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkDat {
    fn type_info(&self) -> &'static TypeInfo {
        LINKDAT_TYPE_INFO
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


pub static LINKDAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkDat-Array",
    name_hash: 2616678528,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("LinkDat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CustomPathLinkData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait CustomPathLinkDataTrait: super::core::DataContainerTrait {
}

impl CustomPathLinkDataTrait for CustomPathLinkData {
}

impl super::core::DataContainerTrait for CustomPathLinkData {
}

pub static CUSTOMPATHLINKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomPathLinkData",
    name_hash: 3399343659,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CustomPathLinkData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CustomPathLinkData as Default>::default())),
            create_boxed: || Box::new(<CustomPathLinkData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CUSTOMPATHLINKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomPathLinkData {
    fn type_info(&self) -> &'static TypeInfo {
        CUSTOMPATHLINKDATA_TYPE_INFO
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


pub static CUSTOMPATHLINKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomPathLinkData-Array",
    name_hash: 3050929567,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("CustomPathLinkData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum NavLinkType {
    #[default]
    JUMP_LINK = 0,
    CUSTOM_LINK = 1,
}

pub static NAVLINKTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavLinkType",
    name_hash: 705090116,
    flags: MemberInfoFlags::new(49429),
    module: "PathfindingShared",
    data: TypeInfoData::Enum,
    array_type: Some(NAVLINKTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NavLinkType {
    fn type_info(&self) -> &'static TypeInfo {
        NAVLINKTYPE_TYPE_INFO
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


pub static NAVLINKTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavLinkType-Array",
    name_hash: 697161456,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("NavLinkType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LinkFlowTune {
    pub _glacier_base: super::core::DataContainer,
    pub max_simultaneous: u32,
}

pub trait LinkFlowTuneTrait: super::core::DataContainerTrait {
    fn max_simultaneous(&self) -> &u32;
    fn max_simultaneous_mut(&mut self) -> &mut u32;
}

impl LinkFlowTuneTrait for LinkFlowTune {
    fn max_simultaneous(&self) -> &u32 {
        &self.max_simultaneous
    }
    fn max_simultaneous_mut(&mut self) -> &mut u32 {
        &mut self.max_simultaneous
    }
}

impl super::core::DataContainerTrait for LinkFlowTune {
}

pub static LINKFLOWTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkFlowTune",
    name_hash: 2461085533,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(LinkFlowTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkFlowTune as Default>::default())),
            create_boxed: || Box::new(<LinkFlowTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxSimultaneous",
                name_hash: 4230377864,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LinkFlowTune, max_simultaneous),
            },
        ],
    }),
    array_type: Some(LINKFLOWTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkFlowTune {
    fn type_info(&self) -> &'static TypeInfo {
        LINKFLOWTUNE_TYPE_INFO
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


pub static LINKFLOWTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkFlowTune-Array",
    name_hash: 2384391017,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("LinkFlowTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingRuntimeResourceAssetResult {
    pub _glacier_base: super::core::DataContainer,
    pub has_path_data: bool,
}

pub trait PathfindingRuntimeResourceAssetResultTrait: super::core::DataContainerTrait {
    fn has_path_data(&self) -> &bool;
    fn has_path_data_mut(&mut self) -> &mut bool;
}

impl PathfindingRuntimeResourceAssetResultTrait for PathfindingRuntimeResourceAssetResult {
    fn has_path_data(&self) -> &bool {
        &self.has_path_data
    }
    fn has_path_data_mut(&mut self) -> &mut bool {
        &mut self.has_path_data
    }
}

impl super::core::DataContainerTrait for PathfindingRuntimeResourceAssetResult {
}

pub static PATHFINDINGRUNTIMERESOURCEASSETRESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAssetResult",
    name_hash: 2128339042,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingRuntimeResourceAssetResult, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingRuntimeResourceAssetResult as Default>::default())),
            create_boxed: || Box::new(<PathfindingRuntimeResourceAssetResult as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HasPathData",
                name_hash: 222684482,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathfindingRuntimeResourceAssetResult, has_path_data),
            },
        ],
    }),
    array_type: Some(PATHFINDINGRUNTIMERESOURCEASSETRESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingRuntimeResourceAssetResult {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGRUNTIMERESOURCEASSETRESULT_TYPE_INFO
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


pub static PATHFINDINGRUNTIMERESOURCEASSETRESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAssetResult-Array",
    name_hash: 855199574,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingRuntimeResourceAssetResult"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingRuntimeResourceAsset {
    pub _glacier_base: super::core::Asset,
    pub resource: glacier_reflect::builtin::ResourceRef,
    pub blob_size: u32,
    pub chunk_sizes: Vec<u32>,
}

pub trait PathfindingRuntimeResourceAssetTrait: super::core::AssetTrait {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn blob_size(&self) -> &u32;
    fn blob_size_mut(&mut self) -> &mut u32;
    fn chunk_sizes(&self) -> &Vec<u32>;
    fn chunk_sizes_mut(&mut self) -> &mut Vec<u32>;
}

impl PathfindingRuntimeResourceAssetTrait for PathfindingRuntimeResourceAsset {
    fn resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.resource
    }
    fn resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.resource
    }
    fn blob_size(&self) -> &u32 {
        &self.blob_size
    }
    fn blob_size_mut(&mut self) -> &mut u32 {
        &mut self.blob_size
    }
    fn chunk_sizes(&self) -> &Vec<u32> {
        &self.chunk_sizes
    }
    fn chunk_sizes_mut(&mut self) -> &mut Vec<u32> {
        &mut self.chunk_sizes
    }
}

impl super::core::AssetTrait for PathfindingRuntimeResourceAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PathfindingRuntimeResourceAsset {
}

pub static PATHFINDINGRUNTIMERESOURCEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAsset",
    name_hash: 2608026603,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingRuntimeResourceAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingRuntimeResourceAsset as Default>::default())),
            create_boxed: || Box::new(<PathfindingRuntimeResourceAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Resource",
                name_hash: 74513935,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(PathfindingRuntimeResourceAsset, resource),
            },
            FieldInfoData {
                name: "BlobSize",
                name_hash: 3257493379,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PathfindingRuntimeResourceAsset, blob_size),
            },
            FieldInfoData {
                name: "ChunkSizes",
                name_hash: 1597710248,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(PathfindingRuntimeResourceAsset, chunk_sizes),
            },
        ],
    }),
    array_type: Some(PATHFINDINGRUNTIMERESOURCEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingRuntimeResourceAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGRUNTIMERESOURCEASSET_TYPE_INFO
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


pub static PATHFINDINGRUNTIMERESOURCEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResourceAsset-Array",
    name_hash: 883180511,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingRuntimeResourceAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingBlobAsset {
    pub _glacier_base: super::core::Asset,
    pub blob: PathfindingBlob,
}

pub trait PathfindingBlobAssetTrait: super::core::AssetTrait {
    fn blob(&self) -> &PathfindingBlob;
    fn blob_mut(&mut self) -> &mut PathfindingBlob;
}

impl PathfindingBlobAssetTrait for PathfindingBlobAsset {
    fn blob(&self) -> &PathfindingBlob {
        &self.blob
    }
    fn blob_mut(&mut self) -> &mut PathfindingBlob {
        &mut self.blob
    }
}

impl super::core::AssetTrait for PathfindingBlobAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PathfindingBlobAsset {
}

pub static PATHFINDINGBLOBASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlobAsset",
    name_hash: 2279042942,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingBlobAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingBlobAsset as Default>::default())),
            create_boxed: || Box::new(<PathfindingBlobAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Blob",
                name_hash: 2088817798,
                flags: MemberInfoFlags::new(0),
                field_type: "PathfindingBlob",
                rust_offset: offset_of!(PathfindingBlobAsset, blob),
            },
        ],
    }),
    array_type: Some(PATHFINDINGBLOBASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingBlobAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGBLOBASSET_TYPE_INFO
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


pub static PATHFINDINGBLOBASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlobAsset-Array",
    name_hash: 3346395210,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingBlobAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingBlob {
    pub blob_id: glacier_util::guid::Guid,
    pub blob_size: u32,
    pub chunk_sizes: Vec<u32>,
}

pub trait PathfindingBlobTrait: TypeObject {
    fn blob_id(&self) -> &glacier_util::guid::Guid;
    fn blob_id_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn blob_size(&self) -> &u32;
    fn blob_size_mut(&mut self) -> &mut u32;
    fn chunk_sizes(&self) -> &Vec<u32>;
    fn chunk_sizes_mut(&mut self) -> &mut Vec<u32>;
}

impl PathfindingBlobTrait for PathfindingBlob {
    fn blob_id(&self) -> &glacier_util::guid::Guid {
        &self.blob_id
    }
    fn blob_id_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.blob_id
    }
    fn blob_size(&self) -> &u32 {
        &self.blob_size
    }
    fn blob_size_mut(&mut self) -> &mut u32 {
        &mut self.blob_size
    }
    fn chunk_sizes(&self) -> &Vec<u32> {
        &self.chunk_sizes
    }
    fn chunk_sizes_mut(&mut self) -> &mut Vec<u32> {
        &mut self.chunk_sizes
    }
}

pub static PATHFINDINGBLOB_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlob",
    name_hash: 1042995310,
    flags: MemberInfoFlags::new(73),
    module: "PathfindingShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingBlob as Default>::default())),
            create_boxed: || Box::new(<PathfindingBlob as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BlobId",
                name_hash: 2684880523,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(PathfindingBlob, blob_id),
            },
            FieldInfoData {
                name: "BlobSize",
                name_hash: 3257493379,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PathfindingBlob, blob_size),
            },
            FieldInfoData {
                name: "ChunkSizes",
                name_hash: 1597710248,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(PathfindingBlob, chunk_sizes),
            },
        ],
    }),
    array_type: Some(PATHFINDINGBLOB_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingBlob {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGBLOB_TYPE_INFO
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


pub static PATHFINDINGBLOB_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingBlob-Array",
    name_hash: 2204193114,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingBlob"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingSystemEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub pathfinding_types_on_level: Vec<u32>,
    pub realm: super::core::Realm,
    pub resource_asset: Option<LockedTypeObject /* PathfindingRuntimeResourceAsset */>,
}

pub trait PathfindingSystemEntityDataTrait: super::entity::EntityDataTrait {
    fn pathfinding_types_on_level(&self) -> &Vec<u32>;
    fn pathfinding_types_on_level_mut(&mut self) -> &mut Vec<u32>;
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn resource_asset(&self) -> &Option<LockedTypeObject /* PathfindingRuntimeResourceAsset */>;
    fn resource_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* PathfindingRuntimeResourceAsset */>;
}

impl PathfindingSystemEntityDataTrait for PathfindingSystemEntityData {
    fn pathfinding_types_on_level(&self) -> &Vec<u32> {
        &self.pathfinding_types_on_level
    }
    fn pathfinding_types_on_level_mut(&mut self) -> &mut Vec<u32> {
        &mut self.pathfinding_types_on_level
    }
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn resource_asset(&self) -> &Option<LockedTypeObject /* PathfindingRuntimeResourceAsset */> {
        &self.resource_asset
    }
    fn resource_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* PathfindingRuntimeResourceAsset */> {
        &mut self.resource_asset
    }
}

impl super::entity::EntityDataTrait for PathfindingSystemEntityData {
}

impl super::entity::GameObjectDataTrait for PathfindingSystemEntityData {
}

impl super::core::DataBusPeerTrait for PathfindingSystemEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PathfindingSystemEntityData {
}

impl super::core::DataContainerTrait for PathfindingSystemEntityData {
}

pub static PATHFINDINGSYSTEMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingSystemEntityData",
    name_hash: 4059939011,
    flags: MemberInfoFlags::new(101),
    module: "PathfindingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingSystemEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingSystemEntityData as Default>::default())),
            create_boxed: || Box::new(<PathfindingSystemEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PathfindingTypesOnLevel",
                name_hash: 2264269169,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(PathfindingSystemEntityData, pathfinding_types_on_level),
            },
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PathfindingSystemEntityData, realm),
            },
            FieldInfoData {
                name: "ResourceAsset",
                name_hash: 3875065919,
                flags: MemberInfoFlags::new(0),
                field_type: "PathfindingRuntimeResourceAsset",
                rust_offset: offset_of!(PathfindingSystemEntityData, resource_asset),
            },
        ],
    }),
    array_type: Some(PATHFINDINGSYSTEMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathfindingSystemEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGSYSTEMENTITYDATA_TYPE_INFO
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


pub static PATHFINDINGSYSTEMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingSystemEntityData-Array",
    name_hash: 3595371511,
    flags: MemberInfoFlags::new(145),
    module: "PathfindingShared",
    data: TypeInfoData::Array("PathfindingSystemEntityData"),
    array_type: None,
    alignment: 8,
};


