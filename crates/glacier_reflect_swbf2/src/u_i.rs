use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_u_i_types(registry: &mut TypeRegistry) {
    registry.register_type(UIVEHICLEHITUPDATEDMESSAGE_TYPE_INFO);
    registry.register_type(UISQUADSTATUSCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(UISOLDIERHITUPDATEDMESSAGE_TYPE_INFO);
    registry.register_type(UIPLAYERVEHICLEHEALTHCHANGEMESSAGE_TYPE_INFO);
    registry.register_type(UIINPUTSTATUSCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(UIHUDUPDATECROSSHAIRMESSAGE_TYPE_INFO);
    registry.register_type(UIHUDTOGGLEMAPZOOMMESSAGE_TYPE_INFO);
    registry.register_type(UIHUDDEBUGPAUSEMESSAGE_TYPE_INFO);
    registry.register_type(UIHASSUPPRESSEDENEMYMESSAGE_TYPE_INFO);
    registry.register_type(UIDAMAGEGIVENTOENEMYMESSAGE_TYPE_INFO);
    registry.register_type(UIMESSAGEENTITYMESSAGE_TYPE_INFO);
    registry.register_type(UICONTROLLERDISCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(UICONTROLLERCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(UIUSERDISCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(UIUSERCONNECTEDMESSAGE_TYPE_INFO);
    registry.register_type(UIUSERNOTIFICATIONMESSAGE_TYPE_INFO);
    registry.register_type(UIUSERSKIPPEDLOGINMESSAGE_TYPE_INFO);
    registry.register_type(MEMORYCARDSAVEPAUSEMESSAGE_TYPE_INFO);
    registry.register_type(MEMORYCARDBOOTCHECKMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDSAVECHECKDONEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDDELETECORRUPTSAVESDONEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDDELETECORRUPTSAVESMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDDELETEDONEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDDELETEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDFINDENTRIESDONEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDFINDENTRIESMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDSAVEDONEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDSAVEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDLOADDONEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDLOADMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDDESTROYEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDINITIALIZEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDCREATEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDPOPUPHIDEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDPOPUPRESPONSEMESSAGEBASE_TYPE_INFO);
    registry.register_type(MEMORYCARDPOPUPREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(UIINPUTACTIONDATAASSET_TYPE_INFO);
    registry.register_type(UIINPUTACTIONDATAASSET_ARRAY_TYPE_INFO);
    registry.register_type(UIANALOGINPUTMAPDATA_TYPE_INFO);
    registry.register_type(UIANALOGINPUTMAPDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIINPUTACTIONMAPDATA_TYPE_INFO);
    registry.register_type(UIINPUTACTIONMAPDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIINPUTSETTINGS_TYPE_INFO);
    registry.register_type(UIINPUTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(UILOCALEFORMATSTRINGS_TYPE_INFO);
    registry.register_type(UILOCALEFORMATSTRINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGBASE_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGBASE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZATIONASSET_TYPE_INFO);
    registry.register_type(LOCALIZATIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(UITEXTDATABASE_TYPE_INFO);
    registry.register_type(UITEXTDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(UICONSOLEKEYBOARDSTATUS_TYPE_INFO);
    registry.register_type(UICONSOLEKEYBOARDSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(UIANALOGINPUT_TYPE_INFO);
    registry.register_type(UIANALOGINPUT_ARRAY_TYPE_INFO);
    registry.register_type(UIANALOGINPUTEVENTTYPE_TYPE_INFO);
    registry.register_type(UIANALOGINPUTEVENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIINPUTACTION_TYPE_INFO);
    registry.register_type(UIINPUTACTION_ARRAY_TYPE_INFO);
    registry.register_type(UITOUCHEVENTTYPE_TYPE_INFO);
    registry.register_type(UITOUCHEVENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIINPUTACTIONEVENTTYPE_TYPE_INFO);
    registry.register_type(UIINPUTACTIONEVENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIKEYBOARDEVENTTYPE_TYPE_INFO);
    registry.register_type(UIKEYBOARDEVENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIMOUSEEVENTTYPE_TYPE_INFO);
    registry.register_type(UIMOUSEEVENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIMOUSEBUTTON_TYPE_INFO);
    registry.register_type(UIMOUSEBUTTON_ARRAY_TYPE_INFO);
    registry.register_type(UISYSTEMTYPE_TYPE_INFO);
    registry.register_type(UISYSTEMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(USERGAMERPICBUFFERRECEIVEDMESSAGE_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIVehicleHitUpdatedMessage {
}

pub const UIVEHICLEHITUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIVehicleHitUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for UIVehicleHitUpdatedMessage {
    fn type_info() -> &'static TypeInfo {
        UIVEHICLEHITUPDATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UISquadStatusChangedMessage {
}

pub const UISQUADSTATUSCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISquadStatusChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UISquadStatusChangedMessage {
    fn type_info() -> &'static TypeInfo {
        UISQUADSTATUSCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UISoldierHitUpdatedMessage {
}

pub const UISOLDIERHITUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISoldierHitUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for UISoldierHitUpdatedMessage {
    fn type_info() -> &'static TypeInfo {
        UISOLDIERHITUPDATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIPlayerVehicleHealthChangeMessage {
}

pub const UIPLAYERVEHICLEHEALTHCHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIPlayerVehicleHealthChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIPlayerVehicleHealthChangeMessage {
    fn type_info() -> &'static TypeInfo {
        UIPLAYERVEHICLEHEALTHCHANGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIInputStatusChangedMessage {
}

pub const UIINPUTSTATUSCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputStatusChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIInputStatusChangedMessage {
    fn type_info() -> &'static TypeInfo {
        UIINPUTSTATUSCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIHudUpdateCrosshairMessage {
}

pub const UIHUDUPDATECROSSHAIRMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHudUpdateCrosshairMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHudUpdateCrosshairMessage {
    fn type_info() -> &'static TypeInfo {
        UIHUDUPDATECROSSHAIRMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIHudToggleMapZoomMessage {
}

pub const UIHUDTOGGLEMAPZOOMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHudToggleMapZoomMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHudToggleMapZoomMessage {
    fn type_info() -> &'static TypeInfo {
        UIHUDTOGGLEMAPZOOMMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIHudDebugPauseMessage {
}

pub const UIHUDDEBUGPAUSEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHudDebugPauseMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHudDebugPauseMessage {
    fn type_info() -> &'static TypeInfo {
        UIHUDDEBUGPAUSEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIHasSuppressedEnemyMessage {
}

pub const UIHASSUPPRESSEDENEMYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHasSuppressedEnemyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHasSuppressedEnemyMessage {
    fn type_info() -> &'static TypeInfo {
        UIHASSUPPRESSEDENEMYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDamageGivenToEnemyMessage {
}

pub const UIDAMAGEGIVENTOENEMYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDamageGivenToEnemyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIDamageGivenToEnemyMessage {
    fn type_info() -> &'static TypeInfo {
        UIDAMAGEGIVENTOENEMYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIMessageEntityMessage {
}

pub const UIMESSAGEENTITYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMessageEntityMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIMessageEntityMessage {
    fn type_info() -> &'static TypeInfo {
        UIMESSAGEENTITYMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIControllerDisconnectedMessage {
}

pub const UICONTROLLERDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIControllerDisconnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIControllerDisconnectedMessage {
    fn type_info() -> &'static TypeInfo {
        UICONTROLLERDISCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIControllerConnectedMessage {
}

pub const UICONTROLLERCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIControllerConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIControllerConnectedMessage {
    fn type_info() -> &'static TypeInfo {
        UICONTROLLERCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIUserDisconnectedMessage {
}

pub const UIUSERDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserDisconnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserDisconnectedMessage {
    fn type_info() -> &'static TypeInfo {
        UIUSERDISCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIUserConnectedMessage {
}

pub const UIUSERCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserConnectedMessage {
    fn type_info() -> &'static TypeInfo {
        UIUSERCONNECTEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIUserNotificationMessage {
}

pub const UIUSERNOTIFICATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserNotificationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserNotificationMessage {
    fn type_info() -> &'static TypeInfo {
        UIUSERNOTIFICATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIUserSkippedLoginMessage {
}

pub const UIUSERSKIPPEDLOGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserSkippedLoginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserSkippedLoginMessage {
    fn type_info() -> &'static TypeInfo {
        UIUSERSKIPPEDLOGINMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardSavePauseMessage {
}

pub const MEMORYCARDSAVEPAUSEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSavePauseMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MemoryCardSavePauseMessage {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDSAVEPAUSEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardBootCheckMessageBase {
}

pub const MEMORYCARDBOOTCHECKMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardBootCheckMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardBootCheckMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDBOOTCHECKMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardSaveCheckDoneMessageBase {
}

pub const MEMORYCARDSAVECHECKDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSaveCheckDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardSaveCheckDoneMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDSAVECHECKDONEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardDeleteCorruptSavesDoneMessageBase {
}

pub const MEMORYCARDDELETECORRUPTSAVESDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteCorruptSavesDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteCorruptSavesDoneMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDDELETECORRUPTSAVESDONEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardDeleteCorruptSavesMessageBase {
}

pub const MEMORYCARDDELETECORRUPTSAVESMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteCorruptSavesMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteCorruptSavesMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDDELETECORRUPTSAVESMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardDeleteDoneMessageBase {
}

pub const MEMORYCARDDELETEDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteDoneMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDDELETEDONEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardDeleteMessageBase {
}

pub const MEMORYCARDDELETEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDDELETEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardFindEntriesDoneMessageBase {
}

pub const MEMORYCARDFINDENTRIESDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardFindEntriesDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardFindEntriesDoneMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDFINDENTRIESDONEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardFindEntriesMessageBase {
}

pub const MEMORYCARDFINDENTRIESMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardFindEntriesMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardFindEntriesMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDFINDENTRIESMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardSaveDoneMessageBase {
}

pub const MEMORYCARDSAVEDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSaveDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardSaveDoneMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDSAVEDONEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardSaveMessageBase {
}

pub const MEMORYCARDSAVEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSaveMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardSaveMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDSAVEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardLoadDoneMessageBase {
}

pub const MEMORYCARDLOADDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardLoadDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardLoadDoneMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDLOADDONEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardLoadMessageBase {
}

pub const MEMORYCARDLOADMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardLoadMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardLoadMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDLOADMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardDestroyedMessageBase {
}

pub const MEMORYCARDDESTROYEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDestroyedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDestroyedMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDDESTROYEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardInitializedMessageBase {
}

pub const MEMORYCARDINITIALIZEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardInitializedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardInitializedMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDINITIALIZEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardCreatedMessageBase {
}

pub const MEMORYCARDCREATEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardCreatedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardCreatedMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDCREATEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardPopupHideMessageBase {
}

pub const MEMORYCARDPOPUPHIDEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardPopupHideMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardPopupHideMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDPOPUPHIDEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardPopupResponseMessageBase {
}

pub const MEMORYCARDPOPUPRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardPopupResponseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardPopupResponseMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDPOPUPRESPONSEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryCardPopupRequestMessageBase {
}

pub const MEMORYCARDPOPUPREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardPopupRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardPopupRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        MEMORYCARDPOPUPREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UIInputActionDataAsset {
    pub default_repeat_delay_sec: f32,
    pub default_repeat_speed_sec: f32,
    pub default_one_axis_dead_zone: f32,
    pub default_two_axis_dead_zone: f32,
    pub input_action_maps: Vec<UIInputActionMapData>,
    pub analog_input_maps: Vec<UIAnalogInputMapData>,
}

pub const UIINPUTACTIONDATAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionDataAsset",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefaultRepeatDelaySec",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionDataAsset, default_repeat_delay_sec),
            },
            FieldInfoData {
                name: "DefaultRepeatSpeedSec",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionDataAsset, default_repeat_speed_sec),
            },
            FieldInfoData {
                name: "DefaultOneAxisDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionDataAsset, default_one_axis_dead_zone),
            },
            FieldInfoData {
                name: "DefaultTwoAxisDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionDataAsset, default_two_axis_dead_zone),
            },
            FieldInfoData {
                name: "InputActionMaps",
                flags: MemberInfoFlags::new(144),
                field_type: UIINPUTACTIONMAPDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionDataAsset, input_action_maps),
            },
            FieldInfoData {
                name: "AnalogInputMaps",
                flags: MemberInfoFlags::new(144),
                field_type: UIANALOGINPUTMAPDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionDataAsset, analog_input_maps),
            },
        ],
    }),
    array_type: Some(UIINPUTACTIONDATAASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIInputActionDataAsset {
    fn type_info() -> &'static TypeInfo {
        UIINPUTACTIONDATAASSET_TYPE_INFO
    }
}


pub const UIINPUTACTIONDATAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionDataAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputActionDataAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIAnalogInputMapData {
    pub analog_input: UIAnalogInput,
    pub x_axis_concept_identifier: i32,
    pub y_axis_concept_identifier: i32,
    pub override_dead_zone: f32,
}

pub const UIANALOGINPUTMAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputMapData",
    flags: MemberInfoFlags::new(32841),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AnalogInput",
                flags: MemberInfoFlags::new(0),
                field_type: UIANALOGINPUT_TYPE_INFO,
                rust_offset: offset_of!(UIAnalogInputMapData, analog_input),
            },
            FieldInfoData {
                name: "XAxisConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIAnalogInputMapData, x_axis_concept_identifier),
            },
            FieldInfoData {
                name: "YAxisConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIAnalogInputMapData, y_axis_concept_identifier),
            },
            FieldInfoData {
                name: "OverrideDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIAnalogInputMapData, override_dead_zone),
            },
        ],
    }),
    array_type: Some(UIANALOGINPUTMAPDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIAnalogInputMapData {
    fn type_info() -> &'static TypeInfo {
        UIANALOGINPUTMAPDATA_TYPE_INFO
    }
}


pub const UIANALOGINPUTMAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputMapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIAnalogInputMapData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIInputActionMapData {
    pub input_action: UIInputAction,
    pub concept_identifier: i32,
    pub allow_repeat: bool,
    pub override_repeat_delay_sec: f32,
    pub override_repeat_speed_sec: f32,
}

pub const UIINPUTACTIONMAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionMapData",
    flags: MemberInfoFlags::new(32841),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InputAction",
                flags: MemberInfoFlags::new(0),
                field_type: UIINPUTACTION_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionMapData, input_action),
            },
            FieldInfoData {
                name: "ConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionMapData, concept_identifier),
            },
            FieldInfoData {
                name: "AllowRepeat",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionMapData, allow_repeat),
            },
            FieldInfoData {
                name: "OverrideRepeatDelaySec",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionMapData, override_repeat_delay_sec),
            },
            FieldInfoData {
                name: "OverrideRepeatSpeedSec",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputActionMapData, override_repeat_speed_sec),
            },
        ],
    }),
    array_type: Some(UIINPUTACTIONMAPDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIInputActionMapData {
    fn type_info() -> &'static TypeInfo {
        UIINPUTACTIONMAPDATA_TYPE_INFO
    }
}


pub const UIINPUTACTIONMAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionMapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputActionMapData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIInputSettings {
    pub u_i_input_actions: UIInputActionDataAsset,
}

pub const UIINPUTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputSettings",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASEINPUTSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UIInputActions",
                flags: MemberInfoFlags::new(0),
                field_type: UIINPUTACTIONDATAASSET_TYPE_INFO,
                rust_offset: offset_of!(UIInputSettings, u_i_input_actions),
            },
        ],
    }),
    array_type: Some(UIINPUTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIInputSettings {
    fn type_info() -> &'static TypeInfo {
        UIINPUTSETTINGS_TYPE_INFO
    }
}


pub const UIINPUTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UILocaleFormatStrings {
    pub number_format_s_i_d: String,
}

pub const UILOCALEFORMATSTRINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILocaleFormatStrings",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "NumberFormatSID",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UILocaleFormatStrings, number_format_s_i_d),
            },
        ],
    }),
    array_type: Some(UILOCALEFORMATSTRINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UILocaleFormatStrings {
    fn type_info() -> &'static TypeInfo {
        UILOCALEFORMATSTRINGS_TYPE_INFO
    }
}


pub const UILOCALEFORMATSTRINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILocaleFormatStrings-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UILocaleFormatStrings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringBase {
}

pub const LOCALIZEDSTRINGBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringBase",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringBase {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGBASE_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("LocalizedStringBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizationAsset {
    pub localized_texts: Vec<UITextDatabase>,
}

pub const LOCALIZATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizationAsset",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocalizedTexts",
                flags: MemberInfoFlags::new(144),
                field_type: UITEXTDATABASE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LocalizationAsset, localized_texts),
            },
        ],
    }),
    array_type: Some(LOCALIZATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizationAsset {
    fn type_info() -> &'static TypeInfo {
        LOCALIZATIONASSET_TYPE_INFO
    }
}


