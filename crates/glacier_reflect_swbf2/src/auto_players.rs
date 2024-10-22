use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_auto_players_types(registry: &mut TypeRegistry) {
    registry.register_type(AUTOPLAYERACTIONOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(AUTOPLAYERACTIONOBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERLISTENERENTITY_TYPE_INFO);
    registry.register_type(AUTOPLAYERLISTENERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYEROBJECTIVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERMOVEMODE_TYPE_INFO);
    registry.register_type(AUTOPLAYERMOVEMODE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERMOVEOBJECTIVEENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERMOVEOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERATTACKOBJECTIVEENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERATTACKOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERACTIONOBJECTIVEENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERACTIONOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGSENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGSCHOICE_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGSCHOICE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGSKIND_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGSKIND_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERMANAGERENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERMANAGERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERENTITYDATA_TYPE_INFO);
    registry.register_type(AUTOPLAYERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGS_TYPE_INFO);
    registry.register_type(AUTOPLAYERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAUTOPLAYERSETTINGSENTITY_TYPE_INFO);
    registry.register_type(SERVERAUTOPLAYERSETTINGSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAUTOPLAYERMANAGERENTITY_TYPE_INFO);
    registry.register_type(SERVERAUTOPLAYERMANAGERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO);
    registry.register_type(AUTOPLAYEROBJECTIVEENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERMOVEOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(AUTOPLAYERMOVEOBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERINTERACTOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(AUTOPLAYERINTERACTOBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERFOLLOWOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(AUTOPLAYERFOLLOWOBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERDEFENDOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(AUTOPLAYERDEFENDOBJECTIVEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(AUTOPLAYERATTACKOBJECTIVEENTITY_TYPE_INFO);
    registry.register_type(AUTOPLAYERATTACKOBJECTIVEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct AutoPlayerActionObjectiveEntity {
    pub _glacier_base: AutoPlayerObjectiveEntityBase,
}

pub trait AutoPlayerActionObjectiveEntityTrait: AutoPlayerObjectiveEntityBaseTrait {
}

impl AutoPlayerActionObjectiveEntityTrait for AutoPlayerActionObjectiveEntity {
}

impl AutoPlayerObjectiveEntityBaseTrait for AutoPlayerActionObjectiveEntity {
}

impl super::entity::EntityTrait for AutoPlayerActionObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerActionObjectiveEntity {
}

pub static AUTOPLAYERACTIONOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerActionObjectiveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERACTIONOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerActionObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERACTIONOBJECTIVEENTITY_TYPE_INFO
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


pub static AUTOPLAYERACTIONOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerActionObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerListenerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait AutoPlayerListenerEntityTrait: super::entity::EntityTrait {
}

impl AutoPlayerListenerEntityTrait for AutoPlayerListenerEntity {
}

impl super::entity::EntityTrait for AutoPlayerListenerEntity {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerListenerEntity {
}

pub static AUTOPLAYERLISTENERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerListenerEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerListenerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERLISTENERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerListenerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERLISTENERENTITY_TYPE_INFO
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


pub static AUTOPLAYERLISTENERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerListenerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerListenerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerObjectiveEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub players: super::dice_shooter_shared::QueryEntityResult,
    pub jesus_mode: bool,
    pub unlimited_ammo: bool,
    pub allow_teleport: bool,
    pub use_objective_teleport: bool,
    pub use_stuck_escape_procedure: bool,
    pub use_navmesh: bool,
    pub allow_objective_move_outside_combat_area: bool,
    pub allow_secondary_objectives: bool,
    pub disable_zoom_when_aiming: bool,
    pub move_mode: AutoPlayerMoveMode,
    pub move_mode_override_int: i32,
    pub debug_name: String,
    pub time_threshold: i32,
    pub clamp_vertical_nav_pos_search_meters: f32,
    pub restricted_area_sphere_centre: super::core::Vec3,
    pub restricted_area_sphere_radius: f32,
}

pub trait AutoPlayerObjectiveEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult;
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult;
    fn jesus_mode(&self) -> &bool;
    fn jesus_mode_mut(&mut self) -> &mut bool;
    fn unlimited_ammo(&self) -> &bool;
    fn unlimited_ammo_mut(&mut self) -> &mut bool;
    fn allow_teleport(&self) -> &bool;
    fn allow_teleport_mut(&mut self) -> &mut bool;
    fn use_objective_teleport(&self) -> &bool;
    fn use_objective_teleport_mut(&mut self) -> &mut bool;
    fn use_stuck_escape_procedure(&self) -> &bool;
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool;
    fn use_navmesh(&self) -> &bool;
    fn use_navmesh_mut(&mut self) -> &mut bool;
    fn allow_objective_move_outside_combat_area(&self) -> &bool;
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool;
    fn allow_secondary_objectives(&self) -> &bool;
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool;
    fn disable_zoom_when_aiming(&self) -> &bool;
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool;
    fn move_mode(&self) -> &AutoPlayerMoveMode;
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode;
    fn move_mode_override_int(&self) -> &i32;
    fn move_mode_override_int_mut(&mut self) -> &mut i32;
    fn debug_name(&self) -> &String;
    fn debug_name_mut(&mut self) -> &mut String;
    fn time_threshold(&self) -> &i32;
    fn time_threshold_mut(&mut self) -> &mut i32;
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32;
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32;
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3;
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3;
    fn restricted_area_sphere_radius(&self) -> &f32;
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32;
}

impl AutoPlayerObjectiveEntityDataTrait for AutoPlayerObjectiveEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        &self.players
    }
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        &mut self.players
    }
    fn jesus_mode(&self) -> &bool {
        &self.jesus_mode
    }
    fn jesus_mode_mut(&mut self) -> &mut bool {
        &mut self.jesus_mode
    }
    fn unlimited_ammo(&self) -> &bool {
        &self.unlimited_ammo
    }
    fn unlimited_ammo_mut(&mut self) -> &mut bool {
        &mut self.unlimited_ammo
    }
    fn allow_teleport(&self) -> &bool {
        &self.allow_teleport
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        &mut self.allow_teleport
    }
    fn use_objective_teleport(&self) -> &bool {
        &self.use_objective_teleport
    }
    fn use_objective_teleport_mut(&mut self) -> &mut bool {
        &mut self.use_objective_teleport
    }
    fn use_stuck_escape_procedure(&self) -> &bool {
        &self.use_stuck_escape_procedure
    }
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        &mut self.use_stuck_escape_procedure
    }
    fn use_navmesh(&self) -> &bool {
        &self.use_navmesh
    }
    fn use_navmesh_mut(&mut self) -> &mut bool {
        &mut self.use_navmesh
    }
    fn allow_objective_move_outside_combat_area(&self) -> &bool {
        &self.allow_objective_move_outside_combat_area
    }
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool {
        &mut self.allow_objective_move_outside_combat_area
    }
    fn allow_secondary_objectives(&self) -> &bool {
        &self.allow_secondary_objectives
    }
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool {
        &mut self.allow_secondary_objectives
    }
    fn disable_zoom_when_aiming(&self) -> &bool {
        &self.disable_zoom_when_aiming
    }
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool {
        &mut self.disable_zoom_when_aiming
    }
    fn move_mode(&self) -> &AutoPlayerMoveMode {
        &self.move_mode
    }
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode {
        &mut self.move_mode
    }
    fn move_mode_override_int(&self) -> &i32 {
        &self.move_mode_override_int
    }
    fn move_mode_override_int_mut(&mut self) -> &mut i32 {
        &mut self.move_mode_override_int
    }
    fn debug_name(&self) -> &String {
        &self.debug_name
    }
    fn debug_name_mut(&mut self) -> &mut String {
        &mut self.debug_name
    }
    fn time_threshold(&self) -> &i32 {
        &self.time_threshold
    }
    fn time_threshold_mut(&mut self) -> &mut i32 {
        &mut self.time_threshold
    }
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32 {
        &self.clamp_vertical_nav_pos_search_meters
    }
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32 {
        &mut self.clamp_vertical_nav_pos_search_meters
    }
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3 {
        &self.restricted_area_sphere_centre
    }
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.restricted_area_sphere_centre
    }
    fn restricted_area_sphere_radius(&self) -> &f32 {
        &self.restricted_area_sphere_radius
    }
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32 {
        &mut self.restricted_area_sphere_radius
    }
}

impl super::entity::EntityDataTrait for AutoPlayerObjectiveEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerObjectiveEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerObjectiveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerObjectiveEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerObjectiveEntityData {
}

pub static AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerObjectiveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, realm),
            },
            FieldInfoData {
                name: "Players",
                flags: MemberInfoFlags::new(0),
                field_type: "QueryEntityResult",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, players),
            },
            FieldInfoData {
                name: "JesusMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, jesus_mode),
            },
            FieldInfoData {
                name: "UnlimitedAmmo",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, unlimited_ammo),
            },
            FieldInfoData {
                name: "AllowTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, allow_teleport),
            },
            FieldInfoData {
                name: "UseObjectiveTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, use_objective_teleport),
            },
            FieldInfoData {
                name: "UseStuckEscapeProcedure",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, use_stuck_escape_procedure),
            },
            FieldInfoData {
                name: "UseNavmesh",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, use_navmesh),
            },
            FieldInfoData {
                name: "AllowObjectiveMoveOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, allow_objective_move_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowSecondaryObjectives",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, allow_secondary_objectives),
            },
            FieldInfoData {
                name: "DisableZoomWhenAiming",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, disable_zoom_when_aiming),
            },
            FieldInfoData {
                name: "MoveMode",
                flags: MemberInfoFlags::new(0),
                field_type: "AutoPlayerMoveMode",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, move_mode),
            },
            FieldInfoData {
                name: "MoveModeOverrideInt",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, move_mode_override_int),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, debug_name),
            },
            FieldInfoData {
                name: "TimeThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, time_threshold),
            },
            FieldInfoData {
                name: "ClampVerticalNavPosSearchMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, clamp_vertical_nav_pos_search_meters),
            },
            FieldInfoData {
                name: "RestrictedAreaSphereCentre",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, restricted_area_sphere_centre),
            },
            FieldInfoData {
                name: "RestrictedAreaSphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, restricted_area_sphere_radius),
            },
        ],
    }),
    array_type: Some(AUTOPLAYEROBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerObjectiveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYEROBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerObjectiveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AutoPlayerMoveMode {
    #[default]
    AutoPlayerMoveMode_Aggressive = 0,
    AutoPlayerMoveMode_Defensive = 1,
    AutoPlayerMoveMode_Passive = 2,
}

pub static AUTOPLAYERMOVEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveMode",
    flags: MemberInfoFlags::new(49429),
    module: "AutoPlayers",
    data: TypeInfoData::Enum,
    array_type: Some(AUTOPLAYERMOVEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPlayerMoveMode {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERMOVEMODE_TYPE_INFO
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


pub static AUTOPLAYERMOVEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerMoveMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerMoveObjectiveEntityData {
    pub _glacier_base: AutoPlayerObjectiveEntityData,
    pub target_position: super::core::Vec3,
    pub move_area_radius: f32,
    pub is_target_reached_as_soon_as_entering_target_area: bool,
    pub use_random_path_spread: bool,
}

pub trait AutoPlayerMoveObjectiveEntityDataTrait: AutoPlayerObjectiveEntityDataTrait {
    fn target_position(&self) -> &super::core::Vec3;
    fn target_position_mut(&mut self) -> &mut super::core::Vec3;
    fn move_area_radius(&self) -> &f32;
    fn move_area_radius_mut(&mut self) -> &mut f32;
    fn is_target_reached_as_soon_as_entering_target_area(&self) -> &bool;
    fn is_target_reached_as_soon_as_entering_target_area_mut(&mut self) -> &mut bool;
    fn use_random_path_spread(&self) -> &bool;
    fn use_random_path_spread_mut(&mut self) -> &mut bool;
}

impl AutoPlayerMoveObjectiveEntityDataTrait for AutoPlayerMoveObjectiveEntityData {
    fn target_position(&self) -> &super::core::Vec3 {
        &self.target_position
    }
    fn target_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.target_position
    }
    fn move_area_radius(&self) -> &f32 {
        &self.move_area_radius
    }
    fn move_area_radius_mut(&mut self) -> &mut f32 {
        &mut self.move_area_radius
    }
    fn is_target_reached_as_soon_as_entering_target_area(&self) -> &bool {
        &self.is_target_reached_as_soon_as_entering_target_area
    }
    fn is_target_reached_as_soon_as_entering_target_area_mut(&mut self) -> &mut bool {
        &mut self.is_target_reached_as_soon_as_entering_target_area
    }
    fn use_random_path_spread(&self) -> &bool {
        &self.use_random_path_spread
    }
    fn use_random_path_spread_mut(&mut self) -> &mut bool {
        &mut self.use_random_path_spread
    }
}

impl AutoPlayerObjectiveEntityDataTrait for AutoPlayerMoveObjectiveEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players()
    }
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players_mut()
    }
    fn jesus_mode(&self) -> &bool {
        self._glacier_base.jesus_mode()
    }
    fn jesus_mode_mut(&mut self) -> &mut bool {
        self._glacier_base.jesus_mode_mut()
    }
    fn unlimited_ammo(&self) -> &bool {
        self._glacier_base.unlimited_ammo()
    }
    fn unlimited_ammo_mut(&mut self) -> &mut bool {
        self._glacier_base.unlimited_ammo_mut()
    }
    fn allow_teleport(&self) -> &bool {
        self._glacier_base.allow_teleport()
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_teleport_mut()
    }
    fn use_objective_teleport(&self) -> &bool {
        self._glacier_base.use_objective_teleport()
    }
    fn use_objective_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.use_objective_teleport_mut()
    }
    fn use_stuck_escape_procedure(&self) -> &bool {
        self._glacier_base.use_stuck_escape_procedure()
    }
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        self._glacier_base.use_stuck_escape_procedure_mut()
    }
    fn use_navmesh(&self) -> &bool {
        self._glacier_base.use_navmesh()
    }
    fn use_navmesh_mut(&mut self) -> &mut bool {
        self._glacier_base.use_navmesh_mut()
    }
    fn allow_objective_move_outside_combat_area(&self) -> &bool {
        self._glacier_base.allow_objective_move_outside_combat_area()
    }
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_objective_move_outside_combat_area_mut()
    }
    fn allow_secondary_objectives(&self) -> &bool {
        self._glacier_base.allow_secondary_objectives()
    }
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_secondary_objectives_mut()
    }
    fn disable_zoom_when_aiming(&self) -> &bool {
        self._glacier_base.disable_zoom_when_aiming()
    }
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool {
        self._glacier_base.disable_zoom_when_aiming_mut()
    }
    fn move_mode(&self) -> &AutoPlayerMoveMode {
        self._glacier_base.move_mode()
    }
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode {
        self._glacier_base.move_mode_mut()
    }
    fn move_mode_override_int(&self) -> &i32 {
        self._glacier_base.move_mode_override_int()
    }
    fn move_mode_override_int_mut(&mut self) -> &mut i32 {
        self._glacier_base.move_mode_override_int_mut()
    }
    fn debug_name(&self) -> &String {
        self._glacier_base.debug_name()
    }
    fn debug_name_mut(&mut self) -> &mut String {
        self._glacier_base.debug_name_mut()
    }
    fn time_threshold(&self) -> &i32 {
        self._glacier_base.time_threshold()
    }
    fn time_threshold_mut(&mut self) -> &mut i32 {
        self._glacier_base.time_threshold_mut()
    }
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters()
    }
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters_mut()
    }
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre()
    }
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre_mut()
    }
    fn restricted_area_sphere_radius(&self) -> &f32 {
        self._glacier_base.restricted_area_sphere_radius()
    }
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.restricted_area_sphere_radius_mut()
    }
}

impl super::entity::EntityDataTrait for AutoPlayerMoveObjectiveEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerMoveObjectiveEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerMoveObjectiveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerMoveObjectiveEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerMoveObjectiveEntityData {
}

pub static AUTOPLAYERMOVEOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerMoveObjectiveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, target_position),
            },
            FieldInfoData {
                name: "MoveAreaRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, move_area_radius),
            },
            FieldInfoData {
                name: "IsTargetReachedAsSoonAsEnteringTargetArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, is_target_reached_as_soon_as_entering_target_area),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERMOVEOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerMoveObjectiveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERMOVEOBJECTIVEENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERMOVEOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerMoveObjectiveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerInteractObjectiveEntityData {
    pub _glacier_base: AutoPlayerObjectiveEntityData,
    pub interaction_entity_position: super::core::Vec3,
    pub interaction_position: super::core::Vec3,
    pub interact_input_action: i32,
    pub find_interact_target_position: bool,
    pub interaction_start_radius: f32,
    pub hold_to_interact_time: f32,
    pub primary_interaction_search_radius: f32,
    pub use_random_path_spread: bool,
}

pub trait AutoPlayerInteractObjectiveEntityDataTrait: AutoPlayerObjectiveEntityDataTrait {
    fn interaction_entity_position(&self) -> &super::core::Vec3;
    fn interaction_entity_position_mut(&mut self) -> &mut super::core::Vec3;
    fn interaction_position(&self) -> &super::core::Vec3;
    fn interaction_position_mut(&mut self) -> &mut super::core::Vec3;
    fn interact_input_action(&self) -> &i32;
    fn interact_input_action_mut(&mut self) -> &mut i32;
    fn find_interact_target_position(&self) -> &bool;
    fn find_interact_target_position_mut(&mut self) -> &mut bool;
    fn interaction_start_radius(&self) -> &f32;
    fn interaction_start_radius_mut(&mut self) -> &mut f32;
    fn hold_to_interact_time(&self) -> &f32;
    fn hold_to_interact_time_mut(&mut self) -> &mut f32;
    fn primary_interaction_search_radius(&self) -> &f32;
    fn primary_interaction_search_radius_mut(&mut self) -> &mut f32;
    fn use_random_path_spread(&self) -> &bool;
    fn use_random_path_spread_mut(&mut self) -> &mut bool;
}

impl AutoPlayerInteractObjectiveEntityDataTrait for AutoPlayerInteractObjectiveEntityData {
    fn interaction_entity_position(&self) -> &super::core::Vec3 {
        &self.interaction_entity_position
    }
    fn interaction_entity_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.interaction_entity_position
    }
    fn interaction_position(&self) -> &super::core::Vec3 {
        &self.interaction_position
    }
    fn interaction_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.interaction_position
    }
    fn interact_input_action(&self) -> &i32 {
        &self.interact_input_action
    }
    fn interact_input_action_mut(&mut self) -> &mut i32 {
        &mut self.interact_input_action
    }
    fn find_interact_target_position(&self) -> &bool {
        &self.find_interact_target_position
    }
    fn find_interact_target_position_mut(&mut self) -> &mut bool {
        &mut self.find_interact_target_position
    }
    fn interaction_start_radius(&self) -> &f32 {
        &self.interaction_start_radius
    }
    fn interaction_start_radius_mut(&mut self) -> &mut f32 {
        &mut self.interaction_start_radius
    }
    fn hold_to_interact_time(&self) -> &f32 {
        &self.hold_to_interact_time
    }
    fn hold_to_interact_time_mut(&mut self) -> &mut f32 {
        &mut self.hold_to_interact_time
    }
    fn primary_interaction_search_radius(&self) -> &f32 {
        &self.primary_interaction_search_radius
    }
    fn primary_interaction_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.primary_interaction_search_radius
    }
    fn use_random_path_spread(&self) -> &bool {
        &self.use_random_path_spread
    }
    fn use_random_path_spread_mut(&mut self) -> &mut bool {
        &mut self.use_random_path_spread
    }
}

impl AutoPlayerObjectiveEntityDataTrait for AutoPlayerInteractObjectiveEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players()
    }
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players_mut()
    }
    fn jesus_mode(&self) -> &bool {
        self._glacier_base.jesus_mode()
    }
    fn jesus_mode_mut(&mut self) -> &mut bool {
        self._glacier_base.jesus_mode_mut()
    }
    fn unlimited_ammo(&self) -> &bool {
        self._glacier_base.unlimited_ammo()
    }
    fn unlimited_ammo_mut(&mut self) -> &mut bool {
        self._glacier_base.unlimited_ammo_mut()
    }
    fn allow_teleport(&self) -> &bool {
        self._glacier_base.allow_teleport()
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_teleport_mut()
    }
    fn use_objective_teleport(&self) -> &bool {
        self._glacier_base.use_objective_teleport()
    }
    fn use_objective_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.use_objective_teleport_mut()
    }
    fn use_stuck_escape_procedure(&self) -> &bool {
        self._glacier_base.use_stuck_escape_procedure()
    }
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        self._glacier_base.use_stuck_escape_procedure_mut()
    }
    fn use_navmesh(&self) -> &bool {
        self._glacier_base.use_navmesh()
    }
    fn use_navmesh_mut(&mut self) -> &mut bool {
        self._glacier_base.use_navmesh_mut()
    }
    fn allow_objective_move_outside_combat_area(&self) -> &bool {
        self._glacier_base.allow_objective_move_outside_combat_area()
    }
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_objective_move_outside_combat_area_mut()
    }
    fn allow_secondary_objectives(&self) -> &bool {
        self._glacier_base.allow_secondary_objectives()
    }
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_secondary_objectives_mut()
    }
    fn disable_zoom_when_aiming(&self) -> &bool {
        self._glacier_base.disable_zoom_when_aiming()
    }
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool {
        self._glacier_base.disable_zoom_when_aiming_mut()
    }
    fn move_mode(&self) -> &AutoPlayerMoveMode {
        self._glacier_base.move_mode()
    }
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode {
        self._glacier_base.move_mode_mut()
    }
    fn move_mode_override_int(&self) -> &i32 {
        self._glacier_base.move_mode_override_int()
    }
    fn move_mode_override_int_mut(&mut self) -> &mut i32 {
        self._glacier_base.move_mode_override_int_mut()
    }
    fn debug_name(&self) -> &String {
        self._glacier_base.debug_name()
    }
    fn debug_name_mut(&mut self) -> &mut String {
        self._glacier_base.debug_name_mut()
    }
    fn time_threshold(&self) -> &i32 {
        self._glacier_base.time_threshold()
    }
    fn time_threshold_mut(&mut self) -> &mut i32 {
        self._glacier_base.time_threshold_mut()
    }
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters()
    }
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters_mut()
    }
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre()
    }
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre_mut()
    }
    fn restricted_area_sphere_radius(&self) -> &f32 {
        self._glacier_base.restricted_area_sphere_radius()
    }
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.restricted_area_sphere_radius_mut()
    }
}

impl super::entity::EntityDataTrait for AutoPlayerInteractObjectiveEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerInteractObjectiveEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerInteractObjectiveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerInteractObjectiveEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerInteractObjectiveEntityData {
}

pub static AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerInteractObjectiveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InteractionEntityPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interaction_entity_position),
            },
            FieldInfoData {
                name: "InteractionPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interaction_position),
            },
            FieldInfoData {
                name: "InteractInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interact_input_action),
            },
            FieldInfoData {
                name: "FindInteractTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, find_interact_target_position),
            },
            FieldInfoData {
                name: "InteractionStartRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interaction_start_radius),
            },
            FieldInfoData {
                name: "HoldToInteractTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, hold_to_interact_time),
            },
            FieldInfoData {
                name: "PrimaryInteractionSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, primary_interaction_search_radius),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerInteractObjectiveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerInteractObjectiveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerFollowObjectiveEntityData {
    pub _glacier_base: AutoPlayerObjectiveEntityData,
    pub follow_target_player: super::dice_shooter_shared::QueryEntityResult,
    pub follow_position_radius: f32,
    pub use_bread_crumbs_path_following: bool,
    pub follow_timeout_seconds: f32,
    pub follow_until_position: super::core::Vec3,
    pub follow_until_position_tolerance_meters: f32,
    pub actively_patrol_follow_player_position: bool,
}

pub trait AutoPlayerFollowObjectiveEntityDataTrait: AutoPlayerObjectiveEntityDataTrait {
    fn follow_target_player(&self) -> &super::dice_shooter_shared::QueryEntityResult;
    fn follow_target_player_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult;
    fn follow_position_radius(&self) -> &f32;
    fn follow_position_radius_mut(&mut self) -> &mut f32;
    fn use_bread_crumbs_path_following(&self) -> &bool;
    fn use_bread_crumbs_path_following_mut(&mut self) -> &mut bool;
    fn follow_timeout_seconds(&self) -> &f32;
    fn follow_timeout_seconds_mut(&mut self) -> &mut f32;
    fn follow_until_position(&self) -> &super::core::Vec3;
    fn follow_until_position_mut(&mut self) -> &mut super::core::Vec3;
    fn follow_until_position_tolerance_meters(&self) -> &f32;
    fn follow_until_position_tolerance_meters_mut(&mut self) -> &mut f32;
    fn actively_patrol_follow_player_position(&self) -> &bool;
    fn actively_patrol_follow_player_position_mut(&mut self) -> &mut bool;
}

