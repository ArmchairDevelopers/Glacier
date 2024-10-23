use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_zone_streamer_types(registry: &mut TypeRegistry) {
    registry.register_type(ZONESTREAMERZONEDESTROYMESSAGE_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONEINITMESSAGE_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONECHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(ZONESTREAMERSHUTDOWNMESSAGE_TYPE_INFO);
    registry.register_type(ZONESTREAMERANNOUNCEMESSAGE_TYPE_INFO);
    registry.register_type(ZONESTREAMERNOTIFICATIONENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERNOTIFICATIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VISTAZONEINFO_TYPE_INFO);
    registry.register_type(VISTAZONEINFO_ARRAY_TYPE_INFO);
    registry.register_type(VISTAZONEMESHINFO_TYPE_INFO);
    registry.register_type(VISTAZONEMESHINFO_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERVISTAENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERVISTAENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERSUBWORLDROD_TYPE_INFO);
    registry.register_type(ZONESTREAMERSUBWORLDROD_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERINFO_TYPE_INFO);
    registry.register_type(ZONESTREAMERINFO_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONEINFO_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONEINFO_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERRASTERNODEUSAGE_TYPE_INFO);
    registry.register_type(ZONESTREAMERRASTERNODEUSAGE_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERSETTINGS_TYPE_INFO);
    registry.register_type(ZONESTREAMERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERFOCUSENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERFOCUSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONEPROXYENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONEPROXYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERTRANSITIONENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERTRANSITIONENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERCONTROLENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERCONTROLENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO);
    registry.register_type(ZONESTREAMERLOGICENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONEPROXYENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERZONEPROXYENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERVISTAENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERVISTAENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERTRANSITIONENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERTRANSITIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERNOTIFICATIONENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERNOTIFICATIONENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERLOGICENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERLOGICENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERGRID_TYPE_INFO);
    registry.register_type(ZONESTREAMERGRID_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERENTITYBASE_TYPE_INFO);
    registry.register_type(ZONESTREAMERENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERCONTROLENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERCONTROLENTITY_ARRAY_TYPE_INFO);
    registry.register_type(REALMPROXY_TYPE_INFO);
    registry.register_type(REALMPROXY_ARRAY_TYPE_INFO);
    registry.register_type(ZONESTREAMERFOCUSENTITY_TYPE_INFO);
    registry.register_type(ZONESTREAMERFOCUSENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerZoneDestroyMessage {
}

pub trait ZoneStreamerZoneDestroyMessageTrait: TypeObject {
}

impl ZoneStreamerZoneDestroyMessageTrait for ZoneStreamerZoneDestroyMessage {
}

pub static ZONESTREAMERZONEDESTROYMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneDestroyMessage",
    name_hash: 3363607783,
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerZoneDestroyMessage as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerZoneDestroyMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneDestroyMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERZONEDESTROYMESSAGE_TYPE_INFO
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
pub struct ZoneStreamerZoneInitMessage {
}

pub trait ZoneStreamerZoneInitMessageTrait: TypeObject {
}

impl ZoneStreamerZoneInitMessageTrait for ZoneStreamerZoneInitMessage {
}

pub static ZONESTREAMERZONEINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneInitMessage",
    name_hash: 1006053279,
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerZoneInitMessage as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerZoneInitMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneInitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERZONEINITMESSAGE_TYPE_INFO
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
pub struct ZoneStreamerZoneChangedMessage {
}

pub trait ZoneStreamerZoneChangedMessageTrait: TypeObject {
}

impl ZoneStreamerZoneChangedMessageTrait for ZoneStreamerZoneChangedMessage {
}

pub static ZONESTREAMERZONECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneChangedMessage",
    name_hash: 2856100295,
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerZoneChangedMessage as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerZoneChangedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERZONECHANGEDMESSAGE_TYPE_INFO
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
pub struct ZoneStreamerShutdownMessage {
}

pub trait ZoneStreamerShutdownMessageTrait: TypeObject {
}

impl ZoneStreamerShutdownMessageTrait for ZoneStreamerShutdownMessage {
}

pub static ZONESTREAMERSHUTDOWNMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerShutdownMessage",
    name_hash: 3422073107,
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerShutdownMessage as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerShutdownMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerShutdownMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERSHUTDOWNMESSAGE_TYPE_INFO
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
pub struct ZoneStreamerAnnounceMessage {
}

pub trait ZoneStreamerAnnounceMessageTrait: TypeObject {
}

impl ZoneStreamerAnnounceMessageTrait for ZoneStreamerAnnounceMessage {
}

pub static ZONESTREAMERANNOUNCEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerAnnounceMessage",
    name_hash: 1040514792,
    flags: MemberInfoFlags::new(36937),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerAnnounceMessage as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerAnnounceMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ZoneStreamerAnnounceMessage {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERANNOUNCEMESSAGE_TYPE_INFO
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
pub struct ZoneStreamerNotificationEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub control_entity: glacier_util::guid::Guid,
    pub bundle_name: String,
}

pub trait ZoneStreamerNotificationEntityDataTrait: super::entity::EntityDataTrait {
    fn control_entity(&self) -> &glacier_util::guid::Guid;
    fn control_entity_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn bundle_name(&self) -> &String;
    fn bundle_name_mut(&mut self) -> &mut String;
}

impl ZoneStreamerNotificationEntityDataTrait for ZoneStreamerNotificationEntityData {
    fn control_entity(&self) -> &glacier_util::guid::Guid {
        &self.control_entity
    }
    fn control_entity_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.control_entity
    }
    fn bundle_name(&self) -> &String {
        &self.bundle_name
    }
    fn bundle_name_mut(&mut self) -> &mut String {
        &mut self.bundle_name
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerNotificationEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerNotificationEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerNotificationEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerNotificationEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerNotificationEntityData {
}

pub static ZONESTREAMERNOTIFICATIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntityData",
    name_hash: 2268839542,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerNotificationEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerNotificationEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerNotificationEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ControlEntity",
                name_hash: 1837063353,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(ZoneStreamerNotificationEntityData, control_entity),
            },
            FieldInfoData {
                name: "BundleName",
                name_hash: 461157046,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ZoneStreamerNotificationEntityData, bundle_name),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERNOTIFICATIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerNotificationEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERNOTIFICATIONENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERNOTIFICATIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntityData-Array",
    name_hash: 3088720706,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerNotificationEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VistaZoneInfo {
    pub neighbours: Vec<i16>,
    pub objects: Vec<BoxedTypeObject /* VistaZoneMeshInfo */>,
}

pub trait VistaZoneInfoTrait: TypeObject {
    fn neighbours(&self) -> &Vec<i16>;
    fn neighbours_mut(&mut self) -> &mut Vec<i16>;
    fn objects(&self) -> &Vec<BoxedTypeObject /* VistaZoneMeshInfo */>;
    fn objects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* VistaZoneMeshInfo */>;
}

impl VistaZoneInfoTrait for VistaZoneInfo {
    fn neighbours(&self) -> &Vec<i16> {
        &self.neighbours
    }
    fn neighbours_mut(&mut self) -> &mut Vec<i16> {
        &mut self.neighbours
    }
    fn objects(&self) -> &Vec<BoxedTypeObject /* VistaZoneMeshInfo */> {
        &self.objects
    }
    fn objects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* VistaZoneMeshInfo */> {
        &mut self.objects
    }
}

pub static VISTAZONEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneInfo",
    name_hash: 89911724,
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VistaZoneInfo as Default>::default())),
            create_boxed: || Box::new(<VistaZoneInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Neighbours",
                name_hash: 53250673,
                flags: MemberInfoFlags::new(144),
                field_type: "Int16-Array",
                rust_offset: offset_of!(VistaZoneInfo, neighbours),
            },
            FieldInfoData {
                name: "Objects",
                name_hash: 105488131,
                flags: MemberInfoFlags::new(144),
                field_type: "VistaZoneMeshInfo-Array",
                rust_offset: offset_of!(VistaZoneInfo, objects),
            },
        ],
    }),
    array_type: Some(VISTAZONEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VistaZoneInfo {
    fn type_info(&self) -> &'static TypeInfo {
        VISTAZONEINFO_TYPE_INFO
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


pub static VISTAZONEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneInfo-Array",
    name_hash: 1223172376,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("VistaZoneInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct VistaZoneMeshInfo {
    pub object: Option<LockedTypeObject /* super::entity::ObjectBlueprint */>,
    pub transform: super::core::LinearTransform,
}

pub trait VistaZoneMeshInfoTrait: TypeObject {
    fn object(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */>;
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl VistaZoneMeshInfoTrait for VistaZoneMeshInfo {
    fn object(&self) -> &Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &self.object
    }
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectBlueprint */> {
        &mut self.object
    }
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
}

pub static VISTAZONEMESHINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneMeshInfo",
    name_hash: 3672083167,
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VistaZoneMeshInfo as Default>::default())),
            create_boxed: || Box::new(<VistaZoneMeshInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Object",
                name_hash: 2866508144,
                flags: MemberInfoFlags::new(0),
                field_type: "ObjectBlueprint",
                rust_offset: offset_of!(VistaZoneMeshInfo, object),
            },
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(VistaZoneMeshInfo, transform),
            },
        ],
    }),
    array_type: Some(VISTAZONEMESHINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for VistaZoneMeshInfo {
    fn type_info(&self) -> &'static TypeInfo {
        VISTAZONEMESHINFO_TYPE_INFO
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


pub static VISTAZONEMESHINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VistaZoneMeshInfo-Array",
    name_hash: 3408928747,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("VistaZoneMeshInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerVistaEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub zone_infos: Vec<BoxedTypeObject /* VistaZoneInfo */>,
    pub control_entity: glacier_util::guid::Guid,
}

pub trait ZoneStreamerVistaEntityDataTrait: super::entity::EntityDataTrait {
    fn zone_infos(&self) -> &Vec<BoxedTypeObject /* VistaZoneInfo */>;
    fn zone_infos_mut(&mut self) -> &mut Vec<BoxedTypeObject /* VistaZoneInfo */>;
    fn control_entity(&self) -> &glacier_util::guid::Guid;
    fn control_entity_mut(&mut self) -> &mut glacier_util::guid::Guid;
}

impl ZoneStreamerVistaEntityDataTrait for ZoneStreamerVistaEntityData {
    fn zone_infos(&self) -> &Vec<BoxedTypeObject /* VistaZoneInfo */> {
        &self.zone_infos
    }
    fn zone_infos_mut(&mut self) -> &mut Vec<BoxedTypeObject /* VistaZoneInfo */> {
        &mut self.zone_infos
    }
    fn control_entity(&self) -> &glacier_util::guid::Guid {
        &self.control_entity
    }
    fn control_entity_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.control_entity
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerVistaEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerVistaEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerVistaEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerVistaEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerVistaEntityData {
}

pub static ZONESTREAMERVISTAENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntityData",
    name_hash: 3257529250,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerVistaEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerVistaEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerVistaEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ZoneInfos",
                name_hash: 779639654,
                flags: MemberInfoFlags::new(144),
                field_type: "VistaZoneInfo-Array",
                rust_offset: offset_of!(ZoneStreamerVistaEntityData, zone_infos),
            },
            FieldInfoData {
                name: "ControlEntity",
                name_hash: 1837063353,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(ZoneStreamerVistaEntityData, control_entity),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERVISTAENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerVistaEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERVISTAENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERVISTAENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntityData-Array",
    name_hash: 321458710,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerVistaEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerSubWorldRod {
    pub _glacier_base: super::entity::SubWorldReferenceObjectData,
}

pub trait ZoneStreamerSubWorldRodTrait: super::entity::SubWorldReferenceObjectDataTrait {
}

impl ZoneStreamerSubWorldRodTrait for ZoneStreamerSubWorldRod {
}

impl super::entity::SubWorldReferenceObjectDataTrait for ZoneStreamerSubWorldRod {
    fn bundle_name(&self) -> &String {
        self._glacier_base.bundle_name()
    }
    fn bundle_name_mut(&mut self) -> &mut String {
        self._glacier_base.bundle_name_mut()
    }
    fn preloaded_bundle_names(&self) -> &Vec<String> {
        self._glacier_base.preloaded_bundle_names()
    }
    fn preloaded_bundle_names_mut(&mut self) -> &mut Vec<String> {
        self._glacier_base.preloaded_bundle_names_mut()
    }
    fn bundle_heap(&self) -> &super::entity::BundleHeapInfo {
        self._glacier_base.bundle_heap()
    }
    fn bundle_heap_mut(&mut self) -> &mut super::entity::BundleHeapInfo {
        self._glacier_base.bundle_heap_mut()
    }
    fn inclusion_settings(&self) -> &Option<LockedTypeObject /* super::entity::SubWorldInclusionSettings */> {
        self._glacier_base.inclusion_settings()
    }
    fn inclusion_settings_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::SubWorldInclusionSettings */> {
        self._glacier_base.inclusion_settings_mut()
    }
    fn auto_load(&self) -> &bool {
        self._glacier_base.auto_load()
    }
    fn auto_load_mut(&mut self) -> &mut bool {
        self._glacier_base.auto_load_mut()
    }
    fn is_detached_sub_level(&self) -> &bool {
        self._glacier_base.is_detached_sub_level()
    }
    fn is_detached_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_detached_sub_level_mut()
    }
    fn is_win32_sub_level(&self) -> &bool {
        self._glacier_base.is_win32_sub_level()
    }
    fn is_win32_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_win32_sub_level_mut()
    }
    fn is_gen4a_sub_level(&self) -> &bool {
        self._glacier_base.is_gen4a_sub_level()
    }
    fn is_gen4a_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_gen4a_sub_level_mut()
    }
    fn is_gen4b_sub_level(&self) -> &bool {
        self._glacier_base.is_gen4b_sub_level()
    }
    fn is_gen4b_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_gen4b_sub_level_mut()
    }
    fn is_i_o_s_sub_level(&self) -> &bool {
        self._glacier_base.is_i_o_s_sub_level()
    }
    fn is_i_o_s_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_i_o_s_sub_level_mut()
    }
    fn is_android_sub_level(&self) -> &bool {
        self._glacier_base.is_android_sub_level()
    }
    fn is_android_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_android_sub_level_mut()
    }
    fn is_o_s_x_sub_level(&self) -> &bool {
        self._glacier_base.is_o_s_x_sub_level()
    }
    fn is_o_s_x_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_o_s_x_sub_level_mut()
    }
    fn is_linux_sub_level(&self) -> &bool {
        self._glacier_base.is_linux_sub_level()
    }
    fn is_linux_sub_level_mut(&mut self) -> &mut bool {
        self._glacier_base.is_linux_sub_level_mut()
    }
    fn on_level_load_fire_on_stream_in(&self) -> &bool {
        self._glacier_base.on_level_load_fire_on_stream_in()
    }
    fn on_level_load_fire_on_stream_in_mut(&mut self) -> &mut bool {
        self._glacier_base.on_level_load_fire_on_stream_in_mut()
    }
    fn use_peer_filtering(&self) -> &bool {
        self._glacier_base.use_peer_filtering()
    }
    fn use_peer_filtering_mut(&mut self) -> &mut bool {
        self._glacier_base.use_peer_filtering_mut()
    }
    fn parents(&self) -> &Vec<BoxedTypeObject /* super::entity::SharedBundleReference */> {
        self._glacier_base.parents()
    }
    fn parents_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::SharedBundleReference */> {
        self._glacier_base.parents_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for ZoneStreamerSubWorldRod {
    fn blueprint_transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.blueprint_transform()
    }
    fn blueprint_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.blueprint_transform_mut()
    }
    fn blueprint(&self) -> &Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint()
    }
    fn blueprint_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::Blueprint */> {
        self._glacier_base.blueprint_mut()
    }
    fn object_variation(&self) -> &Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation()
    }
    fn object_variation_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::ObjectVariation */> {
        self._glacier_base.object_variation_mut()
    }
    fn stream_realm(&self) -> &super::entity::StreamRealm {
        self._glacier_base.stream_realm()
    }
    fn stream_realm_mut(&mut self) -> &mut super::entity::StreamRealm {
        self._glacier_base.stream_realm_mut()
    }
    fn radiosity_type_override(&self) -> &super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override()
    }
    fn radiosity_type_override_mut(&mut self) -> &mut super::core::RadiosityTypeOverride {
        self._glacier_base.radiosity_type_override_mut()
    }
    fn lightmap_resolution_scale(&self) -> &u32 {
        self._glacier_base.lightmap_resolution_scale()
    }
    fn lightmap_resolution_scale_mut(&mut self) -> &mut u32 {
        self._glacier_base.lightmap_resolution_scale_mut()
    }
    fn lightmap_scale_with_size(&self) -> &bool {
        self._glacier_base.lightmap_scale_with_size()
    }
    fn lightmap_scale_with_size_mut(&mut self) -> &mut bool {
        self._glacier_base.lightmap_scale_with_size_mut()
    }
    fn rendering_overrides(&self) -> &super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides()
    }
    fn rendering_overrides_mut(&mut self) -> &mut super::core::RenderingOverrides {
        self._glacier_base.rendering_overrides_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
    fn create_indestructible_entity(&self) -> &bool {
        self._glacier_base.create_indestructible_entity()
    }
    fn create_indestructible_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.create_indestructible_entity_mut()
    }
}

