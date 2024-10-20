use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_weapon_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(WEAPONMISCMODIFIERSETTINGS_TYPE_INFO);
    registry.register_type(WEAPONMISCMODIFIERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONSWAYDATA_TYPE_INFO);
    registry.register_type(WEAPONSWAYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONSTANTS_TYPE_INFO);
    registry.register_type(CONSTANTS_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONUNLOCKS_TYPE_INFO);
    registry.register_type(WEAPONUNLOCKS_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONSLOT_TYPE_INFO);
    registry.register_type(WEAPONSLOT_ARRAY_TYPE_INFO);
    registry.register_type(GEARSLOT_TYPE_INFO);
    registry.register_type(GEARSLOT_ARRAY_TYPE_INFO);
    registry.register_type(CHARACTERSIMPLEWEAPONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(CHARACTERSIMPLEWEAPONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONINPUTROUTERCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WEAPONINPUTROUTERCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WEAPONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONOVERHEATCOMBINABLEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONOVERHEATCOMBINABLEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONOVERHEATMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONOVERHEATMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONRECOILCOMBINABLEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONRECOILCOMBINABLEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONRECOILMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONRECOILMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONOWNERMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONOWNERMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONDISPERSIONCOMBINABLEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONDISPERSIONCOMBINABLEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONDISPERSIONMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONDISPERSIONMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONCROSSHAIRTYPEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONCROSSHAIRTYPEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONMAGAZINEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONMAGAZINEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRELOGICCOMBINABLEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONFIRELOGICCOMBINABLEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRELOGICMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONFIRELOGICMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONSHOTCOMBINABLEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONSHOTCOMBINABLEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONSHOTMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONSHOTMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONPROJECTILEMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONPROJECTILEMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONSOUNDMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONSOUNDMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRINGEFFECTSMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONFIRINGEFFECTSMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRINGDATAMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONFIRINGDATAMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONMISCMODIFIER_TYPE_INFO);
    registry.register_type(WEAPONMISCMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO);
    registry.register_type(WEAPONMODIFIERDYNAMICBASE_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONMODIFIERBASE_TYPE_INFO);
    registry.register_type(WEAPONMODIFIERBASE_ARRAY_TYPE_INFO);
    registry.register_type(LASERPAINTERDATA_TYPE_INFO);
    registry.register_type(LASERPAINTERDATA_ARRAY_TYPE_INFO);
    registry.register_type(LASERDESIGNATORDATA_TYPE_INFO);
    registry.register_type(LASERDESIGNATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCKINGWEAPONDATA_TYPE_INFO);
    registry.register_type(LOCKINGWEAPONDATA_ARRAY_TYPE_INFO);
    registry.register_type(ARTILLERYSTRIKEWEAPONDATA_TYPE_INFO);
    registry.register_type(ARTILLERYSTRIKEWEAPONDATA_ARRAY_TYPE_INFO);
    registry.register_type(ARTILLERYDISPERSIONDATA_TYPE_INFO);
    registry.register_type(ARTILLERYDISPERSIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(MORTARSTRIKEWEAPONDATA_TYPE_INFO);
    registry.register_type(MORTARSTRIKEWEAPONDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONDATA_TYPE_INFO);
    registry.register_type(WEAPONDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRINGDATAASSET_TYPE_INFO);
    registry.register_type(WEAPONFIRINGDATAASSET_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRINGDATA_TYPE_INFO);
    registry.register_type(WEAPONFIRINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(TERTIARYFIREDATA_TYPE_INFO);
    registry.register_type(TERTIARYFIREDATA_ARRAY_TYPE_INFO);
    registry.register_type(SECONDARYFIREDATA_TYPE_INFO);
    registry.register_type(SECONDARYFIREDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRINGRARENETSTATEDATA_TYPE_INFO);
    registry.register_type(WEAPONFIRINGRARENETSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRINGNETSTATEDATA_TYPE_INFO);
    registry.register_type(WEAPONFIRINGNETSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONNETWORKSTATE_TYPE_INFO);
    registry.register_type(WEAPONNETWORKSTATE_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONFIRINGEVENT_TYPE_INFO);
    registry.register_type(WEAPONFIRINGEVENT_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONSUPPRESSIONDATA_TYPE_INFO);
    registry.register_type(WEAPONSUPPRESSIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RUMBLEFIRINGDATA_TYPE_INFO);
    registry.register_type(RUMBLEFIRINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(FIRINGFUNCTIONDATA_TYPE_INFO);
    registry.register_type(FIRINGFUNCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONDISPERSION_TYPE_INFO);
    registry.register_type(WEAPONDISPERSION_ARRAY_TYPE_INFO);
    registry.register_type(OVERHEATDATA_TYPE_INFO);
    registry.register_type(OVERHEATDATA_ARRAY_TYPE_INFO);
    registry.register_type(FIREEFFECTDATA_TYPE_INFO);
    registry.register_type(FIREEFFECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FIRINGDISPERSIONDATA_TYPE_INFO);
    registry.register_type(FIRINGDISPERSIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(FIRELOGICDATA_TYPE_INFO);
    registry.register_type(FIRELOGICDATA_ARRAY_TYPE_INFO);
    registry.register_type(RECOILDATA_TYPE_INFO);
    registry.register_type(RECOILDATA_ARRAY_TYPE_INFO);
    registry.register_type(RELOADLOGIC_TYPE_INFO);
    registry.register_type(RELOADLOGIC_ARRAY_TYPE_INFO);
    registry.register_type(RELOADTYPE_TYPE_INFO);
    registry.register_type(RELOADTYPE_ARRAY_TYPE_INFO);
    registry.register_type(FIRELOGICTYPE_TYPE_INFO);
    registry.register_type(FIRELOGICTYPE_ARRAY_TYPE_INFO);
    registry.register_type(BOLTACTIONDATA_TYPE_INFO);
    registry.register_type(BOLTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(HOLDANDRELEASEDATA_TYPE_INFO);
    registry.register_type(HOLDANDRELEASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHOTCONFIGDATA_TYPE_INFO);
    registry.register_type(SHOTCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(INITIALSPEEDSCALEBYPITCHDATA_TYPE_INFO);
    registry.register_type(INITIALSPEEDSCALEBYPITCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(INITIALDIRECTIONSCALEBYPITCHDATA_TYPE_INFO);
    registry.register_type(INITIALDIRECTIONSCALEBYPITCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEALINGSPHEREDATA_TYPE_INFO);
    registry.register_type(HEALINGSPHEREDATA_ARRAY_TYPE_INFO);
    registry.register_type(MISSILEENTITYDATA_TYPE_INFO);
    registry.register_type(MISSILEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(NEARTARGETDETONATIONDATA_TYPE_INFO);
    registry.register_type(NEARTARGETDETONATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(MISSILEUNGUIDEDDATA_TYPE_INFO);
    registry.register_type(MISSILEUNGUIDEDDATA_ARRAY_TYPE_INFO);
    registry.register_type(MISSILELOCKABLEINFODATA_TYPE_INFO);
    registry.register_type(MISSILELOCKABLEINFODATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCKINGCONTROLLERDATA_TYPE_INFO);
    registry.register_type(LOCKINGCONTROLLERDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCKINGANDHOMINGDATA_TYPE_INFO);
    registry.register_type(LOCKINGANDHOMINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZOOMLEVELLOCKDATA_TYPE_INFO);
    registry.register_type(ZOOMLEVELLOCKDATA_ARRAY_TYPE_INFO);
    registry.register_type(WARNTARGET_TYPE_INFO);
    registry.register_type(WARNTARGET_ARRAY_TYPE_INFO);
    registry.register_type(BULLETENTITYDATA_TYPE_INFO);
    registry.register_type(BULLETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(GHOSTEDPROJECTILEENTITYDATA_TYPE_INFO);
    registry.register_type(GHOSTEDPROJECTILEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MESHPROJECTILEENTITYDATA_TYPE_INFO);
    registry.register_type(MESHPROJECTILEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MISSILEPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(MISSILEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROJECTILEENTITYDATA_TYPE_INFO);
    registry.register_type(PROJECTILEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROJECTILEBLUEPRINT_TYPE_INFO);
    registry.register_type(PROJECTILEBLUEPRINT_ARRAY_TYPE_INFO);
    registry.register_type(ANTHITREACTIONWEAPONTYPE_TYPE_INFO);
    registry.register_type(ANTHITREACTIONWEAPONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(AMMOCONFIGDATA_TYPE_INFO);
    registry.register_type(AMMOCONFIGDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEAPONUNLOCKASSET_TYPE_INFO);
    registry.register_type(WEAPONUNLOCKASSET_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponMiscModifierSettings {
    pub enable_breath_control: bool,
    pub can_be_in_supported_shooting: bool,
    pub un_zoom_on_bolt_action: bool,
    pub hold_bolt_action_until_zoom_release: bool,
    pub is_silenced: bool,
}

pub const WEAPONMISCMODIFIERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifierSettings",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EnableBreathControl",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifierSettings, enable_breath_control),
            },
            FieldInfoData {
                name: "CanBeInSupportedShooting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifierSettings, can_be_in_supported_shooting),
            },
            FieldInfoData {
                name: "UnZoomOnBoltAction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifierSettings, un_zoom_on_bolt_action),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilZoomRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifierSettings, hold_bolt_action_until_zoom_release),
            },
            FieldInfoData {
                name: "IsSilenced",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifierSettings, is_silenced),
            },
        ],
    }),
    array_type: Some(WEAPONMISCMODIFIERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponMiscModifierSettings {
    fn type_info() -> &'static TypeInfo {
        WEAPONMISCMODIFIERSETTINGS_TYPE_INFO
    }
}


pub const WEAPONMISCMODIFIERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifierSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponMiscModifierSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponSwayData {
}

pub const WEAPONSWAYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSwayData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONSWAYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponSwayData {
    fn type_info() -> &'static TypeInfo {
        WEAPONSWAYDATA_TYPE_INFO
    }
}


pub const WEAPONSWAYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSwayData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSwayData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum Constants {
    #[default]
    WeaponComponent_BarrelIndexBits = 4,
}

pub const CONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Constants",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(CONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Constants {
    fn type_info() -> &'static TypeInfo {
        CONSTANTS_TYPE_INFO
    }
}


pub const CONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Constants-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("Constants-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WeaponUnlocks {
    #[default]
    WeaponUnlocks_MaxAmount = 8,
}

pub const WEAPONUNLOCKS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlocks",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONUNLOCKS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponUnlocks {
    fn type_info() -> &'static TypeInfo {
        WEAPONUNLOCKS_TYPE_INFO
    }
}


pub const WEAPONUNLOCKS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlocks-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponUnlocks-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WeaponSlot {
    #[default]
    WeaponSlot_0 = 0,
    WeaponSlot_1 = 1,
    WeaponSlot_2 = 2,
    WeaponSlot_3 = 3,
    WeaponSlot_4 = 4,
    WeaponSlot_5 = 5,
    WeaponSlot_6 = 6,
    WeaponSlot_7 = 7,
    WeaponSlot_8 = 8,
    WeaponSlot_9 = 9,
    WeaponSlot_NumSlots = 10,
    WeaponSlot_NotDefined = 11,
}

pub const WEAPONSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSlot",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONSLOT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponSlot {
    fn type_info() -> &'static TypeInfo {
        WEAPONSLOT_TYPE_INFO
    }
}


pub const WEAPONSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSlot-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GearSlot {
    #[default]
    GearSlot_Melee = 0,
    GearSlot_Sidearm = 1,
    GearSlot_Primary = 2,
    GearSlot_Auxiliary = 3,
    GearSlot_Secondary = 4,
    GearSlot_Gadget1 = 5,
    GearSlot_Gadget2 = 6,
    GearSlot_Gadget3 = 7,
    GearSlot_Gadget4 = 8,
    GearSlot_Gadget5 = 9,
    GearSlot_Gadget6 = 10,
    GearSlot_Gadget7 = 11,
    GearSlot_GearSlotCount = 12,
    GearSlot_NotEquipped = 13,
}

pub const GEARSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearSlot",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(GEARSLOT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearSlot {
    fn type_info() -> &'static TypeInfo {
        GEARSLOT_TYPE_INFO
    }
}


pub const GEARSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("GearSlot-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CharacterSimpleWeaponComponentData {
    pub realm: super::core::Realm,
    pub damage_giver_name: String,
    pub weapon_offset: super::core::LinearTransform,
    pub fire_target: super::core::LinearTransform,
    pub weapon_firing: WeaponFiringData,
}

pub const CHARACTERSIMPLEWEAPONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSimpleWeaponComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, realm),
            },
            FieldInfoData {
                name: "DamageGiverName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, damage_giver_name),
            },
            FieldInfoData {
                name: "WeaponOffset",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, weapon_offset),
            },
            FieldInfoData {
                name: "FireTarget",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, fire_target),
            },
            FieldInfoData {
                name: "WeaponFiring",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONFIRINGDATA_TYPE_INFO,
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, weapon_firing),
            },
        ],
    }),
    array_type: Some(CHARACTERSIMPLEWEAPONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterSimpleWeaponComponentData {
    fn type_info() -> &'static TypeInfo {
        CHARACTERSIMPLEWEAPONCOMPONENTDATA_TYPE_INFO
    }
}


pub const CHARACTERSIMPLEWEAPONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSimpleWeaponComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("CharacterSimpleWeaponComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponInputRouterComponentData {
    pub max_fire_rate: f32,
    pub rotation_count: u32,
}

pub const WEAPONINPUTROUTERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponInputRouterComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxFireRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponInputRouterComponentData, max_fire_rate),
            },
            FieldInfoData {
                name: "RotationCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponInputRouterComponentData, rotation_count),
            },
        ],
    }),
    array_type: Some(WEAPONINPUTROUTERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponInputRouterComponentData {
    fn type_info() -> &'static TypeInfo {
        WEAPONINPUTROUTERCOMPONENTDATA_TYPE_INFO
    }
}


pub const WEAPONINPUTROUTERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponInputRouterComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponInputRouterComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponComponentData {
    pub projectile_spawn_offset: super::core::Vec3,
    pub sequential_firing: bool,
    pub weapon_firing: WeaponFiringData,
    pub damage_giver_name: String,
    pub a_i_data: super::gameplay_sim::GameAIWeaponData,
    pub custom_weapon_type: WeaponData,
    pub impulse_strength: f32,
    pub classification: super::game_shared::WeaponClassification,
    pub reload_time_multiplier: f32,
    pub damage_multiplier: f32,
    pub explosion_damage_multiplier: f32,
    pub overheat_drop_per_second_multiplier: f32,
    pub rate_of_fire_multiplier: f32,
    pub lock_time_multiplier: f32,
    pub locking_acceptance_angle_multiplier: f32,
    pub target_position_override: super::core::Vec3,
    pub weapon_item_hash: u32,
}

pub const WEAPONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BONECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProjectileSpawnOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, projectile_spawn_offset),
            },
            FieldInfoData {
                name: "SequentialFiring",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, sequential_firing),
            },
            FieldInfoData {
                name: "WeaponFiring",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONFIRINGDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, weapon_firing),
            },
            FieldInfoData {
                name: "DamageGiverName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, damage_giver_name),
            },
            FieldInfoData {
                name: "AIData",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEAIWEAPONDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, a_i_data),
            },
            FieldInfoData {
                name: "CustomWeaponType",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, custom_weapon_type),
            },
            FieldInfoData {
                name: "ImpulseStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, impulse_strength),
            },
            FieldInfoData {
                name: "Classification",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONCLASSIFICATION_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, classification),
            },
            FieldInfoData {
                name: "ReloadTimeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, reload_time_multiplier),
            },
            FieldInfoData {
                name: "DamageMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, damage_multiplier),
            },
            FieldInfoData {
                name: "ExplosionDamageMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, explosion_damage_multiplier),
            },
            FieldInfoData {
                name: "OverheatDropPerSecondMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, overheat_drop_per_second_multiplier),
            },
            FieldInfoData {
                name: "RateOfFireMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, rate_of_fire_multiplier),
            },
            FieldInfoData {
                name: "LockTimeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, lock_time_multiplier),
            },
            FieldInfoData {
                name: "LockingAcceptanceAngleMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, locking_acceptance_angle_multiplier),
            },
            FieldInfoData {
                name: "TargetPositionOverride",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, target_position_override),
            },
            FieldInfoData {
                name: "WeaponItemHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponComponentData, weapon_item_hash),
            },
        ],
    }),
    array_type: Some(WEAPONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponComponentData {
    fn type_info() -> &'static TypeInfo {
        WEAPONCOMPONENTDATA_TYPE_INFO
    }
}


pub const WEAPONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponOverheatCombinableModifier {
}

pub const WEAPONOVERHEATCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatCombinableModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONOVERHEATMODIFIER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONOVERHEATCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponOverheatCombinableModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONOVERHEATCOMBINABLEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONOVERHEATCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatCombinableModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponOverheatCombinableModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponOverheatModifier {
    pub heat_per_bullet: f32,
    pub heat_drop_per_second: f32,
    pub heat_increase_per_second: f32,
    pub overheated_penalty_time: f32,
    pub overheat_threshold: f32,
    pub overheat_drop_delay: f32,
    pub heat_per_bullet_multiplier: f32,
    pub heat_drop_per_second_multiplier: f32,
    pub heat_increase_per_second_multiplier: f32,
    pub overheated_penalty_time_multiplier: f32,
    pub overheat_drop_delay_multiplier: f32,
    pub overheated_drop_multiplier: f32,
}

pub const WEAPONOVERHEATMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HeatPerBullet",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, heat_per_bullet),
            },
            FieldInfoData {
                name: "HeatDropPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, heat_drop_per_second),
            },
            FieldInfoData {
                name: "HeatIncreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, heat_increase_per_second),
            },
            FieldInfoData {
                name: "OverheatedPenaltyTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, overheated_penalty_time),
            },
            FieldInfoData {
                name: "OverheatThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, overheat_threshold),
            },
            FieldInfoData {
                name: "OverheatDropDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, overheat_drop_delay),
            },
            FieldInfoData {
                name: "HeatPerBulletMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, heat_per_bullet_multiplier),
            },
            FieldInfoData {
                name: "HeatDropPerSecondMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, heat_drop_per_second_multiplier),
            },
            FieldInfoData {
                name: "HeatIncreasePerSecondMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, heat_increase_per_second_multiplier),
            },
            FieldInfoData {
                name: "OverheatedPenaltyTimeMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, overheated_penalty_time_multiplier),
            },
            FieldInfoData {
                name: "OverheatDropDelayMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, overheat_drop_delay_multiplier),
            },
            FieldInfoData {
                name: "OverheatedDropMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponOverheatModifier, overheated_drop_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONOVERHEATMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponOverheatModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONOVERHEATMODIFIER_TYPE_INFO
    }
}


pub const WEAPONOVERHEATMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponOverheatModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponRecoilCombinableModifier {
}

pub const WEAPONRECOILCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilCombinableModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONRECOILMODIFIER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONRECOILCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponRecoilCombinableModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONRECOILCOMBINABLEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONRECOILCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilCombinableModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponRecoilCombinableModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponRecoilModifier {
    pub max_recoil_angle_x: f32,
    pub min_recoil_angle_x: f32,
    pub max_recoil_angle_y: f32,
    pub min_recoil_angle_y: f32,
    pub max_recoil_angle_z: f32,
    pub min_recoil_angle_z: f32,
    pub max_recoil_fov: f32,
    pub min_recoil_fov: f32,
    pub max_recoil_angle_x_multiplier: f32,
    pub min_recoil_angle_x_multiplier: f32,
    pub max_recoil_angle_y_multiplier: f32,
    pub min_recoil_angle_y_multiplier: f32,
    pub max_recoil_angle_z_multiplier: f32,
    pub min_recoil_angle_z_multiplier: f32,
    pub max_recoil_fov_multiplier: f32,
    pub min_recoil_fov_multiplier: f32,
}

pub const WEAPONRECOILMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxRecoilAngleX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_x),
            },
            FieldInfoData {
                name: "MinRecoilAngleX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_x),
            },
            FieldInfoData {
                name: "MaxRecoilAngleY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_y),
            },
            FieldInfoData {
                name: "MinRecoilAngleY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_y),
            },
            FieldInfoData {
                name: "MaxRecoilAngleZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_z),
            },
            FieldInfoData {
                name: "MinRecoilAngleZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_z),
            },
            FieldInfoData {
                name: "MaxRecoilFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_fov),
            },
            FieldInfoData {
                name: "MinRecoilFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_fov),
            },
            FieldInfoData {
                name: "MaxRecoilAngleXMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_x_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilAngleXMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_x_multiplier),
            },
            FieldInfoData {
                name: "MaxRecoilAngleYMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_y_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilAngleYMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_y_multiplier),
            },
            FieldInfoData {
                name: "MaxRecoilAngleZMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_z_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilAngleZMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_z_multiplier),
            },
            FieldInfoData {
                name: "MaxRecoilFovMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_fov_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilFovMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_fov_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONRECOILMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponRecoilModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONRECOILMODIFIER_TYPE_INFO
    }
}


pub const WEAPONRECOILMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponRecoilModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponOwnerModifier {
    pub unlock: WeaponUnlockAsset,
}

pub const WEAPONOWNERMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOwnerModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Unlock",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONUNLOCKASSET_TYPE_INFO,
                rust_offset: offset_of!(WeaponOwnerModifier, unlock),
            },
        ],
    }),
    array_type: Some(WEAPONOWNERMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponOwnerModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONOWNERMODIFIER_TYPE_INFO
    }
}


pub const WEAPONOWNERMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOwnerModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponOwnerModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponDispersionCombinableModifier {
}

pub const WEAPONDISPERSIONCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionCombinableModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDISPERSIONMODIFIER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONDISPERSIONCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponDispersionCombinableModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONDISPERSIONCOMBINABLEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONDISPERSIONCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionCombinableModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponDispersionCombinableModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponDispersionModifier {
    pub horizontal_modifier: f32,
    pub vertical_modifier: f32,
    pub increase_per_shot: f32,
    pub decrease_per_second: f32,
    pub increase_per_shot_multiplier: f32,
    pub decrease_per_second_multiplier: f32,
}

pub const WEAPONDISPERSIONMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HorizontalModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersionModifier, horizontal_modifier),
            },
            FieldInfoData {
                name: "VerticalModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersionModifier, vertical_modifier),
            },
            FieldInfoData {
                name: "IncreasePerShot",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersionModifier, increase_per_shot),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersionModifier, decrease_per_second),
            },
            FieldInfoData {
                name: "IncreasePerShotMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersionModifier, increase_per_shot_multiplier),
            },
            FieldInfoData {
                name: "DecreasePerSecondMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersionModifier, decrease_per_second_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONDISPERSIONMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponDispersionModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONDISPERSIONMODIFIER_TYPE_INFO
    }
}


pub const WEAPONDISPERSIONMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponDispersionModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponCrosshairTypeModifier {
    pub crosshair_type: super::game_shared::CrosshairTypeAsset,
}

pub const WEAPONCROSSHAIRTYPEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponCrosshairTypeModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CrosshairType",
                flags: MemberInfoFlags::new(0),
                field_type: CROSSHAIRTYPEASSET_TYPE_INFO,
                rust_offset: offset_of!(WeaponCrosshairTypeModifier, crosshair_type),
            },
        ],
    }),
    array_type: Some(WEAPONCROSSHAIRTYPEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponCrosshairTypeModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONCROSSHAIRTYPEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONCROSSHAIRTYPEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponCrosshairTypeModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponCrosshairTypeModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponMagazineModifier {
    pub magazine_capacity: i32,
    pub number_of_magazines: i32,
    pub spawn_without_ammo: bool,
}

pub const WEAPONMAGAZINEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMagazineModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MagazineCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponMagazineModifier, magazine_capacity),
            },
            FieldInfoData {
                name: "NumberOfMagazines",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponMagazineModifier, number_of_magazines),
            },
            FieldInfoData {
                name: "SpawnWithoutAmmo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMagazineModifier, spawn_without_ammo),
            },
        ],
    }),
    array_type: Some(WEAPONMAGAZINEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponMagazineModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONMAGAZINEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONMAGAZINEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMagazineModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponMagazineModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponFireLogicCombinableModifier {
}

pub const WEAPONFIRELOGICCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicCombinableModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONFIRELOGICMODIFIER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONFIRELOGICCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFireLogicCombinableModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRELOGICCOMBINABLEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONFIRELOGICCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicCombinableModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFireLogicCombinableModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponFireLogicModifier {
    pub rate_of_fire: f32,
    pub rate_of_fire_for_burst: f32,
    pub rate_of_fire_multiplier: f32,
    pub rate_of_fire_for_burst_multiplier: f32,
}

pub const WEAPONFIRELOGICMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RateOfFire",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire),
            },
            FieldInfoData {
                name: "RateOfFireForBurst",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire_for_burst),
            },
            FieldInfoData {
                name: "RateOfFireMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire_multiplier),
            },
            FieldInfoData {
                name: "RateOfFireForBurstMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire_for_burst_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONFIRELOGICMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFireLogicModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRELOGICMODIFIER_TYPE_INFO
    }
}


pub const WEAPONFIRELOGICMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFireLogicModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponShotCombinableModifier {
}

pub const WEAPONSHOTCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotCombinableModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONSHOTMODIFIER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONSHOTCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponShotCombinableModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONSHOTCOMBINABLEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONSHOTCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotCombinableModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponShotCombinableModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponShotModifier {
    pub initial_speed: super::core::Vec3,
    pub number_of_bullets_per_shell: i32,
    pub number_of_bullets_per_burst: i32,
    pub initial_speed_multiplier: f32,
    pub number_of_bullets_per_shell_multiplier: f32,
    pub number_of_bullets_per_burst_multiplier: f32,
}

pub const WEAPONSHOTMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InitialSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(WeaponShotModifier, initial_speed),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShell",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_shell),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerBurst",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_burst),
            },
            FieldInfoData {
                name: "InitialSpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponShotModifier, initial_speed_multiplier),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShellMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_shell_multiplier),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerBurstMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_burst_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONSHOTMODIFIER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponShotModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONSHOTMODIFIER_TYPE_INFO
    }
}


pub const WEAPONSHOTMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponShotModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponProjectileModifier {
    pub projectile_data: ProjectileEntityData,
    pub projectile: ProjectileBlueprint,
    pub max_count: i32,
}

