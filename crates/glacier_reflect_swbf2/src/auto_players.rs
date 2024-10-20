use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerActionObjectiveEntity {
}

pub const AUTOPLAYERACTIONOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERACTIONOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerActionObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERACTIONOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const AUTOPLAYERACTIONOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerActionObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerListenerEntity {
}

pub const AUTOPLAYERLISTENERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerListenerEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERLISTENERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerListenerEntity {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERLISTENERENTITY_TYPE_INFO
    }
}


pub const AUTOPLAYERLISTENERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerListenerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerListenerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerObjectiveEntityData {
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

pub const AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, realm),
            },
            FieldInfoData {
                name: "Players",
                flags: MemberInfoFlags::new(0),
                field_type: QUERYENTITYRESULT_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, players),
            },
            FieldInfoData {
                name: "JesusMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, jesus_mode),
            },
            FieldInfoData {
                name: "UnlimitedAmmo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, unlimited_ammo),
            },
            FieldInfoData {
                name: "AllowTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, allow_teleport),
            },
            FieldInfoData {
                name: "UseObjectiveTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, use_objective_teleport),
            },
            FieldInfoData {
                name: "UseStuckEscapeProcedure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, use_stuck_escape_procedure),
            },
            FieldInfoData {
                name: "UseNavmesh",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, use_navmesh),
            },
            FieldInfoData {
                name: "AllowObjectiveMoveOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, allow_objective_move_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowSecondaryObjectives",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, allow_secondary_objectives),
            },
            FieldInfoData {
                name: "DisableZoomWhenAiming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, disable_zoom_when_aiming),
            },
            FieldInfoData {
                name: "MoveMode",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOPLAYERMOVEMODE_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, move_mode),
            },
            FieldInfoData {
                name: "MoveModeOverrideInt",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, move_mode_override_int),
            },
            FieldInfoData {
                name: "DebugName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, debug_name),
            },
            FieldInfoData {
                name: "TimeThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, time_threshold),
            },
            FieldInfoData {
                name: "ClampVerticalNavPosSearchMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, clamp_vertical_nav_pos_search_meters),
            },
            FieldInfoData {
                name: "RestrictedAreaSphereCentre",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, restricted_area_sphere_centre),
            },
            FieldInfoData {
                name: "RestrictedAreaSphereRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerObjectiveEntityData, restricted_area_sphere_radius),
            },
        ],
    }),
    array_type: Some(AUTOPLAYEROBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerObjectiveEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYEROBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerObjectiveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AutoPlayerMoveMode {
    #[default]
    AutoPlayerMoveMode_Aggressive = 0,
    AutoPlayerMoveMode_Defensive = 1,
    AutoPlayerMoveMode_Passive = 2,
}

pub const AUTOPLAYERMOVEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveMode",
    flags: MemberInfoFlags::new(49429),
    module: "AutoPlayers",
    data: TypeInfoData::Enum,
    array_type: Some(AUTOPLAYERMOVEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPlayerMoveMode {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERMOVEMODE_TYPE_INFO
    }
}


pub const AUTOPLAYERMOVEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerMoveMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerMoveObjectiveEntityData {
    pub target_position: super::core::Vec3,
    pub move_area_radius: f32,
    pub is_target_reached_as_soon_as_entering_target_area: bool,
    pub use_random_path_spread: bool,
}

pub const AUTOPLAYERMOVEOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, target_position),
            },
            FieldInfoData {
                name: "MoveAreaRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, move_area_radius),
            },
            FieldInfoData {
                name: "IsTargetReachedAsSoonAsEnteringTargetArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, is_target_reached_as_soon_as_entering_target_area),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerMoveObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERMOVEOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerMoveObjectiveEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERMOVEOBJECTIVEENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERMOVEOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerMoveObjectiveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerInteractObjectiveEntityData {
    pub interaction_entity_position: super::core::Vec3,
    pub interaction_position: super::core::Vec3,
    pub interact_input_action: i32,
    pub find_interact_target_position: bool,
    pub interaction_start_radius: f32,
    pub hold_to_interact_time: f32,
    pub primary_interaction_search_radius: f32,
    pub use_random_path_spread: bool,
}

pub const AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InteractionEntityPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interaction_entity_position),
            },
            FieldInfoData {
                name: "InteractionPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interaction_position),
            },
            FieldInfoData {
                name: "InteractInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interact_input_action),
            },
            FieldInfoData {
                name: "FindInteractTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, find_interact_target_position),
            },
            FieldInfoData {
                name: "InteractionStartRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, interaction_start_radius),
            },
            FieldInfoData {
                name: "HoldToInteractTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, hold_to_interact_time),
            },
            FieldInfoData {
                name: "PrimaryInteractionSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, primary_interaction_search_radius),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerInteractObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerInteractObjectiveEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERINTERACTOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerInteractObjectiveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerFollowObjectiveEntityData {
    pub follow_target_player: super::dice_shooter_shared::QueryEntityResult,
    pub follow_position_radius: f32,
    pub use_bread_crumbs_path_following: bool,
    pub follow_timeout_seconds: f32,
    pub follow_until_position: super::core::Vec3,
    pub follow_until_position_tolerance_meters: f32,
    pub actively_patrol_follow_player_position: bool,
}