impl AutoPlayerFollowObjectiveEntityDataTrait for AutoPlayerFollowObjectiveEntityData {
    fn follow_target_player(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        &self.follow_target_player
    }
    fn follow_target_player_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        &mut self.follow_target_player
    }
    fn follow_position_radius(&self) -> &f32 {
        &self.follow_position_radius
    }
    fn follow_position_radius_mut(&mut self) -> &mut f32 {
        &mut self.follow_position_radius
    }
    fn use_bread_crumbs_path_following(&self) -> &bool {
        &self.use_bread_crumbs_path_following
    }
    fn use_bread_crumbs_path_following_mut(&mut self) -> &mut bool {
        &mut self.use_bread_crumbs_path_following
    }
    fn follow_timeout_seconds(&self) -> &f32 {
        &self.follow_timeout_seconds
    }
    fn follow_timeout_seconds_mut(&mut self) -> &mut f32 {
        &mut self.follow_timeout_seconds
    }
    fn follow_until_position(&self) -> &super::core::Vec3 {
        &self.follow_until_position
    }
    fn follow_until_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.follow_until_position
    }
    fn follow_until_position_tolerance_meters(&self) -> &f32 {
        &self.follow_until_position_tolerance_meters
    }
    fn follow_until_position_tolerance_meters_mut(&mut self) -> &mut f32 {
        &mut self.follow_until_position_tolerance_meters
    }
    fn actively_patrol_follow_player_position(&self) -> &bool {
        &self.actively_patrol_follow_player_position
    }
    fn actively_patrol_follow_player_position_mut(&mut self) -> &mut bool {
        &mut self.actively_patrol_follow_player_position
    }
}

impl AutoPlayerObjectiveEntityDataTrait for AutoPlayerFollowObjectiveEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players()
    }
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players_mut()
    }
    fn jesus_mode(&self) -> &bool {
        self._glacier_base.jesus_mode()
    }
    fn jesus_mode_mut(&mut self) -> &mut bool {
        self._glacier_base.jesus_mode_mut()
    }
    fn unlimited_ammo(&self) -> &bool {
        self._glacier_base.unlimited_ammo()
    }
    fn unlimited_ammo_mut(&mut self) -> &mut bool {
        self._glacier_base.unlimited_ammo_mut()
    }
    fn allow_teleport(&self) -> &bool {
        self._glacier_base.allow_teleport()
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_teleport_mut()
    }
    fn use_objective_teleport(&self) -> &bool {
        self._glacier_base.use_objective_teleport()
    }
    fn use_objective_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.use_objective_teleport_mut()
    }
    fn use_stuck_escape_procedure(&self) -> &bool {
        self._glacier_base.use_stuck_escape_procedure()
    }
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        self._glacier_base.use_stuck_escape_procedure_mut()
    }
    fn use_navmesh(&self) -> &bool {
        self._glacier_base.use_navmesh()
    }
    fn use_navmesh_mut(&mut self) -> &mut bool {
        self._glacier_base.use_navmesh_mut()
    }
    fn allow_objective_move_outside_combat_area(&self) -> &bool {
        self._glacier_base.allow_objective_move_outside_combat_area()
    }
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_objective_move_outside_combat_area_mut()
    }
    fn allow_secondary_objectives(&self) -> &bool {
        self._glacier_base.allow_secondary_objectives()
    }
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_secondary_objectives_mut()
    }
    fn disable_zoom_when_aiming(&self) -> &bool {
        self._glacier_base.disable_zoom_when_aiming()
    }
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool {
        self._glacier_base.disable_zoom_when_aiming_mut()
    }
    fn move_mode(&self) -> &AutoPlayerMoveMode {
        self._glacier_base.move_mode()
    }
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode {
        self._glacier_base.move_mode_mut()
    }
    fn move_mode_override_int(&self) -> &i32 {
        self._glacier_base.move_mode_override_int()
    }
    fn move_mode_override_int_mut(&mut self) -> &mut i32 {
        self._glacier_base.move_mode_override_int_mut()
    }
    fn debug_name(&self) -> &String {
        self._glacier_base.debug_name()
    }
    fn debug_name_mut(&mut self) -> &mut String {
        self._glacier_base.debug_name_mut()
    }
    fn time_threshold(&self) -> &i32 {
        self._glacier_base.time_threshold()
    }
    fn time_threshold_mut(&mut self) -> &mut i32 {
        self._glacier_base.time_threshold_mut()
    }
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters()
    }
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters_mut()
    }
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre()
    }
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre_mut()
    }
    fn restricted_area_sphere_radius(&self) -> &f32 {
        self._glacier_base.restricted_area_sphere_radius()
    }
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.restricted_area_sphere_radius_mut()
    }
}

impl super::entity::EntityDataTrait for AutoPlayerFollowObjectiveEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerFollowObjectiveEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerFollowObjectiveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerFollowObjectiveEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerFollowObjectiveEntityData {
}

pub static AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerFollowObjectiveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FollowTargetPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: "QueryEntityResult",
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_target_player),
            },
            FieldInfoData {
                name: "FollowPositionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_position_radius),
            },
            FieldInfoData {
                name: "UseBreadCrumbsPathFollowing",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, use_bread_crumbs_path_following),
            },
            FieldInfoData {
                name: "FollowTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_timeout_seconds),
            },
            FieldInfoData {
                name: "FollowUntilPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_until_position),
            },
            FieldInfoData {
                name: "FollowUntilPositionToleranceMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_until_position_tolerance_meters),
            },
            FieldInfoData {
                name: "ActivelyPatrolFollowPlayerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, actively_patrol_follow_player_position),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerFollowObjectiveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerFollowObjectiveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerDefendObjectiveEntityData {
    pub _glacier_base: AutoPlayerObjectiveEntityData,
    pub target_position: super::core::Vec3,
    pub defend_area_radius: f32,
    pub use_random_path_spread: bool,
    pub defend_current_position: bool,
    pub is_target_reached_as_soon_as_entering_target_area: bool,
}

pub trait AutoPlayerDefendObjectiveEntityDataTrait: AutoPlayerObjectiveEntityDataTrait {
    fn target_position(&self) -> &super::core::Vec3;
    fn target_position_mut(&mut self) -> &mut super::core::Vec3;
    fn defend_area_radius(&self) -> &f32;
    fn defend_area_radius_mut(&mut self) -> &mut f32;
    fn use_random_path_spread(&self) -> &bool;
    fn use_random_path_spread_mut(&mut self) -> &mut bool;
    fn defend_current_position(&self) -> &bool;
    fn defend_current_position_mut(&mut self) -> &mut bool;
    fn is_target_reached_as_soon_as_entering_target_area(&self) -> &bool;
    fn is_target_reached_as_soon_as_entering_target_area_mut(&mut self) -> &mut bool;
}

impl AutoPlayerDefendObjectiveEntityDataTrait for AutoPlayerDefendObjectiveEntityData {
    fn target_position(&self) -> &super::core::Vec3 {
        &self.target_position
    }
    fn target_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.target_position
    }
    fn defend_area_radius(&self) -> &f32 {
        &self.defend_area_radius
    }
    fn defend_area_radius_mut(&mut self) -> &mut f32 {
        &mut self.defend_area_radius
    }
    fn use_random_path_spread(&self) -> &bool {
        &self.use_random_path_spread
    }
    fn use_random_path_spread_mut(&mut self) -> &mut bool {
        &mut self.use_random_path_spread
    }
    fn defend_current_position(&self) -> &bool {
        &self.defend_current_position
    }
    fn defend_current_position_mut(&mut self) -> &mut bool {
        &mut self.defend_current_position
    }
    fn is_target_reached_as_soon_as_entering_target_area(&self) -> &bool {
        &self.is_target_reached_as_soon_as_entering_target_area
    }
    fn is_target_reached_as_soon_as_entering_target_area_mut(&mut self) -> &mut bool {
        &mut self.is_target_reached_as_soon_as_entering_target_area
    }
}

impl AutoPlayerObjectiveEntityDataTrait for AutoPlayerDefendObjectiveEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players()
    }
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players_mut()
    }
    fn jesus_mode(&self) -> &bool {
        self._glacier_base.jesus_mode()
    }
    fn jesus_mode_mut(&mut self) -> &mut bool {
        self._glacier_base.jesus_mode_mut()
    }
    fn unlimited_ammo(&self) -> &bool {
        self._glacier_base.unlimited_ammo()
    }
    fn unlimited_ammo_mut(&mut self) -> &mut bool {
        self._glacier_base.unlimited_ammo_mut()
    }
    fn allow_teleport(&self) -> &bool {
        self._glacier_base.allow_teleport()
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_teleport_mut()
    }
    fn use_objective_teleport(&self) -> &bool {
        self._glacier_base.use_objective_teleport()
    }
    fn use_objective_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.use_objective_teleport_mut()
    }
    fn use_stuck_escape_procedure(&self) -> &bool {
        self._glacier_base.use_stuck_escape_procedure()
    }
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        self._glacier_base.use_stuck_escape_procedure_mut()
    }
    fn use_navmesh(&self) -> &bool {
        self._glacier_base.use_navmesh()
    }
    fn use_navmesh_mut(&mut self) -> &mut bool {
        self._glacier_base.use_navmesh_mut()
    }
    fn allow_objective_move_outside_combat_area(&self) -> &bool {
        self._glacier_base.allow_objective_move_outside_combat_area()
    }
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_objective_move_outside_combat_area_mut()
    }
    fn allow_secondary_objectives(&self) -> &bool {
        self._glacier_base.allow_secondary_objectives()
    }
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_secondary_objectives_mut()
    }
    fn disable_zoom_when_aiming(&self) -> &bool {
        self._glacier_base.disable_zoom_when_aiming()
    }
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool {
        self._glacier_base.disable_zoom_when_aiming_mut()
    }
    fn move_mode(&self) -> &AutoPlayerMoveMode {
        self._glacier_base.move_mode()
    }
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode {
        self._glacier_base.move_mode_mut()
    }
    fn move_mode_override_int(&self) -> &i32 {
        self._glacier_base.move_mode_override_int()
    }
    fn move_mode_override_int_mut(&mut self) -> &mut i32 {
        self._glacier_base.move_mode_override_int_mut()
    }
    fn debug_name(&self) -> &String {
        self._glacier_base.debug_name()
    }
    fn debug_name_mut(&mut self) -> &mut String {
        self._glacier_base.debug_name_mut()
    }
    fn time_threshold(&self) -> &i32 {
        self._glacier_base.time_threshold()
    }
    fn time_threshold_mut(&mut self) -> &mut i32 {
        self._glacier_base.time_threshold_mut()
    }
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters()
    }
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters_mut()
    }
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre()
    }
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre_mut()
    }
    fn restricted_area_sphere_radius(&self) -> &f32 {
        self._glacier_base.restricted_area_sphere_radius()
    }
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.restricted_area_sphere_radius_mut()
    }
}

impl super::entity::EntityDataTrait for AutoPlayerDefendObjectiveEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerDefendObjectiveEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerDefendObjectiveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerDefendObjectiveEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerDefendObjectiveEntityData {
}

pub static AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerDefendObjectiveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, target_position),
            },
            FieldInfoData {
                name: "DefendAreaRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, defend_area_radius),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, use_random_path_spread),
            },
            FieldInfoData {
                name: "DefendCurrentPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, defend_current_position),
            },
            FieldInfoData {
                name: "IsTargetReachedAsSoonAsEnteringTargetArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, is_target_reached_as_soon_as_entering_target_area),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerDefendObjectiveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerDefendObjectiveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerAttackObjectiveEntityData {
    pub _glacier_base: AutoPlayerObjectiveEntityData,
    pub targets: super::dice_shooter_shared::QueryEntityResult,
    pub weapon: i32,
    pub use_random_path_spread: bool,
}

pub trait AutoPlayerAttackObjectiveEntityDataTrait: AutoPlayerObjectiveEntityDataTrait {
    fn targets(&self) -> &super::dice_shooter_shared::QueryEntityResult;
    fn targets_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult;
    fn weapon(&self) -> &i32;
    fn weapon_mut(&mut self) -> &mut i32;
    fn use_random_path_spread(&self) -> &bool;
    fn use_random_path_spread_mut(&mut self) -> &mut bool;
}

impl AutoPlayerAttackObjectiveEntityDataTrait for AutoPlayerAttackObjectiveEntityData {
    fn targets(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        &self.targets
    }
    fn targets_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        &mut self.targets
    }
    fn weapon(&self) -> &i32 {
        &self.weapon
    }
    fn weapon_mut(&mut self) -> &mut i32 {
        &mut self.weapon
    }
    fn use_random_path_spread(&self) -> &bool {
        &self.use_random_path_spread
    }
    fn use_random_path_spread_mut(&mut self) -> &mut bool {
        &mut self.use_random_path_spread
    }
}

impl AutoPlayerObjectiveEntityDataTrait for AutoPlayerAttackObjectiveEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players()
    }
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players_mut()
    }
    fn jesus_mode(&self) -> &bool {
        self._glacier_base.jesus_mode()
    }
    fn jesus_mode_mut(&mut self) -> &mut bool {
        self._glacier_base.jesus_mode_mut()
    }
    fn unlimited_ammo(&self) -> &bool {
        self._glacier_base.unlimited_ammo()
    }
    fn unlimited_ammo_mut(&mut self) -> &mut bool {
        self._glacier_base.unlimited_ammo_mut()
    }
    fn allow_teleport(&self) -> &bool {
        self._glacier_base.allow_teleport()
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_teleport_mut()
    }
    fn use_objective_teleport(&self) -> &bool {
        self._glacier_base.use_objective_teleport()
    }
    fn use_objective_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.use_objective_teleport_mut()
    }
    fn use_stuck_escape_procedure(&self) -> &bool {
        self._glacier_base.use_stuck_escape_procedure()
    }
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        self._glacier_base.use_stuck_escape_procedure_mut()
    }
    fn use_navmesh(&self) -> &bool {
        self._glacier_base.use_navmesh()
    }
    fn use_navmesh_mut(&mut self) -> &mut bool {
        self._glacier_base.use_navmesh_mut()
    }
    fn allow_objective_move_outside_combat_area(&self) -> &bool {
        self._glacier_base.allow_objective_move_outside_combat_area()
    }
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_objective_move_outside_combat_area_mut()
    }
    fn allow_secondary_objectives(&self) -> &bool {
        self._glacier_base.allow_secondary_objectives()
    }
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_secondary_objectives_mut()
    }
    fn disable_zoom_when_aiming(&self) -> &bool {
        self._glacier_base.disable_zoom_when_aiming()
    }
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool {
        self._glacier_base.disable_zoom_when_aiming_mut()
    }
    fn move_mode(&self) -> &AutoPlayerMoveMode {
        self._glacier_base.move_mode()
    }
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode {
        self._glacier_base.move_mode_mut()
    }
    fn move_mode_override_int(&self) -> &i32 {
        self._glacier_base.move_mode_override_int()
    }
    fn move_mode_override_int_mut(&mut self) -> &mut i32 {
        self._glacier_base.move_mode_override_int_mut()
    }
    fn debug_name(&self) -> &String {
        self._glacier_base.debug_name()
    }
    fn debug_name_mut(&mut self) -> &mut String {
        self._glacier_base.debug_name_mut()
    }
    fn time_threshold(&self) -> &i32 {
        self._glacier_base.time_threshold()
    }
    fn time_threshold_mut(&mut self) -> &mut i32 {
        self._glacier_base.time_threshold_mut()
    }
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters()
    }
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters_mut()
    }
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre()
    }
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre_mut()
    }
    fn restricted_area_sphere_radius(&self) -> &f32 {
        self._glacier_base.restricted_area_sphere_radius()
    }
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.restricted_area_sphere_radius_mut()
    }
}

impl super::entity::EntityDataTrait for AutoPlayerAttackObjectiveEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerAttackObjectiveEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerAttackObjectiveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerAttackObjectiveEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerAttackObjectiveEntityData {
}

pub static AUTOPLAYERATTACKOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerAttackObjectiveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Targets",
                flags: MemberInfoFlags::new(0),
                field_type: "QueryEntityResult",
                rust_offset: offset_of!(AutoPlayerAttackObjectiveEntityData, targets),
            },
            FieldInfoData {
                name: "Weapon",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerAttackObjectiveEntityData, weapon),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerAttackObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERATTACKOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerAttackObjectiveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERATTACKOBJECTIVEENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERATTACKOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerAttackObjectiveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerActionObjectiveEntityData {
    pub _glacier_base: AutoPlayerObjectiveEntityData,
    pub action_target_position: super::core::Vec3,
    pub action_position: super::core::Vec3,
    pub action_start_radius: f32,
    pub select_item_input: i32,
    pub action_input: i32,
    pub action_time: f32,
    pub is_a_spamming_button_action: bool,
    pub use_random_path_spread: bool,
}

pub trait AutoPlayerActionObjectiveEntityDataTrait: AutoPlayerObjectiveEntityDataTrait {
    fn action_target_position(&self) -> &super::core::Vec3;
    fn action_target_position_mut(&mut self) -> &mut super::core::Vec3;
    fn action_position(&self) -> &super::core::Vec3;
    fn action_position_mut(&mut self) -> &mut super::core::Vec3;
    fn action_start_radius(&self) -> &f32;
    fn action_start_radius_mut(&mut self) -> &mut f32;
    fn select_item_input(&self) -> &i32;
    fn select_item_input_mut(&mut self) -> &mut i32;
    fn action_input(&self) -> &i32;
    fn action_input_mut(&mut self) -> &mut i32;
    fn action_time(&self) -> &f32;
    fn action_time_mut(&mut self) -> &mut f32;
    fn is_a_spamming_button_action(&self) -> &bool;
    fn is_a_spamming_button_action_mut(&mut self) -> &mut bool;
    fn use_random_path_spread(&self) -> &bool;
    fn use_random_path_spread_mut(&mut self) -> &mut bool;
}

impl AutoPlayerActionObjectiveEntityDataTrait for AutoPlayerActionObjectiveEntityData {
    fn action_target_position(&self) -> &super::core::Vec3 {
        &self.action_target_position
    }
    fn action_target_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.action_target_position
    }
    fn action_position(&self) -> &super::core::Vec3 {
        &self.action_position
    }
    fn action_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.action_position
    }
    fn action_start_radius(&self) -> &f32 {
        &self.action_start_radius
    }
    fn action_start_radius_mut(&mut self) -> &mut f32 {
        &mut self.action_start_radius
    }
    fn select_item_input(&self) -> &i32 {
        &self.select_item_input
    }
    fn select_item_input_mut(&mut self) -> &mut i32 {
        &mut self.select_item_input
    }
    fn action_input(&self) -> &i32 {
        &self.action_input
    }
    fn action_input_mut(&mut self) -> &mut i32 {
        &mut self.action_input
    }
    fn action_time(&self) -> &f32 {
        &self.action_time
    }
    fn action_time_mut(&mut self) -> &mut f32 {
        &mut self.action_time
    }
    fn is_a_spamming_button_action(&self) -> &bool {
        &self.is_a_spamming_button_action
    }
    fn is_a_spamming_button_action_mut(&mut self) -> &mut bool {
        &mut self.is_a_spamming_button_action
    }
    fn use_random_path_spread(&self) -> &bool {
        &self.use_random_path_spread
    }
    fn use_random_path_spread_mut(&mut self) -> &mut bool {
        &mut self.use_random_path_spread
    }
}

impl AutoPlayerObjectiveEntityDataTrait for AutoPlayerActionObjectiveEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn players(&self) -> &super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players()
    }
    fn players_mut(&mut self) -> &mut super::dice_shooter_shared::QueryEntityResult {
        self._glacier_base.players_mut()
    }
    fn jesus_mode(&self) -> &bool {
        self._glacier_base.jesus_mode()
    }
    fn jesus_mode_mut(&mut self) -> &mut bool {
        self._glacier_base.jesus_mode_mut()
    }
    fn unlimited_ammo(&self) -> &bool {
        self._glacier_base.unlimited_ammo()
    }
    fn unlimited_ammo_mut(&mut self) -> &mut bool {
        self._glacier_base.unlimited_ammo_mut()
    }
    fn allow_teleport(&self) -> &bool {
        self._glacier_base.allow_teleport()
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_teleport_mut()
    }
    fn use_objective_teleport(&self) -> &bool {
        self._glacier_base.use_objective_teleport()
    }
    fn use_objective_teleport_mut(&mut self) -> &mut bool {
        self._glacier_base.use_objective_teleport_mut()
    }
    fn use_stuck_escape_procedure(&self) -> &bool {
        self._glacier_base.use_stuck_escape_procedure()
    }
    fn use_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        self._glacier_base.use_stuck_escape_procedure_mut()
    }
    fn use_navmesh(&self) -> &bool {
        self._glacier_base.use_navmesh()
    }
    fn use_navmesh_mut(&mut self) -> &mut bool {
        self._glacier_base.use_navmesh_mut()
    }
    fn allow_objective_move_outside_combat_area(&self) -> &bool {
        self._glacier_base.allow_objective_move_outside_combat_area()
    }
    fn allow_objective_move_outside_combat_area_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_objective_move_outside_combat_area_mut()
    }
    fn allow_secondary_objectives(&self) -> &bool {
        self._glacier_base.allow_secondary_objectives()
    }
    fn allow_secondary_objectives_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_secondary_objectives_mut()
    }
    fn disable_zoom_when_aiming(&self) -> &bool {
        self._glacier_base.disable_zoom_when_aiming()
    }
    fn disable_zoom_when_aiming_mut(&mut self) -> &mut bool {
        self._glacier_base.disable_zoom_when_aiming_mut()
    }
    fn move_mode(&self) -> &AutoPlayerMoveMode {
        self._glacier_base.move_mode()
    }
    fn move_mode_mut(&mut self) -> &mut AutoPlayerMoveMode {
        self._glacier_base.move_mode_mut()
    }
    fn move_mode_override_int(&self) -> &i32 {
        self._glacier_base.move_mode_override_int()
    }
    fn move_mode_override_int_mut(&mut self) -> &mut i32 {
        self._glacier_base.move_mode_override_int_mut()
    }
    fn debug_name(&self) -> &String {
        self._glacier_base.debug_name()
    }
    fn debug_name_mut(&mut self) -> &mut String {
        self._glacier_base.debug_name_mut()
    }
    fn time_threshold(&self) -> &i32 {
        self._glacier_base.time_threshold()
    }
    fn time_threshold_mut(&mut self) -> &mut i32 {
        self._glacier_base.time_threshold_mut()
    }
    fn clamp_vertical_nav_pos_search_meters(&self) -> &f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters()
    }
    fn clamp_vertical_nav_pos_search_meters_mut(&mut self) -> &mut f32 {
        self._glacier_base.clamp_vertical_nav_pos_search_meters_mut()
    }
    fn restricted_area_sphere_centre(&self) -> &super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre()
    }
    fn restricted_area_sphere_centre_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.restricted_area_sphere_centre_mut()
    }
    fn restricted_area_sphere_radius(&self) -> &f32 {
        self._glacier_base.restricted_area_sphere_radius()
    }
    fn restricted_area_sphere_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.restricted_area_sphere_radius_mut()
    }
}

impl super::entity::EntityDataTrait for AutoPlayerActionObjectiveEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerActionObjectiveEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerActionObjectiveEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerActionObjectiveEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerActionObjectiveEntityData {
}

pub static AUTOPLAYERACTIONOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerActionObjectiveEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ActionTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_target_position),
            },
            FieldInfoData {
                name: "ActionPosition",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_position),
            },
            FieldInfoData {
                name: "ActionStartRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_start_radius),
            },
            FieldInfoData {
                name: "SelectItemInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, select_item_input),
            },
            FieldInfoData {
                name: "ActionInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_input),
            },
            FieldInfoData {
                name: "ActionTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_time),
            },
            FieldInfoData {
                name: "IsASpammingButtonAction",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, is_a_spamming_button_action),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERACTIONOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerActionObjectiveEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERACTIONOBJECTIVEENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERACTIONOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerActionObjectiveEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerSettingsEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub choice: AutoPlayerSettingsChoice,
}

pub trait AutoPlayerSettingsEntityDataTrait: super::entity::EntityDataTrait {
    fn choice(&self) -> &AutoPlayerSettingsChoice;
    fn choice_mut(&mut self) -> &mut AutoPlayerSettingsChoice;
}

impl AutoPlayerSettingsEntityDataTrait for AutoPlayerSettingsEntityData {
    fn choice(&self) -> &AutoPlayerSettingsChoice {
        &self.choice
    }
    fn choice_mut(&mut self) -> &mut AutoPlayerSettingsChoice {
        &mut self.choice
    }
}

impl super::entity::EntityDataTrait for AutoPlayerSettingsEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerSettingsEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerSettingsEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerSettingsEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerSettingsEntityData {
}

pub static AUTOPLAYERSETTINGSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerSettingsEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Choice",
                flags: MemberInfoFlags::new(0),
                field_type: "AutoPlayerSettingsChoice",
                rust_offset: offset_of!(AutoPlayerSettingsEntityData, choice),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERSETTINGSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerSettingsEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERSETTINGSENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERSETTINGSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettingsEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerSettingsChoice {
    pub kind: AutoPlayerSettingsKind,
    pub name: String,
}

pub trait AutoPlayerSettingsChoiceTrait: TypeObject {
    fn kind(&self) -> &AutoPlayerSettingsKind;
    fn kind_mut(&mut self) -> &mut AutoPlayerSettingsKind;
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
}

impl AutoPlayerSettingsChoiceTrait for AutoPlayerSettingsChoice {
    fn kind(&self) -> &AutoPlayerSettingsKind {
        &self.kind
    }
    fn kind_mut(&mut self) -> &mut AutoPlayerSettingsKind {
        &mut self.kind
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

pub static AUTOPLAYERSETTINGSCHOICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsChoice",
    flags: MemberInfoFlags::new(73),
    module: "AutoPlayers",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerSettingsChoice as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Kind",
                flags: MemberInfoFlags::new(0),
                field_type: "AutoPlayerSettingsKind",
                rust_offset: offset_of!(AutoPlayerSettingsChoice, kind),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AutoPlayerSettingsChoice, name),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERSETTINGSCHOICE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerSettingsChoice {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERSETTINGSCHOICE_TYPE_INFO
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


pub static AUTOPLAYERSETTINGSCHOICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsChoice-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettingsChoice"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AutoPlayerSettingsKind {
    #[default]
    Kind_Int = 0,
    Kind_Bool = 1,
    Kind_String = 2,
    Kind_Float = 3,
    Kind_Uint = 4,
}

pub static AUTOPLAYERSETTINGSKIND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsKind",
    flags: MemberInfoFlags::new(49429),
    module: "AutoPlayers",
    data: TypeInfoData::Enum,
    array_type: Some(AUTOPLAYERSETTINGSKIND_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPlayerSettingsKind {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERSETTINGSKIND_TYPE_INFO
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


pub static AUTOPLAYERSETTINGSKIND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsKind-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettingsKind"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerManagerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub player_count: i32,
    pub fill_gameplay_bots_team1: i32,
    pub fill_gameplay_bots_team2: i32,
    pub reset_force_fills: bool,
    pub orphan_time_seconds: f32,
}

pub trait AutoPlayerManagerEntityDataTrait: super::entity::EntityDataTrait {
    fn player_count(&self) -> &i32;
    fn player_count_mut(&mut self) -> &mut i32;
    fn fill_gameplay_bots_team1(&self) -> &i32;
    fn fill_gameplay_bots_team1_mut(&mut self) -> &mut i32;
    fn fill_gameplay_bots_team2(&self) -> &i32;
    fn fill_gameplay_bots_team2_mut(&mut self) -> &mut i32;
    fn reset_force_fills(&self) -> &bool;
    fn reset_force_fills_mut(&mut self) -> &mut bool;
    fn orphan_time_seconds(&self) -> &f32;
    fn orphan_time_seconds_mut(&mut self) -> &mut f32;
}

impl AutoPlayerManagerEntityDataTrait for AutoPlayerManagerEntityData {
    fn player_count(&self) -> &i32 {
        &self.player_count
    }
    fn player_count_mut(&mut self) -> &mut i32 {
        &mut self.player_count
    }
    fn fill_gameplay_bots_team1(&self) -> &i32 {
        &self.fill_gameplay_bots_team1
    }
    fn fill_gameplay_bots_team1_mut(&mut self) -> &mut i32 {
        &mut self.fill_gameplay_bots_team1
    }
    fn fill_gameplay_bots_team2(&self) -> &i32 {
        &self.fill_gameplay_bots_team2
    }
    fn fill_gameplay_bots_team2_mut(&mut self) -> &mut i32 {
        &mut self.fill_gameplay_bots_team2
    }
    fn reset_force_fills(&self) -> &bool {
        &self.reset_force_fills
    }
    fn reset_force_fills_mut(&mut self) -> &mut bool {
        &mut self.reset_force_fills
    }
    fn orphan_time_seconds(&self) -> &f32 {
        &self.orphan_time_seconds
    }
    fn orphan_time_seconds_mut(&mut self) -> &mut f32 {
        &mut self.orphan_time_seconds
    }
}

impl super::entity::EntityDataTrait for AutoPlayerManagerEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerManagerEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerManagerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerManagerEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerManagerEntityData {
}

pub static AUTOPLAYERMANAGERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerManagerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerManagerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PlayerCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerManagerEntityData, player_count),
            },
            FieldInfoData {
                name: "FillGameplayBotsTeam1",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerManagerEntityData, fill_gameplay_bots_team1),
            },
            FieldInfoData {
                name: "FillGameplayBotsTeam2",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerManagerEntityData, fill_gameplay_bots_team2),
            },
            FieldInfoData {
                name: "ResetForceFills",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerManagerEntityData, reset_force_fills),
            },
            FieldInfoData {
                name: "OrphanTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerManagerEntityData, orphan_time_seconds),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERMANAGERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerManagerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERMANAGERENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERMANAGERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerManagerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerManagerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub auto_start: bool,
}

pub trait AutoPlayerEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
}

impl AutoPlayerEntityDataTrait for AutoPlayerEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
}

impl super::entity::EntityDataTrait for AutoPlayerEntityData {
}

impl super::entity::GameObjectDataTrait for AutoPlayerEntityData {
}

impl super::core::DataBusPeerTrait for AutoPlayerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for AutoPlayerEntityData {
}

impl super::core::DataContainerTrait for AutoPlayerEntityData {
}

pub static AUTOPLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(AutoPlayerEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerEntityData, auto_start),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERENTITYDATA_TYPE_INFO
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


pub static AUTOPLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub a_f_k_takeover: f32,
    pub client_enabled: bool,
    pub allow_client_take_over: bool,
    pub force_server_control: bool,
    pub force_server_objective_control: bool,
    pub force_client_objective_control: bool,
    pub force_client_navigation: bool,
    pub debug_draw_enabled: bool,
    pub debug_draw_waypoints: bool,
    pub debug_draw_client_details: bool,
    pub debug_draw_combat_details: bool,
    pub player_count: i32,
    pub forced_server_auto_player_count: i32,
    pub allow_add_auto_fill_players: bool,
    pub allow_remove_auto_fill_players: bool,
    pub force_apply_gameplay_bots_count: bool,
    pub force_fill_gameplay_bots_team1: i32,
    pub force_fill_gameplay_bots_team2: i32,
    pub respawn_delay: f32,
    pub initial_respawn_delay: f32,
    pub client_join_delay: f32,
    pub round_timeout: i32,
    pub squad_members: i32,
    pub allow_gameplay_bots_to_join_player_squads: bool,
    pub allow_gameplay_bots_to_form_own_squads: bool,
    pub allow_vehicle_spawn: bool,
    pub force_disable_vehicle_spawn: bool,
    pub allow_client_vehicle_spawn: bool,
    pub allow_first_client_initial_vehicle_spawn: bool,
    pub control_connectionless_players: bool,
    pub allow_respawn: bool,
    pub pickup_items_secondary_objective_attempt_interval_seconds: i32,
    pub use_telemetry_based_planner: bool,
    pub debug_telemetry_based_planner: bool,
    pub planner_terrain_vertical_cutoff: f32,
    pub planner_connection_cutoff: f32,
    pub planner_max_nodes_search_radius: f32,
    pub planner_link_end_arrival_range: f32,
    pub use_fade_override: bool,
    pub input_scale_yaw: f32,
    pub input_scale_pitch: f32,
    pub input_scale_client: f32,
    pub input_force_mouse: bool,
    pub use_input_override_yaw_pitch: bool,
    pub input_override_yaw: f32,
    pub input_override_pitch: f32,
    pub use_seek_and_destroy: bool,
    pub allow_teleport: bool,
    pub force_allow_all_teleports: bool,
    pub debug_draw_teleports: bool,
    pub update_a_i: bool,
    pub debug_draw_client_only: bool,
    pub debug_draw_client_realm_only: bool,
    pub aim_acceleration: f32,
    pub aim_lap_time: f32,
    pub allow_move_outside_combat_area: bool,
    pub allow_spawn_outside_combat_area: bool,
    pub allow_vehicle_spawn_outside_combat_area: bool,
    pub allow_vehicle_spawn_only: bool,
    pub debug_draw_pretty_path: bool,
    pub debug_draw_use_waypoints_alpha: bool,
    pub debug_draw_invalid_move_intention: bool,
    pub debug_spam: bool,
    pub lof_timeout_s: f32,
    pub lof_reaction_time_s: f32,
    pub server_players_ignore_client_players: bool,
    pub ignore_human_players: bool,
    pub force_kit: i32,
    pub opportunistic_interact: bool,
    pub squad_spawn_probability: f32,
    pub kit_change_probability: f32,
    pub use_default_unlocks_probability: f32,
    pub allow_medic_revive: bool,
    pub allow_pickup_items: bool,
    pub debug_draw_objectives: bool,
    pub debug_draw_objective_always: bool,
    pub wallhack: bool,
    pub weapon_swap_interval_s: f32,
    pub weapon_swap_primary_probability: f32,
    pub vehicle_bail_time: i32,
    pub jump_if_stuck_time_seconds: f32,
    pub jump_cooldown_seconds: f32,
    pub patrol_position_cooldown_seconds: f32,
    pub combat_use_grenades: bool,
    pub combat_use_prone: bool,
    pub combat_use_melee: bool,
    pub use_crouch: bool,
    pub forced_fire_time_max_s: f32,
    pub forced_fire_time_min_s: f32,
    pub allow_primary_weapon_forced_fire: bool,
    pub allow_vehicle_forced_fire: bool,
    pub forced_fire_vehicle_time_scale: f32,
    pub exit_vehicle_when_stuck_timeout: f32,
    pub min_distance_for_vehicle_u_turn: f32,
    pub min_airplane_bail_out_time: i32,
    pub max_airplane_bail_out_time: i32,
    pub login_rate: f32,
    pub spawn_rate: f32,
    pub max_spawns_per_update: i32,
    pub variance: f32,
    pub airplane_exit_input: i32,
    pub secondary_objective_generation_min_seconds: f32,
    pub secondary_objective_generation_max_seconds: f32,
    pub allow_enter_vehicle: bool,
    pub enter_vehicle_cooldown_seconds: f32,
    pub enter_vehicle_probability: f32,
    pub enter_vehicle_search_radius: f32,
    pub print_client_input: bool,
    pub allow_primary_objective: bool,
    pub allow_secondary_objectives_while_passive: bool,
    pub allow_secondary_objectives_while_defensive: bool,
    pub allow_pathfinding: bool,
    pub secondary_objective_timeout_seconds: f32,
    pub force_passive_mode: bool,
    pub force_primary_objective_defensive_mode: bool,
    pub force_primary_objective_aggressive_mode: bool,
    pub force_secondary_objective_defensive_mode: bool,
    pub force_secondary_objective_aggressive_mode: bool,
    pub client_jesus_mode: bool,
    pub allow_fortifications: bool,
    pub fortification_probability: f32,
    pub fortification_search_radius: f32,
    pub repath_cooldown_seconds: f32,
    pub un_stuck_vehicle_actions_trigger_time_seconds: f32,
    pub unstuck_minimal_move_distance: f32,
    pub unstuck_minimal_move_suicide_timeout: f32,
    pub fallen_below_suicide_timeout: f32,
    pub navigation_position_tolerance_meters: f32,
    pub use_name_generator: bool,
    pub allow_stuck_escape_procedure: bool,
    pub exit_stuck_escape_procedure_on_visual_check: bool,
    pub stuck_escape_procedure_sensor_length: f32,
    pub stuck_escape_procedure_p_i_fraction: f32,
    pub stuck_escape_procedure_escape_distance: f32,
    pub stuck_escape_procedure_activation_seconds: f32,
    pub stuck_escape_procedure_update_interval: f32,
    pub stuck_escape_procedure_timeout_seconds: f32,
    pub debug_draw_unstuck: bool,
    pub un_stuck_actions_trigger_time_seconds: f32,
    pub un_stuck_actions_trigger_cooldown: f32,
    pub stuck_escape_procedure_retries: i32,
    pub primary_interaction_search_radius: f32,
    pub allow_suicide: bool,
    pub allow_random_behavior: bool,
    pub allow_secondary_interactions: bool,
    pub secondary_interactions_probability: f32,
    pub secondary_interactions_search_radius: f32,
    pub secondary_objective_pickup_items_search_radius: f32,
    pub secondary_objective_pickup_items_interact_or_action_radius: f32,
    pub secondary_objective_jesus_mode: bool,
    pub secondary_revive_search_distance: f32,
    pub debug_draw_navigation_details: bool,
    pub debug_draw_navigation_progress_details: bool,
    pub debug_draw_custom_input: bool,
    pub expected_travel_time_distance_scale: f32,
    pub expected_travel_time_base: f32,
    pub interact_area_time: f32,
    pub debug_highlight_objective_type: i32,
    pub seek_and_destroy_min_radius: f32,
    pub seek_and_destroy_max_radius: f32,
    pub force_repath_if_too_far_from_waypoint_meters: f32,
    pub waypoint_minimum_progress_meters: f32,
    pub debug_draw_aim_noise: bool,
    pub aim_noise_scale: f32,
    pub target_min_switch_time_s: f32,
    pub max_target_engaging_distance_scale: f32,
    pub allow_random_path_spread: bool,
    pub force_use_random_path_spread: bool,
    pub random_path_spread_radius: f32,
    pub random_path_spread_center_distance: f32,
    pub update_target_cooldown: f32,
    pub forced_target_timeout_seconds: f32,
    pub debug_draw_players_names_and_ids: bool,
    pub verbose_logging: bool,
    pub action_objective_default_time: f32,
    pub allow_action_gadget: bool,
    pub action_gadget_probability: f32,
    pub action_gadget_interactable_search_radius: f32,
    pub hero_spawn_probability_gameplay: f32,
    pub special_spawn_probability_gameplay: f32,
    pub hero_vehicle_spawn_probability_gameplay: f32,
    pub vehicle_spawn_probability_gameplay: f32,
    pub hero_spawn_probability: f32,
    pub special_spawn_probability: f32,
    pub hero_vehicle_spawn_probability: f32,
    pub vehicle_spawn_probability: f32,
    pub follow_target_position_check_cooldown: f32,
    pub not_alive_assert_time: f32,
    pub prefer_f_p_s_camera: bool,
    pub time_on_path_tolerance_seconds: f32,
    pub check_water_depth_for_intermediate_positions: bool,
    pub swimming_suicide_timeout: f32,
    pub lof_prediction_time: f32,
    pub debug_draw_combat_raycast_hit_points: bool,
    pub debug_draw_transforms: bool,
    pub target_tracker_field_of_view_degrees: f32,
    pub pick_random_vehicle_on_secondary_objective: bool,
    pub never_exit_vehicle_after_entering: bool,
    pub update_target_per_frame_cap: u32,
    pub replay_telemetry_file: String,
    pub replay_telemetry_file_format: String,
    pub replay_telemetry_adjust_time: bool,
    pub replay_telemetry_adjust_time_padding: f32,
    pub debug_draw_weapon_details: bool,
    pub debug_draw_extensive_client_details: bool,
    pub evasive_maneuvers_jump_probability: f32,
    pub evasive_maneuvers_dodge_roll_probability: f32,
    pub evasive_maneuvers_invert_strafe_duration_max: f32,
    pub evasive_maneuvers_invert_strafe_duration_min: f32,
    pub leg_head_aim_ratio_override: f32,
    pub attacking_ability_left_probability: f32,
    pub attacking_ability_left_duration_seconds: f32,
    pub attacking_ability_middle_probability: f32,
    pub attacking_ability_middle_duration_seconds: f32,
    pub attacking_ability_right_probability: f32,
    pub attacking_ability_right_duration_seconds: f32,
    pub evasive_maneuvers_crouch_probability: f32,
    pub evasive_maneuvers_crouch_duration: f32,
    pub blaster_leg_head_aim_ratio: f32,
    pub blaster_aim_noise: f32,
    pub sniper_rifle_leg_head_aim_ratio: f32,
    pub sniper_rifle_aim_noise: f32,
    pub lmg_leg_head_aim_ratio: f32,
    pub lmg_aim_noise: f32,
    pub shotgun_leg_head_aim_ratio: f32,
    pub shotgun_aim_noise: f32,
    pub launcher_leg_head_aim_ratio: f32,
    pub launcher_aim_noise: f32,
    pub use_sword_attacking_abilities_from_meters: f32,
    pub sword_attack_duration_time_min_s: f32,
    pub sword_attack_duration_time_max_s: f32,
    pub pause_sword_attack_duration_time_min_s: f32,
    pub pause_sword_attack_duration_time_max_s: f32,
    pub sword_attack_distance_meters_min: f32,
    pub sword_attack_distance_meters_max: f32,
    pub debug_draw_input_details: bool,
    pub debug_window_position_scale_offset_x: f32,
    pub debug_window_position_scale_offset_y: f32,
    pub debug_window_width: i32,
    pub debug_window_height: i32,
    pub path_look_ahead_meters: f32,
    pub path_look_right_meters: f32,
    pub waypoint_tolerance_meters: f32,
    pub debug_draw_aim_at_positions: bool,
    pub evasive_maneuvers_vehicle_scale: f32,
    pub vehicle_aim_noise_scale: f32,
    pub reset_settings_on_level_unload: bool,
    pub sword_guard_duration_time_min_s: f32,
    pub sword_guard_duration_time_max_s: f32,
    pub aim_noise_scale_team1: f32,
    pub aim_noise_scale_team2: f32,
    pub hero_strafe_probability_per_frame: f32,
    pub emote_probability_after_players_death: f32,
    pub emote_duration: f32,
    pub melee_interval_s: f32,
    pub melee_distance_m: f32,
    pub allow_evasive_manouvers_o_o_b: bool,
    pub evasive_maneuvers_ground_check_enabled: bool,
    pub evasive_maneuvers_ground_check_distance_m: f32,
    pub evasive_maneuvers_ground_check_height_distance_m: f32,
    pub evasive_maneuvers_ground_check_height_offset_m: f32,
    pub evasive_maneuvers_ground_check_cooldown_s: f32,
    pub evasive_maneuvers_vehicles_enabled: bool,
    pub vehicle_minimum_forward_throttle: f32,
    pub vehicle_use_character_throttle: bool,
}