pub const WEAPONPROJECTILEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponProjectileModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProjectileData",
                flags: MemberInfoFlags::new(0),
                field_type: PROJECTILEENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponProjectileModifier, projectile_data),
            },
            FieldInfoData {
                name: "Projectile",
                flags: MemberInfoFlags::new(0),
                field_type: PROJECTILEBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(WeaponProjectileModifier, projectile),
            },
            FieldInfoData {
                name: "MaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponProjectileModifier, max_count),
            },
        ],
    }),
    array_type: Some(WEAPONPROJECTILEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponProjectileModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONPROJECTILEMODIFIER_TYPE_INFO
    }
}


pub const WEAPONPROJECTILEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponProjectileModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponProjectileModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponSoundModifier {
    pub sound: super::audio::SoundAsset,
    pub mute_primary_sound_when_active: bool,
}

pub const WEAPONSOUNDMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSoundModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Sound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(WeaponSoundModifier, sound),
            },
            FieldInfoData {
                name: "MutePrimarySoundWhenActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponSoundModifier, mute_primary_sound_when_active),
            },
        ],
    }),
    array_type: Some(WEAPONSOUNDMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponSoundModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONSOUNDMODIFIER_TYPE_INFO
    }
}


pub const WEAPONSOUNDMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSoundModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSoundModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponFiringEffectsModifier {
    pub fire_effects1p: Vec<FireEffectData>,
    pub fire_effects3p: Vec<FireEffectData>,
}

pub const WEAPONFIRINGEFFECTSMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEffectsModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FireEffects1p",
                flags: MemberInfoFlags::new(144),
                field_type: FIREEFFECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringEffectsModifier, fire_effects1p),
            },
            FieldInfoData {
                name: "FireEffects3p",
                flags: MemberInfoFlags::new(144),
                field_type: FIREEFFECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringEffectsModifier, fire_effects3p),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGEFFECTSMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringEffectsModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRINGEFFECTSMODIFIER_TYPE_INFO
    }
}


pub const WEAPONFIRINGEFFECTSMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEffectsModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringEffectsModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponFiringDataModifier {
    pub weapon_firing: WeaponFiringData,
}

pub const WEAPONFIRINGDATAMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WeaponFiring",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONFIRINGDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringDataModifier, weapon_firing),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGDATAMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringDataModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRINGDATAMODIFIER_TYPE_INFO
    }
}


pub const WEAPONFIRINGDATAMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringDataModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponMiscModifier {
    pub enable_breath_control: bool,
    pub can_be_in_supported_shooting: bool,
    pub un_zoom_on_bolt_action: bool,
    pub hold_bolt_action_until_zoom_release: bool,
    pub is_silenced: bool,
}

pub const WEAPONMISCMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifier",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EnableBreathControl",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifier, enable_breath_control),
            },
            FieldInfoData {
                name: "CanBeInSupportedShooting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifier, can_be_in_supported_shooting),
            },
            FieldInfoData {
                name: "UnZoomOnBoltAction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifier, un_zoom_on_bolt_action),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilZoomRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifier, hold_bolt_action_until_zoom_release),
            },
            FieldInfoData {
                name: "IsSilenced",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponMiscModifier, is_silenced),
            },
        ],
    }),
    array_type: Some(WEAPONMISCMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponMiscModifier {
    fn type_info() -> &'static TypeInfo {
        WEAPONMISCMODIFIER_TYPE_INFO
    }
}


pub const WEAPONMISCMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponMiscModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponModifierDynamicBase {
}

pub const WEAPONMODIFIERDYNAMICBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierDynamicBase",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONMODIFIERDYNAMICBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponModifierDynamicBase {
    fn type_info() -> &'static TypeInfo {
        WEAPONMODIFIERDYNAMICBASE_TYPE_INFO
    }
}


pub const WEAPONMODIFIERDYNAMICBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierDynamicBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponModifierDynamicBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponModifierBase {
    pub apply_order: i32,
    pub dynamic_update_enabled: bool,
}

pub const WEAPONMODIFIERBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierBase",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ApplyOrder",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponModifierBase, apply_order),
            },
            FieldInfoData {
                name: "DynamicUpdateEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponModifierBase, dynamic_update_enabled),
            },
        ],
    }),
    array_type: Some(WEAPONMODIFIERBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponModifierBase {
    fn type_info() -> &'static TypeInfo {
        WEAPONMODIFIERBASE_TYPE_INFO
    }
}


pub const WEAPONMODIFIERBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponModifierBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LaserPainterData {
    pub time_object_is_painted: f32,
}

pub const LASERPAINTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserPainterData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCKINGWEAPONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeObjectIsPainted",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LaserPainterData, time_object_is_painted),
            },
        ],
    }),
    array_type: Some(LASERPAINTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LaserPainterData {
    fn type_info() -> &'static TypeInfo {
        LASERPAINTERDATA_TYPE_INFO
    }
}


pub const LASERPAINTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserPainterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LaserPainterData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LaserDesignatorData {
    pub post_lock_time: f32,
    pub bomber_time: f32,
    pub bomb_warn_time: f32,
    pub bomber_sound: super::audio::SoundAsset,
}

pub const LASERDESIGNATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserDesignatorData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCKINGWEAPONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PostLockTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LaserDesignatorData, post_lock_time),
            },
            FieldInfoData {
                name: "BomberTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LaserDesignatorData, bomber_time),
            },
            FieldInfoData {
                name: "BombWarnTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LaserDesignatorData, bomb_warn_time),
            },
            FieldInfoData {
                name: "BomberSound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(LaserDesignatorData, bomber_sound),
            },
        ],
    }),
    array_type: Some(LASERDESIGNATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LaserDesignatorData {
    fn type_info() -> &'static TypeInfo {
        LASERDESIGNATORDATA_TYPE_INFO
    }
}


pub const LASERDESIGNATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserDesignatorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LaserDesignatorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LockingWeaponData {
    pub locking_controller: LockingControllerData,
    pub secondary_locking_controller: LockingControllerData,
    pub override_locking_controller_settings: bool,
    pub is_homing: bool,
    pub is_guided: bool,
    pub is_guided_when_zoomed: bool,
    pub is_guided_homing: bool,
    pub fire_only_when_locked_on: bool,
    pub guide_only_when_locked_on: bool,
    pub warn_lock: WarnTarget,
}

pub const LOCKINGWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingWeaponData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LockingController",
                flags: MemberInfoFlags::new(0),
                field_type: LOCKINGCONTROLLERDATA_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, locking_controller),
            },
            FieldInfoData {
                name: "SecondaryLockingController",
                flags: MemberInfoFlags::new(0),
                field_type: LOCKINGCONTROLLERDATA_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, secondary_locking_controller),
            },
            FieldInfoData {
                name: "OverrideLockingControllerSettings",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, override_locking_controller_settings),
            },
            FieldInfoData {
                name: "IsHoming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, is_homing),
            },
            FieldInfoData {
                name: "IsGuided",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, is_guided),
            },
            FieldInfoData {
                name: "IsGuidedWhenZoomed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, is_guided_when_zoomed),
            },
            FieldInfoData {
                name: "IsGuidedHoming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, is_guided_homing),
            },
            FieldInfoData {
                name: "FireOnlyWhenLockedOn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, fire_only_when_locked_on),
            },
            FieldInfoData {
                name: "GuideOnlyWhenLockedOn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, guide_only_when_locked_on),
            },
            FieldInfoData {
                name: "WarnLock",
                flags: MemberInfoFlags::new(0),
                field_type: WARNTARGET_TYPE_INFO,
                rust_offset: offset_of!(LockingWeaponData, warn_lock),
            },
        ],
    }),
    array_type: Some(LOCKINGWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LockingWeaponData {
    fn type_info() -> &'static TypeInfo {
        LOCKINGWEAPONDATA_TYPE_INFO
    }
}


pub const LOCKINGWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingWeaponData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LockingWeaponData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ArtilleryStrikeWeaponData {
    pub max_strike_distance: f32,
    pub strike_radius: f32,
    pub spawn_height: f32,
    pub max_random_spawn_height: f32,
    pub increase_spawn_height_with_distance: bool,
    pub spawn_height_multiplier: f32,
    pub camera: super::gameplay_sim::TargetCameraData,
    pub aiming_camera_height: f32,
    pub strike_camera_height: f32,
    pub aiming_camera_offset: f32,
    pub strike_camera_offset: f32,
    pub aiming_camera_fov: f32,
    pub delay_before_aiming_camera: f32,
    pub strike_camera_fov: f32,
    pub fire_camera_time: f32,
    pub strike_camera_time: f32,
    pub valid_min_distance: f32,
    pub valid_max_distance: f32,
    pub valid_max_angle: f32,
    pub enable_projectile_trails: bool,
    pub enable_camera_rotation: bool,
    pub artillery_dispersion: ArtilleryDispersionData,
}

pub const ARTILLERYSTRIKEWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryStrikeWeaponData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxStrikeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, max_strike_distance),
            },
            FieldInfoData {
                name: "StrikeRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_radius),
            },
            FieldInfoData {
                name: "SpawnHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, spawn_height),
            },
            FieldInfoData {
                name: "MaxRandomSpawnHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, max_random_spawn_height),
            },
            FieldInfoData {
                name: "IncreaseSpawnHeightWithDistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, increase_spawn_height_with_distance),
            },
            FieldInfoData {
                name: "SpawnHeightMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, spawn_height_multiplier),
            },
            FieldInfoData {
                name: "Camera",
                flags: MemberInfoFlags::new(0),
                field_type: TARGETCAMERADATA_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, camera),
            },
            FieldInfoData {
                name: "AimingCameraHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, aiming_camera_height),
            },
            FieldInfoData {
                name: "StrikeCameraHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_height),
            },
            FieldInfoData {
                name: "AimingCameraOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, aiming_camera_offset),
            },
            FieldInfoData {
                name: "StrikeCameraOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_offset),
            },
            FieldInfoData {
                name: "AimingCameraFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, aiming_camera_fov),
            },
            FieldInfoData {
                name: "DelayBeforeAimingCamera",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, delay_before_aiming_camera),
            },
            FieldInfoData {
                name: "StrikeCameraFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_fov),
            },
            FieldInfoData {
                name: "FireCameraTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, fire_camera_time),
            },
            FieldInfoData {
                name: "StrikeCameraTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_time),
            },
            FieldInfoData {
                name: "ValidMinDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, valid_min_distance),
            },
            FieldInfoData {
                name: "ValidMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, valid_max_distance),
            },
            FieldInfoData {
                name: "ValidMaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, valid_max_angle),
            },
            FieldInfoData {
                name: "EnableProjectileTrails",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, enable_projectile_trails),
            },
            FieldInfoData {
                name: "EnableCameraRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, enable_camera_rotation),
            },
            FieldInfoData {
                name: "ArtilleryDispersion",
                flags: MemberInfoFlags::new(0),
                field_type: ARTILLERYDISPERSIONDATA_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, artillery_dispersion),
            },
        ],
    }),
    array_type: Some(ARTILLERYSTRIKEWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArtilleryStrikeWeaponData {
    fn type_info() -> &'static TypeInfo {
        ARTILLERYSTRIKEWEAPONDATA_TYPE_INFO
    }
}


pub const ARTILLERYSTRIKEWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryStrikeWeaponData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ArtilleryStrikeWeaponData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ArtilleryDispersionData {
    pub dispersion_active: bool,
    pub max_dispersion: f32,
    pub dispersion_aim_move: f32,
    pub dispersion_aim_move_threshold: f32,
    pub dispersion_firing: f32,
    pub dispersion_deployment: f32,
    pub decrease_per_second: f32,
}

pub const ARTILLERYDISPERSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryDispersionData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DispersionActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_active),
            },
            FieldInfoData {
                name: "MaxDispersion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryDispersionData, max_dispersion),
            },
            FieldInfoData {
                name: "DispersionAimMove",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_aim_move),
            },
            FieldInfoData {
                name: "DispersionAimMoveThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_aim_move_threshold),
            },
            FieldInfoData {
                name: "DispersionFiring",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_firing),
            },
            FieldInfoData {
                name: "DispersionDeployment",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_deployment),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ArtilleryDispersionData, decrease_per_second),
            },
        ],
    }),
    array_type: Some(ARTILLERYDISPERSIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ArtilleryDispersionData {
    fn type_info() -> &'static TypeInfo {
        ARTILLERYDISPERSIONDATA_TYPE_INFO
    }
}


pub const ARTILLERYDISPERSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryDispersionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ArtilleryDispersionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MortarStrikeWeaponData {
    pub holding_tolerance: f32,
    pub max_strike_distance: f32,
    pub strike_radius: f32,
    pub max_random_spawn_height: f32,
}

pub const MORTARSTRIKEWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MortarStrikeWeaponData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HoldingTolerance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MortarStrikeWeaponData, holding_tolerance),
            },
            FieldInfoData {
                name: "MaxStrikeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MortarStrikeWeaponData, max_strike_distance),
            },
            FieldInfoData {
                name: "StrikeRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MortarStrikeWeaponData, strike_radius),
            },
            FieldInfoData {
                name: "MaxRandomSpawnHeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MortarStrikeWeaponData, max_random_spawn_height),
            },
        ],
    }),
    array_type: Some(MORTARSTRIKEWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MortarStrikeWeaponData {
    fn type_info() -> &'static TypeInfo {
        MORTARSTRIKEWEAPONDATA_TYPE_INFO
    }
}