impl super::entity::GameObjectDataTrait for ZoneStreamerSubWorldRod {
}

impl super::core::DataBusPeerTrait for ZoneStreamerSubWorldRod {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerSubWorldRod {
}

impl super::core::DataContainerTrait for ZoneStreamerSubWorldRod {
}

pub static ZONESTREAMERSUBWORLDROD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSubWorldRod",
    name_hash: 955163151,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SUBWORLDREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerSubWorldRod, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerSubWorldRod as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerSubWorldRod as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERSUBWORLDROD_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ZoneStreamerSubWorldRod {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERSUBWORLDROD_TYPE_INFO
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


pub static ZONESTREAMERSUBWORLDROD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSubWorldRod-Array",
    name_hash: 742465211,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerSubWorldRod"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub client_side_only: bool,
    pub enable_default_focus: bool,
    pub info: ZoneStreamerInfo,
}

pub trait ZoneStreamerEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn client_side_only(&self) -> &bool;
    fn client_side_only_mut(&mut self) -> &mut bool;
    fn enable_default_focus(&self) -> &bool;
    fn enable_default_focus_mut(&mut self) -> &mut bool;
    fn info(&self) -> &ZoneStreamerInfo;
    fn info_mut(&mut self) -> &mut ZoneStreamerInfo;
}

