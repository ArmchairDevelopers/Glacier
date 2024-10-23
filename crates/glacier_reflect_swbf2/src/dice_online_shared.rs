use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_dice_online_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(DICEONLINESETTINGS_TYPE_INFO);
    registry.register_type(DICEONLINESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DICEONLINELOGLEVELT_TYPE_INFO);
    registry.register_type(DICEONLINELOGLEVELT_ARRAY_TYPE_INFO);
    registry.register_type(AWARDGROUP_TYPE_INFO);
    registry.register_type(AWARDGROUP_ARRAY_TYPE_INFO);
    registry.register_type(STARLEVELCATEGORY_TYPE_INFO);
    registry.register_type(STARLEVELCATEGORY_ARRAY_TYPE_INFO);
    registry.register_type(WSCLASS_TYPE_INFO);
    registry.register_type(WSCLASS_ARRAY_TYPE_INFO);
    registry.register_type(INVENTORYPROGRESS_TYPE_INFO);
    registry.register_type(INVENTORYPROGRESS_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEITEMTYPE_TYPE_INFO);
    registry.register_type(ONLINEITEMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(VIRTUALCURRENCY_TYPE_INFO);
    registry.register_type(VIRTUALCURRENCY_ARRAY_TYPE_INFO);
    registry.register_type(RARITYTYPE_TYPE_INFO);
    registry.register_type(RARITYTYPE_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct DiceOnlineSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub log_level: DiceOnlineLogLevelT,
}

pub trait DiceOnlineSettingsTrait: super::core::SystemSettingsTrait {
    fn log_level(&self) -> &DiceOnlineLogLevelT;
    fn log_level_mut(&mut self) -> &mut DiceOnlineLogLevelT;
}

impl DiceOnlineSettingsTrait for DiceOnlineSettings {
    fn log_level(&self) -> &DiceOnlineLogLevelT {
        &self.log_level
    }
    fn log_level_mut(&mut self) -> &mut DiceOnlineLogLevelT {
        &mut self.log_level
    }
}

impl super::core::SystemSettingsTrait for DiceOnlineSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for DiceOnlineSettings {
}

pub static DICEONLINESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceOnlineSettings",
    name_hash: 1987900292,
    flags: MemberInfoFlags::new(101),
    module: "DiceOnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(DiceOnlineSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DiceOnlineSettings as Default>::default())),
            create_boxed: || Box::new(<DiceOnlineSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LogLevel",
                name_hash: 1867701815,
                flags: MemberInfoFlags::new(0),
                field_type: "DiceOnlineLogLevelT",
                rust_offset: offset_of!(DiceOnlineSettings, log_level),
            },
        ],
    }),
    array_type: Some(DICEONLINESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DiceOnlineSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DICEONLINESETTINGS_TYPE_INFO
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


pub static DICEONLINESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceOnlineSettings-Array",
    name_hash: 3995330480,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("DiceOnlineSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DiceOnlineLogLevelT {
    #[default]
    LLevel_Default = 0,
    LLevel_Fatal = 1,
    LLevel_Error = 2,
    LLevel_Warn = 3,
    LLevel_Info = 4,
    LLevel_Debug = 5,
    LLevel_Trace = 6,
}

pub static DICEONLINELOGLEVELT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceOnlineLogLevelT",
    name_hash: 744470983,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(DICEONLINELOGLEVELT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DiceOnlineLogLevelT {
    fn type_info(&self) -> &'static TypeInfo {
        DICEONLINELOGLEVELT_TYPE_INFO
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


pub static DICEONLINELOGLEVELT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DiceOnlineLogLevelT-Array",
    name_hash: 465922803,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("DiceOnlineLogLevelT"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AwardGroup {
    #[default]
    AwardGroup_Undefined = 0,
    AwardGroup_Achievements = 1,
    AwardGroup_Honors = 2,
    AwardGroup_Challenges = 3,
    AwardGroup_StarLevel = 4,
    AwardGroup_Gate = 5,
    AwardGroup_DailyOrder = 6,
    AwardGroup_Community = 7,
    AwardGroup_Milestones = 8,
    AwardGroup_Collection = 9,
    AwardGroup_Rankup = 10,
    AwardGroup_LastAwardGroup = 11,
}

pub static AWARDGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AwardGroup",
    name_hash: 441163323,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(AWARDGROUP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AwardGroup {
    fn type_info(&self) -> &'static TypeInfo {
        AWARDGROUP_TYPE_INFO
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


pub static AWARDGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AwardGroup-Array",
    name_hash: 3496609167,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("AwardGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum StarLevelCategory {
    #[default]
    StarLevelCategory_global = 0,
    StarLevelCategory_Assault = 1,
    StarLevelCategory_Heavy = 2,
    StarLevelCategory_Officer = 3,
    StarLevelCategory_Specialist = 4,
    StarLevelCategory_Enforcer = 5,
    StarLevelCategory_Aerial = 6,
    StarLevelCategory_GroundVehicle = 7,
    StarLevelCategory_Artillery = 8,
    StarLevelCategory_Speeder = 9,
    StarLevelCategory_Fighter = 10,
    StarLevelCategory_Interceptor = 11,
    StarLevelCategory_Bomber = 12,
    StarLevelCategory_BobaFett = 13,
    StarLevelCategory_Bossk = 14,
    StarLevelCategory_Chewbacca = 15,
    StarLevelCategory_DarthVader = 16,
    StarLevelCategory_Emperor = 17,
    StarLevelCategory_HanSolo = 18,
    StarLevelCategory_Iden = 19,
    StarLevelCategory_KyloRen = 20,
    StarLevelCategory_Lando = 21,
    StarLevelCategory_Leia = 22,
    StarLevelCategory_Luke = 23,
    StarLevelCategory_Maul = 24,
    StarLevelCategory_Rey = 25,
    StarLevelCategory_Yoda = 26,
    StarLevelCategory_Finn = 27,
    StarLevelCategory_Phasma = 28,
    StarLevelCategory_GrizzlyBoat = 29,
    StarLevelCategory_MillenniumFalcon = 30,
    StarLevelCategory_MillenniumFalconNT = 31,
    StarLevelCategory_Scimitar = 32,
    StarLevelCategory_Slave1 = 33,
    StarLevelCategory_TIEAdvanced = 34,
    StarLevelCategory_Red5 = 35,
    StarLevelCategory_YodasStarfighter = 36,
    StarLevelCategory_QueenHoneyBee = 37,
    StarLevelCategory_BlackOne = 38,
    StarLevelCategory_IdensTIE = 39,
    StarLevelCategory_MillenniumFalconRC = 40,
    StarLevelCategory_Ewok = 41,
    StarLevelCategory_Anakin = 42,
    StarLevelCategory_Dooku = 43,
    StarLevelCategory_Grievous = 44,
    StarLevelCategory_ObiWan = 45,
    StarLevelCategory_Infiltrator = 46,
    StarLevelCategory_BB8 = 47,
    StarLevelCategory_BB9E = 48,
    StarLevelCategory_Count = 49,
    StarLevelCategory_None = 50,
    StarLevelCategory_Invalid = 51,
}

pub static STARLEVELCATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StarLevelCategory",
    name_hash: 2720650807,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(STARLEVELCATEGORY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StarLevelCategory {
    fn type_info(&self) -> &'static TypeInfo {
        STARLEVELCATEGORY_TYPE_INFO
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


pub static STARLEVELCATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StarLevelCategory-Array",
    name_hash: 3805450115,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("StarLevelCategory"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WSClass {
    #[default]
    WSClass_Assault = 0,
    WSClass_Heavy = 1,
    WSClass_Officer = 2,
    WSClass_Specialist = 3,
    WSClass_Enforcer = 4,
    WSClass_Aerial = 5,
    WSClass_GroundVehicle = 6,
    WSClass_Artillery = 7,
    WSClass_Speeder = 8,
    WSClass_Fighter = 9,
    WSClass_Interceptor = 10,
    WSClass_Bomber = 11,
    WSClass_BobaFett = 12,
    WSClass_Bossk = 13,
    WSClass_Chewbacca = 14,
    WSClass_DarthVader = 15,
    WSClass_Emperor = 16,
    WSClass_HanSolo = 17,
    WSClass_Iden = 18,
    WSClass_KyloRen = 19,
    WSClass_Lando = 20,
    WSClass_Leia = 21,
    WSClass_Luke = 22,
    WSClass_Maul = 23,
    WSClass_Rey = 24,
    WSClass_Yoda = 25,
    WSClass_Finn = 26,
    WSClass_Phasma = 27,
    WSClass_GrizzlyBoat = 28,
    WSClass_MillenniumFalcon = 29,
    WSClass_MillenniumFalconNT = 30,
    WSClass_Scimitar = 31,
    WSClass_Slave1 = 32,
    WSClass_TIEAdvanced = 33,
    WSClass_Red5 = 34,
    WSClass_YodasStarfighter = 35,
    WSClass_QueenHoneyBee = 36,
    WSClass_BlackOne = 37,
    WSClass_IdensTIE = 38,
    WSClass_MillenniumFalconRC = 39,
    WSClass_Ewok = 40,
    WSClass_Anakin = 41,
    WSClass_Dooku = 42,
    WSClass_Grievous = 43,
    WSClass_ObiWan = 44,
    WSClass_Infiltrator = 45,
    WSClass_BB8 = 46,
    WSClass_BB9E = 47,
    WSClass_Count = 48,
    WSClass_None = 49,
    WSClass_Invalid = 50,
}

pub static WSCLASS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WSClass",
    name_hash: 2642042031,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(WSCLASS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WSClass {
    fn type_info(&self) -> &'static TypeInfo {
        WSCLASS_TYPE_INFO
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


pub static WSCLASS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WSClass-Array",
    name_hash: 100305435,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("WSClass"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InventoryProgress {
    #[default]
    InventoryProgress_NotDownloaded = 0,
    InventoryProgress_Downloaded = 1,
    InventoryProgress_Failed = 2,
}

pub static INVENTORYPROGRESS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InventoryProgress",
    name_hash: 919281138,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVENTORYPROGRESS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InventoryProgress {
    fn type_info(&self) -> &'static TypeInfo {
        INVENTORYPROGRESS_TYPE_INFO
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


pub static INVENTORYPROGRESS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InventoryProgress-Array",
    name_hash: 3478353094,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("InventoryProgress"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OnlineItemType {
    #[default]
    OnlineItemType_None = 0,
    OnlineItemType_StarCard = 1,
    OnlineItemType_TrooperWeapon = 2,
    OnlineItemType_Emote = 3,
    OnlineItemType_VictoryCelebration = 4,
    OnlineItemType_VoiceLine = 5,
    OnlineItemType_Skin = 6,
    OnlineItemType_Credits = 7,
    OnlineItemType_Crystals = 8,
    OnlineItemType_BasicCraftingMaterials = 9,
    OnlineItemType_EpicCraftingMaterials = 10,
    OnlineItemType_CheatJabbaToken = 11,
    OnlineItemType_WeaponAttachment = 12,
    OnlineItemType_Gate = 13,
    OnlineItemType_SkillPoint = 14,
    OnlineItemType_Count = 15,
}

pub static ONLINEITEMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineItemType",
    name_hash: 837601351,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(ONLINEITEMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OnlineItemType {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEITEMTYPE_TYPE_INFO
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


pub static ONLINEITEMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineItemType-Array",
    name_hash: 2206423411,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("OnlineItemType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum VirtualCurrency {
    #[default]
    VirtualCurrency_None = 0,
    VirtualCurrency_Crystals = 1,
    VirtualCurrency_Credits = 2,
    VirtualCurrency_MaterialsCommon = 3,
    VirtualCurrency_MaterialsRare = 4,
    VirtualCurrency_Item = 5,
    VirtualCurrency_SkillPoints = 6,
}

pub static VIRTUALCURRENCY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualCurrency",
    name_hash: 463778979,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(VIRTUALCURRENCY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VirtualCurrency {
    fn type_info(&self) -> &'static TypeInfo {
        VIRTUALCURRENCY_TYPE_INFO
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


pub static VIRTUALCURRENCY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VirtualCurrency-Array",
    name_hash: 2630342679,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("VirtualCurrency"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RarityType {
    #[default]
    RarityType_Common = 0,
    RarityType_Uncommon = 1,
    RarityType_Rare = 2,
    RarityType_Epic = 3,
    RarityType_Legendary = 4,
    RarityType_Event = 5,
    RarityType_Count = 6,
}

pub static RARITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RarityType",
    name_hash: 3677493464,
    flags: MemberInfoFlags::new(49429),
    module: "DiceOnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(RARITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RarityType {
    fn type_info(&self) -> &'static TypeInfo {
        RARITYTYPE_TYPE_INFO
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


pub static RARITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RarityType-Array",
    name_hash: 2844160876,
    flags: MemberInfoFlags::new(145),
    module: "DiceOnlineShared",
    data: TypeInfoData::Array("RarityType"),
    array_type: None,
    alignment: 8,
};