pub const MORTARSTRIKEWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MortarStrikeWeaponData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MortarStrikeWeaponData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponData {
    pub show_laser_painted_vehicles: bool,
    pub apply_power_to_speed: bool,
}

pub const WEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TOOLDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShowLaserPaintedVehicles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponData, show_laser_painted_vehicles),
            },
            FieldInfoData {
                name: "ApplyPowerToSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponData, apply_power_to_speed),
            },
        ],
    }),
    array_type: Some(WEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponData {
    fn type_info() -> &'static TypeInfo {
        WEAPONDATA_TYPE_INFO
    }
}


pub const WEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponFiringDataAsset {
}

pub const WEAPONFIRINGDATAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataAsset",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEDATACONTAINERASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONFIRINGDATAASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringDataAsset {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRINGDATAASSET_TYPE_INFO
    }
}


pub const WEAPONFIRINGDATAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringDataAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponFiringData {
    pub primary_fire: FiringFunctionData,
    pub secondary_fire: SecondaryFireData,
    pub tertiary_fire: TertiaryFireData,
    pub deploy_time: f32,
    pub reactivate_cooldown_time: f32,
    pub disable_zoom_on_deploy_time: f32,
    pub alt_deploy_time: f32,
    pub alt_deploy_id: i32,
    pub use_auto_aiming: bool,
    pub weapon_sway: WeaponSwayData,
    pub inflict_self_damage: bool,
    pub rumble: RumbleFiringData,
    pub support_delay_stand: f32,
    pub support_delay_prone: f32,
    pub show_enemy_nametag_on_aim: bool,
    pub reload_whole_mags: bool,
    pub disable_reload_while_sprinting: bool,
    pub abort_reload_on_sprint: bool,
    pub use_remote_damage_giver_info: bool,
}

pub const WEAPONFIRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEDATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PrimaryFire",
                flags: MemberInfoFlags::new(0),
                field_type: FIRINGFUNCTIONDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, primary_fire),
            },
            FieldInfoData {
                name: "SecondaryFire",
                flags: MemberInfoFlags::new(0),
                field_type: SECONDARYFIREDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, secondary_fire),
            },
            FieldInfoData {
                name: "TertiaryFire",
                flags: MemberInfoFlags::new(0),
                field_type: TERTIARYFIREDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, tertiary_fire),
            },
            FieldInfoData {
                name: "DeployTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, deploy_time),
            },
            FieldInfoData {
                name: "ReactivateCooldownTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, reactivate_cooldown_time),
            },
            FieldInfoData {
                name: "DisableZoomOnDeployTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, disable_zoom_on_deploy_time),
            },
            FieldInfoData {
                name: "AltDeployTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, alt_deploy_time),
            },
            FieldInfoData {
                name: "AltDeployId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, alt_deploy_id),
            },
            FieldInfoData {
                name: "UseAutoAiming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, use_auto_aiming),
            },
            FieldInfoData {
                name: "WeaponSway",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONSWAYDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, weapon_sway),
            },
            FieldInfoData {
                name: "InflictSelfDamage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, inflict_self_damage),
            },
            FieldInfoData {
                name: "Rumble",
                flags: MemberInfoFlags::new(0),
                field_type: RUMBLEFIRINGDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, rumble),
            },
            FieldInfoData {
                name: "SupportDelayStand",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, support_delay_stand),
            },
            FieldInfoData {
                name: "SupportDelayProne",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, support_delay_prone),
            },
            FieldInfoData {
                name: "ShowEnemyNametagOnAim",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, show_enemy_nametag_on_aim),
            },
            FieldInfoData {
                name: "ReloadWholeMags",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, reload_whole_mags),
            },
            FieldInfoData {
                name: "DisableReloadWhileSprinting",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, disable_reload_while_sprinting),
            },
            FieldInfoData {
                name: "AbortReloadOnSprint",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, abort_reload_on_sprint),
            },
            FieldInfoData {
                name: "UseRemoteDamageGiverInfo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringData, use_remote_damage_giver_info),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringData {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRINGDATA_TYPE_INFO
    }
}


pub const WEAPONFIRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TertiaryFireData {
    pub wait_on_trigger_release: bool,
    pub wait_on_primary_trigger_release: bool,
    pub firing_delay: f32,
    pub pending_fire_window: f32,
    pub pending_fire_ignore_fire_rate_window: f32,
}

pub const TERTIARYFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TertiaryFireData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WaitOnTriggerRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TertiaryFireData, wait_on_trigger_release),
            },
            FieldInfoData {
                name: "WaitOnPrimaryTriggerRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TertiaryFireData, wait_on_primary_trigger_release),
            },
            FieldInfoData {
                name: "FiringDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TertiaryFireData, firing_delay),
            },
            FieldInfoData {
                name: "PendingFireWindow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TertiaryFireData, pending_fire_window),
            },
            FieldInfoData {
                name: "PendingFireIgnoreFireRateWindow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TertiaryFireData, pending_fire_ignore_fire_rate_window),
            },
        ],
    }),
    array_type: Some(TERTIARYFIREDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TertiaryFireData {
    fn type_info() -> &'static TypeInfo {
        TERTIARYFIREDATA_TYPE_INFO
    }
}


pub const TERTIARYFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TertiaryFireData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("TertiaryFireData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SecondaryFireData {
    pub wait_on_trigger_release: bool,
    pub wait_on_primary_trigger_release: bool,
    pub firing_delay: f32,
    pub pending_fire_window: f32,
    pub pending_fire_ignore_fire_rate_window: f32,
}

pub const SECONDARYFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SecondaryFireData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WaitOnTriggerRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SecondaryFireData, wait_on_trigger_release),
            },
            FieldInfoData {
                name: "WaitOnPrimaryTriggerRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SecondaryFireData, wait_on_primary_trigger_release),
            },
            FieldInfoData {
                name: "FiringDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SecondaryFireData, firing_delay),
            },
            FieldInfoData {
                name: "PendingFireWindow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SecondaryFireData, pending_fire_window),
            },
            FieldInfoData {
                name: "PendingFireIgnoreFireRateWindow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SecondaryFireData, pending_fire_ignore_fire_rate_window),
            },
        ],
    }),
    array_type: Some(SECONDARYFIREDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SecondaryFireData {
    fn type_info() -> &'static TypeInfo {
        SECONDARYFIREDATA_TYPE_INFO
    }
}


pub const SECONDARYFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SecondaryFireData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("SecondaryFireData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponFiringRareNetStateData {
    pub is_detonator_active: bool,
    pub fire_logic_type: FireLogicType,
}

pub const WEAPONFIRINGRARENETSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringRareNetStateData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsDetonatorActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringRareNetStateData, is_detonator_active),
            },
            FieldInfoData {
                name: "FireLogicType",
                flags: MemberInfoFlags::new(0),
                field_type: FIRELOGICTYPE_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringRareNetStateData, fire_logic_type),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGRARENETSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WeaponFiringRareNetStateData {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRINGRARENETSTATEDATA_TYPE_INFO
    }
}


pub const WEAPONFIRINGRARENETSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringRareNetStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringRareNetStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponFiringNetStateData {
    pub write_weapon_state: WeaponNetworkState,
    pub fired_count: u32,
    pub tick_bits: u32,
    pub primary_heat: f32,
    pub reload_stage: u32,
    pub charge_up: u32,
    pub in_pre_fire: bool,
    pub primary_is_overheated: bool,
    pub fired_hold_and_release_modifier: i32,
}

pub const WEAPONFIRINGNETSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringNetStateData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WriteWeaponState",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONNETWORKSTATE_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, write_weapon_state),
            },
            FieldInfoData {
                name: "FiredCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, fired_count),
            },
            FieldInfoData {
                name: "TickBits",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, tick_bits),
            },
            FieldInfoData {
                name: "PrimaryHeat",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, primary_heat),
            },
            FieldInfoData {
                name: "ReloadStage",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, reload_stage),
            },
            FieldInfoData {
                name: "ChargeUp",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, charge_up),
            },
            FieldInfoData {
                name: "InPreFire",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, in_pre_fire),
            },
            FieldInfoData {
                name: "PrimaryIsOverheated",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, primary_is_overheated),
            },
            FieldInfoData {
                name: "FiredHoldAndReleaseModifier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponFiringNetStateData, fired_hold_and_release_modifier),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGNETSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WeaponFiringNetStateData {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRINGNETSTATEDATA_TYPE_INFO
    }
}


pub const WEAPONFIRINGNETSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringNetStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringNetStateData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WeaponNetworkState {
    #[default]
    WeaponIdle = 0,
    WeaponSingleFiring = 1,
    WeaponAutomaticFiring = 2,
    WeaponReloading = 3,
    WeaponBoltAction = 4,
    WeaponHoldAndReleaseHold = 5,
}

pub const WEAPONNETWORKSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponNetworkState",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONNETWORKSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponNetworkState {
    fn type_info() -> &'static TypeInfo {
        WEAPONNETWORKSTATE_TYPE_INFO
    }
}


pub const WEAPONNETWORKSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponNetworkState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponNetworkState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WeaponFiringEvent {
    #[default]
    WeaponFiringEvent_Push = 0,
    WeaponFiringEvent_Pop = 1,
    WeaponFiringEvent_PrimaryStartedFiringCallback = 2,
    WeaponFiringEvent_PrimaryFireCallback = 3,
    WeaponFiringEvent_PrimaryFireReleaseCallback = 4,
    WeaponFiringEvent_PrimaryFireShotSpawnedCallback = 5,
    WeaponFiringEvent_PrimaryFireAutomaticBeginCallback = 6,
    WeaponFiringEvent_PrimaryFireAutomaticEndCallback = 7,
    WeaponFiringEvent_PrimaryStoppedFiringCallback = 8,
    WeaponFiringEvent_ReloadPrimaryCallback = 9,
    WeaponFiringEvent_ReloadPrimaryEndCallback = 10,
    WeaponFiringEvent_BoltActionCallback = 11,
    WeaponFiringEvent_BoltActionEndCallback = 12,
    WeaponFiringEvent_DetonationSwitchCallback = 13,
    WeaponFiringEvent_HoldAndReleaseReleaseCallback = 14,
    WeaponFiringEvent_UpdateRequired = 15,
    WeaponFiringEvent_HoldAndReleaseBeginCallback = 16,
    WeaponFiringEvent_PreFireStart = 17,
    WeaponFiringEvent_PreFireEnd = 18,
}

pub const WEAPONFIRINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEvent",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONFIRINGEVENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponFiringEvent {
    fn type_info() -> &'static TypeInfo {
        WEAPONFIRINGEVENT_TYPE_INFO
    }
}


pub const WEAPONFIRINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponSuppressionData {
    pub max_multiplier: f32,
    pub min_multiplier: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

pub const WEAPONSUPPRESSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSuppressionData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponSuppressionData, max_multiplier),
            },
            FieldInfoData {
                name: "MinMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponSuppressionData, min_multiplier),
            },
            FieldInfoData {
                name: "MinDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponSuppressionData, min_distance),
            },
            FieldInfoData {
                name: "MaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponSuppressionData, max_distance),
            },
        ],
    }),
    array_type: Some(WEAPONSUPPRESSIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponSuppressionData {
    fn type_info() -> &'static TypeInfo {
        WEAPONSUPPRESSIONDATA_TYPE_INFO
    }
}


pub const WEAPONSUPPRESSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSuppressionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSuppressionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RumbleFiringData {
    pub low_rumble: f32,
    pub high_rumble: f32,
    pub rumble_duration: f32,
}

pub const RUMBLEFIRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleFiringData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LowRumble",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RumbleFiringData, low_rumble),
            },
            FieldInfoData {
                name: "HighRumble",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RumbleFiringData, high_rumble),
            },
            FieldInfoData {
                name: "RumbleDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RumbleFiringData, rumble_duration),
            },
        ],
    }),
    array_type: Some(RUMBLEFIRINGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RumbleFiringData {
    fn type_info() -> &'static TypeInfo {
        RUMBLEFIRINGDATA_TYPE_INFO
    }
}


pub const RUMBLEFIRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleFiringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("RumbleFiringData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FiringFunctionData {
    pub weapon_dispersion: WeaponDispersion,
    pub fire_effects1p: Vec<FireEffectData>,
    pub fire_effects3p: Vec<FireEffectData>,
    pub sound: super::audio::SoundAsset,
    pub shot: ShotConfigData,
    pub fire_logic: FireLogicData,
    pub ammo: AmmoConfigData,
    pub over_heat: OverHeatData,
    pub use_primary_ammo: bool,
    pub unlimited_ammo_for_a_i: bool,
    pub self_heal_time_when_deployed: f32,
}