impl ZoneStreamerEntityDataTrait for ZoneStreamerEntityData {
    fn client_side_only(&self) -> &bool {
        &self.client_side_only
    }
    fn client_side_only_mut(&mut self) -> &mut bool {
        &mut self.client_side_only
    }
    fn enable_default_focus(&self) -> &bool {
        &self.enable_default_focus
    }
    fn enable_default_focus_mut(&mut self) -> &mut bool {
        &mut self.enable_default_focus
    }
    fn info(&self) -> &ZoneStreamerInfo {
        &self.info
    }
    fn info_mut(&mut self) -> &mut ZoneStreamerInfo {
        &mut self.info
    }
}

impl super::entity::SpatialEntityDataTrait for ZoneStreamerEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerEntityData {
}

pub static ZONESTREAMERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityData",
    name_hash: 2392254299,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClientSideOnly",
                name_hash: 3628043763,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerEntityData, client_side_only),
            },
            FieldInfoData {
                name: "EnableDefaultFocus",
                name_hash: 4014959939,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerEntityData, enable_default_focus),
            },
            FieldInfoData {
                name: "Info",
                name_hash: 2088908747,
                flags: MemberInfoFlags::new(0),
                field_type: "ZoneStreamerInfo",
                rust_offset: offset_of!(ZoneStreamerEntityData, info),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ZoneStreamerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityData-Array",
    name_hash: 1977298543,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerInfo {
    pub grid_resource: glacier_reflect::builtin::ResourceRef,
    pub sub_level_path: String,
    pub zone_infos: Vec<BoxedTypeObject /* ZoneStreamerZoneInfo */>,
    pub bundle_parents: Vec<i16>,
    pub bundle_names: Vec<String>,
}

pub trait ZoneStreamerInfoTrait: TypeObject {
    fn grid_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn grid_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
    fn sub_level_path(&self) -> &String;
    fn sub_level_path_mut(&mut self) -> &mut String;
    fn zone_infos(&self) -> &Vec<BoxedTypeObject /* ZoneStreamerZoneInfo */>;
    fn zone_infos_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ZoneStreamerZoneInfo */>;
    fn bundle_parents(&self) -> &Vec<i16>;
    fn bundle_parents_mut(&mut self) -> &mut Vec<i16>;
    fn bundle_names(&self) -> &Vec<String>;
    fn bundle_names_mut(&mut self) -> &mut Vec<String>;
}

impl ZoneStreamerInfoTrait for ZoneStreamerInfo {
    fn grid_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.grid_resource
    }
    fn grid_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.grid_resource
    }
    fn sub_level_path(&self) -> &String {
        &self.sub_level_path
    }
    fn sub_level_path_mut(&mut self) -> &mut String {
        &mut self.sub_level_path
    }
    fn zone_infos(&self) -> &Vec<BoxedTypeObject /* ZoneStreamerZoneInfo */> {
        &self.zone_infos
    }
    fn zone_infos_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ZoneStreamerZoneInfo */> {
        &mut self.zone_infos
    }
    fn bundle_parents(&self) -> &Vec<i16> {
        &self.bundle_parents
    }
    fn bundle_parents_mut(&mut self) -> &mut Vec<i16> {
        &mut self.bundle_parents
    }
    fn bundle_names(&self) -> &Vec<String> {
        &self.bundle_names
    }
    fn bundle_names_mut(&mut self) -> &mut Vec<String> {
        &mut self.bundle_names
    }
}

