use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceOverlayServiceData {
}

pub static PRESENCEOVERLAYSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayServiceData",
    name_hash: 630362187,
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online_shared::PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceOverlayServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOverlayServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceOverlayServiceData as Default>::default()),
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


pub static PRESENCEOVERLAYSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayServiceData-Array",
    name_hash: 2141114239,
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceOverlayServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceOverlayRequestMessageBase {
}

pub trait PresenceOverlayRequestMessageBaseTrait: TypeObject {
}

impl PresenceOverlayRequestMessageBaseTrait for PresenceOverlayRequestMessageBase {
}

pub static PRESENCEOVERLAYREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayRequestMessageBase",
    name_hash: 1269208649,
    flags: MemberInfoFlags::new(36937),
    module: "FirstPartyOverlay",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOverlayRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceOverlayRequestMessageBase as Default>::default()),
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
pub struct PresenceOverlayMessageBase {
}

pub trait PresenceOverlayMessageBaseTrait: TypeObject {
}

impl PresenceOverlayMessageBaseTrait for PresenceOverlayMessageBase {
}

pub static PRESENCEOVERLAYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayMessageBase",
    name_hash: 926932248,
    flags: MemberInfoFlags::new(36937),
    module: "FirstPartyOverlay",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOverlayMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceOverlayMessageBase as Default>::default()),
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
    name_hash: 2238251681,
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceShowInviteUIRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceShowInviteUIRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceShowInviteUIRequestParameters as Default>::default()),
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


pub static PRESENCESHOWINVITEUIREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceShowInviteUIRequestParameters-Array",
    name_hash: 3990236437,
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceShowInviteUIRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3020539078,
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceDisplayFriendRequestDialogRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDisplayFriendRequestDialogRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceDisplayFriendRequestDialogRequestParameters as Default>::default()),
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


pub static PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendRequestDialogRequestParameters-Array",
    name_hash: 1372922482,
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayFriendRequestDialogRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2801541444,
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceDisplayFriendsDialogRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDisplayFriendsDialogRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceDisplayFriendsDialogRequestParameters as Default>::default()),
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


pub static PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendsDialogRequestParameters-Array",
    name_hash: 50488816,
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayFriendsDialogRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2778512439,
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        super_class_offset: offset_of!(PresenceDisplayUserProfileRequestParameters, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceDisplayUserProfileRequestParameters as Default>::default())),
            create_boxed: || Box::new(<PresenceDisplayUserProfileRequestParameters as Default>::default()),
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


pub static PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayUserProfileRequestParameters-Array",
    name_hash: 106558851,
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayUserProfileRequestParameters"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2470751707,
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::online::PRESENCESERVICE_TYPE_INFO),
        super_class_offset: offset_of!(ClientOverlayService, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientOverlayService as Default>::default())),
            create_boxed: || Box::new(<ClientOverlayService as Default>::default()),
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


pub static CLIENTOVERLAYSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOverlayService-Array",
    name_hash: 2422461679,
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("ClientOverlayService"),
    array_type: None,
    alignment: 8,
};


