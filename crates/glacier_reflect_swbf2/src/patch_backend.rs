use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct LiveContentUpdateSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub auto_start: bool,
    pub timeout_initial: i32,
    pub timeout_recheck: i32,
}

pub trait LiveContentUpdateSettingsTrait: super::core::SystemSettingsTrait {
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
    fn timeout_initial(&self) -> &i32;
    fn timeout_initial_mut(&mut self) -> &mut i32;
    fn timeout_recheck(&self) -> &i32;
    fn timeout_recheck_mut(&mut self) -> &mut i32;
}

impl LiveContentUpdateSettingsTrait for LiveContentUpdateSettings {
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
    fn timeout_initial(&self) -> &i32 {
        &self.timeout_initial
    }
    fn timeout_initial_mut(&mut self) -> &mut i32 {
        &mut self.timeout_initial
    }
    fn timeout_recheck(&self) -> &i32 {
        &self.timeout_recheck
    }
    fn timeout_recheck_mut(&mut self) -> &mut i32 {
        &mut self.timeout_recheck
    }
}

impl super::core::SystemSettingsTrait for LiveContentUpdateSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for LiveContentUpdateSettings {
}

pub static LIVECONTENTUPDATESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveContentUpdateSettings",
    flags: MemberInfoFlags::new(101),
    module: "PatchBackend",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LiveContentUpdateSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LiveContentUpdateSettings, auto_start),
            },
            FieldInfoData {
                name: "TimeoutInitial",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LiveContentUpdateSettings, timeout_initial),
            },
            FieldInfoData {
                name: "TimeoutRecheck",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LiveContentUpdateSettings, timeout_recheck),
            },
        ],
    }),
    array_type: Some(LIVECONTENTUPDATESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LiveContentUpdateSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LIVECONTENTUPDATESETTINGS_TYPE_INFO
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


pub static LIVECONTENTUPDATESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LiveContentUpdateSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "PatchBackend",
    data: TypeInfoData::Array("LiveContentUpdateSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LCUServiceMessageProgressMessage {
}

pub trait LCUServiceMessageProgressMessageTrait: TypeObject {
}

impl LCUServiceMessageProgressMessageTrait for LCUServiceMessageProgressMessage {
}

pub static LCUSERVICEMESSAGEPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUServiceMessageProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "PatchBackend",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LCUServiceMessageProgressMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LCUServiceMessageProgressMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LCUSERVICEMESSAGEPROGRESSMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct LCUServiceMessageStateChangedMessage {
}

pub trait LCUServiceMessageStateChangedMessageTrait: TypeObject {
}

impl LCUServiceMessageStateChangedMessageTrait for LCUServiceMessageStateChangedMessage {
}

pub static LCUSERVICEMESSAGESTATECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUServiceMessageStateChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "PatchBackend",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LCUServiceMessageStateChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LCUServiceMessageStateChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LCUSERVICEMESSAGESTATECHANGEDMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct LCUEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub automatic: bool,
}

pub trait LCUEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn automatic(&self) -> &bool;
    fn automatic_mut(&mut self) -> &mut bool;
}

impl LCUEntityDataTrait for LCUEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn automatic(&self) -> &bool {
        &self.automatic
    }
    fn automatic_mut(&mut self) -> &mut bool {
        &mut self.automatic
    }
}

impl super::entity::EntityDataTrait for LCUEntityData {
}

impl super::entity::GameObjectDataTrait for LCUEntityData {
}

impl super::core::DataBusPeerTrait for LCUEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LCUEntityData {
}

impl super::core::DataContainerTrait for LCUEntityData {
}

pub static LCUENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PatchBackend",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LCUEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(LCUEntityData, realm),
            },
            FieldInfoData {
                name: "Automatic",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LCUEntityData, automatic),
            },
        ],
    }),
    array_type: Some(LCUENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LCUEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LCUENTITYDATA_TYPE_INFO
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


pub static LCUENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LCUEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PatchBackend",
    data: TypeInfoData::Array("LCUEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientLCUEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientLCUEntityTrait: super::entity::EntityTrait {
}

impl ClientLCUEntityTrait for ClientLCUEntity {
}

impl super::entity::EntityTrait for ClientLCUEntity {
}

impl super::entity::EntityBusPeerTrait for ClientLCUEntity {
}

pub static CLIENTLCUENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLCUEntity",
    flags: MemberInfoFlags::new(101),
    module: "PatchBackend",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientLCUEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTLCUENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientLCUEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTLCUENTITY_TYPE_INFO
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


pub static CLIENTLCUENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientLCUEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PatchBackend",
    data: TypeInfoData::Array("ClientLCUEntity"),
    array_type: None,
    alignment: 8,
};