pub static ZONESTREAMERINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerInfo",
    name_hash: 442861694,
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerInfo as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "GridResource",
                name_hash: 1502537591,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(ZoneStreamerInfo, grid_resource),
            },
            FieldInfoData {
                name: "SubLevelPath",
                name_hash: 1494405850,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ZoneStreamerInfo, sub_level_path),
            },
            FieldInfoData {
                name: "ZoneInfos",
                name_hash: 779639654,
                flags: MemberInfoFlags::new(144),
                field_type: "ZoneStreamerZoneInfo-Array",
                rust_offset: offset_of!(ZoneStreamerInfo, zone_infos),
            },
            FieldInfoData {
                name: "BundleParents",
                name_hash: 1850034782,
                flags: MemberInfoFlags::new(144),
                field_type: "Int16-Array",
                rust_offset: offset_of!(ZoneStreamerInfo, bundle_parents),
            },
            FieldInfoData {
                name: "BundleNames",
                name_hash: 2333280517,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(ZoneStreamerInfo, bundle_names),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerInfo {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERINFO_TYPE_INFO
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


pub static ZONESTREAMERINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerInfo-Array",
    name_hash: 4235381066,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerZoneInfo {
    pub neighbours: Vec<i16>,
}

pub trait ZoneStreamerZoneInfoTrait: TypeObject {
    fn neighbours(&self) -> &Vec<i16>;
    fn neighbours_mut(&mut self) -> &mut Vec<i16>;
}

impl ZoneStreamerZoneInfoTrait for ZoneStreamerZoneInfo {
    fn neighbours(&self) -> &Vec<i16> {
        &self.neighbours
    }
    fn neighbours_mut(&mut self) -> &mut Vec<i16> {
        &mut self.neighbours
    }
}

pub static ZONESTREAMERZONEINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneInfo",
    name_hash: 4010846208,
    flags: MemberInfoFlags::new(73),
    module: "ZoneStreamer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerZoneInfo as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerZoneInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Neighbours",
                name_hash: 53250673,
                flags: MemberInfoFlags::new(144),
                field_type: "Int16-Array",
                rust_offset: offset_of!(ZoneStreamerZoneInfo, neighbours),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERZONEINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneInfo {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERZONEINFO_TYPE_INFO
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


pub static ZONESTREAMERZONEINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneInfo-Array",
    name_hash: 2542620212,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerZoneInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ZoneStreamerRasterNodeUsage {
    #[default]
    ZoneStreamerRasterNodeUsage_Default = 0,
    ZoneStreamerRasterNodeUsage_Disabled = 1,
    ZoneStreamerRasterNodeUsage_Persistent = 2,
    ZoneStreamerRasterNodeUsage_PersistentDedicatedServer = 3,
    ZoneStreamerRasterNodeUsage_Skipped = 4,
}

pub static ZONESTREAMERRASTERNODEUSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerRasterNodeUsage",
    name_hash: 3651274966,
    flags: MemberInfoFlags::new(49429),
    module: "ZoneStreamer",
    data: TypeInfoData::Enum,
    array_type: Some(ZONESTREAMERRASTERNODEUSAGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ZoneStreamerRasterNodeUsage {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERRASTERNODEUSAGE_TYPE_INFO
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


pub static ZONESTREAMERRASTERNODEUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerRasterNodeUsage-Array",
    name_hash: 2435001954,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerRasterNodeUsage"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerSettings {
    pub _glacier_base: super::core::DataContainer,
    pub test_zone_heights: bool,
    pub pin_visited_zones: bool,
    pub pause_all: bool,
    pub draw_stats: bool,
    pub draw3d_debug: bool,
    pub draw3d_name_scale: f32,
    pub draw2d_debug: bool,
    pub draw2d_scale: f32,
    pub draw2d_zones: bool,
    pub draw2d_rotate: bool,
    pub draw2d_zone_states: bool,
    pub draw2d_centroids: bool,
    pub draw2d_point_size: f32,
    pub draw2d_bg_alpha: f32,
    pub draw2d_names: bool,
    pub draw_terrain_tiles: bool,
    pub draw_terrain_tile_loaded_only: bool,
    pub draw_terrain_tile_to_draw: i32,
    pub selected_streamer: String,
}

pub trait ZoneStreamerSettingsTrait: super::core::DataContainerTrait {
    fn test_zone_heights(&self) -> &bool;
    fn test_zone_heights_mut(&mut self) -> &mut bool;
    fn pin_visited_zones(&self) -> &bool;
    fn pin_visited_zones_mut(&mut self) -> &mut bool;
    fn pause_all(&self) -> &bool;
    fn pause_all_mut(&mut self) -> &mut bool;
    fn draw_stats(&self) -> &bool;
    fn draw_stats_mut(&mut self) -> &mut bool;
    fn draw3d_debug(&self) -> &bool;
    fn draw3d_debug_mut(&mut self) -> &mut bool;
    fn draw3d_name_scale(&self) -> &f32;
    fn draw3d_name_scale_mut(&mut self) -> &mut f32;
    fn draw2d_debug(&self) -> &bool;
    fn draw2d_debug_mut(&mut self) -> &mut bool;
    fn draw2d_scale(&self) -> &f32;
    fn draw2d_scale_mut(&mut self) -> &mut f32;
    fn draw2d_zones(&self) -> &bool;
    fn draw2d_zones_mut(&mut self) -> &mut bool;
    fn draw2d_rotate(&self) -> &bool;
    fn draw2d_rotate_mut(&mut self) -> &mut bool;
    fn draw2d_zone_states(&self) -> &bool;
    fn draw2d_zone_states_mut(&mut self) -> &mut bool;
    fn draw2d_centroids(&self) -> &bool;
    fn draw2d_centroids_mut(&mut self) -> &mut bool;
    fn draw2d_point_size(&self) -> &f32;
    fn draw2d_point_size_mut(&mut self) -> &mut f32;
    fn draw2d_bg_alpha(&self) -> &f32;
    fn draw2d_bg_alpha_mut(&mut self) -> &mut f32;
    fn draw2d_names(&self) -> &bool;
    fn draw2d_names_mut(&mut self) -> &mut bool;
    fn draw_terrain_tiles(&self) -> &bool;
    fn draw_terrain_tiles_mut(&mut self) -> &mut bool;
    fn draw_terrain_tile_loaded_only(&self) -> &bool;
    fn draw_terrain_tile_loaded_only_mut(&mut self) -> &mut bool;
    fn draw_terrain_tile_to_draw(&self) -> &i32;
    fn draw_terrain_tile_to_draw_mut(&mut self) -> &mut i32;
    fn selected_streamer(&self) -> &String;
    fn selected_streamer_mut(&mut self) -> &mut String;
}

impl ZoneStreamerSettingsTrait for ZoneStreamerSettings {
    fn test_zone_heights(&self) -> &bool {
        &self.test_zone_heights
    }
    fn test_zone_heights_mut(&mut self) -> &mut bool {
        &mut self.test_zone_heights
    }
    fn pin_visited_zones(&self) -> &bool {
        &self.pin_visited_zones
    }
    fn pin_visited_zones_mut(&mut self) -> &mut bool {
        &mut self.pin_visited_zones
    }
    fn pause_all(&self) -> &bool {
        &self.pause_all
    }
    fn pause_all_mut(&mut self) -> &mut bool {
        &mut self.pause_all
    }
    fn draw_stats(&self) -> &bool {
        &self.draw_stats
    }
    fn draw_stats_mut(&mut self) -> &mut bool {
        &mut self.draw_stats
    }
    fn draw3d_debug(&self) -> &bool {
        &self.draw3d_debug
    }
    fn draw3d_debug_mut(&mut self) -> &mut bool {
        &mut self.draw3d_debug
    }
    fn draw3d_name_scale(&self) -> &f32 {
        &self.draw3d_name_scale
    }
    fn draw3d_name_scale_mut(&mut self) -> &mut f32 {
        &mut self.draw3d_name_scale
    }
    fn draw2d_debug(&self) -> &bool {
        &self.draw2d_debug
    }
    fn draw2d_debug_mut(&mut self) -> &mut bool {
        &mut self.draw2d_debug
    }
    fn draw2d_scale(&self) -> &f32 {
        &self.draw2d_scale
    }
    fn draw2d_scale_mut(&mut self) -> &mut f32 {
        &mut self.draw2d_scale
    }
    fn draw2d_zones(&self) -> &bool {
        &self.draw2d_zones
    }
    fn draw2d_zones_mut(&mut self) -> &mut bool {
        &mut self.draw2d_zones
    }
    fn draw2d_rotate(&self) -> &bool {
        &self.draw2d_rotate
    }
    fn draw2d_rotate_mut(&mut self) -> &mut bool {
        &mut self.draw2d_rotate
    }
    fn draw2d_zone_states(&self) -> &bool {
        &self.draw2d_zone_states
    }
    fn draw2d_zone_states_mut(&mut self) -> &mut bool {
        &mut self.draw2d_zone_states
    }
    fn draw2d_centroids(&self) -> &bool {
        &self.draw2d_centroids
    }
    fn draw2d_centroids_mut(&mut self) -> &mut bool {
        &mut self.draw2d_centroids
    }
    fn draw2d_point_size(&self) -> &f32 {
        &self.draw2d_point_size
    }
    fn draw2d_point_size_mut(&mut self) -> &mut f32 {
        &mut self.draw2d_point_size
    }
    fn draw2d_bg_alpha(&self) -> &f32 {
        &self.draw2d_bg_alpha
    }
    fn draw2d_bg_alpha_mut(&mut self) -> &mut f32 {
        &mut self.draw2d_bg_alpha
    }
    fn draw2d_names(&self) -> &bool {
        &self.draw2d_names
    }
    fn draw2d_names_mut(&mut self) -> &mut bool {
        &mut self.draw2d_names
    }
    fn draw_terrain_tiles(&self) -> &bool {
        &self.draw_terrain_tiles
    }
    fn draw_terrain_tiles_mut(&mut self) -> &mut bool {
        &mut self.draw_terrain_tiles
    }
    fn draw_terrain_tile_loaded_only(&self) -> &bool {
        &self.draw_terrain_tile_loaded_only
    }
    fn draw_terrain_tile_loaded_only_mut(&mut self) -> &mut bool {
        &mut self.draw_terrain_tile_loaded_only
    }
    fn draw_terrain_tile_to_draw(&self) -> &i32 {
        &self.draw_terrain_tile_to_draw
    }
    fn draw_terrain_tile_to_draw_mut(&mut self) -> &mut i32 {
        &mut self.draw_terrain_tile_to_draw
    }
    fn selected_streamer(&self) -> &String {
        &self.selected_streamer
    }
    fn selected_streamer_mut(&mut self) -> &mut String {
        &mut self.selected_streamer
    }
}

impl super::core::DataContainerTrait for ZoneStreamerSettings {
}

pub static ZONESTREAMERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSettings",
    name_hash: 2922374965,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerSettings as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TestZoneHeights",
                name_hash: 1773128001,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, test_zone_heights),
            },
            FieldInfoData {
                name: "PinVisitedZones",
                name_hash: 6954415,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, pin_visited_zones),
            },
            FieldInfoData {
                name: "PauseAll",
                name_hash: 3633190422,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, pause_all),
            },
            FieldInfoData {
                name: "DrawStats",
                name_hash: 2413142628,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw_stats),
            },
            FieldInfoData {
                name: "Draw3dDebug",
                name_hash: 2192425635,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw3d_debug),
            },
            FieldInfoData {
                name: "Draw3dNameScale",
                name_hash: 3601377133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ZoneStreamerSettings, draw3d_name_scale),
            },
            FieldInfoData {
                name: "Draw2dDebug",
                name_hash: 986193826,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_debug),
            },
            FieldInfoData {
                name: "Draw2dScale",
                name_hash: 956911307,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_scale),
            },
            FieldInfoData {
                name: "Draw2dZones",
                name_hash: 964932734,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_zones),
            },
            FieldInfoData {
                name: "Draw2dRotate",
                name_hash: 1484115370,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_rotate),
            },
            FieldInfoData {
                name: "Draw2dZoneStates",
                name_hash: 1688952553,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_zone_states),
            },
            FieldInfoData {
                name: "Draw2dCentroids",
                name_hash: 3854046828,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_centroids),
            },
            FieldInfoData {
                name: "Draw2dPointSize",
                name_hash: 843156954,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_point_size),
            },
            FieldInfoData {
                name: "Draw2dBgAlpha",
                name_hash: 636207266,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_bg_alpha),
            },
            FieldInfoData {
                name: "Draw2dNames",
                name_hash: 988841831,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw2d_names),
            },
            FieldInfoData {
                name: "DrawTerrainTiles",
                name_hash: 2365395061,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw_terrain_tiles),
            },
            FieldInfoData {
                name: "DrawTerrainTileLoadedOnly",
                name_hash: 1122817141,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerSettings, draw_terrain_tile_loaded_only),
            },
            FieldInfoData {
                name: "DrawTerrainTileToDraw",
                name_hash: 1285380029,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ZoneStreamerSettings, draw_terrain_tile_to_draw),
            },
            FieldInfoData {
                name: "SelectedStreamer",
                name_hash: 3247159111,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ZoneStreamerSettings, selected_streamer),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERSETTINGS_TYPE_INFO
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


