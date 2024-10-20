use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct UIVehicleHitUpdatedMessage {
}

pub trait UIVehicleHitUpdatedMessageTrait: TypeObject {
}

impl UIVehicleHitUpdatedMessageTrait for UIVehicleHitUpdatedMessage {
}

pub static UIVEHICLEHITUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIVehicleHitUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIVehicleHitUpdatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for UIVehicleHitUpdatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIVEHICLEHITUPDATEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UISquadStatusChangedMessage {
}

pub trait UISquadStatusChangedMessageTrait: TypeObject {
}

impl UISquadStatusChangedMessageTrait for UISquadStatusChangedMessage {
}

pub static UISQUADSTATUSCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISquadStatusChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UISquadStatusChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UISquadStatusChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UISQUADSTATUSCHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UISoldierHitUpdatedMessage {
}

pub trait UISoldierHitUpdatedMessageTrait: TypeObject {
}

impl UISoldierHitUpdatedMessageTrait for UISoldierHitUpdatedMessage {
}

pub static UISOLDIERHITUPDATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISoldierHitUpdatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UISoldierHitUpdatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 16,
};

impl TypeObject for UISoldierHitUpdatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UISOLDIERHITUPDATEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIPlayerVehicleHealthChangeMessage {
}

pub trait UIPlayerVehicleHealthChangeMessageTrait: TypeObject {
}

impl UIPlayerVehicleHealthChangeMessageTrait for UIPlayerVehicleHealthChangeMessage {
}

pub static UIPLAYERVEHICLEHEALTHCHANGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIPlayerVehicleHealthChangeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIPlayerVehicleHealthChangeMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIPlayerVehicleHealthChangeMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIPLAYERVEHICLEHEALTHCHANGEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIInputStatusChangedMessage {
}

pub trait UIInputStatusChangedMessageTrait: TypeObject {
}

impl UIInputStatusChangedMessageTrait for UIInputStatusChangedMessage {
}

pub static UIINPUTSTATUSCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputStatusChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIInputStatusChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIInputStatusChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTSTATUSCHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIHudUpdateCrosshairMessage {
}

pub trait UIHudUpdateCrosshairMessageTrait: TypeObject {
}

impl UIHudUpdateCrosshairMessageTrait for UIHudUpdateCrosshairMessage {
}

pub static UIHUDUPDATECROSSHAIRMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHudUpdateCrosshairMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIHudUpdateCrosshairMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHudUpdateCrosshairMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIHUDUPDATECROSSHAIRMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIHudToggleMapZoomMessage {
}

pub trait UIHudToggleMapZoomMessageTrait: TypeObject {
}

impl UIHudToggleMapZoomMessageTrait for UIHudToggleMapZoomMessage {
}

pub static UIHUDTOGGLEMAPZOOMMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHudToggleMapZoomMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIHudToggleMapZoomMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHudToggleMapZoomMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIHUDTOGGLEMAPZOOMMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIHudDebugPauseMessage {
}

pub trait UIHudDebugPauseMessageTrait: TypeObject {
}

impl UIHudDebugPauseMessageTrait for UIHudDebugPauseMessage {
}

pub static UIHUDDEBUGPAUSEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHudDebugPauseMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIHudDebugPauseMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHudDebugPauseMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIHUDDEBUGPAUSEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIHasSuppressedEnemyMessage {
}

pub trait UIHasSuppressedEnemyMessageTrait: TypeObject {
}

impl UIHasSuppressedEnemyMessageTrait for UIHasSuppressedEnemyMessage {
}

pub static UIHASSUPPRESSEDENEMYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHasSuppressedEnemyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIHasSuppressedEnemyMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIHasSuppressedEnemyMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIHASSUPPRESSEDENEMYMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIDamageGivenToEnemyMessage {
}

pub trait UIDamageGivenToEnemyMessageTrait: TypeObject {
}

impl UIDamageGivenToEnemyMessageTrait for UIDamageGivenToEnemyMessage {
}

pub static UIDAMAGEGIVENTOENEMYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDamageGivenToEnemyMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDamageGivenToEnemyMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIDamageGivenToEnemyMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIDAMAGEGIVENTOENEMYMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIMessageEntityMessage {
}

pub trait UIMessageEntityMessageTrait: TypeObject {
}

impl UIMessageEntityMessageTrait for UIMessageEntityMessage {
}

pub static UIMESSAGEENTITYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMessageEntityMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMessageEntityMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIMessageEntityMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIMESSAGEENTITYMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIControllerDisconnectedMessage {
}

pub trait UIControllerDisconnectedMessageTrait: TypeObject {
}

impl UIControllerDisconnectedMessageTrait for UIControllerDisconnectedMessage {
}

pub static UICONTROLLERDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIControllerDisconnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIControllerDisconnectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIControllerDisconnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UICONTROLLERDISCONNECTEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIControllerConnectedMessage {
}

pub trait UIControllerConnectedMessageTrait: TypeObject {
}

impl UIControllerConnectedMessageTrait for UIControllerConnectedMessage {
}

pub static UICONTROLLERCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIControllerConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIControllerConnectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIControllerConnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UICONTROLLERCONNECTEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIUserDisconnectedMessage {
}

pub trait UIUserDisconnectedMessageTrait: TypeObject {
}

impl UIUserDisconnectedMessageTrait for UIUserDisconnectedMessage {
}

pub static UIUSERDISCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserDisconnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIUserDisconnectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserDisconnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIUSERDISCONNECTEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIUserConnectedMessage {
}

pub trait UIUserConnectedMessageTrait: TypeObject {
}

impl UIUserConnectedMessageTrait for UIUserConnectedMessage {
}

pub static UIUSERCONNECTEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserConnectedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIUserConnectedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserConnectedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIUSERCONNECTEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIUserNotificationMessage {
}

pub trait UIUserNotificationMessageTrait: TypeObject {
}

impl UIUserNotificationMessageTrait for UIUserNotificationMessage {
}

pub static UIUSERNOTIFICATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserNotificationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIUserNotificationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserNotificationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIUSERNOTIFICATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIUserSkippedLoginMessage {
}

pub trait UIUserSkippedLoginMessageTrait: TypeObject {
}

impl UIUserSkippedLoginMessageTrait for UIUserSkippedLoginMessage {
}

pub static UIUSERSKIPPEDLOGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIUserSkippedLoginMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIUserSkippedLoginMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIUserSkippedLoginMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIUSERSKIPPEDLOGINMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardSavePauseMessage {
}

pub trait MemoryCardSavePauseMessageTrait: TypeObject {
}

impl MemoryCardSavePauseMessageTrait for MemoryCardSavePauseMessage {
}

pub static MEMORYCARDSAVEPAUSEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSavePauseMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardSavePauseMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for MemoryCardSavePauseMessage {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDSAVEPAUSEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardBootCheckMessageBase {
}

pub trait MemoryCardBootCheckMessageBaseTrait: TypeObject {
}

impl MemoryCardBootCheckMessageBaseTrait for MemoryCardBootCheckMessageBase {
}

pub static MEMORYCARDBOOTCHECKMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardBootCheckMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardBootCheckMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardBootCheckMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDBOOTCHECKMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardSaveCheckDoneMessageBase {
}

pub trait MemoryCardSaveCheckDoneMessageBaseTrait: TypeObject {
}

impl MemoryCardSaveCheckDoneMessageBaseTrait for MemoryCardSaveCheckDoneMessageBase {
}

pub static MEMORYCARDSAVECHECKDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSaveCheckDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardSaveCheckDoneMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardSaveCheckDoneMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDSAVECHECKDONEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardDeleteCorruptSavesDoneMessageBase {
}

pub trait MemoryCardDeleteCorruptSavesDoneMessageBaseTrait: TypeObject {
}

impl MemoryCardDeleteCorruptSavesDoneMessageBaseTrait for MemoryCardDeleteCorruptSavesDoneMessageBase {
}

pub static MEMORYCARDDELETECORRUPTSAVESDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteCorruptSavesDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardDeleteCorruptSavesDoneMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteCorruptSavesDoneMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDDELETECORRUPTSAVESDONEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardDeleteCorruptSavesMessageBase {
}

pub trait MemoryCardDeleteCorruptSavesMessageBaseTrait: TypeObject {
}

impl MemoryCardDeleteCorruptSavesMessageBaseTrait for MemoryCardDeleteCorruptSavesMessageBase {
}

pub static MEMORYCARDDELETECORRUPTSAVESMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteCorruptSavesMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardDeleteCorruptSavesMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteCorruptSavesMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDDELETECORRUPTSAVESMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardDeleteDoneMessageBase {
}

pub trait MemoryCardDeleteDoneMessageBaseTrait: TypeObject {
}

impl MemoryCardDeleteDoneMessageBaseTrait for MemoryCardDeleteDoneMessageBase {
}

pub static MEMORYCARDDELETEDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardDeleteDoneMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteDoneMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDDELETEDONEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardDeleteMessageBase {
}

pub trait MemoryCardDeleteMessageBaseTrait: TypeObject {
}

impl MemoryCardDeleteMessageBaseTrait for MemoryCardDeleteMessageBase {
}

pub static MEMORYCARDDELETEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDeleteMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardDeleteMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDeleteMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDDELETEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardFindEntriesDoneMessageBase {
}

pub trait MemoryCardFindEntriesDoneMessageBaseTrait: TypeObject {
}

impl MemoryCardFindEntriesDoneMessageBaseTrait for MemoryCardFindEntriesDoneMessageBase {
}

pub static MEMORYCARDFINDENTRIESDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardFindEntriesDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardFindEntriesDoneMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardFindEntriesDoneMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDFINDENTRIESDONEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardFindEntriesMessageBase {
}

pub trait MemoryCardFindEntriesMessageBaseTrait: TypeObject {
}

impl MemoryCardFindEntriesMessageBaseTrait for MemoryCardFindEntriesMessageBase {
}

pub static MEMORYCARDFINDENTRIESMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardFindEntriesMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardFindEntriesMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardFindEntriesMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDFINDENTRIESMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardSaveDoneMessageBase {
}

pub trait MemoryCardSaveDoneMessageBaseTrait: TypeObject {
}

impl MemoryCardSaveDoneMessageBaseTrait for MemoryCardSaveDoneMessageBase {
}

pub static MEMORYCARDSAVEDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSaveDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardSaveDoneMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardSaveDoneMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDSAVEDONEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardSaveMessageBase {
}

pub trait MemoryCardSaveMessageBaseTrait: TypeObject {
}

impl MemoryCardSaveMessageBaseTrait for MemoryCardSaveMessageBase {
}

pub static MEMORYCARDSAVEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardSaveMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardSaveMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardSaveMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDSAVEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardLoadDoneMessageBase {
}

pub trait MemoryCardLoadDoneMessageBaseTrait: TypeObject {
}

impl MemoryCardLoadDoneMessageBaseTrait for MemoryCardLoadDoneMessageBase {
}

pub static MEMORYCARDLOADDONEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardLoadDoneMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardLoadDoneMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardLoadDoneMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDLOADDONEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardLoadMessageBase {
}

pub trait MemoryCardLoadMessageBaseTrait: TypeObject {
}

impl MemoryCardLoadMessageBaseTrait for MemoryCardLoadMessageBase {
}

pub static MEMORYCARDLOADMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardLoadMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardLoadMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardLoadMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDLOADMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardDestroyedMessageBase {
}

pub trait MemoryCardDestroyedMessageBaseTrait: TypeObject {
}

impl MemoryCardDestroyedMessageBaseTrait for MemoryCardDestroyedMessageBase {
}

pub static MEMORYCARDDESTROYEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardDestroyedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardDestroyedMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardDestroyedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDDESTROYEDMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardInitializedMessageBase {
}

pub trait MemoryCardInitializedMessageBaseTrait: TypeObject {
}

impl MemoryCardInitializedMessageBaseTrait for MemoryCardInitializedMessageBase {
}

pub static MEMORYCARDINITIALIZEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardInitializedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardInitializedMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardInitializedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDINITIALIZEDMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardCreatedMessageBase {
}

pub trait MemoryCardCreatedMessageBaseTrait: TypeObject {
}

impl MemoryCardCreatedMessageBaseTrait for MemoryCardCreatedMessageBase {
}

pub static MEMORYCARDCREATEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardCreatedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardCreatedMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardCreatedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDCREATEDMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardPopupHideMessageBase {
}

pub trait MemoryCardPopupHideMessageBaseTrait: TypeObject {
}

impl MemoryCardPopupHideMessageBaseTrait for MemoryCardPopupHideMessageBase {
}

pub static MEMORYCARDPOPUPHIDEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardPopupHideMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardPopupHideMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardPopupHideMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDPOPUPHIDEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardPopupResponseMessageBase {
}

pub trait MemoryCardPopupResponseMessageBaseTrait: TypeObject {
}

impl MemoryCardPopupResponseMessageBaseTrait for MemoryCardPopupResponseMessageBase {
}

pub static MEMORYCARDPOPUPRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardPopupResponseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardPopupResponseMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardPopupResponseMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDPOPUPRESPONSEMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryCardPopupRequestMessageBase {
}

pub trait MemoryCardPopupRequestMessageBaseTrait: TypeObject {
}

impl MemoryCardPopupRequestMessageBaseTrait for MemoryCardPopupRequestMessageBase {
}

pub static MEMORYCARDPOPUPREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MemoryCardPopupRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MemoryCardPopupRequestMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for MemoryCardPopupRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        MEMORYCARDPOPUPREQUESTMESSAGEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIInputActionDataAsset {
    pub _glacier_base: super::core::Asset,
    pub default_repeat_delay_sec: f32,
    pub default_repeat_speed_sec: f32,
    pub default_one_axis_dead_zone: f32,
    pub default_two_axis_dead_zone: f32,
    pub input_action_maps: Vec<UIInputActionMapData>,
    pub analog_input_maps: Vec<UIAnalogInputMapData>,
}

pub trait UIInputActionDataAssetTrait: super::core::AssetTrait {
    fn default_repeat_delay_sec(&self) -> &f32;
    fn default_repeat_speed_sec(&self) -> &f32;
    fn default_one_axis_dead_zone(&self) -> &f32;
    fn default_two_axis_dead_zone(&self) -> &f32;
    fn input_action_maps(&self) -> &Vec<UIInputActionMapData>;
    fn analog_input_maps(&self) -> &Vec<UIAnalogInputMapData>;
}

impl UIInputActionDataAssetTrait for UIInputActionDataAsset {
    fn default_repeat_delay_sec(&self) -> &f32 {
        &self.default_repeat_delay_sec
    }
    fn default_repeat_speed_sec(&self) -> &f32 {
        &self.default_repeat_speed_sec
    }
    fn default_one_axis_dead_zone(&self) -> &f32 {
        &self.default_one_axis_dead_zone
    }
    fn default_two_axis_dead_zone(&self) -> &f32 {
        &self.default_two_axis_dead_zone
    }
    fn input_action_maps(&self) -> &Vec<UIInputActionMapData> {
        &self.input_action_maps
    }
    fn analog_input_maps(&self) -> &Vec<UIAnalogInputMapData> {
        &self.analog_input_maps
    }
}

impl super::core::AssetTrait for UIInputActionDataAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UIInputActionDataAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIINPUTACTIONDATAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionDataAsset",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIInputActionDataAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DefaultRepeatDelaySec",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIInputActionDataAsset, default_repeat_delay_sec),
            },
            FieldInfoData {
                name: "DefaultRepeatSpeedSec",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIInputActionDataAsset, default_repeat_speed_sec),
            },
            FieldInfoData {
                name: "DefaultOneAxisDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIInputActionDataAsset, default_one_axis_dead_zone),
            },
            FieldInfoData {
                name: "DefaultTwoAxisDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIInputActionDataAsset, default_two_axis_dead_zone),
            },
            FieldInfoData {
                name: "InputActionMaps",
                flags: MemberInfoFlags::new(144),
                field_type: "UIInputActionMapData-Array",
                rust_offset: offset_of!(UIInputActionDataAsset, input_action_maps),
            },
            FieldInfoData {
                name: "AnalogInputMaps",
                flags: MemberInfoFlags::new(144),
                field_type: "UIAnalogInputMapData-Array",
                rust_offset: offset_of!(UIInputActionDataAsset, analog_input_maps),
            },
        ],
    }),
    array_type: Some(UIINPUTACTIONDATAASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIInputActionDataAsset {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTACTIONDATAASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIINPUTACTIONDATAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionDataAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputActionDataAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIAnalogInputMapData {
    pub analog_input: UIAnalogInput,
    pub x_axis_concept_identifier: i32,
    pub y_axis_concept_identifier: i32,
    pub override_dead_zone: f32,
}

pub trait UIAnalogInputMapDataTrait: TypeObject {
    fn analog_input(&self) -> &UIAnalogInput;
    fn x_axis_concept_identifier(&self) -> &i32;
    fn y_axis_concept_identifier(&self) -> &i32;
    fn override_dead_zone(&self) -> &f32;
}

impl UIAnalogInputMapDataTrait for UIAnalogInputMapData {
    fn analog_input(&self) -> &UIAnalogInput {
        &self.analog_input
    }
    fn x_axis_concept_identifier(&self) -> &i32 {
        &self.x_axis_concept_identifier
    }
    fn y_axis_concept_identifier(&self) -> &i32 {
        &self.y_axis_concept_identifier
    }
    fn override_dead_zone(&self) -> &f32 {
        &self.override_dead_zone
    }
}

pub static UIANALOGINPUTMAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputMapData",
    flags: MemberInfoFlags::new(32841),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIAnalogInputMapData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AnalogInput",
                flags: MemberInfoFlags::new(0),
                field_type: "UIAnalogInput",
                rust_offset: offset_of!(UIAnalogInputMapData, analog_input),
            },
            FieldInfoData {
                name: "XAxisConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIAnalogInputMapData, x_axis_concept_identifier),
            },
            FieldInfoData {
                name: "YAxisConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIAnalogInputMapData, y_axis_concept_identifier),
            },
            FieldInfoData {
                name: "OverrideDeadZone",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIAnalogInputMapData, override_dead_zone),
            },
        ],
    }),
    array_type: Some(UIANALOGINPUTMAPDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIAnalogInputMapData {
    fn type_info(&self) -> &'static TypeInfo {
        UIANALOGINPUTMAPDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIANALOGINPUTMAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputMapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIAnalogInputMapData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIInputActionMapData {
    pub input_action: UIInputAction,
    pub concept_identifier: i32,
    pub allow_repeat: bool,
    pub override_repeat_delay_sec: f32,
    pub override_repeat_speed_sec: f32,
}

pub trait UIInputActionMapDataTrait: TypeObject {
    fn input_action(&self) -> &UIInputAction;
    fn concept_identifier(&self) -> &i32;
    fn allow_repeat(&self) -> &bool;
    fn override_repeat_delay_sec(&self) -> &f32;
    fn override_repeat_speed_sec(&self) -> &f32;
}

impl UIInputActionMapDataTrait for UIInputActionMapData {
    fn input_action(&self) -> &UIInputAction {
        &self.input_action
    }
    fn concept_identifier(&self) -> &i32 {
        &self.concept_identifier
    }
    fn allow_repeat(&self) -> &bool {
        &self.allow_repeat
    }
    fn override_repeat_delay_sec(&self) -> &f32 {
        &self.override_repeat_delay_sec
    }
    fn override_repeat_speed_sec(&self) -> &f32 {
        &self.override_repeat_speed_sec
    }
}

pub static UIINPUTACTIONMAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionMapData",
    flags: MemberInfoFlags::new(32841),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIInputActionMapData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InputAction",
                flags: MemberInfoFlags::new(0),
                field_type: "UIInputAction",
                rust_offset: offset_of!(UIInputActionMapData, input_action),
            },
            FieldInfoData {
                name: "ConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIInputActionMapData, concept_identifier),
            },
            FieldInfoData {
                name: "AllowRepeat",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIInputActionMapData, allow_repeat),
            },
            FieldInfoData {
                name: "OverrideRepeatDelaySec",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIInputActionMapData, override_repeat_delay_sec),
            },
            FieldInfoData {
                name: "OverrideRepeatSpeedSec",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIInputActionMapData, override_repeat_speed_sec),
            },
        ],
    }),
    array_type: Some(UIINPUTACTIONMAPDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIInputActionMapData {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTACTIONMAPDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIINPUTACTIONMAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionMapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputActionMapData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIInputSettings {
    pub _glacier_base: super::input_shared::BaseInputSettings,
    pub u_i_input_actions: Option<Arc<Mutex<dyn UIInputActionDataAssetTrait>>>,
}

pub trait UIInputSettingsTrait: super::input_shared::BaseInputSettingsTrait {
    fn u_i_input_actions(&self) -> &Option<Arc<Mutex<dyn UIInputActionDataAssetTrait>>>;
}

impl UIInputSettingsTrait for UIInputSettings {
    fn u_i_input_actions(&self) -> &Option<Arc<Mutex<dyn UIInputActionDataAssetTrait>>> {
        &self.u_i_input_actions
    }
}

impl super::input_shared::BaseInputSettingsTrait for UIInputSettings {
}

impl super::core::DataContainerTrait for UIInputSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIINPUTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputSettings",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::input_shared::BASEINPUTSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIInputSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UIInputActions",
                flags: MemberInfoFlags::new(0),
                field_type: "UIInputActionDataAsset",
                rust_offset: offset_of!(UIInputSettings, u_i_input_actions),
            },
        ],
    }),
    array_type: Some(UIINPUTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIInputSettings {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIINPUTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UILocaleFormatStrings {
    pub _glacier_base: super::core::DataContainer,
    pub number_format_s_i_d: String,
}

pub trait UILocaleFormatStringsTrait: super::core::DataContainerTrait {
    fn number_format_s_i_d(&self) -> &String;
}

impl UILocaleFormatStringsTrait for UILocaleFormatStrings {
    fn number_format_s_i_d(&self) -> &String {
        &self.number_format_s_i_d
    }
}

impl super::core::DataContainerTrait for UILocaleFormatStrings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UILOCALEFORMATSTRINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILocaleFormatStrings",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UILocaleFormatStrings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "NumberFormatSID",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UILocaleFormatStrings, number_format_s_i_d),
            },
        ],
    }),
    array_type: Some(UILOCALEFORMATSTRINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UILocaleFormatStrings {
    fn type_info(&self) -> &'static TypeInfo {
        UILOCALEFORMATSTRINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UILOCALEFORMATSTRINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UILocaleFormatStrings-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UILocaleFormatStrings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizedStringBase {
    pub _glacier_base: super::core::DataContainer,
}

pub trait LocalizedStringBaseTrait: super::core::DataContainerTrait {
}

impl LocalizedStringBaseTrait for LocalizedStringBase {
}

impl super::core::DataContainerTrait for LocalizedStringBase {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LOCALIZEDSTRINGBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringBase",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringBase {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALIZEDSTRINGBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("LocalizedStringBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizationAsset {
    pub _glacier_base: super::core::Asset,
    pub localized_texts: Vec<Option<Arc<Mutex<dyn UITextDatabaseTrait>>>>,
}

pub trait LocalizationAssetTrait: super::core::AssetTrait {
    fn localized_texts(&self) -> &Vec<Option<Arc<Mutex<dyn UITextDatabaseTrait>>>>;
}

impl LocalizationAssetTrait for LocalizationAsset {
    fn localized_texts(&self) -> &Vec<Option<Arc<Mutex<dyn UITextDatabaseTrait>>>> {
        &self.localized_texts
    }
}

impl super::core::AssetTrait for LocalizationAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for LocalizationAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LOCALIZATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizationAsset",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizationAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocalizedTexts",
                flags: MemberInfoFlags::new(144),
                field_type: "UITextDatabase-Array",
                rust_offset: offset_of!(LocalizationAsset, localized_texts),
            },
        ],
    }),
    array_type: Some(LOCALIZATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizationAsset {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZATIONASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALIZATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("LocalizationAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UITextDatabase {
    pub _glacier_base: super::core::Asset,
    pub language: super::core::LanguageFormat,
    pub binary_chunk: glacier_util::guid::Guid,
    pub binary_chunk_size: u32,
    pub histogram_chunk: glacier_util::guid::Guid,
    pub histogram_chunk_size: u32,
}

pub trait UITextDatabaseTrait: super::core::AssetTrait {
    fn language(&self) -> &super::core::LanguageFormat;
    fn binary_chunk(&self) -> &glacier_util::guid::Guid;
    fn binary_chunk_size(&self) -> &u32;
    fn histogram_chunk(&self) -> &glacier_util::guid::Guid;
    fn histogram_chunk_size(&self) -> &u32;
}

impl UITextDatabaseTrait for UITextDatabase {
    fn language(&self) -> &super::core::LanguageFormat {
        &self.language
    }
    fn binary_chunk(&self) -> &glacier_util::guid::Guid {
        &self.binary_chunk
    }
    fn binary_chunk_size(&self) -> &u32 {
        &self.binary_chunk_size
    }
    fn histogram_chunk(&self) -> &glacier_util::guid::Guid {
        &self.histogram_chunk
    }
    fn histogram_chunk_size(&self) -> &u32 {
        &self.histogram_chunk_size
    }
}

impl super::core::AssetTrait for UITextDatabase {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for UITextDatabase {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UITEXTDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextDatabase",
    flags: MemberInfoFlags::new(101),
    module: "UI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITextDatabase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(UITextDatabase, language),
            },
            FieldInfoData {
                name: "BinaryChunk",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(UITextDatabase, binary_chunk),
            },
            FieldInfoData {
                name: "BinaryChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UITextDatabase, binary_chunk_size),
            },
            FieldInfoData {
                name: "HistogramChunk",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(UITextDatabase, histogram_chunk),
            },
            FieldInfoData {
                name: "HistogramChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UITextDatabase, histogram_chunk_size),
            },
        ],
    }),
    array_type: Some(UITEXTDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UITextDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        UITEXTDATABASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITEXTDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITextDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UITextDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIConsoleKeyboardStatus {
    #[default]
    UIConsoleKeyboardStatus_Success = 0,
    UIConsoleKeyboardStatus_Failed = 1,
    UIConsoleKeyboardStatus_Cancelled = 2,
    UIConsoleKeyboardStatus_Active = 3,
    UIConsoleKeyboardStatus_Inactive = 4,
    UIConsoleKeyboardStatus_Count = 5,
}

pub static UICONSOLEKEYBOARDSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIConsoleKeyboardStatus",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UICONSOLEKEYBOARDSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIConsoleKeyboardStatus {
    fn type_info(&self) -> &'static TypeInfo {
        UICONSOLEKEYBOARDSTATUS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UICONSOLEKEYBOARDSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIConsoleKeyboardStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIConsoleKeyboardStatus"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIAnalogInput {
    #[default]
    UIAnalogInput_Size = 0,
    UIAnalogInput_None = 1,
}

