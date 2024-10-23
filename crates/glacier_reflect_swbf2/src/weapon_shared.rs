use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponMiscModifierSettings {
    pub enable_breath_control: bool,
    pub can_be_in_supported_shooting: bool,
    pub un_zoom_on_bolt_action: bool,
    pub hold_bolt_action_until_zoom_release: bool,
    pub is_silenced: bool,
}

pub trait WeaponMiscModifierSettingsTrait: TypeObject {
    fn enable_breath_control(&self) -> &bool;
    fn enable_breath_control_mut(&mut self) -> &mut bool;
    fn can_be_in_supported_shooting(&self) -> &bool;
    fn can_be_in_supported_shooting_mut(&mut self) -> &mut bool;
    fn un_zoom_on_bolt_action(&self) -> &bool;
    fn un_zoom_on_bolt_action_mut(&mut self) -> &mut bool;
    fn hold_bolt_action_until_zoom_release(&self) -> &bool;
    fn hold_bolt_action_until_zoom_release_mut(&mut self) -> &mut bool;
    fn is_silenced(&self) -> &bool;
    fn is_silenced_mut(&mut self) -> &mut bool;
}

impl WeaponMiscModifierSettingsTrait for WeaponMiscModifierSettings {
    fn enable_breath_control(&self) -> &bool {
        &self.enable_breath_control
    }
    fn enable_breath_control_mut(&mut self) -> &mut bool {
        &mut self.enable_breath_control
    }
    fn can_be_in_supported_shooting(&self) -> &bool {
        &self.can_be_in_supported_shooting
    }
    fn can_be_in_supported_shooting_mut(&mut self) -> &mut bool {
        &mut self.can_be_in_supported_shooting
    }
    fn un_zoom_on_bolt_action(&self) -> &bool {
        &self.un_zoom_on_bolt_action
    }
    fn un_zoom_on_bolt_action_mut(&mut self) -> &mut bool {
        &mut self.un_zoom_on_bolt_action
    }
    fn hold_bolt_action_until_zoom_release(&self) -> &bool {
        &self.hold_bolt_action_until_zoom_release
    }
    fn hold_bolt_action_until_zoom_release_mut(&mut self) -> &mut bool {
        &mut self.hold_bolt_action_until_zoom_release
    }
    fn is_silenced(&self) -> &bool {
        &self.is_silenced
    }
    fn is_silenced_mut(&mut self) -> &mut bool {
        &mut self.is_silenced
    }
}

pub static WEAPONMISCMODIFIERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifierSettings",
    name_hash: 1023399809,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponMiscModifierSettings as Default>::default())),
            create_boxed: || Box::new(<WeaponMiscModifierSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnableBreathControl",
                name_hash: 613505579,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifierSettings, enable_breath_control),
            },
            FieldInfoData {
                name: "CanBeInSupportedShooting",
                name_hash: 2187452328,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifierSettings, can_be_in_supported_shooting),
            },
            FieldInfoData {
                name: "UnZoomOnBoltAction",
                name_hash: 1438634691,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifierSettings, un_zoom_on_bolt_action),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilZoomRelease",
                name_hash: 3133940949,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifierSettings, hold_bolt_action_until_zoom_release),
            },
            FieldInfoData {
                name: "IsSilenced",
                name_hash: 1644964768,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifierSettings, is_silenced),
            },
        ],
    }),
    array_type: Some(WEAPONMISCMODIFIERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponMiscModifierSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONMISCMODIFIERSETTINGS_TYPE_INFO
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


pub static WEAPONMISCMODIFIERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifierSettings-Array",
    name_hash: 2261578037,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponMiscModifierSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponSwayData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait WeaponSwayDataTrait: super::core::DataContainerTrait {
}

impl WeaponSwayDataTrait for WeaponSwayData {
}

impl super::core::DataContainerTrait for WeaponSwayData {
}

pub static WEAPONSWAYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSwayData",
    name_hash: 2851754891,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponSwayData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponSwayData as Default>::default())),
            create_boxed: || Box::new(<WeaponSwayData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONSWAYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponSwayData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONSWAYDATA_TYPE_INFO
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


pub static WEAPONSWAYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSwayData-Array",
    name_hash: 1500267071,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSwayData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum Constants {
    #[default]
    WeaponComponent_BarrelIndexBits = 4,
}

pub static CONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Constants",
    name_hash: 2658787400,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(CONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Constants {
    fn type_info(&self) -> &'static TypeInfo {
        CONSTANTS_TYPE_INFO
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


pub static CONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Constants-Array",
    name_hash: 3100840700,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("Constants"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WeaponUnlocks {
    #[default]
    WeaponUnlocks_MaxAmount = 8,
}

pub static WEAPONUNLOCKS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlocks",
    name_hash: 2456600100,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONUNLOCKS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponUnlocks {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONUNLOCKS_TYPE_INFO
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


pub static WEAPONUNLOCKS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlocks-Array",
    name_hash: 83947920,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponUnlocks"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static WEAPONSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSlot",
    name_hash: 933194083,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONSLOT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponSlot {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONSLOT_TYPE_INFO
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


pub static WEAPONSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSlot-Array",
    name_hash: 57351511,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSlot"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static GEARSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearSlot",
    name_hash: 930851280,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(GEARSLOT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GearSlot {
    fn type_info(&self) -> &'static TypeInfo {
        GEARSLOT_TYPE_INFO
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


pub static GEARSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GearSlot-Array",
    name_hash: 3531171940,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("GearSlot"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CharacterSimpleWeaponComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub realm: super::core::Realm,
    pub damage_giver_name: String,
    pub weapon_offset: super::core::LinearTransform,
    pub fire_target: super::core::LinearTransform,
    pub weapon_firing: Option<LockedTypeObject /* WeaponFiringData */>,
}

pub trait CharacterSimpleWeaponComponentDataTrait: super::entity::GameComponentDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn damage_giver_name(&self) -> &String;
    fn damage_giver_name_mut(&mut self) -> &mut String;
    fn weapon_offset(&self) -> &super::core::LinearTransform;
    fn weapon_offset_mut(&mut self) -> &mut super::core::LinearTransform;
    fn fire_target(&self) -> &super::core::LinearTransform;
    fn fire_target_mut(&mut self) -> &mut super::core::LinearTransform;
    fn weapon_firing(&self) -> &Option<LockedTypeObject /* WeaponFiringData */>;
    fn weapon_firing_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponFiringData */>;
}

impl CharacterSimpleWeaponComponentDataTrait for CharacterSimpleWeaponComponentData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn damage_giver_name(&self) -> &String {
        &self.damage_giver_name
    }
    fn damage_giver_name_mut(&mut self) -> &mut String {
        &mut self.damage_giver_name
    }
    fn weapon_offset(&self) -> &super::core::LinearTransform {
        &self.weapon_offset
    }
    fn weapon_offset_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.weapon_offset
    }
    fn fire_target(&self) -> &super::core::LinearTransform {
        &self.fire_target
    }
    fn fire_target_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.fire_target
    }
    fn weapon_firing(&self) -> &Option<LockedTypeObject /* WeaponFiringData */> {
        &self.weapon_firing
    }
    fn weapon_firing_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponFiringData */> {
        &mut self.weapon_firing
    }
}

impl super::entity::GameComponentDataTrait for CharacterSimpleWeaponComponentData {
}

impl super::entity::ComponentDataTrait for CharacterSimpleWeaponComponentData {
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

impl super::entity::GameObjectDataTrait for CharacterSimpleWeaponComponentData {
}

impl super::core::DataBusPeerTrait for CharacterSimpleWeaponComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for CharacterSimpleWeaponComponentData {
}

impl super::core::DataContainerTrait for CharacterSimpleWeaponComponentData {
}

pub static CHARACTERSIMPLEWEAPONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSimpleWeaponComponentData",
    name_hash: 2548539279,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(CharacterSimpleWeaponComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CharacterSimpleWeaponComponentData as Default>::default())),
            create_boxed: || Box::new(<CharacterSimpleWeaponComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, realm),
            },
            FieldInfoData {
                name: "DamageGiverName",
                name_hash: 38100006,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, damage_giver_name),
            },
            FieldInfoData {
                name: "WeaponOffset",
                name_hash: 2173072170,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, weapon_offset),
            },
            FieldInfoData {
                name: "FireTarget",
                name_hash: 3787489900,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, fire_target),
            },
            FieldInfoData {
                name: "WeaponFiring",
                name_hash: 1823157050,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponFiringData",
                rust_offset: offset_of!(CharacterSimpleWeaponComponentData, weapon_firing),
            },
        ],
    }),
    array_type: Some(CHARACTERSIMPLEWEAPONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CharacterSimpleWeaponComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        CHARACTERSIMPLEWEAPONCOMPONENTDATA_TYPE_INFO
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


pub static CHARACTERSIMPLEWEAPONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CharacterSimpleWeaponComponentData-Array",
    name_hash: 2959227963,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("CharacterSimpleWeaponComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponInputRouterComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub max_fire_rate: f32,
    pub rotation_count: u32,
}

pub trait WeaponInputRouterComponentDataTrait: super::entity::GameComponentDataTrait {
    fn max_fire_rate(&self) -> &f32;
    fn max_fire_rate_mut(&mut self) -> &mut f32;
    fn rotation_count(&self) -> &u32;
    fn rotation_count_mut(&mut self) -> &mut u32;
}

impl WeaponInputRouterComponentDataTrait for WeaponInputRouterComponentData {
    fn max_fire_rate(&self) -> &f32 {
        &self.max_fire_rate
    }
    fn max_fire_rate_mut(&mut self) -> &mut f32 {
        &mut self.max_fire_rate
    }
    fn rotation_count(&self) -> &u32 {
        &self.rotation_count
    }
    fn rotation_count_mut(&mut self) -> &mut u32 {
        &mut self.rotation_count
    }
}

impl super::entity::GameComponentDataTrait for WeaponInputRouterComponentData {
}

impl super::entity::ComponentDataTrait for WeaponInputRouterComponentData {
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

impl super::entity::GameObjectDataTrait for WeaponInputRouterComponentData {
}

impl super::core::DataBusPeerTrait for WeaponInputRouterComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WeaponInputRouterComponentData {
}

impl super::core::DataContainerTrait for WeaponInputRouterComponentData {
}

pub static WEAPONINPUTROUTERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponInputRouterComponentData",
    name_hash: 4238774053,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(WeaponInputRouterComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponInputRouterComponentData as Default>::default())),
            create_boxed: || Box::new(<WeaponInputRouterComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxFireRate",
                name_hash: 369835659,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponInputRouterComponentData, max_fire_rate),
            },
            FieldInfoData {
                name: "RotationCount",
                name_hash: 821183634,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WeaponInputRouterComponentData, rotation_count),
            },
        ],
    }),
    array_type: Some(WEAPONINPUTROUTERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponInputRouterComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONINPUTROUTERCOMPONENTDATA_TYPE_INFO
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


pub static WEAPONINPUTROUTERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponInputRouterComponentData-Array",
    name_hash: 2523580305,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponInputRouterComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponComponentData {
    pub _glacier_base: super::entity::BoneComponentData,
    pub projectile_spawn_offset: super::core::Vec3,
    pub sequential_firing: bool,
    pub weapon_firing: Option<LockedTypeObject /* WeaponFiringData */>,
    pub damage_giver_name: String,
    pub a_i_data: Option<LockedTypeObject /* super::gameplay_sim::GameAIWeaponData */>,
    pub custom_weapon_type: Option<LockedTypeObject /* WeaponData */>,
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

pub trait WeaponComponentDataTrait: super::entity::BoneComponentDataTrait {
    fn projectile_spawn_offset(&self) -> &super::core::Vec3;
    fn projectile_spawn_offset_mut(&mut self) -> &mut super::core::Vec3;
    fn sequential_firing(&self) -> &bool;
    fn sequential_firing_mut(&mut self) -> &mut bool;
    fn weapon_firing(&self) -> &Option<LockedTypeObject /* WeaponFiringData */>;
    fn weapon_firing_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponFiringData */>;
    fn damage_giver_name(&self) -> &String;
    fn damage_giver_name_mut(&mut self) -> &mut String;
    fn a_i_data(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::GameAIWeaponData */>;
    fn a_i_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::GameAIWeaponData */>;
    fn custom_weapon_type(&self) -> &Option<LockedTypeObject /* WeaponData */>;
    fn custom_weapon_type_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponData */>;
    fn impulse_strength(&self) -> &f32;
    fn impulse_strength_mut(&mut self) -> &mut f32;
    fn classification(&self) -> &super::game_shared::WeaponClassification;
    fn classification_mut(&mut self) -> &mut super::game_shared::WeaponClassification;
    fn reload_time_multiplier(&self) -> &f32;
    fn reload_time_multiplier_mut(&mut self) -> &mut f32;
    fn damage_multiplier(&self) -> &f32;
    fn damage_multiplier_mut(&mut self) -> &mut f32;
    fn explosion_damage_multiplier(&self) -> &f32;
    fn explosion_damage_multiplier_mut(&mut self) -> &mut f32;
    fn overheat_drop_per_second_multiplier(&self) -> &f32;
    fn overheat_drop_per_second_multiplier_mut(&mut self) -> &mut f32;
    fn rate_of_fire_multiplier(&self) -> &f32;
    fn rate_of_fire_multiplier_mut(&mut self) -> &mut f32;
    fn lock_time_multiplier(&self) -> &f32;
    fn lock_time_multiplier_mut(&mut self) -> &mut f32;
    fn locking_acceptance_angle_multiplier(&self) -> &f32;
    fn locking_acceptance_angle_multiplier_mut(&mut self) -> &mut f32;
    fn target_position_override(&self) -> &super::core::Vec3;
    fn target_position_override_mut(&mut self) -> &mut super::core::Vec3;
    fn weapon_item_hash(&self) -> &u32;
    fn weapon_item_hash_mut(&mut self) -> &mut u32;
}

impl WeaponComponentDataTrait for WeaponComponentData {
    fn projectile_spawn_offset(&self) -> &super::core::Vec3 {
        &self.projectile_spawn_offset
    }
    fn projectile_spawn_offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.projectile_spawn_offset
    }
    fn sequential_firing(&self) -> &bool {
        &self.sequential_firing
    }
    fn sequential_firing_mut(&mut self) -> &mut bool {
        &mut self.sequential_firing
    }
    fn weapon_firing(&self) -> &Option<LockedTypeObject /* WeaponFiringData */> {
        &self.weapon_firing
    }
    fn weapon_firing_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponFiringData */> {
        &mut self.weapon_firing
    }
    fn damage_giver_name(&self) -> &String {
        &self.damage_giver_name
    }
    fn damage_giver_name_mut(&mut self) -> &mut String {
        &mut self.damage_giver_name
    }
    fn a_i_data(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::GameAIWeaponData */> {
        &self.a_i_data
    }
    fn a_i_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::GameAIWeaponData */> {
        &mut self.a_i_data
    }
    fn custom_weapon_type(&self) -> &Option<LockedTypeObject /* WeaponData */> {
        &self.custom_weapon_type
    }
    fn custom_weapon_type_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponData */> {
        &mut self.custom_weapon_type
    }
    fn impulse_strength(&self) -> &f32 {
        &self.impulse_strength
    }
    fn impulse_strength_mut(&mut self) -> &mut f32 {
        &mut self.impulse_strength
    }
    fn classification(&self) -> &super::game_shared::WeaponClassification {
        &self.classification
    }
    fn classification_mut(&mut self) -> &mut super::game_shared::WeaponClassification {
        &mut self.classification
    }
    fn reload_time_multiplier(&self) -> &f32 {
        &self.reload_time_multiplier
    }
    fn reload_time_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.reload_time_multiplier
    }
    fn damage_multiplier(&self) -> &f32 {
        &self.damage_multiplier
    }
    fn damage_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.damage_multiplier
    }
    fn explosion_damage_multiplier(&self) -> &f32 {
        &self.explosion_damage_multiplier
    }
    fn explosion_damage_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.explosion_damage_multiplier
    }
    fn overheat_drop_per_second_multiplier(&self) -> &f32 {
        &self.overheat_drop_per_second_multiplier
    }
    fn overheat_drop_per_second_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.overheat_drop_per_second_multiplier
    }
    fn rate_of_fire_multiplier(&self) -> &f32 {
        &self.rate_of_fire_multiplier
    }
    fn rate_of_fire_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire_multiplier
    }
    fn lock_time_multiplier(&self) -> &f32 {
        &self.lock_time_multiplier
    }
    fn lock_time_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.lock_time_multiplier
    }
    fn locking_acceptance_angle_multiplier(&self) -> &f32 {
        &self.locking_acceptance_angle_multiplier
    }
    fn locking_acceptance_angle_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.locking_acceptance_angle_multiplier
    }
    fn target_position_override(&self) -> &super::core::Vec3 {
        &self.target_position_override
    }
    fn target_position_override_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.target_position_override
    }
    fn weapon_item_hash(&self) -> &u32 {
        &self.weapon_item_hash
    }
    fn weapon_item_hash_mut(&mut self) -> &mut u32 {
        &mut self.weapon_item_hash
    }
}

impl super::entity::BoneComponentDataTrait for WeaponComponentData {
}

impl super::entity::GameComponentDataTrait for WeaponComponentData {
}

impl super::entity::ComponentDataTrait for WeaponComponentData {
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

impl super::entity::GameObjectDataTrait for WeaponComponentData {
}

impl super::core::DataBusPeerTrait for WeaponComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for WeaponComponentData {
}

impl super::core::DataContainerTrait for WeaponComponentData {
}

pub static WEAPONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponComponentData",
    name_hash: 2059040248,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::BONECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(WeaponComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponComponentData as Default>::default())),
            create_boxed: || Box::new(<WeaponComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ProjectileSpawnOffset",
                name_hash: 36316966,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WeaponComponentData, projectile_spawn_offset),
            },
            FieldInfoData {
                name: "SequentialFiring",
                name_hash: 2022632657,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponComponentData, sequential_firing),
            },
            FieldInfoData {
                name: "WeaponFiring",
                name_hash: 1823157050,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponFiringData",
                rust_offset: offset_of!(WeaponComponentData, weapon_firing),
            },
            FieldInfoData {
                name: "DamageGiverName",
                name_hash: 38100006,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WeaponComponentData, damage_giver_name),
            },
            FieldInfoData {
                name: "AIData",
                name_hash: 2529950045,
                flags: MemberInfoFlags::new(0),
                field_type: "GameAIWeaponData",
                rust_offset: offset_of!(WeaponComponentData, a_i_data),
            },
            FieldInfoData {
                name: "CustomWeaponType",
                name_hash: 3603058156,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponData",
                rust_offset: offset_of!(WeaponComponentData, custom_weapon_type),
            },
            FieldInfoData {
                name: "ImpulseStrength",
                name_hash: 722393051,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, impulse_strength),
            },
            FieldInfoData {
                name: "Classification",
                name_hash: 1196599827,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponClassification",
                rust_offset: offset_of!(WeaponComponentData, classification),
            },
            FieldInfoData {
                name: "ReloadTimeMultiplier",
                name_hash: 2574353098,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, reload_time_multiplier),
            },
            FieldInfoData {
                name: "DamageMultiplier",
                name_hash: 2936063845,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, damage_multiplier),
            },
            FieldInfoData {
                name: "ExplosionDamageMultiplier",
                name_hash: 1685883856,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, explosion_damage_multiplier),
            },
            FieldInfoData {
                name: "OverheatDropPerSecondMultiplier",
                name_hash: 273863558,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, overheat_drop_per_second_multiplier),
            },
            FieldInfoData {
                name: "RateOfFireMultiplier",
                name_hash: 1714855261,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, rate_of_fire_multiplier),
            },
            FieldInfoData {
                name: "LockTimeMultiplier",
                name_hash: 1309101584,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, lock_time_multiplier),
            },
            FieldInfoData {
                name: "LockingAcceptanceAngleMultiplier",
                name_hash: 4054344813,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponComponentData, locking_acceptance_angle_multiplier),
            },
            FieldInfoData {
                name: "TargetPositionOverride",
                name_hash: 3740413593,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WeaponComponentData, target_position_override),
            },
            FieldInfoData {
                name: "WeaponItemHash",
                name_hash: 3522794400,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WeaponComponentData, weapon_item_hash),
            },
        ],
    }),
    array_type: Some(WEAPONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONCOMPONENTDATA_TYPE_INFO
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


pub static WEAPONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponComponentData-Array",
    name_hash: 3090673612,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponOverheatCombinableModifier {
    pub _glacier_base: WeaponOverheatModifier,
}

pub trait WeaponOverheatCombinableModifierTrait: WeaponOverheatModifierTrait {
}

impl WeaponOverheatCombinableModifierTrait for WeaponOverheatCombinableModifier {
}

impl WeaponOverheatModifierTrait for WeaponOverheatCombinableModifier {
    fn heat_per_bullet(&self) -> &f32 {
        self._glacier_base.heat_per_bullet()
    }
    fn heat_per_bullet_mut(&mut self) -> &mut f32 {
        self._glacier_base.heat_per_bullet_mut()
    }
    fn heat_drop_per_second(&self) -> &f32 {
        self._glacier_base.heat_drop_per_second()
    }
    fn heat_drop_per_second_mut(&mut self) -> &mut f32 {
        self._glacier_base.heat_drop_per_second_mut()
    }
    fn heat_increase_per_second(&self) -> &f32 {
        self._glacier_base.heat_increase_per_second()
    }
    fn heat_increase_per_second_mut(&mut self) -> &mut f32 {
        self._glacier_base.heat_increase_per_second_mut()
    }
    fn overheated_penalty_time(&self) -> &f32 {
        self._glacier_base.overheated_penalty_time()
    }
    fn overheated_penalty_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.overheated_penalty_time_mut()
    }
    fn overheat_threshold(&self) -> &f32 {
        self._glacier_base.overheat_threshold()
    }
    fn overheat_threshold_mut(&mut self) -> &mut f32 {
        self._glacier_base.overheat_threshold_mut()
    }
    fn overheat_drop_delay(&self) -> &f32 {
        self._glacier_base.overheat_drop_delay()
    }
    fn overheat_drop_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.overheat_drop_delay_mut()
    }
    fn heat_per_bullet_multiplier(&self) -> &f32 {
        self._glacier_base.heat_per_bullet_multiplier()
    }
    fn heat_per_bullet_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.heat_per_bullet_multiplier_mut()
    }
    fn heat_drop_per_second_multiplier(&self) -> &f32 {
        self._glacier_base.heat_drop_per_second_multiplier()
    }
    fn heat_drop_per_second_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.heat_drop_per_second_multiplier_mut()
    }
    fn heat_increase_per_second_multiplier(&self) -> &f32 {
        self._glacier_base.heat_increase_per_second_multiplier()
    }
    fn heat_increase_per_second_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.heat_increase_per_second_multiplier_mut()
    }
    fn overheated_penalty_time_multiplier(&self) -> &f32 {
        self._glacier_base.overheated_penalty_time_multiplier()
    }
    fn overheated_penalty_time_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.overheated_penalty_time_multiplier_mut()
    }
    fn overheat_drop_delay_multiplier(&self) -> &f32 {
        self._glacier_base.overheat_drop_delay_multiplier()
    }
    fn overheat_drop_delay_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.overheat_drop_delay_multiplier_mut()
    }
    fn overheated_drop_multiplier(&self) -> &f32 {
        self._glacier_base.overheated_drop_multiplier()
    }
    fn overheated_drop_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.overheated_drop_multiplier_mut()
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponOverheatCombinableModifier {
}

impl WeaponModifierBaseTrait for WeaponOverheatCombinableModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponOverheatCombinableModifier {
}

pub static WEAPONOVERHEATCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatCombinableModifier",
    name_hash: 653221640,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONOVERHEATMODIFIER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponOverheatCombinableModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponOverheatCombinableModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponOverheatCombinableModifier as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONOVERHEATCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponOverheatCombinableModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONOVERHEATCOMBINABLEMODIFIER_TYPE_INFO
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


pub static WEAPONOVERHEATCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatCombinableModifier-Array",
    name_hash: 3789070652,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponOverheatCombinableModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponOverheatModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
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

pub trait WeaponOverheatModifierTrait: WeaponModifierDynamicBaseTrait {
    fn heat_per_bullet(&self) -> &f32;
    fn heat_per_bullet_mut(&mut self) -> &mut f32;
    fn heat_drop_per_second(&self) -> &f32;
    fn heat_drop_per_second_mut(&mut self) -> &mut f32;
    fn heat_increase_per_second(&self) -> &f32;
    fn heat_increase_per_second_mut(&mut self) -> &mut f32;
    fn overheated_penalty_time(&self) -> &f32;
    fn overheated_penalty_time_mut(&mut self) -> &mut f32;
    fn overheat_threshold(&self) -> &f32;
    fn overheat_threshold_mut(&mut self) -> &mut f32;
    fn overheat_drop_delay(&self) -> &f32;
    fn overheat_drop_delay_mut(&mut self) -> &mut f32;
    fn heat_per_bullet_multiplier(&self) -> &f32;
    fn heat_per_bullet_multiplier_mut(&mut self) -> &mut f32;
    fn heat_drop_per_second_multiplier(&self) -> &f32;
    fn heat_drop_per_second_multiplier_mut(&mut self) -> &mut f32;
    fn heat_increase_per_second_multiplier(&self) -> &f32;
    fn heat_increase_per_second_multiplier_mut(&mut self) -> &mut f32;
    fn overheated_penalty_time_multiplier(&self) -> &f32;
    fn overheated_penalty_time_multiplier_mut(&mut self) -> &mut f32;
    fn overheat_drop_delay_multiplier(&self) -> &f32;
    fn overheat_drop_delay_multiplier_mut(&mut self) -> &mut f32;
    fn overheated_drop_multiplier(&self) -> &f32;
    fn overheated_drop_multiplier_mut(&mut self) -> &mut f32;
}

impl WeaponOverheatModifierTrait for WeaponOverheatModifier {
    fn heat_per_bullet(&self) -> &f32 {
        &self.heat_per_bullet
    }
    fn heat_per_bullet_mut(&mut self) -> &mut f32 {
        &mut self.heat_per_bullet
    }
    fn heat_drop_per_second(&self) -> &f32 {
        &self.heat_drop_per_second
    }
    fn heat_drop_per_second_mut(&mut self) -> &mut f32 {
        &mut self.heat_drop_per_second
    }
    fn heat_increase_per_second(&self) -> &f32 {
        &self.heat_increase_per_second
    }
    fn heat_increase_per_second_mut(&mut self) -> &mut f32 {
        &mut self.heat_increase_per_second
    }
    fn overheated_penalty_time(&self) -> &f32 {
        &self.overheated_penalty_time
    }
    fn overheated_penalty_time_mut(&mut self) -> &mut f32 {
        &mut self.overheated_penalty_time
    }
    fn overheat_threshold(&self) -> &f32 {
        &self.overheat_threshold
    }
    fn overheat_threshold_mut(&mut self) -> &mut f32 {
        &mut self.overheat_threshold
    }
    fn overheat_drop_delay(&self) -> &f32 {
        &self.overheat_drop_delay
    }
    fn overheat_drop_delay_mut(&mut self) -> &mut f32 {
        &mut self.overheat_drop_delay
    }
    fn heat_per_bullet_multiplier(&self) -> &f32 {
        &self.heat_per_bullet_multiplier
    }
    fn heat_per_bullet_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.heat_per_bullet_multiplier
    }
    fn heat_drop_per_second_multiplier(&self) -> &f32 {
        &self.heat_drop_per_second_multiplier
    }
    fn heat_drop_per_second_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.heat_drop_per_second_multiplier
    }
    fn heat_increase_per_second_multiplier(&self) -> &f32 {
        &self.heat_increase_per_second_multiplier
    }
    fn heat_increase_per_second_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.heat_increase_per_second_multiplier
    }
    fn overheated_penalty_time_multiplier(&self) -> &f32 {
        &self.overheated_penalty_time_multiplier
    }
    fn overheated_penalty_time_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.overheated_penalty_time_multiplier
    }
    fn overheat_drop_delay_multiplier(&self) -> &f32 {
        &self.overheat_drop_delay_multiplier
    }
    fn overheat_drop_delay_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.overheat_drop_delay_multiplier
    }
    fn overheated_drop_multiplier(&self) -> &f32 {
        &self.overheated_drop_multiplier
    }
    fn overheated_drop_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.overheated_drop_multiplier
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponOverheatModifier {
}

impl WeaponModifierBaseTrait for WeaponOverheatModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponOverheatModifier {
}

pub static WEAPONOVERHEATMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatModifier",
    name_hash: 2664469606,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponOverheatModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponOverheatModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponOverheatModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HeatPerBullet",
                name_hash: 2230088284,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, heat_per_bullet),
            },
            FieldInfoData {
                name: "HeatDropPerSecond",
                name_hash: 2173208259,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, heat_drop_per_second),
            },
            FieldInfoData {
                name: "HeatIncreasePerSecond",
                name_hash: 3418659534,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, heat_increase_per_second),
            },
            FieldInfoData {
                name: "OverheatedPenaltyTime",
                name_hash: 3296677436,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, overheated_penalty_time),
            },
            FieldInfoData {
                name: "OverheatThreshold",
                name_hash: 160337860,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, overheat_threshold),
            },
            FieldInfoData {
                name: "OverheatDropDelay",
                name_hash: 3839495503,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, overheat_drop_delay),
            },
            FieldInfoData {
                name: "HeatPerBulletMultiplier",
                name_hash: 3041604887,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, heat_per_bullet_multiplier),
            },
            FieldInfoData {
                name: "HeatDropPerSecondMultiplier",
                name_hash: 2799802120,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, heat_drop_per_second_multiplier),
            },
            FieldInfoData {
                name: "HeatIncreasePerSecondMultiplier",
                name_hash: 2371165573,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, heat_increase_per_second_multiplier),
            },
            FieldInfoData {
                name: "OverheatedPenaltyTimeMultiplier",
                name_hash: 3575847351,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, overheated_penalty_time_multiplier),
            },
            FieldInfoData {
                name: "OverheatDropDelayMultiplier",
                name_hash: 2155728004,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, overheat_drop_delay_multiplier),
            },
            FieldInfoData {
                name: "OverheatedDropMultiplier",
                name_hash: 3890601008,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponOverheatModifier, overheated_drop_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONOVERHEATMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponOverheatModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONOVERHEATMODIFIER_TYPE_INFO
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


pub static WEAPONOVERHEATMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOverheatModifier-Array",
    name_hash: 2380575058,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponOverheatModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponRecoilCombinableModifier {
    pub _glacier_base: WeaponRecoilModifier,
}

pub trait WeaponRecoilCombinableModifierTrait: WeaponRecoilModifierTrait {
}

impl WeaponRecoilCombinableModifierTrait for WeaponRecoilCombinableModifier {
}

impl WeaponRecoilModifierTrait for WeaponRecoilCombinableModifier {
    fn max_recoil_angle_x(&self) -> &f32 {
        self._glacier_base.max_recoil_angle_x()
    }
    fn max_recoil_angle_x_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_angle_x_mut()
    }
    fn min_recoil_angle_x(&self) -> &f32 {
        self._glacier_base.min_recoil_angle_x()
    }
    fn min_recoil_angle_x_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_angle_x_mut()
    }
    fn max_recoil_angle_y(&self) -> &f32 {
        self._glacier_base.max_recoil_angle_y()
    }
    fn max_recoil_angle_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_angle_y_mut()
    }
    fn min_recoil_angle_y(&self) -> &f32 {
        self._glacier_base.min_recoil_angle_y()
    }
    fn min_recoil_angle_y_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_angle_y_mut()
    }
    fn max_recoil_angle_z(&self) -> &f32 {
        self._glacier_base.max_recoil_angle_z()
    }
    fn max_recoil_angle_z_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_angle_z_mut()
    }
    fn min_recoil_angle_z(&self) -> &f32 {
        self._glacier_base.min_recoil_angle_z()
    }
    fn min_recoil_angle_z_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_angle_z_mut()
    }
    fn max_recoil_fov(&self) -> &f32 {
        self._glacier_base.max_recoil_fov()
    }
    fn max_recoil_fov_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_fov_mut()
    }
    fn min_recoil_fov(&self) -> &f32 {
        self._glacier_base.min_recoil_fov()
    }
    fn min_recoil_fov_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_fov_mut()
    }
    fn max_recoil_angle_x_multiplier(&self) -> &f32 {
        self._glacier_base.max_recoil_angle_x_multiplier()
    }
    fn max_recoil_angle_x_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_angle_x_multiplier_mut()
    }
    fn min_recoil_angle_x_multiplier(&self) -> &f32 {
        self._glacier_base.min_recoil_angle_x_multiplier()
    }
    fn min_recoil_angle_x_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_angle_x_multiplier_mut()
    }
    fn max_recoil_angle_y_multiplier(&self) -> &f32 {
        self._glacier_base.max_recoil_angle_y_multiplier()
    }
    fn max_recoil_angle_y_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_angle_y_multiplier_mut()
    }
    fn min_recoil_angle_y_multiplier(&self) -> &f32 {
        self._glacier_base.min_recoil_angle_y_multiplier()
    }
    fn min_recoil_angle_y_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_angle_y_multiplier_mut()
    }
    fn max_recoil_angle_z_multiplier(&self) -> &f32 {
        self._glacier_base.max_recoil_angle_z_multiplier()
    }
    fn max_recoil_angle_z_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_angle_z_multiplier_mut()
    }
    fn min_recoil_angle_z_multiplier(&self) -> &f32 {
        self._glacier_base.min_recoil_angle_z_multiplier()
    }
    fn min_recoil_angle_z_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_angle_z_multiplier_mut()
    }
    fn max_recoil_fov_multiplier(&self) -> &f32 {
        self._glacier_base.max_recoil_fov_multiplier()
    }
    fn max_recoil_fov_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_recoil_fov_multiplier_mut()
    }
    fn min_recoil_fov_multiplier(&self) -> &f32 {
        self._glacier_base.min_recoil_fov_multiplier()
    }
    fn min_recoil_fov_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.min_recoil_fov_multiplier_mut()
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponRecoilCombinableModifier {
}

impl WeaponModifierBaseTrait for WeaponRecoilCombinableModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponRecoilCombinableModifier {
}

pub static WEAPONRECOILCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilCombinableModifier",
    name_hash: 2143728288,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONRECOILMODIFIER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponRecoilCombinableModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponRecoilCombinableModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponRecoilCombinableModifier as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONRECOILCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponRecoilCombinableModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONRECOILCOMBINABLEMODIFIER_TYPE_INFO
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


pub static WEAPONRECOILCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilCombinableModifier-Array",
    name_hash: 2933954580,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponRecoilCombinableModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponRecoilModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
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

pub trait WeaponRecoilModifierTrait: WeaponModifierDynamicBaseTrait {
    fn max_recoil_angle_x(&self) -> &f32;
    fn max_recoil_angle_x_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_x(&self) -> &f32;
    fn min_recoil_angle_x_mut(&mut self) -> &mut f32;
    fn max_recoil_angle_y(&self) -> &f32;
    fn max_recoil_angle_y_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_y(&self) -> &f32;
    fn min_recoil_angle_y_mut(&mut self) -> &mut f32;
    fn max_recoil_angle_z(&self) -> &f32;
    fn max_recoil_angle_z_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_z(&self) -> &f32;
    fn min_recoil_angle_z_mut(&mut self) -> &mut f32;
    fn max_recoil_fov(&self) -> &f32;
    fn max_recoil_fov_mut(&mut self) -> &mut f32;
    fn min_recoil_fov(&self) -> &f32;
    fn min_recoil_fov_mut(&mut self) -> &mut f32;
    fn max_recoil_angle_x_multiplier(&self) -> &f32;
    fn max_recoil_angle_x_multiplier_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_x_multiplier(&self) -> &f32;
    fn min_recoil_angle_x_multiplier_mut(&mut self) -> &mut f32;
    fn max_recoil_angle_y_multiplier(&self) -> &f32;
    fn max_recoil_angle_y_multiplier_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_y_multiplier(&self) -> &f32;
    fn min_recoil_angle_y_multiplier_mut(&mut self) -> &mut f32;
    fn max_recoil_angle_z_multiplier(&self) -> &f32;
    fn max_recoil_angle_z_multiplier_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_z_multiplier(&self) -> &f32;
    fn min_recoil_angle_z_multiplier_mut(&mut self) -> &mut f32;
    fn max_recoil_fov_multiplier(&self) -> &f32;
    fn max_recoil_fov_multiplier_mut(&mut self) -> &mut f32;
    fn min_recoil_fov_multiplier(&self) -> &f32;
    fn min_recoil_fov_multiplier_mut(&mut self) -> &mut f32;
}

impl WeaponRecoilModifierTrait for WeaponRecoilModifier {
    fn max_recoil_angle_x(&self) -> &f32 {
        &self.max_recoil_angle_x
    }
    fn max_recoil_angle_x_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_x
    }
    fn min_recoil_angle_x(&self) -> &f32 {
        &self.min_recoil_angle_x
    }
    fn min_recoil_angle_x_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_x
    }
    fn max_recoil_angle_y(&self) -> &f32 {
        &self.max_recoil_angle_y
    }
    fn max_recoil_angle_y_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_y
    }
    fn min_recoil_angle_y(&self) -> &f32 {
        &self.min_recoil_angle_y
    }
    fn min_recoil_angle_y_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_y
    }
    fn max_recoil_angle_z(&self) -> &f32 {
        &self.max_recoil_angle_z
    }
    fn max_recoil_angle_z_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_z
    }
    fn min_recoil_angle_z(&self) -> &f32 {
        &self.min_recoil_angle_z
    }
    fn min_recoil_angle_z_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_z
    }
    fn max_recoil_fov(&self) -> &f32 {
        &self.max_recoil_fov
    }
    fn max_recoil_fov_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_fov
    }
    fn min_recoil_fov(&self) -> &f32 {
        &self.min_recoil_fov
    }
    fn min_recoil_fov_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_fov
    }
    fn max_recoil_angle_x_multiplier(&self) -> &f32 {
        &self.max_recoil_angle_x_multiplier
    }
    fn max_recoil_angle_x_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_x_multiplier
    }
    fn min_recoil_angle_x_multiplier(&self) -> &f32 {
        &self.min_recoil_angle_x_multiplier
    }
    fn min_recoil_angle_x_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_x_multiplier
    }
    fn max_recoil_angle_y_multiplier(&self) -> &f32 {
        &self.max_recoil_angle_y_multiplier
    }
    fn max_recoil_angle_y_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_y_multiplier
    }
    fn min_recoil_angle_y_multiplier(&self) -> &f32 {
        &self.min_recoil_angle_y_multiplier
    }
    fn min_recoil_angle_y_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_y_multiplier
    }
    fn max_recoil_angle_z_multiplier(&self) -> &f32 {
        &self.max_recoil_angle_z_multiplier
    }
    fn max_recoil_angle_z_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_z_multiplier
    }
    fn min_recoil_angle_z_multiplier(&self) -> &f32 {
        &self.min_recoil_angle_z_multiplier
    }
    fn min_recoil_angle_z_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_z_multiplier
    }
    fn max_recoil_fov_multiplier(&self) -> &f32 {
        &self.max_recoil_fov_multiplier
    }
    fn max_recoil_fov_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_fov_multiplier
    }
    fn min_recoil_fov_multiplier(&self) -> &f32 {
        &self.min_recoil_fov_multiplier
    }
    fn min_recoil_fov_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_fov_multiplier
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponRecoilModifier {
}

impl WeaponModifierBaseTrait for WeaponRecoilModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponRecoilModifier {
}

pub static WEAPONRECOILMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilModifier",
    name_hash: 910860878,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponRecoilModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponRecoilModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponRecoilModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxRecoilAngleX",
                name_hash: 1566215510,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_x),
            },
            FieldInfoData {
                name: "MinRecoilAngleX",
                name_hash: 2010820808,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_x),
            },
            FieldInfoData {
                name: "MaxRecoilAngleY",
                name_hash: 1566215511,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_y),
            },
            FieldInfoData {
                name: "MinRecoilAngleY",
                name_hash: 2010820809,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_y),
            },
            FieldInfoData {
                name: "MaxRecoilAngleZ",
                name_hash: 1566215508,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_z),
            },
            FieldInfoData {
                name: "MinRecoilAngleZ",
                name_hash: 2010820810,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_z),
            },
            FieldInfoData {
                name: "MaxRecoilFov",
                name_hash: 2812200304,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_fov),
            },
            FieldInfoData {
                name: "MinRecoilFov",
                name_hash: 3551417774,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_fov),
            },
            FieldInfoData {
                name: "MaxRecoilAngleXMultiplier",
                name_hash: 151916061,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_x_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilAngleXMultiplier",
                name_hash: 3388417667,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_x_multiplier),
            },
            FieldInfoData {
                name: "MaxRecoilAngleYMultiplier",
                name_hash: 3452971676,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_y_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilAngleYMultiplier",
                name_hash: 2394505986,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_y_multiplier),
            },
            FieldInfoData {
                name: "MaxRecoilAngleZMultiplier",
                name_hash: 2907124255,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_angle_z_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilAngleZMultiplier",
                name_hash: 633209473,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_angle_z_multiplier),
            },
            FieldInfoData {
                name: "MaxRecoilFovMultiplier",
                name_hash: 884425211,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, max_recoil_fov_multiplier),
            },
            FieldInfoData {
                name: "MinRecoilFovMultiplier",
                name_hash: 274047269,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponRecoilModifier, min_recoil_fov_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONRECOILMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponRecoilModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONRECOILMODIFIER_TYPE_INFO
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


pub static WEAPONRECOILMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponRecoilModifier-Array",
    name_hash: 1243970554,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponRecoilModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponOwnerModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
    pub unlock: Option<LockedTypeObject /* WeaponUnlockAsset */>,
}

pub trait WeaponOwnerModifierTrait: WeaponModifierDynamicBaseTrait {
    fn unlock(&self) -> &Option<LockedTypeObject /* WeaponUnlockAsset */>;
    fn unlock_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponUnlockAsset */>;
}

impl WeaponOwnerModifierTrait for WeaponOwnerModifier {
    fn unlock(&self) -> &Option<LockedTypeObject /* WeaponUnlockAsset */> {
        &self.unlock
    }
    fn unlock_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponUnlockAsset */> {
        &mut self.unlock
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponOwnerModifier {
}

impl WeaponModifierBaseTrait for WeaponOwnerModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponOwnerModifier {
}

pub static WEAPONOWNERMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOwnerModifier",
    name_hash: 1934898641,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponOwnerModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponOwnerModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponOwnerModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Unlock",
                name_hash: 2968292597,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponUnlockAsset",
                rust_offset: offset_of!(WeaponOwnerModifier, unlock),
            },
        ],
    }),
    array_type: Some(WEAPONOWNERMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponOwnerModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONOWNERMODIFIER_TYPE_INFO
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


pub static WEAPONOWNERMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponOwnerModifier-Array",
    name_hash: 332253157,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponOwnerModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponDispersionCombinableModifier {
    pub _glacier_base: WeaponDispersionModifier,
}

pub trait WeaponDispersionCombinableModifierTrait: WeaponDispersionModifierTrait {
}

impl WeaponDispersionCombinableModifierTrait for WeaponDispersionCombinableModifier {
}

impl WeaponDispersionModifierTrait for WeaponDispersionCombinableModifier {
    fn horizontal_modifier(&self) -> &f32 {
        self._glacier_base.horizontal_modifier()
    }
    fn horizontal_modifier_mut(&mut self) -> &mut f32 {
        self._glacier_base.horizontal_modifier_mut()
    }
    fn vertical_modifier(&self) -> &f32 {
        self._glacier_base.vertical_modifier()
    }
    fn vertical_modifier_mut(&mut self) -> &mut f32 {
        self._glacier_base.vertical_modifier_mut()
    }
    fn increase_per_shot(&self) -> &f32 {
        self._glacier_base.increase_per_shot()
    }
    fn increase_per_shot_mut(&mut self) -> &mut f32 {
        self._glacier_base.increase_per_shot_mut()
    }
    fn decrease_per_second(&self) -> &f32 {
        self._glacier_base.decrease_per_second()
    }
    fn decrease_per_second_mut(&mut self) -> &mut f32 {
        self._glacier_base.decrease_per_second_mut()
    }
    fn increase_per_shot_multiplier(&self) -> &f32 {
        self._glacier_base.increase_per_shot_multiplier()
    }
    fn increase_per_shot_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.increase_per_shot_multiplier_mut()
    }
    fn decrease_per_second_multiplier(&self) -> &f32 {
        self._glacier_base.decrease_per_second_multiplier()
    }
    fn decrease_per_second_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.decrease_per_second_multiplier_mut()
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponDispersionCombinableModifier {
}

impl WeaponModifierBaseTrait for WeaponDispersionCombinableModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponDispersionCombinableModifier {
}

pub static WEAPONDISPERSIONCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionCombinableModifier",
    name_hash: 2266354812,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDISPERSIONMODIFIER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponDispersionCombinableModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponDispersionCombinableModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponDispersionCombinableModifier as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONDISPERSIONCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponDispersionCombinableModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONDISPERSIONCOMBINABLEMODIFIER_TYPE_INFO
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


pub static WEAPONDISPERSIONCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionCombinableModifier-Array",
    name_hash: 3716144200,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponDispersionCombinableModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponDispersionModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
    pub horizontal_modifier: f32,
    pub vertical_modifier: f32,
    pub increase_per_shot: f32,
    pub decrease_per_second: f32,
    pub increase_per_shot_multiplier: f32,
    pub decrease_per_second_multiplier: f32,
}

pub trait WeaponDispersionModifierTrait: WeaponModifierDynamicBaseTrait {
    fn horizontal_modifier(&self) -> &f32;
    fn horizontal_modifier_mut(&mut self) -> &mut f32;
    fn vertical_modifier(&self) -> &f32;
    fn vertical_modifier_mut(&mut self) -> &mut f32;
    fn increase_per_shot(&self) -> &f32;
    fn increase_per_shot_mut(&mut self) -> &mut f32;
    fn decrease_per_second(&self) -> &f32;
    fn decrease_per_second_mut(&mut self) -> &mut f32;
    fn increase_per_shot_multiplier(&self) -> &f32;
    fn increase_per_shot_multiplier_mut(&mut self) -> &mut f32;
    fn decrease_per_second_multiplier(&self) -> &f32;
    fn decrease_per_second_multiplier_mut(&mut self) -> &mut f32;
}

impl WeaponDispersionModifierTrait for WeaponDispersionModifier {
    fn horizontal_modifier(&self) -> &f32 {
        &self.horizontal_modifier
    }
    fn horizontal_modifier_mut(&mut self) -> &mut f32 {
        &mut self.horizontal_modifier
    }
    fn vertical_modifier(&self) -> &f32 {
        &self.vertical_modifier
    }
    fn vertical_modifier_mut(&mut self) -> &mut f32 {
        &mut self.vertical_modifier
    }
    fn increase_per_shot(&self) -> &f32 {
        &self.increase_per_shot
    }
    fn increase_per_shot_mut(&mut self) -> &mut f32 {
        &mut self.increase_per_shot
    }
    fn decrease_per_second(&self) -> &f32 {
        &self.decrease_per_second
    }
    fn decrease_per_second_mut(&mut self) -> &mut f32 {
        &mut self.decrease_per_second
    }
    fn increase_per_shot_multiplier(&self) -> &f32 {
        &self.increase_per_shot_multiplier
    }
    fn increase_per_shot_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.increase_per_shot_multiplier
    }
    fn decrease_per_second_multiplier(&self) -> &f32 {
        &self.decrease_per_second_multiplier
    }
    fn decrease_per_second_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.decrease_per_second_multiplier
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponDispersionModifier {
}

impl WeaponModifierBaseTrait for WeaponDispersionModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponDispersionModifier {
}

pub static WEAPONDISPERSIONMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionModifier",
    name_hash: 151635474,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponDispersionModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponDispersionModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponDispersionModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HorizontalModifier",
                name_hash: 1914928364,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersionModifier, horizontal_modifier),
            },
            FieldInfoData {
                name: "VerticalModifier",
                name_hash: 3573710656,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersionModifier, vertical_modifier),
            },
            FieldInfoData {
                name: "IncreasePerShot",
                name_hash: 95639814,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersionModifier, increase_per_shot),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                name_hash: 3208554608,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersionModifier, decrease_per_second),
            },
            FieldInfoData {
                name: "IncreasePerShotMultiplier",
                name_hash: 571530445,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersionModifier, increase_per_shot_multiplier),
            },
            FieldInfoData {
                name: "DecreasePerSecondMultiplier",
                name_hash: 3553551611,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersionModifier, decrease_per_second_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONDISPERSIONMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponDispersionModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONDISPERSIONMODIFIER_TYPE_INFO
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


pub static WEAPONDISPERSIONMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersionModifier-Array",
    name_hash: 1151593766,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponDispersionModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponCrosshairTypeModifier {
    pub _glacier_base: WeaponModifierBase,
    pub crosshair_type: Option<LockedTypeObject /* super::game_shared::CrosshairTypeAsset */>,
}

pub trait WeaponCrosshairTypeModifierTrait: WeaponModifierBaseTrait {
    fn crosshair_type(&self) -> &Option<LockedTypeObject /* super::game_shared::CrosshairTypeAsset */>;
    fn crosshair_type_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::CrosshairTypeAsset */>;
}

impl WeaponCrosshairTypeModifierTrait for WeaponCrosshairTypeModifier {
    fn crosshair_type(&self) -> &Option<LockedTypeObject /* super::game_shared::CrosshairTypeAsset */> {
        &self.crosshair_type
    }
    fn crosshair_type_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::CrosshairTypeAsset */> {
        &mut self.crosshair_type
    }
}

impl WeaponModifierBaseTrait for WeaponCrosshairTypeModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponCrosshairTypeModifier {
}

pub static WEAPONCROSSHAIRTYPEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponCrosshairTypeModifier",
    name_hash: 1454310052,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponCrosshairTypeModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponCrosshairTypeModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponCrosshairTypeModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CrosshairType",
                name_hash: 839051281,
                flags: MemberInfoFlags::new(0),
                field_type: "CrosshairTypeAsset",
                rust_offset: offset_of!(WeaponCrosshairTypeModifier, crosshair_type),
            },
        ],
    }),
    array_type: Some(WEAPONCROSSHAIRTYPEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponCrosshairTypeModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONCROSSHAIRTYPEMODIFIER_TYPE_INFO
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


pub static WEAPONCROSSHAIRTYPEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponCrosshairTypeModifier-Array",
    name_hash: 1745131024,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponCrosshairTypeModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponMagazineModifier {
    pub _glacier_base: WeaponModifierBase,
    pub magazine_capacity: i32,
    pub number_of_magazines: i32,
    pub spawn_without_ammo: bool,
}

pub trait WeaponMagazineModifierTrait: WeaponModifierBaseTrait {
    fn magazine_capacity(&self) -> &i32;
    fn magazine_capacity_mut(&mut self) -> &mut i32;
    fn number_of_magazines(&self) -> &i32;
    fn number_of_magazines_mut(&mut self) -> &mut i32;
    fn spawn_without_ammo(&self) -> &bool;
    fn spawn_without_ammo_mut(&mut self) -> &mut bool;
}

impl WeaponMagazineModifierTrait for WeaponMagazineModifier {
    fn magazine_capacity(&self) -> &i32 {
        &self.magazine_capacity
    }
    fn magazine_capacity_mut(&mut self) -> &mut i32 {
        &mut self.magazine_capacity
    }
    fn number_of_magazines(&self) -> &i32 {
        &self.number_of_magazines
    }
    fn number_of_magazines_mut(&mut self) -> &mut i32 {
        &mut self.number_of_magazines
    }
    fn spawn_without_ammo(&self) -> &bool {
        &self.spawn_without_ammo
    }
    fn spawn_without_ammo_mut(&mut self) -> &mut bool {
        &mut self.spawn_without_ammo
    }
}

impl WeaponModifierBaseTrait for WeaponMagazineModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponMagazineModifier {
}

pub static WEAPONMAGAZINEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMagazineModifier",
    name_hash: 2705655170,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponMagazineModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponMagazineModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponMagazineModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MagazineCapacity",
                name_hash: 1634670339,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponMagazineModifier, magazine_capacity),
            },
            FieldInfoData {
                name: "NumberOfMagazines",
                name_hash: 2684433166,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponMagazineModifier, number_of_magazines),
            },
            FieldInfoData {
                name: "SpawnWithoutAmmo",
                name_hash: 4137126140,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMagazineModifier, spawn_without_ammo),
            },
        ],
    }),
    array_type: Some(WEAPONMAGAZINEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponMagazineModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONMAGAZINEMODIFIER_TYPE_INFO
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


pub static WEAPONMAGAZINEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMagazineModifier-Array",
    name_hash: 530449590,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponMagazineModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponFireLogicCombinableModifier {
    pub _glacier_base: WeaponFireLogicModifier,
}

pub trait WeaponFireLogicCombinableModifierTrait: WeaponFireLogicModifierTrait {
}

impl WeaponFireLogicCombinableModifierTrait for WeaponFireLogicCombinableModifier {
}

impl WeaponFireLogicModifierTrait for WeaponFireLogicCombinableModifier {
    fn rate_of_fire(&self) -> &f32 {
        self._glacier_base.rate_of_fire()
    }
    fn rate_of_fire_mut(&mut self) -> &mut f32 {
        self._glacier_base.rate_of_fire_mut()
    }
    fn rate_of_fire_for_burst(&self) -> &f32 {
        self._glacier_base.rate_of_fire_for_burst()
    }
    fn rate_of_fire_for_burst_mut(&mut self) -> &mut f32 {
        self._glacier_base.rate_of_fire_for_burst_mut()
    }
    fn rate_of_fire_multiplier(&self) -> &f32 {
        self._glacier_base.rate_of_fire_multiplier()
    }
    fn rate_of_fire_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.rate_of_fire_multiplier_mut()
    }
    fn rate_of_fire_for_burst_multiplier(&self) -> &f32 {
        self._glacier_base.rate_of_fire_for_burst_multiplier()
    }
    fn rate_of_fire_for_burst_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.rate_of_fire_for_burst_multiplier_mut()
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponFireLogicCombinableModifier {
}

impl WeaponModifierBaseTrait for WeaponFireLogicCombinableModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponFireLogicCombinableModifier {
}

pub static WEAPONFIRELOGICCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicCombinableModifier",
    name_hash: 2267498504,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONFIRELOGICMODIFIER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponFireLogicCombinableModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFireLogicCombinableModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponFireLogicCombinableModifier as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONFIRELOGICCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFireLogicCombinableModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRELOGICCOMBINABLEMODIFIER_TYPE_INFO
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


pub static WEAPONFIRELOGICCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicCombinableModifier-Array",
    name_hash: 278612540,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFireLogicCombinableModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponFireLogicModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
    pub rate_of_fire: f32,
    pub rate_of_fire_for_burst: f32,
    pub rate_of_fire_multiplier: f32,
    pub rate_of_fire_for_burst_multiplier: f32,
}

pub trait WeaponFireLogicModifierTrait: WeaponModifierDynamicBaseTrait {
    fn rate_of_fire(&self) -> &f32;
    fn rate_of_fire_mut(&mut self) -> &mut f32;
    fn rate_of_fire_for_burst(&self) -> &f32;
    fn rate_of_fire_for_burst_mut(&mut self) -> &mut f32;
    fn rate_of_fire_multiplier(&self) -> &f32;
    fn rate_of_fire_multiplier_mut(&mut self) -> &mut f32;
    fn rate_of_fire_for_burst_multiplier(&self) -> &f32;
    fn rate_of_fire_for_burst_multiplier_mut(&mut self) -> &mut f32;
}

impl WeaponFireLogicModifierTrait for WeaponFireLogicModifier {
    fn rate_of_fire(&self) -> &f32 {
        &self.rate_of_fire
    }
    fn rate_of_fire_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire
    }
    fn rate_of_fire_for_burst(&self) -> &f32 {
        &self.rate_of_fire_for_burst
    }
    fn rate_of_fire_for_burst_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire_for_burst
    }
    fn rate_of_fire_multiplier(&self) -> &f32 {
        &self.rate_of_fire_multiplier
    }
    fn rate_of_fire_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire_multiplier
    }
    fn rate_of_fire_for_burst_multiplier(&self) -> &f32 {
        &self.rate_of_fire_for_burst_multiplier
    }
    fn rate_of_fire_for_burst_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire_for_burst_multiplier
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponFireLogicModifier {
}

impl WeaponModifierBaseTrait for WeaponFireLogicModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponFireLogicModifier {
}

pub static WEAPONFIRELOGICMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicModifier",
    name_hash: 3613474150,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponFireLogicModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFireLogicModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponFireLogicModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RateOfFire",
                name_hash: 3866082710,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire),
            },
            FieldInfoData {
                name: "RateOfFireForBurst",
                name_hash: 1393806543,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire_for_burst),
            },
            FieldInfoData {
                name: "RateOfFireMultiplier",
                name_hash: 1714855261,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire_multiplier),
            },
            FieldInfoData {
                name: "RateOfFireForBurstMultiplier",
                name_hash: 2743921668,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFireLogicModifier, rate_of_fire_for_burst_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONFIRELOGICMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFireLogicModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRELOGICMODIFIER_TYPE_INFO
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


pub static WEAPONFIRELOGICMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFireLogicModifier-Array",
    name_hash: 613423698,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFireLogicModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponShotCombinableModifier {
    pub _glacier_base: WeaponShotModifier,
}

pub trait WeaponShotCombinableModifierTrait: WeaponShotModifierTrait {
}

impl WeaponShotCombinableModifierTrait for WeaponShotCombinableModifier {
}

impl WeaponShotModifierTrait for WeaponShotCombinableModifier {
    fn initial_speed(&self) -> &super::core::Vec3 {
        self._glacier_base.initial_speed()
    }
    fn initial_speed_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.initial_speed_mut()
    }
    fn number_of_bullets_per_shell(&self) -> &i32 {
        self._glacier_base.number_of_bullets_per_shell()
    }
    fn number_of_bullets_per_shell_mut(&mut self) -> &mut i32 {
        self._glacier_base.number_of_bullets_per_shell_mut()
    }
    fn number_of_bullets_per_burst(&self) -> &i32 {
        self._glacier_base.number_of_bullets_per_burst()
    }
    fn number_of_bullets_per_burst_mut(&mut self) -> &mut i32 {
        self._glacier_base.number_of_bullets_per_burst_mut()
    }
    fn initial_speed_multiplier(&self) -> &f32 {
        self._glacier_base.initial_speed_multiplier()
    }
    fn initial_speed_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_speed_multiplier_mut()
    }
    fn number_of_bullets_per_shell_multiplier(&self) -> &f32 {
        self._glacier_base.number_of_bullets_per_shell_multiplier()
    }
    fn number_of_bullets_per_shell_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.number_of_bullets_per_shell_multiplier_mut()
    }
    fn number_of_bullets_per_burst_multiplier(&self) -> &f32 {
        self._glacier_base.number_of_bullets_per_burst_multiplier()
    }
    fn number_of_bullets_per_burst_multiplier_mut(&mut self) -> &mut f32 {
        self._glacier_base.number_of_bullets_per_burst_multiplier_mut()
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponShotCombinableModifier {
}

impl WeaponModifierBaseTrait for WeaponShotCombinableModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponShotCombinableModifier {
}

pub static WEAPONSHOTCOMBINABLEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotCombinableModifier",
    name_hash: 1899717758,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONSHOTMODIFIER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponShotCombinableModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponShotCombinableModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponShotCombinableModifier as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONSHOTCOMBINABLEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponShotCombinableModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONSHOTCOMBINABLEMODIFIER_TYPE_INFO
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


pub static WEAPONSHOTCOMBINABLEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotCombinableModifier-Array",
    name_hash: 1799202122,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponShotCombinableModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponShotModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
    pub initial_speed: super::core::Vec3,
    pub number_of_bullets_per_shell: i32,
    pub number_of_bullets_per_burst: i32,
    pub initial_speed_multiplier: f32,
    pub number_of_bullets_per_shell_multiplier: f32,
    pub number_of_bullets_per_burst_multiplier: f32,
}

pub trait WeaponShotModifierTrait: WeaponModifierDynamicBaseTrait {
    fn initial_speed(&self) -> &super::core::Vec3;
    fn initial_speed_mut(&mut self) -> &mut super::core::Vec3;
    fn number_of_bullets_per_shell(&self) -> &i32;
    fn number_of_bullets_per_shell_mut(&mut self) -> &mut i32;
    fn number_of_bullets_per_burst(&self) -> &i32;
    fn number_of_bullets_per_burst_mut(&mut self) -> &mut i32;
    fn initial_speed_multiplier(&self) -> &f32;
    fn initial_speed_multiplier_mut(&mut self) -> &mut f32;
    fn number_of_bullets_per_shell_multiplier(&self) -> &f32;
    fn number_of_bullets_per_shell_multiplier_mut(&mut self) -> &mut f32;
    fn number_of_bullets_per_burst_multiplier(&self) -> &f32;
    fn number_of_bullets_per_burst_multiplier_mut(&mut self) -> &mut f32;
}

impl WeaponShotModifierTrait for WeaponShotModifier {
    fn initial_speed(&self) -> &super::core::Vec3 {
        &self.initial_speed
    }
    fn initial_speed_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.initial_speed
    }
    fn number_of_bullets_per_shell(&self) -> &i32 {
        &self.number_of_bullets_per_shell
    }
    fn number_of_bullets_per_shell_mut(&mut self) -> &mut i32 {
        &mut self.number_of_bullets_per_shell
    }
    fn number_of_bullets_per_burst(&self) -> &i32 {
        &self.number_of_bullets_per_burst
    }
    fn number_of_bullets_per_burst_mut(&mut self) -> &mut i32 {
        &mut self.number_of_bullets_per_burst
    }
    fn initial_speed_multiplier(&self) -> &f32 {
        &self.initial_speed_multiplier
    }
    fn initial_speed_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.initial_speed_multiplier
    }
    fn number_of_bullets_per_shell_multiplier(&self) -> &f32 {
        &self.number_of_bullets_per_shell_multiplier
    }
    fn number_of_bullets_per_shell_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.number_of_bullets_per_shell_multiplier
    }
    fn number_of_bullets_per_burst_multiplier(&self) -> &f32 {
        &self.number_of_bullets_per_burst_multiplier
    }
    fn number_of_bullets_per_burst_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.number_of_bullets_per_burst_multiplier
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponShotModifier {
}

impl WeaponModifierBaseTrait for WeaponShotModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponShotModifier {
}

pub static WEAPONSHOTMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotModifier",
    name_hash: 1419222928,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponShotModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponShotModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponShotModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InitialSpeed",
                name_hash: 3681055580,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WeaponShotModifier, initial_speed),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShell",
                name_hash: 464437027,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_shell),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerBurst",
                name_hash: 491113727,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_burst),
            },
            FieldInfoData {
                name: "InitialSpeedMultiplier",
                name_hash: 3011278359,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponShotModifier, initial_speed_multiplier),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShellMultiplier",
                name_hash: 2636457384,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_shell_multiplier),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerBurstMultiplier",
                name_hash: 2709430388,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponShotModifier, number_of_bullets_per_burst_multiplier),
            },
        ],
    }),
    array_type: Some(WEAPONSHOTMODIFIER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WeaponShotModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONSHOTMODIFIER_TYPE_INFO
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


pub static WEAPONSHOTMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponShotModifier-Array",
    name_hash: 1189260708,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponShotModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponProjectileModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
    pub projectile_data: Option<LockedTypeObject /* ProjectileEntityData */>,
    pub projectile: Option<LockedTypeObject /* ProjectileBlueprint */>,
    pub max_count: i32,
}

pub trait WeaponProjectileModifierTrait: WeaponModifierDynamicBaseTrait {
    fn projectile_data(&self) -> &Option<LockedTypeObject /* ProjectileEntityData */>;
    fn projectile_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileEntityData */>;
    fn projectile(&self) -> &Option<LockedTypeObject /* ProjectileBlueprint */>;
    fn projectile_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileBlueprint */>;
    fn max_count(&self) -> &i32;
    fn max_count_mut(&mut self) -> &mut i32;
}

impl WeaponProjectileModifierTrait for WeaponProjectileModifier {
    fn projectile_data(&self) -> &Option<LockedTypeObject /* ProjectileEntityData */> {
        &self.projectile_data
    }
    fn projectile_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileEntityData */> {
        &mut self.projectile_data
    }
    fn projectile(&self) -> &Option<LockedTypeObject /* ProjectileBlueprint */> {
        &self.projectile
    }
    fn projectile_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileBlueprint */> {
        &mut self.projectile
    }
    fn max_count(&self) -> &i32 {
        &self.max_count
    }
    fn max_count_mut(&mut self) -> &mut i32 {
        &mut self.max_count
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponProjectileModifier {
}

impl WeaponModifierBaseTrait for WeaponProjectileModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponProjectileModifier {
}

pub static WEAPONPROJECTILEMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponProjectileModifier",
    name_hash: 414286341,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponProjectileModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponProjectileModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponProjectileModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ProjectileData",
                name_hash: 2690942720,
                flags: MemberInfoFlags::new(0),
                field_type: "ProjectileEntityData",
                rust_offset: offset_of!(WeaponProjectileModifier, projectile_data),
            },
            FieldInfoData {
                name: "Projectile",
                name_hash: 3883939888,
                flags: MemberInfoFlags::new(0),
                field_type: "ProjectileBlueprint",
                rust_offset: offset_of!(WeaponProjectileModifier, projectile),
            },
            FieldInfoData {
                name: "MaxCount",
                name_hash: 415061138,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponProjectileModifier, max_count),
            },
        ],
    }),
    array_type: Some(WEAPONPROJECTILEMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponProjectileModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONPROJECTILEMODIFIER_TYPE_INFO
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


pub static WEAPONPROJECTILEMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponProjectileModifier-Array",
    name_hash: 1844718001,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponProjectileModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponSoundModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
    pub sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub mute_primary_sound_when_active: bool,
}

pub trait WeaponSoundModifierTrait: WeaponModifierDynamicBaseTrait {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn mute_primary_sound_when_active(&self) -> &bool;
    fn mute_primary_sound_when_active_mut(&mut self) -> &mut bool;
}

impl WeaponSoundModifierTrait for WeaponSoundModifier {
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.sound
    }
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.sound
    }
    fn mute_primary_sound_when_active(&self) -> &bool {
        &self.mute_primary_sound_when_active
    }
    fn mute_primary_sound_when_active_mut(&mut self) -> &mut bool {
        &mut self.mute_primary_sound_when_active
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponSoundModifier {
}

impl WeaponModifierBaseTrait for WeaponSoundModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponSoundModifier {
}

pub static WEAPONSOUNDMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSoundModifier",
    name_hash: 2482439603,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponSoundModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponSoundModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponSoundModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Sound",
                name_hash: 231353798,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(WeaponSoundModifier, sound),
            },
            FieldInfoData {
                name: "MutePrimarySoundWhenActive",
                name_hash: 3151260539,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponSoundModifier, mute_primary_sound_when_active),
            },
        ],
    }),
    array_type: Some(WEAPONSOUNDMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponSoundModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONSOUNDMODIFIER_TYPE_INFO
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


pub static WEAPONSOUNDMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSoundModifier-Array",
    name_hash: 2271588615,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSoundModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponFiringEffectsModifier {
    pub _glacier_base: WeaponModifierDynamicBase,
    pub fire_effects1p: Vec<BoxedTypeObject /* FireEffectData */>,
    pub fire_effects3p: Vec<BoxedTypeObject /* FireEffectData */>,
}

pub trait WeaponFiringEffectsModifierTrait: WeaponModifierDynamicBaseTrait {
    fn fire_effects1p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */>;
    fn fire_effects1p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */>;
    fn fire_effects3p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */>;
    fn fire_effects3p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */>;
}

impl WeaponFiringEffectsModifierTrait for WeaponFiringEffectsModifier {
    fn fire_effects1p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */> {
        &self.fire_effects1p
    }
    fn fire_effects1p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */> {
        &mut self.fire_effects1p
    }
    fn fire_effects3p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */> {
        &self.fire_effects3p
    }
    fn fire_effects3p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */> {
        &mut self.fire_effects3p
    }
}

impl WeaponModifierDynamicBaseTrait for WeaponFiringEffectsModifier {
}

impl WeaponModifierBaseTrait for WeaponFiringEffectsModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponFiringEffectsModifier {
}

pub static WEAPONFIRINGEFFECTSMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEffectsModifier",
    name_hash: 4024544681,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERDYNAMICBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponFiringEffectsModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFiringEffectsModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponFiringEffectsModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FireEffects1p",
                name_hash: 3749424600,
                flags: MemberInfoFlags::new(144),
                field_type: "FireEffectData-Array",
                rust_offset: offset_of!(WeaponFiringEffectsModifier, fire_effects1p),
            },
            FieldInfoData {
                name: "FireEffects3p",
                name_hash: 3749424538,
                flags: MemberInfoFlags::new(144),
                field_type: "FireEffectData-Array",
                rust_offset: offset_of!(WeaponFiringEffectsModifier, fire_effects3p),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGEFFECTSMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringEffectsModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRINGEFFECTSMODIFIER_TYPE_INFO
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


pub static WEAPONFIRINGEFFECTSMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEffectsModifier-Array",
    name_hash: 3143791645,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringEffectsModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponFiringDataModifier {
    pub _glacier_base: WeaponModifierBase,
    pub weapon_firing: Option<LockedTypeObject /* WeaponFiringData */>,
}

pub trait WeaponFiringDataModifierTrait: WeaponModifierBaseTrait {
    fn weapon_firing(&self) -> &Option<LockedTypeObject /* WeaponFiringData */>;
    fn weapon_firing_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponFiringData */>;
}

impl WeaponFiringDataModifierTrait for WeaponFiringDataModifier {
    fn weapon_firing(&self) -> &Option<LockedTypeObject /* WeaponFiringData */> {
        &self.weapon_firing
    }
    fn weapon_firing_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponFiringData */> {
        &mut self.weapon_firing
    }
}

impl WeaponModifierBaseTrait for WeaponFiringDataModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponFiringDataModifier {
}

pub static WEAPONFIRINGDATAMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataModifier",
    name_hash: 1155530429,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponFiringDataModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFiringDataModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponFiringDataModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WeaponFiring",
                name_hash: 1823157050,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponFiringData",
                rust_offset: offset_of!(WeaponFiringDataModifier, weapon_firing),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGDATAMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringDataModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRINGDATAMODIFIER_TYPE_INFO
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


pub static WEAPONFIRINGDATAMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataModifier-Array",
    name_hash: 1961622281,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringDataModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponMiscModifier {
    pub _glacier_base: WeaponModifierBase,
    pub enable_breath_control: bool,
    pub can_be_in_supported_shooting: bool,
    pub un_zoom_on_bolt_action: bool,
    pub hold_bolt_action_until_zoom_release: bool,
    pub is_silenced: bool,
}

pub trait WeaponMiscModifierTrait: WeaponModifierBaseTrait {
    fn enable_breath_control(&self) -> &bool;
    fn enable_breath_control_mut(&mut self) -> &mut bool;
    fn can_be_in_supported_shooting(&self) -> &bool;
    fn can_be_in_supported_shooting_mut(&mut self) -> &mut bool;
    fn un_zoom_on_bolt_action(&self) -> &bool;
    fn un_zoom_on_bolt_action_mut(&mut self) -> &mut bool;
    fn hold_bolt_action_until_zoom_release(&self) -> &bool;
    fn hold_bolt_action_until_zoom_release_mut(&mut self) -> &mut bool;
    fn is_silenced(&self) -> &bool;
    fn is_silenced_mut(&mut self) -> &mut bool;
}

impl WeaponMiscModifierTrait for WeaponMiscModifier {
    fn enable_breath_control(&self) -> &bool {
        &self.enable_breath_control
    }
    fn enable_breath_control_mut(&mut self) -> &mut bool {
        &mut self.enable_breath_control
    }
    fn can_be_in_supported_shooting(&self) -> &bool {
        &self.can_be_in_supported_shooting
    }
    fn can_be_in_supported_shooting_mut(&mut self) -> &mut bool {
        &mut self.can_be_in_supported_shooting
    }
    fn un_zoom_on_bolt_action(&self) -> &bool {
        &self.un_zoom_on_bolt_action
    }
    fn un_zoom_on_bolt_action_mut(&mut self) -> &mut bool {
        &mut self.un_zoom_on_bolt_action
    }
    fn hold_bolt_action_until_zoom_release(&self) -> &bool {
        &self.hold_bolt_action_until_zoom_release
    }
    fn hold_bolt_action_until_zoom_release_mut(&mut self) -> &mut bool {
        &mut self.hold_bolt_action_until_zoom_release
    }
    fn is_silenced(&self) -> &bool {
        &self.is_silenced
    }
    fn is_silenced_mut(&mut self) -> &mut bool {
        &mut self.is_silenced
    }
}

impl WeaponModifierBaseTrait for WeaponMiscModifier {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponMiscModifier {
}

pub static WEAPONMISCMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifier",
    name_hash: 582823428,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponMiscModifier, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponMiscModifier as Default>::default())),
            create_boxed: || Box::new(<WeaponMiscModifier as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnableBreathControl",
                name_hash: 613505579,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifier, enable_breath_control),
            },
            FieldInfoData {
                name: "CanBeInSupportedShooting",
                name_hash: 2187452328,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifier, can_be_in_supported_shooting),
            },
            FieldInfoData {
                name: "UnZoomOnBoltAction",
                name_hash: 1438634691,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifier, un_zoom_on_bolt_action),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilZoomRelease",
                name_hash: 3133940949,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifier, hold_bolt_action_until_zoom_release),
            },
            FieldInfoData {
                name: "IsSilenced",
                name_hash: 1644964768,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponMiscModifier, is_silenced),
            },
        ],
    }),
    array_type: Some(WEAPONMISCMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponMiscModifier {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONMISCMODIFIER_TYPE_INFO
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


pub static WEAPONMISCMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponMiscModifier-Array",
    name_hash: 4182114864,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponMiscModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponModifierDynamicBase {
    pub _glacier_base: WeaponModifierBase,
}

pub trait WeaponModifierDynamicBaseTrait: WeaponModifierBaseTrait {
}

impl WeaponModifierDynamicBaseTrait for WeaponModifierDynamicBase {
}

impl WeaponModifierBaseTrait for WeaponModifierDynamicBase {
    fn apply_order(&self) -> &i32 {
        self._glacier_base.apply_order()
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        self._glacier_base.apply_order_mut()
    }
    fn dynamic_update_enabled(&self) -> &bool {
        self._glacier_base.dynamic_update_enabled()
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.dynamic_update_enabled_mut()
    }
}

impl super::core::DataContainerTrait for WeaponModifierDynamicBase {
}

pub static WEAPONMODIFIERDYNAMICBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierDynamicBase",
    name_hash: 3904287888,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONMODIFIERBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponModifierDynamicBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponModifierDynamicBase as Default>::default())),
            create_boxed: || Box::new(<WeaponModifierDynamicBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONMODIFIERDYNAMICBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponModifierDynamicBase {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONMODIFIERDYNAMICBASE_TYPE_INFO
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


pub static WEAPONMODIFIERDYNAMICBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierDynamicBase-Array",
    name_hash: 3174924964,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponModifierDynamicBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponModifierBase {
    pub _glacier_base: super::core::DataContainer,
    pub apply_order: i32,
    pub dynamic_update_enabled: bool,
}

pub trait WeaponModifierBaseTrait: super::core::DataContainerTrait {
    fn apply_order(&self) -> &i32;
    fn apply_order_mut(&mut self) -> &mut i32;
    fn dynamic_update_enabled(&self) -> &bool;
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool;
}

impl WeaponModifierBaseTrait for WeaponModifierBase {
    fn apply_order(&self) -> &i32 {
        &self.apply_order
    }
    fn apply_order_mut(&mut self) -> &mut i32 {
        &mut self.apply_order
    }
    fn dynamic_update_enabled(&self) -> &bool {
        &self.dynamic_update_enabled
    }
    fn dynamic_update_enabled_mut(&mut self) -> &mut bool {
        &mut self.dynamic_update_enabled
    }
}

impl super::core::DataContainerTrait for WeaponModifierBase {
}

pub static WEAPONMODIFIERBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierBase",
    name_hash: 76686565,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponModifierBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponModifierBase as Default>::default())),
            create_boxed: || Box::new(<WeaponModifierBase as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ApplyOrder",
                name_hash: 4069599615,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponModifierBase, apply_order),
            },
            FieldInfoData {
                name: "DynamicUpdateEnabled",
                name_hash: 1076632388,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponModifierBase, dynamic_update_enabled),
            },
        ],
    }),
    array_type: Some(WEAPONMODIFIERBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponModifierBase {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONMODIFIERBASE_TYPE_INFO
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


pub static WEAPONMODIFIERBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponModifierBase-Array",
    name_hash: 217050577,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponModifierBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LaserPainterData {
    pub _glacier_base: LockingWeaponData,
    pub time_object_is_painted: f32,
}

pub trait LaserPainterDataTrait: LockingWeaponDataTrait {
    fn time_object_is_painted(&self) -> &f32;
    fn time_object_is_painted_mut(&mut self) -> &mut f32;
}

impl LaserPainterDataTrait for LaserPainterData {
    fn time_object_is_painted(&self) -> &f32 {
        &self.time_object_is_painted
    }
    fn time_object_is_painted_mut(&mut self) -> &mut f32 {
        &mut self.time_object_is_painted
    }
}

impl LockingWeaponDataTrait for LaserPainterData {
    fn locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.locking_controller()
    }
    fn locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.locking_controller_mut()
    }
    fn secondary_locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.secondary_locking_controller()
    }
    fn secondary_locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.secondary_locking_controller_mut()
    }
    fn override_locking_controller_settings(&self) -> &bool {
        self._glacier_base.override_locking_controller_settings()
    }
    fn override_locking_controller_settings_mut(&mut self) -> &mut bool {
        self._glacier_base.override_locking_controller_settings_mut()
    }
    fn is_homing(&self) -> &bool {
        self._glacier_base.is_homing()
    }
    fn is_homing_mut(&mut self) -> &mut bool {
        self._glacier_base.is_homing_mut()
    }
    fn is_guided(&self) -> &bool {
        self._glacier_base.is_guided()
    }
    fn is_guided_mut(&mut self) -> &mut bool {
        self._glacier_base.is_guided_mut()
    }
    fn is_guided_when_zoomed(&self) -> &bool {
        self._glacier_base.is_guided_when_zoomed()
    }
    fn is_guided_when_zoomed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_guided_when_zoomed_mut()
    }
    fn is_guided_homing(&self) -> &bool {
        self._glacier_base.is_guided_homing()
    }
    fn is_guided_homing_mut(&mut self) -> &mut bool {
        self._glacier_base.is_guided_homing_mut()
    }
    fn fire_only_when_locked_on(&self) -> &bool {
        self._glacier_base.fire_only_when_locked_on()
    }
    fn fire_only_when_locked_on_mut(&mut self) -> &mut bool {
        self._glacier_base.fire_only_when_locked_on_mut()
    }
    fn guide_only_when_locked_on(&self) -> &bool {
        self._glacier_base.guide_only_when_locked_on()
    }
    fn guide_only_when_locked_on_mut(&mut self) -> &mut bool {
        self._glacier_base.guide_only_when_locked_on_mut()
    }
    fn warn_lock(&self) -> &WarnTarget {
        self._glacier_base.warn_lock()
    }
    fn warn_lock_mut(&mut self) -> &mut WarnTarget {
        self._glacier_base.warn_lock_mut()
    }
}

impl WeaponDataTrait for LaserPainterData {
    fn show_laser_painted_vehicles(&self) -> &bool {
        self._glacier_base.show_laser_painted_vehicles()
    }
    fn show_laser_painted_vehicles_mut(&mut self) -> &mut bool {
        self._glacier_base.show_laser_painted_vehicles_mut()
    }
    fn apply_power_to_speed(&self) -> &bool {
        self._glacier_base.apply_power_to_speed()
    }
    fn apply_power_to_speed_mut(&mut self) -> &mut bool {
        self._glacier_base.apply_power_to_speed_mut()
    }
}

impl super::gameplay_sim::ToolDataTrait for LaserPainterData {
    fn is_always_active(&self) -> &bool {
        self._glacier_base.is_always_active()
    }
    fn is_always_active_mut(&mut self) -> &mut bool {
        self._glacier_base.is_always_active_mut()
    }
}

impl super::core::DataContainerTrait for LaserPainterData {
}

pub static LASERPAINTERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserPainterData",
    name_hash: 1045078793,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCKINGWEAPONDATA_TYPE_INFO),
        super_class_offset: offset_of!(LaserPainterData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LaserPainterData as Default>::default())),
            create_boxed: || Box::new(<LaserPainterData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TimeObjectIsPainted",
                name_hash: 641195484,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LaserPainterData, time_object_is_painted),
            },
        ],
    }),
    array_type: Some(LASERPAINTERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LaserPainterData {
    fn type_info(&self) -> &'static TypeInfo {
        LASERPAINTERDATA_TYPE_INFO
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


pub static LASERPAINTERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserPainterData-Array",
    name_hash: 90290365,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LaserPainterData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LaserDesignatorData {
    pub _glacier_base: LockingWeaponData,
    pub post_lock_time: f32,
    pub bomber_time: f32,
    pub bomb_warn_time: f32,
    pub bomber_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
}

pub trait LaserDesignatorDataTrait: LockingWeaponDataTrait {
    fn post_lock_time(&self) -> &f32;
    fn post_lock_time_mut(&mut self) -> &mut f32;
    fn bomber_time(&self) -> &f32;
    fn bomber_time_mut(&mut self) -> &mut f32;
    fn bomb_warn_time(&self) -> &f32;
    fn bomb_warn_time_mut(&mut self) -> &mut f32;
    fn bomber_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn bomber_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
}

impl LaserDesignatorDataTrait for LaserDesignatorData {
    fn post_lock_time(&self) -> &f32 {
        &self.post_lock_time
    }
    fn post_lock_time_mut(&mut self) -> &mut f32 {
        &mut self.post_lock_time
    }
    fn bomber_time(&self) -> &f32 {
        &self.bomber_time
    }
    fn bomber_time_mut(&mut self) -> &mut f32 {
        &mut self.bomber_time
    }
    fn bomb_warn_time(&self) -> &f32 {
        &self.bomb_warn_time
    }
    fn bomb_warn_time_mut(&mut self) -> &mut f32 {
        &mut self.bomb_warn_time
    }
    fn bomber_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.bomber_sound
    }
    fn bomber_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.bomber_sound
    }
}

impl LockingWeaponDataTrait for LaserDesignatorData {
    fn locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.locking_controller()
    }
    fn locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.locking_controller_mut()
    }
    fn secondary_locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.secondary_locking_controller()
    }
    fn secondary_locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */> {
        self._glacier_base.secondary_locking_controller_mut()
    }
    fn override_locking_controller_settings(&self) -> &bool {
        self._glacier_base.override_locking_controller_settings()
    }
    fn override_locking_controller_settings_mut(&mut self) -> &mut bool {
        self._glacier_base.override_locking_controller_settings_mut()
    }
    fn is_homing(&self) -> &bool {
        self._glacier_base.is_homing()
    }
    fn is_homing_mut(&mut self) -> &mut bool {
        self._glacier_base.is_homing_mut()
    }
    fn is_guided(&self) -> &bool {
        self._glacier_base.is_guided()
    }
    fn is_guided_mut(&mut self) -> &mut bool {
        self._glacier_base.is_guided_mut()
    }
    fn is_guided_when_zoomed(&self) -> &bool {
        self._glacier_base.is_guided_when_zoomed()
    }
    fn is_guided_when_zoomed_mut(&mut self) -> &mut bool {
        self._glacier_base.is_guided_when_zoomed_mut()
    }
    fn is_guided_homing(&self) -> &bool {
        self._glacier_base.is_guided_homing()
    }
    fn is_guided_homing_mut(&mut self) -> &mut bool {
        self._glacier_base.is_guided_homing_mut()
    }
    fn fire_only_when_locked_on(&self) -> &bool {
        self._glacier_base.fire_only_when_locked_on()
    }
    fn fire_only_when_locked_on_mut(&mut self) -> &mut bool {
        self._glacier_base.fire_only_when_locked_on_mut()
    }
    fn guide_only_when_locked_on(&self) -> &bool {
        self._glacier_base.guide_only_when_locked_on()
    }
    fn guide_only_when_locked_on_mut(&mut self) -> &mut bool {
        self._glacier_base.guide_only_when_locked_on_mut()
    }
    fn warn_lock(&self) -> &WarnTarget {
        self._glacier_base.warn_lock()
    }
    fn warn_lock_mut(&mut self) -> &mut WarnTarget {
        self._glacier_base.warn_lock_mut()
    }
}

impl WeaponDataTrait for LaserDesignatorData {
    fn show_laser_painted_vehicles(&self) -> &bool {
        self._glacier_base.show_laser_painted_vehicles()
    }
    fn show_laser_painted_vehicles_mut(&mut self) -> &mut bool {
        self._glacier_base.show_laser_painted_vehicles_mut()
    }
    fn apply_power_to_speed(&self) -> &bool {
        self._glacier_base.apply_power_to_speed()
    }
    fn apply_power_to_speed_mut(&mut self) -> &mut bool {
        self._glacier_base.apply_power_to_speed_mut()
    }
}

impl super::gameplay_sim::ToolDataTrait for LaserDesignatorData {
    fn is_always_active(&self) -> &bool {
        self._glacier_base.is_always_active()
    }
    fn is_always_active_mut(&mut self) -> &mut bool {
        self._glacier_base.is_always_active_mut()
    }
}

impl super::core::DataContainerTrait for LaserDesignatorData {
}

pub static LASERDESIGNATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserDesignatorData",
    name_hash: 1843720614,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCKINGWEAPONDATA_TYPE_INFO),
        super_class_offset: offset_of!(LaserDesignatorData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LaserDesignatorData as Default>::default())),
            create_boxed: || Box::new(<LaserDesignatorData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PostLockTime",
                name_hash: 4004851651,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LaserDesignatorData, post_lock_time),
            },
            FieldInfoData {
                name: "BomberTime",
                name_hash: 3089415333,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LaserDesignatorData, bomber_time),
            },
            FieldInfoData {
                name: "BombWarnTime",
                name_hash: 2755941208,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LaserDesignatorData, bomb_warn_time),
            },
            FieldInfoData {
                name: "BomberSound",
                name_hash: 3167598099,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(LaserDesignatorData, bomber_sound),
            },
        ],
    }),
    array_type: Some(LASERDESIGNATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LaserDesignatorData {
    fn type_info(&self) -> &'static TypeInfo {
        LASERDESIGNATORDATA_TYPE_INFO
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


pub static LASERDESIGNATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LaserDesignatorData-Array",
    name_hash: 2347384338,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LaserDesignatorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LockingWeaponData {
    pub _glacier_base: WeaponData,
    pub locking_controller: Option<LockedTypeObject /* LockingControllerData */>,
    pub secondary_locking_controller: Option<LockedTypeObject /* LockingControllerData */>,
    pub override_locking_controller_settings: bool,
    pub is_homing: bool,
    pub is_guided: bool,
    pub is_guided_when_zoomed: bool,
    pub is_guided_homing: bool,
    pub fire_only_when_locked_on: bool,
    pub guide_only_when_locked_on: bool,
    pub warn_lock: WarnTarget,
}

pub trait LockingWeaponDataTrait: WeaponDataTrait {
    fn locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */>;
    fn locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */>;
    fn secondary_locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */>;
    fn secondary_locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */>;
    fn override_locking_controller_settings(&self) -> &bool;
    fn override_locking_controller_settings_mut(&mut self) -> &mut bool;
    fn is_homing(&self) -> &bool;
    fn is_homing_mut(&mut self) -> &mut bool;
    fn is_guided(&self) -> &bool;
    fn is_guided_mut(&mut self) -> &mut bool;
    fn is_guided_when_zoomed(&self) -> &bool;
    fn is_guided_when_zoomed_mut(&mut self) -> &mut bool;
    fn is_guided_homing(&self) -> &bool;
    fn is_guided_homing_mut(&mut self) -> &mut bool;
    fn fire_only_when_locked_on(&self) -> &bool;
    fn fire_only_when_locked_on_mut(&mut self) -> &mut bool;
    fn guide_only_when_locked_on(&self) -> &bool;
    fn guide_only_when_locked_on_mut(&mut self) -> &mut bool;
    fn warn_lock(&self) -> &WarnTarget;
    fn warn_lock_mut(&mut self) -> &mut WarnTarget;
}

impl LockingWeaponDataTrait for LockingWeaponData {
    fn locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */> {
        &self.locking_controller
    }
    fn locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */> {
        &mut self.locking_controller
    }
    fn secondary_locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */> {
        &self.secondary_locking_controller
    }
    fn secondary_locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */> {
        &mut self.secondary_locking_controller
    }
    fn override_locking_controller_settings(&self) -> &bool {
        &self.override_locking_controller_settings
    }
    fn override_locking_controller_settings_mut(&mut self) -> &mut bool {
        &mut self.override_locking_controller_settings
    }
    fn is_homing(&self) -> &bool {
        &self.is_homing
    }
    fn is_homing_mut(&mut self) -> &mut bool {
        &mut self.is_homing
    }
    fn is_guided(&self) -> &bool {
        &self.is_guided
    }
    fn is_guided_mut(&mut self) -> &mut bool {
        &mut self.is_guided
    }
    fn is_guided_when_zoomed(&self) -> &bool {
        &self.is_guided_when_zoomed
    }
    fn is_guided_when_zoomed_mut(&mut self) -> &mut bool {
        &mut self.is_guided_when_zoomed
    }
    fn is_guided_homing(&self) -> &bool {
        &self.is_guided_homing
    }
    fn is_guided_homing_mut(&mut self) -> &mut bool {
        &mut self.is_guided_homing
    }
    fn fire_only_when_locked_on(&self) -> &bool {
        &self.fire_only_when_locked_on
    }
    fn fire_only_when_locked_on_mut(&mut self) -> &mut bool {
        &mut self.fire_only_when_locked_on
    }
    fn guide_only_when_locked_on(&self) -> &bool {
        &self.guide_only_when_locked_on
    }
    fn guide_only_when_locked_on_mut(&mut self) -> &mut bool {
        &mut self.guide_only_when_locked_on
    }
    fn warn_lock(&self) -> &WarnTarget {
        &self.warn_lock
    }
    fn warn_lock_mut(&mut self) -> &mut WarnTarget {
        &mut self.warn_lock
    }
}

impl WeaponDataTrait for LockingWeaponData {
    fn show_laser_painted_vehicles(&self) -> &bool {
        self._glacier_base.show_laser_painted_vehicles()
    }
    fn show_laser_painted_vehicles_mut(&mut self) -> &mut bool {
        self._glacier_base.show_laser_painted_vehicles_mut()
    }
    fn apply_power_to_speed(&self) -> &bool {
        self._glacier_base.apply_power_to_speed()
    }
    fn apply_power_to_speed_mut(&mut self) -> &mut bool {
        self._glacier_base.apply_power_to_speed_mut()
    }
}

impl super::gameplay_sim::ToolDataTrait for LockingWeaponData {
    fn is_always_active(&self) -> &bool {
        self._glacier_base.is_always_active()
    }
    fn is_always_active_mut(&mut self) -> &mut bool {
        self._glacier_base.is_always_active_mut()
    }
}

impl super::core::DataContainerTrait for LockingWeaponData {
}

pub static LOCKINGWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingWeaponData",
    name_hash: 837219740,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDATA_TYPE_INFO),
        super_class_offset: offset_of!(LockingWeaponData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LockingWeaponData as Default>::default())),
            create_boxed: || Box::new(<LockingWeaponData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LockingController",
                name_hash: 1783566994,
                flags: MemberInfoFlags::new(0),
                field_type: "LockingControllerData",
                rust_offset: offset_of!(LockingWeaponData, locking_controller),
            },
            FieldInfoData {
                name: "SecondaryLockingController",
                name_hash: 212156840,
                flags: MemberInfoFlags::new(0),
                field_type: "LockingControllerData",
                rust_offset: offset_of!(LockingWeaponData, secondary_locking_controller),
            },
            FieldInfoData {
                name: "OverrideLockingControllerSettings",
                name_hash: 1916490723,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingWeaponData, override_locking_controller_settings),
            },
            FieldInfoData {
                name: "IsHoming",
                name_hash: 962805909,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingWeaponData, is_homing),
            },
            FieldInfoData {
                name: "IsGuided",
                name_hash: 911651905,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingWeaponData, is_guided),
            },
            FieldInfoData {
                name: "IsGuidedWhenZoomed",
                name_hash: 951364163,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingWeaponData, is_guided_when_zoomed),
            },
            FieldInfoData {
                name: "IsGuidedHoming",
                name_hash: 399064139,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingWeaponData, is_guided_homing),
            },
            FieldInfoData {
                name: "FireOnlyWhenLockedOn",
                name_hash: 708469046,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingWeaponData, fire_only_when_locked_on),
            },
            FieldInfoData {
                name: "GuideOnlyWhenLockedOn",
                name_hash: 1926737172,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingWeaponData, guide_only_when_locked_on),
            },
            FieldInfoData {
                name: "WarnLock",
                name_hash: 2457242500,
                flags: MemberInfoFlags::new(0),
                field_type: "WarnTarget",
                rust_offset: offset_of!(LockingWeaponData, warn_lock),
            },
        ],
    }),
    array_type: Some(LOCKINGWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LockingWeaponData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCKINGWEAPONDATA_TYPE_INFO
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


pub static LOCKINGWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingWeaponData-Array",
    name_hash: 395939240,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LockingWeaponData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ArtilleryStrikeWeaponData {
    pub _glacier_base: WeaponData,
    pub max_strike_distance: f32,
    pub strike_radius: f32,
    pub spawn_height: f32,
    pub max_random_spawn_height: f32,
    pub increase_spawn_height_with_distance: bool,
    pub spawn_height_multiplier: f32,
    pub camera: Option<LockedTypeObject /* super::gameplay_sim::TargetCameraData */>,
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

pub trait ArtilleryStrikeWeaponDataTrait: WeaponDataTrait {
    fn max_strike_distance(&self) -> &f32;
    fn max_strike_distance_mut(&mut self) -> &mut f32;
    fn strike_radius(&self) -> &f32;
    fn strike_radius_mut(&mut self) -> &mut f32;
    fn spawn_height(&self) -> &f32;
    fn spawn_height_mut(&mut self) -> &mut f32;
    fn max_random_spawn_height(&self) -> &f32;
    fn max_random_spawn_height_mut(&mut self) -> &mut f32;
    fn increase_spawn_height_with_distance(&self) -> &bool;
    fn increase_spawn_height_with_distance_mut(&mut self) -> &mut bool;
    fn spawn_height_multiplier(&self) -> &f32;
    fn spawn_height_multiplier_mut(&mut self) -> &mut f32;
    fn camera(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::TargetCameraData */>;
    fn camera_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::TargetCameraData */>;
    fn aiming_camera_height(&self) -> &f32;
    fn aiming_camera_height_mut(&mut self) -> &mut f32;
    fn strike_camera_height(&self) -> &f32;
    fn strike_camera_height_mut(&mut self) -> &mut f32;
    fn aiming_camera_offset(&self) -> &f32;
    fn aiming_camera_offset_mut(&mut self) -> &mut f32;
    fn strike_camera_offset(&self) -> &f32;
    fn strike_camera_offset_mut(&mut self) -> &mut f32;
    fn aiming_camera_fov(&self) -> &f32;
    fn aiming_camera_fov_mut(&mut self) -> &mut f32;
    fn delay_before_aiming_camera(&self) -> &f32;
    fn delay_before_aiming_camera_mut(&mut self) -> &mut f32;
    fn strike_camera_fov(&self) -> &f32;
    fn strike_camera_fov_mut(&mut self) -> &mut f32;
    fn fire_camera_time(&self) -> &f32;
    fn fire_camera_time_mut(&mut self) -> &mut f32;
    fn strike_camera_time(&self) -> &f32;
    fn strike_camera_time_mut(&mut self) -> &mut f32;
    fn valid_min_distance(&self) -> &f32;
    fn valid_min_distance_mut(&mut self) -> &mut f32;
    fn valid_max_distance(&self) -> &f32;
    fn valid_max_distance_mut(&mut self) -> &mut f32;
    fn valid_max_angle(&self) -> &f32;
    fn valid_max_angle_mut(&mut self) -> &mut f32;
    fn enable_projectile_trails(&self) -> &bool;
    fn enable_projectile_trails_mut(&mut self) -> &mut bool;
    fn enable_camera_rotation(&self) -> &bool;
    fn enable_camera_rotation_mut(&mut self) -> &mut bool;
    fn artillery_dispersion(&self) -> &ArtilleryDispersionData;
    fn artillery_dispersion_mut(&mut self) -> &mut ArtilleryDispersionData;
}

impl ArtilleryStrikeWeaponDataTrait for ArtilleryStrikeWeaponData {
    fn max_strike_distance(&self) -> &f32 {
        &self.max_strike_distance
    }
    fn max_strike_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_strike_distance
    }
    fn strike_radius(&self) -> &f32 {
        &self.strike_radius
    }
    fn strike_radius_mut(&mut self) -> &mut f32 {
        &mut self.strike_radius
    }
    fn spawn_height(&self) -> &f32 {
        &self.spawn_height
    }
    fn spawn_height_mut(&mut self) -> &mut f32 {
        &mut self.spawn_height
    }
    fn max_random_spawn_height(&self) -> &f32 {
        &self.max_random_spawn_height
    }
    fn max_random_spawn_height_mut(&mut self) -> &mut f32 {
        &mut self.max_random_spawn_height
    }
    fn increase_spawn_height_with_distance(&self) -> &bool {
        &self.increase_spawn_height_with_distance
    }
    fn increase_spawn_height_with_distance_mut(&mut self) -> &mut bool {
        &mut self.increase_spawn_height_with_distance
    }
    fn spawn_height_multiplier(&self) -> &f32 {
        &self.spawn_height_multiplier
    }
    fn spawn_height_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.spawn_height_multiplier
    }
    fn camera(&self) -> &Option<LockedTypeObject /* super::gameplay_sim::TargetCameraData */> {
        &self.camera
    }
    fn camera_mut(&mut self) -> &mut Option<LockedTypeObject /* super::gameplay_sim::TargetCameraData */> {
        &mut self.camera
    }
    fn aiming_camera_height(&self) -> &f32 {
        &self.aiming_camera_height
    }
    fn aiming_camera_height_mut(&mut self) -> &mut f32 {
        &mut self.aiming_camera_height
    }
    fn strike_camera_height(&self) -> &f32 {
        &self.strike_camera_height
    }
    fn strike_camera_height_mut(&mut self) -> &mut f32 {
        &mut self.strike_camera_height
    }
    fn aiming_camera_offset(&self) -> &f32 {
        &self.aiming_camera_offset
    }
    fn aiming_camera_offset_mut(&mut self) -> &mut f32 {
        &mut self.aiming_camera_offset
    }
    fn strike_camera_offset(&self) -> &f32 {
        &self.strike_camera_offset
    }
    fn strike_camera_offset_mut(&mut self) -> &mut f32 {
        &mut self.strike_camera_offset
    }
    fn aiming_camera_fov(&self) -> &f32 {
        &self.aiming_camera_fov
    }
    fn aiming_camera_fov_mut(&mut self) -> &mut f32 {
        &mut self.aiming_camera_fov
    }
    fn delay_before_aiming_camera(&self) -> &f32 {
        &self.delay_before_aiming_camera
    }
    fn delay_before_aiming_camera_mut(&mut self) -> &mut f32 {
        &mut self.delay_before_aiming_camera
    }
    fn strike_camera_fov(&self) -> &f32 {
        &self.strike_camera_fov
    }
    fn strike_camera_fov_mut(&mut self) -> &mut f32 {
        &mut self.strike_camera_fov
    }
    fn fire_camera_time(&self) -> &f32 {
        &self.fire_camera_time
    }
    fn fire_camera_time_mut(&mut self) -> &mut f32 {
        &mut self.fire_camera_time
    }
    fn strike_camera_time(&self) -> &f32 {
        &self.strike_camera_time
    }
    fn strike_camera_time_mut(&mut self) -> &mut f32 {
        &mut self.strike_camera_time
    }
    fn valid_min_distance(&self) -> &f32 {
        &self.valid_min_distance
    }
    fn valid_min_distance_mut(&mut self) -> &mut f32 {
        &mut self.valid_min_distance
    }
    fn valid_max_distance(&self) -> &f32 {
        &self.valid_max_distance
    }
    fn valid_max_distance_mut(&mut self) -> &mut f32 {
        &mut self.valid_max_distance
    }
    fn valid_max_angle(&self) -> &f32 {
        &self.valid_max_angle
    }
    fn valid_max_angle_mut(&mut self) -> &mut f32 {
        &mut self.valid_max_angle
    }
    fn enable_projectile_trails(&self) -> &bool {
        &self.enable_projectile_trails
    }
    fn enable_projectile_trails_mut(&mut self) -> &mut bool {
        &mut self.enable_projectile_trails
    }
    fn enable_camera_rotation(&self) -> &bool {
        &self.enable_camera_rotation
    }
    fn enable_camera_rotation_mut(&mut self) -> &mut bool {
        &mut self.enable_camera_rotation
    }
    fn artillery_dispersion(&self) -> &ArtilleryDispersionData {
        &self.artillery_dispersion
    }
    fn artillery_dispersion_mut(&mut self) -> &mut ArtilleryDispersionData {
        &mut self.artillery_dispersion
    }
}

impl WeaponDataTrait for ArtilleryStrikeWeaponData {
    fn show_laser_painted_vehicles(&self) -> &bool {
        self._glacier_base.show_laser_painted_vehicles()
    }
    fn show_laser_painted_vehicles_mut(&mut self) -> &mut bool {
        self._glacier_base.show_laser_painted_vehicles_mut()
    }
    fn apply_power_to_speed(&self) -> &bool {
        self._glacier_base.apply_power_to_speed()
    }
    fn apply_power_to_speed_mut(&mut self) -> &mut bool {
        self._glacier_base.apply_power_to_speed_mut()
    }
}

impl super::gameplay_sim::ToolDataTrait for ArtilleryStrikeWeaponData {
    fn is_always_active(&self) -> &bool {
        self._glacier_base.is_always_active()
    }
    fn is_always_active_mut(&mut self) -> &mut bool {
        self._glacier_base.is_always_active_mut()
    }
}

impl super::core::DataContainerTrait for ArtilleryStrikeWeaponData {
}

pub static ARTILLERYSTRIKEWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryStrikeWeaponData",
    name_hash: 1198720901,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDATA_TYPE_INFO),
        super_class_offset: offset_of!(ArtilleryStrikeWeaponData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ArtilleryStrikeWeaponData as Default>::default())),
            create_boxed: || Box::new(<ArtilleryStrikeWeaponData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxStrikeDistance",
                name_hash: 3419412896,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, max_strike_distance),
            },
            FieldInfoData {
                name: "StrikeRadius",
                name_hash: 4030357871,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_radius),
            },
            FieldInfoData {
                name: "SpawnHeight",
                name_hash: 2780846849,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, spawn_height),
            },
            FieldInfoData {
                name: "MaxRandomSpawnHeight",
                name_hash: 2881356526,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, max_random_spawn_height),
            },
            FieldInfoData {
                name: "IncreaseSpawnHeightWithDistance",
                name_hash: 398131684,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, increase_spawn_height_with_distance),
            },
            FieldInfoData {
                name: "SpawnHeightMultiplier",
                name_hash: 167841610,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, spawn_height_multiplier),
            },
            FieldInfoData {
                name: "Camera",
                name_hash: 2716359356,
                flags: MemberInfoFlags::new(0),
                field_type: "TargetCameraData",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, camera),
            },
            FieldInfoData {
                name: "AimingCameraHeight",
                name_hash: 1398477510,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, aiming_camera_height),
            },
            FieldInfoData {
                name: "StrikeCameraHeight",
                name_hash: 3381601297,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_height),
            },
            FieldInfoData {
                name: "AimingCameraOffset",
                name_hash: 1509617716,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, aiming_camera_offset),
            },
            FieldInfoData {
                name: "StrikeCameraOffset",
                name_hash: 2966290851,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_offset),
            },
            FieldInfoData {
                name: "AimingCameraFov",
                name_hash: 2378493510,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, aiming_camera_fov),
            },
            FieldInfoData {
                name: "DelayBeforeAimingCamera",
                name_hash: 619983381,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, delay_before_aiming_camera),
            },
            FieldInfoData {
                name: "StrikeCameraFov",
                name_hash: 441695729,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_fov),
            },
            FieldInfoData {
                name: "FireCameraTime",
                name_hash: 3536152465,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, fire_camera_time),
            },
            FieldInfoData {
                name: "StrikeCameraTime",
                name_hash: 1691832603,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, strike_camera_time),
            },
            FieldInfoData {
                name: "ValidMinDistance",
                name_hash: 3318112506,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, valid_min_distance),
            },
            FieldInfoData {
                name: "ValidMaxDistance",
                name_hash: 1927644132,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, valid_max_distance),
            },
            FieldInfoData {
                name: "ValidMaxAngle",
                name_hash: 815739846,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, valid_max_angle),
            },
            FieldInfoData {
                name: "EnableProjectileTrails",
                name_hash: 2901124096,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, enable_projectile_trails),
            },
            FieldInfoData {
                name: "EnableCameraRotation",
                name_hash: 3948441033,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, enable_camera_rotation),
            },
            FieldInfoData {
                name: "ArtilleryDispersion",
                name_hash: 1373489959,
                flags: MemberInfoFlags::new(0),
                field_type: "ArtilleryDispersionData",
                rust_offset: offset_of!(ArtilleryStrikeWeaponData, artillery_dispersion),
            },
        ],
    }),
    array_type: Some(ARTILLERYSTRIKEWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ArtilleryStrikeWeaponData {
    fn type_info(&self) -> &'static TypeInfo {
        ARTILLERYSTRIKEWEAPONDATA_TYPE_INFO
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


pub static ARTILLERYSTRIKEWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryStrikeWeaponData-Array",
    name_hash: 1747177265,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ArtilleryStrikeWeaponData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ArtilleryDispersionData {
    pub dispersion_active: bool,
    pub max_dispersion: f32,
    pub dispersion_aim_move: f32,
    pub dispersion_aim_move_threshold: f32,
    pub dispersion_firing: f32,
    pub dispersion_deployment: f32,
    pub decrease_per_second: f32,
}

pub trait ArtilleryDispersionDataTrait: TypeObject {
    fn dispersion_active(&self) -> &bool;
    fn dispersion_active_mut(&mut self) -> &mut bool;
    fn max_dispersion(&self) -> &f32;
    fn max_dispersion_mut(&mut self) -> &mut f32;
    fn dispersion_aim_move(&self) -> &f32;
    fn dispersion_aim_move_mut(&mut self) -> &mut f32;
    fn dispersion_aim_move_threshold(&self) -> &f32;
    fn dispersion_aim_move_threshold_mut(&mut self) -> &mut f32;
    fn dispersion_firing(&self) -> &f32;
    fn dispersion_firing_mut(&mut self) -> &mut f32;
    fn dispersion_deployment(&self) -> &f32;
    fn dispersion_deployment_mut(&mut self) -> &mut f32;
    fn decrease_per_second(&self) -> &f32;
    fn decrease_per_second_mut(&mut self) -> &mut f32;
}

impl ArtilleryDispersionDataTrait for ArtilleryDispersionData {
    fn dispersion_active(&self) -> &bool {
        &self.dispersion_active
    }
    fn dispersion_active_mut(&mut self) -> &mut bool {
        &mut self.dispersion_active
    }
    fn max_dispersion(&self) -> &f32 {
        &self.max_dispersion
    }
    fn max_dispersion_mut(&mut self) -> &mut f32 {
        &mut self.max_dispersion
    }
    fn dispersion_aim_move(&self) -> &f32 {
        &self.dispersion_aim_move
    }
    fn dispersion_aim_move_mut(&mut self) -> &mut f32 {
        &mut self.dispersion_aim_move
    }
    fn dispersion_aim_move_threshold(&self) -> &f32 {
        &self.dispersion_aim_move_threshold
    }
    fn dispersion_aim_move_threshold_mut(&mut self) -> &mut f32 {
        &mut self.dispersion_aim_move_threshold
    }
    fn dispersion_firing(&self) -> &f32 {
        &self.dispersion_firing
    }
    fn dispersion_firing_mut(&mut self) -> &mut f32 {
        &mut self.dispersion_firing
    }
    fn dispersion_deployment(&self) -> &f32 {
        &self.dispersion_deployment
    }
    fn dispersion_deployment_mut(&mut self) -> &mut f32 {
        &mut self.dispersion_deployment
    }
    fn decrease_per_second(&self) -> &f32 {
        &self.decrease_per_second
    }
    fn decrease_per_second_mut(&mut self) -> &mut f32 {
        &mut self.decrease_per_second
    }
}

pub static ARTILLERYDISPERSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryDispersionData",
    name_hash: 1420722263,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ArtilleryDispersionData as Default>::default())),
            create_boxed: || Box::new(<ArtilleryDispersionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DispersionActive",
                name_hash: 1652102635,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_active),
            },
            FieldInfoData {
                name: "MaxDispersion",
                name_hash: 3364316851,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryDispersionData, max_dispersion),
            },
            FieldInfoData {
                name: "DispersionAimMove",
                name_hash: 3496872435,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_aim_move),
            },
            FieldInfoData {
                name: "DispersionAimMoveThreshold",
                name_hash: 697473060,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_aim_move_threshold),
            },
            FieldInfoData {
                name: "DispersionFiring",
                name_hash: 1541939034,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_firing),
            },
            FieldInfoData {
                name: "DispersionDeployment",
                name_hash: 583542238,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryDispersionData, dispersion_deployment),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                name_hash: 3208554608,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ArtilleryDispersionData, decrease_per_second),
            },
        ],
    }),
    array_type: Some(ARTILLERYDISPERSIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ArtilleryDispersionData {
    fn type_info(&self) -> &'static TypeInfo {
        ARTILLERYDISPERSIONDATA_TYPE_INFO
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


pub static ARTILLERYDISPERSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ArtilleryDispersionData-Array",
    name_hash: 2655115619,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ArtilleryDispersionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MortarStrikeWeaponData {
    pub _glacier_base: WeaponData,
    pub holding_tolerance: f32,
    pub max_strike_distance: f32,
    pub strike_radius: f32,
    pub max_random_spawn_height: f32,
}

pub trait MortarStrikeWeaponDataTrait: WeaponDataTrait {
    fn holding_tolerance(&self) -> &f32;
    fn holding_tolerance_mut(&mut self) -> &mut f32;
    fn max_strike_distance(&self) -> &f32;
    fn max_strike_distance_mut(&mut self) -> &mut f32;
    fn strike_radius(&self) -> &f32;
    fn strike_radius_mut(&mut self) -> &mut f32;
    fn max_random_spawn_height(&self) -> &f32;
    fn max_random_spawn_height_mut(&mut self) -> &mut f32;
}

impl MortarStrikeWeaponDataTrait for MortarStrikeWeaponData {
    fn holding_tolerance(&self) -> &f32 {
        &self.holding_tolerance
    }
    fn holding_tolerance_mut(&mut self) -> &mut f32 {
        &mut self.holding_tolerance
    }
    fn max_strike_distance(&self) -> &f32 {
        &self.max_strike_distance
    }
    fn max_strike_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_strike_distance
    }
    fn strike_radius(&self) -> &f32 {
        &self.strike_radius
    }
    fn strike_radius_mut(&mut self) -> &mut f32 {
        &mut self.strike_radius
    }
    fn max_random_spawn_height(&self) -> &f32 {
        &self.max_random_spawn_height
    }
    fn max_random_spawn_height_mut(&mut self) -> &mut f32 {
        &mut self.max_random_spawn_height
    }
}

impl WeaponDataTrait for MortarStrikeWeaponData {
    fn show_laser_painted_vehicles(&self) -> &bool {
        self._glacier_base.show_laser_painted_vehicles()
    }
    fn show_laser_painted_vehicles_mut(&mut self) -> &mut bool {
        self._glacier_base.show_laser_painted_vehicles_mut()
    }
    fn apply_power_to_speed(&self) -> &bool {
        self._glacier_base.apply_power_to_speed()
    }
    fn apply_power_to_speed_mut(&mut self) -> &mut bool {
        self._glacier_base.apply_power_to_speed_mut()
    }
}

impl super::gameplay_sim::ToolDataTrait for MortarStrikeWeaponData {
    fn is_always_active(&self) -> &bool {
        self._glacier_base.is_always_active()
    }
    fn is_always_active_mut(&mut self) -> &mut bool {
        self._glacier_base.is_always_active_mut()
    }
}

impl super::core::DataContainerTrait for MortarStrikeWeaponData {
}

pub static MORTARSTRIKEWEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MortarStrikeWeaponData",
    name_hash: 3312514290,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WEAPONDATA_TYPE_INFO),
        super_class_offset: offset_of!(MortarStrikeWeaponData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MortarStrikeWeaponData as Default>::default())),
            create_boxed: || Box::new(<MortarStrikeWeaponData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HoldingTolerance",
                name_hash: 1278440067,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MortarStrikeWeaponData, holding_tolerance),
            },
            FieldInfoData {
                name: "MaxStrikeDistance",
                name_hash: 3419412896,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MortarStrikeWeaponData, max_strike_distance),
            },
            FieldInfoData {
                name: "StrikeRadius",
                name_hash: 4030357871,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MortarStrikeWeaponData, strike_radius),
            },
            FieldInfoData {
                name: "MaxRandomSpawnHeight",
                name_hash: 2881356526,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MortarStrikeWeaponData, max_random_spawn_height),
            },
        ],
    }),
    array_type: Some(MORTARSTRIKEWEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MortarStrikeWeaponData {
    fn type_info(&self) -> &'static TypeInfo {
        MORTARSTRIKEWEAPONDATA_TYPE_INFO
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


pub static MORTARSTRIKEWEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MortarStrikeWeaponData-Array",
    name_hash: 784239558,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MortarStrikeWeaponData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponData {
    pub _glacier_base: super::gameplay_sim::ToolData,
    pub show_laser_painted_vehicles: bool,
    pub apply_power_to_speed: bool,
}

pub trait WeaponDataTrait: super::gameplay_sim::ToolDataTrait {
    fn show_laser_painted_vehicles(&self) -> &bool;
    fn show_laser_painted_vehicles_mut(&mut self) -> &mut bool;
    fn apply_power_to_speed(&self) -> &bool;
    fn apply_power_to_speed_mut(&mut self) -> &mut bool;
}

impl WeaponDataTrait for WeaponData {
    fn show_laser_painted_vehicles(&self) -> &bool {
        &self.show_laser_painted_vehicles
    }
    fn show_laser_painted_vehicles_mut(&mut self) -> &mut bool {
        &mut self.show_laser_painted_vehicles
    }
    fn apply_power_to_speed(&self) -> &bool {
        &self.apply_power_to_speed
    }
    fn apply_power_to_speed_mut(&mut self) -> &mut bool {
        &mut self.apply_power_to_speed
    }
}

impl super::gameplay_sim::ToolDataTrait for WeaponData {
    fn is_always_active(&self) -> &bool {
        self._glacier_base.is_always_active()
    }
    fn is_always_active_mut(&mut self) -> &mut bool {
        self._glacier_base.is_always_active_mut()
    }
}

impl super::core::DataContainerTrait for WeaponData {
}

pub static WEAPONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponData",
    name_hash: 932650231,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::TOOLDATA_TYPE_INFO),
        super_class_offset: offset_of!(WeaponData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponData as Default>::default())),
            create_boxed: || Box::new(<WeaponData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShowLaserPaintedVehicles",
                name_hash: 3679743847,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponData, show_laser_painted_vehicles),
            },
            FieldInfoData {
                name: "ApplyPowerToSpeed",
                name_hash: 3422010098,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponData, apply_power_to_speed),
            },
        ],
    }),
    array_type: Some(WEAPONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONDATA_TYPE_INFO
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


pub static WEAPONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponData-Array",
    name_hash: 1192093379,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponFiringDataAsset {
    pub _glacier_base: super::entity::GameDataContainerAsset,
}

pub trait WeaponFiringDataAssetTrait: super::entity::GameDataContainerAssetTrait {
}

impl WeaponFiringDataAssetTrait for WeaponFiringDataAsset {
}

impl super::entity::GameDataContainerAssetTrait for WeaponFiringDataAsset {
    fn data(&self) -> &Option<LockedTypeObject /* super::core::GameDataContainer */> {
        self._glacier_base.data()
    }
    fn data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::GameDataContainer */> {
        self._glacier_base.data_mut()
    }
}

impl super::core::AssetTrait for WeaponFiringDataAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for WeaponFiringDataAsset {
}

pub static WEAPONFIRINGDATAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataAsset",
    name_hash: 3421492762,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMEDATACONTAINERASSET_TYPE_INFO),
        super_class_offset: offset_of!(WeaponFiringDataAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFiringDataAsset as Default>::default())),
            create_boxed: || Box::new(<WeaponFiringDataAsset as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONFIRINGDATAASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringDataAsset {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRINGDATAASSET_TYPE_INFO
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


pub static WEAPONFIRINGDATAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringDataAsset-Array",
    name_hash: 39147822,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringDataAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponFiringData {
    pub _glacier_base: super::core::GameDataContainer,
    pub primary_fire: Option<LockedTypeObject /* FiringFunctionData */>,
    pub secondary_fire: SecondaryFireData,
    pub tertiary_fire: TertiaryFireData,
    pub deploy_time: f32,
    pub reactivate_cooldown_time: f32,
    pub disable_zoom_on_deploy_time: f32,
    pub alt_deploy_time: f32,
    pub alt_deploy_id: i32,
    pub use_auto_aiming: bool,
    pub weapon_sway: Option<LockedTypeObject /* WeaponSwayData */>,
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

pub trait WeaponFiringDataTrait: super::core::GameDataContainerTrait {
    fn primary_fire(&self) -> &Option<LockedTypeObject /* FiringFunctionData */>;
    fn primary_fire_mut(&mut self) -> &mut Option<LockedTypeObject /* FiringFunctionData */>;
    fn secondary_fire(&self) -> &SecondaryFireData;
    fn secondary_fire_mut(&mut self) -> &mut SecondaryFireData;
    fn tertiary_fire(&self) -> &TertiaryFireData;
    fn tertiary_fire_mut(&mut self) -> &mut TertiaryFireData;
    fn deploy_time(&self) -> &f32;
    fn deploy_time_mut(&mut self) -> &mut f32;
    fn reactivate_cooldown_time(&self) -> &f32;
    fn reactivate_cooldown_time_mut(&mut self) -> &mut f32;
    fn disable_zoom_on_deploy_time(&self) -> &f32;
    fn disable_zoom_on_deploy_time_mut(&mut self) -> &mut f32;
    fn alt_deploy_time(&self) -> &f32;
    fn alt_deploy_time_mut(&mut self) -> &mut f32;
    fn alt_deploy_id(&self) -> &i32;
    fn alt_deploy_id_mut(&mut self) -> &mut i32;
    fn use_auto_aiming(&self) -> &bool;
    fn use_auto_aiming_mut(&mut self) -> &mut bool;
    fn weapon_sway(&self) -> &Option<LockedTypeObject /* WeaponSwayData */>;
    fn weapon_sway_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSwayData */>;
    fn inflict_self_damage(&self) -> &bool;
    fn inflict_self_damage_mut(&mut self) -> &mut bool;
    fn rumble(&self) -> &RumbleFiringData;
    fn rumble_mut(&mut self) -> &mut RumbleFiringData;
    fn support_delay_stand(&self) -> &f32;
    fn support_delay_stand_mut(&mut self) -> &mut f32;
    fn support_delay_prone(&self) -> &f32;
    fn support_delay_prone_mut(&mut self) -> &mut f32;
    fn show_enemy_nametag_on_aim(&self) -> &bool;
    fn show_enemy_nametag_on_aim_mut(&mut self) -> &mut bool;
    fn reload_whole_mags(&self) -> &bool;
    fn reload_whole_mags_mut(&mut self) -> &mut bool;
    fn disable_reload_while_sprinting(&self) -> &bool;
    fn disable_reload_while_sprinting_mut(&mut self) -> &mut bool;
    fn abort_reload_on_sprint(&self) -> &bool;
    fn abort_reload_on_sprint_mut(&mut self) -> &mut bool;
    fn use_remote_damage_giver_info(&self) -> &bool;
    fn use_remote_damage_giver_info_mut(&mut self) -> &mut bool;
}

impl WeaponFiringDataTrait for WeaponFiringData {
    fn primary_fire(&self) -> &Option<LockedTypeObject /* FiringFunctionData */> {
        &self.primary_fire
    }
    fn primary_fire_mut(&mut self) -> &mut Option<LockedTypeObject /* FiringFunctionData */> {
        &mut self.primary_fire
    }
    fn secondary_fire(&self) -> &SecondaryFireData {
        &self.secondary_fire
    }
    fn secondary_fire_mut(&mut self) -> &mut SecondaryFireData {
        &mut self.secondary_fire
    }
    fn tertiary_fire(&self) -> &TertiaryFireData {
        &self.tertiary_fire
    }
    fn tertiary_fire_mut(&mut self) -> &mut TertiaryFireData {
        &mut self.tertiary_fire
    }
    fn deploy_time(&self) -> &f32 {
        &self.deploy_time
    }
    fn deploy_time_mut(&mut self) -> &mut f32 {
        &mut self.deploy_time
    }
    fn reactivate_cooldown_time(&self) -> &f32 {
        &self.reactivate_cooldown_time
    }
    fn reactivate_cooldown_time_mut(&mut self) -> &mut f32 {
        &mut self.reactivate_cooldown_time
    }
    fn disable_zoom_on_deploy_time(&self) -> &f32 {
        &self.disable_zoom_on_deploy_time
    }
    fn disable_zoom_on_deploy_time_mut(&mut self) -> &mut f32 {
        &mut self.disable_zoom_on_deploy_time
    }
    fn alt_deploy_time(&self) -> &f32 {
        &self.alt_deploy_time
    }
    fn alt_deploy_time_mut(&mut self) -> &mut f32 {
        &mut self.alt_deploy_time
    }
    fn alt_deploy_id(&self) -> &i32 {
        &self.alt_deploy_id
    }
    fn alt_deploy_id_mut(&mut self) -> &mut i32 {
        &mut self.alt_deploy_id
    }
    fn use_auto_aiming(&self) -> &bool {
        &self.use_auto_aiming
    }
    fn use_auto_aiming_mut(&mut self) -> &mut bool {
        &mut self.use_auto_aiming
    }
    fn weapon_sway(&self) -> &Option<LockedTypeObject /* WeaponSwayData */> {
        &self.weapon_sway
    }
    fn weapon_sway_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSwayData */> {
        &mut self.weapon_sway
    }
    fn inflict_self_damage(&self) -> &bool {
        &self.inflict_self_damage
    }
    fn inflict_self_damage_mut(&mut self) -> &mut bool {
        &mut self.inflict_self_damage
    }
    fn rumble(&self) -> &RumbleFiringData {
        &self.rumble
    }
    fn rumble_mut(&mut self) -> &mut RumbleFiringData {
        &mut self.rumble
    }
    fn support_delay_stand(&self) -> &f32 {
        &self.support_delay_stand
    }
    fn support_delay_stand_mut(&mut self) -> &mut f32 {
        &mut self.support_delay_stand
    }
    fn support_delay_prone(&self) -> &f32 {
        &self.support_delay_prone
    }
    fn support_delay_prone_mut(&mut self) -> &mut f32 {
        &mut self.support_delay_prone
    }
    fn show_enemy_nametag_on_aim(&self) -> &bool {
        &self.show_enemy_nametag_on_aim
    }
    fn show_enemy_nametag_on_aim_mut(&mut self) -> &mut bool {
        &mut self.show_enemy_nametag_on_aim
    }
    fn reload_whole_mags(&self) -> &bool {
        &self.reload_whole_mags
    }
    fn reload_whole_mags_mut(&mut self) -> &mut bool {
        &mut self.reload_whole_mags
    }
    fn disable_reload_while_sprinting(&self) -> &bool {
        &self.disable_reload_while_sprinting
    }
    fn disable_reload_while_sprinting_mut(&mut self) -> &mut bool {
        &mut self.disable_reload_while_sprinting
    }
    fn abort_reload_on_sprint(&self) -> &bool {
        &self.abort_reload_on_sprint
    }
    fn abort_reload_on_sprint_mut(&mut self) -> &mut bool {
        &mut self.abort_reload_on_sprint
    }
    fn use_remote_damage_giver_info(&self) -> &bool {
        &self.use_remote_damage_giver_info
    }
    fn use_remote_damage_giver_info_mut(&mut self) -> &mut bool {
        &mut self.use_remote_damage_giver_info
    }
}

impl super::core::GameDataContainerTrait for WeaponFiringData {
}

impl super::core::DataContainerTrait for WeaponFiringData {
}

pub static WEAPONFIRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringData",
    name_hash: 3628051978,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::GAMEDATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponFiringData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFiringData as Default>::default())),
            create_boxed: || Box::new(<WeaponFiringData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PrimaryFire",
                name_hash: 1894912945,
                flags: MemberInfoFlags::new(0),
                field_type: "FiringFunctionData",
                rust_offset: offset_of!(WeaponFiringData, primary_fire),
            },
            FieldInfoData {
                name: "SecondaryFire",
                name_hash: 367243079,
                flags: MemberInfoFlags::new(0),
                field_type: "SecondaryFireData",
                rust_offset: offset_of!(WeaponFiringData, secondary_fire),
            },
            FieldInfoData {
                name: "TertiaryFire",
                name_hash: 3060723369,
                flags: MemberInfoFlags::new(0),
                field_type: "TertiaryFireData",
                rust_offset: offset_of!(WeaponFiringData, tertiary_fire),
            },
            FieldInfoData {
                name: "DeployTime",
                name_hash: 2275884507,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFiringData, deploy_time),
            },
            FieldInfoData {
                name: "ReactivateCooldownTime",
                name_hash: 732426371,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFiringData, reactivate_cooldown_time),
            },
            FieldInfoData {
                name: "DisableZoomOnDeployTime",
                name_hash: 3731232633,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFiringData, disable_zoom_on_deploy_time),
            },
            FieldInfoData {
                name: "AltDeployTime",
                name_hash: 1924470306,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFiringData, alt_deploy_time),
            },
            FieldInfoData {
                name: "AltDeployId",
                name_hash: 1819931514,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponFiringData, alt_deploy_id),
            },
            FieldInfoData {
                name: "UseAutoAiming",
                name_hash: 2972101868,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringData, use_auto_aiming),
            },
            FieldInfoData {
                name: "WeaponSway",
                name_hash: 933173371,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponSwayData",
                rust_offset: offset_of!(WeaponFiringData, weapon_sway),
            },
            FieldInfoData {
                name: "InflictSelfDamage",
                name_hash: 4046344033,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringData, inflict_self_damage),
            },
            FieldInfoData {
                name: "Rumble",
                name_hash: 3275215748,
                flags: MemberInfoFlags::new(0),
                field_type: "RumbleFiringData",
                rust_offset: offset_of!(WeaponFiringData, rumble),
            },
            FieldInfoData {
                name: "SupportDelayStand",
                name_hash: 3920813299,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFiringData, support_delay_stand),
            },
            FieldInfoData {
                name: "SupportDelayProne",
                name_hash: 3926871609,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFiringData, support_delay_prone),
            },
            FieldInfoData {
                name: "ShowEnemyNametagOnAim",
                name_hash: 64653709,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringData, show_enemy_nametag_on_aim),
            },
            FieldInfoData {
                name: "ReloadWholeMags",
                name_hash: 3901798901,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringData, reload_whole_mags),
            },
            FieldInfoData {
                name: "DisableReloadWhileSprinting",
                name_hash: 2335508285,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringData, disable_reload_while_sprinting),
            },
            FieldInfoData {
                name: "AbortReloadOnSprint",
                name_hash: 923004157,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringData, abort_reload_on_sprint),
            },
            FieldInfoData {
                name: "UseRemoteDamageGiverInfo",
                name_hash: 39035080,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringData, use_remote_damage_giver_info),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponFiringData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRINGDATA_TYPE_INFO
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


pub static WEAPONFIRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringData-Array",
    name_hash: 563374398,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TertiaryFireData {
    pub wait_on_trigger_release: bool,
    pub wait_on_primary_trigger_release: bool,
    pub firing_delay: f32,
    pub pending_fire_window: f32,
    pub pending_fire_ignore_fire_rate_window: f32,
}

pub trait TertiaryFireDataTrait: TypeObject {
    fn wait_on_trigger_release(&self) -> &bool;
    fn wait_on_trigger_release_mut(&mut self) -> &mut bool;
    fn wait_on_primary_trigger_release(&self) -> &bool;
    fn wait_on_primary_trigger_release_mut(&mut self) -> &mut bool;
    fn firing_delay(&self) -> &f32;
    fn firing_delay_mut(&mut self) -> &mut f32;
    fn pending_fire_window(&self) -> &f32;
    fn pending_fire_window_mut(&mut self) -> &mut f32;
    fn pending_fire_ignore_fire_rate_window(&self) -> &f32;
    fn pending_fire_ignore_fire_rate_window_mut(&mut self) -> &mut f32;
}

impl TertiaryFireDataTrait for TertiaryFireData {
    fn wait_on_trigger_release(&self) -> &bool {
        &self.wait_on_trigger_release
    }
    fn wait_on_trigger_release_mut(&mut self) -> &mut bool {
        &mut self.wait_on_trigger_release
    }
    fn wait_on_primary_trigger_release(&self) -> &bool {
        &self.wait_on_primary_trigger_release
    }
    fn wait_on_primary_trigger_release_mut(&mut self) -> &mut bool {
        &mut self.wait_on_primary_trigger_release
    }
    fn firing_delay(&self) -> &f32 {
        &self.firing_delay
    }
    fn firing_delay_mut(&mut self) -> &mut f32 {
        &mut self.firing_delay
    }
    fn pending_fire_window(&self) -> &f32 {
        &self.pending_fire_window
    }
    fn pending_fire_window_mut(&mut self) -> &mut f32 {
        &mut self.pending_fire_window
    }
    fn pending_fire_ignore_fire_rate_window(&self) -> &f32 {
        &self.pending_fire_ignore_fire_rate_window
    }
    fn pending_fire_ignore_fire_rate_window_mut(&mut self) -> &mut f32 {
        &mut self.pending_fire_ignore_fire_rate_window
    }
}

pub static TERTIARYFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TertiaryFireData",
    name_hash: 470267353,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TertiaryFireData as Default>::default())),
            create_boxed: || Box::new(<TertiaryFireData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WaitOnTriggerRelease",
                name_hash: 2887852830,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TertiaryFireData, wait_on_trigger_release),
            },
            FieldInfoData {
                name: "WaitOnPrimaryTriggerRelease",
                name_hash: 1308144914,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TertiaryFireData, wait_on_primary_trigger_release),
            },
            FieldInfoData {
                name: "FiringDelay",
                name_hash: 1661067117,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TertiaryFireData, firing_delay),
            },
            FieldInfoData {
                name: "PendingFireWindow",
                name_hash: 3361504206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TertiaryFireData, pending_fire_window),
            },
            FieldInfoData {
                name: "PendingFireIgnoreFireRateWindow",
                name_hash: 1667285324,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TertiaryFireData, pending_fire_ignore_fire_rate_window),
            },
        ],
    }),
    array_type: Some(TERTIARYFIREDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TertiaryFireData {
    fn type_info(&self) -> &'static TypeInfo {
        TERTIARYFIREDATA_TYPE_INFO
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


pub static TERTIARYFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TertiaryFireData-Array",
    name_hash: 2543887341,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("TertiaryFireData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SecondaryFireData {
    pub wait_on_trigger_release: bool,
    pub wait_on_primary_trigger_release: bool,
    pub firing_delay: f32,
    pub pending_fire_window: f32,
    pub pending_fire_ignore_fire_rate_window: f32,
}

pub trait SecondaryFireDataTrait: TypeObject {
    fn wait_on_trigger_release(&self) -> &bool;
    fn wait_on_trigger_release_mut(&mut self) -> &mut bool;
    fn wait_on_primary_trigger_release(&self) -> &bool;
    fn wait_on_primary_trigger_release_mut(&mut self) -> &mut bool;
    fn firing_delay(&self) -> &f32;
    fn firing_delay_mut(&mut self) -> &mut f32;
    fn pending_fire_window(&self) -> &f32;
    fn pending_fire_window_mut(&mut self) -> &mut f32;
    fn pending_fire_ignore_fire_rate_window(&self) -> &f32;
    fn pending_fire_ignore_fire_rate_window_mut(&mut self) -> &mut f32;
}

impl SecondaryFireDataTrait for SecondaryFireData {
    fn wait_on_trigger_release(&self) -> &bool {
        &self.wait_on_trigger_release
    }
    fn wait_on_trigger_release_mut(&mut self) -> &mut bool {
        &mut self.wait_on_trigger_release
    }
    fn wait_on_primary_trigger_release(&self) -> &bool {
        &self.wait_on_primary_trigger_release
    }
    fn wait_on_primary_trigger_release_mut(&mut self) -> &mut bool {
        &mut self.wait_on_primary_trigger_release
    }
    fn firing_delay(&self) -> &f32 {
        &self.firing_delay
    }
    fn firing_delay_mut(&mut self) -> &mut f32 {
        &mut self.firing_delay
    }
    fn pending_fire_window(&self) -> &f32 {
        &self.pending_fire_window
    }
    fn pending_fire_window_mut(&mut self) -> &mut f32 {
        &mut self.pending_fire_window
    }
    fn pending_fire_ignore_fire_rate_window(&self) -> &f32 {
        &self.pending_fire_ignore_fire_rate_window
    }
    fn pending_fire_ignore_fire_rate_window_mut(&mut self) -> &mut f32 {
        &mut self.pending_fire_ignore_fire_rate_window
    }
}

pub static SECONDARYFIREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SecondaryFireData",
    name_hash: 3007859511,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SecondaryFireData as Default>::default())),
            create_boxed: || Box::new(<SecondaryFireData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WaitOnTriggerRelease",
                name_hash: 2887852830,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SecondaryFireData, wait_on_trigger_release),
            },
            FieldInfoData {
                name: "WaitOnPrimaryTriggerRelease",
                name_hash: 1308144914,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SecondaryFireData, wait_on_primary_trigger_release),
            },
            FieldInfoData {
                name: "FiringDelay",
                name_hash: 1661067117,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SecondaryFireData, firing_delay),
            },
            FieldInfoData {
                name: "PendingFireWindow",
                name_hash: 3361504206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SecondaryFireData, pending_fire_window),
            },
            FieldInfoData {
                name: "PendingFireIgnoreFireRateWindow",
                name_hash: 1667285324,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SecondaryFireData, pending_fire_ignore_fire_rate_window),
            },
        ],
    }),
    array_type: Some(SECONDARYFIREDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SecondaryFireData {
    fn type_info(&self) -> &'static TypeInfo {
        SECONDARYFIREDATA_TYPE_INFO
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


pub static SECONDARYFIREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SecondaryFireData-Array",
    name_hash: 808764547,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("SecondaryFireData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponFiringRareNetStateData {
    pub is_detonator_active: bool,
    pub fire_logic_type: FireLogicType,
}

pub trait WeaponFiringRareNetStateDataTrait: TypeObject {
    fn is_detonator_active(&self) -> &bool;
    fn is_detonator_active_mut(&mut self) -> &mut bool;
    fn fire_logic_type(&self) -> &FireLogicType;
    fn fire_logic_type_mut(&mut self) -> &mut FireLogicType;
}

impl WeaponFiringRareNetStateDataTrait for WeaponFiringRareNetStateData {
    fn is_detonator_active(&self) -> &bool {
        &self.is_detonator_active
    }
    fn is_detonator_active_mut(&mut self) -> &mut bool {
        &mut self.is_detonator_active
    }
    fn fire_logic_type(&self) -> &FireLogicType {
        &self.fire_logic_type
    }
    fn fire_logic_type_mut(&mut self) -> &mut FireLogicType {
        &mut self.fire_logic_type
    }
}

pub static WEAPONFIRINGRARENETSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringRareNetStateData",
    name_hash: 505399398,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFiringRareNetStateData as Default>::default())),
            create_boxed: || Box::new(<WeaponFiringRareNetStateData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsDetonatorActive",
                name_hash: 1971743151,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringRareNetStateData, is_detonator_active),
            },
            FieldInfoData {
                name: "FireLogicType",
                name_hash: 1365968171,
                flags: MemberInfoFlags::new(0),
                field_type: "FireLogicType",
                rust_offset: offset_of!(WeaponFiringRareNetStateData, fire_logic_type),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGRARENETSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WeaponFiringRareNetStateData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRINGRARENETSTATEDATA_TYPE_INFO
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


pub static WEAPONFIRINGRARENETSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringRareNetStateData-Array",
    name_hash: 3771393362,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringRareNetStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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

pub trait WeaponFiringNetStateDataTrait: TypeObject {
    fn write_weapon_state(&self) -> &WeaponNetworkState;
    fn write_weapon_state_mut(&mut self) -> &mut WeaponNetworkState;
    fn fired_count(&self) -> &u32;
    fn fired_count_mut(&mut self) -> &mut u32;
    fn tick_bits(&self) -> &u32;
    fn tick_bits_mut(&mut self) -> &mut u32;
    fn primary_heat(&self) -> &f32;
    fn primary_heat_mut(&mut self) -> &mut f32;
    fn reload_stage(&self) -> &u32;
    fn reload_stage_mut(&mut self) -> &mut u32;
    fn charge_up(&self) -> &u32;
    fn charge_up_mut(&mut self) -> &mut u32;
    fn in_pre_fire(&self) -> &bool;
    fn in_pre_fire_mut(&mut self) -> &mut bool;
    fn primary_is_overheated(&self) -> &bool;
    fn primary_is_overheated_mut(&mut self) -> &mut bool;
    fn fired_hold_and_release_modifier(&self) -> &i32;
    fn fired_hold_and_release_modifier_mut(&mut self) -> &mut i32;
}

impl WeaponFiringNetStateDataTrait for WeaponFiringNetStateData {
    fn write_weapon_state(&self) -> &WeaponNetworkState {
        &self.write_weapon_state
    }
    fn write_weapon_state_mut(&mut self) -> &mut WeaponNetworkState {
        &mut self.write_weapon_state
    }
    fn fired_count(&self) -> &u32 {
        &self.fired_count
    }
    fn fired_count_mut(&mut self) -> &mut u32 {
        &mut self.fired_count
    }
    fn tick_bits(&self) -> &u32 {
        &self.tick_bits
    }
    fn tick_bits_mut(&mut self) -> &mut u32 {
        &mut self.tick_bits
    }
    fn primary_heat(&self) -> &f32 {
        &self.primary_heat
    }
    fn primary_heat_mut(&mut self) -> &mut f32 {
        &mut self.primary_heat
    }
    fn reload_stage(&self) -> &u32 {
        &self.reload_stage
    }
    fn reload_stage_mut(&mut self) -> &mut u32 {
        &mut self.reload_stage
    }
    fn charge_up(&self) -> &u32 {
        &self.charge_up
    }
    fn charge_up_mut(&mut self) -> &mut u32 {
        &mut self.charge_up
    }
    fn in_pre_fire(&self) -> &bool {
        &self.in_pre_fire
    }
    fn in_pre_fire_mut(&mut self) -> &mut bool {
        &mut self.in_pre_fire
    }
    fn primary_is_overheated(&self) -> &bool {
        &self.primary_is_overheated
    }
    fn primary_is_overheated_mut(&mut self) -> &mut bool {
        &mut self.primary_is_overheated
    }
    fn fired_hold_and_release_modifier(&self) -> &i32 {
        &self.fired_hold_and_release_modifier
    }
    fn fired_hold_and_release_modifier_mut(&mut self) -> &mut i32 {
        &mut self.fired_hold_and_release_modifier
    }
}

pub static WEAPONFIRINGNETSTATEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringNetStateData",
    name_hash: 3215523650,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponFiringNetStateData as Default>::default())),
            create_boxed: || Box::new(<WeaponFiringNetStateData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WriteWeaponState",
                name_hash: 885040877,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponNetworkState",
                rust_offset: offset_of!(WeaponFiringNetStateData, write_weapon_state),
            },
            FieldInfoData {
                name: "FiredCount",
                name_hash: 1562358202,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WeaponFiringNetStateData, fired_count),
            },
            FieldInfoData {
                name: "TickBits",
                name_hash: 3079854204,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WeaponFiringNetStateData, tick_bits),
            },
            FieldInfoData {
                name: "PrimaryHeat",
                name_hash: 1894283025,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponFiringNetStateData, primary_heat),
            },
            FieldInfoData {
                name: "ReloadStage",
                name_hash: 2940481456,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WeaponFiringNetStateData, reload_stage),
            },
            FieldInfoData {
                name: "ChargeUp",
                name_hash: 1617805722,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WeaponFiringNetStateData, charge_up),
            },
            FieldInfoData {
                name: "InPreFire",
                name_hash: 2353757885,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringNetStateData, in_pre_fire),
            },
            FieldInfoData {
                name: "PrimaryIsOverheated",
                name_hash: 3772723940,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WeaponFiringNetStateData, primary_is_overheated),
            },
            FieldInfoData {
                name: "FiredHoldAndReleaseModifier",
                name_hash: 2736996067,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WeaponFiringNetStateData, fired_hold_and_release_modifier),
            },
        ],
    }),
    array_type: Some(WEAPONFIRINGNETSTATEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WeaponFiringNetStateData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRINGNETSTATEDATA_TYPE_INFO
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


pub static WEAPONFIRINGNETSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringNetStateData-Array",
    name_hash: 1716948726,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringNetStateData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WeaponNetworkState {
    #[default]
    WeaponIdle = 0,
    WeaponSingleFiring = 1,
    WeaponAutomaticFiring = 2,
    WeaponReloading = 3,
    WeaponBoltAction = 4,
    WeaponHoldAndReleaseHold = 5,
}

pub static WEAPONNETWORKSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponNetworkState",
    name_hash: 2366516750,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONNETWORKSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponNetworkState {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONNETWORKSTATE_TYPE_INFO
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


pub static WEAPONNETWORKSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponNetworkState-Array",
    name_hash: 4169307962,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponNetworkState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static WEAPONFIRINGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEvent",
    name_hash: 3769091382,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WEAPONFIRINGEVENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WeaponFiringEvent {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONFIRINGEVENT_TYPE_INFO
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


pub static WEAPONFIRINGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponFiringEvent-Array",
    name_hash: 351304066,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponFiringEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponSuppressionData {
    pub _glacier_base: super::core::DataContainer,
    pub max_multiplier: f32,
    pub min_multiplier: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}

pub trait WeaponSuppressionDataTrait: super::core::DataContainerTrait {
    fn max_multiplier(&self) -> &f32;
    fn max_multiplier_mut(&mut self) -> &mut f32;
    fn min_multiplier(&self) -> &f32;
    fn min_multiplier_mut(&mut self) -> &mut f32;
    fn min_distance(&self) -> &f32;
    fn min_distance_mut(&mut self) -> &mut f32;
    fn max_distance(&self) -> &f32;
    fn max_distance_mut(&mut self) -> &mut f32;
}

impl WeaponSuppressionDataTrait for WeaponSuppressionData {
    fn max_multiplier(&self) -> &f32 {
        &self.max_multiplier
    }
    fn max_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.max_multiplier
    }
    fn min_multiplier(&self) -> &f32 {
        &self.min_multiplier
    }
    fn min_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.min_multiplier
    }
    fn min_distance(&self) -> &f32 {
        &self.min_distance
    }
    fn min_distance_mut(&mut self) -> &mut f32 {
        &mut self.min_distance
    }
    fn max_distance(&self) -> &f32 {
        &self.max_distance
    }
    fn max_distance_mut(&mut self) -> &mut f32 {
        &mut self.max_distance
    }
}

impl super::core::DataContainerTrait for WeaponSuppressionData {
}

pub static WEAPONSUPPRESSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSuppressionData",
    name_hash: 3583296334,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(WeaponSuppressionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponSuppressionData as Default>::default())),
            create_boxed: || Box::new(<WeaponSuppressionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxMultiplier",
                name_hash: 441185306,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponSuppressionData, max_multiplier),
            },
            FieldInfoData {
                name: "MinMultiplier",
                name_hash: 3723256324,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponSuppressionData, min_multiplier),
            },
            FieldInfoData {
                name: "MinDistance",
                name_hash: 1885855628,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponSuppressionData, min_distance),
            },
            FieldInfoData {
                name: "MaxDistance",
                name_hash: 3520454034,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponSuppressionData, max_distance),
            },
        ],
    }),
    array_type: Some(WEAPONSUPPRESSIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponSuppressionData {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONSUPPRESSIONDATA_TYPE_INFO
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


pub static WEAPONSUPPRESSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponSuppressionData-Array",
    name_hash: 4207172858,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponSuppressionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RumbleFiringData {
    pub low_rumble: f32,
    pub high_rumble: f32,
    pub rumble_duration: f32,
}

pub trait RumbleFiringDataTrait: TypeObject {
    fn low_rumble(&self) -> &f32;
    fn low_rumble_mut(&mut self) -> &mut f32;
    fn high_rumble(&self) -> &f32;
    fn high_rumble_mut(&mut self) -> &mut f32;
    fn rumble_duration(&self) -> &f32;
    fn rumble_duration_mut(&mut self) -> &mut f32;
}

impl RumbleFiringDataTrait for RumbleFiringData {
    fn low_rumble(&self) -> &f32 {
        &self.low_rumble
    }
    fn low_rumble_mut(&mut self) -> &mut f32 {
        &mut self.low_rumble
    }
    fn high_rumble(&self) -> &f32 {
        &self.high_rumble
    }
    fn high_rumble_mut(&mut self) -> &mut f32 {
        &mut self.high_rumble
    }
    fn rumble_duration(&self) -> &f32 {
        &self.rumble_duration
    }
    fn rumble_duration_mut(&mut self) -> &mut f32 {
        &mut self.rumble_duration
    }
}

pub static RUMBLEFIRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleFiringData",
    name_hash: 3823996553,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RumbleFiringData as Default>::default())),
            create_boxed: || Box::new(<RumbleFiringData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LowRumble",
                name_hash: 4150294224,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RumbleFiringData, low_rumble),
            },
            FieldInfoData {
                name: "HighRumble",
                name_hash: 2083796746,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RumbleFiringData, high_rumble),
            },
            FieldInfoData {
                name: "RumbleDuration",
                name_hash: 1471546714,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RumbleFiringData, rumble_duration),
            },
        ],
    }),
    array_type: Some(RUMBLEFIRINGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RumbleFiringData {
    fn type_info(&self) -> &'static TypeInfo {
        RUMBLEFIRINGDATA_TYPE_INFO
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


pub static RUMBLEFIRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RumbleFiringData-Array",
    name_hash: 258425917,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("RumbleFiringData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FiringFunctionData {
    pub _glacier_base: super::core::GameDataContainer,
    pub weapon_dispersion: WeaponDispersion,
    pub fire_effects1p: Vec<BoxedTypeObject /* FireEffectData */>,
    pub fire_effects3p: Vec<BoxedTypeObject /* FireEffectData */>,
    pub sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub shot: ShotConfigData,
    pub fire_logic: FireLogicData,
    pub ammo: AmmoConfigData,
    pub over_heat: OverHeatData,
    pub use_primary_ammo: bool,
    pub unlimited_ammo_for_a_i: bool,
    pub self_heal_time_when_deployed: f32,
}

pub trait FiringFunctionDataTrait: super::core::GameDataContainerTrait {
    fn weapon_dispersion(&self) -> &WeaponDispersion;
    fn weapon_dispersion_mut(&mut self) -> &mut WeaponDispersion;
    fn fire_effects1p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */>;
    fn fire_effects1p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */>;
    fn fire_effects3p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */>;
    fn fire_effects3p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */>;
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn shot(&self) -> &ShotConfigData;
    fn shot_mut(&mut self) -> &mut ShotConfigData;
    fn fire_logic(&self) -> &FireLogicData;
    fn fire_logic_mut(&mut self) -> &mut FireLogicData;
    fn ammo(&self) -> &AmmoConfigData;
    fn ammo_mut(&mut self) -> &mut AmmoConfigData;
    fn over_heat(&self) -> &OverHeatData;
    fn over_heat_mut(&mut self) -> &mut OverHeatData;
    fn use_primary_ammo(&self) -> &bool;
    fn use_primary_ammo_mut(&mut self) -> &mut bool;
    fn unlimited_ammo_for_a_i(&self) -> &bool;
    fn unlimited_ammo_for_a_i_mut(&mut self) -> &mut bool;
    fn self_heal_time_when_deployed(&self) -> &f32;
    fn self_heal_time_when_deployed_mut(&mut self) -> &mut f32;
}

impl FiringFunctionDataTrait for FiringFunctionData {
    fn weapon_dispersion(&self) -> &WeaponDispersion {
        &self.weapon_dispersion
    }
    fn weapon_dispersion_mut(&mut self) -> &mut WeaponDispersion {
        &mut self.weapon_dispersion
    }
    fn fire_effects1p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */> {
        &self.fire_effects1p
    }
    fn fire_effects1p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */> {
        &mut self.fire_effects1p
    }
    fn fire_effects3p(&self) -> &Vec<BoxedTypeObject /* FireEffectData */> {
        &self.fire_effects3p
    }
    fn fire_effects3p_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FireEffectData */> {
        &mut self.fire_effects3p
    }
    fn sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.sound
    }
    fn sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.sound
    }
    fn shot(&self) -> &ShotConfigData {
        &self.shot
    }
    fn shot_mut(&mut self) -> &mut ShotConfigData {
        &mut self.shot
    }
    fn fire_logic(&self) -> &FireLogicData {
        &self.fire_logic
    }
    fn fire_logic_mut(&mut self) -> &mut FireLogicData {
        &mut self.fire_logic
    }
    fn ammo(&self) -> &AmmoConfigData {
        &self.ammo
    }
    fn ammo_mut(&mut self) -> &mut AmmoConfigData {
        &mut self.ammo
    }
    fn over_heat(&self) -> &OverHeatData {
        &self.over_heat
    }
    fn over_heat_mut(&mut self) -> &mut OverHeatData {
        &mut self.over_heat
    }
    fn use_primary_ammo(&self) -> &bool {
        &self.use_primary_ammo
    }
    fn use_primary_ammo_mut(&mut self) -> &mut bool {
        &mut self.use_primary_ammo
    }
    fn unlimited_ammo_for_a_i(&self) -> &bool {
        &self.unlimited_ammo_for_a_i
    }
    fn unlimited_ammo_for_a_i_mut(&mut self) -> &mut bool {
        &mut self.unlimited_ammo_for_a_i
    }
    fn self_heal_time_when_deployed(&self) -> &f32 {
        &self.self_heal_time_when_deployed
    }
    fn self_heal_time_when_deployed_mut(&mut self) -> &mut f32 {
        &mut self.self_heal_time_when_deployed
    }
}

impl super::core::GameDataContainerTrait for FiringFunctionData {
}

impl super::core::DataContainerTrait for FiringFunctionData {
}

pub static FIRINGFUNCTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringFunctionData",
    name_hash: 3115868874,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::GAMEDATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(FiringFunctionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FiringFunctionData as Default>::default())),
            create_boxed: || Box::new(<FiringFunctionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "WeaponDispersion",
                name_hash: 1873092453,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponDispersion",
                rust_offset: offset_of!(FiringFunctionData, weapon_dispersion),
            },
            FieldInfoData {
                name: "FireEffects1p",
                name_hash: 3749424600,
                flags: MemberInfoFlags::new(144),
                field_type: "FireEffectData-Array",
                rust_offset: offset_of!(FiringFunctionData, fire_effects1p),
            },
            FieldInfoData {
                name: "FireEffects3p",
                name_hash: 3749424538,
                flags: MemberInfoFlags::new(144),
                field_type: "FireEffectData-Array",
                rust_offset: offset_of!(FiringFunctionData, fire_effects3p),
            },
            FieldInfoData {
                name: "Sound",
                name_hash: 231353798,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(FiringFunctionData, sound),
            },
            FieldInfoData {
                name: "Shot",
                name_hash: 2089430885,
                flags: MemberInfoFlags::new(0),
                field_type: "ShotConfigData",
                rust_offset: offset_of!(FiringFunctionData, shot),
            },
            FieldInfoData {
                name: "FireLogic",
                name_hash: 1556251859,
                flags: MemberInfoFlags::new(0),
                field_type: "FireLogicData",
                rust_offset: offset_of!(FiringFunctionData, fire_logic),
            },
            FieldInfoData {
                name: "Ammo",
                name_hash: 2088639403,
                flags: MemberInfoFlags::new(0),
                field_type: "AmmoConfigData",
                rust_offset: offset_of!(FiringFunctionData, ammo),
            },
            FieldInfoData {
                name: "OverHeat",
                name_hash: 3716934067,
                flags: MemberInfoFlags::new(0),
                field_type: "OverHeatData",
                rust_offset: offset_of!(FiringFunctionData, over_heat),
            },
            FieldInfoData {
                name: "UsePrimaryAmmo",
                name_hash: 1249980612,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FiringFunctionData, use_primary_ammo),
            },
            FieldInfoData {
                name: "UnlimitedAmmoForAI",
                name_hash: 2708726775,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FiringFunctionData, unlimited_ammo_for_a_i),
            },
            FieldInfoData {
                name: "SelfHealTimeWhenDeployed",
                name_hash: 462585362,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FiringFunctionData, self_heal_time_when_deployed),
            },
        ],
    }),
    array_type: Some(FIRINGFUNCTIONDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FiringFunctionData {
    fn type_info(&self) -> &'static TypeInfo {
        FIRINGFUNCTIONDATA_TYPE_INFO
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


pub static FIRINGFUNCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringFunctionData-Array",
    name_hash: 4106350206,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FiringFunctionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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

pub trait WeaponDispersionTrait: TypeObject {
    fn stand_dispersion(&self) -> &FiringDispersionData;
    fn stand_dispersion_mut(&mut self) -> &mut FiringDispersionData;
    fn crouch_dispersion(&self) -> &FiringDispersionData;
    fn crouch_dispersion_mut(&mut self) -> &mut FiringDispersionData;
    fn prone_dispersion(&self) -> &FiringDispersionData;
    fn prone_dispersion_mut(&mut self) -> &mut FiringDispersionData;
    fn jump_dispersion_angle(&self) -> &f32;
    fn jump_dispersion_angle_mut(&mut self) -> &mut f32;
    fn prone_transition_dispersion_angle(&self) -> &f32;
    fn prone_transition_dispersion_angle_mut(&mut self) -> &mut f32;
    fn move_dispersion_angle(&self) -> &f32;
    fn move_dispersion_angle_mut(&mut self) -> &mut f32;
    fn move_zoomed_dispersion_angle(&self) -> &f32;
    fn move_zoomed_dispersion_angle_mut(&mut self) -> &mut f32;
    fn decrease_per_second(&self) -> &f32;
    fn decrease_per_second_mut(&mut self) -> &mut f32;
}

impl WeaponDispersionTrait for WeaponDispersion {
    fn stand_dispersion(&self) -> &FiringDispersionData {
        &self.stand_dispersion
    }
    fn stand_dispersion_mut(&mut self) -> &mut FiringDispersionData {
        &mut self.stand_dispersion
    }
    fn crouch_dispersion(&self) -> &FiringDispersionData {
        &self.crouch_dispersion
    }
    fn crouch_dispersion_mut(&mut self) -> &mut FiringDispersionData {
        &mut self.crouch_dispersion
    }
    fn prone_dispersion(&self) -> &FiringDispersionData {
        &self.prone_dispersion
    }
    fn prone_dispersion_mut(&mut self) -> &mut FiringDispersionData {
        &mut self.prone_dispersion
    }
    fn jump_dispersion_angle(&self) -> &f32 {
        &self.jump_dispersion_angle
    }
    fn jump_dispersion_angle_mut(&mut self) -> &mut f32 {
        &mut self.jump_dispersion_angle
    }
    fn prone_transition_dispersion_angle(&self) -> &f32 {
        &self.prone_transition_dispersion_angle
    }
    fn prone_transition_dispersion_angle_mut(&mut self) -> &mut f32 {
        &mut self.prone_transition_dispersion_angle
    }
    fn move_dispersion_angle(&self) -> &f32 {
        &self.move_dispersion_angle
    }
    fn move_dispersion_angle_mut(&mut self) -> &mut f32 {
        &mut self.move_dispersion_angle
    }
    fn move_zoomed_dispersion_angle(&self) -> &f32 {
        &self.move_zoomed_dispersion_angle
    }
    fn move_zoomed_dispersion_angle_mut(&mut self) -> &mut f32 {
        &mut self.move_zoomed_dispersion_angle
    }
    fn decrease_per_second(&self) -> &f32 {
        &self.decrease_per_second
    }
    fn decrease_per_second_mut(&mut self) -> &mut f32 {
        &mut self.decrease_per_second
    }
}

pub static WEAPONDISPERSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersion",
    name_hash: 1873092453,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponDispersion as Default>::default())),
            create_boxed: || Box::new(<WeaponDispersion as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StandDispersion",
                name_hash: 1584552523,
                flags: MemberInfoFlags::new(0),
                field_type: "FiringDispersionData",
                rust_offset: offset_of!(WeaponDispersion, stand_dispersion),
            },
            FieldInfoData {
                name: "CrouchDispersion",
                name_hash: 429011943,
                flags: MemberInfoFlags::new(0),
                field_type: "FiringDispersionData",
                rust_offset: offset_of!(WeaponDispersion, crouch_dispersion),
            },
            FieldInfoData {
                name: "ProneDispersion",
                name_hash: 1226401409,
                flags: MemberInfoFlags::new(0),
                field_type: "FiringDispersionData",
                rust_offset: offset_of!(WeaponDispersion, prone_dispersion),
            },
            FieldInfoData {
                name: "JumpDispersionAngle",
                name_hash: 1831944196,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersion, jump_dispersion_angle),
            },
            FieldInfoData {
                name: "ProneTransitionDispersionAngle",
                name_hash: 4178809039,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersion, prone_transition_dispersion_angle),
            },
            FieldInfoData {
                name: "MoveDispersionAngle",
                name_hash: 371686199,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersion, move_dispersion_angle),
            },
            FieldInfoData {
                name: "MoveZoomedDispersionAngle",
                name_hash: 2897726465,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersion, move_zoomed_dispersion_angle),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                name_hash: 3208554608,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WeaponDispersion, decrease_per_second),
            },
        ],
    }),
    array_type: Some(WEAPONDISPERSION_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for WeaponDispersion {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONDISPERSION_TYPE_INFO
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


pub static WEAPONDISPERSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponDispersion-Array",
    name_hash: 3656697937,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponDispersion"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OverHeatData {
    pub heat_per_bullet: f32,
    pub heat_drop_per_second: f32,
    pub heat_increase_per_second: f32,
    pub over_heat_drop_delay: f32,
    pub over_heat_penalty_time: f32,
    pub over_heat_threshold: f32,
    pub over_heat_effect: FireEffectData,
}

pub trait OverHeatDataTrait: TypeObject {
    fn heat_per_bullet(&self) -> &f32;
    fn heat_per_bullet_mut(&mut self) -> &mut f32;
    fn heat_drop_per_second(&self) -> &f32;
    fn heat_drop_per_second_mut(&mut self) -> &mut f32;
    fn heat_increase_per_second(&self) -> &f32;
    fn heat_increase_per_second_mut(&mut self) -> &mut f32;
    fn over_heat_drop_delay(&self) -> &f32;
    fn over_heat_drop_delay_mut(&mut self) -> &mut f32;
    fn over_heat_penalty_time(&self) -> &f32;
    fn over_heat_penalty_time_mut(&mut self) -> &mut f32;
    fn over_heat_threshold(&self) -> &f32;
    fn over_heat_threshold_mut(&mut self) -> &mut f32;
    fn over_heat_effect(&self) -> &FireEffectData;
    fn over_heat_effect_mut(&mut self) -> &mut FireEffectData;
}

impl OverHeatDataTrait for OverHeatData {
    fn heat_per_bullet(&self) -> &f32 {
        &self.heat_per_bullet
    }
    fn heat_per_bullet_mut(&mut self) -> &mut f32 {
        &mut self.heat_per_bullet
    }
    fn heat_drop_per_second(&self) -> &f32 {
        &self.heat_drop_per_second
    }
    fn heat_drop_per_second_mut(&mut self) -> &mut f32 {
        &mut self.heat_drop_per_second
    }
    fn heat_increase_per_second(&self) -> &f32 {
        &self.heat_increase_per_second
    }
    fn heat_increase_per_second_mut(&mut self) -> &mut f32 {
        &mut self.heat_increase_per_second
    }
    fn over_heat_drop_delay(&self) -> &f32 {
        &self.over_heat_drop_delay
    }
    fn over_heat_drop_delay_mut(&mut self) -> &mut f32 {
        &mut self.over_heat_drop_delay
    }
    fn over_heat_penalty_time(&self) -> &f32 {
        &self.over_heat_penalty_time
    }
    fn over_heat_penalty_time_mut(&mut self) -> &mut f32 {
        &mut self.over_heat_penalty_time
    }
    fn over_heat_threshold(&self) -> &f32 {
        &self.over_heat_threshold
    }
    fn over_heat_threshold_mut(&mut self) -> &mut f32 {
        &mut self.over_heat_threshold
    }
    fn over_heat_effect(&self) -> &FireEffectData {
        &self.over_heat_effect
    }
    fn over_heat_effect_mut(&mut self) -> &mut FireEffectData {
        &mut self.over_heat_effect
    }
}

pub static OVERHEATDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverHeatData",
    name_hash: 807756483,
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OverHeatData as Default>::default())),
            create_boxed: || Box::new(<OverHeatData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HeatPerBullet",
                name_hash: 2230088284,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OverHeatData, heat_per_bullet),
            },
            FieldInfoData {
                name: "HeatDropPerSecond",
                name_hash: 2173208259,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OverHeatData, heat_drop_per_second),
            },
            FieldInfoData {
                name: "HeatIncreasePerSecond",
                name_hash: 3418659534,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OverHeatData, heat_increase_per_second),
            },
            FieldInfoData {
                name: "OverHeatDropDelay",
                name_hash: 290540335,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OverHeatData, over_heat_drop_delay),
            },
            FieldInfoData {
                name: "OverHeatPenaltyTime",
                name_hash: 1305646237,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OverHeatData, over_heat_penalty_time),
            },
            FieldInfoData {
                name: "OverHeatThreshold",
                name_hash: 1042108772,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OverHeatData, over_heat_threshold),
            },
            FieldInfoData {
                name: "OverHeatEffect",
                name_hash: 3498278916,
                flags: MemberInfoFlags::new(0),
                field_type: "FireEffectData",
                rust_offset: offset_of!(OverHeatData, over_heat_effect),
            },
        ],
    }),
    array_type: Some(OVERHEATDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OverHeatData {
    fn type_info(&self) -> &'static TypeInfo {
        OVERHEATDATA_TYPE_INFO
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


pub static OVERHEATDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OverHeatData-Array",
    name_hash: 807806455,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("OverHeatData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FireEffectData {
    pub effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
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

pub trait FireEffectDataTrait: TypeObject {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn offset(&self) -> &super::core::Vec3;
    fn offset_mut(&mut self) -> &mut super::core::Vec3;
    fn rotation(&self) -> &super::core::Vec3;
    fn rotation_mut(&mut self) -> &mut super::core::Vec3;
    fn zoom_offset(&self) -> &super::core::Vec3;
    fn zoom_offset_mut(&mut self) -> &mut super::core::Vec3;
    fn use_zoom_offset(&self) -> &bool;
    fn use_zoom_offset_mut(&mut self) -> &mut bool;
    fn zoom_rotation(&self) -> &super::core::Vec3;
    fn zoom_rotation_mut(&mut self) -> &mut super::core::Vec3;
    fn use_zoom_rotation(&self) -> &bool;
    fn use_zoom_rotation_mut(&mut self) -> &mut bool;
    fn disable_during_zoom(&self) -> &bool;
    fn disable_during_zoom_mut(&mut self) -> &mut bool;
    fn update_transform(&self) -> &bool;
    fn update_transform_mut(&mut self) -> &mut bool;
    fn stop_looping_effects(&self) -> &bool;
    fn stop_looping_effects_mut(&mut self) -> &mut bool;
}

impl FireEffectDataTrait for FireEffectData {
    fn effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.effect
    }
    fn effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.effect
    }
    fn offset(&self) -> &super::core::Vec3 {
        &self.offset
    }
    fn offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.offset
    }
    fn rotation(&self) -> &super::core::Vec3 {
        &self.rotation
    }
    fn rotation_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.rotation
    }
    fn zoom_offset(&self) -> &super::core::Vec3 {
        &self.zoom_offset
    }
    fn zoom_offset_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.zoom_offset
    }
    fn use_zoom_offset(&self) -> &bool {
        &self.use_zoom_offset
    }
    fn use_zoom_offset_mut(&mut self) -> &mut bool {
        &mut self.use_zoom_offset
    }
    fn zoom_rotation(&self) -> &super::core::Vec3 {
        &self.zoom_rotation
    }
    fn zoom_rotation_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.zoom_rotation
    }
    fn use_zoom_rotation(&self) -> &bool {
        &self.use_zoom_rotation
    }
    fn use_zoom_rotation_mut(&mut self) -> &mut bool {
        &mut self.use_zoom_rotation
    }
    fn disable_during_zoom(&self) -> &bool {
        &self.disable_during_zoom
    }
    fn disable_during_zoom_mut(&mut self) -> &mut bool {
        &mut self.disable_during_zoom
    }
    fn update_transform(&self) -> &bool {
        &self.update_transform
    }
    fn update_transform_mut(&mut self) -> &mut bool {
        &mut self.update_transform
    }
    fn stop_looping_effects(&self) -> &bool {
        &self.stop_looping_effects
    }
    fn stop_looping_effects_mut(&mut self) -> &mut bool {
        &mut self.stop_looping_effects
    }
}

pub static FIREEFFECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireEffectData",
    name_hash: 3470469370,
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FireEffectData as Default>::default())),
            create_boxed: || Box::new(<FireEffectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Effect",
                name_hash: 2332983090,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(FireEffectData, effect),
            },
            FieldInfoData {
                name: "Offset",
                name_hash: 2871410728,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FireEffectData, offset),
            },
            FieldInfoData {
                name: "Rotation",
                name_hash: 48673745,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FireEffectData, rotation),
            },
            FieldInfoData {
                name: "ZoomOffset",
                name_hash: 4245097407,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FireEffectData, zoom_offset),
            },
            FieldInfoData {
                name: "UseZoomOffset",
                name_hash: 3904716028,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireEffectData, use_zoom_offset),
            },
            FieldInfoData {
                name: "ZoomRotation",
                name_hash: 3320560134,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(FireEffectData, zoom_rotation),
            },
            FieldInfoData {
                name: "UseZoomRotation",
                name_hash: 3031914373,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireEffectData, use_zoom_rotation),
            },
            FieldInfoData {
                name: "DisableDuringZoom",
                name_hash: 3735365637,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireEffectData, disable_during_zoom),
            },
            FieldInfoData {
                name: "UpdateTransform",
                name_hash: 2409007832,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireEffectData, update_transform),
            },
            FieldInfoData {
                name: "StopLoopingEffects",
                name_hash: 3709447781,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireEffectData, stop_looping_effects),
            },
        ],
    }),
    array_type: Some(FIREEFFECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FireEffectData {
    fn type_info(&self) -> &'static TypeInfo {
        FIREEFFECTDATA_TYPE_INFO
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


pub static FIREEFFECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireEffectData-Array",
    name_hash: 914299854,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FireEffectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FiringDispersionData {
    pub min_angle: f32,
    pub max_angle: f32,
    pub increase_per_shot: f32,
    pub decrease_per_second: f32,
}

pub trait FiringDispersionDataTrait: TypeObject {
    fn min_angle(&self) -> &f32;
    fn min_angle_mut(&mut self) -> &mut f32;
    fn max_angle(&self) -> &f32;
    fn max_angle_mut(&mut self) -> &mut f32;
    fn increase_per_shot(&self) -> &f32;
    fn increase_per_shot_mut(&mut self) -> &mut f32;
    fn decrease_per_second(&self) -> &f32;
    fn decrease_per_second_mut(&mut self) -> &mut f32;
}

impl FiringDispersionDataTrait for FiringDispersionData {
    fn min_angle(&self) -> &f32 {
        &self.min_angle
    }
    fn min_angle_mut(&mut self) -> &mut f32 {
        &mut self.min_angle
    }
    fn max_angle(&self) -> &f32 {
        &self.max_angle
    }
    fn max_angle_mut(&mut self) -> &mut f32 {
        &mut self.max_angle
    }
    fn increase_per_shot(&self) -> &f32 {
        &self.increase_per_shot
    }
    fn increase_per_shot_mut(&mut self) -> &mut f32 {
        &mut self.increase_per_shot
    }
    fn decrease_per_second(&self) -> &f32 {
        &self.decrease_per_second
    }
    fn decrease_per_second_mut(&mut self) -> &mut f32 {
        &mut self.decrease_per_second
    }
}

pub static FIRINGDISPERSIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringDispersionData",
    name_hash: 745570346,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FiringDispersionData as Default>::default())),
            create_boxed: || Box::new(<FiringDispersionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MinAngle",
                name_hash: 3356124462,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FiringDispersionData, min_angle),
            },
            FieldInfoData {
                name: "MaxAngle",
                name_hash: 417488496,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FiringDispersionData, max_angle),
            },
            FieldInfoData {
                name: "IncreasePerShot",
                name_hash: 95639814,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FiringDispersionData, increase_per_shot),
            },
            FieldInfoData {
                name: "DecreasePerSecond",
                name_hash: 3208554608,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FiringDispersionData, decrease_per_second),
            },
        ],
    }),
    array_type: Some(FIRINGDISPERSIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FiringDispersionData {
    fn type_info(&self) -> &'static TypeInfo {
        FIRINGDISPERSIONDATA_TYPE_INFO
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


pub static FIRINGDISPERSIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FiringDispersionData-Array",
    name_hash: 851860126,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FiringDispersionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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

pub trait FireLogicDataTrait: TypeObject {
    fn hold_and_release(&self) -> &HoldAndReleaseData;
    fn hold_and_release_mut(&mut self) -> &mut HoldAndReleaseData;
    fn bolt_action(&self) -> &BoltActionData;
    fn bolt_action_mut(&mut self) -> &mut BoltActionData;
    fn recoil(&self) -> &RecoilData;
    fn recoil_mut(&mut self) -> &mut RecoilData;
    fn fire_input_action(&self) -> &i32;
    fn fire_input_action_mut(&mut self) -> &mut i32;
    fn reload_input_action(&self) -> &i32;
    fn reload_input_action_mut(&mut self) -> &mut i32;
    fn cycle_fire_mode_input_action(&self) -> &i32;
    fn cycle_fire_mode_input_action_mut(&mut self) -> &mut i32;
    fn charge_input_action(&self) -> &i32;
    fn charge_input_action_mut(&mut self) -> &mut i32;
    fn trigger_pull_weight(&self) -> &f32;
    fn trigger_pull_weight_mut(&mut self) -> &mut f32;
    fn rate_of_fire(&self) -> &f32;
    fn rate_of_fire_mut(&mut self) -> &mut f32;
    fn rate_of_fire_for_burst(&self) -> &f32;
    fn rate_of_fire_for_burst_mut(&mut self) -> &mut f32;
    fn bursts_per_minute(&self) -> &f32;
    fn bursts_per_minute_mut(&mut self) -> &mut f32;
    fn corrected_automatic_fire_replication(&self) -> &bool;
    fn corrected_automatic_fire_replication_mut(&mut self) -> &mut bool;
    fn client_fire_rate_multiplier(&self) -> &f32;
    fn client_fire_rate_multiplier_mut(&mut self) -> &mut f32;
    fn reload_delay(&self) -> &f32;
    fn reload_delay_mut(&mut self) -> &mut f32;
    fn hold_off_reload_until_fire_release(&self) -> &bool;
    fn hold_off_reload_until_fire_release_mut(&mut self) -> &mut bool;
    fn hold_off_reload_until_zoom_release(&self) -> &bool;
    fn hold_off_reload_until_zoom_release_mut(&mut self) -> &mut bool;
    fn force_reload_action_on_fire_trigger(&self) -> &bool;
    fn force_reload_action_on_fire_trigger_mut(&mut self) -> &mut bool;
    fn reload_time(&self) -> &f32;
    fn reload_time_mut(&mut self) -> &mut f32;
    fn reload_timer_array(&self) -> &Vec<f32>;
    fn reload_timer_array_mut(&mut self) -> &mut Vec<f32>;
    fn reload_time_bullets_left(&self) -> &f32;
    fn reload_time_bullets_left_mut(&mut self) -> &mut f32;
    fn reload_threshold(&self) -> &f32;
    fn reload_threshold_mut(&mut self) -> &mut f32;
    fn pre_fire_delay(&self) -> &f32;
    fn pre_fire_delay_mut(&mut self) -> &mut f32;
    fn pending_fire_window(&self) -> &f32;
    fn pending_fire_window_mut(&mut self) -> &mut f32;
    fn pending_fire_ignore_fire_rate_window(&self) -> &f32;
    fn pending_fire_ignore_fire_rate_window_mut(&mut self) -> &mut f32;
    fn reset_pre_fire_wait_on_release(&self) -> &bool;
    fn reset_pre_fire_wait_on_release_mut(&mut self) -> &mut bool;
    fn use_charge_up_logic(&self) -> &bool;
    fn use_charge_up_logic_mut(&mut self) -> &mut bool;
    fn charge_up_increase(&self) -> &f32;
    fn charge_up_increase_mut(&mut self) -> &mut f32;
    fn charge_up_decrease(&self) -> &f32;
    fn charge_up_decrease_mut(&mut self) -> &mut f32;
    fn automatic_delay(&self) -> &f32;
    fn automatic_delay_mut(&mut self) -> &mut f32;
    fn reload_logic(&self) -> &ReloadLogic;
    fn reload_logic_mut(&mut self) -> &mut ReloadLogic;
    fn reload_type(&self) -> &ReloadType;
    fn reload_type_mut(&mut self) -> &mut ReloadType;
    fn fire_logic_type(&self) -> &FireLogicType;
    fn fire_logic_type_mut(&mut self) -> &mut FireLogicType;
    fn fire_logic_type_array(&self) -> &Vec<FireLogicType>;
    fn fire_logic_type_array_mut(&mut self) -> &mut Vec<FireLogicType>;
    fn always_auto_reload(&self) -> &bool;
    fn always_auto_reload_mut(&mut self) -> &mut bool;
    fn auto_reload_ignore_fire_trigger_pressed(&self) -> &bool;
    fn auto_reload_ignore_fire_trigger_pressed_mut(&mut self) -> &mut bool;
}

impl FireLogicDataTrait for FireLogicData {
    fn hold_and_release(&self) -> &HoldAndReleaseData {
        &self.hold_and_release
    }
    fn hold_and_release_mut(&mut self) -> &mut HoldAndReleaseData {
        &mut self.hold_and_release
    }
    fn bolt_action(&self) -> &BoltActionData {
        &self.bolt_action
    }
    fn bolt_action_mut(&mut self) -> &mut BoltActionData {
        &mut self.bolt_action
    }
    fn recoil(&self) -> &RecoilData {
        &self.recoil
    }
    fn recoil_mut(&mut self) -> &mut RecoilData {
        &mut self.recoil
    }
    fn fire_input_action(&self) -> &i32 {
        &self.fire_input_action
    }
    fn fire_input_action_mut(&mut self) -> &mut i32 {
        &mut self.fire_input_action
    }
    fn reload_input_action(&self) -> &i32 {
        &self.reload_input_action
    }
    fn reload_input_action_mut(&mut self) -> &mut i32 {
        &mut self.reload_input_action
    }
    fn cycle_fire_mode_input_action(&self) -> &i32 {
        &self.cycle_fire_mode_input_action
    }
    fn cycle_fire_mode_input_action_mut(&mut self) -> &mut i32 {
        &mut self.cycle_fire_mode_input_action
    }
    fn charge_input_action(&self) -> &i32 {
        &self.charge_input_action
    }
    fn charge_input_action_mut(&mut self) -> &mut i32 {
        &mut self.charge_input_action
    }
    fn trigger_pull_weight(&self) -> &f32 {
        &self.trigger_pull_weight
    }
    fn trigger_pull_weight_mut(&mut self) -> &mut f32 {
        &mut self.trigger_pull_weight
    }
    fn rate_of_fire(&self) -> &f32 {
        &self.rate_of_fire
    }
    fn rate_of_fire_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire
    }
    fn rate_of_fire_for_burst(&self) -> &f32 {
        &self.rate_of_fire_for_burst
    }
    fn rate_of_fire_for_burst_mut(&mut self) -> &mut f32 {
        &mut self.rate_of_fire_for_burst
    }
    fn bursts_per_minute(&self) -> &f32 {
        &self.bursts_per_minute
    }
    fn bursts_per_minute_mut(&mut self) -> &mut f32 {
        &mut self.bursts_per_minute
    }
    fn corrected_automatic_fire_replication(&self) -> &bool {
        &self.corrected_automatic_fire_replication
    }
    fn corrected_automatic_fire_replication_mut(&mut self) -> &mut bool {
        &mut self.corrected_automatic_fire_replication
    }
    fn client_fire_rate_multiplier(&self) -> &f32 {
        &self.client_fire_rate_multiplier
    }
    fn client_fire_rate_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.client_fire_rate_multiplier
    }
    fn reload_delay(&self) -> &f32 {
        &self.reload_delay
    }
    fn reload_delay_mut(&mut self) -> &mut f32 {
        &mut self.reload_delay
    }
    fn hold_off_reload_until_fire_release(&self) -> &bool {
        &self.hold_off_reload_until_fire_release
    }
    fn hold_off_reload_until_fire_release_mut(&mut self) -> &mut bool {
        &mut self.hold_off_reload_until_fire_release
    }
    fn hold_off_reload_until_zoom_release(&self) -> &bool {
        &self.hold_off_reload_until_zoom_release
    }
    fn hold_off_reload_until_zoom_release_mut(&mut self) -> &mut bool {
        &mut self.hold_off_reload_until_zoom_release
    }
    fn force_reload_action_on_fire_trigger(&self) -> &bool {
        &self.force_reload_action_on_fire_trigger
    }
    fn force_reload_action_on_fire_trigger_mut(&mut self) -> &mut bool {
        &mut self.force_reload_action_on_fire_trigger
    }
    fn reload_time(&self) -> &f32 {
        &self.reload_time
    }
    fn reload_time_mut(&mut self) -> &mut f32 {
        &mut self.reload_time
    }
    fn reload_timer_array(&self) -> &Vec<f32> {
        &self.reload_timer_array
    }
    fn reload_timer_array_mut(&mut self) -> &mut Vec<f32> {
        &mut self.reload_timer_array
    }
    fn reload_time_bullets_left(&self) -> &f32 {
        &self.reload_time_bullets_left
    }
    fn reload_time_bullets_left_mut(&mut self) -> &mut f32 {
        &mut self.reload_time_bullets_left
    }
    fn reload_threshold(&self) -> &f32 {
        &self.reload_threshold
    }
    fn reload_threshold_mut(&mut self) -> &mut f32 {
        &mut self.reload_threshold
    }
    fn pre_fire_delay(&self) -> &f32 {
        &self.pre_fire_delay
    }
    fn pre_fire_delay_mut(&mut self) -> &mut f32 {
        &mut self.pre_fire_delay
    }
    fn pending_fire_window(&self) -> &f32 {
        &self.pending_fire_window
    }
    fn pending_fire_window_mut(&mut self) -> &mut f32 {
        &mut self.pending_fire_window
    }
    fn pending_fire_ignore_fire_rate_window(&self) -> &f32 {
        &self.pending_fire_ignore_fire_rate_window
    }
    fn pending_fire_ignore_fire_rate_window_mut(&mut self) -> &mut f32 {
        &mut self.pending_fire_ignore_fire_rate_window
    }
    fn reset_pre_fire_wait_on_release(&self) -> &bool {
        &self.reset_pre_fire_wait_on_release
    }
    fn reset_pre_fire_wait_on_release_mut(&mut self) -> &mut bool {
        &mut self.reset_pre_fire_wait_on_release
    }
    fn use_charge_up_logic(&self) -> &bool {
        &self.use_charge_up_logic
    }
    fn use_charge_up_logic_mut(&mut self) -> &mut bool {
        &mut self.use_charge_up_logic
    }
    fn charge_up_increase(&self) -> &f32 {
        &self.charge_up_increase
    }
    fn charge_up_increase_mut(&mut self) -> &mut f32 {
        &mut self.charge_up_increase
    }
    fn charge_up_decrease(&self) -> &f32 {
        &self.charge_up_decrease
    }
    fn charge_up_decrease_mut(&mut self) -> &mut f32 {
        &mut self.charge_up_decrease
    }
    fn automatic_delay(&self) -> &f32 {
        &self.automatic_delay
    }
    fn automatic_delay_mut(&mut self) -> &mut f32 {
        &mut self.automatic_delay
    }
    fn reload_logic(&self) -> &ReloadLogic {
        &self.reload_logic
    }
    fn reload_logic_mut(&mut self) -> &mut ReloadLogic {
        &mut self.reload_logic
    }
    fn reload_type(&self) -> &ReloadType {
        &self.reload_type
    }
    fn reload_type_mut(&mut self) -> &mut ReloadType {
        &mut self.reload_type
    }
    fn fire_logic_type(&self) -> &FireLogicType {
        &self.fire_logic_type
    }
    fn fire_logic_type_mut(&mut self) -> &mut FireLogicType {
        &mut self.fire_logic_type
    }
    fn fire_logic_type_array(&self) -> &Vec<FireLogicType> {
        &self.fire_logic_type_array
    }
    fn fire_logic_type_array_mut(&mut self) -> &mut Vec<FireLogicType> {
        &mut self.fire_logic_type_array
    }
    fn always_auto_reload(&self) -> &bool {
        &self.always_auto_reload
    }
    fn always_auto_reload_mut(&mut self) -> &mut bool {
        &mut self.always_auto_reload
    }
    fn auto_reload_ignore_fire_trigger_pressed(&self) -> &bool {
        &self.auto_reload_ignore_fire_trigger_pressed
    }
    fn auto_reload_ignore_fire_trigger_pressed_mut(&mut self) -> &mut bool {
        &mut self.auto_reload_ignore_fire_trigger_pressed
    }
}

pub static FIRELOGICDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicData",
    name_hash: 1366516899,
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FireLogicData as Default>::default())),
            create_boxed: || Box::new(<FireLogicData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HoldAndRelease",
                name_hash: 990430632,
                flags: MemberInfoFlags::new(0),
                field_type: "HoldAndReleaseData",
                rust_offset: offset_of!(FireLogicData, hold_and_release),
            },
            FieldInfoData {
                name: "BoltAction",
                name_hash: 2541973070,
                flags: MemberInfoFlags::new(0),
                field_type: "BoltActionData",
                rust_offset: offset_of!(FireLogicData, bolt_action),
            },
            FieldInfoData {
                name: "Recoil",
                name_hash: 3293845435,
                flags: MemberInfoFlags::new(0),
                field_type: "RecoilData",
                rust_offset: offset_of!(FireLogicData, recoil),
            },
            FieldInfoData {
                name: "FireInputAction",
                name_hash: 2678055381,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FireLogicData, fire_input_action),
            },
            FieldInfoData {
                name: "ReloadInputAction",
                name_hash: 3775146780,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FireLogicData, reload_input_action),
            },
            FieldInfoData {
                name: "CycleFireModeInputAction",
                name_hash: 1729611942,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FireLogicData, cycle_fire_mode_input_action),
            },
            FieldInfoData {
                name: "ChargeInputAction",
                name_hash: 3470951671,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FireLogicData, charge_input_action),
            },
            FieldInfoData {
                name: "TriggerPullWeight",
                name_hash: 2750237048,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, trigger_pull_weight),
            },
            FieldInfoData {
                name: "RateOfFire",
                name_hash: 3866082710,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, rate_of_fire),
            },
            FieldInfoData {
                name: "RateOfFireForBurst",
                name_hash: 1393806543,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, rate_of_fire_for_burst),
            },
            FieldInfoData {
                name: "BurstsPerMinute",
                name_hash: 2886919901,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, bursts_per_minute),
            },
            FieldInfoData {
                name: "CorrectedAutomaticFireReplication",
                name_hash: 1051613411,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, corrected_automatic_fire_replication),
            },
            FieldInfoData {
                name: "ClientFireRateMultiplier",
                name_hash: 4266906285,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, client_fire_rate_multiplier),
            },
            FieldInfoData {
                name: "ReloadDelay",
                name_hash: 2958021633,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, reload_delay),
            },
            FieldInfoData {
                name: "HoldOffReloadUntilFireRelease",
                name_hash: 957247311,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, hold_off_reload_until_fire_release),
            },
            FieldInfoData {
                name: "HoldOffReloadUntilZoomRelease",
                name_hash: 3888370560,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, hold_off_reload_until_zoom_release),
            },
            FieldInfoData {
                name: "ForceReloadActionOnFireTrigger",
                name_hash: 2262034806,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, force_reload_action_on_fire_trigger),
            },
            FieldInfoData {
                name: "ReloadTime",
                name_hash: 1390571137,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, reload_time),
            },
            FieldInfoData {
                name: "ReloadTimerArray",
                name_hash: 1035134858,
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(FireLogicData, reload_timer_array),
            },
            FieldInfoData {
                name: "ReloadTimeBulletsLeft",
                name_hash: 257608303,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, reload_time_bullets_left),
            },
            FieldInfoData {
                name: "ReloadThreshold",
                name_hash: 607792483,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, reload_threshold),
            },
            FieldInfoData {
                name: "PreFireDelay",
                name_hash: 52152975,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, pre_fire_delay),
            },
            FieldInfoData {
                name: "PendingFireWindow",
                name_hash: 3361504206,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, pending_fire_window),
            },
            FieldInfoData {
                name: "PendingFireIgnoreFireRateWindow",
                name_hash: 1667285324,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, pending_fire_ignore_fire_rate_window),
            },
            FieldInfoData {
                name: "ResetPreFireWaitOnRelease",
                name_hash: 1571355692,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, reset_pre_fire_wait_on_release),
            },
            FieldInfoData {
                name: "UseChargeUpLogic",
                name_hash: 1344489207,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, use_charge_up_logic),
            },
            FieldInfoData {
                name: "ChargeUpIncrease",
                name_hash: 1366308222,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, charge_up_increase),
            },
            FieldInfoData {
                name: "ChargeUpDecrease",
                name_hash: 1290964760,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, charge_up_decrease),
            },
            FieldInfoData {
                name: "AutomaticDelay",
                name_hash: 3726024813,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FireLogicData, automatic_delay),
            },
            FieldInfoData {
                name: "ReloadLogic",
                name_hash: 2967140250,
                flags: MemberInfoFlags::new(0),
                field_type: "ReloadLogic",
                rust_offset: offset_of!(FireLogicData, reload_logic),
            },
            FieldInfoData {
                name: "ReloadType",
                name_hash: 1390588396,
                flags: MemberInfoFlags::new(0),
                field_type: "ReloadType",
                rust_offset: offset_of!(FireLogicData, reload_type),
            },
            FieldInfoData {
                name: "FireLogicType",
                name_hash: 1365968171,
                flags: MemberInfoFlags::new(0),
                field_type: "FireLogicType",
                rust_offset: offset_of!(FireLogicData, fire_logic_type),
            },
            FieldInfoData {
                name: "FireLogicTypeArray",
                name_hash: 4279400946,
                flags: MemberInfoFlags::new(144),
                field_type: "FireLogicType-Array",
                rust_offset: offset_of!(FireLogicData, fire_logic_type_array),
            },
            FieldInfoData {
                name: "AlwaysAutoReload",
                name_hash: 3053142090,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, always_auto_reload),
            },
            FieldInfoData {
                name: "AutoReloadIgnoreFireTriggerPressed",
                name_hash: 3107288677,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FireLogicData, auto_reload_ignore_fire_trigger_pressed),
            },
        ],
    }),
    array_type: Some(FIRELOGICDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FireLogicData {
    fn type_info(&self) -> &'static TypeInfo {
        FIRELOGICDATA_TYPE_INFO
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


pub static FIRELOGICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicData-Array",
    name_hash: 2128250903,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FireLogicData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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

pub trait RecoilDataTrait: TypeObject {
    fn max_recoil_angle_x(&self) -> &f32;
    fn max_recoil_angle_x_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_x(&self) -> &f32;
    fn min_recoil_angle_x_mut(&mut self) -> &mut f32;
    fn max_recoil_angle_y(&self) -> &f32;
    fn max_recoil_angle_y_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_y(&self) -> &f32;
    fn min_recoil_angle_y_mut(&mut self) -> &mut f32;
    fn max_recoil_angle_z(&self) -> &f32;
    fn max_recoil_angle_z_mut(&mut self) -> &mut f32;
    fn min_recoil_angle_z(&self) -> &f32;
    fn min_recoil_angle_z_mut(&mut self) -> &mut f32;
    fn max_recoil_fov(&self) -> &f32;
    fn max_recoil_fov_mut(&mut self) -> &mut f32;
    fn min_recoil_fov(&self) -> &f32;
    fn min_recoil_fov_mut(&mut self) -> &mut f32;
    fn recoil_follows_dispersion(&self) -> &bool;
    fn recoil_follows_dispersion_mut(&mut self) -> &mut bool;
}

impl RecoilDataTrait for RecoilData {
    fn max_recoil_angle_x(&self) -> &f32 {
        &self.max_recoil_angle_x
    }
    fn max_recoil_angle_x_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_x
    }
    fn min_recoil_angle_x(&self) -> &f32 {
        &self.min_recoil_angle_x
    }
    fn min_recoil_angle_x_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_x
    }
    fn max_recoil_angle_y(&self) -> &f32 {
        &self.max_recoil_angle_y
    }
    fn max_recoil_angle_y_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_y
    }
    fn min_recoil_angle_y(&self) -> &f32 {
        &self.min_recoil_angle_y
    }
    fn min_recoil_angle_y_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_y
    }
    fn max_recoil_angle_z(&self) -> &f32 {
        &self.max_recoil_angle_z
    }
    fn max_recoil_angle_z_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_angle_z
    }
    fn min_recoil_angle_z(&self) -> &f32 {
        &self.min_recoil_angle_z
    }
    fn min_recoil_angle_z_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_angle_z
    }
    fn max_recoil_fov(&self) -> &f32 {
        &self.max_recoil_fov
    }
    fn max_recoil_fov_mut(&mut self) -> &mut f32 {
        &mut self.max_recoil_fov
    }
    fn min_recoil_fov(&self) -> &f32 {
        &self.min_recoil_fov
    }
    fn min_recoil_fov_mut(&mut self) -> &mut f32 {
        &mut self.min_recoil_fov
    }
    fn recoil_follows_dispersion(&self) -> &bool {
        &self.recoil_follows_dispersion
    }
    fn recoil_follows_dispersion_mut(&mut self) -> &mut bool {
        &mut self.recoil_follows_dispersion
    }
}

pub static RECOILDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecoilData",
    name_hash: 2078627019,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RecoilData as Default>::default())),
            create_boxed: || Box::new(<RecoilData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxRecoilAngleX",
                name_hash: 1566215510,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, max_recoil_angle_x),
            },
            FieldInfoData {
                name: "MinRecoilAngleX",
                name_hash: 2010820808,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, min_recoil_angle_x),
            },
            FieldInfoData {
                name: "MaxRecoilAngleY",
                name_hash: 1566215511,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, max_recoil_angle_y),
            },
            FieldInfoData {
                name: "MinRecoilAngleY",
                name_hash: 2010820809,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, min_recoil_angle_y),
            },
            FieldInfoData {
                name: "MaxRecoilAngleZ",
                name_hash: 1566215508,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, max_recoil_angle_z),
            },
            FieldInfoData {
                name: "MinRecoilAngleZ",
                name_hash: 2010820810,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, min_recoil_angle_z),
            },
            FieldInfoData {
                name: "MaxRecoilFov",
                name_hash: 2812200304,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, max_recoil_fov),
            },
            FieldInfoData {
                name: "MinRecoilFov",
                name_hash: 3551417774,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RecoilData, min_recoil_fov),
            },
            FieldInfoData {
                name: "RecoilFollowsDispersion",
                name_hash: 4254392955,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(RecoilData, recoil_follows_dispersion),
            },
        ],
    }),
    array_type: Some(RECOILDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RecoilData {
    fn type_info(&self) -> &'static TypeInfo {
        RECOILDATA_TYPE_INFO
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


pub static RECOILDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecoilData-Array",
    name_hash: 3742561279,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("RecoilData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ReloadLogic {
    #[default]
    rlWeaponSwitchCancelsUnfinishedReload = 0,
    rlReloadUnaffectedByWeaponSwitch = 1,
}

pub static RELOADLOGIC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadLogic",
    name_hash: 2967140250,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(RELOADLOGIC_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ReloadLogic {
    fn type_info(&self) -> &'static TypeInfo {
        RELOADLOGIC_TYPE_INFO
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


pub static RELOADLOGIC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadLogic-Array",
    name_hash: 1785961134,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ReloadLogic"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ReloadType {
    #[default]
    rtSingleBullet = 0,
    rtMagazine = 1,
    rtMagazineWithPossibleShorterReload = 2,
}

pub static RELOADTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadType",
    name_hash: 1390588396,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(RELOADTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ReloadType {
    fn type_info(&self) -> &'static TypeInfo {
        RELOADTYPE_TYPE_INFO
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


pub static RELOADTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReloadType-Array",
    name_hash: 105517528,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ReloadType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static FIRELOGICTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicType",
    name_hash: 1365968171,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(FIRELOGICTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FireLogicType {
    fn type_info(&self) -> &'static TypeInfo {
        FIRELOGICTYPE_TYPE_INFO
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


pub static FIRELOGICTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FireLogicType-Array",
    name_hash: 2175263903,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("FireLogicType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BoltActionData {
    pub bolt_action_delay: f32,
    pub bolt_action_time: f32,
    pub hold_bolt_action_until_fire_release: bool,
    pub hold_bolt_action_until_zoom_release: bool,
    pub force_bolt_action_on_fire_trigger: bool,
    pub un_zoom_on_bolt_action: bool,
    pub return_to_zoom_after_bolt_action: bool,
}

pub trait BoltActionDataTrait: TypeObject {
    fn bolt_action_delay(&self) -> &f32;
    fn bolt_action_delay_mut(&mut self) -> &mut f32;
    fn bolt_action_time(&self) -> &f32;
    fn bolt_action_time_mut(&mut self) -> &mut f32;
    fn hold_bolt_action_until_fire_release(&self) -> &bool;
    fn hold_bolt_action_until_fire_release_mut(&mut self) -> &mut bool;
    fn hold_bolt_action_until_zoom_release(&self) -> &bool;
    fn hold_bolt_action_until_zoom_release_mut(&mut self) -> &mut bool;
    fn force_bolt_action_on_fire_trigger(&self) -> &bool;
    fn force_bolt_action_on_fire_trigger_mut(&mut self) -> &mut bool;
    fn un_zoom_on_bolt_action(&self) -> &bool;
    fn un_zoom_on_bolt_action_mut(&mut self) -> &mut bool;
    fn return_to_zoom_after_bolt_action(&self) -> &bool;
    fn return_to_zoom_after_bolt_action_mut(&mut self) -> &mut bool;
}

impl BoltActionDataTrait for BoltActionData {
    fn bolt_action_delay(&self) -> &f32 {
        &self.bolt_action_delay
    }
    fn bolt_action_delay_mut(&mut self) -> &mut f32 {
        &mut self.bolt_action_delay
    }
    fn bolt_action_time(&self) -> &f32 {
        &self.bolt_action_time
    }
    fn bolt_action_time_mut(&mut self) -> &mut f32 {
        &mut self.bolt_action_time
    }
    fn hold_bolt_action_until_fire_release(&self) -> &bool {
        &self.hold_bolt_action_until_fire_release
    }
    fn hold_bolt_action_until_fire_release_mut(&mut self) -> &mut bool {
        &mut self.hold_bolt_action_until_fire_release
    }
    fn hold_bolt_action_until_zoom_release(&self) -> &bool {
        &self.hold_bolt_action_until_zoom_release
    }
    fn hold_bolt_action_until_zoom_release_mut(&mut self) -> &mut bool {
        &mut self.hold_bolt_action_until_zoom_release
    }
    fn force_bolt_action_on_fire_trigger(&self) -> &bool {
        &self.force_bolt_action_on_fire_trigger
    }
    fn force_bolt_action_on_fire_trigger_mut(&mut self) -> &mut bool {
        &mut self.force_bolt_action_on_fire_trigger
    }
    fn un_zoom_on_bolt_action(&self) -> &bool {
        &self.un_zoom_on_bolt_action
    }
    fn un_zoom_on_bolt_action_mut(&mut self) -> &mut bool {
        &mut self.un_zoom_on_bolt_action
    }
    fn return_to_zoom_after_bolt_action(&self) -> &bool {
        &self.return_to_zoom_after_bolt_action
    }
    fn return_to_zoom_after_bolt_action_mut(&mut self) -> &mut bool {
        &mut self.return_to_zoom_after_bolt_action
    }
}

pub static BOLTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoltActionData",
    name_hash: 1831888766,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoltActionData as Default>::default())),
            create_boxed: || Box::new(<BoltActionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BoltActionDelay",
                name_hash: 322896091,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoltActionData, bolt_action_delay),
            },
            FieldInfoData {
                name: "BoltActionTime",
                name_hash: 1832454555,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoltActionData, bolt_action_time),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilFireRelease",
                name_hash: 832817818,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoltActionData, hold_bolt_action_until_fire_release),
            },
            FieldInfoData {
                name: "HoldBoltActionUntilZoomRelease",
                name_hash: 3133940949,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoltActionData, hold_bolt_action_until_zoom_release),
            },
            FieldInfoData {
                name: "ForceBoltActionOnFireTrigger",
                name_hash: 1053477586,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoltActionData, force_bolt_action_on_fire_trigger),
            },
            FieldInfoData {
                name: "UnZoomOnBoltAction",
                name_hash: 1438634691,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoltActionData, un_zoom_on_bolt_action),
            },
            FieldInfoData {
                name: "ReturnToZoomAfterBoltAction",
                name_hash: 3074155596,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoltActionData, return_to_zoom_after_bolt_action),
            },
        ],
    }),
    array_type: Some(BOLTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoltActionData {
    fn type_info(&self) -> &'static TypeInfo {
        BOLTACTIONDATA_TYPE_INFO
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


pub static BOLTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoltActionData-Array",
    name_hash: 3114854474,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("BoltActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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

pub trait HoldAndReleaseDataTrait: TypeObject {
    fn max_hold_time(&self) -> &f32;
    fn max_hold_time_mut(&mut self) -> &mut f32;
    fn min_power_modifier(&self) -> &f32;
    fn min_power_modifier_mut(&mut self) -> &mut f32;
    fn max_power_modifier(&self) -> &f32;
    fn max_power_modifier_mut(&mut self) -> &mut f32;
    fn power_increase_per_second(&self) -> &f32;
    fn power_increase_per_second_mut(&mut self) -> &mut f32;
    fn delay(&self) -> &f32;
    fn delay_mut(&mut self) -> &mut f32;
    fn force_fire_when_killed_holding(&self) -> &bool;
    fn force_fire_when_killed_holding_mut(&mut self) -> &mut bool;
    fn killed_holding_power_modifier(&self) -> &f32;
    fn killed_holding_power_modifier_mut(&mut self) -> &mut f32;
    fn precise_replicated_power_modifier(&self) -> &bool;
    fn precise_replicated_power_modifier_mut(&mut self) -> &mut bool;
    fn secondary_action_cancel_hold_and_release(&self) -> &bool;
    fn secondary_action_cancel_hold_and_release_mut(&mut self) -> &mut bool;
    fn hold_and_release_when_zooming(&self) -> &bool;
    fn hold_and_release_when_zooming_mut(&mut self) -> &mut bool;
    fn hold_until_fire_release(&self) -> &bool;
    fn hold_until_fire_release_mut(&mut self) -> &mut bool;
}

impl HoldAndReleaseDataTrait for HoldAndReleaseData {
    fn max_hold_time(&self) -> &f32 {
        &self.max_hold_time
    }
    fn max_hold_time_mut(&mut self) -> &mut f32 {
        &mut self.max_hold_time
    }
    fn min_power_modifier(&self) -> &f32 {
        &self.min_power_modifier
    }
    fn min_power_modifier_mut(&mut self) -> &mut f32 {
        &mut self.min_power_modifier
    }
    fn max_power_modifier(&self) -> &f32 {
        &self.max_power_modifier
    }
    fn max_power_modifier_mut(&mut self) -> &mut f32 {
        &mut self.max_power_modifier
    }
    fn power_increase_per_second(&self) -> &f32 {
        &self.power_increase_per_second
    }
    fn power_increase_per_second_mut(&mut self) -> &mut f32 {
        &mut self.power_increase_per_second
    }
    fn delay(&self) -> &f32 {
        &self.delay
    }
    fn delay_mut(&mut self) -> &mut f32 {
        &mut self.delay
    }
    fn force_fire_when_killed_holding(&self) -> &bool {
        &self.force_fire_when_killed_holding
    }
    fn force_fire_when_killed_holding_mut(&mut self) -> &mut bool {
        &mut self.force_fire_when_killed_holding
    }
    fn killed_holding_power_modifier(&self) -> &f32 {
        &self.killed_holding_power_modifier
    }
    fn killed_holding_power_modifier_mut(&mut self) -> &mut f32 {
        &mut self.killed_holding_power_modifier
    }
    fn precise_replicated_power_modifier(&self) -> &bool {
        &self.precise_replicated_power_modifier
    }
    fn precise_replicated_power_modifier_mut(&mut self) -> &mut bool {
        &mut self.precise_replicated_power_modifier
    }
    fn secondary_action_cancel_hold_and_release(&self) -> &bool {
        &self.secondary_action_cancel_hold_and_release
    }
    fn secondary_action_cancel_hold_and_release_mut(&mut self) -> &mut bool {
        &mut self.secondary_action_cancel_hold_and_release
    }
    fn hold_and_release_when_zooming(&self) -> &bool {
        &self.hold_and_release_when_zooming
    }
    fn hold_and_release_when_zooming_mut(&mut self) -> &mut bool {
        &mut self.hold_and_release_when_zooming
    }
    fn hold_until_fire_release(&self) -> &bool {
        &self.hold_until_fire_release
    }
    fn hold_until_fire_release_mut(&mut self) -> &mut bool {
        &mut self.hold_until_fire_release
    }
}

pub static HOLDANDRELEASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HoldAndReleaseData",
    name_hash: 2011632792,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HoldAndReleaseData as Default>::default())),
            create_boxed: || Box::new(<HoldAndReleaseData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaxHoldTime",
                name_hash: 2026966059,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HoldAndReleaseData, max_hold_time),
            },
            FieldInfoData {
                name: "MinPowerModifier",
                name_hash: 1396894279,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HoldAndReleaseData, min_power_modifier),
            },
            FieldInfoData {
                name: "MaxPowerModifier",
                name_hash: 4200451609,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HoldAndReleaseData, max_power_modifier),
            },
            FieldInfoData {
                name: "PowerIncreasePerSecond",
                name_hash: 1198568329,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HoldAndReleaseData, power_increase_per_second),
            },
            FieldInfoData {
                name: "Delay",
                name_hash: 208768368,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HoldAndReleaseData, delay),
            },
            FieldInfoData {
                name: "ForceFireWhenKilledHolding",
                name_hash: 2172080088,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HoldAndReleaseData, force_fire_when_killed_holding),
            },
            FieldInfoData {
                name: "KilledHoldingPowerModifier",
                name_hash: 1213290625,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HoldAndReleaseData, killed_holding_power_modifier),
            },
            FieldInfoData {
                name: "PreciseReplicatedPowerModifier",
                name_hash: 3455547747,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HoldAndReleaseData, precise_replicated_power_modifier),
            },
            FieldInfoData {
                name: "SecondaryActionCancelHoldAndRelease",
                name_hash: 2330274122,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HoldAndReleaseData, secondary_action_cancel_hold_and_release),
            },
            FieldInfoData {
                name: "HoldAndReleaseWhenZooming",
                name_hash: 1712974923,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HoldAndReleaseData, hold_and_release_when_zooming),
            },
            FieldInfoData {
                name: "HoldUntilFireRelease",
                name_hash: 981745553,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(HoldAndReleaseData, hold_until_fire_release),
            },
        ],
    }),
    array_type: Some(HOLDANDRELEASEDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HoldAndReleaseData {
    fn type_info(&self) -> &'static TypeInfo {
        HOLDANDRELEASEDATA_TYPE_INFO
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


pub static HOLDANDRELEASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HoldAndReleaseData-Array",
    name_hash: 162873004,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("HoldAndReleaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ShotConfigData {
    pub initial_position: super::core::Vec3,
    pub initial_direction: super::core::Vec3,
    pub initial_direction_scale_by_pitch: Vec<BoxedTypeObject /* InitialDirectionScaleByPitchData */>,
    pub initial_speed: super::core::Vec3,
    pub initial_speed_scale_by_pitch: Vec<BoxedTypeObject /* InitialSpeedScaleByPitchData */>,
    pub inherit_weapon_speed_amount: f32,
    pub muzzle_explosion: Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>,
    pub projectile_data: Option<LockedTypeObject /* ProjectileEntityData */>,
    pub secondary_projectile_data: Option<LockedTypeObject /* ProjectileEntityData */>,
    pub projectile: Option<LockedTypeObject /* ProjectileBlueprint */>,
    pub secondary_projectile: Option<LockedTypeObject /* ProjectileBlueprint */>,
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

pub trait ShotConfigDataTrait: TypeObject {
    fn initial_position(&self) -> &super::core::Vec3;
    fn initial_position_mut(&mut self) -> &mut super::core::Vec3;
    fn initial_direction(&self) -> &super::core::Vec3;
    fn initial_direction_mut(&mut self) -> &mut super::core::Vec3;
    fn initial_direction_scale_by_pitch(&self) -> &Vec<BoxedTypeObject /* InitialDirectionScaleByPitchData */>;
    fn initial_direction_scale_by_pitch_mut(&mut self) -> &mut Vec<BoxedTypeObject /* InitialDirectionScaleByPitchData */>;
    fn initial_speed(&self) -> &super::core::Vec3;
    fn initial_speed_mut(&mut self) -> &mut super::core::Vec3;
    fn initial_speed_scale_by_pitch(&self) -> &Vec<BoxedTypeObject /* InitialSpeedScaleByPitchData */>;
    fn initial_speed_scale_by_pitch_mut(&mut self) -> &mut Vec<BoxedTypeObject /* InitialSpeedScaleByPitchData */>;
    fn inherit_weapon_speed_amount(&self) -> &f32;
    fn inherit_weapon_speed_amount_mut(&mut self) -> &mut f32;
    fn muzzle_explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn muzzle_explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn projectile_data(&self) -> &Option<LockedTypeObject /* ProjectileEntityData */>;
    fn projectile_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileEntityData */>;
    fn secondary_projectile_data(&self) -> &Option<LockedTypeObject /* ProjectileEntityData */>;
    fn secondary_projectile_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileEntityData */>;
    fn projectile(&self) -> &Option<LockedTypeObject /* ProjectileBlueprint */>;
    fn projectile_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileBlueprint */>;
    fn secondary_projectile(&self) -> &Option<LockedTypeObject /* ProjectileBlueprint */>;
    fn secondary_projectile_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileBlueprint */>;
    fn spawn_delay(&self) -> &f32;
    fn spawn_delay_mut(&mut self) -> &mut f32;
    fn number_of_bullets_per_shell(&self) -> &u32;
    fn number_of_bullets_per_shell_mut(&mut self) -> &mut u32;
    fn number_of_bullets_per_shot(&self) -> &u32;
    fn number_of_bullets_per_shot_mut(&mut self) -> &mut u32;
    fn number_of_bullets_per_burst(&self) -> &u32;
    fn number_of_bullets_per_burst_mut(&mut self) -> &mut u32;
    fn relative_target_aiming(&self) -> &bool;
    fn relative_target_aiming_mut(&mut self) -> &mut bool;
    fn force_spawn_to_camera(&self) -> &bool;
    fn force_spawn_to_camera_mut(&mut self) -> &mut bool;
    fn spawn_visual_at_weapon_bone(&self) -> &bool;
    fn spawn_visual_at_weapon_bone_mut(&mut self) -> &mut bool;
    fn weapon_bone(&self) -> &super::entity::GameplayBones;
    fn weapon_bone_mut(&mut self) -> &mut super::entity::GameplayBones;
    fn active_force_spawn_to_camera(&self) -> &bool;
    fn active_force_spawn_to_camera_mut(&mut self) -> &mut bool;
}

impl ShotConfigDataTrait for ShotConfigData {
    fn initial_position(&self) -> &super::core::Vec3 {
        &self.initial_position
    }
    fn initial_position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.initial_position
    }
    fn initial_direction(&self) -> &super::core::Vec3 {
        &self.initial_direction
    }
    fn initial_direction_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.initial_direction
    }
    fn initial_direction_scale_by_pitch(&self) -> &Vec<BoxedTypeObject /* InitialDirectionScaleByPitchData */> {
        &self.initial_direction_scale_by_pitch
    }
    fn initial_direction_scale_by_pitch_mut(&mut self) -> &mut Vec<BoxedTypeObject /* InitialDirectionScaleByPitchData */> {
        &mut self.initial_direction_scale_by_pitch
    }
    fn initial_speed(&self) -> &super::core::Vec3 {
        &self.initial_speed
    }
    fn initial_speed_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.initial_speed
    }
    fn initial_speed_scale_by_pitch(&self) -> &Vec<BoxedTypeObject /* InitialSpeedScaleByPitchData */> {
        &self.initial_speed_scale_by_pitch
    }
    fn initial_speed_scale_by_pitch_mut(&mut self) -> &mut Vec<BoxedTypeObject /* InitialSpeedScaleByPitchData */> {
        &mut self.initial_speed_scale_by_pitch
    }
    fn inherit_weapon_speed_amount(&self) -> &f32 {
        &self.inherit_weapon_speed_amount
    }
    fn inherit_weapon_speed_amount_mut(&mut self) -> &mut f32 {
        &mut self.inherit_weapon_speed_amount
    }
    fn muzzle_explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &self.muzzle_explosion
    }
    fn muzzle_explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &mut self.muzzle_explosion
    }
    fn projectile_data(&self) -> &Option<LockedTypeObject /* ProjectileEntityData */> {
        &self.projectile_data
    }
    fn projectile_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileEntityData */> {
        &mut self.projectile_data
    }
    fn secondary_projectile_data(&self) -> &Option<LockedTypeObject /* ProjectileEntityData */> {
        &self.secondary_projectile_data
    }
    fn secondary_projectile_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileEntityData */> {
        &mut self.secondary_projectile_data
    }
    fn projectile(&self) -> &Option<LockedTypeObject /* ProjectileBlueprint */> {
        &self.projectile
    }
    fn projectile_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileBlueprint */> {
        &mut self.projectile
    }
    fn secondary_projectile(&self) -> &Option<LockedTypeObject /* ProjectileBlueprint */> {
        &self.secondary_projectile
    }
    fn secondary_projectile_mut(&mut self) -> &mut Option<LockedTypeObject /* ProjectileBlueprint */> {
        &mut self.secondary_projectile
    }
    fn spawn_delay(&self) -> &f32 {
        &self.spawn_delay
    }
    fn spawn_delay_mut(&mut self) -> &mut f32 {
        &mut self.spawn_delay
    }
    fn number_of_bullets_per_shell(&self) -> &u32 {
        &self.number_of_bullets_per_shell
    }
    fn number_of_bullets_per_shell_mut(&mut self) -> &mut u32 {
        &mut self.number_of_bullets_per_shell
    }
    fn number_of_bullets_per_shot(&self) -> &u32 {
        &self.number_of_bullets_per_shot
    }
    fn number_of_bullets_per_shot_mut(&mut self) -> &mut u32 {
        &mut self.number_of_bullets_per_shot
    }
    fn number_of_bullets_per_burst(&self) -> &u32 {
        &self.number_of_bullets_per_burst
    }
    fn number_of_bullets_per_burst_mut(&mut self) -> &mut u32 {
        &mut self.number_of_bullets_per_burst
    }
    fn relative_target_aiming(&self) -> &bool {
        &self.relative_target_aiming
    }
    fn relative_target_aiming_mut(&mut self) -> &mut bool {
        &mut self.relative_target_aiming
    }
    fn force_spawn_to_camera(&self) -> &bool {
        &self.force_spawn_to_camera
    }
    fn force_spawn_to_camera_mut(&mut self) -> &mut bool {
        &mut self.force_spawn_to_camera
    }
    fn spawn_visual_at_weapon_bone(&self) -> &bool {
        &self.spawn_visual_at_weapon_bone
    }
    fn spawn_visual_at_weapon_bone_mut(&mut self) -> &mut bool {
        &mut self.spawn_visual_at_weapon_bone
    }
    fn weapon_bone(&self) -> &super::entity::GameplayBones {
        &self.weapon_bone
    }
    fn weapon_bone_mut(&mut self) -> &mut super::entity::GameplayBones {
        &mut self.weapon_bone
    }
    fn active_force_spawn_to_camera(&self) -> &bool {
        &self.active_force_spawn_to_camera
    }
    fn active_force_spawn_to_camera_mut(&mut self) -> &mut bool {
        &mut self.active_force_spawn_to_camera
    }
}

pub static SHOTCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShotConfigData",
    name_hash: 38484159,
    flags: MemberInfoFlags::new(73),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ShotConfigData as Default>::default())),
            create_boxed: || Box::new(<ShotConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InitialPosition",
                name_hash: 3791803234,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ShotConfigData, initial_position),
            },
            FieldInfoData {
                name: "InitialDirection",
                name_hash: 1073214622,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ShotConfigData, initial_direction),
            },
            FieldInfoData {
                name: "InitialDirectionScaleByPitch",
                name_hash: 314764859,
                flags: MemberInfoFlags::new(144),
                field_type: "InitialDirectionScaleByPitchData-Array",
                rust_offset: offset_of!(ShotConfigData, initial_direction_scale_by_pitch),
            },
            FieldInfoData {
                name: "InitialSpeed",
                name_hash: 3681055580,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ShotConfigData, initial_speed),
            },
            FieldInfoData {
                name: "InitialSpeedScaleByPitch",
                name_hash: 4049759481,
                flags: MemberInfoFlags::new(144),
                field_type: "InitialSpeedScaleByPitchData-Array",
                rust_offset: offset_of!(ShotConfigData, initial_speed_scale_by_pitch),
            },
            FieldInfoData {
                name: "InheritWeaponSpeedAmount",
                name_hash: 3681006953,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShotConfigData, inherit_weapon_speed_amount),
            },
            FieldInfoData {
                name: "MuzzleExplosion",
                name_hash: 2257917377,
                flags: MemberInfoFlags::new(0),
                field_type: "ExplosionEntityData",
                rust_offset: offset_of!(ShotConfigData, muzzle_explosion),
            },
            FieldInfoData {
                name: "ProjectileData",
                name_hash: 2690942720,
                flags: MemberInfoFlags::new(0),
                field_type: "ProjectileEntityData",
                rust_offset: offset_of!(ShotConfigData, projectile_data),
            },
            FieldInfoData {
                name: "SecondaryProjectileData",
                name_hash: 2405794234,
                flags: MemberInfoFlags::new(0),
                field_type: "ProjectileEntityData",
                rust_offset: offset_of!(ShotConfigData, secondary_projectile_data),
            },
            FieldInfoData {
                name: "Projectile",
                name_hash: 3883939888,
                flags: MemberInfoFlags::new(0),
                field_type: "ProjectileBlueprint",
                rust_offset: offset_of!(ShotConfigData, projectile),
            },
            FieldInfoData {
                name: "SecondaryProjectile",
                name_hash: 2606311818,
                flags: MemberInfoFlags::new(0),
                field_type: "ProjectileBlueprint",
                rust_offset: offset_of!(ShotConfigData, secondary_projectile),
            },
            FieldInfoData {
                name: "SpawnDelay",
                name_hash: 3473198411,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ShotConfigData, spawn_delay),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShell",
                name_hash: 464437027,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ShotConfigData, number_of_bullets_per_shell),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerShot",
                name_hash: 404525501,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ShotConfigData, number_of_bullets_per_shot),
            },
            FieldInfoData {
                name: "NumberOfBulletsPerBurst",
                name_hash: 491113727,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ShotConfigData, number_of_bullets_per_burst),
            },
            FieldInfoData {
                name: "RelativeTargetAiming",
                name_hash: 3309775045,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShotConfigData, relative_target_aiming),
            },
            FieldInfoData {
                name: "ForceSpawnToCamera",
                name_hash: 1364356545,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShotConfigData, force_spawn_to_camera),
            },
            FieldInfoData {
                name: "SpawnVisualAtWeaponBone",
                name_hash: 3532019419,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShotConfigData, spawn_visual_at_weapon_bone),
            },
            FieldInfoData {
                name: "WeaponBone",
                name_hash: 932584161,
                flags: MemberInfoFlags::new(0),
                field_type: "GameplayBones",
                rust_offset: offset_of!(ShotConfigData, weapon_bone),
            },
            FieldInfoData {
                name: "ActiveForceSpawnToCamera",
                name_hash: 2063799309,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ShotConfigData, active_force_spawn_to_camera),
            },
        ],
    }),
    array_type: Some(SHOTCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ShotConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        SHOTCONFIGDATA_TYPE_INFO
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


pub static SHOTCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShotConfigData-Array",
    name_hash: 2727554571,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ShotConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InitialSpeedScaleByPitchData {
    pub pitch: f32,
    pub initial_speed_scale: super::core::Vec3,
}

pub trait InitialSpeedScaleByPitchDataTrait: TypeObject {
    fn pitch(&self) -> &f32;
    fn pitch_mut(&mut self) -> &mut f32;
    fn initial_speed_scale(&self) -> &super::core::Vec3;
    fn initial_speed_scale_mut(&mut self) -> &mut super::core::Vec3;
}

impl InitialSpeedScaleByPitchDataTrait for InitialSpeedScaleByPitchData {
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut f32 {
        &mut self.pitch
    }
    fn initial_speed_scale(&self) -> &super::core::Vec3 {
        &self.initial_speed_scale
    }
    fn initial_speed_scale_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.initial_speed_scale
    }
}

pub static INITIALSPEEDSCALEBYPITCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialSpeedScaleByPitchData",
    name_hash: 2255878153,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InitialSpeedScaleByPitchData as Default>::default())),
            create_boxed: || Box::new(<InitialSpeedScaleByPitchData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InitialSpeedScaleByPitchData, pitch),
            },
            FieldInfoData {
                name: "InitialSpeedScale",
                name_hash: 1318906980,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(InitialSpeedScaleByPitchData, initial_speed_scale),
            },
        ],
    }),
    array_type: Some(INITIALSPEEDSCALEBYPITCHDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for InitialSpeedScaleByPitchData {
    fn type_info(&self) -> &'static TypeInfo {
        INITIALSPEEDSCALEBYPITCHDATA_TYPE_INFO
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


pub static INITIALSPEEDSCALEBYPITCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialSpeedScaleByPitchData-Array",
    name_hash: 2549665213,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("InitialSpeedScaleByPitchData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InitialDirectionScaleByPitchData {
    pub pitch: f32,
    pub initial_direction_scale: super::core::Vec3,
}

pub trait InitialDirectionScaleByPitchDataTrait: TypeObject {
    fn pitch(&self) -> &f32;
    fn pitch_mut(&mut self) -> &mut f32;
    fn initial_direction_scale(&self) -> &super::core::Vec3;
    fn initial_direction_scale_mut(&mut self) -> &mut super::core::Vec3;
}

impl InitialDirectionScaleByPitchDataTrait for InitialDirectionScaleByPitchData {
    fn pitch(&self) -> &f32 {
        &self.pitch
    }
    fn pitch_mut(&mut self) -> &mut f32 {
        &mut self.pitch
    }
    fn initial_direction_scale(&self) -> &super::core::Vec3 {
        &self.initial_direction_scale
    }
    fn initial_direction_scale_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.initial_direction_scale
    }
}

pub static INITIALDIRECTIONSCALEBYPITCHDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialDirectionScaleByPitchData",
    name_hash: 2061200203,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InitialDirectionScaleByPitchData as Default>::default())),
            create_boxed: || Box::new(<InitialDirectionScaleByPitchData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Pitch",
                name_hash: 232604323,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InitialDirectionScaleByPitchData, pitch),
            },
            FieldInfoData {
                name: "InitialDirectionScale",
                name_hash: 1744402022,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(InitialDirectionScaleByPitchData, initial_direction_scale),
            },
        ],
    }),
    array_type: Some(INITIALDIRECTIONSCALEBYPITCHDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for InitialDirectionScaleByPitchData {
    fn type_info(&self) -> &'static TypeInfo {
        INITIALDIRECTIONSCALEBYPITCHDATA_TYPE_INFO
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


pub static INITIALDIRECTIONSCALEBYPITCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InitialDirectionScaleByPitchData-Array",
    name_hash: 3168791167,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("InitialDirectionScaleByPitchData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealingSphereData {
    pub radius: f32,
    pub health_inc_speed: f32,
}

pub trait HealingSphereDataTrait: TypeObject {
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn health_inc_speed(&self) -> &f32;
    fn health_inc_speed_mut(&mut self) -> &mut f32;
}

impl HealingSphereDataTrait for HealingSphereData {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn health_inc_speed(&self) -> &f32 {
        &self.health_inc_speed
    }
    fn health_inc_speed_mut(&mut self) -> &mut f32 {
        &mut self.health_inc_speed
    }
}

pub static HEALINGSPHEREDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealingSphereData",
    name_hash: 3087727980,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealingSphereData as Default>::default())),
            create_boxed: || Box::new(<HealingSphereData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealingSphereData, radius),
            },
            FieldInfoData {
                name: "HealthIncSpeed",
                name_hash: 2425137914,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(HealingSphereData, health_inc_speed),
            },
        ],
    }),
    array_type: Some(HEALINGSPHEREDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for HealingSphereData {
    fn type_info(&self) -> &'static TypeInfo {
        HEALINGSPHEREDATA_TYPE_INFO
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


pub static HEALINGSPHEREDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealingSphereData-Array",
    name_hash: 1985805656,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("HealingSphereData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MissileEntityData {
    pub _glacier_base: GhostedProjectileEntityData,
    pub engine_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
    pub dud_explosion: Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>,
    pub fly_by_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
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
    pub locking_controller: Option<LockedTypeObject /* LockingControllerData */>,
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

pub trait MissileEntityDataTrait: GhostedProjectileEntityDataTrait {
    fn engine_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn engine_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn dud_explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn dud_explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn fly_by_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn fly_by_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn engine_strength(&self) -> &f32;
    fn engine_strength_mut(&mut self) -> &mut f32;
    fn max_speed(&self) -> &f32;
    fn max_speed_mut(&mut self) -> &mut f32;
    fn engine_time_to_ignition(&self) -> &f32;
    fn engine_time_to_ignition_mut(&mut self) -> &mut f32;
    fn engine_time_to_live(&self) -> &f32;
    fn engine_time_to_live_mut(&mut self) -> &mut f32;
    fn time_to_activate_guiding_system(&self) -> &f32;
    fn time_to_activate_guiding_system_mut(&mut self) -> &mut f32;
    fn time_to_arm(&self) -> &f32;
    fn time_to_arm_mut(&mut self) -> &mut f32;
    fn max_turn_angle(&self) -> &f32;
    fn max_turn_angle_mut(&mut self) -> &mut f32;
    fn min_turn_angle(&self) -> &f32;
    fn min_turn_angle_mut(&mut self) -> &mut f32;
    fn turn_angle_multiplier(&self) -> &f32;
    fn turn_angle_multiplier_mut(&mut self) -> &mut f32;
    fn turn_y_first(&self) -> &bool;
    fn turn_y_first_mut(&mut self) -> &mut bool;
    fn drag(&self) -> &f32;
    fn drag_mut(&mut self) -> &mut f32;
    fn gravity(&self) -> &f32;
    fn gravity_mut(&mut self) -> &mut f32;
    fn apply_gravity_when_guided(&self) -> &bool;
    fn apply_gravity_when_guided_mut(&mut self) -> &mut bool;
    fn fly_by_sound_radius(&self) -> &f32;
    fn fly_by_sound_radius_mut(&mut self) -> &mut f32;
    fn fly_by_sound_speed(&self) -> &f32;
    fn fly_by_sound_speed_mut(&mut self) -> &mut f32;
    fn impact_impulse(&self) -> &f32;
    fn impact_impulse_mut(&mut self) -> &mut f32;
    fn damage(&self) -> &f32;
    fn damage_mut(&mut self) -> &mut f32;
    fn default_team(&self) -> &super::gameplay_sim::TeamId;
    fn default_team_mut(&mut self) -> &mut super::gameplay_sim::TeamId;
    fn warn_target(&self) -> &WarnTarget;
    fn warn_target_mut(&mut self) -> &mut WarnTarget;
    fn warn_on_pointing_missile(&self) -> &bool;
    fn warn_on_pointing_missile_mut(&mut self) -> &mut bool;
    fn locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */>;
    fn locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */>;
    fn lockable_info(&self) -> &MissileLockableInfoData;
    fn lockable_info_mut(&mut self) -> &mut MissileLockableInfoData;
    fn unguided_data(&self) -> &MissileUnguidedData;
    fn unguided_data_mut(&mut self) -> &mut MissileUnguidedData;
    fn near_target_detonation(&self) -> &NearTargetDetonationData;
    fn near_target_detonation_mut(&mut self) -> &mut NearTargetDetonationData;
    fn enable_banking(&self) -> &bool;
    fn enable_banking_mut(&mut self) -> &mut bool;
    fn max_bank_angle(&self) -> &f32;
    fn max_bank_angle_mut(&mut self) -> &mut f32;
    fn banking_speed(&self) -> &f32;
    fn banking_speed_mut(&mut self) -> &mut f32;
    fn icon(&self) -> &String;
    fn icon_mut(&mut self) -> &mut String;
    fn target_icon(&self) -> &String;
    fn target_icon_mut(&mut self) -> &mut String;
    fn target_icon_enemy(&self) -> &String;
    fn target_icon_enemy_mut(&mut self) -> &mut String;
    fn min_ghost_frequency(&self) -> &f32;
    fn min_ghost_frequency_mut(&mut self) -> &mut f32;
    fn start_effects_on_spawn(&self) -> &bool;
    fn start_effects_on_spawn_mut(&mut self) -> &mut bool;
    fn is_bullet_collision(&self) -> &bool;
    fn is_bullet_collision_mut(&mut self) -> &mut bool;
    fn extrapolate_acceleration(&self) -> &bool;
    fn extrapolate_acceleration_mut(&mut self) -> &mut bool;
    fn calculate_position_based_on_velocity(&self) -> &bool;
    fn calculate_position_based_on_velocity_mut(&mut self) -> &mut bool;
}

impl MissileEntityDataTrait for MissileEntityData {
    fn engine_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.engine_effect
    }
    fn engine_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.engine_effect
    }
    fn dud_explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &self.dud_explosion
    }
    fn dud_explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &mut self.dud_explosion
    }
    fn fly_by_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.fly_by_sound
    }
    fn fly_by_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.fly_by_sound
    }
    fn engine_strength(&self) -> &f32 {
        &self.engine_strength
    }
    fn engine_strength_mut(&mut self) -> &mut f32 {
        &mut self.engine_strength
    }
    fn max_speed(&self) -> &f32 {
        &self.max_speed
    }
    fn max_speed_mut(&mut self) -> &mut f32 {
        &mut self.max_speed
    }
    fn engine_time_to_ignition(&self) -> &f32 {
        &self.engine_time_to_ignition
    }
    fn engine_time_to_ignition_mut(&mut self) -> &mut f32 {
        &mut self.engine_time_to_ignition
    }
    fn engine_time_to_live(&self) -> &f32 {
        &self.engine_time_to_live
    }
    fn engine_time_to_live_mut(&mut self) -> &mut f32 {
        &mut self.engine_time_to_live
    }
    fn time_to_activate_guiding_system(&self) -> &f32 {
        &self.time_to_activate_guiding_system
    }
    fn time_to_activate_guiding_system_mut(&mut self) -> &mut f32 {
        &mut self.time_to_activate_guiding_system
    }
    fn time_to_arm(&self) -> &f32 {
        &self.time_to_arm
    }
    fn time_to_arm_mut(&mut self) -> &mut f32 {
        &mut self.time_to_arm
    }
    fn max_turn_angle(&self) -> &f32 {
        &self.max_turn_angle
    }
    fn max_turn_angle_mut(&mut self) -> &mut f32 {
        &mut self.max_turn_angle
    }
    fn min_turn_angle(&self) -> &f32 {
        &self.min_turn_angle
    }
    fn min_turn_angle_mut(&mut self) -> &mut f32 {
        &mut self.min_turn_angle
    }
    fn turn_angle_multiplier(&self) -> &f32 {
        &self.turn_angle_multiplier
    }
    fn turn_angle_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.turn_angle_multiplier
    }
    fn turn_y_first(&self) -> &bool {
        &self.turn_y_first
    }
    fn turn_y_first_mut(&mut self) -> &mut bool {
        &mut self.turn_y_first
    }
    fn drag(&self) -> &f32 {
        &self.drag
    }
    fn drag_mut(&mut self) -> &mut f32 {
        &mut self.drag
    }
    fn gravity(&self) -> &f32 {
        &self.gravity
    }
    fn gravity_mut(&mut self) -> &mut f32 {
        &mut self.gravity
    }
    fn apply_gravity_when_guided(&self) -> &bool {
        &self.apply_gravity_when_guided
    }
    fn apply_gravity_when_guided_mut(&mut self) -> &mut bool {
        &mut self.apply_gravity_when_guided
    }
    fn fly_by_sound_radius(&self) -> &f32 {
        &self.fly_by_sound_radius
    }
    fn fly_by_sound_radius_mut(&mut self) -> &mut f32 {
        &mut self.fly_by_sound_radius
    }
    fn fly_by_sound_speed(&self) -> &f32 {
        &self.fly_by_sound_speed
    }
    fn fly_by_sound_speed_mut(&mut self) -> &mut f32 {
        &mut self.fly_by_sound_speed
    }
    fn impact_impulse(&self) -> &f32 {
        &self.impact_impulse
    }
    fn impact_impulse_mut(&mut self) -> &mut f32 {
        &mut self.impact_impulse
    }
    fn damage(&self) -> &f32 {
        &self.damage
    }
    fn damage_mut(&mut self) -> &mut f32 {
        &mut self.damage
    }
    fn default_team(&self) -> &super::gameplay_sim::TeamId {
        &self.default_team
    }
    fn default_team_mut(&mut self) -> &mut super::gameplay_sim::TeamId {
        &mut self.default_team
    }
    fn warn_target(&self) -> &WarnTarget {
        &self.warn_target
    }
    fn warn_target_mut(&mut self) -> &mut WarnTarget {
        &mut self.warn_target
    }
    fn warn_on_pointing_missile(&self) -> &bool {
        &self.warn_on_pointing_missile
    }
    fn warn_on_pointing_missile_mut(&mut self) -> &mut bool {
        &mut self.warn_on_pointing_missile
    }
    fn locking_controller(&self) -> &Option<LockedTypeObject /* LockingControllerData */> {
        &self.locking_controller
    }
    fn locking_controller_mut(&mut self) -> &mut Option<LockedTypeObject /* LockingControllerData */> {
        &mut self.locking_controller
    }
    fn lockable_info(&self) -> &MissileLockableInfoData {
        &self.lockable_info
    }
    fn lockable_info_mut(&mut self) -> &mut MissileLockableInfoData {
        &mut self.lockable_info
    }
    fn unguided_data(&self) -> &MissileUnguidedData {
        &self.unguided_data
    }
    fn unguided_data_mut(&mut self) -> &mut MissileUnguidedData {
        &mut self.unguided_data
    }
    fn near_target_detonation(&self) -> &NearTargetDetonationData {
        &self.near_target_detonation
    }
    fn near_target_detonation_mut(&mut self) -> &mut NearTargetDetonationData {
        &mut self.near_target_detonation
    }
    fn enable_banking(&self) -> &bool {
        &self.enable_banking
    }
    fn enable_banking_mut(&mut self) -> &mut bool {
        &mut self.enable_banking
    }
    fn max_bank_angle(&self) -> &f32 {
        &self.max_bank_angle
    }
    fn max_bank_angle_mut(&mut self) -> &mut f32 {
        &mut self.max_bank_angle
    }
    fn banking_speed(&self) -> &f32 {
        &self.banking_speed
    }
    fn banking_speed_mut(&mut self) -> &mut f32 {
        &mut self.banking_speed
    }
    fn icon(&self) -> &String {
        &self.icon
    }
    fn icon_mut(&mut self) -> &mut String {
        &mut self.icon
    }
    fn target_icon(&self) -> &String {
        &self.target_icon
    }
    fn target_icon_mut(&mut self) -> &mut String {
        &mut self.target_icon
    }
    fn target_icon_enemy(&self) -> &String {
        &self.target_icon_enemy
    }
    fn target_icon_enemy_mut(&mut self) -> &mut String {
        &mut self.target_icon_enemy
    }
    fn min_ghost_frequency(&self) -> &f32 {
        &self.min_ghost_frequency
    }
    fn min_ghost_frequency_mut(&mut self) -> &mut f32 {
        &mut self.min_ghost_frequency
    }
    fn start_effects_on_spawn(&self) -> &bool {
        &self.start_effects_on_spawn
    }
    fn start_effects_on_spawn_mut(&mut self) -> &mut bool {
        &mut self.start_effects_on_spawn
    }
    fn is_bullet_collision(&self) -> &bool {
        &self.is_bullet_collision
    }
    fn is_bullet_collision_mut(&mut self) -> &mut bool {
        &mut self.is_bullet_collision
    }
    fn extrapolate_acceleration(&self) -> &bool {
        &self.extrapolate_acceleration
    }
    fn extrapolate_acceleration_mut(&mut self) -> &mut bool {
        &mut self.extrapolate_acceleration
    }
    fn calculate_position_based_on_velocity(&self) -> &bool {
        &self.calculate_position_based_on_velocity
    }
    fn calculate_position_based_on_velocity_mut(&mut self) -> &mut bool {
        &mut self.calculate_position_based_on_velocity
    }
}

impl GhostedProjectileEntityDataTrait for MissileEntityData {
    fn proxy_convergence_delay(&self) -> &f32 {
        self._glacier_base.proxy_convergence_delay()
    }
    fn proxy_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_convergence_delay_mut()
    }
    fn proxy_convergence_duration(&self) -> &f32 {
        self._glacier_base.proxy_convergence_duration()
    }
    fn proxy_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_convergence_duration_mut()
    }
    fn proxy_convergence_instant_on_attach(&self) -> &bool {
        self._glacier_base.proxy_convergence_instant_on_attach()
    }
    fn proxy_convergence_instant_on_attach_mut(&mut self) -> &mut bool {
        self._glacier_base.proxy_convergence_instant_on_attach_mut()
    }
    fn force_proxy_convergence(&self) -> &bool {
        self._glacier_base.force_proxy_convergence()
    }
    fn force_proxy_convergence_mut(&mut self) -> &mut bool {
        self._glacier_base.force_proxy_convergence_mut()
    }
    fn convergence_using_initial_speed(&self) -> &bool {
        self._glacier_base.convergence_using_initial_speed()
    }
    fn convergence_using_initial_speed_mut(&mut self) -> &mut bool {
        self._glacier_base.convergence_using_initial_speed_mut()
    }
}

impl MeshProjectileEntityDataTrait for MissileEntityData {
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh()
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh_mut()
    }
    fn trail_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        self._glacier_base.trail_effect()
    }
    fn trail_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        self._glacier_base.trail_effect_mut()
    }
    fn is_attachable(&self) -> &bool {
        self._glacier_base.is_attachable()
    }
    fn is_attachable_mut(&mut self) -> &mut bool {
        self._glacier_base.is_attachable_mut()
    }
    fn instant_attachable_test_distance(&self) -> &f32 {
        self._glacier_base.instant_attachable_test_distance()
    }
    fn instant_attachable_test_distance_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_test_distance_mut()
    }
    fn instant_attachable_visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.instant_attachable_visual_convergence_delay()
    }
    fn instant_attachable_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_visual_convergence_delay_mut()
    }
    fn instant_attachable_visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.instant_attachable_visual_convergence_duration()
    }
    fn instant_attachable_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_visual_convergence_duration_mut()
    }
    fn instant_attachable_test_under_reticule(&self) -> &bool {
        self._glacier_base.instant_attachable_test_under_reticule()
    }
    fn instant_attachable_test_under_reticule_mut(&mut self) -> &mut bool {
        self._glacier_base.instant_attachable_test_under_reticule_mut()
    }
    fn max_attachable_inclination(&self) -> &f32 {
        self._glacier_base.max_attachable_inclination()
    }
    fn max_attachable_inclination_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_attachable_inclination_mut()
    }
    fn extra_damping(&self) -> &bool {
        self._glacier_base.extra_damping()
    }
    fn extra_damping_mut(&mut self) -> &mut bool {
        self._glacier_base.extra_damping_mut()
    }
    fn initial_angular_velocity(&self) -> &super::core::Vec3 {
        self._glacier_base.initial_angular_velocity()
    }
    fn initial_angular_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.initial_angular_velocity_mut()
    }
    fn unspawn_after_detonation_delay(&self) -> &f32 {
        self._glacier_base.unspawn_after_detonation_delay()
    }
    fn unspawn_after_detonation_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.unspawn_after_detonation_delay_mut()
    }
}

impl ProjectileEntityDataTrait for MissileEntityData {
    fn initial_speed(&self) -> &f32 {
        self._glacier_base.initial_speed()
    }
    fn initial_speed_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_speed_mut()
    }
    fn time_to_live(&self) -> &f32 {
        self._glacier_base.time_to_live()
    }
    fn time_to_live_mut(&mut self) -> &mut f32 {
        self._glacier_base.time_to_live_mut()
    }
    fn max_count(&self) -> &u32 {
        self._glacier_base.max_count()
    }
    fn max_count_mut(&mut self) -> &mut u32 {
        self._glacier_base.max_count_mut()
    }
    fn init_mesh_hide_time(&self) -> &f32 {
        self._glacier_base.init_mesh_hide_time()
    }
    fn init_mesh_hide_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.init_mesh_hide_time_mut()
    }
    fn visual_converge_distance(&self) -> &f32 {
        self._glacier_base.visual_converge_distance()
    }
    fn visual_converge_distance_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_converge_distance_mut()
    }
    fn visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.visual_convergence_delay()
    }
    fn visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_delay_mut()
    }
    fn visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.visual_convergence_duration()
    }
    fn visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_duration_mut()
    }
    fn proxy_visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_delay()
    }
    fn proxy_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_delay_mut()
    }
    fn proxy_visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_duration()
    }
    fn proxy_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_duration_mut()
    }
    fn detonate_on_timeout(&self) -> &bool {
        self._glacier_base.detonate_on_timeout()
    }
    fn detonate_on_timeout_mut(&mut self) -> &mut bool {
        self._glacier_base.detonate_on_timeout_mut()
    }
    fn server_projectile_disabled(&self) -> &bool {
        self._glacier_base.server_projectile_disabled()
    }
    fn server_projectile_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.server_projectile_disabled_mut()
    }
    fn explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion()
    }
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion_mut()
    }
    fn suppression_data(&self) -> &Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data()
    }
    fn suppression_data_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data_mut()
    }
    fn ammunition_type(&self) -> &String {
        self._glacier_base.ammunition_type()
    }
    fn ammunition_type_mut(&mut self) -> &mut String {
        self._glacier_base.ammunition_type_mut()
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        self._glacier_base.material_pair()
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        self._glacier_base.material_pair_mut()
    }
    fn hit_reaction_weapon_type(&self) -> &AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type()
    }
    fn hit_reaction_weapon_type_mut(&mut self) -> &mut AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type_mut()
    }
    fn hide_on_detonation(&self) -> &bool {
        self._glacier_base.hide_on_detonation()
    }
    fn hide_on_detonation_mut(&mut self) -> &mut bool {
        self._glacier_base.hide_on_detonation_mut()
    }
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info()
    }
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info_mut()
    }
}

impl super::physics::GamePhysicsEntityDataTrait for MissileEntityData {
}

impl super::entity::GameComponentEntityDataTrait for MissileEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for MissileEntityData {
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

impl super::entity::SpatialEntityDataTrait for MissileEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for MissileEntityData {
}

impl super::entity::GameObjectDataTrait for MissileEntityData {
}

impl super::core::DataBusPeerTrait for MissileEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MissileEntityData {
}

impl super::core::DataContainerTrait for MissileEntityData {
}

pub static MISSILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileEntityData",
    name_hash: 3593468682,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GHOSTEDPROJECTILEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MissileEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MissileEntityData as Default>::default())),
            create_boxed: || Box::new(<MissileEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EngineEffect",
                name_hash: 2238351068,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MissileEntityData, engine_effect),
            },
            FieldInfoData {
                name: "DudExplosion",
                name_hash: 123118469,
                flags: MemberInfoFlags::new(0),
                field_type: "ExplosionEntityData",
                rust_offset: offset_of!(MissileEntityData, dud_explosion),
            },
            FieldInfoData {
                name: "FlyBySound",
                name_hash: 3743063886,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(MissileEntityData, fly_by_sound),
            },
            FieldInfoData {
                name: "EngineStrength",
                name_hash: 1553827310,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, engine_strength),
            },
            FieldInfoData {
                name: "MaxSpeed",
                name_hash: 396228950,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, max_speed),
            },
            FieldInfoData {
                name: "EngineTimeToIgnition",
                name_hash: 916282736,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, engine_time_to_ignition),
            },
            FieldInfoData {
                name: "EngineTimeToLive",
                name_hash: 41196403,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, engine_time_to_live),
            },
            FieldInfoData {
                name: "TimeToActivateGuidingSystem",
                name_hash: 2537799752,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, time_to_activate_guiding_system),
            },
            FieldInfoData {
                name: "TimeToArm",
                name_hash: 177776085,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, time_to_arm),
            },
            FieldInfoData {
                name: "MaxTurnAngle",
                name_hash: 2995598829,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, max_turn_angle),
            },
            FieldInfoData {
                name: "MinTurnAngle",
                name_hash: 1816933171,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, min_turn_angle),
            },
            FieldInfoData {
                name: "TurnAngleMultiplier",
                name_hash: 3783836082,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, turn_angle_multiplier),
            },
            FieldInfoData {
                name: "TurnYFirst",
                name_hash: 2423585435,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, turn_y_first),
            },
            FieldInfoData {
                name: "Drag",
                name_hash: 2088752789,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, drag),
            },
            FieldInfoData {
                name: "Gravity",
                name_hash: 3295092963,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, gravity),
            },
            FieldInfoData {
                name: "ApplyGravityWhenGuided",
                name_hash: 407646333,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, apply_gravity_when_guided),
            },
            FieldInfoData {
                name: "FlyBySoundRadius",
                name_hash: 3728826774,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, fly_by_sound_radius),
            },
            FieldInfoData {
                name: "FlyBySoundSpeed",
                name_hash: 4276155561,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, fly_by_sound_speed),
            },
            FieldInfoData {
                name: "ImpactImpulse",
                name_hash: 59311612,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, impact_impulse),
            },
            FieldInfoData {
                name: "Damage",
                name_hash: 2589892334,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, damage),
            },
            FieldInfoData {
                name: "DefaultTeam",
                name_hash: 2015081331,
                flags: MemberInfoFlags::new(0),
                field_type: "TeamId",
                rust_offset: offset_of!(MissileEntityData, default_team),
            },
            FieldInfoData {
                name: "WarnTarget",
                name_hash: 1103784926,
                flags: MemberInfoFlags::new(0),
                field_type: "WarnTarget",
                rust_offset: offset_of!(MissileEntityData, warn_target),
            },
            FieldInfoData {
                name: "WarnOnPointingMissile",
                name_hash: 288907718,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, warn_on_pointing_missile),
            },
            FieldInfoData {
                name: "LockingController",
                name_hash: 1783566994,
                flags: MemberInfoFlags::new(0),
                field_type: "LockingControllerData",
                rust_offset: offset_of!(MissileEntityData, locking_controller),
            },
            FieldInfoData {
                name: "LockableInfo",
                name_hash: 632168042,
                flags: MemberInfoFlags::new(0),
                field_type: "MissileLockableInfoData",
                rust_offset: offset_of!(MissileEntityData, lockable_info),
            },
            FieldInfoData {
                name: "UnguidedData",
                name_hash: 952180624,
                flags: MemberInfoFlags::new(0),
                field_type: "MissileUnguidedData",
                rust_offset: offset_of!(MissileEntityData, unguided_data),
            },
            FieldInfoData {
                name: "NearTargetDetonation",
                name_hash: 3676526149,
                flags: MemberInfoFlags::new(0),
                field_type: "NearTargetDetonationData",
                rust_offset: offset_of!(MissileEntityData, near_target_detonation),
            },
            FieldInfoData {
                name: "EnableBanking",
                name_hash: 1830158498,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, enable_banking),
            },
            FieldInfoData {
                name: "MaxBankAngle",
                name_hash: 306549910,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, max_bank_angle),
            },
            FieldInfoData {
                name: "BankingSpeed",
                name_hash: 3469733156,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, banking_speed),
            },
            FieldInfoData {
                name: "Icon",
                name_hash: 2088920302,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MissileEntityData, icon),
            },
            FieldInfoData {
                name: "TargetIcon",
                name_hash: 329061247,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MissileEntityData, target_icon),
            },
            FieldInfoData {
                name: "TargetIconEnemy",
                name_hash: 1393236581,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MissileEntityData, target_icon_enemy),
            },
            FieldInfoData {
                name: "MinGhostFrequency",
                name_hash: 2196612396,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileEntityData, min_ghost_frequency),
            },
            FieldInfoData {
                name: "StartEffectsOnSpawn",
                name_hash: 4020696987,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, start_effects_on_spawn),
            },
            FieldInfoData {
                name: "IsBulletCollision",
                name_hash: 2138758087,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, is_bullet_collision),
            },
            FieldInfoData {
                name: "ExtrapolateAcceleration",
                name_hash: 35611198,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, extrapolate_acceleration),
            },
            FieldInfoData {
                name: "CalculatePositionBasedOnVelocity",
                name_hash: 702678175,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileEntityData, calculate_position_based_on_velocity),
            },
        ],
    }),
    array_type: Some(MISSILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MissileEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MISSILEENTITYDATA_TYPE_INFO
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


pub static MISSILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileEntityData-Array",
    name_hash: 399144510,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissileEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NearTargetDetonationData {
    pub detonate_near_target: bool,
    pub detonation_radius: f32,
    pub max_detonation_delay: f32,
    pub min_detonation_delay: f32,
}

pub trait NearTargetDetonationDataTrait: TypeObject {
    fn detonate_near_target(&self) -> &bool;
    fn detonate_near_target_mut(&mut self) -> &mut bool;
    fn detonation_radius(&self) -> &f32;
    fn detonation_radius_mut(&mut self) -> &mut f32;
    fn max_detonation_delay(&self) -> &f32;
    fn max_detonation_delay_mut(&mut self) -> &mut f32;
    fn min_detonation_delay(&self) -> &f32;
    fn min_detonation_delay_mut(&mut self) -> &mut f32;
}

impl NearTargetDetonationDataTrait for NearTargetDetonationData {
    fn detonate_near_target(&self) -> &bool {
        &self.detonate_near_target
    }
    fn detonate_near_target_mut(&mut self) -> &mut bool {
        &mut self.detonate_near_target
    }
    fn detonation_radius(&self) -> &f32 {
        &self.detonation_radius
    }
    fn detonation_radius_mut(&mut self) -> &mut f32 {
        &mut self.detonation_radius
    }
    fn max_detonation_delay(&self) -> &f32 {
        &self.max_detonation_delay
    }
    fn max_detonation_delay_mut(&mut self) -> &mut f32 {
        &mut self.max_detonation_delay
    }
    fn min_detonation_delay(&self) -> &f32 {
        &self.min_detonation_delay
    }
    fn min_detonation_delay_mut(&mut self) -> &mut f32 {
        &mut self.min_detonation_delay
    }
}

pub static NEARTARGETDETONATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NearTargetDetonationData",
    name_hash: 3449364789,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NearTargetDetonationData as Default>::default())),
            create_boxed: || Box::new(<NearTargetDetonationData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DetonateNearTarget",
                name_hash: 2722241128,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(NearTargetDetonationData, detonate_near_target),
            },
            FieldInfoData {
                name: "DetonationRadius",
                name_hash: 3488213844,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NearTargetDetonationData, detonation_radius),
            },
            FieldInfoData {
                name: "MaxDetonationDelay",
                name_hash: 23395725,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NearTargetDetonationData, max_detonation_delay),
            },
            FieldInfoData {
                name: "MinDetonationDelay",
                name_hash: 1672486355,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(NearTargetDetonationData, min_detonation_delay),
            },
        ],
    }),
    array_type: Some(NEARTARGETDETONATIONDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for NearTargetDetonationData {
    fn type_info(&self) -> &'static TypeInfo {
        NEARTARGETDETONATIONDATA_TYPE_INFO
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


pub static NEARTARGETDETONATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NearTargetDetonationData-Array",
    name_hash: 1528144257,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("NearTargetDetonationData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MissileUnguidedData {
    pub use_static_position: bool,
    pub static_position: super::core::Vec2,
    pub use_target_position: bool,
    pub target_position_offset: super::core::Vec2,
}

pub trait MissileUnguidedDataTrait: TypeObject {
    fn use_static_position(&self) -> &bool;
    fn use_static_position_mut(&mut self) -> &mut bool;
    fn static_position(&self) -> &super::core::Vec2;
    fn static_position_mut(&mut self) -> &mut super::core::Vec2;
    fn use_target_position(&self) -> &bool;
    fn use_target_position_mut(&mut self) -> &mut bool;
    fn target_position_offset(&self) -> &super::core::Vec2;
    fn target_position_offset_mut(&mut self) -> &mut super::core::Vec2;
}

impl MissileUnguidedDataTrait for MissileUnguidedData {
    fn use_static_position(&self) -> &bool {
        &self.use_static_position
    }
    fn use_static_position_mut(&mut self) -> &mut bool {
        &mut self.use_static_position
    }
    fn static_position(&self) -> &super::core::Vec2 {
        &self.static_position
    }
    fn static_position_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.static_position
    }
    fn use_target_position(&self) -> &bool {
        &self.use_target_position
    }
    fn use_target_position_mut(&mut self) -> &mut bool {
        &mut self.use_target_position
    }
    fn target_position_offset(&self) -> &super::core::Vec2 {
        &self.target_position_offset
    }
    fn target_position_offset_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.target_position_offset
    }
}

pub static MISSILEUNGUIDEDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileUnguidedData",
    name_hash: 332190324,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MissileUnguidedData as Default>::default())),
            create_boxed: || Box::new(<MissileUnguidedData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UseStaticPosition",
                name_hash: 753327591,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileUnguidedData, use_static_position),
            },
            FieldInfoData {
                name: "StaticPosition",
                name_hash: 242876836,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(MissileUnguidedData, static_position),
            },
            FieldInfoData {
                name: "UseTargetPosition",
                name_hash: 1827844270,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MissileUnguidedData, use_target_position),
            },
            FieldInfoData {
                name: "TargetPositionOffset",
                name_hash: 3190891072,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(MissileUnguidedData, target_position_offset),
            },
        ],
    }),
    array_type: Some(MISSILEUNGUIDEDDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MissileUnguidedData {
    fn type_info(&self) -> &'static TypeInfo {
        MISSILEUNGUIDEDDATA_TYPE_INFO
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


pub static MISSILEUNGUIDEDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileUnguidedData-Array",
    name_hash: 2692611648,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissileUnguidedData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MissileLockableInfoData {
    pub heat_signature: f32,
    pub radar_signature: f32,
}

pub trait MissileLockableInfoDataTrait: TypeObject {
    fn heat_signature(&self) -> &f32;
    fn heat_signature_mut(&mut self) -> &mut f32;
    fn radar_signature(&self) -> &f32;
    fn radar_signature_mut(&mut self) -> &mut f32;
}

impl MissileLockableInfoDataTrait for MissileLockableInfoData {
    fn heat_signature(&self) -> &f32 {
        &self.heat_signature
    }
    fn heat_signature_mut(&mut self) -> &mut f32 {
        &mut self.heat_signature
    }
    fn radar_signature(&self) -> &f32 {
        &self.radar_signature
    }
    fn radar_signature_mut(&mut self) -> &mut f32 {
        &mut self.radar_signature
    }
}

pub static MISSILELOCKABLEINFODATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileLockableInfoData",
    name_hash: 1421140862,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MissileLockableInfoData as Default>::default())),
            create_boxed: || Box::new(<MissileLockableInfoData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HeatSignature",
                name_hash: 2221510681,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileLockableInfoData, heat_signature),
            },
            FieldInfoData {
                name: "RadarSignature",
                name_hash: 1808737829,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MissileLockableInfoData, radar_signature),
            },
        ],
    }),
    array_type: Some(MISSILELOCKABLEINFODATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MissileLockableInfoData {
    fn type_info(&self) -> &'static TypeInfo {
        MISSILELOCKABLEINFODATA_TYPE_INFO
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


pub static MISSILELOCKABLEINFODATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissileLockableInfoData-Array",
    name_hash: 1771725898,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissileLockableInfoData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LockingControllerData {
    pub _glacier_base: super::core::DataContainer,
    pub zoom_level_lock: Vec<BoxedTypeObject /* ZoomLevelLockData */>,
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

pub trait LockingControllerDataTrait: super::core::DataContainerTrait {
    fn zoom_level_lock(&self) -> &Vec<BoxedTypeObject /* ZoomLevelLockData */>;
    fn zoom_level_lock_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ZoomLevelLockData */>;
    fn lock_time(&self) -> &f32;
    fn lock_time_mut(&mut self) -> &mut f32;
    fn un_lock_time(&self) -> &f32;
    fn un_lock_time_mut(&mut self) -> &mut f32;
    fn release_time(&self) -> &f32;
    fn release_time_mut(&mut self) -> &mut f32;
    fn release_on_new_target_time(&self) -> &f32;
    fn release_on_new_target_time_mut(&mut self) -> &mut f32;
    fn sample_rate(&self) -> &f32;
    fn sample_rate_mut(&mut self) -> &mut f32;
    fn hold_still_threshold(&self) -> &f32;
    fn hold_still_threshold_mut(&mut self) -> &mut f32;
    fn check_visibility_length(&self) -> &f32;
    fn check_visibility_length_mut(&mut self) -> &mut f32;
    fn ray_length(&self) -> &f32;
    fn ray_length_mut(&mut self) -> &mut f32;
    fn lock_on_visible_targets_only(&self) -> &bool;
    fn lock_on_visible_targets_only_mut(&mut self) -> &mut bool;
    fn require_ammo_to_lock(&self) -> &bool;
    fn require_ammo_to_lock_mut(&mut self) -> &mut bool;
    fn position_only(&self) -> &bool;
    fn position_only_mut(&mut self) -> &mut bool;
    fn use_unlock_time_with_position_only(&self) -> &bool;
    fn use_unlock_time_with_position_only_mut(&mut self) -> &mut bool;
    fn lock_on_world_space_pos(&self) -> &bool;
    fn lock_on_world_space_pos_mut(&mut self) -> &mut bool;
    fn acceptance_angle(&self) -> &f32;
    fn acceptance_angle_mut(&mut self) -> &mut f32;
    fn angle_constant(&self) -> &f32;
    fn angle_constant_mut(&mut self) -> &mut f32;
    fn distance_constant(&self) -> &f32;
    fn distance_constant_mut(&mut self) -> &mut f32;
    fn normalize_constant_weights(&self) -> &bool;
    fn normalize_constant_weights_mut(&mut self) -> &mut bool;
    fn check_target_lock_strength_on_raycast(&self) -> &bool;
    fn check_target_lock_strength_on_raycast_mut(&mut self) -> &mut bool;
    fn sensitivity(&self) -> &f32;
    fn sensitivity_mut(&mut self) -> &mut f32;
    fn minimum_lock_time(&self) -> &f32;
    fn minimum_lock_time_mut(&mut self) -> &mut f32;
    fn lock_on_vehicles(&self) -> &bool;
    fn lock_on_vehicles_mut(&mut self) -> &mut bool;
    fn lock_on_empty_vehicles(&self) -> &bool;
    fn lock_on_empty_vehicles_mut(&mut self) -> &mut bool;
    fn lock_on_characters(&self) -> &bool;
    fn lock_on_characters_mut(&mut self) -> &mut bool;
    fn lock_on_characters_in_open_entries(&self) -> &bool;
    fn lock_on_characters_in_open_entries_mut(&mut self) -> &mut bool;
    fn ignore_heigth_lock_distance(&self) -> &bool;
    fn ignore_heigth_lock_distance_mut(&mut self) -> &mut bool;
    fn lock_in_combat_area_only(&self) -> &bool;
    fn lock_in_combat_area_only_mut(&mut self) -> &mut bool;
    fn locking_weapon_data(&self) -> &LockingAndHomingData;
    fn locking_weapon_data_mut(&mut self) -> &mut LockingAndHomingData;
}

impl LockingControllerDataTrait for LockingControllerData {
    fn zoom_level_lock(&self) -> &Vec<BoxedTypeObject /* ZoomLevelLockData */> {
        &self.zoom_level_lock
    }
    fn zoom_level_lock_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ZoomLevelLockData */> {
        &mut self.zoom_level_lock
    }
    fn lock_time(&self) -> &f32 {
        &self.lock_time
    }
    fn lock_time_mut(&mut self) -> &mut f32 {
        &mut self.lock_time
    }
    fn un_lock_time(&self) -> &f32 {
        &self.un_lock_time
    }
    fn un_lock_time_mut(&mut self) -> &mut f32 {
        &mut self.un_lock_time
    }
    fn release_time(&self) -> &f32 {
        &self.release_time
    }
    fn release_time_mut(&mut self) -> &mut f32 {
        &mut self.release_time
    }
    fn release_on_new_target_time(&self) -> &f32 {
        &self.release_on_new_target_time
    }
    fn release_on_new_target_time_mut(&mut self) -> &mut f32 {
        &mut self.release_on_new_target_time
    }
    fn sample_rate(&self) -> &f32 {
        &self.sample_rate
    }
    fn sample_rate_mut(&mut self) -> &mut f32 {
        &mut self.sample_rate
    }
    fn hold_still_threshold(&self) -> &f32 {
        &self.hold_still_threshold
    }
    fn hold_still_threshold_mut(&mut self) -> &mut f32 {
        &mut self.hold_still_threshold
    }
    fn check_visibility_length(&self) -> &f32 {
        &self.check_visibility_length
    }
    fn check_visibility_length_mut(&mut self) -> &mut f32 {
        &mut self.check_visibility_length
    }
    fn ray_length(&self) -> &f32 {
        &self.ray_length
    }
    fn ray_length_mut(&mut self) -> &mut f32 {
        &mut self.ray_length
    }
    fn lock_on_visible_targets_only(&self) -> &bool {
        &self.lock_on_visible_targets_only
    }
    fn lock_on_visible_targets_only_mut(&mut self) -> &mut bool {
        &mut self.lock_on_visible_targets_only
    }
    fn require_ammo_to_lock(&self) -> &bool {
        &self.require_ammo_to_lock
    }
    fn require_ammo_to_lock_mut(&mut self) -> &mut bool {
        &mut self.require_ammo_to_lock
    }
    fn position_only(&self) -> &bool {
        &self.position_only
    }
    fn position_only_mut(&mut self) -> &mut bool {
        &mut self.position_only
    }
    fn use_unlock_time_with_position_only(&self) -> &bool {
        &self.use_unlock_time_with_position_only
    }
    fn use_unlock_time_with_position_only_mut(&mut self) -> &mut bool {
        &mut self.use_unlock_time_with_position_only
    }
    fn lock_on_world_space_pos(&self) -> &bool {
        &self.lock_on_world_space_pos
    }
    fn lock_on_world_space_pos_mut(&mut self) -> &mut bool {
        &mut self.lock_on_world_space_pos
    }
    fn acceptance_angle(&self) -> &f32 {
        &self.acceptance_angle
    }
    fn acceptance_angle_mut(&mut self) -> &mut f32 {
        &mut self.acceptance_angle
    }
    fn angle_constant(&self) -> &f32 {
        &self.angle_constant
    }
    fn angle_constant_mut(&mut self) -> &mut f32 {
        &mut self.angle_constant
    }
    fn distance_constant(&self) -> &f32 {
        &self.distance_constant
    }
    fn distance_constant_mut(&mut self) -> &mut f32 {
        &mut self.distance_constant
    }
    fn normalize_constant_weights(&self) -> &bool {
        &self.normalize_constant_weights
    }
    fn normalize_constant_weights_mut(&mut self) -> &mut bool {
        &mut self.normalize_constant_weights
    }
    fn check_target_lock_strength_on_raycast(&self) -> &bool {
        &self.check_target_lock_strength_on_raycast
    }
    fn check_target_lock_strength_on_raycast_mut(&mut self) -> &mut bool {
        &mut self.check_target_lock_strength_on_raycast
    }
    fn sensitivity(&self) -> &f32 {
        &self.sensitivity
    }
    fn sensitivity_mut(&mut self) -> &mut f32 {
        &mut self.sensitivity
    }
    fn minimum_lock_time(&self) -> &f32 {
        &self.minimum_lock_time
    }
    fn minimum_lock_time_mut(&mut self) -> &mut f32 {
        &mut self.minimum_lock_time
    }
    fn lock_on_vehicles(&self) -> &bool {
        &self.lock_on_vehicles
    }
    fn lock_on_vehicles_mut(&mut self) -> &mut bool {
        &mut self.lock_on_vehicles
    }
    fn lock_on_empty_vehicles(&self) -> &bool {
        &self.lock_on_empty_vehicles
    }
    fn lock_on_empty_vehicles_mut(&mut self) -> &mut bool {
        &mut self.lock_on_empty_vehicles
    }
    fn lock_on_characters(&self) -> &bool {
        &self.lock_on_characters
    }
    fn lock_on_characters_mut(&mut self) -> &mut bool {
        &mut self.lock_on_characters
    }
    fn lock_on_characters_in_open_entries(&self) -> &bool {
        &self.lock_on_characters_in_open_entries
    }
    fn lock_on_characters_in_open_entries_mut(&mut self) -> &mut bool {
        &mut self.lock_on_characters_in_open_entries
    }
    fn ignore_heigth_lock_distance(&self) -> &bool {
        &self.ignore_heigth_lock_distance
    }
    fn ignore_heigth_lock_distance_mut(&mut self) -> &mut bool {
        &mut self.ignore_heigth_lock_distance
    }
    fn lock_in_combat_area_only(&self) -> &bool {
        &self.lock_in_combat_area_only
    }
    fn lock_in_combat_area_only_mut(&mut self) -> &mut bool {
        &mut self.lock_in_combat_area_only
    }
    fn locking_weapon_data(&self) -> &LockingAndHomingData {
        &self.locking_weapon_data
    }
    fn locking_weapon_data_mut(&mut self) -> &mut LockingAndHomingData {
        &mut self.locking_weapon_data
    }
}

impl super::core::DataContainerTrait for LockingControllerData {
}

pub static LOCKINGCONTROLLERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingControllerData",
    name_hash: 1236839074,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(LockingControllerData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LockingControllerData as Default>::default())),
            create_boxed: || Box::new(<LockingControllerData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ZoomLevelLock",
                name_hash: 1609958223,
                flags: MemberInfoFlags::new(144),
                field_type: "ZoomLevelLockData-Array",
                rust_offset: offset_of!(LockingControllerData, zoom_level_lock),
            },
            FieldInfoData {
                name: "LockTime",
                name_hash: 1982096347,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, lock_time),
            },
            FieldInfoData {
                name: "UnLockTime",
                name_hash: 1362482368,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, un_lock_time),
            },
            FieldInfoData {
                name: "ReleaseTime",
                name_hash: 892319833,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, release_time),
            },
            FieldInfoData {
                name: "ReleaseOnNewTargetTime",
                name_hash: 133838421,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, release_on_new_target_time),
            },
            FieldInfoData {
                name: "SampleRate",
                name_hash: 604757697,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, sample_rate),
            },
            FieldInfoData {
                name: "HoldStillThreshold",
                name_hash: 3981331987,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, hold_still_threshold),
            },
            FieldInfoData {
                name: "CheckVisibilityLength",
                name_hash: 3580554745,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, check_visibility_length),
            },
            FieldInfoData {
                name: "RayLength",
                name_hash: 714575027,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, ray_length),
            },
            FieldInfoData {
                name: "LockOnVisibleTargetsOnly",
                name_hash: 1817419671,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, lock_on_visible_targets_only),
            },
            FieldInfoData {
                name: "RequireAmmoToLock",
                name_hash: 1624603382,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, require_ammo_to_lock),
            },
            FieldInfoData {
                name: "PositionOnly",
                name_hash: 3572870760,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, position_only),
            },
            FieldInfoData {
                name: "UseUnlockTimeWithPositionOnly",
                name_hash: 876039692,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, use_unlock_time_with_position_only),
            },
            FieldInfoData {
                name: "LockOnWorldSpacePos",
                name_hash: 2930832517,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, lock_on_world_space_pos),
            },
            FieldInfoData {
                name: "AcceptanceAngle",
                name_hash: 66845485,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, acceptance_angle),
            },
            FieldInfoData {
                name: "AngleConstant",
                name_hash: 2438038842,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, angle_constant),
            },
            FieldInfoData {
                name: "DistanceConstant",
                name_hash: 1734890008,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, distance_constant),
            },
            FieldInfoData {
                name: "NormalizeConstantWeights",
                name_hash: 2137597165,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, normalize_constant_weights),
            },
            FieldInfoData {
                name: "CheckTargetLockStrengthOnRaycast",
                name_hash: 2461239378,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, check_target_lock_strength_on_raycast),
            },
            FieldInfoData {
                name: "Sensitivity",
                name_hash: 2124383208,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, sensitivity),
            },
            FieldInfoData {
                name: "MinimumLockTime",
                name_hash: 1585650381,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LockingControllerData, minimum_lock_time),
            },
            FieldInfoData {
                name: "LockOnVehicles",
                name_hash: 368525092,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, lock_on_vehicles),
            },
            FieldInfoData {
                name: "LockOnEmptyVehicles",
                name_hash: 633097329,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, lock_on_empty_vehicles),
            },
            FieldInfoData {
                name: "LockOnCharacters",
                name_hash: 3662913221,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, lock_on_characters),
            },
            FieldInfoData {
                name: "LockOnCharactersInOpenEntries",
                name_hash: 479264644,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, lock_on_characters_in_open_entries),
            },
            FieldInfoData {
                name: "IgnoreHeigthLockDistance",
                name_hash: 1465082762,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, ignore_heigth_lock_distance),
            },
            FieldInfoData {
                name: "LockInCombatAreaOnly",
                name_hash: 2572107708,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingControllerData, lock_in_combat_area_only),
            },
            FieldInfoData {
                name: "LockingWeaponData",
                name_hash: 837219740,
                flags: MemberInfoFlags::new(0),
                field_type: "LockingAndHomingData",
                rust_offset: offset_of!(LockingControllerData, locking_weapon_data),
            },
        ],
    }),
    array_type: Some(LOCKINGCONTROLLERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LockingControllerData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCKINGCONTROLLERDATA_TYPE_INFO
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


pub static LOCKINGCONTROLLERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingControllerData-Array",
    name_hash: 1856309526,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LockingControllerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LockingAndHomingData {
    pub is_homing: bool,
    pub is_guided: bool,
    pub is_guided_when_zoomed: bool,
    pub is_guided_homing: bool,
    pub fire_only_when_locked_on: bool,
    pub warn_lock: WarnTarget,
}

pub trait LockingAndHomingDataTrait: TypeObject {
    fn is_homing(&self) -> &bool;
    fn is_homing_mut(&mut self) -> &mut bool;
    fn is_guided(&self) -> &bool;
    fn is_guided_mut(&mut self) -> &mut bool;
    fn is_guided_when_zoomed(&self) -> &bool;
    fn is_guided_when_zoomed_mut(&mut self) -> &mut bool;
    fn is_guided_homing(&self) -> &bool;
    fn is_guided_homing_mut(&mut self) -> &mut bool;
    fn fire_only_when_locked_on(&self) -> &bool;
    fn fire_only_when_locked_on_mut(&mut self) -> &mut bool;
    fn warn_lock(&self) -> &WarnTarget;
    fn warn_lock_mut(&mut self) -> &mut WarnTarget;
}

impl LockingAndHomingDataTrait for LockingAndHomingData {
    fn is_homing(&self) -> &bool {
        &self.is_homing
    }
    fn is_homing_mut(&mut self) -> &mut bool {
        &mut self.is_homing
    }
    fn is_guided(&self) -> &bool {
        &self.is_guided
    }
    fn is_guided_mut(&mut self) -> &mut bool {
        &mut self.is_guided
    }
    fn is_guided_when_zoomed(&self) -> &bool {
        &self.is_guided_when_zoomed
    }
    fn is_guided_when_zoomed_mut(&mut self) -> &mut bool {
        &mut self.is_guided_when_zoomed
    }
    fn is_guided_homing(&self) -> &bool {
        &self.is_guided_homing
    }
    fn is_guided_homing_mut(&mut self) -> &mut bool {
        &mut self.is_guided_homing
    }
    fn fire_only_when_locked_on(&self) -> &bool {
        &self.fire_only_when_locked_on
    }
    fn fire_only_when_locked_on_mut(&mut self) -> &mut bool {
        &mut self.fire_only_when_locked_on
    }
    fn warn_lock(&self) -> &WarnTarget {
        &self.warn_lock
    }
    fn warn_lock_mut(&mut self) -> &mut WarnTarget {
        &mut self.warn_lock
    }
}

pub static LOCKINGANDHOMINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingAndHomingData",
    name_hash: 4072506911,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LockingAndHomingData as Default>::default())),
            create_boxed: || Box::new(<LockingAndHomingData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsHoming",
                name_hash: 962805909,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingAndHomingData, is_homing),
            },
            FieldInfoData {
                name: "IsGuided",
                name_hash: 911651905,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingAndHomingData, is_guided),
            },
            FieldInfoData {
                name: "IsGuidedWhenZoomed",
                name_hash: 951364163,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingAndHomingData, is_guided_when_zoomed),
            },
            FieldInfoData {
                name: "IsGuidedHoming",
                name_hash: 399064139,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingAndHomingData, is_guided_homing),
            },
            FieldInfoData {
                name: "FireOnlyWhenLockedOn",
                name_hash: 708469046,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LockingAndHomingData, fire_only_when_locked_on),
            },
            FieldInfoData {
                name: "WarnLock",
                name_hash: 2457242500,
                flags: MemberInfoFlags::new(0),
                field_type: "WarnTarget",
                rust_offset: offset_of!(LockingAndHomingData, warn_lock),
            },
        ],
    }),
    array_type: Some(LOCKINGANDHOMINGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LockingAndHomingData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCKINGANDHOMINGDATA_TYPE_INFO
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


pub static LOCKINGANDHOMINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LockingAndHomingData-Array",
    name_hash: 910019243,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("LockingAndHomingData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoomLevelLockData {
    pub outline_tagged_distance: f32,
    pub lock_type: super::gameplay_sim::LockType,
}

pub trait ZoomLevelLockDataTrait: TypeObject {
    fn outline_tagged_distance(&self) -> &f32;
    fn outline_tagged_distance_mut(&mut self) -> &mut f32;
    fn lock_type(&self) -> &super::gameplay_sim::LockType;
    fn lock_type_mut(&mut self) -> &mut super::gameplay_sim::LockType;
}

impl ZoomLevelLockDataTrait for ZoomLevelLockData {
    fn outline_tagged_distance(&self) -> &f32 {
        &self.outline_tagged_distance
    }
    fn outline_tagged_distance_mut(&mut self) -> &mut f32 {
        &mut self.outline_tagged_distance
    }
    fn lock_type(&self) -> &super::gameplay_sim::LockType {
        &self.lock_type
    }
    fn lock_type_mut(&mut self) -> &mut super::gameplay_sim::LockType {
        &mut self.lock_type
    }
}

pub static ZOOMLEVELLOCKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoomLevelLockData",
    name_hash: 2801099583,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoomLevelLockData as Default>::default())),
            create_boxed: || Box::new(<ZoomLevelLockData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OutlineTaggedDistance",
                name_hash: 2697603954,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ZoomLevelLockData, outline_tagged_distance),
            },
            FieldInfoData {
                name: "LockType",
                name_hash: 1982079030,
                flags: MemberInfoFlags::new(0),
                field_type: "LockType",
                rust_offset: offset_of!(ZoomLevelLockData, lock_type),
            },
        ],
    }),
    array_type: Some(ZOOMLEVELLOCKDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ZoomLevelLockData {
    fn type_info(&self) -> &'static TypeInfo {
        ZOOMLEVELLOCKDATA_TYPE_INFO
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


pub static ZOOMLEVELLOCKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoomLevelLockData-Array",
    name_hash: 701708427,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ZoomLevelLockData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WarnTarget {
    #[default]
    wtWarnSoldierAndVehicle = 0,
    wtWarnSoldier = 1,
    wtWarnVehicle = 2,
    wtWarnNone = 3,
}

pub static WARNTARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WarnTarget",
    name_hash: 1103784926,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(WARNTARGET_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WarnTarget {
    fn type_info(&self) -> &'static TypeInfo {
        WARNTARGET_TYPE_INFO
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


pub static WARNTARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WarnTarget-Array",
    name_hash: 2135502186,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WarnTarget"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BulletEntityData {
    pub _glacier_base: MeshProjectileEntityData,
    pub fly_by_sound: Option<LockedTypeObject /* super::audio::SoundAsset */>,
    pub dud_explosion: Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>,
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

pub trait BulletEntityDataTrait: MeshProjectileEntityDataTrait {
    fn fly_by_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn fly_by_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */>;
    fn dud_explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn dud_explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn gravity(&self) -> &f32;
    fn gravity_mut(&mut self) -> &mut f32;
    fn impact_impulse(&self) -> &f32;
    fn impact_impulse_mut(&mut self) -> &mut f32;
    fn detonation_time_variation(&self) -> &f32;
    fn detonation_time_variation_mut(&mut self) -> &mut f32;
    fn vehicle_detonation_radius(&self) -> &f32;
    fn vehicle_detonation_radius_mut(&mut self) -> &mut f32;
    fn vehicle_detonation_activation_delay(&self) -> &f32;
    fn vehicle_detonation_activation_delay_mut(&mut self) -> &mut f32;
    fn fly_by_sound_radius(&self) -> &f32;
    fn fly_by_sound_radius_mut(&mut self) -> &mut f32;
    fn fly_by_sound_speed(&self) -> &f32;
    fn fly_by_sound_speed_mut(&mut self) -> &mut f32;
    fn stamina(&self) -> &f32;
    fn stamina_mut(&mut self) -> &mut f32;
    fn distribute_damage_over_time(&self) -> &f32;
    fn distribute_damage_over_time_mut(&mut self) -> &mut f32;
    fn start_damage(&self) -> &f32;
    fn start_damage_mut(&mut self) -> &mut f32;
    fn end_damage(&self) -> &f32;
    fn end_damage_mut(&mut self) -> &mut f32;
    fn damage_falloff_start_distance(&self) -> &f32;
    fn damage_falloff_start_distance_mut(&mut self) -> &mut f32;
    fn damage_falloff_end_distance(&self) -> &f32;
    fn damage_falloff_end_distance_mut(&mut self) -> &mut f32;
    fn time_to_arm_explosion(&self) -> &f32;
    fn time_to_arm_explosion_mut(&mut self) -> &mut f32;
    fn has_vehicle_detonation(&self) -> &bool;
    fn has_vehicle_detonation_mut(&mut self) -> &mut bool;
    fn instant_hit(&self) -> &bool;
    fn instant_hit_mut(&mut self) -> &mut bool;
    fn first_frame_travel_distance(&self) -> &f32;
    fn first_frame_travel_distance_mut(&mut self) -> &mut f32;
    fn stop_trail_effect_on_unspawn(&self) -> &bool;
    fn stop_trail_effect_on_unspawn_mut(&mut self) -> &mut bool;
    fn stop_trail_effect_on_unspawn_frame_delay(&self) -> &i32;
    fn stop_trail_effect_on_unspawn_frame_delay_mut(&mut self) -> &mut i32;
}

impl BulletEntityDataTrait for BulletEntityData {
    fn fly_by_sound(&self) -> &Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &self.fly_by_sound
    }
    fn fly_by_sound_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::SoundAsset */> {
        &mut self.fly_by_sound
    }
    fn dud_explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &self.dud_explosion
    }
    fn dud_explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &mut self.dud_explosion
    }
    fn gravity(&self) -> &f32 {
        &self.gravity
    }
    fn gravity_mut(&mut self) -> &mut f32 {
        &mut self.gravity
    }
    fn impact_impulse(&self) -> &f32 {
        &self.impact_impulse
    }
    fn impact_impulse_mut(&mut self) -> &mut f32 {
        &mut self.impact_impulse
    }
    fn detonation_time_variation(&self) -> &f32 {
        &self.detonation_time_variation
    }
    fn detonation_time_variation_mut(&mut self) -> &mut f32 {
        &mut self.detonation_time_variation
    }
    fn vehicle_detonation_radius(&self) -> &f32 {
        &self.vehicle_detonation_radius
    }
    fn vehicle_detonation_radius_mut(&mut self) -> &mut f32 {
        &mut self.vehicle_detonation_radius
    }
    fn vehicle_detonation_activation_delay(&self) -> &f32 {
        &self.vehicle_detonation_activation_delay
    }
    fn vehicle_detonation_activation_delay_mut(&mut self) -> &mut f32 {
        &mut self.vehicle_detonation_activation_delay
    }
    fn fly_by_sound_radius(&self) -> &f32 {
        &self.fly_by_sound_radius
    }
    fn fly_by_sound_radius_mut(&mut self) -> &mut f32 {
        &mut self.fly_by_sound_radius
    }
    fn fly_by_sound_speed(&self) -> &f32 {
        &self.fly_by_sound_speed
    }
    fn fly_by_sound_speed_mut(&mut self) -> &mut f32 {
        &mut self.fly_by_sound_speed
    }
    fn stamina(&self) -> &f32 {
        &self.stamina
    }
    fn stamina_mut(&mut self) -> &mut f32 {
        &mut self.stamina
    }
    fn distribute_damage_over_time(&self) -> &f32 {
        &self.distribute_damage_over_time
    }
    fn distribute_damage_over_time_mut(&mut self) -> &mut f32 {
        &mut self.distribute_damage_over_time
    }
    fn start_damage(&self) -> &f32 {
        &self.start_damage
    }
    fn start_damage_mut(&mut self) -> &mut f32 {
        &mut self.start_damage
    }
    fn end_damage(&self) -> &f32 {
        &self.end_damage
    }
    fn end_damage_mut(&mut self) -> &mut f32 {
        &mut self.end_damage
    }
    fn damage_falloff_start_distance(&self) -> &f32 {
        &self.damage_falloff_start_distance
    }
    fn damage_falloff_start_distance_mut(&mut self) -> &mut f32 {
        &mut self.damage_falloff_start_distance
    }
    fn damage_falloff_end_distance(&self) -> &f32 {
        &self.damage_falloff_end_distance
    }
    fn damage_falloff_end_distance_mut(&mut self) -> &mut f32 {
        &mut self.damage_falloff_end_distance
    }
    fn time_to_arm_explosion(&self) -> &f32 {
        &self.time_to_arm_explosion
    }
    fn time_to_arm_explosion_mut(&mut self) -> &mut f32 {
        &mut self.time_to_arm_explosion
    }
    fn has_vehicle_detonation(&self) -> &bool {
        &self.has_vehicle_detonation
    }
    fn has_vehicle_detonation_mut(&mut self) -> &mut bool {
        &mut self.has_vehicle_detonation
    }
    fn instant_hit(&self) -> &bool {
        &self.instant_hit
    }
    fn instant_hit_mut(&mut self) -> &mut bool {
        &mut self.instant_hit
    }
    fn first_frame_travel_distance(&self) -> &f32 {
        &self.first_frame_travel_distance
    }
    fn first_frame_travel_distance_mut(&mut self) -> &mut f32 {
        &mut self.first_frame_travel_distance
    }
    fn stop_trail_effect_on_unspawn(&self) -> &bool {
        &self.stop_trail_effect_on_unspawn
    }
    fn stop_trail_effect_on_unspawn_mut(&mut self) -> &mut bool {
        &mut self.stop_trail_effect_on_unspawn
    }
    fn stop_trail_effect_on_unspawn_frame_delay(&self) -> &i32 {
        &self.stop_trail_effect_on_unspawn_frame_delay
    }
    fn stop_trail_effect_on_unspawn_frame_delay_mut(&mut self) -> &mut i32 {
        &mut self.stop_trail_effect_on_unspawn_frame_delay
    }
}

impl MeshProjectileEntityDataTrait for BulletEntityData {
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh()
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh_mut()
    }
    fn trail_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        self._glacier_base.trail_effect()
    }
    fn trail_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        self._glacier_base.trail_effect_mut()
    }
    fn is_attachable(&self) -> &bool {
        self._glacier_base.is_attachable()
    }
    fn is_attachable_mut(&mut self) -> &mut bool {
        self._glacier_base.is_attachable_mut()
    }
    fn instant_attachable_test_distance(&self) -> &f32 {
        self._glacier_base.instant_attachable_test_distance()
    }
    fn instant_attachable_test_distance_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_test_distance_mut()
    }
    fn instant_attachable_visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.instant_attachable_visual_convergence_delay()
    }
    fn instant_attachable_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_visual_convergence_delay_mut()
    }
    fn instant_attachable_visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.instant_attachable_visual_convergence_duration()
    }
    fn instant_attachable_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_visual_convergence_duration_mut()
    }
    fn instant_attachable_test_under_reticule(&self) -> &bool {
        self._glacier_base.instant_attachable_test_under_reticule()
    }
    fn instant_attachable_test_under_reticule_mut(&mut self) -> &mut bool {
        self._glacier_base.instant_attachable_test_under_reticule_mut()
    }
    fn max_attachable_inclination(&self) -> &f32 {
        self._glacier_base.max_attachable_inclination()
    }
    fn max_attachable_inclination_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_attachable_inclination_mut()
    }
    fn extra_damping(&self) -> &bool {
        self._glacier_base.extra_damping()
    }
    fn extra_damping_mut(&mut self) -> &mut bool {
        self._glacier_base.extra_damping_mut()
    }
    fn initial_angular_velocity(&self) -> &super::core::Vec3 {
        self._glacier_base.initial_angular_velocity()
    }
    fn initial_angular_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.initial_angular_velocity_mut()
    }
    fn unspawn_after_detonation_delay(&self) -> &f32 {
        self._glacier_base.unspawn_after_detonation_delay()
    }
    fn unspawn_after_detonation_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.unspawn_after_detonation_delay_mut()
    }
}

impl ProjectileEntityDataTrait for BulletEntityData {
    fn initial_speed(&self) -> &f32 {
        self._glacier_base.initial_speed()
    }
    fn initial_speed_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_speed_mut()
    }
    fn time_to_live(&self) -> &f32 {
        self._glacier_base.time_to_live()
    }
    fn time_to_live_mut(&mut self) -> &mut f32 {
        self._glacier_base.time_to_live_mut()
    }
    fn max_count(&self) -> &u32 {
        self._glacier_base.max_count()
    }
    fn max_count_mut(&mut self) -> &mut u32 {
        self._glacier_base.max_count_mut()
    }
    fn init_mesh_hide_time(&self) -> &f32 {
        self._glacier_base.init_mesh_hide_time()
    }
    fn init_mesh_hide_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.init_mesh_hide_time_mut()
    }
    fn visual_converge_distance(&self) -> &f32 {
        self._glacier_base.visual_converge_distance()
    }
    fn visual_converge_distance_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_converge_distance_mut()
    }
    fn visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.visual_convergence_delay()
    }
    fn visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_delay_mut()
    }
    fn visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.visual_convergence_duration()
    }
    fn visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_duration_mut()
    }
    fn proxy_visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_delay()
    }
    fn proxy_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_delay_mut()
    }
    fn proxy_visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_duration()
    }
    fn proxy_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_duration_mut()
    }
    fn detonate_on_timeout(&self) -> &bool {
        self._glacier_base.detonate_on_timeout()
    }
    fn detonate_on_timeout_mut(&mut self) -> &mut bool {
        self._glacier_base.detonate_on_timeout_mut()
    }
    fn server_projectile_disabled(&self) -> &bool {
        self._glacier_base.server_projectile_disabled()
    }
    fn server_projectile_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.server_projectile_disabled_mut()
    }
    fn explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion()
    }
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion_mut()
    }
    fn suppression_data(&self) -> &Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data()
    }
    fn suppression_data_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data_mut()
    }
    fn ammunition_type(&self) -> &String {
        self._glacier_base.ammunition_type()
    }
    fn ammunition_type_mut(&mut self) -> &mut String {
        self._glacier_base.ammunition_type_mut()
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        self._glacier_base.material_pair()
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        self._glacier_base.material_pair_mut()
    }
    fn hit_reaction_weapon_type(&self) -> &AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type()
    }
    fn hit_reaction_weapon_type_mut(&mut self) -> &mut AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type_mut()
    }
    fn hide_on_detonation(&self) -> &bool {
        self._glacier_base.hide_on_detonation()
    }
    fn hide_on_detonation_mut(&mut self) -> &mut bool {
        self._glacier_base.hide_on_detonation_mut()
    }
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info()
    }
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info_mut()
    }
}

impl super::physics::GamePhysicsEntityDataTrait for BulletEntityData {
}

impl super::entity::GameComponentEntityDataTrait for BulletEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for BulletEntityData {
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

impl super::entity::SpatialEntityDataTrait for BulletEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for BulletEntityData {
}

impl super::entity::GameObjectDataTrait for BulletEntityData {
}

impl super::core::DataBusPeerTrait for BulletEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BulletEntityData {
}

impl super::core::DataContainerTrait for BulletEntityData {
}

pub static BULLETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BulletEntityData",
    name_hash: 77594408,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHPROJECTILEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(BulletEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BulletEntityData as Default>::default())),
            create_boxed: || Box::new(<BulletEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FlyBySound",
                name_hash: 3743063886,
                flags: MemberInfoFlags::new(0),
                field_type: "SoundAsset",
                rust_offset: offset_of!(BulletEntityData, fly_by_sound),
            },
            FieldInfoData {
                name: "DudExplosion",
                name_hash: 123118469,
                flags: MemberInfoFlags::new(0),
                field_type: "ExplosionEntityData",
                rust_offset: offset_of!(BulletEntityData, dud_explosion),
            },
            FieldInfoData {
                name: "Gravity",
                name_hash: 3295092963,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, gravity),
            },
            FieldInfoData {
                name: "ImpactImpulse",
                name_hash: 59311612,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, impact_impulse),
            },
            FieldInfoData {
                name: "DetonationTimeVariation",
                name_hash: 246395272,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, detonation_time_variation),
            },
            FieldInfoData {
                name: "VehicleDetonationRadius",
                name_hash: 1470001196,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, vehicle_detonation_radius),
            },
            FieldInfoData {
                name: "VehicleDetonationActivationDelay",
                name_hash: 4100261205,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, vehicle_detonation_activation_delay),
            },
            FieldInfoData {
                name: "FlyBySoundRadius",
                name_hash: 3728826774,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, fly_by_sound_radius),
            },
            FieldInfoData {
                name: "FlyBySoundSpeed",
                name_hash: 4276155561,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, fly_by_sound_speed),
            },
            FieldInfoData {
                name: "Stamina",
                name_hash: 2176368840,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, stamina),
            },
            FieldInfoData {
                name: "DistributeDamageOverTime",
                name_hash: 3294703682,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, distribute_damage_over_time),
            },
            FieldInfoData {
                name: "StartDamage",
                name_hash: 4249391310,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, start_damage),
            },
            FieldInfoData {
                name: "EndDamage",
                name_hash: 3080388609,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, end_damage),
            },
            FieldInfoData {
                name: "DamageFalloffStartDistance",
                name_hash: 170301445,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, damage_falloff_start_distance),
            },
            FieldInfoData {
                name: "DamageFalloffEndDistance",
                name_hash: 2644592906,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, damage_falloff_end_distance),
            },
            FieldInfoData {
                name: "TimeToArmExplosion",
                name_hash: 3533655904,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, time_to_arm_explosion),
            },
            FieldInfoData {
                name: "HasVehicleDetonation",
                name_hash: 2164993390,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BulletEntityData, has_vehicle_detonation),
            },
            FieldInfoData {
                name: "InstantHit",
                name_hash: 3819550411,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BulletEntityData, instant_hit),
            },
            FieldInfoData {
                name: "FirstFrameTravelDistance",
                name_hash: 3753755161,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BulletEntityData, first_frame_travel_distance),
            },
            FieldInfoData {
                name: "StopTrailEffectOnUnspawn",
                name_hash: 2737808201,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BulletEntityData, stop_trail_effect_on_unspawn),
            },
            FieldInfoData {
                name: "StopTrailEffectOnUnspawnFrameDelay",
                name_hash: 2831187329,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BulletEntityData, stop_trail_effect_on_unspawn_frame_delay),
            },
        ],
    }),
    array_type: Some(BULLETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BulletEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        BULLETENTITYDATA_TYPE_INFO
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


pub static BULLETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BulletEntityData-Array",
    name_hash: 3156254876,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("BulletEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GhostedProjectileEntityData {
    pub _glacier_base: MeshProjectileEntityData,
    pub proxy_convergence_delay: f32,
    pub proxy_convergence_duration: f32,
    pub proxy_convergence_instant_on_attach: bool,
    pub force_proxy_convergence: bool,
    pub convergence_using_initial_speed: bool,
}

pub trait GhostedProjectileEntityDataTrait: MeshProjectileEntityDataTrait {
    fn proxy_convergence_delay(&self) -> &f32;
    fn proxy_convergence_delay_mut(&mut self) -> &mut f32;
    fn proxy_convergence_duration(&self) -> &f32;
    fn proxy_convergence_duration_mut(&mut self) -> &mut f32;
    fn proxy_convergence_instant_on_attach(&self) -> &bool;
    fn proxy_convergence_instant_on_attach_mut(&mut self) -> &mut bool;
    fn force_proxy_convergence(&self) -> &bool;
    fn force_proxy_convergence_mut(&mut self) -> &mut bool;
    fn convergence_using_initial_speed(&self) -> &bool;
    fn convergence_using_initial_speed_mut(&mut self) -> &mut bool;
}

impl GhostedProjectileEntityDataTrait for GhostedProjectileEntityData {
    fn proxy_convergence_delay(&self) -> &f32 {
        &self.proxy_convergence_delay
    }
    fn proxy_convergence_delay_mut(&mut self) -> &mut f32 {
        &mut self.proxy_convergence_delay
    }
    fn proxy_convergence_duration(&self) -> &f32 {
        &self.proxy_convergence_duration
    }
    fn proxy_convergence_duration_mut(&mut self) -> &mut f32 {
        &mut self.proxy_convergence_duration
    }
    fn proxy_convergence_instant_on_attach(&self) -> &bool {
        &self.proxy_convergence_instant_on_attach
    }
    fn proxy_convergence_instant_on_attach_mut(&mut self) -> &mut bool {
        &mut self.proxy_convergence_instant_on_attach
    }
    fn force_proxy_convergence(&self) -> &bool {
        &self.force_proxy_convergence
    }
    fn force_proxy_convergence_mut(&mut self) -> &mut bool {
        &mut self.force_proxy_convergence
    }
    fn convergence_using_initial_speed(&self) -> &bool {
        &self.convergence_using_initial_speed
    }
    fn convergence_using_initial_speed_mut(&mut self) -> &mut bool {
        &mut self.convergence_using_initial_speed
    }
}

impl MeshProjectileEntityDataTrait for GhostedProjectileEntityData {
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh()
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        self._glacier_base.mesh_mut()
    }
    fn trail_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        self._glacier_base.trail_effect()
    }
    fn trail_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        self._glacier_base.trail_effect_mut()
    }
    fn is_attachable(&self) -> &bool {
        self._glacier_base.is_attachable()
    }
    fn is_attachable_mut(&mut self) -> &mut bool {
        self._glacier_base.is_attachable_mut()
    }
    fn instant_attachable_test_distance(&self) -> &f32 {
        self._glacier_base.instant_attachable_test_distance()
    }
    fn instant_attachable_test_distance_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_test_distance_mut()
    }
    fn instant_attachable_visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.instant_attachable_visual_convergence_delay()
    }
    fn instant_attachable_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_visual_convergence_delay_mut()
    }
    fn instant_attachable_visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.instant_attachable_visual_convergence_duration()
    }
    fn instant_attachable_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.instant_attachable_visual_convergence_duration_mut()
    }
    fn instant_attachable_test_under_reticule(&self) -> &bool {
        self._glacier_base.instant_attachable_test_under_reticule()
    }
    fn instant_attachable_test_under_reticule_mut(&mut self) -> &mut bool {
        self._glacier_base.instant_attachable_test_under_reticule_mut()
    }
    fn max_attachable_inclination(&self) -> &f32 {
        self._glacier_base.max_attachable_inclination()
    }
    fn max_attachable_inclination_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_attachable_inclination_mut()
    }
    fn extra_damping(&self) -> &bool {
        self._glacier_base.extra_damping()
    }
    fn extra_damping_mut(&mut self) -> &mut bool {
        self._glacier_base.extra_damping_mut()
    }
    fn initial_angular_velocity(&self) -> &super::core::Vec3 {
        self._glacier_base.initial_angular_velocity()
    }
    fn initial_angular_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.initial_angular_velocity_mut()
    }
    fn unspawn_after_detonation_delay(&self) -> &f32 {
        self._glacier_base.unspawn_after_detonation_delay()
    }
    fn unspawn_after_detonation_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.unspawn_after_detonation_delay_mut()
    }
}

impl ProjectileEntityDataTrait for GhostedProjectileEntityData {
    fn initial_speed(&self) -> &f32 {
        self._glacier_base.initial_speed()
    }
    fn initial_speed_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_speed_mut()
    }
    fn time_to_live(&self) -> &f32 {
        self._glacier_base.time_to_live()
    }
    fn time_to_live_mut(&mut self) -> &mut f32 {
        self._glacier_base.time_to_live_mut()
    }
    fn max_count(&self) -> &u32 {
        self._glacier_base.max_count()
    }
    fn max_count_mut(&mut self) -> &mut u32 {
        self._glacier_base.max_count_mut()
    }
    fn init_mesh_hide_time(&self) -> &f32 {
        self._glacier_base.init_mesh_hide_time()
    }
    fn init_mesh_hide_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.init_mesh_hide_time_mut()
    }
    fn visual_converge_distance(&self) -> &f32 {
        self._glacier_base.visual_converge_distance()
    }
    fn visual_converge_distance_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_converge_distance_mut()
    }
    fn visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.visual_convergence_delay()
    }
    fn visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_delay_mut()
    }
    fn visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.visual_convergence_duration()
    }
    fn visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_duration_mut()
    }
    fn proxy_visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_delay()
    }
    fn proxy_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_delay_mut()
    }
    fn proxy_visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_duration()
    }
    fn proxy_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_duration_mut()
    }
    fn detonate_on_timeout(&self) -> &bool {
        self._glacier_base.detonate_on_timeout()
    }
    fn detonate_on_timeout_mut(&mut self) -> &mut bool {
        self._glacier_base.detonate_on_timeout_mut()
    }
    fn server_projectile_disabled(&self) -> &bool {
        self._glacier_base.server_projectile_disabled()
    }
    fn server_projectile_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.server_projectile_disabled_mut()
    }
    fn explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion()
    }
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion_mut()
    }
    fn suppression_data(&self) -> &Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data()
    }
    fn suppression_data_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data_mut()
    }
    fn ammunition_type(&self) -> &String {
        self._glacier_base.ammunition_type()
    }
    fn ammunition_type_mut(&mut self) -> &mut String {
        self._glacier_base.ammunition_type_mut()
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        self._glacier_base.material_pair()
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        self._glacier_base.material_pair_mut()
    }
    fn hit_reaction_weapon_type(&self) -> &AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type()
    }
    fn hit_reaction_weapon_type_mut(&mut self) -> &mut AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type_mut()
    }
    fn hide_on_detonation(&self) -> &bool {
        self._glacier_base.hide_on_detonation()
    }
    fn hide_on_detonation_mut(&mut self) -> &mut bool {
        self._glacier_base.hide_on_detonation_mut()
    }
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info()
    }
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info_mut()
    }
}

impl super::physics::GamePhysicsEntityDataTrait for GhostedProjectileEntityData {
}

impl super::entity::GameComponentEntityDataTrait for GhostedProjectileEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for GhostedProjectileEntityData {
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

impl super::entity::SpatialEntityDataTrait for GhostedProjectileEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for GhostedProjectileEntityData {
}

impl super::entity::GameObjectDataTrait for GhostedProjectileEntityData {
}

impl super::core::DataBusPeerTrait for GhostedProjectileEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GhostedProjectileEntityData {
}

impl super::core::DataContainerTrait for GhostedProjectileEntityData {
}

pub static GHOSTEDPROJECTILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GhostedProjectileEntityData",
    name_hash: 2383646077,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHPROJECTILEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(GhostedProjectileEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GhostedProjectileEntityData as Default>::default())),
            create_boxed: || Box::new(<GhostedProjectileEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ProxyConvergenceDelay",
                name_hash: 427148469,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GhostedProjectileEntityData, proxy_convergence_delay),
            },
            FieldInfoData {
                name: "ProxyConvergenceDuration",
                name_hash: 2536011774,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GhostedProjectileEntityData, proxy_convergence_duration),
            },
            FieldInfoData {
                name: "ProxyConvergenceInstantOnAttach",
                name_hash: 771477649,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GhostedProjectileEntityData, proxy_convergence_instant_on_attach),
            },
            FieldInfoData {
                name: "ForceProxyConvergence",
                name_hash: 2512211421,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GhostedProjectileEntityData, force_proxy_convergence),
            },
            FieldInfoData {
                name: "ConvergenceUsingInitialSpeed",
                name_hash: 609876019,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GhostedProjectileEntityData, convergence_using_initial_speed),
            },
        ],
    }),
    array_type: Some(GHOSTEDPROJECTILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GhostedProjectileEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        GHOSTEDPROJECTILEENTITYDATA_TYPE_INFO
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


pub static GHOSTEDPROJECTILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GhostedProjectileEntityData-Array",
    name_hash: 722459209,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("GhostedProjectileEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MeshProjectileEntityData {
    pub _glacier_base: ProjectileEntityData,
    pub mesh: Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>,
    pub trail_effect: Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>,
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

pub trait MeshProjectileEntityDataTrait: ProjectileEntityDataTrait {
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */>;
    fn trail_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn trail_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */>;
    fn is_attachable(&self) -> &bool;
    fn is_attachable_mut(&mut self) -> &mut bool;
    fn instant_attachable_test_distance(&self) -> &f32;
    fn instant_attachable_test_distance_mut(&mut self) -> &mut f32;
    fn instant_attachable_visual_convergence_delay(&self) -> &f32;
    fn instant_attachable_visual_convergence_delay_mut(&mut self) -> &mut f32;
    fn instant_attachable_visual_convergence_duration(&self) -> &f32;
    fn instant_attachable_visual_convergence_duration_mut(&mut self) -> &mut f32;
    fn instant_attachable_test_under_reticule(&self) -> &bool;
    fn instant_attachable_test_under_reticule_mut(&mut self) -> &mut bool;
    fn max_attachable_inclination(&self) -> &f32;
    fn max_attachable_inclination_mut(&mut self) -> &mut f32;
    fn extra_damping(&self) -> &bool;
    fn extra_damping_mut(&mut self) -> &mut bool;
    fn initial_angular_velocity(&self) -> &super::core::Vec3;
    fn initial_angular_velocity_mut(&mut self) -> &mut super::core::Vec3;
    fn unspawn_after_detonation_delay(&self) -> &f32;
    fn unspawn_after_detonation_delay_mut(&mut self) -> &mut f32;
}

impl MeshProjectileEntityDataTrait for MeshProjectileEntityData {
    fn mesh(&self) -> &Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Option<LockedTypeObject /* super::render_base::MeshBaseAsset */> {
        &mut self.mesh
    }
    fn trail_effect(&self) -> &Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &self.trail_effect
    }
    fn trail_effect_mut(&mut self) -> &mut Option<LockedTypeObject /* super::effect_base::EffectBlueprint */> {
        &mut self.trail_effect
    }
    fn is_attachable(&self) -> &bool {
        &self.is_attachable
    }
    fn is_attachable_mut(&mut self) -> &mut bool {
        &mut self.is_attachable
    }
    fn instant_attachable_test_distance(&self) -> &f32 {
        &self.instant_attachable_test_distance
    }
    fn instant_attachable_test_distance_mut(&mut self) -> &mut f32 {
        &mut self.instant_attachable_test_distance
    }
    fn instant_attachable_visual_convergence_delay(&self) -> &f32 {
        &self.instant_attachable_visual_convergence_delay
    }
    fn instant_attachable_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        &mut self.instant_attachable_visual_convergence_delay
    }
    fn instant_attachable_visual_convergence_duration(&self) -> &f32 {
        &self.instant_attachable_visual_convergence_duration
    }
    fn instant_attachable_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        &mut self.instant_attachable_visual_convergence_duration
    }
    fn instant_attachable_test_under_reticule(&self) -> &bool {
        &self.instant_attachable_test_under_reticule
    }
    fn instant_attachable_test_under_reticule_mut(&mut self) -> &mut bool {
        &mut self.instant_attachable_test_under_reticule
    }
    fn max_attachable_inclination(&self) -> &f32 {
        &self.max_attachable_inclination
    }
    fn max_attachable_inclination_mut(&mut self) -> &mut f32 {
        &mut self.max_attachable_inclination
    }
    fn extra_damping(&self) -> &bool {
        &self.extra_damping
    }
    fn extra_damping_mut(&mut self) -> &mut bool {
        &mut self.extra_damping
    }
    fn initial_angular_velocity(&self) -> &super::core::Vec3 {
        &self.initial_angular_velocity
    }
    fn initial_angular_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.initial_angular_velocity
    }
    fn unspawn_after_detonation_delay(&self) -> &f32 {
        &self.unspawn_after_detonation_delay
    }
    fn unspawn_after_detonation_delay_mut(&mut self) -> &mut f32 {
        &mut self.unspawn_after_detonation_delay
    }
}

impl ProjectileEntityDataTrait for MeshProjectileEntityData {
    fn initial_speed(&self) -> &f32 {
        self._glacier_base.initial_speed()
    }
    fn initial_speed_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_speed_mut()
    }
    fn time_to_live(&self) -> &f32 {
        self._glacier_base.time_to_live()
    }
    fn time_to_live_mut(&mut self) -> &mut f32 {
        self._glacier_base.time_to_live_mut()
    }
    fn max_count(&self) -> &u32 {
        self._glacier_base.max_count()
    }
    fn max_count_mut(&mut self) -> &mut u32 {
        self._glacier_base.max_count_mut()
    }
    fn init_mesh_hide_time(&self) -> &f32 {
        self._glacier_base.init_mesh_hide_time()
    }
    fn init_mesh_hide_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.init_mesh_hide_time_mut()
    }
    fn visual_converge_distance(&self) -> &f32 {
        self._glacier_base.visual_converge_distance()
    }
    fn visual_converge_distance_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_converge_distance_mut()
    }
    fn visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.visual_convergence_delay()
    }
    fn visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_delay_mut()
    }
    fn visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.visual_convergence_duration()
    }
    fn visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.visual_convergence_duration_mut()
    }
    fn proxy_visual_convergence_delay(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_delay()
    }
    fn proxy_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_delay_mut()
    }
    fn proxy_visual_convergence_duration(&self) -> &f32 {
        self._glacier_base.proxy_visual_convergence_duration()
    }
    fn proxy_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.proxy_visual_convergence_duration_mut()
    }
    fn detonate_on_timeout(&self) -> &bool {
        self._glacier_base.detonate_on_timeout()
    }
    fn detonate_on_timeout_mut(&mut self) -> &mut bool {
        self._glacier_base.detonate_on_timeout_mut()
    }
    fn server_projectile_disabled(&self) -> &bool {
        self._glacier_base.server_projectile_disabled()
    }
    fn server_projectile_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.server_projectile_disabled_mut()
    }
    fn explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion()
    }
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        self._glacier_base.explosion_mut()
    }
    fn suppression_data(&self) -> &Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data()
    }
    fn suppression_data_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSuppressionData */> {
        self._glacier_base.suppression_data_mut()
    }
    fn ammunition_type(&self) -> &String {
        self._glacier_base.ammunition_type()
    }
    fn ammunition_type_mut(&mut self) -> &mut String {
        self._glacier_base.ammunition_type_mut()
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        self._glacier_base.material_pair()
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        self._glacier_base.material_pair_mut()
    }
    fn hit_reaction_weapon_type(&self) -> &AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type()
    }
    fn hit_reaction_weapon_type_mut(&mut self) -> &mut AntHitReactionWeaponType {
        self._glacier_base.hit_reaction_weapon_type_mut()
    }
    fn hide_on_detonation(&self) -> &bool {
        self._glacier_base.hide_on_detonation()
    }
    fn hide_on_detonation_mut(&mut self) -> &mut bool {
        self._glacier_base.hide_on_detonation_mut()
    }
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info()
    }
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        self._glacier_base.voice_over_info_mut()
    }
}

impl super::physics::GamePhysicsEntityDataTrait for MeshProjectileEntityData {
}

impl super::entity::GameComponentEntityDataTrait for MeshProjectileEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for MeshProjectileEntityData {
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

impl super::entity::SpatialEntityDataTrait for MeshProjectileEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for MeshProjectileEntityData {
}

impl super::entity::GameObjectDataTrait for MeshProjectileEntityData {
}

impl super::core::DataBusPeerTrait for MeshProjectileEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MeshProjectileEntityData {
}

impl super::core::DataContainerTrait for MeshProjectileEntityData {
}

pub static MESHPROJECTILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProjectileEntityData",
    name_hash: 1001401768,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROJECTILEENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MeshProjectileEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshProjectileEntityData as Default>::default())),
            create_boxed: || Box::new(<MeshProjectileEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Mesh",
                name_hash: 2088783990,
                flags: MemberInfoFlags::new(0),
                field_type: "MeshBaseAsset",
                rust_offset: offset_of!(MeshProjectileEntityData, mesh),
            },
            FieldInfoData {
                name: "TrailEffect",
                name_hash: 3334711472,
                flags: MemberInfoFlags::new(0),
                field_type: "EffectBlueprint",
                rust_offset: offset_of!(MeshProjectileEntityData, trail_effect),
            },
            FieldInfoData {
                name: "IsAttachable",
                name_hash: 2068890974,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshProjectileEntityData, is_attachable),
            },
            FieldInfoData {
                name: "InstantAttachableTestDistance",
                name_hash: 796394410,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_test_distance),
            },
            FieldInfoData {
                name: "InstantAttachableVisualConvergenceDelay",
                name_hash: 3224578007,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_visual_convergence_delay),
            },
            FieldInfoData {
                name: "InstantAttachableVisualConvergenceDuration",
                name_hash: 1305351580,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_visual_convergence_duration),
            },
            FieldInfoData {
                name: "InstantAttachableTestUnderReticule",
                name_hash: 3982945140,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshProjectileEntityData, instant_attachable_test_under_reticule),
            },
            FieldInfoData {
                name: "MaxAttachableInclination",
                name_hash: 39813154,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshProjectileEntityData, max_attachable_inclination),
            },
            FieldInfoData {
                name: "ExtraDamping",
                name_hash: 2980785831,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshProjectileEntityData, extra_damping),
            },
            FieldInfoData {
                name: "InitialAngularVelocity",
                name_hash: 2385892654,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(MeshProjectileEntityData, initial_angular_velocity),
            },
            FieldInfoData {
                name: "UnspawnAfterDetonationDelay",
                name_hash: 2461171741,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MeshProjectileEntityData, unspawn_after_detonation_delay),
            },
        ],
    }),
    array_type: Some(MESHPROJECTILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshProjectileEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MESHPROJECTILEENTITYDATA_TYPE_INFO
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


pub static MESHPROJECTILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshProjectileEntityData-Array",
    name_hash: 1933160220,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MeshProjectileEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MissilePhysicsComponentData {
    pub _glacier_base: super::gameplay_sim::GamePhysicsComponentData,
}

pub trait MissilePhysicsComponentDataTrait: super::gameplay_sim::GamePhysicsComponentDataTrait {
}

impl MissilePhysicsComponentDataTrait for MissilePhysicsComponentData {
}

impl super::gameplay_sim::GamePhysicsComponentDataTrait for MissilePhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn effect_parameters(&self) -> &Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters()
    }
    fn effect_parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::effect_base::EffectParameter */>> {
        self._glacier_base.effect_parameters_mut()
    }
}

impl super::physics::PhysicsComponentDataTrait for MissilePhysicsComponentData {
    fn physics_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies()
    }
    fn physics_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsBodyData */>> {
        self._glacier_base.physics_bodies_mut()
    }
    fn physics_constraints(&self) -> &Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints()
    }
    fn physics_constraints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::PhysicsConstraintData */>> {
        self._glacier_base.physics_constraints_mut()
    }
    fn parts(&self) -> &Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts()
    }
    fn parts_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::physics::PhysicsPartData */> {
        self._glacier_base.parts_mut()
    }
    fn movable_parts(&self) -> &bool {
        self._glacier_base.movable_parts()
    }
    fn movable_parts_mut(&mut self) -> &mut bool {
        self._glacier_base.movable_parts_mut()
    }
    fn internal_collision_disabling(&self) -> &super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling()
    }
    fn internal_collision_disabling_mut(&mut self) -> &mut super::physics::InternalCollisionDisablingBehavior {
        self._glacier_base.internal_collision_disabling_mut()
    }
    fn enable_collision_events(&self) -> &bool {
        self._glacier_base.enable_collision_events()
    }
    fn enable_collision_events_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_collision_events_mut()
    }
}

impl super::entity::ComponentDataTrait for MissilePhysicsComponentData {
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

impl super::entity::GameObjectDataTrait for MissilePhysicsComponentData {
}

impl super::core::DataBusPeerTrait for MissilePhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MissilePhysicsComponentData {
}

impl super::core::DataContainerTrait for MissilePhysicsComponentData {
}

pub static MISSILEPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissilePhysicsComponentData",
    name_hash: 1589290261,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::GAMEPHYSICSCOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(MissilePhysicsComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MissilePhysicsComponentData as Default>::default())),
            create_boxed: || Box::new(<MissilePhysicsComponentData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(MISSILEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MissilePhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        MISSILEPHYSICSCOMPONENTDATA_TYPE_INFO
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


pub static MISSILEPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MissilePhysicsComponentData-Array",
    name_hash: 1553880225,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("MissilePhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProjectileEntityData {
    pub _glacier_base: super::physics::GamePhysicsEntityData,
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
    pub explosion: Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>,
    pub suppression_data: Option<LockedTypeObject /* WeaponSuppressionData */>,
    pub ammunition_type: String,
    pub material_pair: super::entity::MaterialDecl,
    pub hit_reaction_weapon_type: AntHitReactionWeaponType,
    pub hide_on_detonation: bool,
    pub voice_over_info: Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>,
}

pub trait ProjectileEntityDataTrait: super::physics::GamePhysicsEntityDataTrait {
    fn initial_speed(&self) -> &f32;
    fn initial_speed_mut(&mut self) -> &mut f32;
    fn time_to_live(&self) -> &f32;
    fn time_to_live_mut(&mut self) -> &mut f32;
    fn max_count(&self) -> &u32;
    fn max_count_mut(&mut self) -> &mut u32;
    fn init_mesh_hide_time(&self) -> &f32;
    fn init_mesh_hide_time_mut(&mut self) -> &mut f32;
    fn visual_converge_distance(&self) -> &f32;
    fn visual_converge_distance_mut(&mut self) -> &mut f32;
    fn visual_convergence_delay(&self) -> &f32;
    fn visual_convergence_delay_mut(&mut self) -> &mut f32;
    fn visual_convergence_duration(&self) -> &f32;
    fn visual_convergence_duration_mut(&mut self) -> &mut f32;
    fn proxy_visual_convergence_delay(&self) -> &f32;
    fn proxy_visual_convergence_delay_mut(&mut self) -> &mut f32;
    fn proxy_visual_convergence_duration(&self) -> &f32;
    fn proxy_visual_convergence_duration_mut(&mut self) -> &mut f32;
    fn detonate_on_timeout(&self) -> &bool;
    fn detonate_on_timeout_mut(&mut self) -> &mut bool;
    fn server_projectile_disabled(&self) -> &bool;
    fn server_projectile_disabled_mut(&mut self) -> &mut bool;
    fn explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */>;
    fn suppression_data(&self) -> &Option<LockedTypeObject /* WeaponSuppressionData */>;
    fn suppression_data_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSuppressionData */>;
    fn ammunition_type(&self) -> &String;
    fn ammunition_type_mut(&mut self) -> &mut String;
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl;
    fn hit_reaction_weapon_type(&self) -> &AntHitReactionWeaponType;
    fn hit_reaction_weapon_type_mut(&mut self) -> &mut AntHitReactionWeaponType;
    fn hide_on_detonation(&self) -> &bool;
    fn hide_on_detonation_mut(&mut self) -> &mut bool;
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>;
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */>;
}

impl ProjectileEntityDataTrait for ProjectileEntityData {
    fn initial_speed(&self) -> &f32 {
        &self.initial_speed
    }
    fn initial_speed_mut(&mut self) -> &mut f32 {
        &mut self.initial_speed
    }
    fn time_to_live(&self) -> &f32 {
        &self.time_to_live
    }
    fn time_to_live_mut(&mut self) -> &mut f32 {
        &mut self.time_to_live
    }
    fn max_count(&self) -> &u32 {
        &self.max_count
    }
    fn max_count_mut(&mut self) -> &mut u32 {
        &mut self.max_count
    }
    fn init_mesh_hide_time(&self) -> &f32 {
        &self.init_mesh_hide_time
    }
    fn init_mesh_hide_time_mut(&mut self) -> &mut f32 {
        &mut self.init_mesh_hide_time
    }
    fn visual_converge_distance(&self) -> &f32 {
        &self.visual_converge_distance
    }
    fn visual_converge_distance_mut(&mut self) -> &mut f32 {
        &mut self.visual_converge_distance
    }
    fn visual_convergence_delay(&self) -> &f32 {
        &self.visual_convergence_delay
    }
    fn visual_convergence_delay_mut(&mut self) -> &mut f32 {
        &mut self.visual_convergence_delay
    }
    fn visual_convergence_duration(&self) -> &f32 {
        &self.visual_convergence_duration
    }
    fn visual_convergence_duration_mut(&mut self) -> &mut f32 {
        &mut self.visual_convergence_duration
    }
    fn proxy_visual_convergence_delay(&self) -> &f32 {
        &self.proxy_visual_convergence_delay
    }
    fn proxy_visual_convergence_delay_mut(&mut self) -> &mut f32 {
        &mut self.proxy_visual_convergence_delay
    }
    fn proxy_visual_convergence_duration(&self) -> &f32 {
        &self.proxy_visual_convergence_duration
    }
    fn proxy_visual_convergence_duration_mut(&mut self) -> &mut f32 {
        &mut self.proxy_visual_convergence_duration
    }
    fn detonate_on_timeout(&self) -> &bool {
        &self.detonate_on_timeout
    }
    fn detonate_on_timeout_mut(&mut self) -> &mut bool {
        &mut self.detonate_on_timeout
    }
    fn server_projectile_disabled(&self) -> &bool {
        &self.server_projectile_disabled
    }
    fn server_projectile_disabled_mut(&mut self) -> &mut bool {
        &mut self.server_projectile_disabled
    }
    fn explosion(&self) -> &Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &self.explosion
    }
    fn explosion_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::ExplosionEntityData */> {
        &mut self.explosion
    }
    fn suppression_data(&self) -> &Option<LockedTypeObject /* WeaponSuppressionData */> {
        &self.suppression_data
    }
    fn suppression_data_mut(&mut self) -> &mut Option<LockedTypeObject /* WeaponSuppressionData */> {
        &mut self.suppression_data
    }
    fn ammunition_type(&self) -> &String {
        &self.ammunition_type
    }
    fn ammunition_type_mut(&mut self) -> &mut String {
        &mut self.ammunition_type
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.material_pair
    }
    fn hit_reaction_weapon_type(&self) -> &AntHitReactionWeaponType {
        &self.hit_reaction_weapon_type
    }
    fn hit_reaction_weapon_type_mut(&mut self) -> &mut AntHitReactionWeaponType {
        &mut self.hit_reaction_weapon_type
    }
    fn hide_on_detonation(&self) -> &bool {
        &self.hide_on_detonation
    }
    fn hide_on_detonation_mut(&mut self) -> &mut bool {
        &mut self.hide_on_detonation
    }
    fn voice_over_info(&self) -> &Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        &self.voice_over_info
    }
    fn voice_over_info_mut(&mut self) -> &mut Option<LockedTypeObject /* super::audio::EntityVoiceOverInfo */> {
        &mut self.voice_over_info
    }
}

impl super::physics::GamePhysicsEntityDataTrait for ProjectileEntityData {
}

impl super::entity::GameComponentEntityDataTrait for ProjectileEntityData {
    fn enabled(&self) -> &bool {
        self._glacier_base.enabled()
    }
    fn enabled_mut(&mut self) -> &mut bool {
        self._glacier_base.enabled_mut()
    }
}

impl super::entity::ComponentEntityDataTrait for ProjectileEntityData {
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

impl super::entity::SpatialEntityDataTrait for ProjectileEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for ProjectileEntityData {
}

impl super::entity::GameObjectDataTrait for ProjectileEntityData {
}

impl super::core::DataBusPeerTrait for ProjectileEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ProjectileEntityData {
}

impl super::core::DataContainerTrait for ProjectileEntityData {
}

pub static PROJECTILEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileEntityData",
    name_hash: 192635291,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::GAMEPHYSICSENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ProjectileEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProjectileEntityData as Default>::default())),
            create_boxed: || Box::new(<ProjectileEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InitialSpeed",
                name_hash: 3681055580,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, initial_speed),
            },
            FieldInfoData {
                name: "TimeToLive",
                name_hash: 1571456733,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, time_to_live),
            },
            FieldInfoData {
                name: "MaxCount",
                name_hash: 415061138,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ProjectileEntityData, max_count),
            },
            FieldInfoData {
                name: "InitMeshHideTime",
                name_hash: 4189709497,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, init_mesh_hide_time),
            },
            FieldInfoData {
                name: "VisualConvergeDistance",
                name_hash: 4030464179,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, visual_converge_distance),
            },
            FieldInfoData {
                name: "VisualConvergenceDelay",
                name_hash: 4248065837,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, visual_convergence_delay),
            },
            FieldInfoData {
                name: "VisualConvergenceDuration",
                name_hash: 3944650086,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, visual_convergence_duration),
            },
            FieldInfoData {
                name: "ProxyVisualConvergenceDelay",
                name_hash: 613684097,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, proxy_visual_convergence_delay),
            },
            FieldInfoData {
                name: "ProxyVisualConvergenceDuration",
                name_hash: 515861130,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProjectileEntityData, proxy_visual_convergence_duration),
            },
            FieldInfoData {
                name: "DetonateOnTimeout",
                name_hash: 3547425179,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProjectileEntityData, detonate_on_timeout),
            },
            FieldInfoData {
                name: "ServerProjectileDisabled",
                name_hash: 3578279269,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProjectileEntityData, server_projectile_disabled),
            },
            FieldInfoData {
                name: "Explosion",
                name_hash: 2222171184,
                flags: MemberInfoFlags::new(0),
                field_type: "ExplosionEntityData",
                rust_offset: offset_of!(ProjectileEntityData, explosion),
            },
            FieldInfoData {
                name: "SuppressionData",
                name_hash: 1230905484,
                flags: MemberInfoFlags::new(0),
                field_type: "WeaponSuppressionData",
                rust_offset: offset_of!(ProjectileEntityData, suppression_data),
            },
            FieldInfoData {
                name: "AmmunitionType",
                name_hash: 1231027442,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ProjectileEntityData, ammunition_type),
            },
            FieldInfoData {
                name: "MaterialPair",
                name_hash: 161392100,
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(ProjectileEntityData, material_pair),
            },
            FieldInfoData {
                name: "HitReactionWeaponType",
                name_hash: 2133337731,
                flags: MemberInfoFlags::new(0),
                field_type: "AntHitReactionWeaponType",
                rust_offset: offset_of!(ProjectileEntityData, hit_reaction_weapon_type),
            },
            FieldInfoData {
                name: "HideOnDetonation",
                name_hash: 1741071405,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProjectileEntityData, hide_on_detonation),
            },
            FieldInfoData {
                name: "VoiceOverInfo",
                name_hash: 1260547539,
                flags: MemberInfoFlags::new(0),
                field_type: "EntityVoiceOverInfo",
                rust_offset: offset_of!(ProjectileEntityData, voice_over_info),
            },
        ],
    }),
    array_type: Some(PROJECTILEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProjectileEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PROJECTILEENTITYDATA_TYPE_INFO
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


pub static PROJECTILEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileEntityData-Array",
    name_hash: 2526189615,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ProjectileEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProjectileBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
    pub time_delta_type: super::entity::TimeDeltaType,
}

pub trait ProjectileBlueprintTrait: super::entity::ObjectBlueprintTrait {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType;
    fn time_delta_type_mut(&mut self) -> &mut super::entity::TimeDeltaType;
}

impl ProjectileBlueprintTrait for ProjectileBlueprint {
    fn time_delta_type(&self) -> &super::entity::TimeDeltaType {
        &self.time_delta_type
    }
    fn time_delta_type_mut(&mut self) -> &mut super::entity::TimeDeltaType {
        &mut self.time_delta_type
    }
}

impl super::entity::ObjectBlueprintTrait for ProjectileBlueprint {
    fn object(&self) -> &Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object()
    }
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object_mut()
    }
}

impl super::entity::BlueprintTrait for ProjectileBlueprint {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for ProjectileBlueprint {
    fn event_connections(&self) -> &Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for ProjectileBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.flags_mut()
    }
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface_mut()
    }
}

impl super::core::AssetTrait for ProjectileBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProjectileBlueprint {
}

pub static PROJECTILEBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileBlueprint",
    name_hash: 3934002719,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        super_class_offset: offset_of!(ProjectileBlueprint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProjectileBlueprint as Default>::default())),
            create_boxed: || Box::new(<ProjectileBlueprint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TimeDeltaType",
                name_hash: 1100516816,
                flags: MemberInfoFlags::new(0),
                field_type: "TimeDeltaType",
                rust_offset: offset_of!(ProjectileBlueprint, time_delta_type),
            },
        ],
    }),
    array_type: Some(PROJECTILEBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProjectileBlueprint {
    fn type_info(&self) -> &'static TypeInfo {
        PROJECTILEBLUEPRINT_TYPE_INFO
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


pub static PROJECTILEBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectileBlueprint-Array",
    name_hash: 411984555,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("ProjectileBlueprint"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static ANTHITREACTIONWEAPONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntHitReactionWeaponType",
    name_hash: 1418685976,
    flags: MemberInfoFlags::new(49429),
    module: "WeaponShared",
    data: TypeInfoData::Enum,
    array_type: Some(ANTHITREACTIONWEAPONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AntHitReactionWeaponType {
    fn type_info(&self) -> &'static TypeInfo {
        ANTHITREACTIONWEAPONTYPE_TYPE_INFO
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


pub static ANTHITREACTIONWEAPONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AntHitReactionWeaponType-Array",
    name_hash: 1083881516,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("AntHitReactionWeaponType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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

pub trait AmmoConfigDataTrait: TypeObject {
    fn magazine_capacity(&self) -> &i32;
    fn magazine_capacity_mut(&mut self) -> &mut i32;
    fn number_of_magazines(&self) -> &i32;
    fn number_of_magazines_mut(&mut self) -> &mut i32;
    fn spawn_without_ammo(&self) -> &bool;
    fn spawn_without_ammo_mut(&mut self) -> &mut bool;
    fn trace_frequency(&self) -> &u32;
    fn trace_frequency_mut(&mut self) -> &mut u32;
    fn ammo_pickup_min_amount(&self) -> &u32;
    fn ammo_pickup_min_amount_mut(&mut self) -> &mut u32;
    fn ammo_pickup_max_amount(&self) -> &u32;
    fn ammo_pickup_max_amount_mut(&mut self) -> &mut u32;
    fn auto_replenish_magazine(&self) -> &bool;
    fn auto_replenish_magazine_mut(&mut self) -> &mut bool;
    fn auto_replenish_delay(&self) -> &f32;
    fn auto_replenish_delay_mut(&mut self) -> &mut f32;
    fn ammo_bag_pickup_delay_multiplier(&self) -> &f32;
    fn ammo_bag_pickup_delay_multiplier_mut(&mut self) -> &mut f32;
    fn ammo_bag_pickup_amount(&self) -> &i32;
    fn ammo_bag_pickup_amount_mut(&mut self) -> &mut i32;
}

impl AmmoConfigDataTrait for AmmoConfigData {
    fn magazine_capacity(&self) -> &i32 {
        &self.magazine_capacity
    }
    fn magazine_capacity_mut(&mut self) -> &mut i32 {
        &mut self.magazine_capacity
    }
    fn number_of_magazines(&self) -> &i32 {
        &self.number_of_magazines
    }
    fn number_of_magazines_mut(&mut self) -> &mut i32 {
        &mut self.number_of_magazines
    }
    fn spawn_without_ammo(&self) -> &bool {
        &self.spawn_without_ammo
    }
    fn spawn_without_ammo_mut(&mut self) -> &mut bool {
        &mut self.spawn_without_ammo
    }
    fn trace_frequency(&self) -> &u32 {
        &self.trace_frequency
    }
    fn trace_frequency_mut(&mut self) -> &mut u32 {
        &mut self.trace_frequency
    }
    fn ammo_pickup_min_amount(&self) -> &u32 {
        &self.ammo_pickup_min_amount
    }
    fn ammo_pickup_min_amount_mut(&mut self) -> &mut u32 {
        &mut self.ammo_pickup_min_amount
    }
    fn ammo_pickup_max_amount(&self) -> &u32 {
        &self.ammo_pickup_max_amount
    }
    fn ammo_pickup_max_amount_mut(&mut self) -> &mut u32 {
        &mut self.ammo_pickup_max_amount
    }
    fn auto_replenish_magazine(&self) -> &bool {
        &self.auto_replenish_magazine
    }
    fn auto_replenish_magazine_mut(&mut self) -> &mut bool {
        &mut self.auto_replenish_magazine
    }
    fn auto_replenish_delay(&self) -> &f32 {
        &self.auto_replenish_delay
    }
    fn auto_replenish_delay_mut(&mut self) -> &mut f32 {
        &mut self.auto_replenish_delay
    }
    fn ammo_bag_pickup_delay_multiplier(&self) -> &f32 {
        &self.ammo_bag_pickup_delay_multiplier
    }
    fn ammo_bag_pickup_delay_multiplier_mut(&mut self) -> &mut f32 {
        &mut self.ammo_bag_pickup_delay_multiplier
    }
    fn ammo_bag_pickup_amount(&self) -> &i32 {
        &self.ammo_bag_pickup_amount
    }
    fn ammo_bag_pickup_amount_mut(&mut self) -> &mut i32 {
        &mut self.ammo_bag_pickup_amount
    }
}

pub static AMMOCONFIGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AmmoConfigData",
    name_hash: 919227121,
    flags: MemberInfoFlags::new(36937),
    module: "WeaponShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AmmoConfigData as Default>::default())),
            create_boxed: || Box::new(<AmmoConfigData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MagazineCapacity",
                name_hash: 1634670339,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AmmoConfigData, magazine_capacity),
            },
            FieldInfoData {
                name: "NumberOfMagazines",
                name_hash: 2684433166,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AmmoConfigData, number_of_magazines),
            },
            FieldInfoData {
                name: "SpawnWithoutAmmo",
                name_hash: 4137126140,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AmmoConfigData, spawn_without_ammo),
            },
            FieldInfoData {
                name: "TraceFrequency",
                name_hash: 1101083936,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AmmoConfigData, trace_frequency),
            },
            FieldInfoData {
                name: "AmmoPickupMinAmount",
                name_hash: 1049056889,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AmmoConfigData, ammo_pickup_min_amount),
            },
            FieldInfoData {
                name: "AmmoPickupMaxAmount",
                name_hash: 1825618791,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AmmoConfigData, ammo_pickup_max_amount),
            },
            FieldInfoData {
                name: "AutoReplenishMagazine",
                name_hash: 3199294250,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AmmoConfigData, auto_replenish_magazine),
            },
            FieldInfoData {
                name: "AutoReplenishDelay",
                name_hash: 2884902349,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AmmoConfigData, auto_replenish_delay),
            },
            FieldInfoData {
                name: "AmmoBagPickupDelayMultiplier",
                name_hash: 3753352741,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AmmoConfigData, ammo_bag_pickup_delay_multiplier),
            },
            FieldInfoData {
                name: "AmmoBagPickupAmount",
                name_hash: 3198605367,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AmmoConfigData, ammo_bag_pickup_amount),
            },
        ],
    }),
    array_type: Some(AMMOCONFIGDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AmmoConfigData {
    fn type_info(&self) -> &'static TypeInfo {
        AMMOCONFIGDATA_TYPE_INFO
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


pub static AMMOCONFIGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AmmoConfigData-Array",
    name_hash: 3699355077,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("AmmoConfigData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WeaponUnlockAsset {
    pub _glacier_base: super::game_shared::UnlockAssetBase,
}

pub trait WeaponUnlockAssetTrait: super::game_shared::UnlockAssetBaseTrait {
}

impl WeaponUnlockAssetTrait for WeaponUnlockAsset {
}

impl super::game_shared::UnlockAssetBaseTrait for WeaponUnlockAsset {
    fn unlock_user_data(&self) -> &Option<LockedTypeObject /* super::game_shared::UnlockUserDataBase */> {
        self._glacier_base.unlock_user_data()
    }
    fn unlock_user_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::UnlockUserDataBase */> {
        self._glacier_base.unlock_user_data_mut()
    }
    fn debug_unlock_id(&self) -> &String {
        self._glacier_base.debug_unlock_id()
    }
    fn debug_unlock_id_mut(&mut self) -> &mut String {
        self._glacier_base.debug_unlock_id_mut()
    }
    fn identifier(&self) -> &u32 {
        self._glacier_base.identifier()
    }
    fn identifier_mut(&mut self) -> &mut u32 {
        self._glacier_base.identifier_mut()
    }
    fn unlock_score(&self) -> &u32 {
        self._glacier_base.unlock_score()
    }
    fn unlock_score_mut(&mut self) -> &mut u32 {
        self._glacier_base.unlock_score_mut()
    }
    fn auto_available(&self) -> &bool {
        self._glacier_base.auto_available()
    }
    fn auto_available_mut(&mut self) -> &mut bool {
        self._glacier_base.auto_available_mut()
    }
    fn next_level_unlock_asset(&self) -> &Option<LockedTypeObject /* super::game_shared::UnlockAssetBase */> {
        self._glacier_base.next_level_unlock_asset()
    }
    fn next_level_unlock_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::UnlockAssetBase */> {
        self._glacier_base.next_level_unlock_asset_mut()
    }
    fn hidden_in_progression(&self) -> &bool {
        self._glacier_base.hidden_in_progression()
    }
    fn hidden_in_progression_mut(&mut self) -> &mut bool {
        self._glacier_base.hidden_in_progression_mut()
    }
    fn available_for_player(&self) -> &super::game_shared::UnlockAvailability {
        self._glacier_base.available_for_player()
    }
    fn available_for_player_mut(&mut self) -> &mut super::game_shared::UnlockAvailability {
        self._glacier_base.available_for_player_mut()
    }
}

impl super::core::DataContainerPolicyAssetTrait for WeaponUnlockAsset {
}

impl super::core::AssetTrait for WeaponUnlockAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for WeaponUnlockAsset {
}

pub static WEAPONUNLOCKASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlockAsset",
    name_hash: 1477356935,
    flags: MemberInfoFlags::new(101),
    module: "WeaponShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared::UNLOCKASSETBASE_TYPE_INFO),
        super_class_offset: offset_of!(WeaponUnlockAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WeaponUnlockAsset as Default>::default())),
            create_boxed: || Box::new(<WeaponUnlockAsset as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(WEAPONUNLOCKASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WeaponUnlockAsset {
    fn type_info(&self) -> &'static TypeInfo {
        WEAPONUNLOCKASSET_TYPE_INFO
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


pub static WEAPONUNLOCKASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WeaponUnlockAsset-Array",
    name_hash: 513568307,
    flags: MemberInfoFlags::new(145),
    module: "WeaponShared",
    data: TypeInfoData::Array("WeaponUnlockAsset"),
    array_type: None,
    alignment: 8,
};


