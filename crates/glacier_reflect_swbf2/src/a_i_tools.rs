use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NavigationInterfaceData {
}

pub const NAVIGATIONINTERFACEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavigationInterfaceData",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NAVIGATIONINTERFACEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NavigationInterfaceData {
    fn type_info() -> &'static TypeInfo {
        NAVIGATIONINTERFACEDATA_TYPE_INFO
    }
}


pub const NAVIGATIONINTERFACEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavigationInterfaceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("NavigationInterfaceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocoEntityData {
}

pub const LOCOENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntityData",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCOENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocoEntityData {
    fn type_info() -> &'static TypeInfo {
        LOCOENTITYDATA_TYPE_INFO
    }
}


pub const LOCOENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("LocoEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIWaypointExtraWaypointDataPtr {
    pub waypoint_data_ptr: super::pathfinding_shared::WaypointData,
    pub sublevel_i_d: i32,
}

pub const AIWAYPOINTEXTRAWAYPOINTDATAPTR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraWaypointDataPtr",
    flags: MemberInfoFlags::new(73),
    module: "AITools",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "WaypointDataPtr",
                flags: MemberInfoFlags::new(0),
                field_type: WAYPOINTDATA_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraWaypointDataPtr, waypoint_data_ptr),
            },
            FieldInfoData {
                name: "SublevelID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraWaypointDataPtr, sublevel_i_d),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRAWAYPOINTDATAPTR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AIWaypointExtraWaypointDataPtr {
    fn type_info() -> &'static TypeInfo {
        AIWAYPOINTEXTRAWAYPOINTDATAPTR_TYPE_INFO
    }
}


pub const AIWAYPOINTEXTRAWAYPOINTDATAPTR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraWaypointDataPtr-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointExtraWaypointDataPtr-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AIWaypointExtraTeleport {
    pub position: super::core::Vec3,
    pub yaw: f32,
}

pub const AIWAYPOINTEXTRATELEPORT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraTeleport",
    flags: MemberInfoFlags::new(36937),
    module: "AITools",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraTeleport, position),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraTeleport, yaw),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRATELEPORT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AIWaypointExtraTeleport {
    fn type_info() -> &'static TypeInfo {
        AIWAYPOINTEXTRATELEPORT_TYPE_INFO
    }
}


pub const AIWAYPOINTEXTRATELEPORT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraTeleport-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointExtraTeleport-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AIWaypointExtraSpatial {
    pub position: super::core::Vec3,
    pub radius: f32,
    pub yaw: f32,
}

pub const AIWAYPOINTEXTRASPATIAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraSpatial",
    flags: MemberInfoFlags::new(36937),
    module: "AITools",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraSpatial, position),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraSpatial, radius),
            },
            FieldInfoData {
                name: "Yaw",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointExtraSpatial, yaw),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTEXTRASPATIAL_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AIWaypointExtraSpatial {
    fn type_info() -> &'static TypeInfo {
        AIWAYPOINTEXTRASPATIAL_TYPE_INFO
    }
}


pub const AIWAYPOINTEXTRASPATIAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointExtraSpatial-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointExtraSpatial-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AIWaypointGUID {
    pub g_u_i_d: i32,
}

pub const AIWAYPOINTGUID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointGUID",
    flags: MemberInfoFlags::new(36937),
    module: "AITools",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "GUID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AIWaypointGUID, g_u_i_d),
            },
        ],
    }),
    array_type: Some(AIWAYPOINTGUID_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AIWaypointGUID {
    fn type_info() -> &'static TypeInfo {
        AIWAYPOINTGUID_TYPE_INFO
    }
}


pub const AIWAYPOINTGUID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AIWaypointGUID-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("AIWaypointGUID-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerNavigationInterface {
}

pub const SERVERNAVIGATIONINTERFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavigationInterface",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERNAVIGATIONINTERFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerNavigationInterface {
    fn type_info() -> &'static TypeInfo {
        SERVERNAVIGATIONINTERFACE_TYPE_INFO
    }
}


pub const SERVERNAVIGATIONINTERFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavigationInterface-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("ServerNavigationInterface-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerAuthNavigationInterface {
}

pub const SERVERAUTHNAVIGATIONINTERFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAuthNavigationInterface",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERAUTHNAVIGATIONINTERFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerAuthNavigationInterface {
    fn type_info() -> &'static TypeInfo {
        SERVERAUTHNAVIGATIONINTERFACE_TYPE_INFO
    }
}


pub const SERVERAUTHNAVIGATIONINTERFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerAuthNavigationInterface-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("ServerAuthNavigationInterface-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocoEntity {
}

pub const LOCOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntity",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocoEntity {
    fn type_info() -> &'static TypeInfo {
        LOCOENTITY_TYPE_INFO
    }
}


pub const LOCOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocoEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("LocoEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientNavigationInterface {
}

pub const CLIENTNAVIGATIONINTERFACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientNavigationInterface",
    flags: MemberInfoFlags::new(101),
    module: "AITools",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTNAVIGATIONINTERFACE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientNavigationInterface {
    fn type_info() -> &'static TypeInfo {
        CLIENTNAVIGATIONINTERFACE_TYPE_INFO
    }
}


pub const CLIENTNAVIGATIONINTERFACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientNavigationInterface-Array",
    flags: MemberInfoFlags::new(145),
    module: "AITools",
    data: TypeInfoData::Array("ClientNavigationInterface-Array"),
    array_type: None,
    alignment: 8,
};