pub const FIRINGFUNCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringFunctionData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEDATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WeaponDispersion",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONDISPERSION_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, weapon_dispersion),
            },
            FieldInfoData {
                name: "FireEffects1p",
                flags: MemberInfoFlags::new(144),
                field_type: FIREEFFECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, fire_effects1p),
            },
            FieldInfoData {
                name: "FireEffects3p",
                flags: MemberInfoFlags::new(144),
                field_type: FIREEFFECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, fire_effects3p),
            },
            FieldInfoData {
                name: "Sound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, sound),
            },
            FieldInfoData {
                name: "Shot",
                flags: MemberInfoFlags::new(0),
                field_type: SHOTCONFIGDATA_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, shot),
            },
            FieldInfoData {
                name: "FireLogic",
                flags: MemberInfoFlags::new(0),
                field_type: FIRELOGICDATA_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, fire_logic),
            },
            FieldInfoData {
                name: "Ammo",
                flags: MemberInfoFlags::new(0),
                field_type: AMMOCONFIGDATA_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, ammo),
            },
            FieldInfoData {
                name: "OverHeat",
                flags: MemberInfoFlags::new(0),
                field_type: OVERHEATDATA_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, over_heat),
            },
            FieldInfoData {
                name: "UsePrimaryAmmo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, use_primary_ammo),
            },
            FieldInfoData {
                name: "UnlimitedAmmoForAI",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, unlimited_ammo_for_a_i),
            },
            FieldInfoData {
                name: "SelfHealTimeWhenDeployed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FiringFunctionData, self_heal_time_when_deployed),
            },
        ],
    }),
    array_type: Some(FIRINGFUNCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FiringFunctionData {
    fn type_info() -> &'static TypeInfo {
        FIRINGFUNCTIONDATA_TYPE_INFO
    }
}


pub const FIRINGFUNCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringFunctionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FiringFunctionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WeaponDispersion {
    pub stand_dispersion: FiringDispersionData,
    pub crouch_dispersion: FiringDispersionData,
    pub prone_dispersion: FiringDispersionData,
    pub jump_dispersion_angle: f32,
    pub prone_transition_dispersion_angle: f32,
    pub move_dispersion_angle: f32,
    pub move_zoomed_dispersion_angle: f32,
    pub decrease_per_second: f32,
}

pub const WEAPONDISPERSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersion",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StandDispersion",
                flags: MemberInfoFlags::new(0),
                field_type: FIRINGDISPERSIONDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, stand_dispersion),
            },
            FieldInfoData {
                name: "CrouchDispersion",
                flags: MemberInfoFlags::new(0),
                field_type: FIRINGDISPERSIONDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, crouch_dispersion),
            },
            FieldInfoData {
                name: "ProneDispersion",
                flags: MemberInfoFlags::new(0),
                field_type: FIRINGDISPERSIONDATA_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, prone_dispersion),
            },
            FieldInfoData {
                name: "JumpDispersionAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, jump_dispersion_angle),
            },
            FieldInfoData {
                name: "ProneTransitionDispersionAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, prone_transition_dispersion_angle),
            },
            FieldInfoData {
                name: "MoveDispersionAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, move_dispersion_angle),
            },
            FieldInfoData {
                name: "MoveZoomedDispersionAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, move_zoomed_dispersion_angle),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(WeaponDispersion, decrease_per_second),
            },
        ],
    }),
    array_type: Some(WEAPONDISPERSION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WeaponDispersion {
    fn type_info() -> &'static TypeInfo {
        WEAPONDISPERSION_TYPE_INFO
    }
}


pub const WEAPONDISPERSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersion-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponDispersion-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct OverHeatData {
    pub heat_per_bullet: f32,
    pub heat_drop_per_second: f32,
    pub heat_increase_per_second: f32,
    pub over_heat_drop_delay: f32,
    pub over_heat_penalty_time: f32,
    pub over_heat_threshold: f32,
    pub over_heat_effect: FireEffectData,
}

pub const OVERHEATDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverHeatData",
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "HeatPerBullet",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OverHeatData, heat_per_bullet),
            },
            FieldInfoData {
                name: "HeatDropPerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OverHeatData, heat_drop_per_second),
            },
            FieldInfoData {
                name: "HeatIncreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OverHeatData, heat_increase_per_second),
            },
            FieldInfoData {
                name: "OverHeatDropDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OverHeatData, over_heat_drop_delay),
            },
            FieldInfoData {
                name: "OverHeatPenaltyTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OverHeatData, over_heat_penalty_time),
            },
            FieldInfoData {
                name: "OverHeatThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(OverHeatData, over_heat_threshold),
            },
            FieldInfoData {
                name: "OverHeatEffect",
                flags: MemberInfoFlags::new(0),
                field_type: FIREEFFECTDATA_TYPE_INFO,
                rust_offset: offset_of!(OverHeatData, over_heat_effect),
            },
        ],
    }),
    array_type: Some(OVERHEATDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OverHeatData {
    fn type_info() -> &'static TypeInfo {
        OVERHEATDATA_TYPE_INFO
    }
}


pub const OVERHEATDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverHeatData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("OverHeatData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FireEffectData {
    pub effect: super::effect_base::EffectBlueprint,
    pub offset: super::core::Vec3,
    pub rotation: super::core::Vec3,
    pub zoom_offset: super::core::Vec3,
    pub use_zoom_offset: bool,
    pub zoom_rotation: super::core::Vec3,
    pub use_zoom_rotation: bool,
    pub disable_during_zoom: bool,
    pub update_transform: bool,
    pub stop_looping_effects: bool,
}

pub const FIREEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireEffectData",
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Effect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, effect),
            },
            FieldInfoData {
                name: "Offset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, offset),
            },
            FieldInfoData {
                name: "Rotation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, rotation),
            },
            FieldInfoData {
                name: "ZoomOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, zoom_offset),
            },
            FieldInfoData {
                name: "UseZoomOffset",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, use_zoom_offset),
            },
            FieldInfoData {
                name: "ZoomRotation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, zoom_rotation),
            },
            FieldInfoData {
                name: "UseZoomRotation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, use_zoom_rotation),
            },
            FieldInfoData {
                name: "DisableDuringZoom",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, disable_during_zoom),
            },
            FieldInfoData {
                name: "UpdateTransform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, update_transform),
            },
            FieldInfoData {
                name: "StopLoopingEffects",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireEffectData, stop_looping_effects),
            },
        ],
    }),
    array_type: Some(FIREEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FireEffectData {
    fn type_info() -> &'static TypeInfo {
        FIREEFFECTDATA_TYPE_INFO
    }
}


pub const FIREEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireEffectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FireEffectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FiringDispersionData {
    pub min_angle: f32,
    pub max_angle: f32,
    pub increase_per_shot: f32,
    pub decrease_per_second: f32,
}

pub const FIRINGDISPERSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringDispersionData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MinAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FiringDispersionData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FiringDispersionData, max_angle),
            },
            FieldInfoData {
                name: "IncreasePerShot",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FiringDispersionData, increase_per_shot),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FiringDispersionData, decrease_per_second),
            },
        ],
    }),
    array_type: Some(FIRINGDISPERSIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FiringDispersionData {
    fn type_info() -> &'static TypeInfo {
        FIRINGDISPERSIONDATA_TYPE_INFO
    }
}


pub const FIRINGDISPERSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringDispersionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FiringDispersionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FireLogicData {
    pub hold_and_release: HoldAndReleaseData,
    pub bolt_action: BoltActionData,
    pub recoil: RecoilData,
    pub fire_input_action: i32,
    pub reload_input_action: i32,
    pub cycle_fire_mode_input_action: i32,
    pub charge_input_action: i32,
    pub trigger_pull_weight: f32,
    pub rate_of_fire: f32,
    pub rate_of_fire_for_burst: f32,
    pub bursts_per_minute: f32,
    pub corrected_automatic_fire_replication: bool,
    pub client_fire_rate_multiplier: f32,
    pub reload_delay: f32,
    pub hold_off_reload_until_fire_release: bool,
    pub hold_off_reload_until_zoom_release: bool,
    pub force_reload_action_on_fire_trigger: bool,
    pub reload_time: f32,
    pub reload_timer_array: Vec<f32>,
    pub reload_time_bullets_left: f32,
    pub reload_threshold: f32,
    pub pre_fire_delay: f32,
    pub pending_fire_window: f32,
    pub pending_fire_ignore_fire_rate_window: f32,
    pub reset_pre_fire_wait_on_release: bool,
    pub use_charge_up_logic: bool,
    pub charge_up_increase: f32,
    pub charge_up_decrease: f32,
    pub automatic_delay: f32,
    pub reload_logic: ReloadLogic,
    pub reload_type: ReloadType,
    pub fire_logic_type: FireLogicType,
    pub fire_logic_type_array: Vec<FireLogicType>,
    pub always_auto_reload: bool,
    pub auto_reload_ignore_fire_trigger_pressed: bool,
}

pub const FIRELOGICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicData",
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "HoldAndRelease",
                flags: MemberInfoFlags::new(0),
                field_type: HOLDANDRELEASEDATA_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, hold_and_release),
            },
            FieldInfoData {
                name: "BoltAction",
                flags: MemberInfoFlags::new(0),
                field_type: BOLTACTIONDATA_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, bolt_action),
            },
            FieldInfoData {
                name: "Recoil",
                flags: MemberInfoFlags::new(0),
                field_type: RECOILDATA_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, recoil),
            },
            FieldInfoData {
                name: "FireInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, fire_input_action),
            },
            FieldInfoData {
                name: "ReloadInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_input_action),
            },
            FieldInfoData {
                name: "CycleFireModeInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, cycle_fire_mode_input_action),
            },
            FieldInfoData {
                name: "ChargeInputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, charge_input_action),
            },
            FieldInfoData {
                name: "TriggerPullWeight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, trigger_pull_weight),
            },
            FieldInfoData {
                name: "RateOfFire",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, rate_of_fire),
            },
            FieldInfoData {
                name: "RateOfFireForBurst",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, rate_of_fire_for_burst),
            },
            FieldInfoData {
                name: "BurstsPerMinute",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, bursts_per_minute),
            },
            FieldInfoData {
                name: "CorrectedAutomaticFireReplication",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, corrected_automatic_fire_replication),
            },
            FieldInfoData {
                name: "ClientFireRateMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, client_fire_rate_multiplier),
            },
            FieldInfoData {
                name: "ReloadDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_delay),
            },
            FieldInfoData {
                name: "HoldOffReloadUntilFireRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, hold_off_reload_until_fire_release),
            },
            FieldInfoData {
                name: "HoldOffReloadUntilZoomRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, hold_off_reload_until_zoom_release),
            },
            FieldInfoData {
                name: "ForceReloadActionOnFireTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, force_reload_action_on_fire_trigger),
            },
            FieldInfoData {
                name: "ReloadTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_time),
            },
            FieldInfoData {
                name: "ReloadTimerArray",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_timer_array),
            },
            FieldInfoData {
                name: "ReloadTimeBulletsLeft",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_time_bullets_left),
            },
            FieldInfoData {
                name: "ReloadThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_threshold),
            },
            FieldInfoData {
                name: "PreFireDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, pre_fire_delay),
            },
            FieldInfoData {
                name: "PendingFireWindow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, pending_fire_window),
            },
            FieldInfoData {
                name: "PendingFireIgnoreFireRateWindow",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, pending_fire_ignore_fire_rate_window),
            },
            FieldInfoData {
                name: "ResetPreFireWaitOnRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reset_pre_fire_wait_on_release),
            },
            FieldInfoData {
                name: "UseChargeUpLogic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, use_charge_up_logic),
            },
            FieldInfoData {
                name: "ChargeUpIncrease",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, charge_up_increase),
            },
            FieldInfoData {
                name: "ChargeUpDecrease",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, charge_up_decrease),
            },
            FieldInfoData {
                name: "AutomaticDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, automatic_delay),
            },
            FieldInfoData {
                name: "ReloadLogic",
                flags: MemberInfoFlags::new(0),
                field_type: RELOADLOGIC_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_logic),
            },
            FieldInfoData {
                name: "ReloadType",
                flags: MemberInfoFlags::new(0),
                field_type: RELOADTYPE_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, reload_type),
            },
            FieldInfoData {
                name: "FireLogicType",
                flags: MemberInfoFlags::new(0),
                field_type: FIRELOGICTYPE_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, fire_logic_type),
            },
            FieldInfoData {
                name: "FireLogicTypeArray",
                flags: MemberInfoFlags::new(144),
                field_type: FIRELOGICTYPE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, fire_logic_type_array),
            },
            FieldInfoData {
                name: "AlwaysAutoReload",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, always_auto_reload),
            },
            FieldInfoData {
                name: "AutoReloadIgnoreFireTriggerPressed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FireLogicData, auto_reload_ignore_fire_trigger_pressed),
            },
        ],
    }),
    array_type: Some(FIRELOGICDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FireLogicData {
    fn type_info() -> &'static TypeInfo {
        FIRELOGICDATA_TYPE_INFO
    }
}


pub const FIRELOGICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FireLogicData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RecoilData {
    pub max_recoil_angle_x: f32,
    pub min_recoil_angle_x: f32,
    pub max_recoil_angle_y: f32,
    pub min_recoil_angle_y: f32,
    pub max_recoil_angle_z: f32,
    pub min_recoil_angle_z: f32,
    pub max_recoil_fov: f32,
    pub min_recoil_fov: f32,
    pub recoil_follows_dispersion: bool,
}

pub const RECOILDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecoilData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MaxRecoilAngleX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, max_recoil_angle_x),
            },
            FieldInfoData {
                name: "MinRecoilAngleX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, min_recoil_angle_x),
            },
            FieldInfoData {
                name: "MaxRecoilAngleY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, max_recoil_angle_y),
            },
            FieldInfoData {
                name: "MinRecoilAngleY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, min_recoil_angle_y),
            },
            FieldInfoData {
                name: "MaxRecoilAngleZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, max_recoil_angle_z),
            },
            FieldInfoData {
                name: "MinRecoilAngleZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, min_recoil_angle_z),
            },
            FieldInfoData {
                name: "MaxRecoilFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, max_recoil_fov),
            },
            FieldInfoData {
                name: "MinRecoilFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, min_recoil_fov),
            },
            FieldInfoData {
                name: "RecoilFollowsDispersion",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RecoilData, recoil_follows_dispersion),
            },
        ],
    }),
    array_type: Some(RECOILDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RecoilData {
    fn type_info() -> &'static TypeInfo {
        RECOILDATA_TYPE_INFO
    }
}