pub const AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FollowTargetPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: QUERYENTITYRESULT_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_target_player),
            },
            FieldInfoData {
                name: "FollowPositionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_position_radius),
            },
            FieldInfoData {
                name: "UseBreadCrumbsPathFollowing",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, use_bread_crumbs_path_following),
            },
            FieldInfoData {
                name: "FollowTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_timeout_seconds),
            },
            FieldInfoData {
                name: "FollowUntilPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_until_position),
            },
            FieldInfoData {
                name: "FollowUntilPositionToleranceMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, follow_until_position_tolerance_meters),
            },
            FieldInfoData {
                name: "ActivelyPatrolFollowPlayerPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerFollowObjectiveEntityData, actively_patrol_follow_player_position),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerFollowObjectiveEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERFOLLOWOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerFollowObjectiveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerDefendObjectiveEntityData {
    pub target_position: super::core::Vec3,
    pub defend_area_radius: f32,
    pub use_random_path_spread: bool,
    pub defend_current_position: bool,
    pub is_target_reached_as_soon_as_entering_target_area: bool,
}

pub const AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, target_position),
            },
            FieldInfoData {
                name: "DefendAreaRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, defend_area_radius),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, use_random_path_spread),
            },
            FieldInfoData {
                name: "DefendCurrentPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, defend_current_position),
            },
            FieldInfoData {
                name: "IsTargetReachedAsSoonAsEnteringTargetArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerDefendObjectiveEntityData, is_target_reached_as_soon_as_entering_target_area),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerDefendObjectiveEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERDEFENDOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerDefendObjectiveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerAttackObjectiveEntityData {
    pub targets: super::dice_shooter_shared::QueryEntityResult,
    pub weapon: i32,
    pub use_random_path_spread: bool,
}

pub const AUTOPLAYERATTACKOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Targets",
                flags: MemberInfoFlags::new(0),
                field_type: QUERYENTITYRESULT_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerAttackObjectiveEntityData, targets),
            },
            FieldInfoData {
                name: "Weapon",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerAttackObjectiveEntityData, weapon),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerAttackObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERATTACKOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerAttackObjectiveEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERATTACKOBJECTIVEENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERATTACKOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerAttackObjectiveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerActionObjectiveEntityData {
    pub action_target_position: super::core::Vec3,
    pub action_position: super::core::Vec3,
    pub action_start_radius: f32,
    pub select_item_input: i32,
    pub action_input: i32,
    pub action_time: f32,
    pub is_a_spamming_button_action: bool,
    pub use_random_path_spread: bool,
}

pub const AUTOPLAYERACTIONOBJECTIVEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ActionTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_target_position),
            },
            FieldInfoData {
                name: "ActionPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_position),
            },
            FieldInfoData {
                name: "ActionStartRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_start_radius),
            },
            FieldInfoData {
                name: "SelectItemInput",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, select_item_input),
            },
            FieldInfoData {
                name: "ActionInput",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_input),
            },
            FieldInfoData {
                name: "ActionTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, action_time),
            },
            FieldInfoData {
                name: "IsASpammingButtonAction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, is_a_spamming_button_action),
            },
            FieldInfoData {
                name: "UseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerActionObjectiveEntityData, use_random_path_spread),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERACTIONOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AutoPlayerActionObjectiveEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERACTIONOBJECTIVEENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERACTIONOBJECTIVEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerActionObjectiveEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerActionObjectiveEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerSettingsEntityData {
    pub choice: AutoPlayerSettingsChoice,
}

pub const AUTOPLAYERSETTINGSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Choice",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOPLAYERSETTINGSCHOICE_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettingsEntityData, choice),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERSETTINGSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerSettingsEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERSETTINGSENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERSETTINGSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettingsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerSettingsChoice {
    pub kind: AutoPlayerSettingsKind,
    pub name: String,
}

pub const AUTOPLAYERSETTINGSCHOICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsChoice",
    flags: MemberInfoFlags::new(73),
    module: "AutoPlayers",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Kind",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOPLAYERSETTINGSKIND_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettingsChoice, kind),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettingsChoice, name),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERSETTINGSCHOICE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerSettingsChoice {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERSETTINGSCHOICE_TYPE_INFO
    }
}


pub const AUTOPLAYERSETTINGSCHOICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsChoice-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettingsChoice-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AutoPlayerSettingsKind {
    #[default]
    Kind_Int = 0,
    Kind_Bool = 1,
    Kind_String = 2,
    Kind_Float = 3,
    Kind_Uint = 4,
}

pub const AUTOPLAYERSETTINGSKIND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsKind",
    flags: MemberInfoFlags::new(49429),
    module: "AutoPlayers",
    data: TypeInfoData::Enum,
    array_type: Some(AUTOPLAYERSETTINGSKIND_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AutoPlayerSettingsKind {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERSETTINGSKIND_TYPE_INFO
    }
}