pub const LOCALIZATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("LocalizationAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UITextDatabase {
    pub language: super::core::LanguageFormat,
    pub binary_chunk: super::core::Guid,
    pub binary_chunk_size: u32,
    pub histogram_chunk: super::core::Guid,
    pub histogram_chunk_size: u32,
}

pub const UITEXTDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextDatabase",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: LANGUAGEFORMAT_TYPE_INFO,
                rust_offset: offset_of!(UITextDatabase, language),
            },
            FieldInfoData {
                name: "BinaryChunk",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(UITextDatabase, binary_chunk),
            },
            FieldInfoData {
                name: "BinaryChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UITextDatabase, binary_chunk_size),
            },
            FieldInfoData {
                name: "HistogramChunk",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(UITextDatabase, histogram_chunk),
            },
            FieldInfoData {
                name: "HistogramChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UITextDatabase, histogram_chunk_size),
            },
        ],
    }),
    array_type: Some(UITEXTDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextDatabase {
    fn type_info() -> &'static TypeInfo {
        UITEXTDATABASE_TYPE_INFO
    }
}


pub const UITEXTDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UITextDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIConsoleKeyboardStatus {
    #[default]
    UIConsoleKeyboardStatus_Success = 0,
    UIConsoleKeyboardStatus_Failed = 1,
    UIConsoleKeyboardStatus_Cancelled = 2,
    UIConsoleKeyboardStatus_Active = 3,
    UIConsoleKeyboardStatus_Inactive = 4,
    UIConsoleKeyboardStatus_Count = 5,
}

pub const UICONSOLEKEYBOARDSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIConsoleKeyboardStatus",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UICONSOLEKEYBOARDSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIConsoleKeyboardStatus {
    fn type_info() -> &'static TypeInfo {
        UICONSOLEKEYBOARDSTATUS_TYPE_INFO
    }
}


pub const UICONSOLEKEYBOARDSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIConsoleKeyboardStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIConsoleKeyboardStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIAnalogInput {
    #[default]
    UIAnalogInput_Size = 0,
    UIAnalogInput_None = 1,
}

pub const UIANALOGINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInput",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIANALOGINPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIAnalogInput {
    fn type_info() -> &'static TypeInfo {
        UIANALOGINPUT_TYPE_INFO
    }
}


pub const UIANALOGINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIAnalogInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIAnalogInputEventType {
    #[default]
    UIAnalogInputEventType_Moved = 1,
    UIAnalogInputEventType_Released = 2,
}

pub const UIANALOGINPUTEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIANALOGINPUTEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIAnalogInputEventType {
    fn type_info() -> &'static TypeInfo {
        UIANALOGINPUTEVENTTYPE_TYPE_INFO
    }
}


pub const UIANALOGINPUTEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIAnalogInputEventType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIInputAction {
    #[default]
    UIInputAction_NavigateUp = 0,
    UIInputAction_NavigateDown = 1,
    UIInputAction_NavigateLeft = 2,
    UIInputAction_NavigateRight = 3,
    UIInputAction_TabLeft = 4,
    UIInputAction_TabRight = 5,
    UIInputAction_Activate = 6,
    UIInputAction_Deactivate = 7,
    UIInputAction_Menu = 8,
    UIInputAction_Cancel = 9,
    UIInputAction_OK = 10,
    UIInputAction_Back = 11,
    UIInputAction_Tab = 12,
    UIInputAction_Edit = 13,
    UIInputAction_View = 14,
    UIInputAction_LThumb = 15,
    UIInputAction_RThumb = 16,
    UIInputAction_MapZoom = 17,
    UIInputAction_MapSize = 18,
    UIInputAction_SayAllChat = 19,
    UIInputAction_TeamChat = 20,
    UIInputAction_SquadChat = 21,
    UIInputAction_CommoRose = 22,
    UIInputAction_ToggleChat = 23,
    UIInputAction_ToggleMinimapType = 24,
    UIInputAction_DigitalUp = 25,
    UIInputAction_DigitalDown = 26,
    UIInputAction_DigitalLeft = 27,
    UIInputAction_DigitalRight = 28,
    UIInputAction_NavigateRUp = 29,
    UIInputAction_NavigateRDown = 30,
    UIInputAction_NavigateRLeft = 31,
    UIInputAction_NavigateRRight = 32,
    UIInputAction_MenuTriggerLeft = 33,
    UIInputAction_MenuTriggerRight = 34,
    UIInputAction_TacticalMenu = 35,
    UIInputAction_ConversationSelect = 36,
    UIInputAction_ConversationSkip = 37,
    UIInputAction_ConversationChangeSelection = 38,
    UIInputAction_BattledashToggle = 39,
    UIInputAction_VoipPushToTalk = 40,
    UIInputAction_MultipleSelect = 41,
    UIInputAction_SpectatorViewPrev = 42,
    UIInputAction_SpectatorViewNext = 43,
    UIInputAction_SpectatorTargetPrev = 44,
    UIInputAction_SpectatorTargetNext = 45,
    UIInputAction_SpectatorViewTableTop = 46,
    UIInputAction_SpectatorViewFirstPerson = 47,
    UIInputAction_SpectatorViewThirdPerson = 48,
    UIInputAction_SpectatorViewFreeCam = 49,
    UIInputAction_SpectatorViewPlayer1 = 50,
    UIInputAction_SpectatorViewPlayer2 = 51,
    UIInputAction_SpectatorViewPlayer3 = 52,
    UIInputAction_SpectatorViewPlayer4 = 53,
    UIInputAction_SpectatorViewPlayer5 = 54,
    UIInputAction_SpectatorViewPlayer6 = 55,
    UIInputAction_SpectatorViewPlayer7 = 56,
    UIInputAction_SpectatorViewPlayer8 = 57,
    UIInputAction_SpectatorViewPlayer9 = 58,
    UIInputAction_SpectatorViewPlayer10 = 59,
    UIInputAction_SpectatorViewPlayer11 = 60,
    UIInputAction_SpectatorViewPlayer12 = 61,
    UIInputAction_SpectatorViewPlayer13 = 62,
    UIInputAction_SpectatorViewPlayer14 = 63,
    UIInputAction_SpectatorViewPlayer15 = 64,
    UIInputAction_SpectatorViewPlayer16 = 65,
    UIInputAction_SpectatorViewOptions = 66,
    UIInputAction_SpectatorHudVisibility = 67,
    UIInputAction_SpectatorTargetPrevInSquad = 68,
    UIInputAction_SpectatorTargetNextInSquad = 69,
    UIInputAction_SpectatorTargetPrevOnTeam = 70,
    UIInputAction_SpectatorTargetNextOnTeam = 71,
    UIInputAction_SpectatorSquadLeft = 72,
    UIInputAction_SpectatorSquadRight = 73,
    UIInputAction_SpectatorSquadUp = 74,
    UIInputAction_SpectatorSquadDown = 75,
    UIInputAction_SpectatorSquadActivate = 76,
    UIInputAction_Start = 77,
    UIInputAction_Minimize = 78,
    UIInputAction_SelectAll = 79,
    UIInputAction_Size = 80,
    UIInputAction_None = 81,
}