pub const RECOILDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecoilData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("RecoilData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ReloadLogic {
    #[default]
    rlWeaponSwitchCancelsUnfinishedReload = 0,
    rlReloadUnaffectedByWeaponSwitch = 1,
}

pub const RELOADLOGIC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadLogic",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(RELOADLOGIC_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ReloadLogic {
    fn type_info() -> &'static TypeInfo {
        RELOADLOGIC_TYPE_INFO
    }
}


pub const RELOADLOGIC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadLogic-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ReloadLogic-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ReloadType {
    #[default]
    rtSingleBullet = 0,
    rtMagazine = 1,
    rtMagazineWithPossibleShorterReload = 2,
}

pub const RELOADTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadType",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(RELOADTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ReloadType {
    fn type_info() -> &'static TypeInfo {
        RELOADTYPE_TYPE_INFO
    }
}


pub const RELOADTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ReloadType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FireLogicType {
    #[default]
    fltSingleFire = 0,
    fltSingleFireWithBoltAction = 1,
    fltAutomaticFire = 2,
    fltBurstFire = 3,
    fltHoldAndRelease = 4,
    fltDetonatedFiring = 5,
    fltCount = 6,
}

pub const FIRELOGICTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicType",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(FIRELOGICTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FireLogicType {
    fn type_info() -> &'static TypeInfo {
        FIRELOGICTYPE_TYPE_INFO
    }
}


pub const FIRELOGICTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FireLogicType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoltActionData {
    pub bolt_action_delay: f32,
    pub bolt_action_time: f32,
    pub hold_bolt_action_until_fire_release: bool,
    pub hold_bolt_action_until_zoom_release: bool,
    pub force_bolt_action_on_fire_trigger: bool,
    pub un_zoom_on_bolt_action: bool,
    pub return_to_zoom_after_bolt_action: bool,
}

pub const BOLTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoltActionData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BoltActionDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoltActionData, bolt_action_delay),
            },
            FieldInfoData {
                name: "BoltActionTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoltActionData, bolt_action_time),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilFireRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoltActionData, hold_bolt_action_until_fire_release),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilZoomRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoltActionData, hold_bolt_action_until_zoom_release),
            },
            FieldInfoData {
                name: "ForceBoltActionOnFireTrigger",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoltActionData, force_bolt_action_on_fire_trigger),
            },
            FieldInfoData {
                name: "UnZoomOnBoltAction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoltActionData, un_zoom_on_bolt_action),
            },
            FieldInfoData {
                name: "ReturnToZoomAfterBoltAction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoltActionData, return_to_zoom_after_bolt_action),
            },
        ],
    }),
    array_type: Some(BOLTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoltActionData {
    fn type_info() -> &'static TypeInfo {
        BOLTACTIONDATA_TYPE_INFO
    }
}


pub const BOLTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoltActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("BoltActionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HoldAndReleaseData {
    pub max_hold_time: f32,
    pub min_power_modifier: f32,
    pub max_power_modifier: f32,
    pub power_increase_per_second: f32,
    pub delay: f32,
    pub force_fire_when_killed_holding: bool,
    pub killed_holding_power_modifier: f32,
    pub precise_replicated_power_modifier: bool,
    pub secondary_action_cancel_hold_and_release: bool,
    pub hold_and_release_when_zooming: bool,
    pub hold_until_fire_release: bool,
}

pub const HOLDANDRELEASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HoldAndReleaseData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MaxHoldTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, max_hold_time),
            },
            FieldInfoData {
                name: "MinPowerModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, min_power_modifier),
            },
            FieldInfoData {
                name: "MaxPowerModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, max_power_modifier),
            },
            FieldInfoData {
                name: "PowerIncreasePerSecond",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, power_increase_per_second),
            },
            FieldInfoData {
                name: "Delay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, delay),
            },
            FieldInfoData {
                name: "ForceFireWhenKilledHolding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, force_fire_when_killed_holding),
            },
            FieldInfoData {
                name: "KilledHoldingPowerModifier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, killed_holding_power_modifier),
            },
            FieldInfoData {
                name: "PreciseReplicatedPowerModifier",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, precise_replicated_power_modifier),
            },
            FieldInfoData {
                name: "SecondaryActionCancelHoldAndRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, secondary_action_cancel_hold_and_release),
            },
            FieldInfoData {
                name: "HoldAndReleaseWhenZooming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, hold_and_release_when_zooming),
            },
            FieldInfoData {
                name: "HoldUntilFireRelease",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(HoldAndReleaseData, hold_until_fire_release),
            },
        ],
    }),
    array_type: Some(HOLDANDRELEASEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HoldAndReleaseData {
    fn type_info() -> &'static TypeInfo {
        HOLDANDRELEASEDATA_TYPE_INFO
    }
}


pub const HOLDANDRELEASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HoldAndReleaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("HoldAndReleaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ShotConfigData {
    pub initial_position: super::core::Vec3,
    pub initial_direction: super::core::Vec3,
    pub initial_direction_scale_by_pitch: Vec<InitialDirectionScaleByPitchData>,
    pub initial_speed: super::core::Vec3,
    pub initial_speed_scale_by_pitch: Vec<InitialSpeedScaleByPitchData>,
    pub inherit_weapon_speed_amount: f32,
    pub muzzle_explosion: super::game_shared::ExplosionEntityData,
    pub projectile_data: ProjectileEntityData,
    pub secondary_projectile_data: ProjectileEntityData,
    pub projectile: ProjectileBlueprint,
    pub secondary_projectile: ProjectileBlueprint,
    pub spawn_delay: f32,
    pub number_of_bullets_per_shell: u32,
    pub number_of_bullets_per_shot: u32,
    pub number_of_bullets_per_burst: u32,
    pub relative_target_aiming: bool,
    pub force_spawn_to_camera: bool,
    pub spawn_visual_at_weapon_bone: bool,
    pub weapon_bone: super::entity::GameplayBones,
    pub active_force_spawn_to_camera: bool,
}

pub const SHOTCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShotConfigData",
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InitialPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, initial_position),
            },
            FieldInfoData {
                name: "InitialDirection",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, initial_direction),
            },
            FieldInfoData {
                name: "InitialDirectionScaleByPitch",
                flags: MemberInfoFlags::new(144),
                field_type: INITIALDIRECTIONSCALEBYPITCHDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, initial_direction_scale_by_pitch),
            },
            FieldInfoData {
                name: "InitialSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, initial_speed),
            },
            FieldInfoData {
                name: "InitialSpeedScaleByPitch",
                flags: MemberInfoFlags::new(144),
                field_type: INITIALSPEEDSCALEBYPITCHDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, initial_speed_scale_by_pitch),
            },
            FieldInfoData {
                name: "InheritWeaponSpeedAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, inherit_weapon_speed_amount),
            },
            FieldInfoData {
                name: "MuzzleExplosion",
                flags: MemberInfoFlags::new(0),
                field_type: EXPLOSIONENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, muzzle_explosion),
            },
            FieldInfoData {
                name: "ProjectileData",
                flags: MemberInfoFlags::new(0),
                field_type: PROJECTILEENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, projectile_data),
            },
            FieldInfoData {
                name: "SecondaryProjectileData",
                flags: MemberInfoFlags::new(0),
                field_type: PROJECTILEENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, secondary_projectile_data),
            },
            FieldInfoData {
                name: "Projectile",
                flags: MemberInfoFlags::new(0),
                field_type: PROJECTILEBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, projectile),
            },
            FieldInfoData {
                name: "SecondaryProjectile",
                flags: MemberInfoFlags::new(0),
                field_type: PROJECTILEBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, secondary_projectile),
            },
            FieldInfoData {
                name: "SpawnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, spawn_delay),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShell",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, number_of_bullets_per_shell),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShot",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, number_of_bullets_per_shot),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerBurst",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, number_of_bullets_per_burst),
            },
            FieldInfoData {
                name: "RelativeTargetAiming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, relative_target_aiming),
            },
            FieldInfoData {
                name: "ForceSpawnToCamera",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, force_spawn_to_camera),
            },
            FieldInfoData {
                name: "SpawnVisualAtWeaponBone",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, spawn_visual_at_weapon_bone),
            },
            FieldInfoData {
                name: "WeaponBone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, weapon_bone),
            },
            FieldInfoData {
                name: "ActiveForceSpawnToCamera",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShotConfigData, active_force_spawn_to_camera),
            },
        ],
    }),
    array_type: Some(SHOTCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShotConfigData {
    fn type_info() -> &'static TypeInfo {
        SHOTCONFIGDATA_TYPE_INFO
    }
}


pub const SHOTCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShotConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ShotConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InitialSpeedScaleByPitchData {
    pub pitch: f32,
    pub initial_speed_scale: super::core::Vec3,
}

pub const INITIALSPEEDSCALEBYPITCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialSpeedScaleByPitchData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InitialSpeedScaleByPitchData, pitch),
            },
            FieldInfoData {
                name: "InitialSpeedScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(InitialSpeedScaleByPitchData, initial_speed_scale),
            },
        ],
    }),
    array_type: Some(INITIALSPEEDSCALEBYPITCHDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for InitialSpeedScaleByPitchData {
    fn type_info() -> &'static TypeInfo {
        INITIALSPEEDSCALEBYPITCHDATA_TYPE_INFO
    }
}


pub const INITIALSPEEDSCALEBYPITCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialSpeedScaleByPitchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("InitialSpeedScaleByPitchData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct InitialDirectionScaleByPitchData {
    pub pitch: f32,
    pub initial_direction_scale: super::core::Vec3,
}

pub const INITIALDIRECTIONSCALEBYPITCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialDirectionScaleByPitchData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Pitch",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(InitialDirectionScaleByPitchData, pitch),
            },
            FieldInfoData {
                name: "InitialDirectionScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(InitialDirectionScaleByPitchData, initial_direction_scale),
            },
        ],
    }),
    array_type: Some(INITIALDIRECTIONSCALEBYPITCHDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for InitialDirectionScaleByPitchData {
    fn type_info() -> &'static TypeInfo {
        INITIALDIRECTIONSCALEBYPITCHDATA_TYPE_INFO
    }
}


pub const INITIALDIRECTIONSCALEBYPITCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialDirectionScaleByPitchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("InitialDirectionScaleByPitchData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HealingSphereData {
    pub radius: f32,
    pub health_inc_speed: f32,
}

pub const HEALINGSPHEREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealingSphereData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealingSphereData, radius),
            },
            FieldInfoData {
                name: "HealthIncSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(HealingSphereData, health_inc_speed),
            },
        ],
    }),
    array_type: Some(HEALINGSPHEREDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HealingSphereData {
    fn type_info() -> &'static TypeInfo {
        HEALINGSPHEREDATA_TYPE_INFO
    }
}


pub const HEALINGSPHEREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealingSphereData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("HealingSphereData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MissileEntityData {
    pub engine_effect: super::effect_base::EffectBlueprint,
    pub dud_explosion: super::game_shared::ExplosionEntityData,
    pub fly_by_sound: super::audio::SoundAsset,
    pub engine_strength: f32,
    pub max_speed: f32,
    pub engine_time_to_ignition: f32,
    pub engine_time_to_live: f32,
    pub time_to_activate_guiding_system: f32,
    pub time_to_arm: f32,
    pub max_turn_angle: f32,
    pub min_turn_angle: f32,
    pub turn_angle_multiplier: f32,
    pub turn_y_first: bool,
    pub drag: f32,
    pub gravity: f32,
    pub apply_gravity_when_guided: bool,
    pub fly_by_sound_radius: f32,
    pub fly_by_sound_speed: f32,
    pub impact_impulse: f32,
    pub damage: f32,
    pub default_team: super::gameplay_sim::TeamId,
    pub warn_target: WarnTarget,
    pub warn_on_pointing_missile: bool,
    pub locking_controller: LockingControllerData,
    pub lockable_info: MissileLockableInfoData,
    pub unguided_data: MissileUnguidedData,
    pub near_target_detonation: NearTargetDetonationData,
    pub enable_banking: bool,
    pub max_bank_angle: f32,
    pub banking_speed: f32,
    pub icon: String,
    pub target_icon: String,
    pub target_icon_enemy: String,
    pub min_ghost_frequency: f32,
    pub start_effects_on_spawn: bool,
    pub is_bullet_collision: bool,
    pub extrapolate_acceleration: bool,
    pub calculate_position_based_on_velocity: bool,
}