pub trait AutoPlayerSettingsTrait: super::core::SystemSettingsTrait {
    fn a_f_k_takeover(&self) -> &f32;
    fn a_f_k_takeover_mut(&mut self) -> &mut f32;
    fn client_enabled(&self) -> &bool;
    fn client_enabled_mut(&mut self) -> &mut bool;
    fn allow_client_take_over(&self) -> &bool;
    fn allow_client_take_over_mut(&mut self) -> &mut bool;
    fn force_server_control(&self) -> &bool;
    fn force_server_control_mut(&mut self) -> &mut bool;
    fn force_server_objective_control(&self) -> &bool;
    fn force_server_objective_control_mut(&mut self) -> &mut bool;
    fn force_client_objective_control(&self) -> &bool;
    fn force_client_objective_control_mut(&mut self) -> &mut bool;
    fn force_client_navigation(&self) -> &bool;
    fn force_client_navigation_mut(&mut self) -> &mut bool;
    fn debug_draw_enabled(&self) -> &bool;
    fn debug_draw_enabled_mut(&mut self) -> &mut bool;
    fn debug_draw_waypoints(&self) -> &bool;
    fn debug_draw_waypoints_mut(&mut self) -> &mut bool;
    fn debug_draw_client_details(&self) -> &bool;
    fn debug_draw_client_details_mut(&mut self) -> &mut bool;
    fn debug_draw_combat_details(&self) -> &bool;
    fn debug_draw_combat_details_mut(&mut self) -> &mut bool;
    fn player_count(&self) -> &i32;
    fn player_count_mut(&mut self) -> &mut i32;
    fn forced_server_auto_player_count(&self) -> &i32;
    fn forced_server_auto_player_count_mut(&mut self) -> &mut i32;
    fn allow_add_auto_fill_players(&self) -> &bool;
    fn allow_add_auto_fill_players_mut(&mut self) -> &mut bool;
    fn allow_remove_auto_fill_players(&self) -> &bool;
    fn allow_remove_auto_fill_players_mut(&mut self) -> &mut bool;
    fn force_apply_gameplay_bots_count(&self) -> &bool;
    fn force_apply_gameplay_bots_count_mut(&mut self) -> &mut bool;
    fn force_fill_gameplay_bots_team1(&self) -> &i32;
    fn force_fill_gameplay_bots_team1_mut(&mut self) -> &mut i32;
    fn force_fill_gameplay_bots_team2(&self) -> &i32;
    fn force_fill_gameplay_bots_team2_mut(&mut self) -> &mut i32;
    fn respawn_delay(&self) -> &f32;
    fn respawn_delay_mut(&mut self) -> &mut f32;
    fn initial_respawn_delay(&self) -> &f32;
    fn initial_respawn_delay_mut(&mut self) -> &mut f32;
    fn client_join_delay(&self) -> &f32;
    fn client_join_delay_mut(&mut self) -> &mut f32;
    fn round_timeout(&self) -> &i32;
    fn round_timeout_mut(&mut self) -> &mut i32;
    fn squad_members(&self) -> &i32;
    fn squad_members_mut(&mut self) -> &mut i32;
    fn allow_gameplay_bots_to_join_player_squads(&self) -> &bool;
    fn allow_gameplay_bots_to_join_player_squads_mut(&mut self) -> &mut bool;
    fn allow_gameplay_bots_to_form_own_squads(&self) -> &bool;
    fn allow_gameplay_bots_to_form_own_squads_mut(&mut self) -> &mut bool;
    fn allow_vehicle_spawn(&self) -> &bool;
    fn allow_vehicle_spawn_mut(&mut self) -> &mut bool;
    fn force_disable_vehicle_spawn(&self) -> &bool;
    fn force_disable_vehicle_spawn_mut(&mut self) -> &mut bool;
    fn allow_client_vehicle_spawn(&self) -> &bool;
    fn allow_client_vehicle_spawn_mut(&mut self) -> &mut bool;
    fn allow_first_client_initial_vehicle_spawn(&self) -> &bool;
    fn allow_first_client_initial_vehicle_spawn_mut(&mut self) -> &mut bool;
    fn control_connectionless_players(&self) -> &bool;
    fn control_connectionless_players_mut(&mut self) -> &mut bool;
    fn allow_respawn(&self) -> &bool;
    fn allow_respawn_mut(&mut self) -> &mut bool;
    fn pickup_items_secondary_objective_attempt_interval_seconds(&self) -> &i32;
    fn pickup_items_secondary_objective_attempt_interval_seconds_mut(&mut self) -> &mut i32;
    fn use_telemetry_based_planner(&self) -> &bool;
    fn use_telemetry_based_planner_mut(&mut self) -> &mut bool;
    fn debug_telemetry_based_planner(&self) -> &bool;
    fn debug_telemetry_based_planner_mut(&mut self) -> &mut bool;
    fn planner_terrain_vertical_cutoff(&self) -> &f32;
    fn planner_terrain_vertical_cutoff_mut(&mut self) -> &mut f32;
    fn planner_connection_cutoff(&self) -> &f32;
    fn planner_connection_cutoff_mut(&mut self) -> &mut f32;
    fn planner_max_nodes_search_radius(&self) -> &f32;
    fn planner_max_nodes_search_radius_mut(&mut self) -> &mut f32;
    fn planner_link_end_arrival_range(&self) -> &f32;
    fn planner_link_end_arrival_range_mut(&mut self) -> &mut f32;
    fn use_fade_override(&self) -> &bool;
    fn use_fade_override_mut(&mut self) -> &mut bool;
    fn input_scale_yaw(&self) -> &f32;
    fn input_scale_yaw_mut(&mut self) -> &mut f32;
    fn input_scale_pitch(&self) -> &f32;
    fn input_scale_pitch_mut(&mut self) -> &mut f32;
    fn input_scale_client(&self) -> &f32;
    fn input_scale_client_mut(&mut self) -> &mut f32;
    fn input_force_mouse(&self) -> &bool;
    fn input_force_mouse_mut(&mut self) -> &mut bool;
    fn use_input_override_yaw_pitch(&self) -> &bool;
    fn use_input_override_yaw_pitch_mut(&mut self) -> &mut bool;
    fn input_override_yaw(&self) -> &f32;
    fn input_override_yaw_mut(&mut self) -> &mut f32;
    fn input_override_pitch(&self) -> &f32;
    fn input_override_pitch_mut(&mut self) -> &mut f32;
    fn use_seek_and_destroy(&self) -> &bool;
    fn use_seek_and_destroy_mut(&mut self) -> &mut bool;
    fn allow_teleport(&self) -> &bool;
    fn allow_teleport_mut(&mut self) -> &mut bool;
    fn force_allow_all_teleports(&self) -> &bool;
    fn force_allow_all_teleports_mut(&mut self) -> &mut bool;
    fn debug_draw_teleports(&self) -> &bool;
    fn debug_draw_teleports_mut(&mut self) -> &mut bool;
    fn update_a_i(&self) -> &bool;
    fn update_a_i_mut(&mut self) -> &mut bool;
    fn debug_draw_client_only(&self) -> &bool;
    fn debug_draw_client_only_mut(&mut self) -> &mut bool;
    fn debug_draw_client_realm_only(&self) -> &bool;
    fn debug_draw_client_realm_only_mut(&mut self) -> &mut bool;
    fn aim_acceleration(&self) -> &f32;
    fn aim_acceleration_mut(&mut self) -> &mut f32;
    fn aim_lap_time(&self) -> &f32;
    fn aim_lap_time_mut(&mut self) -> &mut f32;
    fn allow_move_outside_combat_area(&self) -> &bool;
    fn allow_move_outside_combat_area_mut(&mut self) -> &mut bool;
    fn allow_spawn_outside_combat_area(&self) -> &bool;
    fn allow_spawn_outside_combat_area_mut(&mut self) -> &mut bool;
    fn allow_vehicle_spawn_outside_combat_area(&self) -> &bool;
    fn allow_vehicle_spawn_outside_combat_area_mut(&mut self) -> &mut bool;
    fn allow_vehicle_spawn_only(&self) -> &bool;
    fn allow_vehicle_spawn_only_mut(&mut self) -> &mut bool;
    fn debug_draw_pretty_path(&self) -> &bool;
    fn debug_draw_pretty_path_mut(&mut self) -> &mut bool;
    fn debug_draw_use_waypoints_alpha(&self) -> &bool;
    fn debug_draw_use_waypoints_alpha_mut(&mut self) -> &mut bool;
    fn debug_draw_invalid_move_intention(&self) -> &bool;
    fn debug_draw_invalid_move_intention_mut(&mut self) -> &mut bool;
    fn debug_spam(&self) -> &bool;
    fn debug_spam_mut(&mut self) -> &mut bool;
    fn lof_timeout_s(&self) -> &f32;
    fn lof_timeout_s_mut(&mut self) -> &mut f32;
    fn lof_reaction_time_s(&self) -> &f32;
    fn lof_reaction_time_s_mut(&mut self) -> &mut f32;
    fn server_players_ignore_client_players(&self) -> &bool;
    fn server_players_ignore_client_players_mut(&mut self) -> &mut bool;
    fn ignore_human_players(&self) -> &bool;
    fn ignore_human_players_mut(&mut self) -> &mut bool;
    fn force_kit(&self) -> &i32;
    fn force_kit_mut(&mut self) -> &mut i32;
    fn opportunistic_interact(&self) -> &bool;
    fn opportunistic_interact_mut(&mut self) -> &mut bool;
    fn squad_spawn_probability(&self) -> &f32;
    fn squad_spawn_probability_mut(&mut self) -> &mut f32;
    fn kit_change_probability(&self) -> &f32;
    fn kit_change_probability_mut(&mut self) -> &mut f32;
    fn use_default_unlocks_probability(&self) -> &f32;
    fn use_default_unlocks_probability_mut(&mut self) -> &mut f32;
    fn allow_medic_revive(&self) -> &bool;
    fn allow_medic_revive_mut(&mut self) -> &mut bool;
    fn allow_pickup_items(&self) -> &bool;
    fn allow_pickup_items_mut(&mut self) -> &mut bool;
    fn debug_draw_objectives(&self) -> &bool;
    fn debug_draw_objectives_mut(&mut self) -> &mut bool;
    fn debug_draw_objective_always(&self) -> &bool;
    fn debug_draw_objective_always_mut(&mut self) -> &mut bool;
    fn wallhack(&self) -> &bool;
    fn wallhack_mut(&mut self) -> &mut bool;
    fn weapon_swap_interval_s(&self) -> &f32;
    fn weapon_swap_interval_s_mut(&mut self) -> &mut f32;
    fn weapon_swap_primary_probability(&self) -> &f32;
    fn weapon_swap_primary_probability_mut(&mut self) -> &mut f32;
    fn vehicle_bail_time(&self) -> &i32;
    fn vehicle_bail_time_mut(&mut self) -> &mut i32;
    fn jump_if_stuck_time_seconds(&self) -> &f32;
    fn jump_if_stuck_time_seconds_mut(&mut self) -> &mut f32;
    fn jump_cooldown_seconds(&self) -> &f32;
    fn jump_cooldown_seconds_mut(&mut self) -> &mut f32;
    fn patrol_position_cooldown_seconds(&self) -> &f32;
    fn patrol_position_cooldown_seconds_mut(&mut self) -> &mut f32;
    fn combat_use_grenades(&self) -> &bool;
    fn combat_use_grenades_mut(&mut self) -> &mut bool;
    fn combat_use_prone(&self) -> &bool;
    fn combat_use_prone_mut(&mut self) -> &mut bool;
    fn combat_use_melee(&self) -> &bool;
    fn combat_use_melee_mut(&mut self) -> &mut bool;
    fn use_crouch(&self) -> &bool;
    fn use_crouch_mut(&mut self) -> &mut bool;
    fn forced_fire_time_max_s(&self) -> &f32;
    fn forced_fire_time_max_s_mut(&mut self) -> &mut f32;
    fn forced_fire_time_min_s(&self) -> &f32;
    fn forced_fire_time_min_s_mut(&mut self) -> &mut f32;
    fn allow_primary_weapon_forced_fire(&self) -> &bool;
    fn allow_primary_weapon_forced_fire_mut(&mut self) -> &mut bool;
    fn allow_vehicle_forced_fire(&self) -> &bool;
    fn allow_vehicle_forced_fire_mut(&mut self) -> &mut bool;
    fn forced_fire_vehicle_time_scale(&self) -> &f32;
    fn forced_fire_vehicle_time_scale_mut(&mut self) -> &mut f32;
    fn exit_vehicle_when_stuck_timeout(&self) -> &f32;
    fn exit_vehicle_when_stuck_timeout_mut(&mut self) -> &mut f32;
    fn min_distance_for_vehicle_u_turn(&self) -> &f32;
    fn min_distance_for_vehicle_u_turn_mut(&mut self) -> &mut f32;
    fn min_airplane_bail_out_time(&self) -> &i32;
    fn min_airplane_bail_out_time_mut(&mut self) -> &mut i32;
    fn max_airplane_bail_out_time(&self) -> &i32;
    fn max_airplane_bail_out_time_mut(&mut self) -> &mut i32;
    fn login_rate(&self) -> &f32;
    fn login_rate_mut(&mut self) -> &mut f32;
    fn spawn_rate(&self) -> &f32;
    fn spawn_rate_mut(&mut self) -> &mut f32;
    fn max_spawns_per_update(&self) -> &i32;
    fn max_spawns_per_update_mut(&mut self) -> &mut i32;
    fn variance(&self) -> &f32;
    fn variance_mut(&mut self) -> &mut f32;
    fn airplane_exit_input(&self) -> &i32;
    fn airplane_exit_input_mut(&mut self) -> &mut i32;
    fn secondary_objective_generation_min_seconds(&self) -> &f32;
    fn secondary_objective_generation_min_seconds_mut(&mut self) -> &mut f32;
    fn secondary_objective_generation_max_seconds(&self) -> &f32;
    fn secondary_objective_generation_max_seconds_mut(&mut self) -> &mut f32;
    fn allow_enter_vehicle(&self) -> &bool;
    fn allow_enter_vehicle_mut(&mut self) -> &mut bool;
    fn enter_vehicle_cooldown_seconds(&self) -> &f32;
    fn enter_vehicle_cooldown_seconds_mut(&mut self) -> &mut f32;
    fn enter_vehicle_probability(&self) -> &f32;
    fn enter_vehicle_probability_mut(&mut self) -> &mut f32;
    fn enter_vehicle_search_radius(&self) -> &f32;
    fn enter_vehicle_search_radius_mut(&mut self) -> &mut f32;
    fn print_client_input(&self) -> &bool;
    fn print_client_input_mut(&mut self) -> &mut bool;
    fn allow_primary_objective(&self) -> &bool;
    fn allow_primary_objective_mut(&mut self) -> &mut bool;
    fn allow_secondary_objectives_while_passive(&self) -> &bool;
    fn allow_secondary_objectives_while_passive_mut(&mut self) -> &mut bool;
    fn allow_secondary_objectives_while_defensive(&self) -> &bool;
    fn allow_secondary_objectives_while_defensive_mut(&mut self) -> &mut bool;
    fn allow_pathfinding(&self) -> &bool;
    fn allow_pathfinding_mut(&mut self) -> &mut bool;
    fn secondary_objective_timeout_seconds(&self) -> &f32;
    fn secondary_objective_timeout_seconds_mut(&mut self) -> &mut f32;
    fn force_passive_mode(&self) -> &bool;
    fn force_passive_mode_mut(&mut self) -> &mut bool;
    fn force_primary_objective_defensive_mode(&self) -> &bool;
    fn force_primary_objective_defensive_mode_mut(&mut self) -> &mut bool;
    fn force_primary_objective_aggressive_mode(&self) -> &bool;
    fn force_primary_objective_aggressive_mode_mut(&mut self) -> &mut bool;
    fn force_secondary_objective_defensive_mode(&self) -> &bool;
    fn force_secondary_objective_defensive_mode_mut(&mut self) -> &mut bool;
    fn force_secondary_objective_aggressive_mode(&self) -> &bool;
    fn force_secondary_objective_aggressive_mode_mut(&mut self) -> &mut bool;
    fn client_jesus_mode(&self) -> &bool;
    fn client_jesus_mode_mut(&mut self) -> &mut bool;
    fn allow_fortifications(&self) -> &bool;
    fn allow_fortifications_mut(&mut self) -> &mut bool;
    fn fortification_probability(&self) -> &f32;
    fn fortification_probability_mut(&mut self) -> &mut f32;
    fn fortification_search_radius(&self) -> &f32;
    fn fortification_search_radius_mut(&mut self) -> &mut f32;
    fn repath_cooldown_seconds(&self) -> &f32;
    fn repath_cooldown_seconds_mut(&mut self) -> &mut f32;
    fn un_stuck_vehicle_actions_trigger_time_seconds(&self) -> &f32;
    fn un_stuck_vehicle_actions_trigger_time_seconds_mut(&mut self) -> &mut f32;
    fn unstuck_minimal_move_distance(&self) -> &f32;
    fn unstuck_minimal_move_distance_mut(&mut self) -> &mut f32;
    fn unstuck_minimal_move_suicide_timeout(&self) -> &f32;
    fn unstuck_minimal_move_suicide_timeout_mut(&mut self) -> &mut f32;
    fn fallen_below_suicide_timeout(&self) -> &f32;
    fn fallen_below_suicide_timeout_mut(&mut self) -> &mut f32;
    fn navigation_position_tolerance_meters(&self) -> &f32;
    fn navigation_position_tolerance_meters_mut(&mut self) -> &mut f32;
    fn use_name_generator(&self) -> &bool;
    fn use_name_generator_mut(&mut self) -> &mut bool;
    fn allow_stuck_escape_procedure(&self) -> &bool;
    fn allow_stuck_escape_procedure_mut(&mut self) -> &mut bool;
    fn exit_stuck_escape_procedure_on_visual_check(&self) -> &bool;
    fn exit_stuck_escape_procedure_on_visual_check_mut(&mut self) -> &mut bool;
    fn stuck_escape_procedure_sensor_length(&self) -> &f32;
    fn stuck_escape_procedure_sensor_length_mut(&mut self) -> &mut f32;
    fn stuck_escape_procedure_p_i_fraction(&self) -> &f32;
    fn stuck_escape_procedure_p_i_fraction_mut(&mut self) -> &mut f32;
    fn stuck_escape_procedure_escape_distance(&self) -> &f32;
    fn stuck_escape_procedure_escape_distance_mut(&mut self) -> &mut f32;
    fn stuck_escape_procedure_activation_seconds(&self) -> &f32;
    fn stuck_escape_procedure_activation_seconds_mut(&mut self) -> &mut f32;
    fn stuck_escape_procedure_update_interval(&self) -> &f32;
    fn stuck_escape_procedure_update_interval_mut(&mut self) -> &mut f32;
    fn stuck_escape_procedure_timeout_seconds(&self) -> &f32;
    fn stuck_escape_procedure_timeout_seconds_mut(&mut self) -> &mut f32;
    fn debug_draw_unstuck(&self) -> &bool;
    fn debug_draw_unstuck_mut(&mut self) -> &mut bool;
    fn un_stuck_actions_trigger_time_seconds(&self) -> &f32;
    fn un_stuck_actions_trigger_time_seconds_mut(&mut self) -> &mut f32;
    fn un_stuck_actions_trigger_cooldown(&self) -> &f32;
    fn un_stuck_actions_trigger_cooldown_mut(&mut self) -> &mut f32;
    fn stuck_escape_procedure_retries(&self) -> &i32;
    fn stuck_escape_procedure_retries_mut(&mut self) -> &mut i32;
    fn primary_interaction_search_radius(&self) -> &f32;
    fn primary_interaction_search_radius_mut(&mut self) -> &mut f32;
    fn allow_suicide(&self) -> &bool;
    fn allow_suicide_mut(&mut self) -> &mut bool;
    fn allow_random_behavior(&self) -> &bool;
    fn allow_random_behavior_mut(&mut self) -> &mut bool;
    fn allow_secondary_interactions(&self) -> &bool;
    fn allow_secondary_interactions_mut(&mut self) -> &mut bool;
    fn secondary_interactions_probability(&self) -> &f32;
    fn secondary_interactions_probability_mut(&mut self) -> &mut f32;
    fn secondary_interactions_search_radius(&self) -> &f32;
    fn secondary_interactions_search_radius_mut(&mut self) -> &mut f32;
    fn secondary_objective_pickup_items_search_radius(&self) -> &f32;
    fn secondary_objective_pickup_items_search_radius_mut(&mut self) -> &mut f32;
    fn secondary_objective_pickup_items_interact_or_action_radius(&self) -> &f32;
    fn secondary_objective_pickup_items_interact_or_action_radius_mut(&mut self) -> &mut f32;
    fn secondary_objective_jesus_mode(&self) -> &bool;
    fn secondary_objective_jesus_mode_mut(&mut self) -> &mut bool;
    fn secondary_revive_search_distance(&self) -> &f32;
    fn secondary_revive_search_distance_mut(&mut self) -> &mut f32;
    fn debug_draw_navigation_details(&self) -> &bool;
    fn debug_draw_navigation_details_mut(&mut self) -> &mut bool;
    fn debug_draw_navigation_progress_details(&self) -> &bool;
    fn debug_draw_navigation_progress_details_mut(&mut self) -> &mut bool;
    fn debug_draw_custom_input(&self) -> &bool;
    fn debug_draw_custom_input_mut(&mut self) -> &mut bool;
    fn expected_travel_time_distance_scale(&self) -> &f32;
    fn expected_travel_time_distance_scale_mut(&mut self) -> &mut f32;
    fn expected_travel_time_base(&self) -> &f32;
    fn expected_travel_time_base_mut(&mut self) -> &mut f32;
    fn interact_area_time(&self) -> &f32;
    fn interact_area_time_mut(&mut self) -> &mut f32;
    fn debug_highlight_objective_type(&self) -> &i32;
    fn debug_highlight_objective_type_mut(&mut self) -> &mut i32;
    fn seek_and_destroy_min_radius(&self) -> &f32;
    fn seek_and_destroy_min_radius_mut(&mut self) -> &mut f32;
    fn seek_and_destroy_max_radius(&self) -> &f32;
    fn seek_and_destroy_max_radius_mut(&mut self) -> &mut f32;
    fn force_repath_if_too_far_from_waypoint_meters(&self) -> &f32;
    fn force_repath_if_too_far_from_waypoint_meters_mut(&mut self) -> &mut f32;
    fn waypoint_minimum_progress_meters(&self) -> &f32;
    fn waypoint_minimum_progress_meters_mut(&mut self) -> &mut f32;
    fn debug_draw_aim_noise(&self) -> &bool;
    fn debug_draw_aim_noise_mut(&mut self) -> &mut bool;
    fn aim_noise_scale(&self) -> &f32;
    fn aim_noise_scale_mut(&mut self) -> &mut f32;
    fn target_min_switch_time_s(&self) -> &f32;
    fn target_min_switch_time_s_mut(&mut self) -> &mut f32;
    fn max_target_engaging_distance_scale(&self) -> &f32;
    fn max_target_engaging_distance_scale_mut(&mut self) -> &mut f32;
    fn allow_random_path_spread(&self) -> &bool;
    fn allow_random_path_spread_mut(&mut self) -> &mut bool;
    fn force_use_random_path_spread(&self) -> &bool;
    fn force_use_random_path_spread_mut(&mut self) -> &mut bool;
    fn random_path_spread_radius(&self) -> &f32;
    fn random_path_spread_radius_mut(&mut self) -> &mut f32;
    fn random_path_spread_center_distance(&self) -> &f32;
    fn random_path_spread_center_distance_mut(&mut self) -> &mut f32;
    fn update_target_cooldown(&self) -> &f32;
    fn update_target_cooldown_mut(&mut self) -> &mut f32;
    fn forced_target_timeout_seconds(&self) -> &f32;
    fn forced_target_timeout_seconds_mut(&mut self) -> &mut f32;
    fn debug_draw_players_names_and_ids(&self) -> &bool;
    fn debug_draw_players_names_and_ids_mut(&mut self) -> &mut bool;
    fn verbose_logging(&self) -> &bool;
    fn verbose_logging_mut(&mut self) -> &mut bool;
    fn action_objective_default_time(&self) -> &f32;
    fn action_objective_default_time_mut(&mut self) -> &mut f32;
    fn allow_action_gadget(&self) -> &bool;
    fn allow_action_gadget_mut(&mut self) -> &mut bool;
    fn action_gadget_probability(&self) -> &f32;
    fn action_gadget_probability_mut(&mut self) -> &mut f32;
    fn action_gadget_interactable_search_radius(&self) -> &f32;
    fn action_gadget_interactable_search_radius_mut(&mut self) -> &mut f32;
    fn hero_spawn_probability_gameplay(&self) -> &f32;
    fn hero_spawn_probability_gameplay_mut(&mut self) -> &mut f32;
    fn special_spawn_probability_gameplay(&self) -> &f32;
    fn special_spawn_probability_gameplay_mut(&mut self) -> &mut f32;
    fn hero_vehicle_spawn_probability_gameplay(&self) -> &f32;
    fn hero_vehicle_spawn_probability_gameplay_mut(&mut self) -> &mut f32;
    fn vehicle_spawn_probability_gameplay(&self) -> &f32;
    fn vehicle_spawn_probability_gameplay_mut(&mut self) -> &mut f32;
    fn hero_spawn_probability(&self) -> &f32;
    fn hero_spawn_probability_mut(&mut self) -> &mut f32;
    fn special_spawn_probability(&self) -> &f32;
    fn special_spawn_probability_mut(&mut self) -> &mut f32;
    fn hero_vehicle_spawn_probability(&self) -> &f32;
    fn hero_vehicle_spawn_probability_mut(&mut self) -> &mut f32;
    fn vehicle_spawn_probability(&self) -> &f32;
    fn vehicle_spawn_probability_mut(&mut self) -> &mut f32;
    fn follow_target_position_check_cooldown(&self) -> &f32;
    fn follow_target_position_check_cooldown_mut(&mut self) -> &mut f32;
    fn not_alive_assert_time(&self) -> &f32;
    fn not_alive_assert_time_mut(&mut self) -> &mut f32;
    fn prefer_f_p_s_camera(&self) -> &bool;
    fn prefer_f_p_s_camera_mut(&mut self) -> &mut bool;
    fn time_on_path_tolerance_seconds(&self) -> &f32;
    fn time_on_path_tolerance_seconds_mut(&mut self) -> &mut f32;
    fn check_water_depth_for_intermediate_positions(&self) -> &bool;
    fn check_water_depth_for_intermediate_positions_mut(&mut self) -> &mut bool;
    fn swimming_suicide_timeout(&self) -> &f32;
    fn swimming_suicide_timeout_mut(&mut self) -> &mut f32;
    fn lof_prediction_time(&self) -> &f32;
    fn lof_prediction_time_mut(&mut self) -> &mut f32;
    fn debug_draw_combat_raycast_hit_points(&self) -> &bool;
    fn debug_draw_combat_raycast_hit_points_mut(&mut self) -> &mut bool;
    fn debug_draw_transforms(&self) -> &bool;
    fn debug_draw_transforms_mut(&mut self) -> &mut bool;
    fn target_tracker_field_of_view_degrees(&self) -> &f32;
    fn target_tracker_field_of_view_degrees_mut(&mut self) -> &mut f32;
    fn pick_random_vehicle_on_secondary_objective(&self) -> &bool;
    fn pick_random_vehicle_on_secondary_objective_mut(&mut self) -> &mut bool;
    fn never_exit_vehicle_after_entering(&self) -> &bool;
    fn never_exit_vehicle_after_entering_mut(&mut self) -> &mut bool;
    fn update_target_per_frame_cap(&self) -> &u32;
    fn update_target_per_frame_cap_mut(&mut self) -> &mut u32;
    fn replay_telemetry_file(&self) -> &String;
    fn replay_telemetry_file_mut(&mut self) -> &mut String;
    fn replay_telemetry_file_format(&self) -> &String;
    fn replay_telemetry_file_format_mut(&mut self) -> &mut String;
    fn replay_telemetry_adjust_time(&self) -> &bool;
    fn replay_telemetry_adjust_time_mut(&mut self) -> &mut bool;
    fn replay_telemetry_adjust_time_padding(&self) -> &f32;
    fn replay_telemetry_adjust_time_padding_mut(&mut self) -> &mut f32;
    fn debug_draw_weapon_details(&self) -> &bool;
    fn debug_draw_weapon_details_mut(&mut self) -> &mut bool;
    fn debug_draw_extensive_client_details(&self) -> &bool;
    fn debug_draw_extensive_client_details_mut(&mut self) -> &mut bool;
    fn evasive_maneuvers_jump_probability(&self) -> &f32;
    fn evasive_maneuvers_jump_probability_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_dodge_roll_probability(&self) -> &f32;
    fn evasive_maneuvers_dodge_roll_probability_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_invert_strafe_duration_max(&self) -> &f32;
    fn evasive_maneuvers_invert_strafe_duration_max_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_invert_strafe_duration_min(&self) -> &f32;
    fn evasive_maneuvers_invert_strafe_duration_min_mut(&mut self) -> &mut f32;
    fn leg_head_aim_ratio_override(&self) -> &f32;
    fn leg_head_aim_ratio_override_mut(&mut self) -> &mut f32;
    fn attacking_ability_left_probability(&self) -> &f32;
    fn attacking_ability_left_probability_mut(&mut self) -> &mut f32;
    fn attacking_ability_left_duration_seconds(&self) -> &f32;
    fn attacking_ability_left_duration_seconds_mut(&mut self) -> &mut f32;
    fn attacking_ability_middle_probability(&self) -> &f32;
    fn attacking_ability_middle_probability_mut(&mut self) -> &mut f32;
    fn attacking_ability_middle_duration_seconds(&self) -> &f32;
    fn attacking_ability_middle_duration_seconds_mut(&mut self) -> &mut f32;
    fn attacking_ability_right_probability(&self) -> &f32;
    fn attacking_ability_right_probability_mut(&mut self) -> &mut f32;
    fn attacking_ability_right_duration_seconds(&self) -> &f32;
    fn attacking_ability_right_duration_seconds_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_crouch_probability(&self) -> &f32;
    fn evasive_maneuvers_crouch_probability_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_crouch_duration(&self) -> &f32;
    fn evasive_maneuvers_crouch_duration_mut(&mut self) -> &mut f32;
    fn blaster_leg_head_aim_ratio(&self) -> &f32;
    fn blaster_leg_head_aim_ratio_mut(&mut self) -> &mut f32;
    fn blaster_aim_noise(&self) -> &f32;
    fn blaster_aim_noise_mut(&mut self) -> &mut f32;
    fn sniper_rifle_leg_head_aim_ratio(&self) -> &f32;
    fn sniper_rifle_leg_head_aim_ratio_mut(&mut self) -> &mut f32;
    fn sniper_rifle_aim_noise(&self) -> &f32;
    fn sniper_rifle_aim_noise_mut(&mut self) -> &mut f32;
    fn lmg_leg_head_aim_ratio(&self) -> &f32;
    fn lmg_leg_head_aim_ratio_mut(&mut self) -> &mut f32;
    fn lmg_aim_noise(&self) -> &f32;
    fn lmg_aim_noise_mut(&mut self) -> &mut f32;
    fn shotgun_leg_head_aim_ratio(&self) -> &f32;
    fn shotgun_leg_head_aim_ratio_mut(&mut self) -> &mut f32;
    fn shotgun_aim_noise(&self) -> &f32;
    fn shotgun_aim_noise_mut(&mut self) -> &mut f32;
    fn launcher_leg_head_aim_ratio(&self) -> &f32;
    fn launcher_leg_head_aim_ratio_mut(&mut self) -> &mut f32;
    fn launcher_aim_noise(&self) -> &f32;
    fn launcher_aim_noise_mut(&mut self) -> &mut f32;
    fn use_sword_attacking_abilities_from_meters(&self) -> &f32;
    fn use_sword_attacking_abilities_from_meters_mut(&mut self) -> &mut f32;
    fn sword_attack_duration_time_min_s(&self) -> &f32;
    fn sword_attack_duration_time_min_s_mut(&mut self) -> &mut f32;
    fn sword_attack_duration_time_max_s(&self) -> &f32;
    fn sword_attack_duration_time_max_s_mut(&mut self) -> &mut f32;
    fn pause_sword_attack_duration_time_min_s(&self) -> &f32;
    fn pause_sword_attack_duration_time_min_s_mut(&mut self) -> &mut f32;
    fn pause_sword_attack_duration_time_max_s(&self) -> &f32;
    fn pause_sword_attack_duration_time_max_s_mut(&mut self) -> &mut f32;
    fn sword_attack_distance_meters_min(&self) -> &f32;
    fn sword_attack_distance_meters_min_mut(&mut self) -> &mut f32;
    fn sword_attack_distance_meters_max(&self) -> &f32;
    fn sword_attack_distance_meters_max_mut(&mut self) -> &mut f32;
    fn debug_draw_input_details(&self) -> &bool;
    fn debug_draw_input_details_mut(&mut self) -> &mut bool;
    fn debug_window_position_scale_offset_x(&self) -> &f32;
    fn debug_window_position_scale_offset_x_mut(&mut self) -> &mut f32;
    fn debug_window_position_scale_offset_y(&self) -> &f32;
    fn debug_window_position_scale_offset_y_mut(&mut self) -> &mut f32;
    fn debug_window_width(&self) -> &i32;
    fn debug_window_width_mut(&mut self) -> &mut i32;
    fn debug_window_height(&self) -> &i32;
    fn debug_window_height_mut(&mut self) -> &mut i32;
    fn path_look_ahead_meters(&self) -> &f32;
    fn path_look_ahead_meters_mut(&mut self) -> &mut f32;
    fn path_look_right_meters(&self) -> &f32;
    fn path_look_right_meters_mut(&mut self) -> &mut f32;
    fn waypoint_tolerance_meters(&self) -> &f32;
    fn waypoint_tolerance_meters_mut(&mut self) -> &mut f32;
    fn debug_draw_aim_at_positions(&self) -> &bool;
    fn debug_draw_aim_at_positions_mut(&mut self) -> &mut bool;
    fn evasive_maneuvers_vehicle_scale(&self) -> &f32;
    fn evasive_maneuvers_vehicle_scale_mut(&mut self) -> &mut f32;
    fn vehicle_aim_noise_scale(&self) -> &f32;
    fn vehicle_aim_noise_scale_mut(&mut self) -> &mut f32;
    fn reset_settings_on_level_unload(&self) -> &bool;
    fn reset_settings_on_level_unload_mut(&mut self) -> &mut bool;
    fn sword_guard_duration_time_min_s(&self) -> &f32;
    fn sword_guard_duration_time_min_s_mut(&mut self) -> &mut f32;
    fn sword_guard_duration_time_max_s(&self) -> &f32;
    fn sword_guard_duration_time_max_s_mut(&mut self) -> &mut f32;
    fn aim_noise_scale_team1(&self) -> &f32;
    fn aim_noise_scale_team1_mut(&mut self) -> &mut f32;
    fn aim_noise_scale_team2(&self) -> &f32;
    fn aim_noise_scale_team2_mut(&mut self) -> &mut f32;
    fn hero_strafe_probability_per_frame(&self) -> &f32;
    fn hero_strafe_probability_per_frame_mut(&mut self) -> &mut f32;
    fn emote_probability_after_players_death(&self) -> &f32;
    fn emote_probability_after_players_death_mut(&mut self) -> &mut f32;
    fn emote_duration(&self) -> &f32;
    fn emote_duration_mut(&mut self) -> &mut f32;
    fn melee_interval_s(&self) -> &f32;
    fn melee_interval_s_mut(&mut self) -> &mut f32;
    fn melee_distance_m(&self) -> &f32;
    fn melee_distance_m_mut(&mut self) -> &mut f32;
    fn allow_evasive_manouvers_o_o_b(&self) -> &bool;
    fn allow_evasive_manouvers_o_o_b_mut(&mut self) -> &mut bool;
    fn evasive_maneuvers_ground_check_enabled(&self) -> &bool;
    fn evasive_maneuvers_ground_check_enabled_mut(&mut self) -> &mut bool;
    fn evasive_maneuvers_ground_check_distance_m(&self) -> &f32;
    fn evasive_maneuvers_ground_check_distance_m_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_ground_check_height_distance_m(&self) -> &f32;
    fn evasive_maneuvers_ground_check_height_distance_m_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_ground_check_height_offset_m(&self) -> &f32;
    fn evasive_maneuvers_ground_check_height_offset_m_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_ground_check_cooldown_s(&self) -> &f32;
    fn evasive_maneuvers_ground_check_cooldown_s_mut(&mut self) -> &mut f32;
    fn evasive_maneuvers_vehicles_enabled(&self) -> &bool;
    fn evasive_maneuvers_vehicles_enabled_mut(&mut self) -> &mut bool;
    fn vehicle_minimum_forward_throttle(&self) -> &f32;
    fn vehicle_minimum_forward_throttle_mut(&mut self) -> &mut f32;
    fn vehicle_use_character_throttle(&self) -> &bool;
    fn vehicle_use_character_throttle_mut(&mut self) -> &mut bool;
}