pub const AUTOPLAYERSETTINGSKIND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettingsKind-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettingsKind-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerManagerEntityData {
    pub player_count: i32,
    pub fill_gameplay_bots_team1: i32,
    pub fill_gameplay_bots_team2: i32,
    pub reset_force_fills: bool,
    pub orphan_time_seconds: f32,
}

pub const AUTOPLAYERMANAGERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerManagerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PlayerCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerManagerEntityData, player_count),
            },
            FieldInfoData {
                name: "FillGameplayBotsTeam1",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerManagerEntityData, fill_gameplay_bots_team1),
            },
            FieldInfoData {
                name: "FillGameplayBotsTeam2",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerManagerEntityData, fill_gameplay_bots_team2),
            },
            FieldInfoData {
                name: "ResetForceFills",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerManagerEntityData, reset_force_fills),
            },
            FieldInfoData {
                name: "OrphanTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerManagerEntityData, orphan_time_seconds),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERMANAGERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerManagerEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERMANAGERENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERMANAGERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerManagerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerManagerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerEntityData {
    pub realm: super::core::Realm,
    pub auto_start: bool,
}

pub const AUTOPLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerEntityData, realm),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerEntityData, auto_start),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerEntityData {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERENTITYDATA_TYPE_INFO
    }
}