pub const UIINPUTACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputAction",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIINPUTACTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIInputAction {
    fn type_info() -> &'static TypeInfo {
        UIINPUTACTION_TYPE_INFO
    }
}


pub const UIINPUTACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputAction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UITouchEventType {
    #[default]
    UITouchEventType_Start = 1,
    UITouchEventType_Move = 2,
    UITouchEventType_End = 3,
}

pub const UITOUCHEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITouchEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UITOUCHEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UITouchEventType {
    fn type_info() -> &'static TypeInfo {
        UITOUCHEVENTTYPE_TYPE_INFO
    }
}


pub const UITOUCHEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITouchEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UITouchEventType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIInputActionEventType {
    #[default]
    UIInputActionEventType_Pressed = 1,
    UIInputActionEventType_Released = 2,
    UIInputActionEventType_Repeat = 3,
}

pub const UIINPUTACTIONEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIINPUTACTIONEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIInputActionEventType {
    fn type_info() -> &'static TypeInfo {
        UIINPUTACTIONEVENTTYPE_TYPE_INFO
    }
}


pub const UIINPUTACTIONEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputActionEventType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIKeyboardEventType {
    #[default]
    UIKeyboardEventType_KeyDown = 0,
    UIKeyboardEventType_KeyUp = 1,
    UIKeyboardEventType_Char = 2,
}