pub static ZONESTREAMERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerSettings-Array",
    name_hash: 1596516225,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerFocusEntityData {
    pub _glacier_base: ZoneStreamerLogicEntityData,
    pub focus_point: super::core::LinearTransform,
    pub auto_enabled: bool,
}

pub trait ZoneStreamerFocusEntityDataTrait: ZoneStreamerLogicEntityDataTrait {
    fn focus_point(&self) -> &super::core::LinearTransform;
    fn focus_point_mut(&mut self) -> &mut super::core::LinearTransform;
    fn auto_enabled(&self) -> &bool;
    fn auto_enabled_mut(&mut self) -> &mut bool;
}

impl ZoneStreamerFocusEntityDataTrait for ZoneStreamerFocusEntityData {
    fn focus_point(&self) -> &super::core::LinearTransform {
        &self.focus_point
    }
    fn focus_point_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.focus_point
    }
    fn auto_enabled(&self) -> &bool {
        &self.auto_enabled
    }
    fn auto_enabled_mut(&mut self) -> &mut bool {
        &mut self.auto_enabled
    }
}

impl ZoneStreamerLogicEntityDataTrait for ZoneStreamerFocusEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerFocusEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerFocusEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerFocusEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerFocusEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerFocusEntityData {
}

pub static ZONESTREAMERFOCUSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntityData",
    name_hash: 2382181911,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerFocusEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerFocusEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerFocusEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FocusPoint",
                name_hash: 2983182053,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ZoneStreamerFocusEntityData, focus_point),
            },
            FieldInfoData {
                name: "AutoEnabled",
                name_hash: 3489342063,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerFocusEntityData, auto_enabled),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERFOCUSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ZoneStreamerFocusEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERFOCUSENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERFOCUSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntityData-Array",
    name_hash: 3422690979,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerFocusEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerZoneProxyEntityData {
    pub _glacier_base: ZoneStreamerLogicEntityData,
    pub zone_and_region_names: Vec<String>,
}

