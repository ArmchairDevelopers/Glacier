use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_party_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCEPARTYSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEPARTYSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEDURANGOPARTYBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEDURANGOPARTYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPS4PARTYBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEPS4PARTYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEORIGINPARTYBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEORIGINPARTYBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(DURANGOCURRENTACTIVITY_TYPE_INFO);
    registry.register_type(DURANGOCURRENTACTIVITY_ARRAY_TYPE_INFO);
    registry.register_type(ORIGINPARTYTYPE_TYPE_INFO);
    registry.register_type(ORIGINPARTYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPARTYREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPARTYMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCELEAVEPARTYREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCELEAVEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCESENDPARTYINVITESREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCESENDPARTYINVITESREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCECREATEPARTYREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCECREATEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPARTYSERVICE_TYPE_INFO);
    registry.register_type(CLIENTPARTYSERVICE_ARRAY_TYPE_INFO);
    registry.register_type(PARTYEVENT_TYPE_INFO);
    registry.register_type(PARTYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(ORIGINPARTYBACKEND_TYPE_INFO);
    registry.register_type(ORIGINPARTYBACKEND_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePartyServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
    pub user_profile_refresh_timeout: f32,
    pub user_profiles_fetch_enabled: bool,
}

pub trait PresencePartyServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
    fn user_profile_refresh_timeout(&self) -> &f32;
    fn user_profile_refresh_timeout_mut(&mut self) -> &mut f32;
    fn user_profiles_fetch_enabled(&self) -> &bool;
    fn user_profiles_fetch_enabled_mut(&mut self) -> &mut bool;
}

impl PresencePartyServiceDataTrait for PresencePartyServiceData {
    fn user_profile_refresh_timeout(&self) -> &f32 {
        &self.user_profile_refresh_timeout
    }
    fn user_profile_refresh_timeout_mut(&mut self) -> &mut f32 {
        &mut self.user_profile_refresh_timeout
    }
    fn user_profiles_fetch_enabled(&self) -> &bool {
        &self.user_profiles_fetch_enabled
    }
    fn user_profiles_fetch_enabled_mut(&mut self) -> &mut bool {
        &mut self.user_profiles_fetch_enabled
    }
}

impl super::online_shared::PresenceServiceDataTrait for PresencePartyServiceData {
}

impl super::core::AssetTrait for PresencePartyServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresencePartyServiceData {
}

pub static PRESENCEPARTYSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyServiceData",
    name_hash: 3987226079,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresencePartyServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePartyServiceData as Default>::default())),
            create_boxed: || Box::new(<PresencePartyServiceData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UserProfileRefreshTimeout",
                name_hash: 2110357753,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PresencePartyServiceData, user_profile_refresh_timeout),
            },
            FieldInfoData {
                name: "UserProfilesFetchEnabled",
                name_hash: 1406502357,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PresencePartyServiceData, user_profiles_fetch_enabled),
            },
        ],
    }),
    array_type: Some(PRESENCEPARTYSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePartyServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPARTYSERVICEDATA_TYPE_INFO
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


pub static PRESENCEPARTYSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyServiceData-Array",
    name_hash: 3430093547,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresencePartyServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceDurangoPartyBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub party_session_template: String,
    pub party_invite_context: String,
    pub auto_create_party: bool,
    pub current_activity: DurangoCurrentActivity,
}

pub trait PresenceDurangoPartyBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn party_session_template(&self) -> &String;
    fn party_session_template_mut(&mut self) -> &mut String;
    fn party_invite_context(&self) -> &String;
    fn party_invite_context_mut(&mut self) -> &mut String;
    fn auto_create_party(&self) -> &bool;
    fn auto_create_party_mut(&mut self) -> &mut bool;
    fn current_activity(&self) -> &DurangoCurrentActivity;
    fn current_activity_mut(&mut self) -> &mut DurangoCurrentActivity;
}

impl PresenceDurangoPartyBackendDataTrait for PresenceDurangoPartyBackendData {
    fn party_session_template(&self) -> &String {
        &self.party_session_template
    }
    fn party_session_template_mut(&mut self) -> &mut String {
        &mut self.party_session_template
    }
    fn party_invite_context(&self) -> &String {
        &self.party_invite_context
    }
    fn party_invite_context_mut(&mut self) -> &mut String {
        &mut self.party_invite_context
    }
    fn auto_create_party(&self) -> &bool {
        &self.auto_create_party
    }
    fn auto_create_party_mut(&mut self) -> &mut bool {
        &mut self.auto_create_party
    }
    fn current_activity(&self) -> &DurangoCurrentActivity {
        &self.current_activity
    }
    fn current_activity_mut(&mut self) -> &mut DurangoCurrentActivity {
        &mut self.current_activity
    }
}

impl super::online_shared::PresenceBackendDataTrait for PresenceDurangoPartyBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for PresenceDurangoPartyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceDurangoPartyBackendData {
}

pub static PRESENCEDURANGOPARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDurangoPartyBackendData",
    name_hash: 2553248546,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceDurangoPartyBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDurangoPartyBackendData as Default>::default())),
            create_boxed: || Box::new(<PresenceDurangoPartyBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PartySessionTemplate",
                name_hash: 3942993925,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, party_session_template),
            },
            FieldInfoData {
                name: "PartyInviteContext",
                name_hash: 1034706621,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, party_invite_context),
            },
            FieldInfoData {
                name: "AutoCreateParty",
                name_hash: 136368704,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, auto_create_party),
            },
            FieldInfoData {
                name: "CurrentActivity",
                name_hash: 3644106881,
                flags: MemberInfoFlags::new(0),
                field_type: "DurangoCurrentActivity",
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, current_activity),
            },
        ],
    }),
    array_type: Some(PRESENCEDURANGOPARTYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceDurangoPartyBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEDURANGOPARTYBACKENDDATA_TYPE_INFO
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


pub static PRESENCEDURANGOPARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDurangoPartyBackendData-Array",
    name_hash: 202783126,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceDurangoPartyBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePs4PartyBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
}

pub trait PresencePs4PartyBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
}

impl PresencePs4PartyBackendDataTrait for PresencePs4PartyBackendData {
}

impl super::online_shared::PresenceBackendDataTrait for PresencePs4PartyBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for PresencePs4PartyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresencePs4PartyBackendData {
}

pub static PRESENCEPS4PARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePs4PartyBackendData",
    name_hash: 3438045489,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresencePs4PartyBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePs4PartyBackendData as Default>::default())),
            create_boxed: || Box::new(<PresencePs4PartyBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPS4PARTYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePs4PartyBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPS4PARTYBACKENDDATA_TYPE_INFO
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


pub static PRESENCEPS4PARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePs4PartyBackendData-Array",
    name_hash: 2357148549,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresencePs4PartyBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceOriginPartyBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub party_type: OriginPartyType,
}

pub trait PresenceOriginPartyBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn party_type(&self) -> &OriginPartyType;
    fn party_type_mut(&mut self) -> &mut OriginPartyType;
}

impl PresenceOriginPartyBackendDataTrait for PresenceOriginPartyBackendData {
    fn party_type(&self) -> &OriginPartyType {
        &self.party_type
    }
    fn party_type_mut(&mut self) -> &mut OriginPartyType {
        &mut self.party_type
    }
}

impl super::online_shared::PresenceBackendDataTrait for PresenceOriginPartyBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for PresenceOriginPartyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceOriginPartyBackendData {
}

