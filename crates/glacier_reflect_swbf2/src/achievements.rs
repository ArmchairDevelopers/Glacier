use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_achievements_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCEACHIEVEMENTSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEACHIEVEMENTSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEACHIEVEMENTREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEACHIEVEMENTMESSAGEBASE_TYPE_INFO);
    registry.register_type(XB1ACHIEVEMENTSBACKENDDATA_TYPE_INFO);
    registry.register_type(XB1ACHIEVEMENTSBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PS4TROPHYBACKENDDATA_TYPE_INFO);
    registry.register_type(PS4TROPHYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTACHIEVEMENTSERVICE_TYPE_INFO);
    registry.register_type(CLIENTACHIEVEMENTSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct PresenceAchievementServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
}

pub trait PresenceAchievementServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
}

impl PresenceAchievementServiceDataTrait for PresenceAchievementServiceData {
}

impl super::online_shared::PresenceServiceDataTrait for PresenceAchievementServiceData {
}

impl super::core::AssetTrait for PresenceAchievementServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PresenceAchievementServiceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESENCEACHIEVEMENTSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAchievementServiceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACHIEVEMENTSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceAchievementServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACHIEVEMENTSERVICEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCEACHIEVEMENTSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceAchievementServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceAchievementRequestMessageBase {
}

pub trait PresenceAchievementRequestMessageBaseTrait: TypeObject {
}

impl PresenceAchievementRequestMessageBaseTrait for PresenceAchievementRequestMessageBase {
}

pub static PRESENCEACHIEVEMENTREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Achievements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAchievementRequestMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAchievementRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACHIEVEMENTREQUESTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PresenceAchievementMessageBase {
}

pub trait PresenceAchievementMessageBaseTrait: TypeObject {
}

impl PresenceAchievementMessageBaseTrait for PresenceAchievementMessageBase {
}

pub static PRESENCEACHIEVEMENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Achievements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAchievementMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAchievementMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACHIEVEMENTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Xb1AchievementsBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
}

pub trait Xb1AchievementsBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
}

impl Xb1AchievementsBackendDataTrait for Xb1AchievementsBackendData {
}

impl super::online_shared::PresenceBackendDataTrait for Xb1AchievementsBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
}

impl super::core::AssetTrait for Xb1AchievementsBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for Xb1AchievementsBackendData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static XB1ACHIEVEMENTSBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Xb1AchievementsBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Xb1AchievementsBackendData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(XB1ACHIEVEMENTSBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Xb1AchievementsBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        XB1ACHIEVEMENTSBACKENDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static XB1ACHIEVEMENTSBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Xb1AchievementsBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("Xb1AchievementsBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Ps4TrophyBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub enable_trophy_support: bool,
}

pub trait Ps4TrophyBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn enable_trophy_support(&self) -> &bool;
}

impl Ps4TrophyBackendDataTrait for Ps4TrophyBackendData {
    fn enable_trophy_support(&self) -> &bool {
        &self.enable_trophy_support
    }
}

impl super::online_shared::PresenceBackendDataTrait for Ps4TrophyBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
}

impl super::core::AssetTrait for Ps4TrophyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for Ps4TrophyBackendData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PS4TROPHYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4TrophyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Ps4TrophyBackendData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EnableTrophySupport",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Ps4TrophyBackendData, enable_trophy_support),
            },
        ],
    }),
    array_type: Some(PS4TROPHYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4TrophyBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PS4TROPHYBACKENDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PS4TROPHYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4TrophyBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("Ps4TrophyBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientAchievementService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientAchievementServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientAchievementServiceTrait for ClientAchievementService {
}

impl super::online::PresenceServiceTrait for ClientAchievementService {
}

pub static CLIENTACHIEVEMENTSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAchievementService",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAchievementService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTACHIEVEMENTSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientAchievementService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTACHIEVEMENTSERVICE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTACHIEVEMENTSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAchievementService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("ClientAchievementService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceUpdateAchievementsRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceUpdateAchievementsRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceUpdateAchievementsRequestParametersTrait for PresenceUpdateAchievementsRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceUpdateAchievementsRequestParameters {
}

pub static PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUpdateAchievementsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUpdateAchievementsRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceUpdateAchievementsRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUpdateAchievementsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceUpdateAchievementsRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceUnlockAchievementsRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceUnlockAchievementsRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceUnlockAchievementsRequestParametersTrait for PresenceUnlockAchievementsRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceUnlockAchievementsRequestParameters {
}

pub static PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUnlockAchievementsRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUnlockAchievementsRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceUnlockAchievementsRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUnlockAchievementsRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceUnlockAchievementsRequestParameters"),
    array_type: None,
    alignment: 8,
};