pub trait ZoneStreamerZoneProxyEntityDataTrait: ZoneStreamerLogicEntityDataTrait {
    fn zone_and_region_names(&self) -> &Vec<String>;
    fn zone_and_region_names_mut(&mut self) -> &mut Vec<String>;
}

impl ZoneStreamerZoneProxyEntityDataTrait for ZoneStreamerZoneProxyEntityData {
    fn zone_and_region_names(&self) -> &Vec<String> {
        &self.zone_and_region_names
    }
    fn zone_and_region_names_mut(&mut self) -> &mut Vec<String> {
        &mut self.zone_and_region_names
    }
}

impl ZoneStreamerLogicEntityDataTrait for ZoneStreamerZoneProxyEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerZoneProxyEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerZoneProxyEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerZoneProxyEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerZoneProxyEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerZoneProxyEntityData {
}

pub static ZONESTREAMERZONEPROXYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntityData",
    name_hash: 495050857,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerZoneProxyEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerZoneProxyEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerZoneProxyEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ZoneAndRegionNames",
                name_hash: 3820785404,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(ZoneStreamerZoneProxyEntityData, zone_and_region_names),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERZONEPROXYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerZoneProxyEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERZONEPROXYENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERZONEPROXYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntityData-Array",
    name_hash: 1172174685,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerZoneProxyEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerTransitionEntityData {
    pub _glacier_base: ZoneStreamerLogicEntityData,
    pub auto_begin: bool,
}