pub const AUTOPLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AutoPlayerSettings {
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

pub const AUTOPLAYERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettings",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AFKTakeover",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, a_f_k_takeover),
            },
            FieldInfoData {
                name: "ClientEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, client_enabled),
            },
            FieldInfoData {
                name: "AllowClientTakeOver",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_client_take_over),
            },
            FieldInfoData {
                name: "ForceServerControl",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_server_control),
            },
            FieldInfoData {
                name: "ForceServerObjectiveControl",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_server_objective_control),
            },
            FieldInfoData {
                name: "ForceClientObjectiveControl",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_client_objective_control),
            },
            FieldInfoData {
                name: "ForceClientNavigation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_client_navigation),
            },
            FieldInfoData {
                name: "DebugDrawEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_enabled),
            },
            FieldInfoData {
                name: "DebugDrawWaypoints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_waypoints),
            },
            FieldInfoData {
                name: "DebugDrawClientDetails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_client_details),
            },
            FieldInfoData {
                name: "DebugDrawCombatDetails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_combat_details),
            },
            FieldInfoData {
                name: "PlayerCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, player_count),
            },
            FieldInfoData {
                name: "ForcedServerAutoPlayerCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, forced_server_auto_player_count),
            },
            FieldInfoData {
                name: "AllowAddAutoFillPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_add_auto_fill_players),
            },
            FieldInfoData {
                name: "AllowRemoveAutoFillPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_remove_auto_fill_players),
            },
            FieldInfoData {
                name: "ForceApplyGameplayBotsCount",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_apply_gameplay_bots_count),
            },
            FieldInfoData {
                name: "ForceFillGameplayBotsTeam1",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_fill_gameplay_bots_team1),
            },
            FieldInfoData {
                name: "ForceFillGameplayBotsTeam2",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_fill_gameplay_bots_team2),
            },
            FieldInfoData {
                name: "RespawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, respawn_delay),
            },
            FieldInfoData {
                name: "InitialRespawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, initial_respawn_delay),
            },
            FieldInfoData {
                name: "ClientJoinDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, client_join_delay),
            },
            FieldInfoData {
                name: "RoundTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, round_timeout),
            },
            FieldInfoData {
                name: "SquadMembers",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, squad_members),
            },
            FieldInfoData {
                name: "AllowGameplayBotsToJoinPlayerSquads",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_gameplay_bots_to_join_player_squads),
            },
            FieldInfoData {
                name: "AllowGameplayBotsToFormOwnSquads",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_gameplay_bots_to_form_own_squads),
            },
            FieldInfoData {
                name: "AllowVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_spawn),
            },
            FieldInfoData {
                name: "ForceDisableVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_disable_vehicle_spawn),
            },
            FieldInfoData {
                name: "AllowClientVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_client_vehicle_spawn),
            },
            FieldInfoData {
                name: "AllowFirstClientInitialVehicleSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_first_client_initial_vehicle_spawn),
            },
            FieldInfoData {
                name: "ControlConnectionlessPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, control_connectionless_players),
            },
            FieldInfoData {
                name: "AllowRespawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_respawn),
            },
            FieldInfoData {
                name: "PickupItemsSecondaryObjectiveAttemptIntervalSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, pickup_items_secondary_objective_attempt_interval_seconds),
            },
            FieldInfoData {
                name: "UseTelemetryBasedPlanner",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_telemetry_based_planner),
            },
            FieldInfoData {
                name: "DebugTelemetryBasedPlanner",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_telemetry_based_planner),
            },
            FieldInfoData {
                name: "PlannerTerrainVerticalCutoff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, planner_terrain_vertical_cutoff),
            },
            FieldInfoData {
                name: "PlannerConnectionCutoff",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, planner_connection_cutoff),
            },
            FieldInfoData {
                name: "PlannerMaxNodesSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, planner_max_nodes_search_radius),
            },
            FieldInfoData {
                name: "PlannerLinkEndArrivalRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, planner_link_end_arrival_range),
            },
            FieldInfoData {
                name: "UseFadeOverride",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_fade_override),
            },
            FieldInfoData {
                name: "InputScaleYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, input_scale_yaw),
            },
            FieldInfoData {
                name: "InputScalePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, input_scale_pitch),
            },
            FieldInfoData {
                name: "InputScaleClient",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, input_scale_client),
            },
            FieldInfoData {
                name: "InputForceMouse",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, input_force_mouse),
            },
            FieldInfoData {
                name: "UseInputOverrideYawPitch",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_input_override_yaw_pitch),
            },
            FieldInfoData {
                name: "InputOverrideYaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, input_override_yaw),
            },
            FieldInfoData {
                name: "InputOverridePitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, input_override_pitch),
            },
            FieldInfoData {
                name: "UseSeekAndDestroy",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_seek_and_destroy),
            },
            FieldInfoData {
                name: "AllowTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_teleport),
            },
            FieldInfoData {
                name: "ForceAllowAllTeleports",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_allow_all_teleports),
            },
            FieldInfoData {
                name: "DebugDrawTeleports",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_teleports),
            },
            FieldInfoData {
                name: "UpdateAI",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, update_a_i),
            },
            FieldInfoData {
                name: "DebugDrawClientOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_client_only),
            },
            FieldInfoData {
                name: "DebugDrawClientRealmOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_client_realm_only),
            },
            FieldInfoData {
                name: "AimAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, aim_acceleration),
            },
            FieldInfoData {
                name: "AimLapTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, aim_lap_time),
            },
            FieldInfoData {
                name: "AllowMoveOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_move_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowSpawnOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_spawn_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowVehicleSpawnOutsideCombatArea",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_spawn_outside_combat_area),
            },
            FieldInfoData {
                name: "AllowVehicleSpawnOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_spawn_only),
            },
            FieldInfoData {
                name: "DebugDrawPrettyPath",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_pretty_path),
            },
            FieldInfoData {
                name: "DebugDrawUseWaypointsAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_use_waypoints_alpha),
            },
            FieldInfoData {
                name: "DebugDrawInvalidMoveIntention",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_invalid_move_intention),
            },
            FieldInfoData {
                name: "DebugSpam",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_spam),
            },
            FieldInfoData {
                name: "LofTimeoutS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, lof_timeout_s),
            },
            FieldInfoData {
                name: "LofReactionTimeS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, lof_reaction_time_s),
            },
            FieldInfoData {
                name: "ServerPlayersIgnoreClientPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, server_players_ignore_client_players),
            },
            FieldInfoData {
                name: "IgnoreHumanPlayers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, ignore_human_players),
            },
            FieldInfoData {
                name: "ForceKit",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_kit),
            },
            FieldInfoData {
                name: "OpportunisticInteract",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, opportunistic_interact),
            },
            FieldInfoData {
                name: "SquadSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, squad_spawn_probability),
            },
            FieldInfoData {
                name: "KitChangeProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, kit_change_probability),
            },
            FieldInfoData {
                name: "UseDefaultUnlocksProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_default_unlocks_probability),
            },
            FieldInfoData {
                name: "AllowMedicRevive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_medic_revive),
            },
            FieldInfoData {
                name: "AllowPickupItems",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_pickup_items),
            },
            FieldInfoData {
                name: "DebugDrawObjectives",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_objectives),
            },
            FieldInfoData {
                name: "DebugDrawObjectiveAlways",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_objective_always),
            },
            FieldInfoData {
                name: "Wallhack",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, wallhack),
            },
            FieldInfoData {
                name: "WeaponSwapIntervalS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, weapon_swap_interval_s),
            },
            FieldInfoData {
                name: "WeaponSwapPrimaryProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, weapon_swap_primary_probability),
            },
            FieldInfoData {
                name: "VehicleBailTime",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_bail_time),
            },
            FieldInfoData {
                name: "JumpIfStuckTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, jump_if_stuck_time_seconds),
            },
            FieldInfoData {
                name: "JumpCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, jump_cooldown_seconds),
            },
            FieldInfoData {
                name: "PatrolPositionCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, patrol_position_cooldown_seconds),
            },
            FieldInfoData {
                name: "CombatUseGrenades",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, combat_use_grenades),
            },
            FieldInfoData {
                name: "CombatUseProne",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, combat_use_prone),
            },
            FieldInfoData {
                name: "CombatUseMelee",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, combat_use_melee),
            },
            FieldInfoData {
                name: "UseCrouch",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_crouch),
            },
            FieldInfoData {
                name: "ForcedFireTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, forced_fire_time_max_s),
            },
            FieldInfoData {
                name: "ForcedFireTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, forced_fire_time_min_s),
            },
            FieldInfoData {
                name: "AllowPrimaryWeaponForcedFire",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_primary_weapon_forced_fire),
            },
            FieldInfoData {
                name: "AllowVehicleForcedFire",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_vehicle_forced_fire),
            },
            FieldInfoData {
                name: "ForcedFireVehicleTimeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, forced_fire_vehicle_time_scale),
            },
            FieldInfoData {
                name: "ExitVehicleWhenStuckTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, exit_vehicle_when_stuck_timeout),
            },
            FieldInfoData {
                name: "MinDistanceForVehicleUTurn",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, min_distance_for_vehicle_u_turn),
            },
            FieldInfoData {
                name: "MinAirplaneBailOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, min_airplane_bail_out_time),
            },
            FieldInfoData {
                name: "MaxAirplaneBailOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, max_airplane_bail_out_time),
            },
            FieldInfoData {
                name: "LoginRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, login_rate),
            },
            FieldInfoData {
                name: "SpawnRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, spawn_rate),
            },
            FieldInfoData {
                name: "MaxSpawnsPerUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, max_spawns_per_update),
            },
            FieldInfoData {
                name: "Variance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, variance),
            },
            FieldInfoData {
                name: "AirplaneExitInput",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, airplane_exit_input),
            },
            FieldInfoData {
                name: "SecondaryObjectiveGenerationMinSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_generation_min_seconds),
            },
            FieldInfoData {
                name: "SecondaryObjectiveGenerationMaxSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_generation_max_seconds),
            },
            FieldInfoData {
                name: "AllowEnterVehicle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_enter_vehicle),
            },
            FieldInfoData {
                name: "EnterVehicleCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, enter_vehicle_cooldown_seconds),
            },
            FieldInfoData {
                name: "EnterVehicleProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, enter_vehicle_probability),
            },
            FieldInfoData {
                name: "EnterVehicleSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, enter_vehicle_search_radius),
            },
            FieldInfoData {
                name: "PrintClientInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, print_client_input),
            },
            FieldInfoData {
                name: "AllowPrimaryObjective",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_primary_objective),
            },
            FieldInfoData {
                name: "AllowSecondaryObjectivesWhilePassive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_secondary_objectives_while_passive),
            },
            FieldInfoData {
                name: "AllowSecondaryObjectivesWhileDefensive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_secondary_objectives_while_defensive),
            },
            FieldInfoData {
                name: "AllowPathfinding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_pathfinding),
            },
            FieldInfoData {
                name: "SecondaryObjectiveTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_timeout_seconds),
            },
            FieldInfoData {
                name: "ForcePassiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_passive_mode),
            },
            FieldInfoData {
                name: "ForcePrimaryObjectiveDefensiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_primary_objective_defensive_mode),
            },
            FieldInfoData {
                name: "ForcePrimaryObjectiveAggressiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_primary_objective_aggressive_mode),
            },
            FieldInfoData {
                name: "ForceSecondaryObjectiveDefensiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_secondary_objective_defensive_mode),
            },
            FieldInfoData {
                name: "ForceSecondaryObjectiveAggressiveMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_secondary_objective_aggressive_mode),
            },
            FieldInfoData {
                name: "ClientJesusMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, client_jesus_mode),
            },
            FieldInfoData {
                name: "AllowFortifications",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_fortifications),
            },
            FieldInfoData {
                name: "FortificationProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, fortification_probability),
            },
            FieldInfoData {
                name: "FortificationSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, fortification_search_radius),
            },
            FieldInfoData {
                name: "RepathCooldownSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, repath_cooldown_seconds),
            },
            FieldInfoData {
                name: "UnStuckVehicleActionsTriggerTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, un_stuck_vehicle_actions_trigger_time_seconds),
            },
            FieldInfoData {
                name: "UnstuckMinimalMoveDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, unstuck_minimal_move_distance),
            },
            FieldInfoData {
                name: "UnstuckMinimalMoveSuicideTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, unstuck_minimal_move_suicide_timeout),
            },
            FieldInfoData {
                name: "FallenBelowSuicideTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, fallen_below_suicide_timeout),
            },
            FieldInfoData {
                name: "NavigationPositionToleranceMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, navigation_position_tolerance_meters),
            },
            FieldInfoData {
                name: "UseNameGenerator",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_name_generator),
            },
            FieldInfoData {
                name: "AllowStuckEscapeProcedure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_stuck_escape_procedure),
            },
            FieldInfoData {
                name: "ExitStuckEscapeProcedureOnVisualCheck",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, exit_stuck_escape_procedure_on_visual_check),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureSensorLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_sensor_length),
            },
            FieldInfoData {
                name: "StuckEscapeProcedurePIFraction",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_p_i_fraction),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureEscapeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_escape_distance),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureActivationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_activation_seconds),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureUpdateInterval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_update_interval),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_timeout_seconds),
            },
            FieldInfoData {
                name: "DebugDrawUnstuck",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_unstuck),
            },
            FieldInfoData {
                name: "UnStuckActionsTriggerTimeSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, un_stuck_actions_trigger_time_seconds),
            },
            FieldInfoData {
                name: "UnStuckActionsTriggerCooldown",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, un_stuck_actions_trigger_cooldown),
            },
            FieldInfoData {
                name: "StuckEscapeProcedureRetries",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, stuck_escape_procedure_retries),
            },
            FieldInfoData {
                name: "PrimaryInteractionSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, primary_interaction_search_radius),
            },
            FieldInfoData {
                name: "AllowSuicide",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_suicide),
            },
            FieldInfoData {
                name: "AllowRandomBehavior",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_random_behavior),
            },
            FieldInfoData {
                name: "AllowSecondaryInteractions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_secondary_interactions),
            },
            FieldInfoData {
                name: "SecondaryInteractionsProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_interactions_probability),
            },
            FieldInfoData {
                name: "SecondaryInteractionsSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_interactions_search_radius),
            },
            FieldInfoData {
                name: "SecondaryObjectivePickupItemsSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_pickup_items_search_radius),
            },
            FieldInfoData {
                name: "SecondaryObjectivePickupItemsInteractOrActionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_pickup_items_interact_or_action_radius),
            },
            FieldInfoData {
                name: "SecondaryObjectiveJesusMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_objective_jesus_mode),
            },
            FieldInfoData {
                name: "SecondaryReviveSearchDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, secondary_revive_search_distance),
            },
            FieldInfoData {
                name: "DebugDrawNavigationDetails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_navigation_details),
            },
            FieldInfoData {
                name: "DebugDrawNavigationProgressDetails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_navigation_progress_details),
            },
            FieldInfoData {
                name: "DebugDrawCustomInput",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_custom_input),
            },
            FieldInfoData {
                name: "ExpectedTravelTimeDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, expected_travel_time_distance_scale),
            },
            FieldInfoData {
                name: "ExpectedTravelTimeBase",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, expected_travel_time_base),
            },
            FieldInfoData {
                name: "InteractAreaTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, interact_area_time),
            },
            FieldInfoData {
                name: "DebugHighlightObjectiveType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_highlight_objective_type),
            },
            FieldInfoData {
                name: "SeekAndDestroyMinRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, seek_and_destroy_min_radius),
            },
            FieldInfoData {
                name: "SeekAndDestroyMaxRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, seek_and_destroy_max_radius),
            },
            FieldInfoData {
                name: "ForceRepathIfTooFarFromWaypointMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_repath_if_too_far_from_waypoint_meters),
            },
            FieldInfoData {
                name: "WaypointMinimumProgressMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, waypoint_minimum_progress_meters),
            },
            FieldInfoData {
                name: "DebugDrawAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_aim_noise),
            },
            FieldInfoData {
                name: "AimNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, aim_noise_scale),
            },
            FieldInfoData {
                name: "TargetMinSwitchTimeS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, target_min_switch_time_s),
            },
            FieldInfoData {
                name: "MaxTargetEngagingDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, max_target_engaging_distance_scale),
            },
            FieldInfoData {
                name: "AllowRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_random_path_spread),
            },
            FieldInfoData {
                name: "ForceUseRandomPathSpread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, force_use_random_path_spread),
            },
            FieldInfoData {
                name: "RandomPathSpreadRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, random_path_spread_radius),
            },
            FieldInfoData {
                name: "RandomPathSpreadCenterDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, random_path_spread_center_distance),
            },
            FieldInfoData {
                name: "UpdateTargetCooldown",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, update_target_cooldown),
            },
            FieldInfoData {
                name: "ForcedTargetTimeoutSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, forced_target_timeout_seconds),
            },
            FieldInfoData {
                name: "DebugDrawPlayersNamesAndIds",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_players_names_and_ids),
            },
            FieldInfoData {
                name: "VerboseLogging",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, verbose_logging),
            },
            FieldInfoData {
                name: "ActionObjectiveDefaultTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, action_objective_default_time),
            },
            FieldInfoData {
                name: "AllowActionGadget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_action_gadget),
            },
            FieldInfoData {
                name: "ActionGadgetProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, action_gadget_probability),
            },
            FieldInfoData {
                name: "ActionGadgetInteractableSearchRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, action_gadget_interactable_search_radius),
            },
            FieldInfoData {
                name: "HeroSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, hero_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "SpecialSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, special_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "HeroVehicleSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, hero_vehicle_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "VehicleSpawnProbability_Gameplay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_spawn_probability_gameplay),
            },
            FieldInfoData {
                name: "HeroSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, hero_spawn_probability),
            },
            FieldInfoData {
                name: "SpecialSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, special_spawn_probability),
            },
            FieldInfoData {
                name: "HeroVehicleSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, hero_vehicle_spawn_probability),
            },
            FieldInfoData {
                name: "VehicleSpawnProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_spawn_probability),
            },
            FieldInfoData {
                name: "FollowTargetPositionCheckCooldown",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, follow_target_position_check_cooldown),
            },
            FieldInfoData {
                name: "NotAliveAssertTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, not_alive_assert_time),
            },
            FieldInfoData {
                name: "PreferFPSCamera",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, prefer_f_p_s_camera),
            },
            FieldInfoData {
                name: "TimeOnPathToleranceSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, time_on_path_tolerance_seconds),
            },
            FieldInfoData {
                name: "CheckWaterDepthForIntermediatePositions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, check_water_depth_for_intermediate_positions),
            },
            FieldInfoData {
                name: "SwimmingSuicideTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, swimming_suicide_timeout),
            },
            FieldInfoData {
                name: "LofPredictionTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, lof_prediction_time),
            },
            FieldInfoData {
                name: "DebugDrawCombatRaycastHitPoints",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_combat_raycast_hit_points),
            },
            FieldInfoData {
                name: "DebugDrawTransforms",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_transforms),
            },
            FieldInfoData {
                name: "TargetTrackerFieldOfViewDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, target_tracker_field_of_view_degrees),
            },
            FieldInfoData {
                name: "PickRandomVehicleOnSecondaryObjective",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, pick_random_vehicle_on_secondary_objective),
            },
            FieldInfoData {
                name: "NeverExitVehicleAfterEntering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, never_exit_vehicle_after_entering),
            },
            FieldInfoData {
                name: "UpdateTargetPerFrameCap",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, update_target_per_frame_cap),
            },
            FieldInfoData {
                name: "ReplayTelemetryFile",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_file),
            },
            FieldInfoData {
                name: "ReplayTelemetryFileFormat",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_file_format),
            },
            FieldInfoData {
                name: "ReplayTelemetryAdjustTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_adjust_time),
            },
            FieldInfoData {
                name: "ReplayTelemetryAdjustTimePadding",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, replay_telemetry_adjust_time_padding),
            },
            FieldInfoData {
                name: "DebugDrawWeaponDetails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_weapon_details),
            },
            FieldInfoData {
                name: "DebugDrawExtensiveClientDetails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_extensive_client_details),
            },
            FieldInfoData {
                name: "EvasiveManeuversJumpProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_jump_probability),
            },
            FieldInfoData {
                name: "EvasiveManeuversDodgeRollProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_dodge_roll_probability),
            },
            FieldInfoData {
                name: "EvasiveManeuversInvertStrafeDurationMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_invert_strafe_duration_max),
            },
            FieldInfoData {
                name: "EvasiveManeuversInvertStrafeDurationMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_invert_strafe_duration_min),
            },
            FieldInfoData {
                name: "LegHeadAimRatioOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, leg_head_aim_ratio_override),
            },
            FieldInfoData {
                name: "AttackingAbilityLeftProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_left_probability),
            },
            FieldInfoData {
                name: "AttackingAbilityLeftDurationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_left_duration_seconds),
            },
            FieldInfoData {
                name: "AttackingAbilityMiddleProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_middle_probability),
            },
            FieldInfoData {
                name: "AttackingAbilityMiddleDurationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_middle_duration_seconds),
            },
            FieldInfoData {
                name: "AttackingAbilityRightProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_right_probability),
            },
            FieldInfoData {
                name: "AttackingAbilityRightDurationSeconds",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, attacking_ability_right_duration_seconds),
            },
            FieldInfoData {
                name: "EvasiveManeuversCrouchProbability",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_crouch_probability),
            },
            FieldInfoData {
                name: "EvasiveManeuversCrouchDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_crouch_duration),
            },
            FieldInfoData {
                name: "BlasterLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, blaster_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "BlasterAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, blaster_aim_noise),
            },
            FieldInfoData {
                name: "SniperRifleLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sniper_rifle_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "SniperRifleAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sniper_rifle_aim_noise),
            },
            FieldInfoData {
                name: "LmgLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, lmg_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "LmgAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, lmg_aim_noise),
            },
            FieldInfoData {
                name: "ShotgunLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, shotgun_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "ShotgunAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, shotgun_aim_noise),
            },
            FieldInfoData {
                name: "LauncherLegHeadAimRatio",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, launcher_leg_head_aim_ratio),
            },
            FieldInfoData {
                name: "LauncherAimNoise",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, launcher_aim_noise),
            },
            FieldInfoData {
                name: "UseSwordAttackingAbilitiesFromMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, use_sword_attacking_abilities_from_meters),
            },
            FieldInfoData {
                name: "SwordAttackDurationTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_duration_time_min_s),
            },
            FieldInfoData {
                name: "SwordAttackDurationTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_duration_time_max_s),
            },
            FieldInfoData {
                name: "PauseSwordAttackDurationTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, pause_sword_attack_duration_time_min_s),
            },
            FieldInfoData {
                name: "PauseSwordAttackDurationTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, pause_sword_attack_duration_time_max_s),
            },
            FieldInfoData {
                name: "SwordAttackDistanceMetersMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_distance_meters_min),
            },
            FieldInfoData {
                name: "SwordAttackDistanceMetersMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sword_attack_distance_meters_max),
            },
            FieldInfoData {
                name: "DebugDrawInputDetails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_input_details),
            },
            FieldInfoData {
                name: "DebugWindowPositionScaleOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_position_scale_offset_x),
            },
            FieldInfoData {
                name: "DebugWindowPositionScaleOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_position_scale_offset_y),
            },
            FieldInfoData {
                name: "DebugWindowWidth",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_width),
            },
            FieldInfoData {
                name: "DebugWindowHeight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_window_height),
            },
            FieldInfoData {
                name: "PathLookAheadMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, path_look_ahead_meters),
            },
            FieldInfoData {
                name: "PathLookRightMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, path_look_right_meters),
            },
            FieldInfoData {
                name: "WaypointToleranceMeters",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, waypoint_tolerance_meters),
            },
            FieldInfoData {
                name: "DebugDrawAimAtPositions",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, debug_draw_aim_at_positions),
            },
            FieldInfoData {
                name: "EvasiveManeuversVehicleScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_vehicle_scale),
            },
            FieldInfoData {
                name: "VehicleAimNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_aim_noise_scale),
            },
            FieldInfoData {
                name: "ResetSettingsOnLevelUnload",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, reset_settings_on_level_unload),
            },
            FieldInfoData {
                name: "SwordGuardDurationTimeMinS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sword_guard_duration_time_min_s),
            },
            FieldInfoData {
                name: "SwordGuardDurationTimeMaxS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, sword_guard_duration_time_max_s),
            },
            FieldInfoData {
                name: "AimNoiseScaleTeam1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, aim_noise_scale_team1),
            },
            FieldInfoData {
                name: "AimNoiseScaleTeam2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, aim_noise_scale_team2),
            },
            FieldInfoData {
                name: "HeroStrafeProbabilityPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, hero_strafe_probability_per_frame),
            },
            FieldInfoData {
                name: "EmoteProbabilityAfterPlayersDeath",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, emote_probability_after_players_death),
            },
            FieldInfoData {
                name: "EmoteDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, emote_duration),
            },
            FieldInfoData {
                name: "MeleeIntervalS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, melee_interval_s),
            },
            FieldInfoData {
                name: "MeleeDistanceM",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, melee_distance_m),
            },
            FieldInfoData {
                name: "AllowEvasiveManouversOOB",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, allow_evasive_manouvers_o_o_b),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_enabled),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckDistanceM",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_distance_m),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckHeightDistanceM",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_height_distance_m),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckHeightOffsetM",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_height_offset_m),
            },
            FieldInfoData {
                name: "EvasiveManeuversGroundCheckCooldownS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_ground_check_cooldown_s),
            },
            FieldInfoData {
                name: "EvasiveManeuversVehiclesEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, evasive_maneuvers_vehicles_enabled),
            },
            FieldInfoData {
                name: "VehicleMinimumForwardThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_minimum_forward_throttle),
            },
            FieldInfoData {
                name: "VehicleUseCharacterThrottle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AutoPlayerSettings, vehicle_use_character_throttle),
            },
        ],
    }),
    array_type: Some(AUTOPLAYERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoPlayerSettings {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERSETTINGS_TYPE_INFO
    }
}


