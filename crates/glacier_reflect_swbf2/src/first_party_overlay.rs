use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_first_party_overlay_types(registry: &mut TypeRegistry) {
    registry.register_type(PRESENCEOVERLAYSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEOVERLAYSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEOVERLAYREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEOVERLAYMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCESHOWINVITEUIREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCESHOWINVITEUIREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_TYPE_INFO);
    registry.register_type(PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTOVERLAYSERVICE_TYPE_INFO);
    registry.register_type(CLIENTOVERLAYSERVICE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct PresenceOverlayServiceData {
    pub _glacier_base: super::online_shared::PresenceServiceData,
}

pub trait PresenceOverlayServiceDataTrait: super::online_shared::PresenceServiceDataTrait {
}

impl PresenceOverlayServiceDataTrait for PresenceOverlayServiceData {
}

impl super::online_shared::PresenceServiceDataTrait for PresenceOverlayServiceData {
}

impl super::core::AssetTrait for PresenceOverlayServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PresenceOverlayServiceData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRESENCEOVERLAYSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayServiceData",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOverlayServiceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEOVERLAYSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceOverlayServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEOVERLAYSERVICEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCEOVERLAYSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceOverlayServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceOverlayRequestMessageBase {
}

pub trait PresenceOverlayRequestMessageBaseTrait: TypeObject {
}

impl PresenceOverlayRequestMessageBaseTrait for PresenceOverlayRequestMessageBase {
}

pub static PRESENCEOVERLAYREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "FirstPartyOverlay",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOverlayRequestMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOverlayRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEOVERLAYREQUESTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PresenceOverlayMessageBase {
}

pub trait PresenceOverlayMessageBaseTrait: TypeObject {
}

impl PresenceOverlayMessageBaseTrait for PresenceOverlayMessageBase {
}

pub static PRESENCEOVERLAYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "FirstPartyOverlay",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOverlayMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOverlayMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEOVERLAYMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct PresenceShowInviteUIRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceShowInviteUIRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceShowInviteUIRequestParametersTrait for PresenceShowInviteUIRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceShowInviteUIRequestParameters {
}

pub static PRESENCESHOWINVITEUIREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceShowInviteUIRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceShowInviteUIRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESHOWINVITEUIREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceShowInviteUIRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESHOWINVITEUIREQUESTPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCESHOWINVITEUIREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceShowInviteUIRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceShowInviteUIRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceDisplayFriendRequestDialogRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceDisplayFriendRequestDialogRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceDisplayFriendRequestDialogRequestParametersTrait for PresenceDisplayFriendRequestDialogRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceDisplayFriendRequestDialogRequestParameters {
}

pub static PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendRequestDialogRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDisplayFriendRequestDialogRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDisplayFriendRequestDialogRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendRequestDialogRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayFriendRequestDialogRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceDisplayFriendsDialogRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceDisplayFriendsDialogRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceDisplayFriendsDialogRequestParametersTrait for PresenceDisplayFriendsDialogRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceDisplayFriendsDialogRequestParameters {
}

pub static PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendsDialogRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDisplayFriendsDialogRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDisplayFriendsDialogRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendsDialogRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayFriendsDialogRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PresenceDisplayUserProfileRequestParameters {
    pub _glacier_base: super::online::PresenceRequestParameters,
}

pub trait PresenceDisplayUserProfileRequestParametersTrait: super::online::PresenceRequestParametersTrait {
}

impl PresenceDisplayUserProfileRequestParametersTrait for PresenceDisplayUserProfileRequestParameters {
}

impl super::online::PresenceRequestParametersTrait for PresenceDisplayUserProfileRequestParameters {
}

pub static PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayUserProfileRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDisplayUserProfileRequestParameters as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDisplayUserProfileRequestParameters {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayUserProfileRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayUserProfileRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientOverlayService {
    pub _glacier_base: super::online::PresenceService,
}

pub trait ClientOverlayServiceTrait: super::online::PresenceServiceTrait {
}

impl ClientOverlayServiceTrait for ClientOverlayService {
}

impl super::online::PresenceServiceTrait for ClientOverlayService {
}

pub static CLIENTOVERLAYSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOverlayService",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientOverlayService as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOVERLAYSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientOverlayService {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTOVERLAYSERVICE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTOVERLAYSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOverlayService-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("ClientOverlayService"),
    array_type: None,
    alignment: 8,
};


