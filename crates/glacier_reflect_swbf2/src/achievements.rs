use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceAchievementServiceData {
}

pub static PRESENCEACHIEVEMENTSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementServiceData",
    name_hash: 1575093238,
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceAchievementServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAchievementServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceAchievementServiceData as Default>::default()),
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


pub static PRESENCEACHIEVEMENTSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementServiceData-Array",
    name_hash: 1656654018,
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceAchievementServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceAchievementRequestMessageBase {
}

pub trait PresenceAchievementRequestMessageBaseTrait: TypeObject {
}

impl PresenceAchievementRequestMessageBaseTrait for PresenceAchievementRequestMessageBase {
}

pub static PRESENCEACHIEVEMENTREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementRequestMessageBase",
    name_hash: 485070420,
    flags: MemberInfoFlags::new(36937),
    module: "Achievements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAchievementRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceAchievementRequestMessageBase as Default>::default()),
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
pub struct PresenceAchievementMessageBase {
}

pub trait PresenceAchievementMessageBaseTrait: TypeObject {
}

impl PresenceAchievementMessageBaseTrait for PresenceAchievementMessageBase {
}

pub static PRESENCEACHIEVEMENTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAchievementMessageBase",
    name_hash: 39077029,
    flags: MemberInfoFlags::new(36937),
    module: "Achievements",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAchievementMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceAchievementMessageBase as Default>::default()),
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
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for Xb1AchievementsBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Xb1AchievementsBackendData {
}

pub static XB1ACHIEVEMENTSBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Xb1AchievementsBackendData",
    name_hash: 999074030,
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(Xb1AchievementsBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Xb1AchievementsBackendData as Default>::default())),
            create_boxed: || Box::new(<Xb1AchievementsBackendData as Default>::default()),
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


pub static XB1ACHIEVEMENTSBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Xb1AchievementsBackendData-Array",
    name_hash: 335380954,
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("Xb1AchievementsBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Ps4TrophyBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub enable_trophy_support: bool,
}

pub trait Ps4TrophyBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn enable_trophy_support(&self) -> &bool;
    fn enable_trophy_support_mut(&mut self) -> &mut bool;
}

impl Ps4TrophyBackendDataTrait for Ps4TrophyBackendData {
    fn enable_trophy_support(&self) -> &bool {
        &self.enable_trophy_support
    }
    fn enable_trophy_support_mut(&mut self) -> &mut bool {
        &mut self.enable_trophy_support
    }
}

impl super::online_shared::PresenceBackendDataTrait for Ps4TrophyBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for Ps4TrophyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Ps4TrophyBackendData {
}

pub static PS4TROPHYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4TrophyBackendData",
    name_hash: 4061862958,
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(Ps4TrophyBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Ps4TrophyBackendData as Default>::default())),
            create_boxed: || Box::new(<Ps4TrophyBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnableTrophySupport",
                name_hash: 924025475,
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


pub static PS4TROPHYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4TrophyBackendData-Array",
    name_hash: 2922751130,
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("Ps4TrophyBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2403403238,
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientAchievementService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientAchievementService as Default>::default())),
            create_boxed: || Box::new(<ClientAchievementService as Default>::default()),
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


pub static CLIENTACHIEVEMENTSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientAchievementService-Array",
    name_hash: 71582418,
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("ClientAchievementService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1336468146,
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceUpdateAchievementsRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUpdateAchievementsRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceUpdateAchievementsRequestParameters as Default>::default()),
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


pub static PRESENCEUPDATEACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUpdateAchievementsRequestParameters-Array",
    name_hash: 3044149510,
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceUpdateAchievementsRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 581114867,
    flags: MemberInfoFlags::new(101),
    module: "Achievements",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceUnlockAchievementsRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUnlockAchievementsRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceUnlockAchievementsRequestParameters as Default>::default()),
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


pub static PRESENCEUNLOCKACHIEVEMENTSREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUnlockAchievementsRequestParameters-Array",
    name_hash: 798848967,
    flags: MemberInfoFlags::new(145),
    module: "Achievements",
    data: TypeInfoData::Array("PresenceUnlockAchievementsRequestParameters"),
    array_type: None,
    alignment: 8,
};


