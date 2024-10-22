use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_a_i_tools_types(registry: &mut TypeRegistry) {
    registry.register_type(NAVIGATIONINTERFACEDATA_TYPE_INFO);
    registry.register_type(NAVIGATIONINTERFACEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCOENTITYDATA_TYPE_INFO);
    registry.register_type(LOCOENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRAWAYPOINTDATAPTR_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRAWAYPOINTDATAPTR_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRATELEPORT_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRATELEPORT_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRASPATIAL_TYPE_INFO);
    registry.register_type(AIWAYPOINTEXTRASPATIAL_ARRAY_TYPE_INFO);
    registry.register_type(AIWAYPOINTGUID_TYPE_INFO);
    registry.register_type(AIWAYPOINTGUID_ARRAY_TYPE_INFO);
    registry.register_type(SERVERNAVIGATIONINTERFACE_TYPE_INFO);
    registry.register_type(SERVERNAVIGATIONINTERFACE_ARRAY_TYPE_INFO);
    registry.register_type(SERVERAUTHNAVIGATIONINTERFACE_TYPE_INFO);
    registry.register_type(SERVERAUTHNAVIGATIONINTERFACE_ARRAY_TYPE_INFO);
    registry.register_type(LOCOENTITY_TYPE_INFO);
    registry.register_type(LOCOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTNAVIGATIONINTERFACE_TYPE_INFO);
    registry.register_type(CLIENTNAVIGATIONINTERFACE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct NavigationInterfaceData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait NavigationInterfaceDataTrait: super::entity::EntityDataTrait {
}

impl NavigationInterfaceDataTrait for NavigationInterfaceData {
}

impl super::entity::EntityDataTrait for NavigationInterfaceData {
}

impl super::entity::GameObjectDataTrait for NavigationInterfaceData {
}

impl super::core::DataBusPeerTrait for NavigationInterfaceData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for NavigationInterfaceData {
}

impl super::core::DataContainerTrait for NavigationInterfaceData {
}

pub static NAVIGATIONINTERFACEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavigationInterfaceData",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NavigationInterfaceData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(NAVIGATIONINTERFACEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NavigationInterfaceData {
    fn type_info(&self) -> &'static TypeInfo {
        NAVIGATIONINTERFACEDATA_TYPE_INFO
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


pub static NAVIGATIONINTERFACEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavigationInterfaceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("NavigationInterfaceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocoEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait LocoEntityDataTrait: super::entity::EntityDataTrait {
}

impl LocoEntityDataTrait for LocoEntityData {
}

impl super::entity::EntityDataTrait for LocoEntityData {
}

impl super::entity::GameObjectDataTrait for LocoEntityData {
}

impl super::core::DataBusPeerTrait for LocoEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LocoEntityData {
}

impl super::core::DataContainerTrait for LocoEntityData {
}

pub static LOCOENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocoEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCOENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocoEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCOENTITYDATA_TYPE_INFO
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


pub static LOCOENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("LocoEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIWaypointExtraWaypointDataPtr {
    pub waypoint_data_ptr: Option<Arc<Mutex<dyn super::pathfinding_shared::WaypointDataTrait>>>,
    pub sublevel_i_d: i32,
}

pub trait AIWaypointExtraWaypointDataPtrTrait: TypeObject {
    fn waypoint_data_ptr(&self) -> &Option<Arc<Mutex<dyn super::pathfinding_shared::WaypointDataTrait>>>;
    fn waypoint_data_ptr_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::pathfinding_shared::WaypointDataTrait>>>;
    fn sublevel_i_d(&self) -> &i32;
    fn sublevel_i_d_mut(&mut self) -> &mut i32;
}

impl AIWaypointExtraWaypointDataPtrTrait for AIWaypointExtraWaypointDataPtr {
    fn waypoint_data_ptr(&self) -> &Option<Arc<Mutex<dyn super::pathfinding_shared::WaypointDataTrait>>> {
        &self.waypoint_data_ptr
    }
    fn waypoint_data_ptr_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::pathfinding_shared::WaypointDataTrait>>> {
        &mut self.waypoint_data_ptr
    }
    fn sublevel_i_d(&self) -> &i32 {
        &self.sublevel_i_d
    }
    fn sublevel_i_d_mut(&mut self) -> &mut i32 {
        &mut self.sublevel_i_d
    }
}

pub static AIWAYPOINTEXTRAWAYPOINTDATAPTR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraWaypointDataPtr",
    flags: MemberInfoFlags::new(73),
    module: "AITools",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIWaypointExtraWaypointDataPtr as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "WaypointDataPtr",
                flags: MemberInfoFlags::new(0),
                field_type: "WaypointData",
                rust_offset: offset_of!(AIWaypointExtraWaypointDataPtr, waypoint_data_ptr),
            },
            FieldInfoData {
                name: "SublevelID",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AIWaypointExtraWaypointDataPtr, sublevel_i_d),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRAWAYPOINTDATAPTR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AIWaypointExtraWaypointDataPtr {
    fn type_info(&self) -> &'static TypeInfo {
        AIWAYPOINTEXTRAWAYPOINTDATAPTR_TYPE_INFO
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


pub static AIWAYPOINTEXTRAWAYPOINTDATAPTR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraWaypointDataPtr-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointExtraWaypointDataPtr"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIWaypointExtraTeleport {
    pub position: super::core::Vec3,
    pub yaw: f32,
}

pub trait AIWaypointExtraTeleportTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn position_mut(&mut self) -> &mut super::core::Vec3;
    fn yaw(&self) -> &f32;
    fn yaw_mut(&mut self) -> &mut f32;
}