pub const UIKEYBOARDEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIKeyboardEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIKEYBOARDEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIKeyboardEventType {
    fn type_info() -> &'static TypeInfo {
        UIKEYBOARDEVENTTYPE_TYPE_INFO
    }
}


pub const UIKEYBOARDEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIKeyboardEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIKeyboardEventType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIMouseEventType {
    #[default]
    UIMouseEventType_MouseMove = 0,
    UIMouseEventType_MouseWheel = 1,
    UIMouseEventType_ButtonDown = 2,
    UIMouseEventType_ButtonUp = 3,
}

pub const UIMOUSEEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIMOUSEEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIMouseEventType {
    fn type_info() -> &'static TypeInfo {
        UIMOUSEEVENTTYPE_TYPE_INFO
    }
}


pub const UIMOUSEEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIMouseEventType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIMouseButton {
    #[default]
    UIMouseButton_Left = 0,
    UIMouseButton_Right = 1,
    UIMouseButton_Middle = 2,
    UIMouseButton_Button3 = 3,
    UIMouseButton_Button4 = 4,
    UIMouseButton_Count = 5,
}

pub const UIMOUSEBUTTON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseButton",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIMOUSEBUTTON_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIMouseButton {
    fn type_info() -> &'static TypeInfo {
        UIMOUSEBUTTON_TYPE_INFO
    }
}


pub const UIMOUSEBUTTON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseButton-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIMouseButton-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UISystemType {
    #[default]
    UISystem_None = 0,
    UISystem_Standard = 1,
    UISystem_ThinClient = 2,
}

pub const UISYSTEMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystemType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UISYSTEMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UISystemType {
    fn type_info() -> &'static TypeInfo {
        UISYSTEMTYPE_TYPE_INFO
    }
}


pub const UISYSTEMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystemType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UISystemType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UserGamerpicBufferReceivedMessage {
}

pub const USERGAMERPICBUFFERRECEIVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserGamerpicBufferReceivedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UserGamerpicBufferReceivedMessage {
    fn type_info() -> &'static TypeInfo {
        USERGAMERPICBUFFERRECEIVEDMESSAGE_TYPE_INFO
    }
}