impl AutoPlayerSettingsTrait for AutoPlayerSettings {
    fn a_f_k_takeover(&self) -> &f32 {
        &self.a_f_k_takeover
    }
    fn a_f_k_takeover_mut(&mut self) -> &mut f32 {
        &mut self.a_f_k_takeover
    }
    fn client_enabled(&self) -> &bool {
        &self.client_enabled
    }
    fn client_enabled_mut(&mut self) -> &mut bool {
        &mut self.client_enabled
    }
    fn allow_client_take_over(&self) -> &bool {
        &self.allow_client_take_over
    }
    fn allow_client_take_over_mut(&mut self) -> &mut bool {
        &mut self.allow_client_take_over
    }
    fn force_server_control(&self) -> &bool {
        &self.force_server_control
    }
    fn force_server_control_mut(&mut self) -> &mut bool {
        &mut self.force_server_control
    }
    fn force_server_objective_control(&self) -> &bool {
        &self.force_server_objective_control
    }
    fn force_server_objective_control_mut(&mut self) -> &mut bool {
        &mut self.force_server_objective_control
    }
    fn force_client_objective_control(&self) -> &bool {
        &self.force_client_objective_control
    }
    fn force_client_objective_control_mut(&mut self) -> &mut bool {
        &mut self.force_client_objective_control
    }
    fn force_client_navigation(&self) -> &bool {
        &self.force_client_navigation
    }
    fn force_client_navigation_mut(&mut self) -> &mut bool {
        &mut self.force_client_navigation
    }
    fn debug_draw_enabled(&self) -> &bool {
        &self.debug_draw_enabled
    }
    fn debug_draw_enabled_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_enabled
    }
    fn debug_draw_waypoints(&self) -> &bool {
        &self.debug_draw_waypoints
    }
    fn debug_draw_waypoints_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_waypoints
    }
    fn debug_draw_client_details(&self) -> &bool {
        &self.debug_draw_client_details
    }
    fn debug_draw_client_details_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_client_details
    }
    fn debug_draw_combat_details(&self) -> &bool {
        &self.debug_draw_combat_details
    }
    fn debug_draw_combat_details_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_combat_details
    }
    fn player_count(&self) -> &i32 {
        &self.player_count
    }
    fn player_count_mut(&mut self) -> &mut i32 {
        &mut self.player_count
    }
    fn forced_server_auto_player_count(&self) -> &i32 {
        &self.forced_server_auto_player_count
    }
    fn forced_server_auto_player_count_mut(&mut self) -> &mut i32 {
        &mut self.forced_server_auto_player_count
    }
    fn allow_add_auto_fill_players(&self) -> &bool {
        &self.allow_add_auto_fill_players
    }
    fn allow_add_auto_fill_players_mut(&mut self) -> &mut bool {
        &mut self.allow_add_auto_fill_players
    }
    fn allow_remove_auto_fill_players(&self) -> &bool {
        &self.allow_remove_auto_fill_players
    }
    fn allow_remove_auto_fill_players_mut(&mut self) -> &mut bool {
        &mut self.allow_remove_auto_fill_players
    }
    fn force_apply_gameplay_bots_count(&self) -> &bool {
        &self.force_apply_gameplay_bots_count
    }
    fn force_apply_gameplay_bots_count_mut(&mut self) -> &mut bool {
        &mut self.force_apply_gameplay_bots_count
    }
    fn force_fill_gameplay_bots_team1(&self) -> &i32 {
        &self.force_fill_gameplay_bots_team1
    }
    fn force_fill_gameplay_bots_team1_mut(&mut self) -> &mut i32 {
        &mut self.force_fill_gameplay_bots_team1
    }
    fn force_fill_gameplay_bots_team2(&self) -> &i32 {
        &self.force_fill_gameplay_bots_team2
    }
    fn force_fill_gameplay_bots_team2_mut(&mut self) -> &mut i32 {
        &mut self.force_fill_gameplay_bots_team2
    }
    fn respawn_delay(&self) -> &f32 {
        &self.respawn_delay
    }
    fn respawn_delay_mut(&mut self) -> &mut f32 {
        &mut self.respawn_delay
    }
    fn initial_respawn_delay(&self) -> &f32 {
        &self.initial_respawn_delay
    }
    fn initial_respawn_delay_mut(&mut self) -> &mut f32 {
        &mut self.initial_respawn_delay
    }
    fn client_join_delay(&self) -> &f32 {
        &self.client_join_delay
    }
    fn client_join_delay_mut(&mut self) -> &mut f32 {
        &mut self.client_join_delay
    }
    fn round_timeout(&self) -> &i32 {
        &self.round_timeout
    }
    fn round_timeout_mut(&mut self) -> &mut i32 {
        &mut self.round_timeout
    }
    fn squad_members(&self) -> &i32 {
        &self.squad_members
    }
    fn squad_members_mut(&mut self) -> &mut i32 {
        &mut self.squad_members
    }
    fn allow_gameplay_bots_to_join_player_squads(&self) -> &bool {
        &self.allow_gameplay_bots_to_join_player_squads
    }
    fn allow_gameplay_bots_to_join_player_squads_mut(&mut self) -> &mut bool {
        &mut self.allow_gameplay_bots_to_join_player_squads
    }
    fn allow_gameplay_bots_to_form_own_squads(&self) -> &bool {
        &self.allow_gameplay_bots_to_form_own_squads
    }
    fn allow_gameplay_bots_to_form_own_squads_mut(&mut self) -> &mut bool {
        &mut self.allow_gameplay_bots_to_form_own_squads
    }
    fn allow_vehicle_spawn(&self) -> &bool {
        &self.allow_vehicle_spawn
    }
    fn allow_vehicle_spawn_mut(&mut self) -> &mut bool {
        &mut self.allow_vehicle_spawn
    }
    fn force_disable_vehicle_spawn(&self) -> &bool {
        &self.force_disable_vehicle_spawn
    }
    fn force_disable_vehicle_spawn_mut(&mut self) -> &mut bool {
        &mut self.force_disable_vehicle_spawn
    }
    fn allow_client_vehicle_spawn(&self) -> &bool {
        &self.allow_client_vehicle_spawn
    }
    fn allow_client_vehicle_spawn_mut(&mut self) -> &mut bool {
        &mut self.allow_client_vehicle_spawn
    }
    fn allow_first_client_initial_vehicle_spawn(&self) -> &bool {
        &self.allow_first_client_initial_vehicle_spawn
    }
    fn allow_first_client_initial_vehicle_spawn_mut(&mut self) -> &mut bool {
        &mut self.allow_first_client_initial_vehicle_spawn
    }
    fn control_connectionless_players(&self) -> &bool {
        &self.control_connectionless_players
    }
    fn control_connectionless_players_mut(&mut self) -> &mut bool {
        &mut self.control_connectionless_players
    }
    fn allow_respawn(&self) -> &bool {
        &self.allow_respawn
    }
    fn allow_respawn_mut(&mut self) -> &mut bool {
        &mut self.allow_respawn
    }
    fn pickup_items_secondary_objective_attempt_interval_seconds(&self) -> &i32 {
        &self.pickup_items_secondary_objective_attempt_interval_seconds
    }
    fn pickup_items_secondary_objective_attempt_interval_seconds_mut(&mut self) -> &mut i32 {
        &mut self.pickup_items_secondary_objective_attempt_interval_seconds
    }
    fn use_telemetry_based_planner(&self) -> &bool {
        &self.use_telemetry_based_planner
    }
    fn use_telemetry_based_planner_mut(&mut self) -> &mut bool {
        &mut self.use_telemetry_based_planner
    }
    fn debug_telemetry_based_planner(&self) -> &bool {
        &self.debug_telemetry_based_planner
    }
    fn debug_telemetry_based_planner_mut(&mut self) -> &mut bool {
        &mut self.debug_telemetry_based_planner
    }
    fn planner_terrain_vertical_cutoff(&self) -> &f32 {
        &self.planner_terrain_vertical_cutoff
    }
    fn planner_terrain_vertical_cutoff_mut(&mut self) -> &mut f32 {
        &mut self.planner_terrain_vertical_cutoff
    }
    fn planner_connection_cutoff(&self) -> &f32 {
        &self.planner_connection_cutoff
    }
    fn planner_connection_cutoff_mut(&mut self) -> &mut f32 {
        &mut self.planner_connection_cutoff
    }
    fn planner_max_nodes_search_radius(&self) -> &f32 {
        &self.planner_max_nodes_search_radius
    }
    fn planner_max_nodes_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.planner_max_nodes_search_radius
    }
    fn planner_link_end_arrival_range(&self) -> &f32 {
        &self.planner_link_end_arrival_range
    }
    fn planner_link_end_arrival_range_mut(&mut self) -> &mut f32 {
        &mut self.planner_link_end_arrival_range
    }
    fn use_fade_override(&self) -> &bool {
        &self.use_fade_override
    }
    fn use_fade_override_mut(&mut self) -> &mut bool {
        &mut self.use_fade_override
    }
    fn input_scale_yaw(&self) -> &f32 {
        &self.input_scale_yaw
    }
    fn input_scale_yaw_mut(&mut self) -> &mut f32 {
        &mut self.input_scale_yaw
    }
    fn input_scale_pitch(&self) -> &f32 {
        &self.input_scale_pitch
    }
    fn input_scale_pitch_mut(&mut self) -> &mut f32 {
        &mut self.input_scale_pitch
    }
    fn input_scale_client(&self) -> &f32 {
        &self.input_scale_client
    }
    fn input_scale_client_mut(&mut self) -> &mut f32 {
        &mut self.input_scale_client
    }
    fn input_force_mouse(&self) -> &bool {
        &self.input_force_mouse
    }
    fn input_force_mouse_mut(&mut self) -> &mut bool {
        &mut self.input_force_mouse
    }
    fn use_input_override_yaw_pitch(&self) -> &bool {
        &self.use_input_override_yaw_pitch
    }
    fn use_input_override_yaw_pitch_mut(&mut self) -> &mut bool {
        &mut self.use_input_override_yaw_pitch
    }
    fn input_override_yaw(&self) -> &f32 {
        &self.input_override_yaw
    }
    fn input_override_yaw_mut(&mut self) -> &mut f32 {
        &mut self.input_override_yaw
    }
    fn input_override_pitch(&self) -> &f32 {
        &self.input_override_pitch
    }
    fn input_override_pitch_mut(&mut self) -> &mut f32 {
        &mut self.input_override_pitch
    }
    fn use_seek_and_destroy(&self) -> &bool {
        &self.use_seek_and_destroy
    }
    fn use_seek_and_destroy_mut(&mut self) -> &mut bool {
        &mut self.use_seek_and_destroy
    }
    fn allow_teleport(&self) -> &bool {
        &self.allow_teleport
    }
    fn allow_teleport_mut(&mut self) -> &mut bool {
        &mut self.allow_teleport
    }
    fn force_allow_all_teleports(&self) -> &bool {
        &self.force_allow_all_teleports
    }
    fn force_allow_all_teleports_mut(&mut self) -> &mut bool {
        &mut self.force_allow_all_teleports
    }
    fn debug_draw_teleports(&self) -> &bool {
        &self.debug_draw_teleports
    }
    fn debug_draw_teleports_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_teleports
    }
    fn update_a_i(&self) -> &bool {
        &self.update_a_i
    }
    fn update_a_i_mut(&mut self) -> &mut bool {
        &mut self.update_a_i
    }
    fn debug_draw_client_only(&self) -> &bool {
        &self.debug_draw_client_only
    }
    fn debug_draw_client_only_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_client_only
    }
    fn debug_draw_client_realm_only(&self) -> &bool {
        &self.debug_draw_client_realm_only
    }
    fn debug_draw_client_realm_only_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_client_realm_only
    }
    fn aim_acceleration(&self) -> &f32 {
        &self.aim_acceleration
    }
    fn aim_acceleration_mut(&mut self) -> &mut f32 {
        &mut self.aim_acceleration
    }
    fn aim_lap_time(&self) -> &f32 {
        &self.aim_lap_time
    }
    fn aim_lap_time_mut(&mut self) -> &mut f32 {
        &mut self.aim_lap_time
    }
    fn allow_move_outside_combat_area(&self) -> &bool {
        &self.allow_move_outside_combat_area
    }
    fn allow_move_outside_combat_area_mut(&mut self) -> &mut bool {
        &mut self.allow_move_outside_combat_area
    }
    fn allow_spawn_outside_combat_area(&self) -> &bool {
        &self.allow_spawn_outside_combat_area
    }
    fn allow_spawn_outside_combat_area_mut(&mut self) -> &mut bool {
        &mut self.allow_spawn_outside_combat_area
    }
    fn allow_vehicle_spawn_outside_combat_area(&self) -> &bool {
        &self.allow_vehicle_spawn_outside_combat_area
    }
    fn allow_vehicle_spawn_outside_combat_area_mut(&mut self) -> &mut bool {
        &mut self.allow_vehicle_spawn_outside_combat_area
    }
    fn allow_vehicle_spawn_only(&self) -> &bool {
        &self.allow_vehicle_spawn_only
    }
    fn allow_vehicle_spawn_only_mut(&mut self) -> &mut bool {
        &mut self.allow_vehicle_spawn_only
    }
    fn debug_draw_pretty_path(&self) -> &bool {
        &self.debug_draw_pretty_path
    }
    fn debug_draw_pretty_path_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_pretty_path
    }
    fn debug_draw_use_waypoints_alpha(&self) -> &bool {
        &self.debug_draw_use_waypoints_alpha
    }
    fn debug_draw_use_waypoints_alpha_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_use_waypoints_alpha
    }
    fn debug_draw_invalid_move_intention(&self) -> &bool {
        &self.debug_draw_invalid_move_intention
    }
    fn debug_draw_invalid_move_intention_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_invalid_move_intention
    }
    fn debug_spam(&self) -> &bool {
        &self.debug_spam
    }
    fn debug_spam_mut(&mut self) -> &mut bool {
        &mut self.debug_spam
    }
    fn lof_timeout_s(&self) -> &f32 {
        &self.lof_timeout_s
    }
    fn lof_timeout_s_mut(&mut self) -> &mut f32 {
        &mut self.lof_timeout_s
    }
    fn lof_reaction_time_s(&self) -> &f32 {
        &self.lof_reaction_time_s
    }
    fn lof_reaction_time_s_mut(&mut self) -> &mut f32 {
        &mut self.lof_reaction_time_s
    }
    fn server_players_ignore_client_players(&self) -> &bool {
        &self.server_players_ignore_client_players
    }
    fn server_players_ignore_client_players_mut(&mut self) -> &mut bool {
        &mut self.server_players_ignore_client_players
    }
    fn ignore_human_players(&self) -> &bool {
        &self.ignore_human_players
    }
    fn ignore_human_players_mut(&mut self) -> &mut bool {
        &mut self.ignore_human_players
    }
    fn force_kit(&self) -> &i32 {
        &self.force_kit
    }
    fn force_kit_mut(&mut self) -> &mut i32 {
        &mut self.force_kit
    }
    fn opportunistic_interact(&self) -> &bool {
        &self.opportunistic_interact
    }
    fn opportunistic_interact_mut(&mut self) -> &mut bool {
        &mut self.opportunistic_interact
    }
    fn squad_spawn_probability(&self) -> &f32 {
        &self.squad_spawn_probability
    }
    fn squad_spawn_probability_mut(&mut self) -> &mut f32 {
        &mut self.squad_spawn_probability
    }
    fn kit_change_probability(&self) -> &f32 {
        &self.kit_change_probability
    }
    fn kit_change_probability_mut(&mut self) -> &mut f32 {
        &mut self.kit_change_probability
    }
    fn use_default_unlocks_probability(&self) -> &f32 {
        &self.use_default_unlocks_probability
    }
    fn use_default_unlocks_probability_mut(&mut self) -> &mut f32 {
        &mut self.use_default_unlocks_probability
    }
    fn allow_medic_revive(&self) -> &bool {
        &self.allow_medic_revive
    }
    fn allow_medic_revive_mut(&mut self) -> &mut bool {
        &mut self.allow_medic_revive
    }
    fn allow_pickup_items(&self) -> &bool {
        &self.allow_pickup_items
    }
    fn allow_pickup_items_mut(&mut self) -> &mut bool {
        &mut self.allow_pickup_items
    }
    fn debug_draw_objectives(&self) -> &bool {
        &self.debug_draw_objectives
    }
    fn debug_draw_objectives_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_objectives
    }
    fn debug_draw_objective_always(&self) -> &bool {
        &self.debug_draw_objective_always
    }
    fn debug_draw_objective_always_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_objective_always
    }
    fn wallhack(&self) -> &bool {
        &self.wallhack
    }
    fn wallhack_mut(&mut self) -> &mut bool {
        &mut self.wallhack
    }
    fn weapon_swap_interval_s(&self) -> &f32 {
        &self.weapon_swap_interval_s
    }
    fn weapon_swap_interval_s_mut(&mut self) -> &mut f32 {
        &mut self.weapon_swap_interval_s
    }
    fn weapon_swap_primary_probability(&self) -> &f32 {
        &self.weapon_swap_primary_probability
    }
    fn weapon_swap_primary_probability_mut(&mut self) -> &mut f32 {
        &mut self.weapon_swap_primary_probability
    }
    fn vehicle_bail_time(&self) -> &i32 {
        &self.vehicle_bail_time
    }
    fn vehicle_bail_time_mut(&mut self) -> &mut i32 {
        &mut self.vehicle_bail_time
    }
    fn jump_if_stuck_time_seconds(&self) -> &f32 {
        &self.jump_if_stuck_time_seconds
    }
    fn jump_if_stuck_time_seconds_mut(&mut self) -> &mut f32 {
        &mut self.jump_if_stuck_time_seconds
    }
    fn jump_cooldown_seconds(&self) -> &f32 {
        &self.jump_cooldown_seconds
    }
    fn jump_cooldown_seconds_mut(&mut self) -> &mut f32 {
        &mut self.jump_cooldown_seconds
    }
    fn patrol_position_cooldown_seconds(&self) -> &f32 {
        &self.patrol_position_cooldown_seconds
    }
    fn patrol_position_cooldown_seconds_mut(&mut self) -> &mut f32 {
        &mut self.patrol_position_cooldown_seconds
    }
    fn combat_use_grenades(&self) -> &bool {
        &self.combat_use_grenades
    }
    fn combat_use_grenades_mut(&mut self) -> &mut bool {
        &mut self.combat_use_grenades
    }
    fn combat_use_prone(&self) -> &bool {
        &self.combat_use_prone
    }
    fn combat_use_prone_mut(&mut self) -> &mut bool {
        &mut self.combat_use_prone
    }
    fn combat_use_melee(&self) -> &bool {
        &self.combat_use_melee
    }
    fn combat_use_melee_mut(&mut self) -> &mut bool {
        &mut self.combat_use_melee
    }
    fn use_crouch(&self) -> &bool {
        &self.use_crouch
    }
    fn use_crouch_mut(&mut self) -> &mut bool {
        &mut self.use_crouch
    }
    fn forced_fire_time_max_s(&self) -> &f32 {
        &self.forced_fire_time_max_s
    }
    fn forced_fire_time_max_s_mut(&mut self) -> &mut f32 {
        &mut self.forced_fire_time_max_s
    }
    fn forced_fire_time_min_s(&self) -> &f32 {
        &self.forced_fire_time_min_s
    }
    fn forced_fire_time_min_s_mut(&mut self) -> &mut f32 {
        &mut self.forced_fire_time_min_s
    }
    fn allow_primary_weapon_forced_fire(&self) -> &bool {
        &self.allow_primary_weapon_forced_fire
    }
    fn allow_primary_weapon_forced_fire_mut(&mut self) -> &mut bool {
        &mut self.allow_primary_weapon_forced_fire
    }
    fn allow_vehicle_forced_fire(&self) -> &bool {
        &self.allow_vehicle_forced_fire
    }
    fn allow_vehicle_forced_fire_mut(&mut self) -> &mut bool {
        &mut self.allow_vehicle_forced_fire
    }
    fn forced_fire_vehicle_time_scale(&self) -> &f32 {
        &self.forced_fire_vehicle_time_scale
    }
    fn forced_fire_vehicle_time_scale_mut(&mut self) -> &mut f32 {
        &mut self.forced_fire_vehicle_time_scale
    }
    fn exit_vehicle_when_stuck_timeout(&self) -> &f32 {
        &self.exit_vehicle_when_stuck_timeout
    }
    fn exit_vehicle_when_stuck_timeout_mut(&mut self) -> &mut f32 {
        &mut self.exit_vehicle_when_stuck_timeout
    }
    fn min_distance_for_vehicle_u_turn(&self) -> &f32 {
        &self.min_distance_for_vehicle_u_turn
    }
    fn min_distance_for_vehicle_u_turn_mut(&mut self) -> &mut f32 {
        &mut self.min_distance_for_vehicle_u_turn
    }
    fn min_airplane_bail_out_time(&self) -> &i32 {
        &self.min_airplane_bail_out_time
    }
    fn min_airplane_bail_out_time_mut(&mut self) -> &mut i32 {
        &mut self.min_airplane_bail_out_time
    }
    fn max_airplane_bail_out_time(&self) -> &i32 {
        &self.max_airplane_bail_out_time
    }
    fn max_airplane_bail_out_time_mut(&mut self) -> &mut i32 {
        &mut self.max_airplane_bail_out_time
    }
    fn login_rate(&self) -> &f32 {
        &self.login_rate
    }
    fn login_rate_mut(&mut self) -> &mut f32 {
        &mut self.login_rate
    }
    fn spawn_rate(&self) -> &f32 {
        &self.spawn_rate
    }
    fn spawn_rate_mut(&mut self) -> &mut f32 {
        &mut self.spawn_rate
    }
    fn max_spawns_per_update(&self) -> &i32 {
        &self.max_spawns_per_update
    }
    fn max_spawns_per_update_mut(&mut self) -> &mut i32 {
        &mut self.max_spawns_per_update
    }
    fn variance(&self) -> &f32 {
        &self.variance
    }
    fn variance_mut(&mut self) -> &mut f32 {
        &mut self.variance
    }
    fn airplane_exit_input(&self) -> &i32 {
        &self.airplane_exit_input
    }
    fn airplane_exit_input_mut(&mut self) -> &mut i32 {
        &mut self.airplane_exit_input
    }
    fn secondary_objective_generation_min_seconds(&self) -> &f32 {
        &self.secondary_objective_generation_min_seconds
    }
    fn secondary_objective_generation_min_seconds_mut(&mut self) -> &mut f32 {
        &mut self.secondary_objective_generation_min_seconds
    }
    fn secondary_objective_generation_max_seconds(&self) -> &f32 {
        &self.secondary_objective_generation_max_seconds
    }
    fn secondary_objective_generation_max_seconds_mut(&mut self) -> &mut f32 {
        &mut self.secondary_objective_generation_max_seconds
    }
    fn allow_enter_vehicle(&self) -> &bool {
        &self.allow_enter_vehicle
    }
    fn allow_enter_vehicle_mut(&mut self) -> &mut bool {
        &mut self.allow_enter_vehicle
    }
    fn enter_vehicle_cooldown_seconds(&self) -> &f32 {
        &self.enter_vehicle_cooldown_seconds
    }
    fn enter_vehicle_cooldown_seconds_mut(&mut self) -> &mut f32 {
        &mut self.enter_vehicle_cooldown_seconds
    }
    fn enter_vehicle_probability(&self) -> &f32 {
        &self.enter_vehicle_probability
    }
    fn enter_vehicle_probability_mut(&mut self) -> &mut f32 {
        &mut self.enter_vehicle_probability
    }
    fn enter_vehicle_search_radius(&self) -> &f32 {
        &self.enter_vehicle_search_radius
    }
    fn enter_vehicle_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.enter_vehicle_search_radius
    }
    fn print_client_input(&self) -> &bool {
        &self.print_client_input
    }
    fn print_client_input_mut(&mut self) -> &mut bool {
        &mut self.print_client_input
    }
    fn allow_primary_objective(&self) -> &bool {
        &self.allow_primary_objective
    }
    fn allow_primary_objective_mut(&mut self) -> &mut bool {
        &mut self.allow_primary_objective
    }
    fn allow_secondary_objectives_while_passive(&self) -> &bool {
        &self.allow_secondary_objectives_while_passive
    }
    fn allow_secondary_objectives_while_passive_mut(&mut self) -> &mut bool {
        &mut self.allow_secondary_objectives_while_passive
    }
    fn allow_secondary_objectives_while_defensive(&self) -> &bool {
        &self.allow_secondary_objectives_while_defensive
    }
    fn allow_secondary_objectives_while_defensive_mut(&mut self) -> &mut bool {
        &mut self.allow_secondary_objectives_while_defensive
    }
    fn allow_pathfinding(&self) -> &bool {
        &self.allow_pathfinding
    }
    fn allow_pathfinding_mut(&mut self) -> &mut bool {
        &mut self.allow_pathfinding
    }
    fn secondary_objective_timeout_seconds(&self) -> &f32 {
        &self.secondary_objective_timeout_seconds
    }
    fn secondary_objective_timeout_seconds_mut(&mut self) -> &mut f32 {
        &mut self.secondary_objective_timeout_seconds
    }
    fn force_passive_mode(&self) -> &bool {
        &self.force_passive_mode
    }
    fn force_passive_mode_mut(&mut self) -> &mut bool {
        &mut self.force_passive_mode
    }
    fn force_primary_objective_defensive_mode(&self) -> &bool {
        &self.force_primary_objective_defensive_mode
    }
    fn force_primary_objective_defensive_mode_mut(&mut self) -> &mut bool {
        &mut self.force_primary_objective_defensive_mode
    }
    fn force_primary_objective_aggressive_mode(&self) -> &bool {
        &self.force_primary_objective_aggressive_mode
    }
    fn force_primary_objective_aggressive_mode_mut(&mut self) -> &mut bool {
        &mut self.force_primary_objective_aggressive_mode
    }
    fn force_secondary_objective_defensive_mode(&self) -> &bool {
        &self.force_secondary_objective_defensive_mode
    }
    fn force_secondary_objective_defensive_mode_mut(&mut self) -> &mut bool {
        &mut self.force_secondary_objective_defensive_mode
    }
    fn force_secondary_objective_aggressive_mode(&self) -> &bool {
        &self.force_secondary_objective_aggressive_mode
    }
    fn force_secondary_objective_aggressive_mode_mut(&mut self) -> &mut bool {
        &mut self.force_secondary_objective_aggressive_mode
    }
    fn client_jesus_mode(&self) -> &bool {
        &self.client_jesus_mode
    }
    fn client_jesus_mode_mut(&mut self) -> &mut bool {
        &mut self.client_jesus_mode
    }
    fn allow_fortifications(&self) -> &bool {
        &self.allow_fortifications
    }
    fn allow_fortifications_mut(&mut self) -> &mut bool {
        &mut self.allow_fortifications
    }
    fn fortification_probability(&self) -> &f32 {
        &self.fortification_probability
    }
    fn fortification_probability_mut(&mut self) -> &mut f32 {
        &mut self.fortification_probability
    }
    fn fortification_search_radius(&self) -> &f32 {
        &self.fortification_search_radius
    }
    fn fortification_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.fortification_search_radius
    }
    fn repath_cooldown_seconds(&self) -> &f32 {
        &self.repath_cooldown_seconds
    }
    fn repath_cooldown_seconds_mut(&mut self) -> &mut f32 {
        &mut self.repath_cooldown_seconds
    }
    fn un_stuck_vehicle_actions_trigger_time_seconds(&self) -> &f32 {
        &self.un_stuck_vehicle_actions_trigger_time_seconds
    }
    fn un_stuck_vehicle_actions_trigger_time_seconds_mut(&mut self) -> &mut f32 {
        &mut self.un_stuck_vehicle_actions_trigger_time_seconds
    }
    fn unstuck_minimal_move_distance(&self) -> &f32 {
        &self.unstuck_minimal_move_distance
    }
    fn unstuck_minimal_move_distance_mut(&mut self) -> &mut f32 {
        &mut self.unstuck_minimal_move_distance
    }
    fn unstuck_minimal_move_suicide_timeout(&self) -> &f32 {
        &self.unstuck_minimal_move_suicide_timeout
    }
    fn unstuck_minimal_move_suicide_timeout_mut(&mut self) -> &mut f32 {
        &mut self.unstuck_minimal_move_suicide_timeout
    }
    fn fallen_below_suicide_timeout(&self) -> &f32 {
        &self.fallen_below_suicide_timeout
    }
    fn fallen_below_suicide_timeout_mut(&mut self) -> &mut f32 {
        &mut self.fallen_below_suicide_timeout
    }
    fn navigation_position_tolerance_meters(&self) -> &f32 {
        &self.navigation_position_tolerance_meters
    }
    fn navigation_position_tolerance_meters_mut(&mut self) -> &mut f32 {
        &mut self.navigation_position_tolerance_meters
    }
    fn use_name_generator(&self) -> &bool {
        &self.use_name_generator
    }
    fn use_name_generator_mut(&mut self) -> &mut bool {
        &mut self.use_name_generator
    }
    fn allow_stuck_escape_procedure(&self) -> &bool {
        &self.allow_stuck_escape_procedure
    }
    fn allow_stuck_escape_procedure_mut(&mut self) -> &mut bool {
        &mut self.allow_stuck_escape_procedure
    }
    fn exit_stuck_escape_procedure_on_visual_check(&self) -> &bool {
        &self.exit_stuck_escape_procedure_on_visual_check
    }
    fn exit_stuck_escape_procedure_on_visual_check_mut(&mut self) -> &mut bool {
        &mut self.exit_stuck_escape_procedure_on_visual_check
    }
    fn stuck_escape_procedure_sensor_length(&self) -> &f32 {
        &self.stuck_escape_procedure_sensor_length
    }
    fn stuck_escape_procedure_sensor_length_mut(&mut self) -> &mut f32 {
        &mut self.stuck_escape_procedure_sensor_length
    }
    fn stuck_escape_procedure_p_i_fraction(&self) -> &f32 {
        &self.stuck_escape_procedure_p_i_fraction
    }
    fn stuck_escape_procedure_p_i_fraction_mut(&mut self) -> &mut f32 {
        &mut self.stuck_escape_procedure_p_i_fraction
    }
    fn stuck_escape_procedure_escape_distance(&self) -> &f32 {
        &self.stuck_escape_procedure_escape_distance
    }
    fn stuck_escape_procedure_escape_distance_mut(&mut self) -> &mut f32 {
        &mut self.stuck_escape_procedure_escape_distance
    }
    fn stuck_escape_procedure_activation_seconds(&self) -> &f32 {
        &self.stuck_escape_procedure_activation_seconds
    }
    fn stuck_escape_procedure_activation_seconds_mut(&mut self) -> &mut f32 {
        &mut self.stuck_escape_procedure_activation_seconds
    }
    fn stuck_escape_procedure_update_interval(&self) -> &f32 {
        &self.stuck_escape_procedure_update_interval
    }
    fn stuck_escape_procedure_update_interval_mut(&mut self) -> &mut f32 {
        &mut self.stuck_escape_procedure_update_interval
    }
    fn stuck_escape_procedure_timeout_seconds(&self) -> &f32 {
        &self.stuck_escape_procedure_timeout_seconds
    }
    fn stuck_escape_procedure_timeout_seconds_mut(&mut self) -> &mut f32 {
        &mut self.stuck_escape_procedure_timeout_seconds
    }
    fn debug_draw_unstuck(&self) -> &bool {
        &self.debug_draw_unstuck
    }
    fn debug_draw_unstuck_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_unstuck
    }
    fn un_stuck_actions_trigger_time_seconds(&self) -> &f32 {
        &self.un_stuck_actions_trigger_time_seconds
    }
    fn un_stuck_actions_trigger_time_seconds_mut(&mut self) -> &mut f32 {
        &mut self.un_stuck_actions_trigger_time_seconds
    }
    fn un_stuck_actions_trigger_cooldown(&self) -> &f32 {
        &self.un_stuck_actions_trigger_cooldown
    }
    fn un_stuck_actions_trigger_cooldown_mut(&mut self) -> &mut f32 {
        &mut self.un_stuck_actions_trigger_cooldown
    }
    fn stuck_escape_procedure_retries(&self) -> &i32 {
        &self.stuck_escape_procedure_retries
    }
    fn stuck_escape_procedure_retries_mut(&mut self) -> &mut i32 {
        &mut self.stuck_escape_procedure_retries
    }
    fn primary_interaction_search_radius(&self) -> &f32 {
        &self.primary_interaction_search_radius
    }
    fn primary_interaction_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.primary_interaction_search_radius
    }
    fn allow_suicide(&self) -> &bool {
        &self.allow_suicide
    }
    fn allow_suicide_mut(&mut self) -> &mut bool {
        &mut self.allow_suicide
    }
    fn allow_random_behavior(&self) -> &bool {
        &self.allow_random_behavior
    }
    fn allow_random_behavior_mut(&mut self) -> &mut bool {
        &mut self.allow_random_behavior
    }
    fn allow_secondary_interactions(&self) -> &bool {
        &self.allow_secondary_interactions
    }
    fn allow_secondary_interactions_mut(&mut self) -> &mut bool {
        &mut self.allow_secondary_interactions
    }
    fn secondary_interactions_probability(&self) -> &f32 {
        &self.secondary_interactions_probability
    }
    fn secondary_interactions_probability_mut(&mut self) -> &mut f32 {
        &mut self.secondary_interactions_probability
    }
    fn secondary_interactions_search_radius(&self) -> &f32 {
        &self.secondary_interactions_search_radius
    }
    fn secondary_interactions_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.secondary_interactions_search_radius
    }
    fn secondary_objective_pickup_items_search_radius(&self) -> &f32 {
        &self.secondary_objective_pickup_items_search_radius
    }
    fn secondary_objective_pickup_items_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.secondary_objective_pickup_items_search_radius
    }
    fn secondary_objective_pickup_items_interact_or_action_radius(&self) -> &f32 {
        &self.secondary_objective_pickup_items_interact_or_action_radius
    }
    fn secondary_objective_pickup_items_interact_or_action_radius_mut(&mut self) -> &mut f32 {
        &mut self.secondary_objective_pickup_items_interact_or_action_radius
    }
    fn secondary_objective_jesus_mode(&self) -> &bool {
        &self.secondary_objective_jesus_mode
    }
    fn secondary_objective_jesus_mode_mut(&mut self) -> &mut bool {
        &mut self.secondary_objective_jesus_mode
    }
    fn secondary_revive_search_distance(&self) -> &f32 {
        &self.secondary_revive_search_distance
    }
    fn secondary_revive_search_distance_mut(&mut self) -> &mut f32 {
        &mut self.secondary_revive_search_distance
    }
    fn debug_draw_navigation_details(&self) -> &bool {
        &self.debug_draw_navigation_details
    }
    fn debug_draw_navigation_details_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_navigation_details
    }
    fn debug_draw_navigation_progress_details(&self) -> &bool {
        &self.debug_draw_navigation_progress_details
    }
    fn debug_draw_navigation_progress_details_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_navigation_progress_details
    }
    fn debug_draw_custom_input(&self) -> &bool {
        &self.debug_draw_custom_input
    }
    fn debug_draw_custom_input_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_custom_input
    }
    fn expected_travel_time_distance_scale(&self) -> &f32 {
        &self.expected_travel_time_distance_scale
    }
    fn expected_travel_time_distance_scale_mut(&mut self) -> &mut f32 {
        &mut self.expected_travel_time_distance_scale
    }
    fn expected_travel_time_base(&self) -> &f32 {
        &self.expected_travel_time_base
    }
    fn expected_travel_time_base_mut(&mut self) -> &mut f32 {
        &mut self.expected_travel_time_base
    }
    fn interact_area_time(&self) -> &f32 {
        &self.interact_area_time
    }
    fn interact_area_time_mut(&mut self) -> &mut f32 {
        &mut self.interact_area_time
    }
    fn debug_highlight_objective_type(&self) -> &i32 {
        &self.debug_highlight_objective_type
    }
    fn debug_highlight_objective_type_mut(&mut self) -> &mut i32 {
        &mut self.debug_highlight_objective_type
    }
    fn seek_and_destroy_min_radius(&self) -> &f32 {
        &self.seek_and_destroy_min_radius
    }
    fn seek_and_destroy_min_radius_mut(&mut self) -> &mut f32 {
        &mut self.seek_and_destroy_min_radius
    }
    fn seek_and_destroy_max_radius(&self) -> &f32 {
        &self.seek_and_destroy_max_radius
    }
    fn seek_and_destroy_max_radius_mut(&mut self) -> &mut f32 {
        &mut self.seek_and_destroy_max_radius
    }
    fn force_repath_if_too_far_from_waypoint_meters(&self) -> &f32 {
        &self.force_repath_if_too_far_from_waypoint_meters
    }
    fn force_repath_if_too_far_from_waypoint_meters_mut(&mut self) -> &mut f32 {
        &mut self.force_repath_if_too_far_from_waypoint_meters
    }
    fn waypoint_minimum_progress_meters(&self) -> &f32 {
        &self.waypoint_minimum_progress_meters
    }
    fn waypoint_minimum_progress_meters_mut(&mut self) -> &mut f32 {
        &mut self.waypoint_minimum_progress_meters
    }
    fn debug_draw_aim_noise(&self) -> &bool {
        &self.debug_draw_aim_noise
    }
    fn debug_draw_aim_noise_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_aim_noise
    }
    fn aim_noise_scale(&self) -> &f32 {
        &self.aim_noise_scale
    }
    fn aim_noise_scale_mut(&mut self) -> &mut f32 {
        &mut self.aim_noise_scale
    }
    fn target_min_switch_time_s(&self) -> &f32 {
        &self.target_min_switch_time_s
    }
    fn target_min_switch_time_s_mut(&mut self) -> &mut f32 {
        &mut self.target_min_switch_time_s
    }
    fn max_target_engaging_distance_scale(&self) -> &f32 {
        &self.max_target_engaging_distance_scale
    }
    fn max_target_engaging_distance_scale_mut(&mut self) -> &mut f32 {
        &mut self.max_target_engaging_distance_scale
    }
    fn allow_random_path_spread(&self) -> &bool {
        &self.allow_random_path_spread
    }
    fn allow_random_path_spread_mut(&mut self) -> &mut bool {
        &mut self.allow_random_path_spread
    }
    fn force_use_random_path_spread(&self) -> &bool {
        &self.force_use_random_path_spread
    }
    fn force_use_random_path_spread_mut(&mut self) -> &mut bool {
        &mut self.force_use_random_path_spread
    }
    fn random_path_spread_radius(&self) -> &f32 {
        &self.random_path_spread_radius
    }
    fn random_path_spread_radius_mut(&mut self) -> &mut f32 {
        &mut self.random_path_spread_radius
    }
    fn random_path_spread_center_distance(&self) -> &f32 {
        &self.random_path_spread_center_distance
    }
    fn random_path_spread_center_distance_mut(&mut self) -> &mut f32 {
        &mut self.random_path_spread_center_distance
    }
    fn update_target_cooldown(&self) -> &f32 {
        &self.update_target_cooldown
    }
    fn update_target_cooldown_mut(&mut self) -> &mut f32 {
        &mut self.update_target_cooldown
    }
    fn forced_target_timeout_seconds(&self) -> &f32 {
        &self.forced_target_timeout_seconds
    }
    fn forced_target_timeout_seconds_mut(&mut self) -> &mut f32 {
        &mut self.forced_target_timeout_seconds
    }
    fn debug_draw_players_names_and_ids(&self) -> &bool {
        &self.debug_draw_players_names_and_ids
    }
    fn debug_draw_players_names_and_ids_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_players_names_and_ids
    }
    fn verbose_logging(&self) -> &bool {
        &self.verbose_logging
    }
    fn verbose_logging_mut(&mut self) -> &mut bool {
        &mut self.verbose_logging
    }
    fn action_objective_default_time(&self) -> &f32 {
        &self.action_objective_default_time
    }
    fn action_objective_default_time_mut(&mut self) -> &mut f32 {
        &mut self.action_objective_default_time
    }
    fn allow_action_gadget(&self) -> &bool {
        &self.allow_action_gadget
    }
    fn allow_action_gadget_mut(&mut self) -> &mut bool {
        &mut self.allow_action_gadget
    }
    fn action_gadget_probability(&self) -> &f32 {
        &self.action_gadget_probability
    }
    fn action_gadget_probability_mut(&mut self) -> &mut f32 {
        &mut self.action_gadget_probability
    }
    fn action_gadget_interactable_search_radius(&self) -> &f32 {
        &self.action_gadget_interactable_search_radius
    }
    fn action_gadget_interactable_search_radius_mut(&mut self) -> &mut f32 {
        &mut self.action_gadget_interactable_search_radius
    }
    fn hero_spawn_probability_gameplay(&self) -> &f32 {
        &self.hero_spawn_probability_gameplay
    }
    fn hero_spawn_probability_gameplay_mut(&mut self) -> &mut f32 {
        &mut self.hero_spawn_probability_gameplay
    }
    fn special_spawn_probability_gameplay(&self) -> &f32 {
        &self.special_spawn_probability_gameplay
    }
    fn special_spawn_probability_gameplay_mut(&mut self) -> &mut f32 {
        &mut self.special_spawn_probability_gameplay
    }
    fn hero_vehicle_spawn_probability_gameplay(&self) -> &f32 {
        &self.hero_vehicle_spawn_probability_gameplay
    }
    fn hero_vehicle_spawn_probability_gameplay_mut(&mut self) -> &mut f32 {
        &mut self.hero_vehicle_spawn_probability_gameplay
    }
    fn vehicle_spawn_probability_gameplay(&self) -> &f32 {
        &self.vehicle_spawn_probability_gameplay
    }
    fn vehicle_spawn_probability_gameplay_mut(&mut self) -> &mut f32 {
        &mut self.vehicle_spawn_probability_gameplay
    }
    fn hero_spawn_probability(&self) -> &f32 {
        &self.hero_spawn_probability
    }
    fn hero_spawn_probability_mut(&mut self) -> &mut f32 {
        &mut self.hero_spawn_probability
    }
    fn special_spawn_probability(&self) -> &f32 {
        &self.special_spawn_probability
    }
    fn special_spawn_probability_mut(&mut self) -> &mut f32 {
        &mut self.special_spawn_probability
    }
    fn hero_vehicle_spawn_probability(&self) -> &f32 {
        &self.hero_vehicle_spawn_probability
    }
    fn hero_vehicle_spawn_probability_mut(&mut self) -> &mut f32 {
        &mut self.hero_vehicle_spawn_probability
    }
    fn vehicle_spawn_probability(&self) -> &f32 {
        &self.vehicle_spawn_probability
    }
    fn vehicle_spawn_probability_mut(&mut self) -> &mut f32 {
        &mut self.vehicle_spawn_probability
    }
    fn follow_target_position_check_cooldown(&self) -> &f32 {
        &self.follow_target_position_check_cooldown
    }
    fn follow_target_position_check_cooldown_mut(&mut self) -> &mut f32 {
        &mut self.follow_target_position_check_cooldown
    }
    fn not_alive_assert_time(&self) -> &f32 {
        &self.not_alive_assert_time
    }
    fn not_alive_assert_time_mut(&mut self) -> &mut f32 {
        &mut self.not_alive_assert_time
    }
    fn prefer_f_p_s_camera(&self) -> &bool {
        &self.prefer_f_p_s_camera
    }
    fn prefer_f_p_s_camera_mut(&mut self) -> &mut bool {
        &mut self.prefer_f_p_s_camera
    }
    fn time_on_path_tolerance_seconds(&self) -> &f32 {
        &self.time_on_path_tolerance_seconds
    }
    fn time_on_path_tolerance_seconds_mut(&mut self) -> &mut f32 {
        &mut self.time_on_path_tolerance_seconds
    }
    fn check_water_depth_for_intermediate_positions(&self) -> &bool {
        &self.check_water_depth_for_intermediate_positions
    }
    fn check_water_depth_for_intermediate_positions_mut(&mut self) -> &mut bool {
        &mut self.check_water_depth_for_intermediate_positions
    }
    fn swimming_suicide_timeout(&self) -> &f32 {
        &self.swimming_suicide_timeout
    }
    fn swimming_suicide_timeout_mut(&mut self) -> &mut f32 {
        &mut self.swimming_suicide_timeout
    }
    fn lof_prediction_time(&self) -> &f32 {
        &self.lof_prediction_time
    }
    fn lof_prediction_time_mut(&mut self) -> &mut f32 {
        &mut self.lof_prediction_time
    }
    fn debug_draw_combat_raycast_hit_points(&self) -> &bool {
        &self.debug_draw_combat_raycast_hit_points
    }
    fn debug_draw_combat_raycast_hit_points_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_combat_raycast_hit_points
    }
    fn debug_draw_transforms(&self) -> &bool {
        &self.debug_draw_transforms
    }
    fn debug_draw_transforms_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_transforms
    }
    fn target_tracker_field_of_view_degrees(&self) -> &f32 {
        &self.target_tracker_field_of_view_degrees
    }
    fn target_tracker_field_of_view_degrees_mut(&mut self) -> &mut f32 {
        &mut self.target_tracker_field_of_view_degrees
    }
    fn pick_random_vehicle_on_secondary_objective(&self) -> &bool {
        &self.pick_random_vehicle_on_secondary_objective
    }
    fn pick_random_vehicle_on_secondary_objective_mut(&mut self) -> &mut bool {
        &mut self.pick_random_vehicle_on_secondary_objective
    }
    fn never_exit_vehicle_after_entering(&self) -> &bool {
        &self.never_exit_vehicle_after_entering
    }
    fn never_exit_vehicle_after_entering_mut(&mut self) -> &mut bool {
        &mut self.never_exit_vehicle_after_entering
    }
    fn update_target_per_frame_cap(&self) -> &u32 {
        &self.update_target_per_frame_cap
    }
    fn update_target_per_frame_cap_mut(&mut self) -> &mut u32 {
        &mut self.update_target_per_frame_cap
    }
    fn replay_telemetry_file(&self) -> &String {
        &self.replay_telemetry_file
    }
    fn replay_telemetry_file_mut(&mut self) -> &mut String {
        &mut self.replay_telemetry_file
    }
    fn replay_telemetry_file_format(&self) -> &String {
        &self.replay_telemetry_file_format
    }
    fn replay_telemetry_file_format_mut(&mut self) -> &mut String {
        &mut self.replay_telemetry_file_format
    }
    fn replay_telemetry_adjust_time(&self) -> &bool {
        &self.replay_telemetry_adjust_time
    }
    fn replay_telemetry_adjust_time_mut(&mut self) -> &mut bool {
        &mut self.replay_telemetry_adjust_time
    }
    fn replay_telemetry_adjust_time_padding(&self) -> &f32 {
        &self.replay_telemetry_adjust_time_padding
    }
    fn replay_telemetry_adjust_time_padding_mut(&mut self) -> &mut f32 {
        &mut self.replay_telemetry_adjust_time_padding
    }
    fn debug_draw_weapon_details(&self) -> &bool {
        &self.debug_draw_weapon_details
    }
    fn debug_draw_weapon_details_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_weapon_details
    }
    fn debug_draw_extensive_client_details(&self) -> &bool {
        &self.debug_draw_extensive_client_details
    }
    fn debug_draw_extensive_client_details_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_extensive_client_details
    }
    fn evasive_maneuvers_jump_probability(&self) -> &f32 {
        &self.evasive_maneuvers_jump_probability
    }
    fn evasive_maneuvers_jump_probability_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_jump_probability
    }
    fn evasive_maneuvers_dodge_roll_probability(&self) -> &f32 {
        &self.evasive_maneuvers_dodge_roll_probability
    }
    fn evasive_maneuvers_dodge_roll_probability_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_dodge_roll_probability
    }
    fn evasive_maneuvers_invert_strafe_duration_max(&self) -> &f32 {
        &self.evasive_maneuvers_invert_strafe_duration_max
    }
    fn evasive_maneuvers_invert_strafe_duration_max_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_invert_strafe_duration_max
    }
    fn evasive_maneuvers_invert_strafe_duration_min(&self) -> &f32 {
        &self.evasive_maneuvers_invert_strafe_duration_min
    }
    fn evasive_maneuvers_invert_strafe_duration_min_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_invert_strafe_duration_min
    }
    fn leg_head_aim_ratio_override(&self) -> &f32 {
        &self.leg_head_aim_ratio_override
    }
    fn leg_head_aim_ratio_override_mut(&mut self) -> &mut f32 {
        &mut self.leg_head_aim_ratio_override
    }
    fn attacking_ability_left_probability(&self) -> &f32 {
        &self.attacking_ability_left_probability
    }
    fn attacking_ability_left_probability_mut(&mut self) -> &mut f32 {
        &mut self.attacking_ability_left_probability
    }
    fn attacking_ability_left_duration_seconds(&self) -> &f32 {
        &self.attacking_ability_left_duration_seconds
    }
    fn attacking_ability_left_duration_seconds_mut(&mut self) -> &mut f32 {
        &mut self.attacking_ability_left_duration_seconds
    }
    fn attacking_ability_middle_probability(&self) -> &f32 {
        &self.attacking_ability_middle_probability
    }
    fn attacking_ability_middle_probability_mut(&mut self) -> &mut f32 {
        &mut self.attacking_ability_middle_probability
    }
    fn attacking_ability_middle_duration_seconds(&self) -> &f32 {
        &self.attacking_ability_middle_duration_seconds
    }
    fn attacking_ability_middle_duration_seconds_mut(&mut self) -> &mut f32 {
        &mut self.attacking_ability_middle_duration_seconds
    }
    fn attacking_ability_right_probability(&self) -> &f32 {
        &self.attacking_ability_right_probability
    }
    fn attacking_ability_right_probability_mut(&mut self) -> &mut f32 {
        &mut self.attacking_ability_right_probability
    }
    fn attacking_ability_right_duration_seconds(&self) -> &f32 {
        &self.attacking_ability_right_duration_seconds
    }
    fn attacking_ability_right_duration_seconds_mut(&mut self) -> &mut f32 {
        &mut self.attacking_ability_right_duration_seconds
    }
    fn evasive_maneuvers_crouch_probability(&self) -> &f32 {
        &self.evasive_maneuvers_crouch_probability
    }
    fn evasive_maneuvers_crouch_probability_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_crouch_probability
    }
    fn evasive_maneuvers_crouch_duration(&self) -> &f32 {
        &self.evasive_maneuvers_crouch_duration
    }
    fn evasive_maneuvers_crouch_duration_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_crouch_duration
    }
    fn blaster_leg_head_aim_ratio(&self) -> &f32 {
        &self.blaster_leg_head_aim_ratio
    }
    fn blaster_leg_head_aim_ratio_mut(&mut self) -> &mut f32 {
        &mut self.blaster_leg_head_aim_ratio
    }
    fn blaster_aim_noise(&self) -> &f32 {
        &self.blaster_aim_noise
    }
    fn blaster_aim_noise_mut(&mut self) -> &mut f32 {
        &mut self.blaster_aim_noise
    }
    fn sniper_rifle_leg_head_aim_ratio(&self) -> &f32 {
        &self.sniper_rifle_leg_head_aim_ratio
    }
    fn sniper_rifle_leg_head_aim_ratio_mut(&mut self) -> &mut f32 {
        &mut self.sniper_rifle_leg_head_aim_ratio
    }
    fn sniper_rifle_aim_noise(&self) -> &f32 {
        &self.sniper_rifle_aim_noise
    }
    fn sniper_rifle_aim_noise_mut(&mut self) -> &mut f32 {
        &mut self.sniper_rifle_aim_noise
    }
    fn lmg_leg_head_aim_ratio(&self) -> &f32 {
        &self.lmg_leg_head_aim_ratio
    }
    fn lmg_leg_head_aim_ratio_mut(&mut self) -> &mut f32 {
        &mut self.lmg_leg_head_aim_ratio
    }
    fn lmg_aim_noise(&self) -> &f32 {
        &self.lmg_aim_noise
    }
    fn lmg_aim_noise_mut(&mut self) -> &mut f32 {
        &mut self.lmg_aim_noise
    }
    fn shotgun_leg_head_aim_ratio(&self) -> &f32 {
        &self.shotgun_leg_head_aim_ratio
    }
    fn shotgun_leg_head_aim_ratio_mut(&mut self) -> &mut f32 {
        &mut self.shotgun_leg_head_aim_ratio
    }
    fn shotgun_aim_noise(&self) -> &f32 {
        &self.shotgun_aim_noise
    }
    fn shotgun_aim_noise_mut(&mut self) -> &mut f32 {
        &mut self.shotgun_aim_noise
    }
    fn launcher_leg_head_aim_ratio(&self) -> &f32 {
        &self.launcher_leg_head_aim_ratio
    }
    fn launcher_leg_head_aim_ratio_mut(&mut self) -> &mut f32 {
        &mut self.launcher_leg_head_aim_ratio
    }
    fn launcher_aim_noise(&self) -> &f32 {
        &self.launcher_aim_noise
    }
    fn launcher_aim_noise_mut(&mut self) -> &mut f32 {
        &mut self.launcher_aim_noise
    }
    fn use_sword_attacking_abilities_from_meters(&self) -> &f32 {
        &self.use_sword_attacking_abilities_from_meters
    }
    fn use_sword_attacking_abilities_from_meters_mut(&mut self) -> &mut f32 {
        &mut self.use_sword_attacking_abilities_from_meters
    }
    fn sword_attack_duration_time_min_s(&self) -> &f32 {
        &self.sword_attack_duration_time_min_s
    }
    fn sword_attack_duration_time_min_s_mut(&mut self) -> &mut f32 {
        &mut self.sword_attack_duration_time_min_s
    }
    fn sword_attack_duration_time_max_s(&self) -> &f32 {
        &self.sword_attack_duration_time_max_s
    }
    fn sword_attack_duration_time_max_s_mut(&mut self) -> &mut f32 {
        &mut self.sword_attack_duration_time_max_s
    }
    fn pause_sword_attack_duration_time_min_s(&self) -> &f32 {
        &self.pause_sword_attack_duration_time_min_s
    }
    fn pause_sword_attack_duration_time_min_s_mut(&mut self) -> &mut f32 {
        &mut self.pause_sword_attack_duration_time_min_s
    }
    fn pause_sword_attack_duration_time_max_s(&self) -> &f32 {
        &self.pause_sword_attack_duration_time_max_s
    }
    fn pause_sword_attack_duration_time_max_s_mut(&mut self) -> &mut f32 {
        &mut self.pause_sword_attack_duration_time_max_s
    }
    fn sword_attack_distance_meters_min(&self) -> &f32 {
        &self.sword_attack_distance_meters_min
    }
    fn sword_attack_distance_meters_min_mut(&mut self) -> &mut f32 {
        &mut self.sword_attack_distance_meters_min
    }
    fn sword_attack_distance_meters_max(&self) -> &f32 {
        &self.sword_attack_distance_meters_max
    }
    fn sword_attack_distance_meters_max_mut(&mut self) -> &mut f32 {
        &mut self.sword_attack_distance_meters_max
    }
    fn debug_draw_input_details(&self) -> &bool {
        &self.debug_draw_input_details
    }
    fn debug_draw_input_details_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_input_details
    }
    fn debug_window_position_scale_offset_x(&self) -> &f32 {
        &self.debug_window_position_scale_offset_x
    }
    fn debug_window_position_scale_offset_x_mut(&mut self) -> &mut f32 {
        &mut self.debug_window_position_scale_offset_x
    }
    fn debug_window_position_scale_offset_y(&self) -> &f32 {
        &self.debug_window_position_scale_offset_y
    }
    fn debug_window_position_scale_offset_y_mut(&mut self) -> &mut f32 {
        &mut self.debug_window_position_scale_offset_y
    }
    fn debug_window_width(&self) -> &i32 {
        &self.debug_window_width
    }
    fn debug_window_width_mut(&mut self) -> &mut i32 {
        &mut self.debug_window_width
    }
    fn debug_window_height(&self) -> &i32 {
        &self.debug_window_height
    }
    fn debug_window_height_mut(&mut self) -> &mut i32 {
        &mut self.debug_window_height
    }
    fn path_look_ahead_meters(&self) -> &f32 {
        &self.path_look_ahead_meters
    }
    fn path_look_ahead_meters_mut(&mut self) -> &mut f32 {
        &mut self.path_look_ahead_meters
    }
    fn path_look_right_meters(&self) -> &f32 {
        &self.path_look_right_meters
    }
    fn path_look_right_meters_mut(&mut self) -> &mut f32 {
        &mut self.path_look_right_meters
    }
    fn waypoint_tolerance_meters(&self) -> &f32 {
        &self.waypoint_tolerance_meters
    }
    fn waypoint_tolerance_meters_mut(&mut self) -> &mut f32 {
        &mut self.waypoint_tolerance_meters
    }
    fn debug_draw_aim_at_positions(&self) -> &bool {
        &self.debug_draw_aim_at_positions
    }
    fn debug_draw_aim_at_positions_mut(&mut self) -> &mut bool {
        &mut self.debug_draw_aim_at_positions
    }
    fn evasive_maneuvers_vehicle_scale(&self) -> &f32 {
        &self.evasive_maneuvers_vehicle_scale
    }
    fn evasive_maneuvers_vehicle_scale_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_vehicle_scale
    }
    fn vehicle_aim_noise_scale(&self) -> &f32 {
        &self.vehicle_aim_noise_scale
    }
    fn vehicle_aim_noise_scale_mut(&mut self) -> &mut f32 {
        &mut self.vehicle_aim_noise_scale
    }
    fn reset_settings_on_level_unload(&self) -> &bool {
        &self.reset_settings_on_level_unload
    }
    fn reset_settings_on_level_unload_mut(&mut self) -> &mut bool {
        &mut self.reset_settings_on_level_unload
    }
    fn sword_guard_duration_time_min_s(&self) -> &f32 {
        &self.sword_guard_duration_time_min_s
    }
    fn sword_guard_duration_time_min_s_mut(&mut self) -> &mut f32 {
        &mut self.sword_guard_duration_time_min_s
    }
    fn sword_guard_duration_time_max_s(&self) -> &f32 {
        &self.sword_guard_duration_time_max_s
    }
    fn sword_guard_duration_time_max_s_mut(&mut self) -> &mut f32 {
        &mut self.sword_guard_duration_time_max_s
    }
    fn aim_noise_scale_team1(&self) -> &f32 {
        &self.aim_noise_scale_team1
    }
    fn aim_noise_scale_team1_mut(&mut self) -> &mut f32 {
        &mut self.aim_noise_scale_team1
    }
    fn aim_noise_scale_team2(&self) -> &f32 {
        &self.aim_noise_scale_team2
    }
    fn aim_noise_scale_team2_mut(&mut self) -> &mut f32 {
        &mut self.aim_noise_scale_team2
    }
    fn hero_strafe_probability_per_frame(&self) -> &f32 {
        &self.hero_strafe_probability_per_frame
    }
    fn hero_strafe_probability_per_frame_mut(&mut self) -> &mut f32 {
        &mut self.hero_strafe_probability_per_frame
    }
    fn emote_probability_after_players_death(&self) -> &f32 {
        &self.emote_probability_after_players_death
    }
    fn emote_probability_after_players_death_mut(&mut self) -> &mut f32 {
        &mut self.emote_probability_after_players_death
    }
    fn emote_duration(&self) -> &f32 {
        &self.emote_duration
    }
    fn emote_duration_mut(&mut self) -> &mut f32 {
        &mut self.emote_duration
    }
    fn melee_interval_s(&self) -> &f32 {
        &self.melee_interval_s
    }
    fn melee_interval_s_mut(&mut self) -> &mut f32 {
        &mut self.melee_interval_s
    }
    fn melee_distance_m(&self) -> &f32 {
        &self.melee_distance_m
    }
    fn melee_distance_m_mut(&mut self) -> &mut f32 {
        &mut self.melee_distance_m
    }
    fn allow_evasive_manouvers_o_o_b(&self) -> &bool {
        &self.allow_evasive_manouvers_o_o_b
    }
    fn allow_evasive_manouvers_o_o_b_mut(&mut self) -> &mut bool {
        &mut self.allow_evasive_manouvers_o_o_b
    }
    fn evasive_maneuvers_ground_check_enabled(&self) -> &bool {
        &self.evasive_maneuvers_ground_check_enabled
    }
    fn evasive_maneuvers_ground_check_enabled_mut(&mut self) -> &mut bool {
        &mut self.evasive_maneuvers_ground_check_enabled
    }
    fn evasive_maneuvers_ground_check_distance_m(&self) -> &f32 {
        &self.evasive_maneuvers_ground_check_distance_m
    }
    fn evasive_maneuvers_ground_check_distance_m_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_ground_check_distance_m
    }
    fn evasive_maneuvers_ground_check_height_distance_m(&self) -> &f32 {
        &self.evasive_maneuvers_ground_check_height_distance_m
    }
    fn evasive_maneuvers_ground_check_height_distance_m_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_ground_check_height_distance_m
    }
    fn evasive_maneuvers_ground_check_height_offset_m(&self) -> &f32 {
        &self.evasive_maneuvers_ground_check_height_offset_m
    }
    fn evasive_maneuvers_ground_check_height_offset_m_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_ground_check_height_offset_m
    }
    fn evasive_maneuvers_ground_check_cooldown_s(&self) -> &f32 {
        &self.evasive_maneuvers_ground_check_cooldown_s
    }
    fn evasive_maneuvers_ground_check_cooldown_s_mut(&mut self) -> &mut f32 {
        &mut self.evasive_maneuvers_ground_check_cooldown_s
    }
    fn evasive_maneuvers_vehicles_enabled(&self) -> &bool {
        &self.evasive_maneuvers_vehicles_enabled
    }
    fn evasive_maneuvers_vehicles_enabled_mut(&mut self) -> &mut bool {
        &mut self.evasive_maneuvers_vehicles_enabled
    }
    fn vehicle_minimum_forward_throttle(&self) -> &f32 {
        &self.vehicle_minimum_forward_throttle
    }
    fn vehicle_minimum_forward_throttle_mut(&mut self) -> &mut f32 {
        &mut self.vehicle_minimum_forward_throttle
    }
    fn vehicle_use_character_throttle(&self) -> &bool {
        &self.vehicle_use_character_throttle
    }
    fn vehicle_use_character_throttle_mut(&mut self) -> &mut bool {
        &mut self.vehicle_use_character_throttle
    }
}