pub trait ZoneStreamerTransitionEntityDataTrait: ZoneStreamerLogicEntityDataTrait {
    fn auto_begin(&self) -> &bool;
    fn auto_begin_mut(&mut self) -> &mut bool;
}

impl ZoneStreamerTransitionEntityDataTrait for ZoneStreamerTransitionEntityData {
    fn auto_begin(&self) -> &bool {
        &self.auto_begin
    }
    fn auto_begin_mut(&mut self) -> &mut bool {
        &mut self.auto_begin
    }
}

impl ZoneStreamerLogicEntityDataTrait for ZoneStreamerTransitionEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerTransitionEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerTransitionEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerTransitionEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerTransitionEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerTransitionEntityData {
}

pub static ZONESTREAMERTRANSITIONENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntityData",
    name_hash: 3137346772,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerTransitionEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerTransitionEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerTransitionEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AutoBegin",
                name_hash: 775570125,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerTransitionEntityData, auto_begin),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERTRANSITIONENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerTransitionEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERTRANSITIONENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERTRANSITIONENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntityData-Array",
    name_hash: 1938278752,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerTransitionEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerControlEntityData {
    pub _glacier_base: ZoneStreamerLogicEntityData,
    pub start_paused: bool,
}

pub trait ZoneStreamerControlEntityDataTrait: ZoneStreamerLogicEntityDataTrait {
    fn start_paused(&self) -> &bool;
    fn start_paused_mut(&mut self) -> &mut bool;
}

impl ZoneStreamerControlEntityDataTrait for ZoneStreamerControlEntityData {
    fn start_paused(&self) -> &bool {
        &self.start_paused
    }
    fn start_paused_mut(&mut self) -> &mut bool {
        &mut self.start_paused
    }
}

impl ZoneStreamerLogicEntityDataTrait for ZoneStreamerControlEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerControlEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerControlEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerControlEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerControlEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerControlEntityData {
}

pub static ZONESTREAMERCONTROLENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntityData",
    name_hash: 2593494748,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerControlEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerControlEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerControlEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StartPaused",
                name_hash: 735997331,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ZoneStreamerControlEntityData, start_paused),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERCONTROLENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ZoneStreamerControlEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERCONTROLENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERCONTROLENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntityData-Array",
    name_hash: 1917853544,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerControlEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerLogicEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
}

pub trait ZoneStreamerLogicEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl ZoneStreamerLogicEntityDataTrait for ZoneStreamerLogicEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl super::entity::EntityDataTrait for ZoneStreamerLogicEntityData {
}

impl super::entity::GameObjectDataTrait for ZoneStreamerLogicEntityData {
}

impl super::core::DataBusPeerTrait for ZoneStreamerLogicEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ZoneStreamerLogicEntityData {
}

impl super::core::DataContainerTrait for ZoneStreamerLogicEntityData {
}

pub static ZONESTREAMERLOGICENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntityData",
    name_hash: 1931611669,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerLogicEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerLogicEntityData as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerLogicEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ZoneStreamerLogicEntityData, realm),
            },
        ],
    }),
    array_type: Some(ZONESTREAMERLOGICENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerLogicEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERLOGICENTITYDATA_TYPE_INFO
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


pub static ZONESTREAMERLOGICENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntityData-Array",
    name_hash: 2234547617,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerLogicEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerZoneProxyEntity {
    pub _glacier_base: ZoneStreamerLogicEntity,
}

pub trait ZoneStreamerZoneProxyEntityTrait: ZoneStreamerLogicEntityTrait {
}

impl ZoneStreamerZoneProxyEntityTrait for ZoneStreamerZoneProxyEntity {
}

impl ZoneStreamerLogicEntityTrait for ZoneStreamerZoneProxyEntity {
}

impl super::entity::EntityTrait for ZoneStreamerZoneProxyEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerZoneProxyEntity {
}

pub static ZONESTREAMERZONEPROXYENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntity",
    name_hash: 1983147161,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerZoneProxyEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerZoneProxyEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerZoneProxyEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERZONEPROXYENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerZoneProxyEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERZONEPROXYENTITY_TYPE_INFO
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


pub static ZONESTREAMERZONEPROXYENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerZoneProxyEntity-Array",
    name_hash: 2389666349,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerZoneProxyEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerVistaEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ZoneStreamerVistaEntityTrait: super::entity::EntityTrait {
}

impl ZoneStreamerVistaEntityTrait for ZoneStreamerVistaEntity {
}

impl super::entity::EntityTrait for ZoneStreamerVistaEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerVistaEntity {
}

pub static ZONESTREAMERVISTAENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntity",
    name_hash: 864279442,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerVistaEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerVistaEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerVistaEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERVISTAENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerVistaEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERVISTAENTITY_TYPE_INFO
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


pub static ZONESTREAMERVISTAENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerVistaEntity-Array",
    name_hash: 3870827174,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerVistaEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerTransitionEntity {
    pub _glacier_base: ZoneStreamerLogicEntity,
}

pub trait ZoneStreamerTransitionEntityTrait: ZoneStreamerLogicEntityTrait {
}

impl ZoneStreamerTransitionEntityTrait for ZoneStreamerTransitionEntity {
}

impl ZoneStreamerLogicEntityTrait for ZoneStreamerTransitionEntity {
}

impl super::entity::EntityTrait for ZoneStreamerTransitionEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerTransitionEntity {
}

pub static ZONESTREAMERTRANSITIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntity",
    name_hash: 3357970916,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerTransitionEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerTransitionEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerTransitionEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERTRANSITIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerTransitionEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERTRANSITIONENTITY_TYPE_INFO
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


pub static ZONESTREAMERTRANSITIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerTransitionEntity-Array",
    name_hash: 3564541392,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerTransitionEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerNotificationEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ZoneStreamerNotificationEntityTrait: super::entity::EntityTrait {
}

impl ZoneStreamerNotificationEntityTrait for ZoneStreamerNotificationEntity {
}

impl super::entity::EntityTrait for ZoneStreamerNotificationEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerNotificationEntity {
}

pub static ZONESTREAMERNOTIFICATIONENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntity",
    name_hash: 3393849670,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerNotificationEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerNotificationEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerNotificationEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERNOTIFICATIONENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerNotificationEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERNOTIFICATIONENTITY_TYPE_INFO
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


pub static ZONESTREAMERNOTIFICATIONENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerNotificationEntity-Array",
    name_hash: 223790834,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerNotificationEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerLogicEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ZoneStreamerLogicEntityTrait: super::entity::EntityTrait {
}

impl ZoneStreamerLogicEntityTrait for ZoneStreamerLogicEntity {
}

impl super::entity::EntityTrait for ZoneStreamerLogicEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerLogicEntity {
}

pub static ZONESTREAMERLOGICENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntity",
    name_hash: 3841837797,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerLogicEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerLogicEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerLogicEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERLOGICENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerLogicEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERLOGICENTITY_TYPE_INFO
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


pub static ZONESTREAMERLOGICENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerLogicEntity-Array",
    name_hash: 2499810257,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerLogicEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerGrid {
}

pub trait ZoneStreamerGridTrait: TypeObject {
}

impl ZoneStreamerGridTrait for ZoneStreamerGrid {
}

pub static ZONESTREAMERGRID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerGrid",
    name_hash: 442912008,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerGrid as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerGrid as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERGRID_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerGrid {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERGRID_TYPE_INFO
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


pub static ZONESTREAMERGRID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerGrid-Array",
    name_hash: 3125415740,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerGrid"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait ZoneStreamerEntityBaseTrait: super::entity::EntityTrait {
}

impl ZoneStreamerEntityBaseTrait for ZoneStreamerEntityBase {
}

impl super::entity::EntityTrait for ZoneStreamerEntityBase {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerEntityBase {
}

pub static ZONESTREAMERENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityBase",
    name_hash: 2391899454,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerEntityBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerEntityBase as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerEntityBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERENTITYBASE_TYPE_INFO
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


pub static ZONESTREAMERENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntityBase-Array",
    name_hash: 3955461514,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerEntity {
    pub _glacier_base: ZoneStreamerEntityBase,
}

pub trait ZoneStreamerEntityTrait: ZoneStreamerEntityBaseTrait {
}

impl ZoneStreamerEntityTrait for ZoneStreamerEntity {
}

impl ZoneStreamerEntityBaseTrait for ZoneStreamerEntity {
}

impl super::entity::EntityTrait for ZoneStreamerEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerEntity {
}

pub static ZONESTREAMERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntity",
    name_hash: 1092611627,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERENTITY_TYPE_INFO
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


pub static ZONESTREAMERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerEntity-Array",
    name_hash: 698846623,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerControlEntity {
    pub _glacier_base: ZoneStreamerLogicEntity,
}

pub trait ZoneStreamerControlEntityTrait: ZoneStreamerLogicEntityTrait {
}

impl ZoneStreamerControlEntityTrait for ZoneStreamerControlEntity {
}

impl ZoneStreamerLogicEntityTrait for ZoneStreamerControlEntity {
}

impl super::entity::EntityTrait for ZoneStreamerControlEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerControlEntity {
}

pub static ZONESTREAMERCONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntity",
    name_hash: 3846195180,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerControlEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerControlEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerControlEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERCONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerControlEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERCONTROLENTITY_TYPE_INFO
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


pub static ZONESTREAMERCONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerControlEntity-Array",
    name_hash: 3389007832,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerControlEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RealmProxy {
    pub _glacier_base: ZoneStreamerEntityBase,
}

pub trait RealmProxyTrait: ZoneStreamerEntityBaseTrait {
}

impl RealmProxyTrait for RealmProxy {
}

impl ZoneStreamerEntityBaseTrait for RealmProxy {
}

impl super::entity::EntityTrait for RealmProxy {
}

impl super::entity::EntityBusPeerTrait for RealmProxy {
}

pub static REALMPROXY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RealmProxy",
    name_hash: 3096907102,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERENTITYBASE_TYPE_INFO),
        super_class_offset: offset_of!(RealmProxy, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RealmProxy as Default>::default())),
            create_boxed: || Box::new(<RealmProxy as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(REALMPROXY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RealmProxy {
    fn type_info(&self) -> &'static TypeInfo {
        REALMPROXY_TYPE_INFO
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


pub static REALMPROXY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RealmProxy-Array",
    name_hash: 3160993514,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("RealmProxy"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ZoneStreamerFocusEntity {
    pub _glacier_base: ZoneStreamerLogicEntity,
}

pub trait ZoneStreamerFocusEntityTrait: ZoneStreamerLogicEntityTrait {
}

impl ZoneStreamerFocusEntityTrait for ZoneStreamerFocusEntity {
}

impl ZoneStreamerLogicEntityTrait for ZoneStreamerFocusEntity {
}

impl super::entity::EntityTrait for ZoneStreamerFocusEntity {
}

impl super::entity::EntityBusPeerTrait for ZoneStreamerFocusEntity {
}

pub static ZONESTREAMERFOCUSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntity",
    name_hash: 1646199527,
    flags: MemberInfoFlags::new(101),
    module: "ZoneStreamer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ZONESTREAMERLOGICENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ZoneStreamerFocusEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ZoneStreamerFocusEntity as Default>::default())),
            create_boxed: || Box::new(<ZoneStreamerFocusEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ZONESTREAMERFOCUSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ZoneStreamerFocusEntity {
    fn type_info(&self) -> &'static TypeInfo {
        ZONESTREAMERFOCUSENTITY_TYPE_INFO
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


pub static ZONESTREAMERFOCUSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ZoneStreamerFocusEntity-Array",
    name_hash: 4258759891,
    flags: MemberInfoFlags::new(145),
    module: "ZoneStreamer",
    data: TypeInfoData::Array("ZoneStreamerFocusEntity"),
    array_type: None,
    alignment: 8,
};