pub const MISSILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GHOSTEDPROJECTILEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EngineEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, engine_effect),
            },
            FieldInfoData {
                name: "DudExplosion",
                flags: MemberInfoFlags::new(0),
                field_type: EXPLOSIONENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, dud_explosion),
            },
            FieldInfoData {
                name: "FlyBySound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, fly_by_sound),
            },
            FieldInfoData {
                name: "EngineStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, engine_strength),
            },
            FieldInfoData {
                name: "MaxSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, max_speed),
            },
            FieldInfoData {
                name: "EngineTimeToIgnition",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, engine_time_to_ignition),
            },
            FieldInfoData {
                name: "EngineTimeToLive",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, engine_time_to_live),
            },
            FieldInfoData {
                name: "TimeToActivateGuidingSystem",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, time_to_activate_guiding_system),
            },
            FieldInfoData {
                name: "TimeToArm",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, time_to_arm),
            },
            FieldInfoData {
                name: "MaxTurnAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, max_turn_angle),
            },
            FieldInfoData {
                name: "MinTurnAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, min_turn_angle),
            },
            FieldInfoData {
                name: "TurnAngleMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, turn_angle_multiplier),
            },
            FieldInfoData {
                name: "TurnYFirst",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, turn_y_first),
            },
            FieldInfoData {
                name: "Drag",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, drag),
            },
            FieldInfoData {
                name: "Gravity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, gravity),
            },
            FieldInfoData {
                name: "ApplyGravityWhenGuided",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, apply_gravity_when_guided),
            },
            FieldInfoData {
                name: "FlyBySoundRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, fly_by_sound_radius),
            },
            FieldInfoData {
                name: "FlyBySoundSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, fly_by_sound_speed),
            },
            FieldInfoData {
                name: "ImpactImpulse",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, impact_impulse),
            },
            FieldInfoData {
                name: "Damage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, damage),
            },
            FieldInfoData {
                name: "DefaultTeam",
                flags: MemberInfoFlags::new(0),
                field_type: TEAMID_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, default_team),
            },
            FieldInfoData {
                name: "WarnTarget",
                flags: MemberInfoFlags::new(0),
                field_type: WARNTARGET_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, warn_target),
            },
            FieldInfoData {
                name: "WarnOnPointingMissile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, warn_on_pointing_missile),
            },
            FieldInfoData {
                name: "LockingController",
                flags: MemberInfoFlags::new(0),
                field_type: LOCKINGCONTROLLERDATA_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, locking_controller),
            },
            FieldInfoData {
                name: "LockableInfo",
                flags: MemberInfoFlags::new(0),
                field_type: MISSILELOCKABLEINFODATA_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, lockable_info),
            },
            FieldInfoData {
                name: "UnguidedData",
                flags: MemberInfoFlags::new(0),
                field_type: MISSILEUNGUIDEDDATA_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, unguided_data),
            },
            FieldInfoData {
                name: "NearTargetDetonation",
                flags: MemberInfoFlags::new(0),
                field_type: NEARTARGETDETONATIONDATA_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, near_target_detonation),
            },
            FieldInfoData {
                name: "EnableBanking",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, enable_banking),
            },
            FieldInfoData {
                name: "MaxBankAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, max_bank_angle),
            },
            FieldInfoData {
                name: "BankingSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, banking_speed),
            },
            FieldInfoData {
                name: "Icon",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, icon),
            },
            FieldInfoData {
                name: "TargetIcon",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, target_icon),
            },
            FieldInfoData {
                name: "TargetIconEnemy",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, target_icon_enemy),
            },
            FieldInfoData {
                name: "MinGhostFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, min_ghost_frequency),
            },
            FieldInfoData {
                name: "StartEffectsOnSpawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, start_effects_on_spawn),
            },
            FieldInfoData {
                name: "IsBulletCollision",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, is_bullet_collision),
            },
            FieldInfoData {
                name: "ExtrapolateAcceleration",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, extrapolate_acceleration),
            },
            FieldInfoData {
                name: "CalculatePositionBasedOnVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileEntityData, calculate_position_based_on_velocity),
            },
        ],
    }),
    array_type: Some(MISSILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MissileEntityData {
    fn type_info() -> &'static TypeInfo {
        MISSILEENTITYDATA_TYPE_INFO
    }
}


pub const MISSILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissileEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct NearTargetDetonationData {
    pub detonate_near_target: bool,
    pub detonation_radius: f32,
    pub max_detonation_delay: f32,
    pub min_detonation_delay: f32,
}

pub const NEARTARGETDETONATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NearTargetDetonationData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DetonateNearTarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(NearTargetDetonationData, detonate_near_target),
            },
            FieldInfoData {
                name: "DetonationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NearTargetDetonationData, detonation_radius),
            },
            FieldInfoData {
                name: "MaxDetonationDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NearTargetDetonationData, max_detonation_delay),
            },
            FieldInfoData {
                name: "MinDetonationDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(NearTargetDetonationData, min_detonation_delay),
            },
        ],
    }),
    array_type: Some(NEARTARGETDETONATIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for NearTargetDetonationData {
    fn type_info() -> &'static TypeInfo {
        NEARTARGETDETONATIONDATA_TYPE_INFO
    }
}


pub const NEARTARGETDETONATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NearTargetDetonationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("NearTargetDetonationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MissileUnguidedData {
    pub use_static_position: bool,
    pub static_position: super::core::Vec2,
    pub use_target_position: bool,
    pub target_position_offset: super::core::Vec2,
}

pub const MISSILEUNGUIDEDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileUnguidedData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "UseStaticPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileUnguidedData, use_static_position),
            },
            FieldInfoData {
                name: "StaticPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(MissileUnguidedData, static_position),
            },
            FieldInfoData {
                name: "UseTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MissileUnguidedData, use_target_position),
            },
            FieldInfoData {
                name: "TargetPositionOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(MissileUnguidedData, target_position_offset),
            },
        ],
    }),
    array_type: Some(MISSILEUNGUIDEDDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MissileUnguidedData {
    fn type_info() -> &'static TypeInfo {
        MISSILEUNGUIDEDDATA_TYPE_INFO
    }
}


pub const MISSILEUNGUIDEDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileUnguidedData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissileUnguidedData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MissileLockableInfoData {
    pub heat_signature: f32,
    pub radar_signature: f32,
}

pub const MISSILELOCKABLEINFODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileLockableInfoData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "HeatSignature",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileLockableInfoData, heat_signature),
            },
            FieldInfoData {
                name: "RadarSignature",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MissileLockableInfoData, radar_signature),
            },
        ],
    }),
    array_type: Some(MISSILELOCKABLEINFODATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MissileLockableInfoData {
    fn type_info() -> &'static TypeInfo {
        MISSILELOCKABLEINFODATA_TYPE_INFO
    }
}


pub const MISSILELOCKABLEINFODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileLockableInfoData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissileLockableInfoData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LockingControllerData {
    pub zoom_level_lock: Vec<ZoomLevelLockData>,
    pub lock_time: f32,
    pub un_lock_time: f32,
    pub release_time: f32,
    pub release_on_new_target_time: f32,
    pub sample_rate: f32,
    pub hold_still_threshold: f32,
    pub check_visibility_length: f32,
    pub ray_length: f32,
    pub lock_on_visible_targets_only: bool,
    pub require_ammo_to_lock: bool,
    pub position_only: bool,
    pub use_unlock_time_with_position_only: bool,
    pub lock_on_world_space_pos: bool,
    pub acceptance_angle: f32,
    pub angle_constant: f32,
    pub distance_constant: f32,
    pub normalize_constant_weights: bool,
    pub check_target_lock_strength_on_raycast: bool,
    pub sensitivity: f32,
    pub minimum_lock_time: f32,
    pub lock_on_vehicles: bool,
    pub lock_on_empty_vehicles: bool,
    pub lock_on_characters: bool,
    pub lock_on_characters_in_open_entries: bool,
    pub ignore_heigth_lock_distance: bool,
    pub lock_in_combat_area_only: bool,
    pub locking_weapon_data: LockingAndHomingData,
}

pub const LOCKINGCONTROLLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingControllerData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ZoomLevelLock",
                flags: MemberInfoFlags::new(144),
                field_type: ZOOMLEVELLOCKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, zoom_level_lock),
            },
            FieldInfoData {
                name: "LockTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_time),
            },
            FieldInfoData {
                name: "UnLockTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, un_lock_time),
            },
            FieldInfoData {
                name: "ReleaseTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, release_time),
            },
            FieldInfoData {
                name: "ReleaseOnNewTargetTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, release_on_new_target_time),
            },
            FieldInfoData {
                name: "SampleRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, sample_rate),
            },
            FieldInfoData {
                name: "HoldStillThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, hold_still_threshold),
            },
            FieldInfoData {
                name: "CheckVisibilityLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, check_visibility_length),
            },
            FieldInfoData {
                name: "RayLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, ray_length),
            },
            FieldInfoData {
                name: "LockOnVisibleTargetsOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_on_visible_targets_only),
            },
            FieldInfoData {
                name: "RequireAmmoToLock",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, require_ammo_to_lock),
            },
            FieldInfoData {
                name: "PositionOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, position_only),
            },
            FieldInfoData {
                name: "UseUnlockTimeWithPositionOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, use_unlock_time_with_position_only),
            },
            FieldInfoData {
                name: "LockOnWorldSpacePos",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_on_world_space_pos),
            },
            FieldInfoData {
                name: "AcceptanceAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, acceptance_angle),
            },
            FieldInfoData {
                name: "AngleConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, angle_constant),
            },
            FieldInfoData {
                name: "DistanceConstant",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, distance_constant),
            },
            FieldInfoData {
                name: "NormalizeConstantWeights",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, normalize_constant_weights),
            },
            FieldInfoData {
                name: "CheckTargetLockStrengthOnRaycast",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, check_target_lock_strength_on_raycast),
            },
            FieldInfoData {
                name: "Sensitivity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, sensitivity),
            },
            FieldInfoData {
                name: "MinimumLockTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, minimum_lock_time),
            },
            FieldInfoData {
                name: "LockOnVehicles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_on_vehicles),
            },
            FieldInfoData {
                name: "LockOnEmptyVehicles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_on_empty_vehicles),
            },
            FieldInfoData {
                name: "LockOnCharacters",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_on_characters),
            },
            FieldInfoData {
                name: "LockOnCharactersInOpenEntries",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_on_characters_in_open_entries),
            },
            FieldInfoData {
                name: "IgnoreHeigthLockDistance",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, ignore_heigth_lock_distance),
            },
            FieldInfoData {
                name: "LockInCombatAreaOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, lock_in_combat_area_only),
            },
            FieldInfoData {
                name: "LockingWeaponData",
                flags: MemberInfoFlags::new(0),
                field_type: LOCKINGANDHOMINGDATA_TYPE_INFO,
                rust_offset: offset_of!(LockingControllerData, locking_weapon_data),
            },
        ],
    }),
    array_type: Some(LOCKINGCONTROLLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LockingControllerData {
    fn type_info() -> &'static TypeInfo {
        LOCKINGCONTROLLERDATA_TYPE_INFO
    }
}


pub const LOCKINGCONTROLLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingControllerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LockingControllerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LockingAndHomingData {
    pub is_homing: bool,
    pub is_guided: bool,
    pub is_guided_when_zoomed: bool,
    pub is_guided_homing: bool,
    pub fire_only_when_locked_on: bool,
    pub warn_lock: WarnTarget,
}

pub const LOCKINGANDHOMINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingAndHomingData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsHoming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingAndHomingData, is_homing),
            },
            FieldInfoData {
                name: "IsGuided",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingAndHomingData, is_guided),
            },
            FieldInfoData {
                name: "IsGuidedWhenZoomed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingAndHomingData, is_guided_when_zoomed),
            },
            FieldInfoData {
                name: "IsGuidedHoming",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingAndHomingData, is_guided_homing),
            },
            FieldInfoData {
                name: "FireOnlyWhenLockedOn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LockingAndHomingData, fire_only_when_locked_on),
            },
            FieldInfoData {
                name: "WarnLock",
                flags: MemberInfoFlags::new(0),
                field_type: WARNTARGET_TYPE_INFO,
                rust_offset: offset_of!(LockingAndHomingData, warn_lock),
            },
        ],
    }),
    array_type: Some(LOCKINGANDHOMINGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LockingAndHomingData {
    fn type_info() -> &'static TypeInfo {
        LOCKINGANDHOMINGDATA_TYPE_INFO
    }
}


pub const LOCKINGANDHOMINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingAndHomingData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LockingAndHomingData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ZoomLevelLockData {
    pub outline_tagged_distance: f32,
    pub lock_type: super::gameplay_sim::LockType,
}

pub const ZOOMLEVELLOCKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoomLevelLockData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "OutlineTaggedDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ZoomLevelLockData, outline_tagged_distance),
            },
            FieldInfoData {
                name: "LockType",
                flags: MemberInfoFlags::new(0),
                field_type: LOCKTYPE_TYPE_INFO,
                rust_offset: offset_of!(ZoomLevelLockData, lock_type),
            },
        ],
    }),
    array_type: Some(ZOOMLEVELLOCKDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ZoomLevelLockData {
    fn type_info() -> &'static TypeInfo {
        ZOOMLEVELLOCKDATA_TYPE_INFO
    }
}


pub const ZOOMLEVELLOCKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoomLevelLockData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ZoomLevelLockData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WarnTarget {
    #[default]
    wtWarnSoldierAndVehicle = 0,
    wtWarnSoldier = 1,
    wtWarnVehicle = 2,
    wtWarnNone = 3,
}

pub const WARNTARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WarnTarget",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WARNTARGET_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WarnTarget {
    fn type_info() -> &'static TypeInfo {
        WARNTARGET_TYPE_INFO
    }
}


pub const WARNTARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WarnTarget-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WarnTarget-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BulletEntityData {
    pub fly_by_sound: super::audio::SoundAsset,
    pub dud_explosion: super::game_shared::ExplosionEntityData,
    pub gravity: f32,
    pub impact_impulse: f32,
    pub detonation_time_variation: f32,
    pub vehicle_detonation_radius: f32,
    pub vehicle_detonation_activation_delay: f32,
    pub fly_by_sound_radius: f32,
    pub fly_by_sound_speed: f32,
    pub stamina: f32,
    pub distribute_damage_over_time: f32,
    pub start_damage: f32,
    pub end_damage: f32,
    pub damage_falloff_start_distance: f32,
    pub damage_falloff_end_distance: f32,
    pub time_to_arm_explosion: f32,
    pub has_vehicle_detonation: bool,
    pub instant_hit: bool,
    pub first_frame_travel_distance: f32,
    pub stop_trail_effect_on_unspawn: bool,
    pub stop_trail_effect_on_unspawn_frame_delay: i32,
}

