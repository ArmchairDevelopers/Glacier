use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceOverlayServiceData {
}

pub const PRESENCEOVERLAYSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayServiceData",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEOVERLAYSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceOverlayServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEOVERLAYSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEOVERLAYSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceOverlayServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceOverlayRequestMessageBase {
}

pub const PRESENCEOVERLAYREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOverlayRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEOVERLAYREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceOverlayMessageBase {
}

pub const PRESENCEOVERLAYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOverlayMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOverlayMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEOVERLAYMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceShowInviteUIRequestParameters {
}

pub const PRESENCESHOWINVITEUIREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceShowInviteUIRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESHOWINVITEUIREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceShowInviteUIRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCESHOWINVITEUIREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCESHOWINVITEUIREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceShowInviteUIRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceShowInviteUIRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceDisplayFriendRequestDialogRequestParameters {
}

pub const PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendRequestDialogRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDisplayFriendRequestDialogRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEDISPLAYFRIENDREQUESTDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendRequestDialogRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayFriendRequestDialogRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceDisplayFriendsDialogRequestParameters {
}

pub const PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendsDialogRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDisplayFriendsDialogRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEDISPLAYFRIENDSDIALOGREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayFriendsDialogRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayFriendsDialogRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceDisplayUserProfileRequestParameters {
}

pub const PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayUserProfileRequestParameters",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEREQUESTPARAMETERS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PresenceDisplayUserProfileRequestParameters {
    fn type_info() -> &'static TypeInfo {
        PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_TYPE_INFO
    }
}


pub const PRESENCEDISPLAYUSERPROFILEREQUESTPARAMETERS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceDisplayUserProfileRequestParameters-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("PresenceDisplayUserProfileRequestParameters-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientOverlayService {
}

pub const CLIENTOVERLAYSERVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOverlayService",
    flags: MemberInfoFlags::new(101),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTOVERLAYSERVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientOverlayService {
    fn type_info() -> &'static TypeInfo {
        CLIENTOVERLAYSERVICE_TYPE_INFO
    }
}


pub const CLIENTOVERLAYSERVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientOverlayService-Array",
    flags: MemberInfoFlags::new(145),
    module: "FirstPartyOverlay",
    data: TypeInfoData::Array("ClientOverlayService-Array"),
    array_type: None,
    alignment: 8,
};


