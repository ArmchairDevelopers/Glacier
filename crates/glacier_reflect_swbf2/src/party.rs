use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct PresencePartyServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
    pub user_profile_refresh_timeout: f32,
    pub user_profiles_fetch_enabled: bool,
}

pub trait PresencePartyServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
    fn user_profile_refresh_timeout(&self) -> &f32;
    fn user_profiles_fetch_enabled(&self) -> &bool;
}

impl PresencePartyServiceDataTrait for PresencePartyServiceData {
    fn user_profile_refresh_timeout(&self) -> &f32 {
        &self.user_profile_refresh_timeout
    }
    fn user_profiles_fetch_enabled(&self) -> &bool {
        &self.user_profiles_fetch_enabled
    }
}

impl super::online_shared::PresenceServiceDataTrait for PresencePartyServiceData {
}

impl super::core::AssetTrait for PresencePartyServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PresencePartyServiceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESENCEPARTYSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyServiceData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePartyServiceData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UserProfileRefreshTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PresencePartyServiceData, user_profile_refresh_timeout),
            },
            FieldInfoData {
                name: "UserProfilesFetchEnabled",
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
}


pub static PRESENCEPARTYSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresencePartyServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceDurangoPartyBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub party_session_template: String,
    pub party_invite_context: String,
    pub auto_create_party: bool,
    pub current_activity: DurangoCurrentActivity,
}

pub trait PresenceDurangoPartyBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn party_session_template(&self) -> &String;
    fn party_invite_context(&self) -> &String;
    fn auto_create_party(&self) -> &bool;
    fn current_activity(&self) -> &DurangoCurrentActivity;
}

impl PresenceDurangoPartyBackendDataTrait for PresenceDurangoPartyBackendData {
    fn party_session_template(&self) -> &String {
        &self.party_session_template
    }
    fn party_invite_context(&self) -> &String {
        &self.party_invite_context
    }
    fn auto_create_party(&self) -> &bool {
        &self.auto_create_party
    }
    fn current_activity(&self) -> &DurangoCurrentActivity {
        &self.current_activity
    }
}

impl super::online_shared::PresenceBackendDataTrait for PresenceDurangoPartyBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
}

impl super::core::AssetTrait for PresenceDurangoPartyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PresenceDurangoPartyBackendData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESENCEDURANGOPARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDurangoPartyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDurangoPartyBackendData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PartySessionTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, party_session_template),
            },
            FieldInfoData {
                name: "PartyInviteContext",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, party_invite_context),
            },
            FieldInfoData {
                name: "AutoCreateParty",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PresenceDurangoPartyBackendData, auto_create_party),
            },
            FieldInfoData {
                name: "CurrentActivity",
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
}


pub static PRESENCEDURANGOPARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDurangoPartyBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceDurangoPartyBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
}

impl super::core::AssetTrait for PresencePs4PartyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PresencePs4PartyBackendData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESENCEPS4PARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePs4PartyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePs4PartyBackendData as Default>::default())),
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
}


pub static PRESENCEPS4PARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePs4PartyBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresencePs4PartyBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceOriginPartyBackendData {
    pub _glacier_base: super::online_shared::PresenceBackendData,
    pub party_type: OriginPartyType,
}

pub trait PresenceOriginPartyBackendDataTrait: super::online_shared::PresenceBackendDataTrait {
    fn party_type(&self) -> &OriginPartyType;
}

impl PresenceOriginPartyBackendDataTrait for PresenceOriginPartyBackendData {
    fn party_type(&self) -> &OriginPartyType {
        &self.party_type
    }
}

impl super::online_shared::PresenceBackendDataTrait for PresenceOriginPartyBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
}

impl super::core::AssetTrait for PresenceOriginPartyBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PresenceOriginPartyBackendData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESENCEORIGINPARTYBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginPartyBackendData",
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCEBACKENDDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOriginPartyBackendData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PartyType",
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
}


pub static PRESENCEORIGINPARTYBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginPartyBackendData-Array",
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
}


pub static DURANGOCURRENTACTIVITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoCurrentActivity-Array",
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
}


pub static ORIGINPARTYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("OriginPartyType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresencePartyRequestMessageBase {
}

pub trait PresencePartyRequestMessageBaseTrait: TypeObject {
}

impl PresencePartyRequestMessageBaseTrait for PresencePartyRequestMessageBase {
}

pub static PRESENCEPARTYREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Party",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePartyRequestMessageBase as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct PresencePartyMessageBase {
}

pub trait PresencePartyMessageBaseTrait: TypeObject {
}

impl PresencePartyMessageBaseTrait for PresencePartyMessageBase {
}

pub static PRESENCEPARTYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePartyMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "Party",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePartyMessageBase as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLeavePartyRequestParameters as Default>::default())),
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
}


pub static PRESENCELEAVEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLeavePartyRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceLeavePartyRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAcceptPartyInviteRequestParameters as Default>::default())),
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
}


pub static PRESENCEACCEPTPARTYINVITEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAcceptPartyInviteRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceAcceptPartyInviteRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceSendPartyInvitesRequestParameters as Default>::default())),
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
}


pub static PRESENCESENDPARTYINVITESREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceSendPartyInvitesRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceSendPartyInvitesRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceCreatePartyRequestParameters as Default>::default())),
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
}


pub static PRESENCECREATEPARTYREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCreatePartyRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PresenceCreatePartyRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPartyService as Default>::default())),
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
}


pub static CLIENTPARTYSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPartyService-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("ClientPartyService"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartyEvent as Default>::default())),
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
}


pub static PARTYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartyEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("PartyEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "Party",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEBACKEND_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginPartyBackend as Default>::default())),
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
}


pub static ORIGINPARTYBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPartyBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Party",
    data: TypeInfoData::Array("OriginPartyBackend"),
    array_type: None,
    alignment: 8,
};