pub const BULLETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BulletEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHPROJECTILEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FlyBySound",
                flags: MemberInfoFlags::new(0),
                field_type: SOUNDASSET_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, fly_by_sound),
            },
            FieldInfoData {
                name: "DudExplosion",
                flags: MemberInfoFlags::new(0),
                field_type: EXPLOSIONENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, dud_explosion),
            },
            FieldInfoData {
                name: "Gravity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, gravity),
            },
            FieldInfoData {
                name: "ImpactImpulse",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, impact_impulse),
            },
            FieldInfoData {
                name: "DetonationTimeVariation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, detonation_time_variation),
            },
            FieldInfoData {
                name: "VehicleDetonationRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, vehicle_detonation_radius),
            },
            FieldInfoData {
                name: "VehicleDetonationActivationDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, vehicle_detonation_activation_delay),
            },
            FieldInfoData {
                name: "FlyBySoundRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, fly_by_sound_radius),
            },
            FieldInfoData {
                name: "FlyBySoundSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, fly_by_sound_speed),
            },
            FieldInfoData {
                name: "Stamina",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, stamina),
            },
            FieldInfoData {
                name: "DistributeDamageOverTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, distribute_damage_over_time),
            },
            FieldInfoData {
                name: "StartDamage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, start_damage),
            },
            FieldInfoData {
                name: "EndDamage",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, end_damage),
            },
            FieldInfoData {
                name: "DamageFalloffStartDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, damage_falloff_start_distance),
            },
            FieldInfoData {
                name: "DamageFalloffEndDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, damage_falloff_end_distance),
            },
            FieldInfoData {
                name: "TimeToArmExplosion",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, time_to_arm_explosion),
            },
            FieldInfoData {
                name: "HasVehicleDetonation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, has_vehicle_detonation),
            },
            FieldInfoData {
                name: "InstantHit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, instant_hit),
            },
            FieldInfoData {
                name: "FirstFrameTravelDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, first_frame_travel_distance),
            },
            FieldInfoData {
                name: "StopTrailEffectOnUnspawn",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, stop_trail_effect_on_unspawn),
            },
            FieldInfoData {
                name: "StopTrailEffectOnUnspawnFrameDelay",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BulletEntityData, stop_trail_effect_on_unspawn_frame_delay),
            },
        ],
    }),
    array_type: Some(BULLETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BulletEntityData {
    fn type_info() -> &'static TypeInfo {
        BULLETENTITYDATA_TYPE_INFO
    }
}


pub const BULLETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BulletEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("BulletEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GhostedProjectileEntityData {
    pub proxy_convergence_delay: f32,
    pub proxy_convergence_duration: f32,
    pub proxy_convergence_instant_on_attach: bool,
    pub force_proxy_convergence: bool,
    pub convergence_using_initial_speed: bool,
}

pub const GHOSTEDPROJECTILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GhostedProjectileEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHPROJECTILEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProxyConvergenceDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GhostedProjectileEntityData, proxy_convergence_delay),
            },
            FieldInfoData {
                name: "ProxyConvergenceDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GhostedProjectileEntityData, proxy_convergence_duration),
            },
            FieldInfoData {
                name: "ProxyConvergenceInstantOnAttach",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GhostedProjectileEntityData, proxy_convergence_instant_on_attach),
            },
            FieldInfoData {
                name: "ForceProxyConvergence",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GhostedProjectileEntityData, force_proxy_convergence),
            },
            FieldInfoData {
                name: "ConvergenceUsingInitialSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GhostedProjectileEntityData, convergence_using_initial_speed),
            },
        ],
    }),
    array_type: Some(GHOSTEDPROJECTILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GhostedProjectileEntityData {
    fn type_info() -> &'static TypeInfo {
        GHOSTEDPROJECTILEENTITYDATA_TYPE_INFO
    }
}


pub const GHOSTEDPROJECTILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GhostedProjectileEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("GhostedProjectileEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshProjectileEntityData {
    pub mesh: super::render_base::MeshBaseAsset,
    pub trail_effect: super::effect_base::EffectBlueprint,
    pub is_attachable: bool,
    pub instant_attachable_test_distance: f32,
    pub instant_attachable_visual_convergence_delay: f32,
    pub instant_attachable_visual_convergence_duration: f32,
    pub instant_attachable_test_under_reticule: bool,
    pub max_attachable_inclination: f32,
    pub extra_damping: bool,
    pub initial_angular_velocity: super::core::Vec3,
    pub unspawn_after_detonation_delay: f32,
}

pub const MESHPROJECTILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProjectileEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROJECTILEENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, mesh),
            },
            FieldInfoData {
                name: "TrailEffect",
                flags: MemberInfoFlags::new(0),
                field_type: EFFECTBLUEPRINT_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, trail_effect),
            },
            FieldInfoData {
                name: "IsAttachable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, is_attachable),
            },
            FieldInfoData {
                name: "InstantAttachableTestDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_test_distance),
            },
            FieldInfoData {
                name: "InstantAttachableVisualConvergenceDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_visual_convergence_delay),
            },
            FieldInfoData {
                name: "InstantAttachableVisualConvergenceDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_visual_convergence_duration),
            },
            FieldInfoData {
                name: "InstantAttachableTestUnderReticule",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_test_under_reticule),
            },
            FieldInfoData {
                name: "MaxAttachableInclination",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, max_attachable_inclination),
            },
            FieldInfoData {
                name: "ExtraDamping",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, extra_damping),
            },
            FieldInfoData {
                name: "InitialAngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, initial_angular_velocity),
            },
            FieldInfoData {
                name: "UnspawnAfterDetonationDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshProjectileEntityData, unspawn_after_detonation_delay),
            },
        ],
    }),
    array_type: Some(MESHPROJECTILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshProjectileEntityData {
    fn type_info() -> &'static TypeInfo {
        MESHPROJECTILEENTITYDATA_TYPE_INFO
    }
}


pub const MESHPROJECTILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProjectileEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MeshProjectileEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MissilePhysicsComponentData {
}

pub const MISSILEPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissilePhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MISSILEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MissilePhysicsComponentData {
    fn type_info() -> &'static TypeInfo {
        MISSILEPHYSICSCOMPONENTDATA_TYPE_INFO
    }
}


pub const MISSILEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissilePhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissilePhysicsComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProjectileEntityData {
    pub initial_speed: f32,
    pub time_to_live: f32,
    pub max_count: u32,
    pub init_mesh_hide_time: f32,
    pub visual_converge_distance: f32,
    pub visual_convergence_delay: f32,
    pub visual_convergence_duration: f32,
    pub proxy_visual_convergence_delay: f32,
    pub proxy_visual_convergence_duration: f32,
    pub detonate_on_timeout: bool,
    pub server_projectile_disabled: bool,
    pub explosion: super::game_shared::ExplosionEntityData,
    pub suppression_data: WeaponSuppressionData,
    pub ammunition_type: String,
    pub material_pair: super::entity::MaterialDecl,
    pub hit_reaction_weapon_type: AntHitReactionWeaponType,
    pub hide_on_detonation: bool,
    pub voice_over_info: super::audio::EntityVoiceOverInfo,
}

pub const PROJECTILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileEntityData",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEPHYSICSENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InitialSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, initial_speed),
            },
            FieldInfoData {
                name: "TimeToLive",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, time_to_live),
            },
            FieldInfoData {
                name: "MaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, max_count),
            },
            FieldInfoData {
                name: "InitMeshHideTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, init_mesh_hide_time),
            },
            FieldInfoData {
                name: "VisualConvergeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, visual_converge_distance),
            },
            FieldInfoData {
                name: "VisualConvergenceDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, visual_convergence_delay),
            },
            FieldInfoData {
                name: "VisualConvergenceDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, visual_convergence_duration),
            },
            FieldInfoData {
                name: "ProxyVisualConvergenceDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, proxy_visual_convergence_delay),
            },
            FieldInfoData {
                name: "ProxyVisualConvergenceDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, proxy_visual_convergence_duration),
            },
            FieldInfoData {
                name: "DetonateOnTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, detonate_on_timeout),
            },
            FieldInfoData {
                name: "ServerProjectileDisabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, server_projectile_disabled),
            },
            FieldInfoData {
                name: "Explosion",
                flags: MemberInfoFlags::new(0),
                field_type: EXPLOSIONENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, explosion),
            },
            FieldInfoData {
                name: "SuppressionData",
                flags: MemberInfoFlags::new(0),
                field_type: WEAPONSUPPRESSIONDATA_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, suppression_data),
            },
            FieldInfoData {
                name: "AmmunitionType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, ammunition_type),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: MATERIALDECL_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, material_pair),
            },
            FieldInfoData {
                name: "HitReactionWeaponType",
                flags: MemberInfoFlags::new(0),
                field_type: ANTHITREACTIONWEAPONTYPE_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, hit_reaction_weapon_type),
            },
            FieldInfoData {
                name: "HideOnDetonation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, hide_on_detonation),
            },
            FieldInfoData {
                name: "VoiceOverInfo",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYVOICEOVERINFO_TYPE_INFO,
                rust_offset: offset_of!(ProjectileEntityData, voice_over_info),
            },
        ],
    }),
    array_type: Some(PROJECTILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProjectileEntityData {
    fn type_info() -> &'static TypeInfo {
        PROJECTILEENTITYDATA_TYPE_INFO
    }
}


pub const PROJECTILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ProjectileEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProjectileBlueprint {
    pub time_delta_type: super::entity::TimeDeltaType,
}

pub const PROJECTILEBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBJECTBLUEPRINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(ProjectileBlueprint, time_delta_type),
            },
        ],
    }),
    array_type: Some(PROJECTILEBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProjectileBlueprint {
    fn type_info() -> &'static TypeInfo {
        PROJECTILEBLUEPRINT_TYPE_INFO
    }
}


pub const PROJECTILEBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ProjectileBlueprint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AntHitReactionWeaponType {
    #[default]
    AntHitReactionWeaponType_Pistol = 0,
    AntHitReactionWeaponType_SMG = 1,
    AntHitReactionWeaponType_AssaultRifle = 2,
    AntHitReactionWeaponType_LMG = 3,
    AntHitReactionWeaponType_Shotgun = 4,
    AntHitReactionWeaponType_SniperRifle = 5,
    AntHitReactionWeaponType_Explosion = 6,
    AntHitReactionWeaponType_Flashbang = 7,
    AntHitReactionWeaponType_Melee = 8,
}

pub const ANTHITREACTIONWEAPONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntHitReactionWeaponType",
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(ANTHITREACTIONWEAPONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntHitReactionWeaponType {
    fn type_info() -> &'static TypeInfo {
        ANTHITREACTIONWEAPONTYPE_TYPE_INFO
    }
}


pub const ANTHITREACTIONWEAPONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntHitReactionWeaponType-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("AntHitReactionWeaponType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AmmoConfigData {
    pub magazine_capacity: i32,
    pub number_of_magazines: i32,
    pub spawn_without_ammo: bool,
    pub trace_frequency: u32,
    pub ammo_pickup_min_amount: u32,
    pub ammo_pickup_max_amount: u32,
    pub auto_replenish_magazine: bool,
    pub auto_replenish_delay: f32,
    pub ammo_bag_pickup_delay_multiplier: f32,
    pub ammo_bag_pickup_amount: i32,
}

pub const AMMOCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AmmoConfigData",
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MagazineCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, magazine_capacity),
            },
            FieldInfoData {
                name: "NumberOfMagazines",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, number_of_magazines),
            },
            FieldInfoData {
                name: "SpawnWithoutAmmo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, spawn_without_ammo),
            },
            FieldInfoData {
                name: "TraceFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, trace_frequency),
            },
            FieldInfoData {
                name: "AmmoPickupMinAmount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, ammo_pickup_min_amount),
            },
            FieldInfoData {
                name: "AmmoPickupMaxAmount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, ammo_pickup_max_amount),
            },
            FieldInfoData {
                name: "AutoReplenishMagazine",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, auto_replenish_magazine),
            },
            FieldInfoData {
                name: "AutoReplenishDelay",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, auto_replenish_delay),
            },
            FieldInfoData {
                name: "AmmoBagPickupDelayMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, ammo_bag_pickup_delay_multiplier),
            },
            FieldInfoData {
                name: "AmmoBagPickupAmount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AmmoConfigData, ammo_bag_pickup_amount),
            },
        ],
    }),
    array_type: Some(AMMOCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AmmoConfigData {
    fn type_info() -> &'static TypeInfo {
        AMMOCONFIGDATA_TYPE_INFO
    }
}


pub const AMMOCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AmmoConfigData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("AmmoConfigData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WeaponUnlockAsset {
}

pub const WEAPONUNLOCKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlockAsset",
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UNLOCKASSETBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WEAPONUNLOCKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponUnlockAsset {
    fn type_info() -> &'static TypeInfo {
        WEAPONUNLOCKASSET_TYPE_INFO
    }
}


pub const WEAPONUNLOCKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlockAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponUnlockAsset-Array"),
    array_type: None,
    alignment: 8,
};