pub static UIANALOGINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInput",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIANALOGINPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIAnalogInput {
    fn type_info(&self) -> &'static TypeInfo {
        UIANALOGINPUT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIANALOGINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIAnalogInput"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIAnalogInputEventType {
    #[default]
    UIAnalogInputEventType_Moved = 1,
    UIAnalogInputEventType_Released = 2,
}

pub static UIANALOGINPUTEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIANALOGINPUTEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIAnalogInputEventType {
    fn type_info(&self) -> &'static TypeInfo {
        UIANALOGINPUTEVENTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIANALOGINPUTEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIAnalogInputEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIAnalogInputEventType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static UIINPUTACTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputAction",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIINPUTACTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIInputAction {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTACTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIINPUTACTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputAction-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputAction"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UITouchEventType {
    #[default]
    UITouchEventType_Start = 1,
    UITouchEventType_Move = 2,
    UITouchEventType_End = 3,
}

pub static UITOUCHEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITouchEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UITOUCHEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UITouchEventType {
    fn type_info(&self) -> &'static TypeInfo {
        UITOUCHEVENTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UITOUCHEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITouchEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UITouchEventType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIInputActionEventType {
    #[default]
    UIInputActionEventType_Pressed = 1,
    UIInputActionEventType_Released = 2,
    UIInputActionEventType_Repeat = 3,
}

pub static UIINPUTACTIONEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIINPUTACTIONEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIInputActionEventType {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTACTIONEVENTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIINPUTACTIONEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputActionEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIInputActionEventType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIKeyboardEventType {
    #[default]
    UIKeyboardEventType_KeyDown = 0,
    UIKeyboardEventType_KeyUp = 1,
    UIKeyboardEventType_Char = 2,
}

pub static UIKEYBOARDEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIKeyboardEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIKEYBOARDEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIKeyboardEventType {
    fn type_info(&self) -> &'static TypeInfo {
        UIKEYBOARDEVENTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIKEYBOARDEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIKeyboardEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIKeyboardEventType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIMouseEventType {
    #[default]
    UIMouseEventType_MouseMove = 0,
    UIMouseEventType_MouseWheel = 1,
    UIMouseEventType_ButtonDown = 2,
    UIMouseEventType_ButtonUp = 3,
}

pub static UIMOUSEEVENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseEventType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIMOUSEEVENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIMouseEventType {
    fn type_info(&self) -> &'static TypeInfo {
        UIMOUSEEVENTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIMOUSEEVENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseEventType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIMouseEventType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIMouseButton {
    #[default]
    UIMouseButton_Left = 0,
    UIMouseButton_Right = 1,
    UIMouseButton_Middle = 2,
    UIMouseButton_Button3 = 3,
    UIMouseButton_Button4 = 4,
    UIMouseButton_Count = 5,
}

pub static UIMOUSEBUTTON_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseButton",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UIMOUSEBUTTON_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIMouseButton {
    fn type_info(&self) -> &'static TypeInfo {
        UIMOUSEBUTTON_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UIMOUSEBUTTON_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseButton-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UIMouseButton"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UISystemType {
    #[default]
    UISystem_None = 0,
    UISystem_Standard = 1,
    UISystem_ThinClient = 2,
}

pub static UISYSTEMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystemType",
    flags: MemberInfoFlags::new(49429),
    module: "UI",
    data: TypeInfoData::Enum,
    array_type: Some(UISYSTEMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UISystemType {
    fn type_info(&self) -> &'static TypeInfo {
        UISYSTEMTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static UISYSTEMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystemType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UI",
    data: TypeInfoData::Array("UISystemType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UserGamerpicBufferReceivedMessage {
}

pub trait UserGamerpicBufferReceivedMessageTrait: TypeObject {
}

impl UserGamerpicBufferReceivedMessageTrait for UserGamerpicBufferReceivedMessage {
}

pub static USERGAMERPICBUFFERRECEIVEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UserGamerpicBufferReceivedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "UI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UserGamerpicBufferReceivedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UserGamerpicBufferReceivedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        USERGAMERPICBUFFERRECEIVEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