impl AIWaypointExtraTeleportTrait for AIWaypointExtraTeleport {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.position
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn yaw_mut(&mut self) -> &mut f32 {
        &mut self.yaw
    }
}

pub static AIWAYPOINTEXTRATELEPORT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraTeleport",
    flags: MemberInfoFlags::new(36937),
    module: "AITools",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIWaypointExtraTeleport as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AIWaypointExtraTeleport, position),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AIWaypointExtraTeleport, yaw),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRATELEPORT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AIWaypointExtraTeleport {
    fn type_info(&self) -> &'static TypeInfo {
        AIWAYPOINTEXTRATELEPORT_TYPE_INFO
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


pub static AIWAYPOINTEXTRATELEPORT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraTeleport-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointExtraTeleport"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIWaypointExtraSpatial {
    pub position: super::core::Vec3,
    pub radius: f32,
    pub yaw: f32,
}

pub trait AIWaypointExtraSpatialTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn position_mut(&mut self) -> &mut super::core::Vec3;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn yaw(&self) -> &f32;
    fn yaw_mut(&mut self) -> &mut f32;
}

impl AIWaypointExtraSpatialTrait for AIWaypointExtraSpatial {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn position_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.position
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn yaw(&self) -> &f32 {
        &self.yaw
    }
    fn yaw_mut(&mut self) -> &mut f32 {
        &mut self.yaw
    }
}

pub static AIWAYPOINTEXTRASPATIAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraSpatial",
    flags: MemberInfoFlags::new(36937),
    module: "AITools",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIWaypointExtraSpatial as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AIWaypointExtraSpatial, position),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AIWaypointExtraSpatial, radius),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AIWaypointExtraSpatial, yaw),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRASPATIAL_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AIWaypointExtraSpatial {
    fn type_info(&self) -> &'static TypeInfo {
        AIWAYPOINTEXTRASPATIAL_TYPE_INFO
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


pub static AIWAYPOINTEXTRASPATIAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraSpatial-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointExtraSpatial"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AIWaypointGUID {
    pub g_u_i_d: i32,
}

pub trait AIWaypointGUIDTrait: TypeObject {
    fn g_u_i_d(&self) -> &i32;
    fn g_u_i_d_mut(&mut self) -> &mut i32;
}

impl AIWaypointGUIDTrait for AIWaypointGUID {
    fn g_u_i_d(&self) -> &i32 {
        &self.g_u_i_d
    }
    fn g_u_i_d_mut(&mut self) -> &mut i32 {
        &mut self.g_u_i_d
    }
}

pub static AIWAYPOINTGUID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointGUID",
    flags: MemberInfoFlags::new(36937),
    module: "AITools",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AIWaypointGUID as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GUID",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(AIWaypointGUID, g_u_i_d),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTGUID_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AIWaypointGUID {
    fn type_info(&self) -> &'static TypeInfo {
        AIWAYPOINTGUID_TYPE_INFO
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


pub static AIWAYPOINTGUID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointGUID-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointGUID"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerNavigationInterface {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerNavigationInterfaceTrait: super::entity::EntityTrait {
}

impl ServerNavigationInterfaceTrait for ServerNavigationInterface {
}

impl super::entity::EntityTrait for ServerNavigationInterface {
}

impl super::entity::EntityBusPeerTrait for ServerNavigationInterface {
}

pub static SERVERNAVIGATIONINTERFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavigationInterface",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerNavigationInterface as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERNAVIGATIONINTERFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerNavigationInterface {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERNAVIGATIONINTERFACE_TYPE_INFO
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


pub static SERVERNAVIGATIONINTERFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavigationInterface-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("ServerNavigationInterface"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerAuthNavigationInterface {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerAuthNavigationInterfaceTrait: super::entity::EntityTrait {
}

impl ServerAuthNavigationInterfaceTrait for ServerAuthNavigationInterface {
}

impl super::entity::EntityTrait for ServerAuthNavigationInterface {
}

impl super::entity::EntityBusPeerTrait for ServerAuthNavigationInterface {
}

pub static SERVERAUTHNAVIGATIONINTERFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAuthNavigationInterface",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerAuthNavigationInterface as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERAUTHNAVIGATIONINTERFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAuthNavigationInterface {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERAUTHNAVIGATIONINTERFACE_TYPE_INFO
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


pub static SERVERAUTHNAVIGATIONINTERFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAuthNavigationInterface-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("ServerAuthNavigationInterface"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocoEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LocoEntityTrait: super::entity::EntityTrait {
}

impl LocoEntityTrait for LocoEntity {
}

impl super::entity::EntityTrait for LocoEntity {
}

impl super::entity::EntityBusPeerTrait for LocoEntity {
}

pub static LOCOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntity",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocoEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocoEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOCOENTITY_TYPE_INFO
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


pub static LOCOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("LocoEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientNavigationInterface {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientNavigationInterfaceTrait: super::entity::EntityTrait {
}

impl ClientNavigationInterfaceTrait for ClientNavigationInterface {
}

impl super::entity::EntityTrait for ClientNavigationInterface {
}

impl super::entity::EntityBusPeerTrait for ClientNavigationInterface {
}

pub static CLIENTNAVIGATIONINTERFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientNavigationInterface",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientNavigationInterface as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTNAVIGATIONINTERFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientNavigationInterface {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTNAVIGATIONINTERFACE_TYPE_INFO
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


pub static CLIENTNAVIGATIONINTERFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientNavigationInterface-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("ClientNavigationInterface"),
    array_type: None,
    alignment: 8,
};