pub static PRESENCEORIGINPARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginPartyBackendData",
    name_hash: 3534487570,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceOriginPartyBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOriginPartyBackendData as Default>::default())),
            create_boxed: || Box::new(<PresenceOriginPartyBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PartyType",
                name_hash: 3152136947,
                flags: MemberInfoFlags::new(0),
                field_type: "OriginPartyType",
                rust_offset: offset_of!(PresenceOriginPartyBackendData, party_type),
            },
        ],
    }),
    array_type: Some(PRESENCEORIGINPARTYBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceOriginPartyBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEORIGINPARTYBACKENDDATA_TYPE_INFO
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


pub static PRESENCEORIGINPARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginPartyBackendData-Array",
    name_hash: 4052330278,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceOriginPartyBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DurangoCurrentActivity {
    #[default]
    DurangoCurrentActivity_Game = 0,
    DurangoCurrentActivity_Party = 1,
    DurangoCurrentActivity_Count = 2,
}

pub static DURANGOCURRENTACTIVITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoCurrentActivity",
    name_hash: 1531607877,
    flags: MemberInfoFlags::new(49429),
    module: "Party",
    data: TypeInfoData::Enum,
    array_type: Some(DURANGOCURRENTACTIVITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DurangoCurrentActivity {
    fn type_info(&self) -> &'static TypeInfo {
        DURANGOCURRENTACTIVITY_TYPE_INFO
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


pub static DURANGOCURRENTACTIVITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoCurrentActivity-Array",
    name_hash: 3715044209,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("DurangoCurrentActivity"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OriginPartyType {
    #[default]
    OriginPartyType_Public = 0,
    OriginPartyType_Private = 1,
    OriginPartyType_Count = 2,
}

pub static ORIGINPARTYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyType",
    name_hash: 2490330983,
    flags: MemberInfoFlags::new(49429),
    module: "Party",
    data: TypeInfoData::Enum,
    array_type: Some(ORIGINPARTYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OriginPartyType {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINPARTYTYPE_TYPE_INFO
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


pub static ORIGINPARTYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyType-Array",
    name_hash: 3135721811,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("OriginPartyType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePartyRequestMessageBase {
}

pub trait PresencePartyRequestMessageBaseTrait: TypeObject {
}

impl PresencePartyRequestMessageBaseTrait for PresencePartyRequestMessageBase {
}

pub static PRESENCEPARTYREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyRequestMessageBase",
    name_hash: 1354325085,
    flags: MemberInfoFlags::new(36937),
    module: "Party",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePartyRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePartyRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePartyRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPARTYREQUESTMESSAGEBASE_TYPE_INFO
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
pub struct PresencePartyMessageBase {
}

pub trait PresencePartyMessageBaseTrait: TypeObject {
}

impl PresencePartyMessageBaseTrait for PresencePartyMessageBase {
}

pub static PRESENCEPARTYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyMessageBase",
    name_hash: 4196171916,
    flags: MemberInfoFlags::new(36937),
    module: "Party",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePartyMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePartyMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePartyMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPARTYMESSAGEBASE_TYPE_INFO
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
pub struct PresenceLeavePartyRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceLeavePartyRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceLeavePartyRequestParametersTrait for PresenceLeavePartyRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceLeavePartyRequestParameters {
}

pub static PRESENCELEAVEPARTYREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeavePartyRequestParameters",
    name_hash: 3062780834,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceLeavePartyRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLeavePartyRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceLeavePartyRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCELEAVEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceLeavePartyRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCELEAVEPARTYREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCELEAVEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeavePartyRequestParameters-Array",
    name_hash: 2572513814,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceLeavePartyRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceAcceptPartyInviteRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceAcceptPartyInviteRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceAcceptPartyInviteRequestParametersTrait for PresenceAcceptPartyInviteRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceAcceptPartyInviteRequestParameters {
}

pub static PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptPartyInviteRequestParameters",
    name_hash: 3550397072,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceAcceptPartyInviteRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAcceptPartyInviteRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceAcceptPartyInviteRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceAcceptPartyInviteRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptPartyInviteRequestParameters-Array",
    name_hash: 3849851044,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceAcceptPartyInviteRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceSendPartyInvitesRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceSendPartyInvitesRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceSendPartyInvitesRequestParametersTrait for PresenceSendPartyInvitesRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceSendPartyInvitesRequestParameters {
}

pub static PRESENCESENDPARTYINVITESREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendPartyInvitesRequestParameters",
    name_hash: 2756107999,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceSendPartyInvitesRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceSendPartyInvitesRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceSendPartyInvitesRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESENDPARTYINVITESREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceSendPartyInvitesRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESENDPARTYINVITESREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCESENDPARTYINVITESREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendPartyInvitesRequestParameters-Array",
    name_hash: 3787879403,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceSendPartyInvitesRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceCreatePartyRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceCreatePartyRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceCreatePartyRequestParametersTrait for PresenceCreatePartyRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceCreatePartyRequestParameters {
}

pub static PRESENCECREATEPARTYREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreatePartyRequestParameters",
    name_hash: 3165794429,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceCreatePartyRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceCreatePartyRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceCreatePartyRequestParameters as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECREATEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceCreatePartyRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCECREATEPARTYREQUESTPARAMETERS_TYPE_INFO
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


pub static PRESENCECREATEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreatePartyRequestParameters-Array",
    name_hash: 2900147529,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceCreatePartyRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientPartyService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientPartyServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientPartyServiceTrait for ClientPartyService {
}

impl super::online::PresenceServiceTrait for ClientPartyService {
}

pub static CLIENTPARTYSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartyService",
    name_hash: 560627023,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientPartyService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPartyService as Default>::default())),
            create_boxed: || Box::new(<ClientPartyService as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPARTYSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPartyService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPARTYSERVICE_TYPE_INFO
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


pub static CLIENTPARTYSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartyService-Array",
    name_hash: 2352228987,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("ClientPartyService"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PartyEvent {
    pub _glacier_base: super::online::PresenceEvent,
}

pub trait PartyEventTrait: super::online::PresenceEventTrait {
}

impl PartyEventTrait for PartyEvent {
}

impl super::online::PresenceEventTrait for PartyEvent {
}

pub static PARTYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartyEvent",
    name_hash: 915558823,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEEVENT_TYPE_INFO),
        super_class_offset: offset_of!(PartyEvent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartyEvent as Default>::default())),
            create_boxed: || Box::new(<PartyEvent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PARTYEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PartyEvent {
    fn type_info(&self) -> &'static TypeInfo {
        PARTYEVENT_TYPE_INFO
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


pub static PARTYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartyEvent-Array",
    name_hash: 234283795,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PartyEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OriginPartyBackend {
    pub _glacier_base: super::online::PresenceBackend,
}

pub trait OriginPartyBackendTrait: super::online::PresenceBackendTrait {
}

impl OriginPartyBackendTrait for OriginPartyBackend {
}

impl super::online::PresenceBackendTrait for OriginPartyBackend {
}

pub static ORIGINPARTYBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyBackend",
    name_hash: 1554749051,
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEBACKEND_TYPE_INFO),
        super_class_offset: offset_of!(OriginPartyBackend, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginPartyBackend as Default>::default())),
            create_boxed: || Box::new(<OriginPartyBackend as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ORIGINPARTYBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OriginPartyBackend {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINPARTYBACKEND_TYPE_INFO
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


pub static ORIGINPARTYBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyBackend-Array",
    name_hash: 3493277263,
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("OriginPartyBackend"),
    array_type: None,
    alignment: 8,
};