pub const AUTOPLAYERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAutoPlayerSettingsEntity {
}

pub const SERVERAUTOPLAYERSETTINGSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerSettingsEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAUTOPLAYERSETTINGSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAutoPlayerSettingsEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAUTOPLAYERSETTINGSENTITY_TYPE_INFO
    }
}


pub const SERVERAUTOPLAYERSETTINGSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerSettingsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("ServerAutoPlayerSettingsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAutoPlayerManagerEntity {
}

pub const SERVERAUTOPLAYERMANAGERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerManagerEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAUTOPLAYERMANAGERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAutoPlayerManagerEntity {
    fn type_info() -> &'static TypeInfo {
        SERVERAUTOPLAYERMANAGERENTITY_TYPE_INFO
    }
}


pub const SERVERAUTOPLAYERMANAGERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAutoPlayerManagerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("ServerAutoPlayerManagerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerObjectiveEntityBase {
}

pub const AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYEROBJECTIVEENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerObjectiveEntityBase {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO
    }
}


pub const AUTOPLAYEROBJECTIVEENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerObjectiveEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerObjectiveEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerMoveObjectiveEntity {
}

pub const AUTOPLAYERMOVEOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERMOVEOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerMoveObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERMOVEOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const AUTOPLAYERMOVEOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerMoveObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerMoveObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerInteractObjectiveEntity {
}

pub const AUTOPLAYERINTERACTOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERINTERACTOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerInteractObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERINTERACTOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const AUTOPLAYERINTERACTOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerInteractObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerInteractObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerFollowObjectiveEntity {
}

pub const AUTOPLAYERFOLLOWOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERFOLLOWOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerFollowObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERFOLLOWOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const AUTOPLAYERFOLLOWOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerFollowObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerFollowObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerDefendObjectiveEntity {
}

pub const AUTOPLAYERDEFENDOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERDEFENDOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerDefendObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERDEFENDOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const AUTOPLAYERDEFENDOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerDefendObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerDefendObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutoPlayerAttackObjectiveEntity {
}

pub const AUTOPLAYERATTACKOBJECTIVEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntity",
    flags: MemberInfoFlags::new(101),
    module: "AutoPlayers",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOPLAYEROBJECTIVEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(AUTOPLAYERATTACKOBJECTIVEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AutoPlayerAttackObjectiveEntity {
    fn type_info() -> &'static TypeInfo {
        AUTOPLAYERATTACKOBJECTIVEENTITY_TYPE_INFO
    }
}


pub const AUTOPLAYERATTACKOBJECTIVEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoPlayerAttackObjectiveEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AutoPlayers",
    data: TypeInfoData::Array("AutoPlayerAttackObjectiveEntity-Array"),
    array_type: None,
    alignment: 8,
};


