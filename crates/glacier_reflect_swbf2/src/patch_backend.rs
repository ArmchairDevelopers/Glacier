use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_patch_backend_types(registry: &mut TypeRegistry) {
    registry.register_type(LIVECONTENTUPDATESETTINGS_TYPE_INFO);
    registry.register_type(LIVECONTENTUPDATESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LCUSERVICEMESSAGEPROGRESSMESSAGE_TYPE_INFO);
    registry.register_type(LCUSERVICEMESSAGESTATECHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(LCUENTITYDATA_TYPE_INFO);
    registry.register_type(LCUENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTLCUENTITY_TYPE_INFO);
    registry.register_type(CLIENTLCUENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LiveContentUpdateSettings {
    pub auto_start: bool,
    pub timeout_initial: i32,
    pub timeout_recheck: i32,
}

pub const LIVECONTENTUPDATESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveContentUpdateSettings",
    flags: MemberInfoFlags::new(101),
    module: "PatchBackend",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LiveContentUpdateSettings, auto_start),
            },
            FieldInfoData {
                name: "TimeoutInitial",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LiveContentUpdateSettings, timeout_initial),
            },
            FieldInfoData {
                name: "TimeoutRecheck",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LiveContentUpdateSettings, timeout_recheck),
            },
        ],
    }),
    array_type: Some(LIVECONTENTUPDATESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LiveContentUpdateSettings {
    fn type_info() -> &'static TypeInfo {
        LIVECONTENTUPDATESETTINGS_TYPE_INFO
    }
}


pub const LIVECONTENTUPDATESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveContentUpdateSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "PatchBackend",
    data: TypeInfoData::Array("LiveContentUpdateSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LCUServiceMessageProgressMessage {
}

pub const LCUSERVICEMESSAGEPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUServiceMessageProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "PatchBackend",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LCUServiceMessageProgressMessage {
    fn type_info() -> &'static TypeInfo {
        LCUSERVICEMESSAGEPROGRESSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LCUServiceMessageStateChangedMessage {
}

pub const LCUSERVICEMESSAGESTATECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUServiceMessageStateChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "PatchBackend",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LCUServiceMessageStateChangedMessage {
    fn type_info() -> &'static TypeInfo {
        LCUSERVICEMESSAGESTATECHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LCUEntityData {
    pub realm: super::core::Realm,
    pub automatic: bool,
}

pub const LCUENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PatchBackend",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LCUEntityData, realm),
            },
            FieldInfoData {
                name: "Automatic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LCUEntityData, automatic),
            },
        ],
    }),
    array_type: Some(LCUENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LCUEntityData {
    fn type_info() -> &'static TypeInfo {
        LCUENTITYDATA_TYPE_INFO
    }
}


pub const LCUENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PatchBackend",
    data: TypeInfoData::Array("LCUEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientLCUEntity {
}

pub const CLIENTLCUENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLCUEntity",
    flags: MemberInfoFlags::new(101),
    module: "PatchBackend",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLCUENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLCUEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTLCUENTITY_TYPE_INFO
    }
}


pub const CLIENTLCUENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLCUEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PatchBackend",
    data: TypeInfoData::Array("ClientLCUEntity-Array"),
    array_type: None,
    alignment: 8,
};