impl super::core::SystemSettingsTrait for AutoPlayerSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for AutoPlayerSettings {
}

pub static AUTOPLAYERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettings",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AFKTakeover",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, a_f_k_takeover),
            },
            FieldInfoData {
                name: "ClientEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, client_enabled),
            },
            FieldInfoData {
                name: "AllowClientTakeOver",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_client_take_over),
            },
            FieldInfoData {
                name: "ForceServerControl",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_server_control),
            },
            FieldInfoData {
                name: "ForceServerObjectiveControl",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_server_objective_control),
            },
            FieldInfoData {
                name: "ForceClientObjectiveControl",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_client_objective_control),
            },
            FieldInfoData {
                name: "ForceClientNavigation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_client_navigation),
            },
            FieldInfoData {
                name: "DebugDrawEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_enabled),
            },
            FieldInfoData {
                name: "DebugDrawWaypoints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_waypoints),
            },
            FieldInfoData {
                name: "DebugDrawClientDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_client_details),
            },
            FieldInfoData {
                name: "DebugDrawCombatDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_combat_details),
            },
            FieldInfoData {
                name: "PlayerCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, player_count),
            },
            FieldInfoData {
                name: "ForcedServerAutoPlayerCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, forced_server_auto_player_count),
            },
            FieldInfoData {
                name: "AllowAddAutoFillPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_add_auto_fill_players),
            },
            FieldInfoData {
                name: "AllowRemoveAutoFillPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_remove_auto_fill_players),
            },
            FieldInfoData {
                name: "ForceApplyGameplayBotsCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_apply_gameplay_bots_count),
            },
            FieldInfoData {
                name: "ForceFillGameplayBotsTeam1",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, force_fill_gameplay_bots_team1),
            },
            FieldInfoData {
                name: "ForceFillGameplayBotsTeam2",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, force_fill_gameplay_bots_team2),
            },
            FieldInfoData {
                name: "RespawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, respawn_delay),
            },
            FieldInfoData {
                name: "InitialRespawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, initial_respawn_delay),
            },
            FieldInfoData {
                name: "ClientJoinDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, client_join_delay),
            },
            FieldInfoData {
                name: "RoundTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, round_timeout),
            },
            FieldInfoData {
                name: "SquadMembers",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, squad_members),
            },
            FieldInfoData {
                name: "AllowGameplayBotsToJoinPlayerSquads",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_gameplay_bots_to_join_player_squads),
            },
            FieldInfoData {
                name: "AllowGameplayBotsToFormOwnSquads",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_gameplay_bots_to_form_own_squads),
            },
            FieldInfoData {
                name: "AllowVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_spawn),
            },
            FieldInfoData {
                name: "ForceDisableVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_disable_vehicle_spawn),
            },
            FieldInfoData {
                name: "AllowClientVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_client_vehicle_spawn),
            },
            FieldInfoData {
                name: "AllowFirstClientInitialVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_first_client_initial_vehicle_spawn),
            },
            FieldInfoData {
                name: "ControlConnectionlessPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, control_connectionless_players),
            },
            FieldInfoData {
                name: "AllowRespawn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_respawn),
            },
            FieldInfoData {
                name: "PickupItemsSecondaryObjectiveAttemptIntervalSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, pickup_items_secondary_objective_attempt_interval_seconds),
            },
            FieldInfoData {
                name: "UseTelemetryBasedPlanner",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, use_telemetry_based_planner),
            },
            FieldInfoData {
                name: "DebugTelemetryBasedPlanner",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_telemetry_based_planner),
            },
            FieldInfoData {
                name: "PlannerTerrainVerticalCutoff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, planner_terrain_vertical_cutoff),
            },
            FieldInfoData {
                name: "PlannerConnectionCutoff",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, planner_connection_cutoff),
            },
            FieldInfoData {
                name: "PlannerMaxNodesSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, planner_max_nodes_search_radius),
            },
            FieldInfoData {
                name: "PlannerLinkEndArrivalRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, planner_link_end_arrival_range),
            },
            FieldInfoData {
                name: "UseFadeOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, use_fade_override),
            },
            FieldInfoData {
                name: "InputScaleYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, input_scale_yaw),
            },
            FieldInfoData {
                name: "InputScalePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, input_scale_pitch),
            },
            FieldInfoData {
                name: "InputScaleClient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, input_scale_client),
            },
            FieldInfoData {
                name: "InputForceMouse",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, input_force_mouse),
            },
            FieldInfoData {
                name: "UseInputOverrideYawPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, use_input_override_yaw_pitch),
            },
            FieldInfoData {
                name: "InputOverrideYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, input_override_yaw),
            },
            FieldInfoData {
                name: "InputOverridePitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, input_override_pitch),
            },
            FieldInfoData {
                name: "UseSeekAndDestroy",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, use_seek_and_destroy),
            },
            FieldInfoData {
                name: "AllowTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_teleport),
            },
            FieldInfoData {
                name: "ForceAllowAllTeleports",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_allow_all_teleports),
            },
            FieldInfoData {
                name: "DebugDrawTeleports",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_teleports),
            },
            FieldInfoData {
                name: "UpdateAI",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, update_a_i),
            },
            FieldInfoData {
                name: "DebugDrawClientOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_client_only),
            },
            FieldInfoData {
                name: "DebugDrawClientRealmOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_client_realm_only),
            },
            FieldInfoData {
                name: "AimAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, aim_acceleration),
            },
            FieldInfoData {
                name: "AimLapTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, aim_lap_time),
            },
            FieldInfoData {
                name: "AllowMoveOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_move_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowSpawnOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_spawn_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowVehicleSpawnOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_spawn_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowVehicleSpawnOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_spawn_only),
            },
            FieldInfoData {
                name: "DebugDrawPrettyPath",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_pretty_path),
            },
            FieldInfoData {
                name: "DebugDrawUseWaypointsAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_use_waypoints_alpha),
            },
            FieldInfoData {
                name: "DebugDrawInvalidMoveIntention",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_invalid_move_intention),
            },
            FieldInfoData {
                name: "DebugSpam",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_spam),
            },
            FieldInfoData {
                name: "LofTimeoutS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, lof_timeout_s),
            },
            FieldInfoData {
                name: "LofReactionTimeS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, lof_reaction_time_s),
            },
            FieldInfoData {
                name: "ServerPlayersIgnoreClientPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, server_players_ignore_client_players),
            },
            FieldInfoData {
                name: "IgnoreHumanPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, ignore_human_players),
            },
            FieldInfoData {
                name: "ForceKit",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, force_kit),
            },
            FieldInfoData {
                name: "OpportunisticInteract",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, opportunistic_interact),
            },
            FieldInfoData {
                name: "SquadSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, squad_spawn_probability),
            },
            FieldInfoData {
                name: "KitChangeProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, kit_change_probability),
            },
            FieldInfoData {
                name: "UseDefaultUnlocksProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, use_default_unlocks_probability),
            },
            FieldInfoData {
                name: "AllowMedicRevive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_medic_revive),
            },
            FieldInfoData {
                name: "AllowPickupItems",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_pickup_items),
            },
            FieldInfoData {
                name: "DebugDrawObjectives",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_objectives),
            },
            FieldInfoData {
                name: "DebugDrawObjectiveAlways",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_objective_always),
            },
            FieldInfoData {
                name: "Wallhack",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, wallhack),
            },
            FieldInfoData {
                name: "WeaponSwapIntervalS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, weapon_swap_interval_s),
            },
            FieldInfoData {
                name: "WeaponSwapPrimaryProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, weapon_swap_primary_probability),
            },
            FieldInfoData {
                name: "VehicleBailTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_bail_time),
            },
            FieldInfoData {
                name: "JumpIfStuckTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, jump_if_stuck_time_seconds),
            },
            FieldInfoData {
                name: "JumpCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, jump_cooldown_seconds),
            },
            FieldInfoData {
                name: "PatrolPositionCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, patrol_position_cooldown_seconds),
            },
            FieldInfoData {
                name: "CombatUseGrenades",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, combat_use_grenades),
            },
            FieldInfoData {
                name: "CombatUseProne",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, combat_use_prone),
            },
            FieldInfoData {
                name: "CombatUseMelee",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, combat_use_melee),
            },
            FieldInfoData {
                name: "UseCrouch",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, use_crouch),
            },
            FieldInfoData {
                name: "ForcedFireTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, forced_fire_time_max_s),
            },
            FieldInfoData {
                name: "ForcedFireTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, forced_fire_time_min_s),
            },
            FieldInfoData {
                name: "AllowPrimaryWeaponForcedFire",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_primary_weapon_forced_fire),
            },
            FieldInfoData {
                name: "AllowVehicleForcedFire",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_forced_fire),
            },
            FieldInfoData {
                name: "ForcedFireVehicleTimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, forced_fire_vehicle_time_scale),
            },
            FieldInfoData {
                name: "ExitVehicleWhenStuckTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, exit_vehicle_when_stuck_timeout),
            },
            FieldInfoData {
                name: "MinDistanceForVehicleUTurn",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, min_distance_for_vehicle_u_turn),
            },
            FieldInfoData {
                name: "MinAirplaneBailOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, min_airplane_bail_out_time),
            },
            FieldInfoData {
                name: "MaxAirplaneBailOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, max_airplane_bail_out_time),
            },
            FieldInfoData {
                name: "LoginRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, login_rate),
            },
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, spawn_rate),
            },
            FieldInfoData {
                name: "MaxSpawnsPerUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, max_spawns_per_update),
            },
            FieldInfoData {
                name: "Variance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, variance),
            },
            FieldInfoData {
                name: "AirplaneExitInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, airplane_exit_input),
            },
            FieldInfoData {
                name: "SecondaryObjectiveGenerationMinSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_generation_min_seconds),
            },
            FieldInfoData {
                name: "SecondaryObjectiveGenerationMaxSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_generation_max_seconds),
            },
            FieldInfoData {
                name: "AllowEnterVehicle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_enter_vehicle),
            },
            FieldInfoData {
                name: "EnterVehicleCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, enter_vehicle_cooldown_seconds),
            },
            FieldInfoData {
                name: "EnterVehicleProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, enter_vehicle_probability),
            },
            FieldInfoData {
                name: "EnterVehicleSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, enter_vehicle_search_radius),
            },
            FieldInfoData {
                name: "PrintClientInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, print_client_input),
            },
            FieldInfoData {
                name: "AllowPrimaryObjective",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_primary_objective),
            },
            FieldInfoData {
                name: "AllowSecondaryObjectivesWhilePassive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_secondary_objectives_while_passive),
            },
            FieldInfoData {
                name: "AllowSecondaryObjectivesWhileDefensive",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_secondary_objectives_while_defensive),
            },
            FieldInfoData {
                name: "AllowPathfinding",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_pathfinding),
            },
            FieldInfoData {
                name: "SecondaryObjectiveTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_timeout_seconds),
            },
            FieldInfoData {
                name: "ForcePassiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_passive_mode),
            },
            FieldInfoData {
                name: "ForcePrimaryObjectiveDefensiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_primary_objective_defensive_mode),
            },
            FieldInfoData {
                name: "ForcePrimaryObjectiveAggressiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_primary_objective_aggressive_mode),
            },
            FieldInfoData {
                name: "ForceSecondaryObjectiveDefensiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_secondary_objective_defensive_mode),
            },
            FieldInfoData {
                name: "ForceSecondaryObjectiveAggressiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_secondary_objective_aggressive_mode),
            },
            FieldInfoData {
                name: "ClientJesusMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, client_jesus_mode),
            },
            FieldInfoData {
                name: "AllowFortifications",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_fortifications),
            },
            FieldInfoData {
                name: "FortificationProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, fortification_probability),
            },
            FieldInfoData {
                name: "FortificationSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, fortification_search_radius),
            },
            FieldInfoData {
                name: "RepathCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, repath_cooldown_seconds),
            },
            FieldInfoData {
                name: "UnStuckVehicleActionsTriggerTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, un_stuck_vehicle_actions_trigger_time_seconds),
            },
            FieldInfoData {
                name: "UnstuckMinimalMoveDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, unstuck_minimal_move_distance),
            },
            FieldInfoData {
                name: "UnstuckMinimalMoveSuicideTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, unstuck_minimal_move_suicide_timeout),
            },
            FieldInfoData {
                name: "FallenBelowSuicideTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, fallen_below_suicide_timeout),
            },
            FieldInfoData {
                name: "NavigationPositionToleranceMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, navigation_position_tolerance_meters),
            },
            FieldInfoData {
                name: "UseNameGenerator",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, use_name_generator),
            },
            FieldInfoData {
                name: "AllowStuckEscapeProcedure",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_stuck_escape_procedure),
            },
            FieldInfoData {
                name: "ExitStuckEscapeProcedureOnVisualCheck",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, exit_stuck_escape_procedure_on_visual_check),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureSensorLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_sensor_length),
            },
            FieldInfoData {
                name: "StuckEscapeProcedurePIFraction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_p_i_fraction),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureEscapeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_escape_distance),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureActivationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_activation_seconds),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureUpdateInterval",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_update_interval),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_timeout_seconds),
            },
            FieldInfoData {
                name: "DebugDrawUnstuck",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_unstuck),
            },
            FieldInfoData {
                name: "UnStuckActionsTriggerTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, un_stuck_actions_trigger_time_seconds),
            },
            FieldInfoData {
                name: "UnStuckActionsTriggerCooldown",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, un_stuck_actions_trigger_cooldown),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureRetries",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_retries),
            },
            FieldInfoData {
                name: "PrimaryInteractionSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, primary_interaction_search_radius),
            },
            FieldInfoData {
                name: "AllowSuicide",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_suicide),
            },
            FieldInfoData {
                name: "AllowRandomBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_random_behavior),
            },
            FieldInfoData {
                name: "AllowSecondaryInteractions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_secondary_interactions),
            },
            FieldInfoData {
                name: "SecondaryInteractionsProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_interactions_probability),
            },
            FieldInfoData {
                name: "SecondaryInteractionsSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_interactions_search_radius),
            },
            FieldInfoData {
                name: "SecondaryObjectivePickupItemsSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_pickup_items_search_radius),
            },
            FieldInfoData {
                name: "SecondaryObjectivePickupItemsInteractOrActionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_pickup_items_interact_or_action_radius),
            },
            FieldInfoData {
                name: "SecondaryObjectiveJesusMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_jesus_mode),
            },
            FieldInfoData {
                name: "SecondaryReviveSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, secondary_revive_search_distance),
            },
            FieldInfoData {
                name: "DebugDrawNavigationDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_navigation_details),
            },
            FieldInfoData {
                name: "DebugDrawNavigationProgressDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_navigation_progress_details),
            },
            FieldInfoData {
                name: "DebugDrawCustomInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_custom_input),
            },
            FieldInfoData {
                name: "ExpectedTravelTimeDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, expected_travel_time_distance_scale),
            },
            FieldInfoData {
                name: "ExpectedTravelTimeBase",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, expected_travel_time_base),
            },
            FieldInfoData {
                name: "InteractAreaTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, interact_area_time),
            },
            FieldInfoData {
                name: "DebugHighlightObjectiveType",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, debug_highlight_objective_type),
            },
            FieldInfoData {
                name: "SeekAndDestroyMinRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, seek_and_destroy_min_radius),
            },
            FieldInfoData {
                name: "SeekAndDestroyMaxRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, seek_and_destroy_max_radius),
            },
            FieldInfoData {
                name: "ForceRepathIfTooFarFromWaypointMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, force_repath_if_too_far_from_waypoint_meters),
            },
            FieldInfoData {
                name: "WaypointMinimumProgressMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, waypoint_minimum_progress_meters),
            },
            FieldInfoData {
                name: "DebugDrawAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_aim_noise),
            },
            FieldInfoData {
                name: "AimNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, aim_noise_scale),
            },
            FieldInfoData {
                name: "TargetMinSwitchTimeS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, target_min_switch_time_s),
            },
            FieldInfoData {
                name: "MaxTargetEngagingDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, max_target_engaging_distance_scale),
            },
            FieldInfoData {
                name: "AllowRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_random_path_spread),
            },
            FieldInfoData {
                name: "ForceUseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, force_use_random_path_spread),
            },
            FieldInfoData {
                name: "RandomPathSpreadRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, random_path_spread_radius),
            },
            FieldInfoData {
                name: "RandomPathSpreadCenterDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, random_path_spread_center_distance),
            },
            FieldInfoData {
                name: "UpdateTargetCooldown",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, update_target_cooldown),
            },
            FieldInfoData {
                name: "ForcedTargetTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, forced_target_timeout_seconds),
            },
            FieldInfoData {
                name: "DebugDrawPlayersNamesAndIds",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_players_names_and_ids),
            },
            FieldInfoData {
                name: "VerboseLogging",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, verbose_logging),
            },
            FieldInfoData {
                name: "ActionObjectiveDefaultTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, action_objective_default_time),
            },
            FieldInfoData {
                name: "AllowActionGadget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_action_gadget),
            },
            FieldInfoData {
                name: "ActionGadgetProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, action_gadget_probability),
            },
            FieldInfoData {
                name: "ActionGadgetInteractableSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, action_gadget_interactable_search_radius),
            },
            FieldInfoData {
                name: "HeroSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, hero_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "SpecialSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, special_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "HeroVehicleSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, hero_vehicle_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "VehicleSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "HeroSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, hero_spawn_probability),
            },
            FieldInfoData {
                name: "SpecialSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, special_spawn_probability),
            },
            FieldInfoData {
                name: "HeroVehicleSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, hero_vehicle_spawn_probability),
            },
            FieldInfoData {
                name: "VehicleSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_spawn_probability),
            },
            FieldInfoData {
                name: "FollowTargetPositionCheckCooldown",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, follow_target_position_check_cooldown),
            },
            FieldInfoData {
                name: "NotAliveAssertTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, not_alive_assert_time),
            },
            FieldInfoData {
                name: "PreferFPSCamera",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, prefer_f_p_s_camera),
            },
            FieldInfoData {
                name: "TimeOnPathToleranceSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, time_on_path_tolerance_seconds),
            },
            FieldInfoData {
                name: "CheckWaterDepthForIntermediatePositions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, check_water_depth_for_intermediate_positions),
            },
            FieldInfoData {
                name: "SwimmingSuicideTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, swimming_suicide_timeout),
            },
            FieldInfoData {
                name: "LofPredictionTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, lof_prediction_time),
            },
            FieldInfoData {
                name: "DebugDrawCombatRaycastHitPoints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_combat_raycast_hit_points),
            },
            FieldInfoData {
                name: "DebugDrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_transforms),
            },
            FieldInfoData {
                name: "TargetTrackerFieldOfViewDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, target_tracker_field_of_view_degrees),
            },
            FieldInfoData {
                name: "PickRandomVehicleOnSecondaryObjective",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, pick_random_vehicle_on_secondary_objective),
            },
            FieldInfoData {
                name: "NeverExitVehicleAfterEntering",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, never_exit_vehicle_after_entering),
            },
            FieldInfoData {
                name: "UpdateTargetPerFrameCap",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AutoPlayerSettings, update_target_per_frame_cap),
            },
            FieldInfoData {
                name: "ReplayTelemetryFile",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_file),
            },
            FieldInfoData {
                name: "ReplayTelemetryFileFormat",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_file_format),
            },
            FieldInfoData {
                name: "ReplayTelemetryAdjustTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_adjust_time),
            },
            FieldInfoData {
                name: "ReplayTelemetryAdjustTimePadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_adjust_time_padding),
            },
            FieldInfoData {
                name: "DebugDrawWeaponDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_weapon_details),
            },
            FieldInfoData {
                name: "DebugDrawExtensiveClientDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_extensive_client_details),
            },
            FieldInfoData {
                name: "EvasiveManeuversJumpProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_jump_probability),
            },
            FieldInfoData {
                name: "EvasiveManeuversDodgeRollProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_dodge_roll_probability),
            },
            FieldInfoData {
                name: "EvasiveManeuversInvertStrafeDurationMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_invert_strafe_duration_max),
            },
            FieldInfoData {
                name: "EvasiveManeuversInvertStrafeDurationMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_invert_strafe_duration_min),
            },
            FieldInfoData {
                name: "LegHeadAimRatioOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, leg_head_aim_ratio_override),
            },
            FieldInfoData {
                name: "AttackingAbilityLeftProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_left_probability),
            },
            FieldInfoData {
                name: "AttackingAbilityLeftDurationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_left_duration_seconds),
            },
            FieldInfoData {
                name: "AttackingAbilityMiddleProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_middle_probability),
            },
            FieldInfoData {
                name: "AttackingAbilityMiddleDurationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_middle_duration_seconds),
            },
            FieldInfoData {
                name: "AttackingAbilityRightProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_right_probability),
            },
            FieldInfoData {
                name: "AttackingAbilityRightDurationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_right_duration_seconds),
            },
            FieldInfoData {
                name: "EvasiveManeuversCrouchProbability",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_crouch_probability),
            },
            FieldInfoData {
                name: "EvasiveManeuversCrouchDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_crouch_duration),
            },
            FieldInfoData {
                name: "BlasterLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, blaster_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "BlasterAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, blaster_aim_noise),
            },
            FieldInfoData {
                name: "SniperRifleLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sniper_rifle_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "SniperRifleAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sniper_rifle_aim_noise),
            },
            FieldInfoData {
                name: "LmgLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, lmg_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "LmgAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, lmg_aim_noise),
            },
            FieldInfoData {
                name: "ShotgunLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, shotgun_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "ShotgunAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, shotgun_aim_noise),
            },
            FieldInfoData {
                name: "LauncherLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, launcher_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "LauncherAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, launcher_aim_noise),
            },
            FieldInfoData {
                name: "UseSwordAttackingAbilitiesFromMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, use_sword_attacking_abilities_from_meters),
            },
            FieldInfoData {
                name: "SwordAttackDurationTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_duration_time_min_s),
            },
            FieldInfoData {
                name: "SwordAttackDurationTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_duration_time_max_s),
            },
            FieldInfoData {
                name: "PauseSwordAttackDurationTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, pause_sword_attack_duration_time_min_s),
            },
            FieldInfoData {
                name: "PauseSwordAttackDurationTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, pause_sword_attack_duration_time_max_s),
            },
            FieldInfoData {
                name: "SwordAttackDistanceMetersMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_distance_meters_min),
            },
            FieldInfoData {
                name: "SwordAttackDistanceMetersMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_distance_meters_max),
            },
            FieldInfoData {
                name: "DebugDrawInputDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_input_details),
            },
            FieldInfoData {
                name: "DebugWindowPositionScaleOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_position_scale_offset_x),
            },
            FieldInfoData {
                name: "DebugWindowPositionScaleOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_position_scale_offset_y),
            },
            FieldInfoData {
                name: "DebugWindowWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_width),
            },
            FieldInfoData {
                name: "DebugWindowHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_height),
            },
            FieldInfoData {
                name: "PathLookAheadMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, path_look_ahead_meters),
            },
            FieldInfoData {
                name: "PathLookRightMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, path_look_right_meters),
            },
            FieldInfoData {
                name: "WaypointToleranceMeters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, waypoint_tolerance_meters),
            },
            FieldInfoData {
                name: "DebugDrawAimAtPositions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_aim_at_positions),
            },
            FieldInfoData {
                name: "EvasiveManeuversVehicleScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_vehicle_scale),
            },
            FieldInfoData {
                name: "VehicleAimNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_aim_noise_scale),
            },
            FieldInfoData {
                name: "ResetSettingsOnLevelUnload",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, reset_settings_on_level_unload),
            },
            FieldInfoData {
                name: "SwordGuardDurationTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sword_guard_duration_time_min_s),
            },
            FieldInfoData {
                name: "SwordGuardDurationTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, sword_guard_duration_time_max_s),
            },
            FieldInfoData {
                name: "AimNoiseScaleTeam1",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, aim_noise_scale_team1),
            },
            FieldInfoData {
                name: "AimNoiseScaleTeam2",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, aim_noise_scale_team2),
            },
            FieldInfoData {
                name: "HeroStrafeProbabilityPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, hero_strafe_probability_per_frame),
            },
            FieldInfoData {
                name: "EmoteProbabilityAfterPlayersDeath",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, emote_probability_after_players_death),
            },
            FieldInfoData {
                name: "EmoteDuration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, emote_duration),
            },
            FieldInfoData {
                name: "MeleeIntervalS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, melee_interval_s),
            },
            FieldInfoData {
                name: "MeleeDistanceM",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, melee_distance_m),
            },
            FieldInfoData {
                name: "AllowEvasiveManouversOOB",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, allow_evasive_manouvers_o_o_b),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_enabled),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckDistanceM",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_distance_m),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckHeightDistanceM",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_height_distance_m),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckHeightOffsetM",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_height_offset_m),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckCooldownS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_cooldown_s),
            },
            FieldInfoData {
                name: "EvasiveManeuversVehiclesEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_vehicles_enabled),
            },
            FieldInfoData {
                name: "VehicleMinimumForwardThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_minimum_forward_throttle),
            },
            FieldInfoData {
                name: "VehicleUseCharacterThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_use_character_throttle),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERSETTINGS_TYPE_INFO
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


pub static AUTOPLAYERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAutoPlayerSettingsEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAutoPlayerSettingsEntityTrait: super::entity::EntityTrait {
}

impl ServerAutoPlayerSettingsEntityTrait for ServerAutoPlayerSettingsEntity {
}

impl super::entity::EntityTrait for ServerAutoPlayerSettingsEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAutoPlayerSettingsEntity {
}

pub static SERVERAUTOPLAYERSETTINGSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerSettingsEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAutoPlayerSettingsEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAUTOPLAYERSETTINGSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAutoPlayerSettingsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAUTOPLAYERSETTINGSENTITY_TYPE_INFO
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


pub static SERVERAUTOPLAYERSETTINGSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerSettingsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("ServerAutoPlayerSettingsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAutoPlayerManagerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAutoPlayerManagerEntityTrait: super::entity::EntityTrait {
}

impl ServerAutoPlayerManagerEntityTrait for ServerAutoPlayerManagerEntity {
}

impl super::entity::EntityTrait for ServerAutoPlayerManagerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerAutoPlayerManagerEntity {
}

pub static SERVERAUTOPLAYERMANAGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerManagerEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAutoPlayerManagerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAUTOPLAYERMANAGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAutoPlayerManagerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAUTOPLAYERMANAGERENTITY_TYPE_INFO
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


pub static SERVERAUTOPLAYERMANAGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerManagerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("ServerAutoPlayerManagerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerObjectiveEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait AutoPlayerObjectiveEntityBaseTrait: super::entity::EntityTrait {
}

impl AutoPlayerObjectiveEntityBaseTrait for AutoPlayerObjectiveEntityBase {
}

impl super::entity::EntityTrait for AutoPlayerObjectiveEntityBase {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerObjectiveEntityBase {
}

pub static AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerObjectiveEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYEROBJECTIVEENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerObjectiveEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO
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


pub static AUTOPLAYEROBJECTIVEENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerObjectiveEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerMoveObjectiveEntity {
    pub _glacier_base: AutoPlayerObjectiveEntityBase,
}

pub trait AutoPlayerMoveObjectiveEntityTrait: AutoPlayerObjectiveEntityBaseTrait {
}

impl AutoPlayerMoveObjectiveEntityTrait for AutoPlayerMoveObjectiveEntity {
}

impl AutoPlayerObjectiveEntityBaseTrait for AutoPlayerMoveObjectiveEntity {
}

impl super::entity::EntityTrait for AutoPlayerMoveObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerMoveObjectiveEntity {
}

pub static AUTOPLAYERMOVEOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerMoveObjectiveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERMOVEOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerMoveObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERMOVEOBJECTIVEENTITY_TYPE_INFO
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


pub static AUTOPLAYERMOVEOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerMoveObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerInteractObjectiveEntity {
    pub _glacier_base: AutoPlayerObjectiveEntityBase,
}

pub trait AutoPlayerInteractObjectiveEntityTrait: AutoPlayerObjectiveEntityBaseTrait {
}

impl AutoPlayerInteractObjectiveEntityTrait for AutoPlayerInteractObjectiveEntity {
}

impl AutoPlayerObjectiveEntityBaseTrait for AutoPlayerInteractObjectiveEntity {
}

impl super::entity::EntityTrait for AutoPlayerInteractObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerInteractObjectiveEntity {
}

pub static AUTOPLAYERINTERACTOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerInteractObjectiveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERINTERACTOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerInteractObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERINTERACTOBJECTIVEENTITY_TYPE_INFO
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


pub static AUTOPLAYERINTERACTOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerInteractObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerFollowObjectiveEntity {
    pub _glacier_base: AutoPlayerObjectiveEntityBase,
}

pub trait AutoPlayerFollowObjectiveEntityTrait: AutoPlayerObjectiveEntityBaseTrait {
}

impl AutoPlayerFollowObjectiveEntityTrait for AutoPlayerFollowObjectiveEntity {
}

impl AutoPlayerObjectiveEntityBaseTrait for AutoPlayerFollowObjectiveEntity {
}

impl super::entity::EntityTrait for AutoPlayerFollowObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerFollowObjectiveEntity {
}

pub static AUTOPLAYERFOLLOWOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerFollowObjectiveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERFOLLOWOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerFollowObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERFOLLOWOBJECTIVEENTITY_TYPE_INFO
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


pub static AUTOPLAYERFOLLOWOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerFollowObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerDefendObjectiveEntity {
    pub _glacier_base: AutoPlayerObjectiveEntityBase,
}

pub trait AutoPlayerDefendObjectiveEntityTrait: AutoPlayerObjectiveEntityBaseTrait {
}

impl AutoPlayerDefendObjectiveEntityTrait for AutoPlayerDefendObjectiveEntity {
}

impl AutoPlayerObjectiveEntityBaseTrait for AutoPlayerDefendObjectiveEntity {
}

impl super::entity::EntityTrait for AutoPlayerDefendObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerDefendObjectiveEntity {
}

pub static AUTOPLAYERDEFENDOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerDefendObjectiveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERDEFENDOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerDefendObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERDEFENDOBJECTIVEENTITY_TYPE_INFO
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


pub static AUTOPLAYERDEFENDOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerDefendObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoPlayerAttackObjectiveEntity {
    pub _glacier_base: AutoPlayerObjectiveEntityBase,
}

pub trait AutoPlayerAttackObjectiveEntityTrait: AutoPlayerObjectiveEntityBaseTrait {
}

impl AutoPlayerAttackObjectiveEntityTrait for AutoPlayerAttackObjectiveEntity {
}

impl AutoPlayerObjectiveEntityBaseTrait for AutoPlayerAttackObjectiveEntity {
}

impl super::entity::EntityTrait for AutoPlayerAttackObjectiveEntity {
}

impl super::entity::EntityBusPeerTrait for AutoPlayerAttackObjectiveEntity {
}

pub static AUTOPLAYERATTACKOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoPlayerAttackObjectiveEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERATTACKOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerAttackObjectiveEntity {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOPLAYERATTACKOBJECTIVEENTITY_TYPE_INFO
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


pub static AUTOPLAYERATTACKOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerAttackObjectiveEntity"),
    array_type: None,
    alignment: 8,
};


